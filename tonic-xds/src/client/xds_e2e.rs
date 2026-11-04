/*
 *
 * Copyright 2025 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */
//! End-to-end tests: a real xDS channel resolving through a fake ADS control
//! plane to live gRPC backends.
//!
//! Unlike the unit tests in [`super::channel`], which inject endpoints through
//! `build_grpc_channel_from_parts`, these tests exercise the full pipeline:
//! bootstrap loading, the ADS transport, and LDS -> RDS -> CDS -> EDS resolution
//! served by [`XdsTestControlPlaneService`]. Traffic is routed to greeter (echo)
//! backends spawned by the tests.
mod test {
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::sync::Arc;

    use envoy_types::pb::envoy::admin::v3::ClientResourceStatus;
    use envoy_types::pb::envoy::config::core::v3::HttpProtocolOptions;
    use envoy_types::pb::envoy::config::listener::v3::Listener;
    use envoy_types::pb::envoy::config::route::v3::RetryPolicy;
    use envoy_types::pb::envoy::config::route::v3::RouteAction;
    use envoy_types::pb::envoy::config::route::v3::RouteConfiguration;
    use envoy_types::pb::envoy::config::route::v3::retry_policy::RetryBackOff;
    use envoy_types::pb::envoy::config::route::v3::route::Action;
    use envoy_types::pb::envoy::config::route::v3::route_action;
    use envoy_types::pb::envoy::extensions::filters::network::http_connection_manager::v3::HttpConnectionManager;
    use envoy_types::pb::envoy::service::status::v3::client_status_discovery_service_client::ClientStatusDiscoveryServiceClient;
    use envoy_types::pb::envoy::service::status::v3::{ClientConfig, ClientStatusRequest};
    use envoy_types::pb::google::protobuf::Duration as ProtoDuration;
    use envoy_types::pb::google::protobuf::UInt32Value;
    use prost::Message;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::sync::watch;
    use tokio::time::Duration;
    use tokio_stream::wrappers::TcpListenerStream;
    use tokio_util::sync::CancellationToken;
    use xds_test_util::RunningControlPlane;
    use xds_test_util::XdsTestControlPlaneService;
    use xds_test_util::config;

    use crate::BootstrapConfig;
    use crate::CsdsService;
    use crate::XdsChannelBuilder;
    use crate::XdsChannelConfig;
    use crate::XdsChannelGrpc;
    use crate::XdsClientPool;
    use crate::XdsUri;
    use crate::client::channel::{LB_BACKENDS, LbBackend};
    use crate::csds::CsdsRegistry;
    use crate::testutil::grpc::GreeterClient;
    use crate::testutil::grpc::HelloRequest;
    use crate::testutil::grpc::ScriptedGreeter;
    use crate::testutil::grpc::spawn_greeter_server;
    use crate::testutil::grpc::spawn_scripted_server;

    /// Starts the fake ADS control plane on an ephemeral port. Returns the
    /// running control plane (which injects config and shuts down on drop)
    /// and its address.
    async fn start_control_plane() -> (RunningControlPlane, SocketAddr) {
        let running = XdsTestControlPlaneService::new()
            .start()
            .await
            .expect("start control plane");
        let addr = running.addr();
        (running, addr)
    }

    /// Builds a real xDS channel whose bootstrap points at `cp_addr`, whose
    /// target resolves the listener `listener_name`, and whose clusters are
    /// load-balanced by `backend`.
    fn build_channel_with_backend(
        cp_addr: SocketAddr,
        listener_name: &str,
        backend: LbBackend,
    ) -> XdsChannelGrpc {
        builder_for(cp_addr, listener_name)
            .with_lb_backend(backend)
            .build_grpc_channel()
            .expect("build xds channel")
    }

    fn builder_for(cp_addr: SocketAddr, listener_name: &str) -> XdsChannelBuilder {
        let target = XdsUri::parse(&format!("xds:///{listener_name}")).expect("parse target");
        XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap_for(cp_addr)))
    }

    fn bootstrap_for(cp_addr: SocketAddr) -> BootstrapConfig {
        let bootstrap_json = format!(
            r#"{{"xds_servers":[{{"server_uri":"http://{cp_addr}","channel_creds":[{{"type":"insecure"}}]}}],"node":{{"id":"test"}}}}"#
        );
        BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap")
    }

    fn configure_route(
        control_plane: &XdsTestControlPlaneService,
        name: &str,
        backend_addr: SocketAddr,
    ) {
        let cluster = format!("{name}-cluster");
        control_plane.set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                name.to_string(),
                config::build_inline_listener(name, &cluster),
            )]),
        );
        control_plane.set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(cluster.clone(), config::build_cluster(&cluster))]),
        );
        control_plane.set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                cluster.clone(),
                config::build_cla(
                    &cluster,
                    &[(backend_addr.ip().to_string(), backend_addr.port())],
                ),
            )]),
        );
    }

    /// Sends `say_hello` in a loop until a reply starting with `want_prefix` is
    /// observed (xDS resolution and config updates are asynchronous), returning
    /// that reply. Panics if it never arrives.
    async fn say_hello_until_prefix(
        client: &mut GreeterClient<XdsChannelGrpc>,
        want_prefix: &str,
    ) -> String {
        const RETRIES: usize = 100;
        let mut last = None;
        for _ in 0..RETRIES {
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

    /// End-to-end test: verifies that an xDS channel routes traffic to the
    /// correct backend.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn xds_channel_e2e_routes_to_backend() {
        for lb in LB_BACKENDS {
            routes_to_backend(lb).await;
        }
    }

    async fn routes_to_backend(lb: LbBackend) {
        let backend = spawn_greeter_server("backend", None, None)
            .await
            .expect("spawn greeter backend");
        let backend_addr = backend.addr;

        let (control_plane, cp_addr) = start_control_plane().await;

        // Configure LDS (inline route) -> CDS -> EDS pointing at the backend.
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "my-service".to_string(),
                config::build_inline_listener("my-service", "my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cluster("my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cla(
                    "my-cluster",
                    &[(backend_addr.ip().to_string(), backend_addr.port())],
                ),
            )]),
        );

        let mut client = GreeterClient::new(build_channel_with_backend(cp_addr, "my-service", lb));
        let reply = say_hello_until_prefix(&mut client, "backend:").await;
        assert_eq!(reply, "backend: world", "{lb:?}");

        // The control plane observed exactly one ADS stream from the client.
        let counts = control_plane.get_service().get_subscriber_counts();
        assert_eq!(counts.get(&config::AdsTypeUrl::Lds), Some(&1), "{lb:?}");
        assert_eq!(counts.get(&config::AdsTypeUrl::Cds), Some(&1), "{lb:?}");
        assert_eq!(counts.get(&config::AdsTypeUrl::Eds), Some(&1), "{lb:?}");

        let _ = backend.shutdown.send(());
    }

    #[cfg(unix)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn xds_channel_e2e_uds_control_plane_routes_to_backend() {
        use crate::common::async_util::AbortOnDrop;
        use envoy_types::pb::envoy::service::discovery::v3::aggregated_discovery_service_server::AggregatedDiscoveryServiceServer;
        use tokio_stream::wrappers::UnixListenerStream;

        const LISTENER: &str = "my-service";

        let backend = spawn_greeter_server("backend", None, None)
            .await
            .expect("spawn greeter backend");
        let control_plane = XdsTestControlPlaneService::new();
        configure_route(&control_plane, LISTENER, backend.addr);

        let directory = tempfile::tempdir().expect("create socket directory");
        let path = directory.path().join("ads.sock");
        let listener = tokio::net::UnixListener::bind(&path).expect("bind control-plane socket");
        let service = control_plane.clone();
        let _server = AbortOnDrop(tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(AggregatedDiscoveryServiceServer::new(service))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .expect("serve UDS control plane");
        }));

        let bootstrap = BootstrapConfig::builder(format!("unix://{}", path.display()))
            .channel_creds([crate::ChannelCredentialType::Insecure])
            .node_id("test")
            .build()
            .expect("build UDS bootstrap");
        let target = XdsUri::parse(&format!("xds:///{LISTENER}")).expect("parse target");
        let channel =
            XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap))
                .build_grpc_channel()
                .expect("build xds channel");
        let mut client = GreeterClient::new(channel);
        let reply = tokio::time::timeout(
            Duration::from_secs(10),
            say_hello_until_prefix(&mut client, "backend:"),
        )
        .await
        .expect("route RPC through the UDS control plane");
        assert_eq!(reply, "backend: world");

        let counts = control_plane.get_subscriber_counts();
        for resource_type in [
            config::AdsTypeUrl::Lds,
            config::AdsTypeUrl::Cds,
            config::AdsTypeUrl::Eds,
        ] {
            assert_eq!(counts.get(&resource_type), Some(&1), "{resource_type:?}");
        }
        let _ = backend.shutdown.send(());
    }

    /// The client is initially routing to
    /// one cluster, update the RDS route to point at a different cluster and
    /// assert traffic shifts to the new backend — exercising the control
    /// plane pushing a live update to a connected client.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn xds_channel_e2e_route_update_shifts_traffic() {
        for lb in LB_BACKENDS {
            route_update_shifts_traffic(lb).await;
        }
    }

    async fn route_update_shifts_traffic(lb: LbBackend) {
        let backend_a = spawn_greeter_server("backend-a", None, None)
            .await
            .expect("spawn backend-a");
        let backend_b = spawn_greeter_server("backend-b", None, None)
            .await
            .expect("spawn backend-b");

        let (control_plane, cp_addr) = start_control_plane().await;

        // LDS -> RDS "route-config"; both clusters and their endpoints are configured
        // up front, and the route initially targets cluster-a.
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "my-service".to_string(),
                config::build_rds_listener("my-service", "route-config"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Rds,
            HashMap::from([(
                "route-config".to_string(),
                config::build_route_config("route-config", "cluster-a"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([
                ("cluster-a".to_string(), config::build_cluster("cluster-a")),
                ("cluster-b".to_string(), config::build_cluster("cluster-b")),
            ]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([
                (
                    "cluster-a".to_string(),
                    config::build_cla(
                        "cluster-a",
                        &[(backend_a.addr.ip().to_string(), backend_a.addr.port())],
                    ),
                ),
                (
                    "cluster-b".to_string(),
                    config::build_cla(
                        "cluster-b",
                        &[(backend_b.addr.ip().to_string(), backend_b.addr.port())],
                    ),
                ),
            ]),
        );

        let mut client = GreeterClient::new(build_channel_with_backend(cp_addr, "my-service", lb));

        // Initially routed to cluster-a -> backend-a.
        let reply = say_hello_until_prefix(&mut client, "backend-a:").await;
        assert_eq!(reply, "backend-a: world", "{lb:?}");

        // Update the RDS route to target cluster-b; the control plane pushes the new
        // RouteConfiguration to the connected client.
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Rds,
            HashMap::from([(
                "route-config".to_string(),
                config::build_route_config("route-config", "cluster-b"),
            )]),
        );

        // Traffic shifts to cluster-b -> backend-b.
        let reply = say_hello_until_prefix(&mut client, "backend-b:").await;
        assert_eq!(reply, "backend-b: world", "{lb:?}");

        let _ = backend_a.shutdown.send(());
        let _ = backend_b.shutdown.send(());
    }

    /// Load balancing: with several EDS endpoints behind one cluster, traffic
    /// should spread across all of them. Runs against both balancer backends —
    /// Tower's P2C and the in-crate one — through the full control-plane
    /// pipeline.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn xds_channel_e2e_p2c_spreads_across_backends() {
        for lb in LB_BACKENDS {
            spreads_across_backends(lb).await;
        }
    }

    async fn spreads_across_backends(lb: LbBackend) {
        const NUM_BACKENDS: usize = 5;
        const NUM_REQUESTS: usize = 1000;

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

        let (control_plane, cp_addr) = start_control_plane().await;

        // Configure LDS (inline route) -> CDS -> EDS with all backends in one cluster.
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "my-service".to_string(),
                config::build_inline_listener("my-service", "my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cluster("my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cla("my-cluster", &endpoints),
            )]),
        );

        let mut client = GreeterClient::new(build_channel_with_backend(cp_addr, "my-service", lb));

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
        assert_eq!(counts.values().sum::<usize>(), NUM_REQUESTS, "{lb:?}");
        // Each backend should receive at least 50% of the fair share of requests.
        let min_per_backend = (NUM_REQUESTS / NUM_BACKENDS) / 2;
        for i in 0..NUM_BACKENDS {
            let name = format!("backend-{i}");
            let count = counts.get(&name).copied().unwrap_or(0);
            assert!(
                count >= min_per_backend,
                "{lb:?}: backend {name} received {count} requests (< {min_per_backend}); \
                 distribution: {counts:?}",
            );
        }

        for backend in backends {
            let _ = backend.shutdown.send(());
        }
    }

    /// A CDS response past tonic's 4 MiB decode default only arrives when the
    /// channel raises the limit, so successful routing proves the configured
    /// ceiling reached the ADS transport.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn configured_decoding_limit_admits_oversized_ads_response() {
        const PADDING: usize = 5 * 1024 * 1024;

        let backend = spawn_greeter_server("backend", None, None)
            .await
            .expect("spawn greeter backend");
        let backend_addr = backend.addr;

        let (control_plane, cp_addr) = start_control_plane().await;

        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "my-service".to_string(),
                config::build_inline_listener("my-service", "my-cluster"),
            )]),
        );
        // `alt_stat_name` is ignored by CDS validation, so it pads the response
        // without changing how the cluster behaves.
        let mut cluster = config::build_cluster("my-cluster");
        cluster.alt_stat_name = "x".repeat(PADDING);
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([("my-cluster".to_string(), cluster)]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cla(
                    "my-cluster",
                    &[(backend_addr.ip().to_string(), backend_addr.port())],
                ),
            )]),
        );

        let bootstrap_json = format!(
            r#"{{"xds_servers":[{{"server_uri":"http://{cp_addr}","channel_creds":[{{"type":"insecure"}}]}}],"node":{{"id":"test"}}}}"#
        );
        let bootstrap = BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap");
        let target = XdsUri::parse("xds:///my-service").expect("parse target");
        let channel = XdsChannelBuilder::new(
            XdsChannelConfig::new(target)
                .with_bootstrap(bootstrap)
                .with_max_decoding_message_size(2 * PADDING),
        )
        .build_grpc_channel()
        .expect("build xds channel");

        let mut client = GreeterClient::new(channel);
        let reply = tokio::time::timeout(
            Duration::from_secs(30),
            say_hello_until_prefix(&mut client, "backend:"),
        )
        .await
        .expect("oversized CDS response never routed; decoding limit not applied");
        assert_eq!(reply, "backend: world");

        let _ = backend.shutdown.send(());
    }

    mod user_transport {
        use super::*;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use tower::ServiceExt as _;

        type TestService = tower::util::BoxCloneService<
            http::Request<tonic::body::Body>,
            http::Response<tonic::body::Body>,
            tonic::transport::Error,
        >;

        struct TestMakeControlPlaneConnector {
            addr: SocketAddr,
            expected_uri: String,
            keep_alive: Option<(Option<Duration>, Duration)>,
            calls: Arc<AtomicUsize>,
        }

        impl crate::MakeControlPlaneConnector for TestMakeControlPlaneConnector {
            type Service = TestService;

            fn make_connector(
                &self,
                config: crate::ControlPlaneConfig<'_>,
            ) -> xds_client::Result<Arc<dyn crate::ControlPlaneConnector<Service = Self::Service>>>
            {
                assert_eq!(config.server().uri(), self.expected_uri);
                assert_eq!(config.keep_alive(), self.keep_alive);
                Ok(Arc::new(TestControlPlaneConnector {
                    addr: self.addr,
                    keep_alive: config.keep_alive(),
                    calls: self.calls.clone(),
                }))
            }
        }

        struct TestControlPlaneConnector {
            addr: SocketAddr,
            keep_alive: Option<(Option<Duration>, Duration)>,
            calls: Arc<AtomicUsize>,
        }

        impl crate::ControlPlaneConnector for TestControlPlaneConnector {
            type Service = TestService;

            fn is_secure(&self) -> bool {
                false
            }

            fn connect(&self) -> crate::BoxFuture<xds_client::Result<Self::Service>> {
                let addr = self.addr;
                let keep_alive = self.keep_alive;
                let calls = self.calls.clone();
                Box::pin(async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    let mut builder = xds_client::TonicTransportBuilder::new();
                    if let Some((interval, timeout)) = keep_alive {
                        builder = builder.with_keep_alive(interval, timeout);
                    }
                    let server = crate::ServerConfig::new(format!("http://{addr}"));
                    let channel = builder.connect_channel(&server).await?;
                    let service =
                        tower::service_fn(move |request: http::Request<tonic::body::Body>| {
                            assert_eq!(
                                request.headers().get("authorization").unwrap(),
                                "Bearer custom"
                            );
                            channel.clone().oneshot(request)
                        });
                    Ok(tower::util::BoxCloneService::new(service))
                })
            }
        }

        #[derive(Debug)]
        struct TestCallCredentials;

        #[tonic::async_trait]
        impl crate::TonicCallCredentials for TestCallCredentials {
            async fn get_request_metadata(
                &self,
                metadata: &mut tonic::metadata::MetadataMap,
            ) -> Result<(), tonic::Status> {
                metadata.insert("authorization", "Bearer custom".parse().unwrap());
                Ok(())
            }

            fn requires_secure_transport(&self) -> bool {
                false
            }
        }

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn custom_control_plane_connector_owns_transport() {
            routes_through_custom_control_plane_connector(false).await;
        }

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn pooled_custom_control_plane_connector_owns_transport() {
            routes_through_custom_control_plane_connector(true).await;
        }

        async fn routes_through_custom_control_plane_connector(use_pool: bool) {
            const LISTENER: &str = "my-service";

            let backend = spawn_greeter_server("backend", None, None)
                .await
                .expect("spawn greeter backend");
            let (control_plane, cp_addr) = start_control_plane().await;
            configure_route(control_plane.get_service(), LISTENER, backend.addr);

            let calls = Arc::new(AtomicUsize::new(0));
            let connector = TestMakeControlPlaneConnector {
                addr: cp_addr,
                expected_uri: cp_addr.to_string(),
                keep_alive: Some((None, Duration::from_secs(1))),
                calls: calls.clone(),
            };
            let bootstrap = BootstrapConfig::from_json(&format!(
                r#"{{"xds_servers":[{{"server_uri":"{cp_addr}","channel_creds":[{{"type":"insecure"}}]}}],"node":{{"id":"test"}}}}"#
            ))
            .unwrap();
            let target = XdsUri::parse(&format!("xds:///{LISTENER}")).expect("parse target");

            let builder = if use_pool {
                let pool = XdsClientPool::builder()
                    .with_bootstrap(bootstrap)
                    .with_call_credentials(Arc::new(TestCallCredentials))
                    .with_control_plane_connector(connector)
                    .with_ads_keep_alive(None, Duration::from_secs(1))
                    .build()
                    .expect("build pool");
                XdsChannelBuilder::new(XdsChannelConfig::new(target)).with_xds_client_pool(pool)
            } else {
                XdsChannelBuilder::new(
                    XdsChannelConfig::new(target)
                        .with_bootstrap(bootstrap)
                        .with_call_credentials(Arc::new(TestCallCredentials))
                        .with_ads_keep_alive(None, Duration::from_secs(1)),
                )
                .with_control_plane_connector(connector)
            };
            let channel = builder.build_grpc_channel().expect("build xds channel");

            let mut client = GreeterClient::new(channel);
            assert_eq!(
                say_hello_until_prefix(&mut client, "backend:").await,
                "backend: world"
            );
            assert!(calls.load(Ordering::SeqCst) > 0, "connector was not used");

            let _ = backend.shutdown.send(());
        }
    }

    /// Forwards TCP connections to `upstream` until `blackhole` is cancelled,
    /// then stops forwarding without closing the sockets, leaving every
    /// connection half-open. Returns the proxy address and the number of
    /// connections accepted so far.
    async fn start_blackhole_proxy(
        upstream: SocketAddr,
        blackhole: CancellationToken,
    ) -> (SocketAddr, watch::Receiver<usize>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind proxy");
        let addr = listener.local_addr().expect("proxy addr");
        let (accepted_tx, accepted_rx) = watch::channel(0);
        tokio::spawn(async move {
            while let Ok((mut inbound, _)) = listener.accept().await {
                accepted_tx.send_modify(|accepted| *accepted += 1);
                let blackhole = blackhole.clone();
                tokio::spawn(async move {
                    let Ok(mut outbound) = TcpStream::connect(upstream).await else {
                        return;
                    };
                    tokio::select! {
                        _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound) => {}
                        _ = blackhole.cancelled() => std::future::pending::<()>().await,
                    }
                });
            }
        });
        (addr, accepted_rx)
    }

    /// When the control plane stops answering, an unacked keepalive PING
    /// closes the ADS connection and the client dials a new one. Reconnecting
    /// within seconds, well under the default keepalive interval, proves the
    /// configured keepalive reached the ADS transport.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn configured_keep_alive_replaces_half_open_ads_connection() {
        let (control_plane, cp_addr) = start_control_plane().await;
        let blackhole = CancellationToken::new();
        let (proxy_addr, mut accepted) = start_blackhole_proxy(cp_addr, blackhole.clone()).await;

        let bootstrap_json = format!(
            r#"{{"xds_servers":[{{"server_uri":"http://{proxy_addr}","channel_creds":[{{"type":"insecure"}}]}}],"node":{{"id":"test"}}}}"#
        );
        let bootstrap = BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap");
        let target = XdsUri::parse("xds:///my-service").expect("parse target");
        let _channel = XdsChannelBuilder::new(
            XdsChannelConfig::new(target)
                .with_bootstrap(bootstrap)
                .with_ads_keep_alive(Some(Duration::from_millis(200)), Duration::from_millis(200)),
        )
        .build_grpc_channel()
        .expect("build xds channel");

        tokio::time::timeout(Duration::from_secs(10), async {
            while !control_plane
                .get_service()
                .has_subscriber_for(&config::AdsTypeUrl::Lds, "my-service")
            {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("ADS stream never established");
        assert_eq!(*accepted.borrow(), 1);

        blackhole.cancel();

        tokio::time::timeout(Duration::from_secs(10), accepted.wait_for(|n| *n >= 2))
            .await
            .expect("half-open ADS connection never replaced; keepalive not applied")
            .expect("proxy stopped accepting");
    }

    /// gRFC A31: the listener's HCM `max_stream_duration` bounds calls whose
    /// route sets none, a route value overrides it (0 lifts it), the
    /// application's own deadline still applies on top, and the limit spans
    /// the whole call, retries included.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn xds_channel_e2e_max_stream_duration() {
        const LISTENER_MSD: Duration = Duration::from_millis(300);
        const ROUTE_MSD: Duration = Duration::from_millis(600);
        const SLOW_DELAY: Duration = Duration::from_secs(1);

        let backend = spawn_scripted_server("backend", SLOW_DELAY)
            .await
            .expect("spawn backend");
        let (control_plane, cp_addr) = start_control_plane().await;

        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "my-service".to_string(),
                rds_listener_with_max_stream_duration("my-service", "route-config", LISTENER_MSD),
            )]),
        );
        serve_route_config(
            &control_plane,
            config::build_route_config("route-config", "my-cluster"),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cluster("my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cla(
                    "my-cluster",
                    &[(backend.addr.ip().to_string(), backend.addr.port())],
                ),
            )]),
        );

        let channel = builder_for(cp_addr, "my-service")
            .build_grpc_channel()
            .expect("build xds channel");
        let mut client = GreeterClient::new(channel);
        say_hello_until_prefix(&mut client, "backend:").await;

        // The route sets no limit, so the listener's applies.
        let start = tokio::time::Instant::now();
        let status = say(&mut client, ScriptedGreeter::SLOW, None)
            .await
            .expect_err("listener msd");
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded, "{status:?}");
        assert!(start.elapsed() >= LISTENER_MSD, "{:?}", start.elapsed());

        // An explicit 0 on the route disables the listener's limit.
        let mut route_config = config::build_route_config("route-config", "my-cluster");
        route_action_mut(&mut route_config).max_stream_duration =
            Some(route_action::MaxStreamDuration {
                grpc_timeout_header_max: Some(ProtoDuration::default()),
                ..Default::default()
            });
        serve_route_config(&control_plane, route_config);
        say_until(&mut client, ScriptedGreeter::SLOW, Result::is_ok).await;

        // The application's deadline still bounds the call.
        let status = say(
            &mut client,
            ScriptedGreeter::SLOW,
            Some(Duration::from_millis(200)),
        )
        .await
        .expect_err("app deadline");
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded, "{status:?}");

        // The route's limit bounds the whole call, retry backoff included: a
        // failing call outlives the listener's limit but ends at the route's
        // with DEADLINE_EXCEEDED, long before its retries (whose backoff alone
        // is >= 1.2s) would run out and surface UNAVAILABLE.
        let mut route_config = config::build_route_config("route-config", "my-cluster");
        let action = route_action_mut(&mut route_config);
        action.max_stream_duration = Some(route_action::MaxStreamDuration {
            max_stream_duration: Some(to_proto(ROUTE_MSD)),
            ..Default::default()
        });
        action.retry_policy = Some(RetryPolicy {
            retry_on: "unavailable".to_string(),
            num_retries: Some(UInt32Value { value: 4 }),
            retry_back_off: Some(RetryBackOff {
                base_interval: Some(to_proto(Duration::from_millis(100))),
                max_interval: None,
            }),
            ..Default::default()
        });
        serve_route_config(&control_plane, route_config);
        let elapsed = say_until(
            &mut client,
            ScriptedGreeter::FAIL,
            |reply| matches!(reply, Err(status) if status.code() == tonic::Code::DeadlineExceeded),
        )
        .await;
        assert!(elapsed >= ROUTE_MSD, "{elapsed:?}");

        let _ = backend.shutdown.send(());
    }

    /// Sends a request named `name` to the [`ScriptedGreeter`], optionally with
    /// an application deadline.
    async fn say(
        client: &mut GreeterClient<XdsChannelGrpc>,
        name: &str,
        timeout: Option<Duration>,
    ) -> Result<String, tonic::Status> {
        let mut request = tonic::Request::new(HelloRequest {
            name: name.to_string(),
        });
        if let Some(timeout) = timeout {
            request.set_timeout(timeout);
        }
        Ok(client.say_hello(request).await?.into_inner().message)
    }

    /// Sends `name` until its reply satisfies `done` (xDS updates apply
    /// asynchronously), returning how long that call took.
    async fn say_until(
        client: &mut GreeterClient<XdsChannelGrpc>,
        name: &str,
        done: impl Fn(&Result<String, tonic::Status>) -> bool,
    ) -> Duration {
        let mut last = None;
        for _ in 0..50 {
            let start = tokio::time::Instant::now();
            let reply = say(client, name, None).await;
            if done(&reply) {
                return start.elapsed();
            }
            last = Some(reply);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        panic!("never observed the expected reply to {name:?}; last seen: {last:?}");
    }

    fn serve_route_config(control_plane: &RunningControlPlane, route_config: RouteConfiguration) {
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Rds,
            HashMap::from([(route_config.name.clone(), route_config)]),
        );
    }

    /// An RDS listener whose HCM sets `max_stream_duration`.
    fn rds_listener_with_max_stream_duration(
        name: &str,
        rds_name: &str,
        max_stream_duration: Duration,
    ) -> Listener {
        let mut listener = config::build_rds_listener(name, rds_name);
        let any = listener
            .api_listener
            .as_mut()
            .and_then(|api| api.api_listener.as_mut())
            .expect("api listener");
        let mut hcm = HttpConnectionManager::decode(any.value.as_slice()).expect("decode HCM");
        hcm.common_http_protocol_options = Some(HttpProtocolOptions {
            max_stream_duration: Some(to_proto(max_stream_duration)),
            ..Default::default()
        });
        any.value = hcm.encode_to_vec();
        listener
    }

    /// The action of the first route in `route_config`.
    fn route_action_mut(route_config: &mut RouteConfiguration) -> &mut RouteAction {
        let Some(Action::Route(action)) = route_config.virtual_hosts[0].routes[0].action.as_mut()
        else {
            panic!("expected a route action");
        };
        action
    }

    fn to_proto(duration: Duration) -> ProtoDuration {
        ProtoDuration {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    /// Waits until exactly `want` ADS streams are subscribed to LDS.
    async fn wait_for_lds_streams(control_plane: &RunningControlPlane, want: usize) {
        tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                let counts = control_plane.get_service().get_subscriber_counts();
                if counts.get(&config::AdsTypeUrl::Lds).copied().unwrap_or(0) == want {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap_or_else(|_| panic!("never observed {want} LDS streams"));
    }

    /// Channels built from one `XdsClientPool` share an ADS stream per key:
    /// by default per target (gRFC A71), or across targets under one explicit
    /// key. The stream closes once its last channel is dropped. Channels
    /// without a pool each have their own, so they share nothing.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn pooled_channels_share_ads_stream_per_key() {
        let backend = spawn_greeter_server("backend", None, None)
            .await
            .expect("spawn greeter backend");
        let backend_addr = backend.addr;

        let (control_plane, cp_addr) = start_control_plane().await;
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([
                (
                    "svc-a".to_string(),
                    config::build_inline_listener("svc-a", "my-cluster"),
                ),
                (
                    "svc-b".to_string(),
                    config::build_inline_listener("svc-b", "my-cluster"),
                ),
            ]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cluster("my-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "my-cluster".to_string(),
                config::build_cla(
                    "my-cluster",
                    &[(backend_addr.ip().to_string(), backend_addr.port())],
                ),
            )]),
        );

        let pool = XdsClientPool::builder()
            .with_bootstrap(bootstrap_for(cp_addr))
            .build()
            .expect("build pool");
        let pooled = |listener_name: &str| {
            let target = XdsUri::parse(&format!("xds:///{listener_name}")).expect("parse target");
            XdsChannelBuilder::new(XdsChannelConfig::new(target)).with_xds_client_pool(pool.clone())
        };
        let mut clients = Vec::new();

        // Channels for one target share a client; another target gets its own.
        for listener_name in ["svc-a", "svc-a", "svc-b"] {
            let channel = pooled(listener_name).build_grpc_channel().expect("build");
            clients.push(GreeterClient::new(channel));
        }
        for client in &mut clients {
            say_hello_until_prefix(client, "backend:").await;
        }
        wait_for_lds_streams(&control_plane, 2).await;

        drop(clients);
        wait_for_lds_streams(&control_plane, 0).await;

        // Channels for different targets share one client under one key.
        let mut clients = Vec::new();
        for listener_name in ["svc-a", "svc-b"] {
            let channel = pooled(listener_name)
                .with_xds_client_key("shared")
                .build_grpc_channel()
                .expect("build");
            clients.push(GreeterClient::new(channel));
        }
        for client in &mut clients {
            say_hello_until_prefix(client, "backend:").await;
        }
        wait_for_lds_streams(&control_plane, 1).await;

        drop(clients);
        wait_for_lds_streams(&control_plane, 0).await;

        let mut clients = Vec::new();
        for listener_name in ["svc-a", "svc-a"] {
            let channel = builder_for(cp_addr, listener_name)
                .with_xds_client_key("shared")
                .build_grpc_channel()
                .expect("build");
            clients.push(GreeterClient::new(channel));
        }
        for client in &mut clients {
            say_hello_until_prefix(client, "backend:").await;
        }
        wait_for_lds_streams(&control_plane, 2).await;

        let _ = backend.shutdown.send(());
    }

    /// Fetches the CSDS config of every reported xDS client.
    async fn csds_configs(
        csds: &mut ClientStatusDiscoveryServiceClient<tonic::transport::Channel>,
        exclude_resource_contents: bool,
    ) -> Vec<ClientConfig> {
        let request = ClientStatusRequest {
            exclude_resource_contents,
            ..Default::default()
        };
        csds.fetch_client_status(request)
            .await
            .expect("fetch client status")
            .into_inner()
            .config
    }

    /// CSDS reports the resources of an xDS client once, however many
    /// channels share it, and stops reporting it once the last of them is
    /// dropped.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn csds_reports_channel_resources() {
        let backend = spawn_greeter_server("backend", None, None)
            .await
            .expect("spawn greeter backend");
        let backend_addr = backend.addr;
        let (control_plane, cp_addr) = start_control_plane().await;
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "csds-service".to_string(),
                config::build_inline_listener("csds-service", "csds-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([(
                "csds-cluster".to_string(),
                config::build_cluster("csds-cluster"),
            )]),
        );
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([(
                "csds-cluster".to_string(),
                config::build_cla(
                    "csds-cluster",
                    &[(backend_addr.ip().to_string(), backend_addr.port())],
                ),
            )]),
        );

        // A private registry, so other tests' clients are not reported.
        let registry = Arc::new(CsdsRegistry::default());
        let pool = XdsClientPool::builder()
            .with_bootstrap(bootstrap_for(cp_addr))
            .with_csds_registry(Arc::clone(&registry))
            .build()
            .expect("build pool");
        let mut clients = Vec::new();
        for _ in 0..2 {
            let target = XdsUri::parse("xds:///csds-service").expect("parse target");
            let channel = XdsChannelBuilder::new(XdsChannelConfig::new(target))
                .with_xds_client_pool(pool.clone())
                .build_grpc_channel()
                .expect("build xds channel");
            clients.push(GreeterClient::new(channel));
        }
        for client in &mut clients {
            say_hello_until_prefix(client, "backend:").await;
        }

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let csds_addr = listener.local_addr().unwrap();
        tokio::spawn(
            tonic::transport::Server::builder()
                .add_service(CsdsService::with_registry(registry))
                .serve_with_incoming(TcpListenerStream::new(listener)),
        );
        let mut csds = ClientStatusDiscoveryServiceClient::connect(format!("http://{csds_addr}"))
            .await
            .expect("connect to CSDS");

        let mut configs = csds_configs(&mut csds, false).await;
        assert_eq!(configs.len(), 1, "the shared client is reported once");
        let config = configs.remove(0);
        assert_eq!(config.client_scope, "xds:///csds-service");
        assert_eq!(config.node.as_ref().expect("node is reported").id, "test");
        assert!(
            config
                .generic_xds_configs
                .iter()
                .all(|r| r.xds_config.is_some())
        );
        let without_contents = csds_configs(&mut csds, true).await;
        assert!(
            without_contents[0]
                .generic_xds_configs
                .iter()
                .all(|r| r.xds_config.is_none() && !r.version_info.is_empty())
        );
        let mut resources: Vec<(String, String, i32)> = config
            .generic_xds_configs
            .into_iter()
            .map(|r| (r.type_url, r.name, r.client_status))
            .collect();
        resources.sort();
        let acked = ClientResourceStatus::Acked as i32;
        assert_eq!(
            resources,
            [
                (
                    "type.googleapis.com/envoy.config.cluster.v3.Cluster".to_string(),
                    "csds-cluster".to_string(),
                    acked
                ),
                (
                    "type.googleapis.com/envoy.config.endpoint.v3.ClusterLoadAssignment"
                        .to_string(),
                    "csds-cluster".to_string(),
                    acked
                ),
                (
                    "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
                    "csds-service".to_string(),
                    acked
                ),
            ]
        );

        drop(clients);
        tokio::time::timeout(Duration::from_secs(10), async {
            while !csds_configs(&mut csds, false).await.is_empty() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("client of the dropped channels is still reported");

        let _ = backend.shutdown.send(());
    }
}

/// gRFC A65 end-to-end test
#[cfg(feature = "_tls-any")]
mod a65_ads_tls {
    use std::collections::HashMap;
    use std::io::Write;

    use tempfile::NamedTempFile;
    use tokio::time::Duration;
    use tonic::transport::{Certificate, Identity, ServerTlsConfig};
    use xds_test_util::RunningControlPlane;
    use xds_test_util::XdsTestControlPlaneService;
    use xds_test_util::config;

    use crate::BootstrapConfig;
    use crate::XdsChannelBuilder;
    use crate::XdsChannelConfig;
    use crate::XdsUri;

    /// The service the client under test subscribes to.
    const SUBJECT_SERVICE: &str = "my-service";

    struct TestCa {
        params: rcgen::CertificateParams,
        key: rcgen::KeyPair,
        pem: String,
    }

    fn gen_ca() -> TestCa {
        let key = rcgen::KeyPair::generate().unwrap();
        let mut params = rcgen::CertificateParams::new(vec!["a65-test-ca".into()]).unwrap();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::CrlSign,
        ];
        let pem = params.self_signed(&key).unwrap().pem();
        TestCa { params, key, pem }
    }

    /// Issue a leaf cert from `ca`. Returns `(cert_pem, key_pem)`.
    fn gen_leaf(
        ca: &TestCa,
        san: &str,
        purpose: rcgen::ExtendedKeyUsagePurpose,
    ) -> (String, String) {
        let key = rcgen::KeyPair::generate().unwrap();
        let mut params = rcgen::CertificateParams::new(vec![san.to_string()]).unwrap();
        params.extended_key_usages = vec![purpose];
        let issuer = rcgen::Issuer::from_params(&ca.params, &ca.key);
        let cert = params.signed_by(&key, &issuer).unwrap();
        (cert.pem(), key.serialize_pem())
    }

    fn write_temp(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    /// Pin a process-wide rustls crypto provider. With both `tls-ring` and
    /// `tls-aws-lc` compiled in, as under CI's `--all-features`, rustls cannot
    /// choose one on its own and panics.
    fn ensure_crypto_provider() {
        if rustls::crypto::CryptoProvider::get_default().is_some() {
            return;
        }
        #[cfg(feature = "tls-ring")]
        let _ = rustls::crypto::ring::default_provider().install_default();
        #[cfg(all(not(feature = "tls-ring"), feature = "tls-aws-lc"))]
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    }

    /// Start an ADS control plane that serves TLS and requires client certs
    /// signed by `ca`.
    async fn start_mtls_control_plane(ca: &TestCa) -> RunningControlPlane {
        ensure_crypto_provider();
        let (server_cert, server_key) =
            gen_leaf(ca, "127.0.0.1", rcgen::ExtendedKeyUsagePurpose::ServerAuth);
        let tls = ServerTlsConfig::new()
            .identity(Identity::from_pem(server_cert, server_key))
            .client_ca_root(Certificate::from_pem(ca.pem.clone()));

        let running = XdsTestControlPlaneService::new()
            .start_with_tls(tls)
            .await
            .expect("start TLS control plane");

        running.get_service().set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                SUBJECT_SERVICE.to_string(),
                config::build_inline_listener(SUBJECT_SERVICE, "my-cluster"),
            )]),
        );
        running
    }

    /// Build a channel whose bootstrap requests A65 TLS channel credentials.
    fn build_channel_with_tls_creds(
        control_plane: &RunningControlPlane,
        ca_file: &NamedTempFile,
        cert_file: &NamedTempFile,
        key_file: &NamedTempFile,
    ) -> Result<crate::XdsChannelGrpc, crate::BuildError> {
        let bootstrap_json = serde_json::json!({
            "xds_servers": [{
                "server_uri": control_plane.addr().to_string(),
                "channel_creds": [{"type": "tls", "config": {
                    "ca_certificate_file": ca_file.path().to_str().unwrap(),
                    "certificate_file": cert_file.path().to_str().unwrap(),
                    "private_key_file": key_file.path().to_str().unwrap(),
                }}],
            }],
            "node": {"id": "a65-test"},
        })
        .to_string();

        let bootstrap = BootstrapConfig::from_json(&bootstrap_json).expect("parse bootstrap");
        let target = XdsUri::parse(&format!("xds:///{SUBJECT_SERVICE}")).expect("parse target");
        XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap))
            .build_grpc_channel()
    }

    /// Poll until `SUBJECT_SERVICE` appears as an LDS subscriber, i.e. the
    /// client's ADS stream completed its TLS handshake. `timeout` is a failure
    /// ceiling, not a wait. Returns `false` on timeout.
    async fn ads_stream_established(
        control_plane: &RunningControlPlane,
        timeout: Duration,
    ) -> bool {
        let deadline = tokio::time::Instant::now() + timeout;
        while tokio::time::Instant::now() < deadline {
            if control_plane
                .get_service()
                .has_subscriber_for(&config::AdsTypeUrl::Lds, SUBJECT_SERVICE)
            {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        false
    }

    /// Bootstrap names a CA plus a client identity, and the mTLS-demanding
    /// control plane accepts the stream.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn bootstrap_tls_creds_are_used_for_the_ads_connection() {
        let ca = gen_ca();
        let control_plane = start_mtls_control_plane(&ca).await;

        let ca_file = write_temp(&ca.pem);
        let (client_cert, client_key) = gen_leaf(
            &ca,
            "a65-client",
            rcgen::ExtendedKeyUsagePurpose::ClientAuth,
        );
        let cert_file = write_temp(&client_cert);
        let key_file = write_temp(&client_key);

        let _channel =
            build_channel_with_tls_creds(&control_plane, &ca_file, &cert_file, &key_file)
                .expect("build xds channel");

        assert!(
            ads_stream_established(&control_plane, Duration::from_secs(10)).await,
            "ADS stream never established; the bootstrap CA and client identity \
             were not used for the control plane connection",
        );
    }
}
