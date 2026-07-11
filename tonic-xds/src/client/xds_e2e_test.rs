//! End-to-end tests: a real xDS channel resolving through a fake ADS control
//! plane to live gRPC backends.
//!
//! Unlike the unit tests in [`super::channel`], which inject endpoints through
//! `build_grpc_channel_from_parts`, these tests exercise the full pipeline:
//! bootstrap loading, the ADS transport, and LDS -> RDS -> CDS -> EDS resolution
//! served by [`XdsTestControlPlaneService`]. Traffic is routed to greeter (echo)
//! backends spawned by the tests.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use envoy_types::pb::envoy::config::cluster::v3::Cluster;
use envoy_types::pb::envoy::config::cluster::v3::cluster::{ClusterDiscoveryType, DiscoveryType};
use envoy_types::pb::envoy::config::core::v3 as core_v3;
use envoy_types::pb::envoy::config::endpoint::v3::{
    ClusterLoadAssignment, Endpoint, LbEndpoint, LocalityLbEndpoints, lb_endpoint::HostIdentifier,
};
use envoy_types::pb::envoy::config::listener::v3::{ApiListener, Listener};
use envoy_types::pb::envoy::config::route::v3::route::Action;
use envoy_types::pb::envoy::config::route::v3::route_action::ClusterSpecifier;
use envoy_types::pb::envoy::config::route::v3::route_match::PathSpecifier;
use envoy_types::pb::envoy::config::route::v3::{
    Route, RouteAction, RouteConfiguration, RouteMatch, VirtualHost,
};
use envoy_types::pb::envoy::extensions::filters::network::http_connection_manager::v3::{
    HttpConnectionManager, Rds, http_connection_manager::RouteSpecifier,
};
use envoy_types::pb::google::protobuf::Any;
use prost::Message;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use xds_test_util::{
    ADS_TYPE_URL_CDS, ADS_TYPE_URL_EDS, ADS_TYPE_URL_LDS, ADS_TYPE_URL_RDS,
    AggregatedDiscoveryServiceServer, XdsTestControlPlaneService,
};

use crate::testutil::grpc::{GreeterClient, HelloRequest, spawn_greeter_server};
use crate::{BootstrapConfig, XdsChannelBuilder, XdsChannelConfig, XdsChannelGrpc, XdsUri};

const TYPE_HCM: &str = "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager";

/// Wraps a `HttpConnectionManager` route specifier in an `ApiListener` listener.
fn build_listener(name: &str, route: RouteSpecifier) -> Listener {
    let hcm = HttpConnectionManager {
        route_specifier: Some(route),
        ..Default::default()
    };
    Listener {
        name: name.to_string(),
        api_listener: Some(ApiListener {
            api_listener: Some(Any {
                type_url: TYPE_HCM.to_string(),
                value: hcm.encode_to_vec(),
            }),
        }),
        ..Default::default()
    }
}

/// Builds a listener whose route configuration is embedded inline and sends all
/// traffic to `cluster`.
fn build_inline_listener(name: &str, cluster: &str) -> Listener {
    build_listener(
        name,
        RouteSpecifier::RouteConfig(build_route_config(&format!("{name}-route"), cluster)),
    )
}

/// Builds a listener that fetches its route configuration via RDS by name.
fn build_rds_listener(name: &str, rds_name: &str) -> Listener {
    build_listener(
        name,
        RouteSpecifier::Rds(Rds {
            route_config_name: rds_name.to_string(),
            ..Default::default()
        }),
    )
}

/// Builds a route configuration that sends all traffic to `cluster`.
fn build_route_config(name: &str, cluster: &str) -> RouteConfiguration {
    RouteConfiguration {
        name: name.to_string(),
        virtual_hosts: vec![VirtualHost {
            name: "default".to_string(),
            domains: vec!["*".to_string()],
            routes: vec![Route {
                r#match: Some(RouteMatch {
                    path_specifier: Some(PathSpecifier::Prefix("/".to_string())),
                    ..Default::default()
                }),
                action: Some(Action::Route(RouteAction {
                    cluster_specifier: Some(ClusterSpecifier::Cluster(cluster.to_string())),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Builds an EDS-discovered cluster named `name`.
fn build_cluster(name: &str) -> Cluster {
    Cluster {
        name: name.to_string(),
        cluster_discovery_type: Some(ClusterDiscoveryType::Type(DiscoveryType::Eds as i32)),
        ..Default::default()
    }
}

/// Builds a `ClusterLoadAssignment` for `cluster` with a single endpoint.
fn build_endpoints(cluster: &str, host: &str, port: u16) -> ClusterLoadAssignment {
    build_endpoints_multi(cluster, &[(host.to_string(), port)])
}

/// Builds a `ClusterLoadAssignment` for `cluster` with the given endpoints, all
/// placed in a single locality.
fn build_endpoints_multi(cluster: &str, endpoints: &[(String, u16)]) -> ClusterLoadAssignment {
    ClusterLoadAssignment {
        cluster_name: cluster.to_string(),
        endpoints: vec![LocalityLbEndpoints {
            lb_endpoints: endpoints
                .iter()
                .map(|(host, port)| lb_endpoint(host, *port))
                .collect(),
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Builds a single `LbEndpoint` at `host:port`.
fn lb_endpoint(host: &str, port: u16) -> LbEndpoint {
    LbEndpoint {
        host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
            address: Some(core_v3::Address {
                address: Some(core_v3::address::Address::SocketAddress(
                    core_v3::SocketAddress {
                        address: host.to_string(),
                        port_specifier: Some(core_v3::socket_address::PortSpecifier::PortValue(
                            u32::from(port),
                        )),
                        ..Default::default()
                    },
                )),
            }),
            ..Default::default()
        })),
        ..Default::default()
    }
}

/// Starts the fake ADS control plane on an ephemeral port. Returns the service
/// handle (for injecting config), its address, and the server task handle.
async fn start_control_plane() -> (
    XdsTestControlPlaneService,
    SocketAddr,
    JoinHandle<Result<(), tonic::transport::Error>>,
) {
    let control_plane = XdsTestControlPlaneService::new();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let service = control_plane.clone();
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AggregatedDiscoveryServiceServer::new(service))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
    });
    (control_plane, addr, handle)
}

/// Builds a real xDS channel whose bootstrap points at `cp_addr` and whose
/// target resolves the listener `listener_name`.
fn build_channel(cp_addr: SocketAddr, listener_name: &str) -> XdsChannelGrpc {
    let bootstrap_json =
        format!(r#"{{"xds_servers":[{{"server_uri":"http://{cp_addr}"}}],"node":{{"id":"test"}}}}"#);
    let bootstrap = BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap");
    let target = XdsUri::parse(&format!("xds:///{listener_name}")).expect("parse target");
    XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .expect("build xds channel")
}

/// Sends `say_hello` in a loop until a reply starting with `want_prefix` is
/// observed (xDS resolution and config updates are asynchronous), returning that
/// reply. Panics if it never arrives.
async fn say_hello_until_prefix(
    client: &mut GreeterClient<XdsChannelGrpc>,
    want_prefix: &str,
) -> String {
    let mut last = None;
    for _ in 0..50 {
        match client
            .say_hello(HelloRequest {
                name: "world".to_string(),
            })
            .await
        {
            Ok(response) => {
                let message = response.into_inner().message;
                if message.starts_with(want_prefix) {
                    return message;
                }
                last = Some(Ok(message));
            }
            Err(status) => last = Some(Err(status)),
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("never observed reply starting with {want_prefix:?}; last seen: {last:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn xds_channel_e2e_routes_to_backend() {
    let backend = spawn_greeter_server("backend", None, None)
        .await
        .expect("spawn greeter backend");
    let backend_addr = backend.addr;

    let (control_plane, cp_addr, cp_handle) = start_control_plane().await;

    // Configure LDS (inline route) -> CDS -> EDS pointing at the backend.
    control_plane.set_xds_config(
        ADS_TYPE_URL_LDS,
        HashMap::from([(
            "my-service".to_string(),
            build_inline_listener("my-service", "my-cluster"),
        )]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_CDS,
        HashMap::from([("my-cluster".to_string(), build_cluster("my-cluster"))]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_EDS,
        HashMap::from([(
            "my-cluster".to_string(),
            build_endpoints(
                "my-cluster",
                &backend_addr.ip().to_string(),
                backend_addr.port(),
            ),
        )]),
    );

    let mut client = GreeterClient::new(build_channel(cp_addr, "my-service"));
    let reply = say_hello_until_prefix(&mut client, "backend:").await;
    assert_eq!(reply, "backend: world");

    // The control plane observed exactly one ADS stream from the client.
    let counts = control_plane.get_subscriber_counts();
    assert_eq!(counts.get(ADS_TYPE_URL_LDS), Some(&1));
    assert_eq!(counts.get(ADS_TYPE_URL_CDS), Some(&1));
    assert_eq!(counts.get(ADS_TYPE_URL_EDS), Some(&1));

    let _ = backend.shutdown.send(());
    cp_handle.abort();
}

/// Analog of grpc-java's `changeClusterForRoute`: once the client is routing to
/// one cluster, update the RDS route to point at a different cluster and assert
/// traffic shifts to the new backend — exercising the control plane pushing a
/// live update to a connected client.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn xds_channel_e2e_route_update_shifts_traffic() {
    let backend_a = spawn_greeter_server("backend-a", None, None)
        .await
        .expect("spawn backend-a");
    let backend_b = spawn_greeter_server("backend-b", None, None)
        .await
        .expect("spawn backend-b");

    let (control_plane, cp_addr, cp_handle) = start_control_plane().await;

    // LDS -> RDS "route-config"; both clusters and their endpoints are configured
    // up front, and the route initially targets cluster-a.
    control_plane.set_xds_config(
        ADS_TYPE_URL_LDS,
        HashMap::from([(
            "my-service".to_string(),
            build_rds_listener("my-service", "route-config"),
        )]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_RDS,
        HashMap::from([(
            "route-config".to_string(),
            build_route_config("route-config", "cluster-a"),
        )]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_CDS,
        HashMap::from([
            ("cluster-a".to_string(), build_cluster("cluster-a")),
            ("cluster-b".to_string(), build_cluster("cluster-b")),
        ]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_EDS,
        HashMap::from([
            (
                "cluster-a".to_string(),
                build_endpoints(
                    "cluster-a",
                    &backend_a.addr.ip().to_string(),
                    backend_a.addr.port(),
                ),
            ),
            (
                "cluster-b".to_string(),
                build_endpoints(
                    "cluster-b",
                    &backend_b.addr.ip().to_string(),
                    backend_b.addr.port(),
                ),
            ),
        ]),
    );

    let mut client = GreeterClient::new(build_channel(cp_addr, "my-service"));

    // Initially routed to cluster-a -> backend-a.
    let reply = say_hello_until_prefix(&mut client, "backend-a:").await;
    assert_eq!(reply, "backend-a: world");

    // Update the RDS route to target cluster-b; the control plane pushes the new
    // RouteConfiguration to the connected client.
    control_plane.set_xds_config(
        ADS_TYPE_URL_RDS,
        HashMap::from([(
            "route-config".to_string(),
            build_route_config("route-config", "cluster-b"),
        )]),
    );

    // Traffic shifts to cluster-b -> backend-b.
    let reply = say_hello_until_prefix(&mut client, "backend-b:").await;
    assert_eq!(reply, "backend-b: world");

    let _ = backend_a.shutdown.send(());
    let _ = backend_b.shutdown.send(());
    cp_handle.abort();
}

/// P2C load balancing: with several EDS endpoints behind one cluster, traffic
/// should spread across all of them. tonic-xds uses power-of-two-choices as its
/// balancer (see the unit-level `test_xds_channel_grpc_with_p2c_lb`); this is the
/// full-pipeline version through the control plane.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn xds_channel_e2e_p2c_spreads_across_backends() {
    const NUM_BACKENDS: usize = 5;
    const NUM_REQUESTS: usize = 300;

    // Spawn several greeter backends, each reporting a distinct name.
    let mut backends = Vec::new();
    for i in 0..NUM_BACKENDS {
        backends.push(
            spawn_greeter_server(&format!("backend-{i}"), None, None)
                .await
                .expect("spawn backend"),
        );
    }
    let endpoints: Vec<(String, u16)> = backends
        .iter()
        .map(|backend| (backend.addr.ip().to_string(), backend.addr.port()))
        .collect();

    let (control_plane, cp_addr, cp_handle) = start_control_plane().await;

    // Configure LDS (inline route) -> CDS -> EDS with all backends in one cluster.
    control_plane.set_xds_config(
        ADS_TYPE_URL_LDS,
        HashMap::from([(
            "my-service".to_string(),
            build_inline_listener("my-service", "my-cluster"),
        )]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_CDS,
        HashMap::from([("my-cluster".to_string(), build_cluster("my-cluster"))]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_EDS,
        HashMap::from([(
            "my-cluster".to_string(),
            build_endpoints_multi("my-cluster", &endpoints),
        )]),
    );

    let mut client = GreeterClient::new(build_channel(cp_addr, "my-service"));

    // Warm up until xDS resolution completes and the first RPC succeeds.
    say_hello_until_prefix(&mut client, "backend-").await;

    // Tally which backend served each request.
    let mut counts: HashMap<String, usize> = HashMap::new();
    for _ in 0..NUM_REQUESTS {
        let reply = client
            .say_hello(HelloRequest {
                name: "world".to_string(),
            })
            .await
            .expect("rpc succeeds")
            .into_inner()
            .message;
        let server = reply.split(':').next().unwrap_or_default().to_string();
        *counts.entry(server).or_default() += 1;
    }

    // Every backend should have received a fair share of the traffic.
    assert_eq!(counts.values().sum::<usize>(), NUM_REQUESTS);
    let min_per_backend = NUM_REQUESTS / NUM_BACKENDS / 4;
    for i in 0..NUM_BACKENDS {
        let name = format!("backend-{i}");
        let count = counts.get(&name).copied().unwrap_or(0);
        assert!(
            count >= min_per_backend,
            "backend {name} received {count} requests (< {min_per_backend}); distribution: {counts:?}",
        );
    }

    for backend in backends {
        let _ = backend.shutdown.send(());
    }
    cp_handle.abort();
}
