//! End-to-end test: a real xDS channel resolving through a fake ADS control
//! plane to a live gRPC backend.
//!
//! Unlike the unit tests in [`super::channel`], which inject endpoints through
//! `build_grpc_channel_from_parts`, this test exercises the full pipeline:
//! bootstrap loading, the ADS transport, and LDS -> CDS -> EDS resolution served
//! by [`XdsTestControlPlaneService`]. Traffic is routed to a greeter (echo)
//! backend spawned by the test.

use std::collections::HashMap;
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
    HttpConnectionManager, http_connection_manager::RouteSpecifier,
};
use envoy_types::pb::google::protobuf::Any;
use prost::Message;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use xds_test_util::{
    ADS_TYPE_URL_CDS, ADS_TYPE_URL_EDS, ADS_TYPE_URL_LDS, AggregatedDiscoveryServiceServer,
    XdsTestControlPlaneService,
};

use crate::testutil::grpc::{GreeterClient, HelloRequest, spawn_greeter_server};
use crate::{BootstrapConfig, XdsChannelBuilder, XdsChannelConfig, XdsUri};

const LISTENER_NAME: &str = "my-service";
const CLUSTER_NAME: &str = "my-cluster";
const TYPE_HCM: &str = "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager";

/// Builds a gRPC listener whose inline route configuration sends all traffic to
/// `CLUSTER_NAME`.
fn build_listener() -> Listener {
    let hcm = HttpConnectionManager {
        route_specifier: Some(RouteSpecifier::RouteConfig(RouteConfiguration {
            name: format!("{LISTENER_NAME}-route"),
            virtual_hosts: vec![VirtualHost {
                name: "default".to_string(),
                domains: vec!["*".to_string()],
                routes: vec![Route {
                    r#match: Some(RouteMatch {
                        path_specifier: Some(PathSpecifier::Prefix("/".to_string())),
                        ..Default::default()
                    }),
                    action: Some(Action::Route(RouteAction {
                        cluster_specifier: Some(ClusterSpecifier::Cluster(
                            CLUSTER_NAME.to_string(),
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        })),
        ..Default::default()
    };
    Listener {
        name: LISTENER_NAME.to_string(),
        api_listener: Some(ApiListener {
            api_listener: Some(Any {
                type_url: TYPE_HCM.to_string(),
                value: hcm.encode_to_vec(),
            }),
        }),
        ..Default::default()
    }
}

/// Builds an EDS-discovered cluster named `CLUSTER_NAME`.
fn build_cluster() -> Cluster {
    Cluster {
        name: CLUSTER_NAME.to_string(),
        cluster_discovery_type: Some(ClusterDiscoveryType::Type(DiscoveryType::Eds as i32)),
        ..Default::default()
    }
}

/// Builds a `ClusterLoadAssignment` with a single endpoint at `host:port`.
fn build_endpoints(host: &str, port: u16) -> ClusterLoadAssignment {
    ClusterLoadAssignment {
        cluster_name: CLUSTER_NAME.to_string(),
        endpoints: vec![LocalityLbEndpoints {
            lb_endpoints: vec![LbEndpoint {
                host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
                    address: Some(core_v3::Address {
                        address: Some(core_v3::address::Address::SocketAddress(
                            core_v3::SocketAddress {
                                address: host.to_string(),
                                port_specifier: Some(
                                    core_v3::socket_address::PortSpecifier::PortValue(u32::from(
                                        port,
                                    )),
                                ),
                                ..Default::default()
                            },
                        )),
                    }),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn xds_channel_e2e_routes_to_backend() {
    // 1. Spawn the greeter (echo) backend.
    let backend = spawn_greeter_server("backend", None, None)
        .await
        .expect("spawn greeter backend");
    let backend_addr = backend.addr;

    // 2. Start the fake ADS control plane on an ephemeral port.
    let control_plane = XdsTestControlPlaneService::new();
    let cp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let cp_addr = cp_listener.local_addr().unwrap();
    let cp_service = control_plane.clone();
    let cp_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AggregatedDiscoveryServiceServer::new(cp_service))
            .serve_with_incoming(TcpListenerStream::new(cp_listener))
            .await
    });

    // 3. Configure LDS -> CDS -> EDS pointing at the backend.
    control_plane.set_xds_config(
        ADS_TYPE_URL_LDS,
        HashMap::from([(LISTENER_NAME.to_string(), build_listener())]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_CDS,
        HashMap::from([(CLUSTER_NAME.to_string(), build_cluster())]),
    );
    control_plane.set_xds_config(
        ADS_TYPE_URL_EDS,
        HashMap::from([(
            CLUSTER_NAME.to_string(),
            build_endpoints(&backend_addr.ip().to_string(), backend_addr.port()),
        )]),
    );

    // 4. Build a real xDS channel via bootstrap pointing at the control plane.
    let bootstrap_json = format!(
        r#"{{"xds_servers":[{{"server_uri":"http://{cp_addr}"}}],"node":{{"id":"test"}}}}"#
    );
    let bootstrap = BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap");
    let target = XdsUri::parse(&format!("xds:///{LISTENER_NAME}")).expect("parse target");
    let channel = XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .expect("build xds channel");

    let mut client = GreeterClient::new(channel);

    // 5. Send requests, retrying until xDS resolution completes.
    let mut reply = None;
    let mut last_err = None;
    for _ in 0..50 {
        match client
            .say_hello(HelloRequest {
                name: "world".to_string(),
            })
            .await
        {
            Ok(response) => {
                reply = Some(response.into_inner().message);
                break;
            }
            Err(status) => {
                last_err = Some(status);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    let reply = reply.unwrap_or_else(|| panic!("no successful RPC; last error: {last_err:?}"));
    assert_eq!(reply, "backend: world");

    // The control plane observed exactly one ADS stream from the client.
    let counts = control_plane.get_subscriber_counts();
    assert_eq!(counts.get(ADS_TYPE_URL_LDS), Some(&1));
    assert_eq!(counts.get(ADS_TYPE_URL_CDS), Some(&1));
    assert_eq!(counts.get(ADS_TYPE_URL_EDS), Some(&1));

    // 6. Cleanup.
    let _ = backend.shutdown.send(());
    cp_handle.abort();
}
