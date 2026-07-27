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

use crate::common::async_util::BoxFuture;
use crate::xds::resource::route_config::RouteConfigMetadata;
use crate::xds::routing::RoutingError;
use http::Request;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{BoxError, Layer, Service};

/// Represents the input for routing decisions.
#[allow(dead_code)]
pub(crate) struct RouteInput<'a> {
    /// The authority (host) of the request URI.
    pub authority: &'a str,
    /// The HTTP headers of the request. These can be used for header-based routing decisions.
    pub headers: &'a http::HeaderMap,
}

/// Represents the routing decision made by the routing layer.
#[derive(Clone, Debug)]
pub(crate) struct RouteDecision {
    /// The name of the cluster to which the request should be routed.
    pub cluster: String,
    /// The request hash computed from the route's hash policies (gRFC A42),
    /// consumed by the ring-hash LB picker. `None` when no hash policy produced
    /// a hash, in which case the picker falls back to a random hash.
    // Populated by the routing layer; consumed by the ring-hash picker (later PR).
    #[allow(dead_code)]
    pub request_hash: Option<u64>,
}

/// A hook that runs before xDS route selection.
///
/// The interceptor may mutate the request headers using the route
/// configuration's [`RouteConfigMetadata`]. Because it runs before routing, its
/// mutations are visible to route matching — enabling config-driven request
/// transformation (e.g. computing a partition/shard key and injecting a routing
/// header the router then matches on). It cannot otherwise influence routing.
pub trait PreRouteInterceptor: Send + Sync + 'static {
    /// Inspects and optionally mutates `headers` using the route-config `metadata`.
    fn on_request(&self, headers: &mut http::HeaderMap, metadata: &RouteConfigMetadata);
}

/// Trait for routing requests to clusters.
///
/// Implementations resolve a request's authority and headers into a target
/// cluster name. The xDS-backed implementation is
/// [`XdsRouter`](crate::xds::routing::XdsRouter).
pub(crate) trait Router: Send + Sync + 'static {
    fn route(&self, input: &RouteInput<'_>) -> BoxFuture<Result<RouteDecision, RoutingError>>;

    /// Current route-config metadata, if available, used to feed a
    /// [`PreRouteInterceptor`]. Defaults to `None` (e.g. for mock routers).
    fn metadata(&self) -> Option<RouteConfigMetadata> {
        None
    }
}

/// Tower service for routing requests to the appropriate cluster.
/// Attaches routing decision as [`RouteDecision`] to the request extensions.
/// The [`RouteDecision`] will be used by the `XdsLbService` to identify the
/// cluster to which the request should be routed.
#[derive(Clone)]
pub(crate) struct XdsRoutingService<S> {
    /// The inner Tower service to which the request will be forwarded after routing decision is made.
    inner: S,
    /// The router used to make routing decisions based on the request.
    router: Arc<dyn Router>,
    /// Optional hook run before routing; may mutate request headers.
    interceptor: Option<Arc<dyn PreRouteInterceptor>>,
    /// Channel-level authority used as the routing key.
    authority: Arc<str>,
}

impl<S, B> Service<Request<B>> for XdsRoutingService<S>
where
    S: Service<Request<B>, Error: Into<BoxError>> + Clone + Send + 'static,
    B: Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        let router = self.router.clone();
        let interceptor = self.interceptor.clone();
        let authority = self.authority.clone();
        let mut inner_service = self.inner.clone();
        Box::pin(async move {
            if let Some(interceptor) = interceptor.as_ref()
                && let Some(metadata) = router.metadata()
            {
                interceptor.on_request(request.headers_mut(), &metadata);
            }
            let headers = &request.headers();
            let route_input = RouteInput {
                authority: &authority,
                headers,
            };
            let route_decision = router.route(&route_input).await?;
            request.extensions_mut().insert(route_decision);
            inner_service.call(request).await.map_err(Into::into)
        })
    }
}

/// Tower layer for routing requests to the appropriate cluster.
#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct XdsRoutingLayer {
    router: Arc<dyn Router>,
    interceptor: Option<Arc<dyn PreRouteInterceptor>>,
    authority: Arc<str>,
}

impl XdsRoutingLayer {
    /// Creates a new `XdsRoutingLayer` with the given [`Router`], optional
    /// pre-route interceptor, and authority.
    ///
    /// `authority` is the routing key matched against `VirtualHost.domains`
    /// in RDS. It should be the endpoint portion of the xDS target.
    #[allow(dead_code)]
    pub(crate) fn new(
        router: Arc<dyn Router>,
        interceptor: Option<Arc<dyn PreRouteInterceptor>>,
        authority: Arc<str>,
    ) -> Self {
        Self {
            router,
            interceptor,
            authority,
        }
    }
}

impl<S> Layer<S> for XdsRoutingLayer {
    type Service = XdsRoutingService<S>;

    fn layer(&self, service: S) -> Self::Service {
        XdsRoutingService {
            inner: service,
            router: self.router.clone(),
            interceptor: self.interceptor.clone(),
            authority: self.authority.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tower::ServiceExt;
    use tower::service_fn;

    /// Mock router that records the `authority` it was called with.
    struct CaptureAuthorityRouter {
        captured: Arc<Mutex<Option<String>>>,
    }

    impl Router for CaptureAuthorityRouter {
        fn route(&self, input: &RouteInput<'_>) -> BoxFuture<Result<RouteDecision, RoutingError>> {
            *self.captured.lock().unwrap() = Some(input.authority.to_string());
            Box::pin(async move {
                Ok(RouteDecision {
                    cluster: "test-cluster".to_string(),
                    request_hash: None,
                })
            })
        }
    }

    /// Verifies the routing layer always sources `authority` from its layer
    /// config, not from the request URI.
    #[tokio::test]
    async fn uses_layer_authority_regardless_of_request_uri() {
        let captured = Arc::new(Mutex::new(None));
        let router: Arc<dyn Router> = Arc::new(CaptureAuthorityRouter {
            captured: captured.clone(),
        });
        let layer = XdsRoutingLayer::new(router, None, Arc::from("greeter.svc:50051"));

        let inner =
            service_fn(
                |_req: Request<()>| async move { Ok::<_, BoxError>(http::Response::new(())) },
            );
        let svc = layer.layer(inner);

        // Case 1: request with no authority on the URI (typical tonic-generated
        // client — see `tonic/src/client/grpc.rs::prepare_request`).
        let req = Request::builder()
            .uri("/pkg.Greeter/SayHello")
            .body(())
            .unwrap();
        svc.clone().oneshot(req).await.unwrap();
        assert_eq!(
            captured.lock().unwrap().as_deref(),
            Some("greeter.svc:50051"),
        );

        // Case 2: request with a different authority on the URI — the layer
        // must still use its own configured authority.
        *captured.lock().unwrap() = None;
        let req = Request::builder()
            .uri("http://other.example:443/pkg.Greeter/SayHello")
            .body(())
            .unwrap();
        svc.oneshot(req).await.unwrap();
        assert_eq!(
            captured.lock().unwrap().as_deref(),
            Some("greeter.svc:50051"),
        );
    }

    /// End-to-end check of the pre-route seam: the interceptor reads route-config
    /// metadata and injects a partition header, which the router then matches.
    #[tokio::test]
    async fn pre_route_interceptor_drives_partition_selection() {
        use crate::xds::cache::XdsCache;
        use crate::xds::resource::route_config::{
            HeaderMatchSpecifierConfig, HeaderMatcherConfig, PathSpecifierConfig, RouteConfig,
            RouteConfigAction, RouteConfigMatch, RouteConfigResource, VirtualHostConfig,
        };
        use crate::xds::routing::XdsRouter;
        use envoy_types::pb::envoy::config::core::v3::Metadata;
        use envoy_types::pb::google::protobuf::{Any, Struct};

        fn partition_route(partition: i64, cluster: &str) -> RouteConfig {
            RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::Prefix("/".into()),
                    headers: vec![HeaderMatcherConfig {
                        name: "x-partition".into(),
                        match_specifier: HeaderMatchSpecifierConfig::Range {
                            start: partition,
                            end: partition + 1,
                        },
                        invert_match: false,
                    }],
                    case_sensitive: true,
                    match_fraction: None,
                },
                action: RouteConfigAction::Cluster(cluster.into()),
            }
        }

        // Config carries both untyped and typed metadata the interceptor sees.
        let mut filter_metadata = std::collections::HashMap::new();
        filter_metadata.insert("partitioning".to_string(), Struct::default());
        let mut typed_filter_metadata = std::collections::HashMap::new();
        typed_filter_metadata.insert(
            "partitioning-typed".to_string(),
            Any {
                type_url: "type.example/Partitioning".to_string(),
                value: vec![1, 2, 3],
            },
        );
        let metadata = RouteConfigMetadata::from_proto(Metadata {
            filter_metadata,
            typed_filter_metadata,
        });

        let cache = XdsCache::new();
        cache.update_route_config(Arc::new(RouteConfigResource {
            name: "rc".into(),
            virtual_hosts: vec![VirtualHostConfig {
                name: "vh".into(),
                domains: vec!["*".into()],
                routes: vec![
                    partition_route(1, "cluster-p1"),
                    partition_route(2, "cluster-p2"),
                ],
            }],
            metadata,
        }));
        let router: Arc<dyn Router> = Arc::new(XdsRouter::new(&cache));
        tokio::task::yield_now().await;

        /// Reads the `hint` header, verifies metadata delivery, and injects the
        /// partition header the router selects on.
        struct PartitionInterceptor;
        impl PreRouteInterceptor for PartitionInterceptor {
            fn on_request(&self, headers: &mut http::HeaderMap, metadata: &RouteConfigMetadata) {
                assert!(
                    metadata.filter_metadata("partitioning").is_some(),
                    "interceptor must see the untyped metadata",
                );
                let typed = metadata
                    .typed_filter_metadata("partitioning-typed")
                    .expect("interceptor must see the typed metadata");
                assert_eq!(typed.type_url, "type.example/Partitioning");
                assert_eq!(typed.value.as_ref(), [1, 2, 3]);
                let partition = match headers.get("hint").and_then(|v| v.to_str().ok()) {
                    Some("a") => "1",
                    _ => "2",
                };
                headers.insert("x-partition", partition.parse().unwrap());
            }
        }

        let captured: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let terminal = {
            let captured = captured.clone();
            service_fn(move |req: Request<()>| {
                let captured = captured.clone();
                async move {
                    let cluster = req
                        .extensions()
                        .get::<RouteDecision>()
                        .map(|d| d.cluster.clone());
                    *captured.lock().unwrap() = cluster;
                    Ok::<_, BoxError>(http::Response::new(()))
                }
            })
        };

        let interceptor: Arc<dyn PreRouteInterceptor> = Arc::new(PartitionInterceptor);
        let svc = XdsRoutingLayer::new(router, Some(interceptor), Arc::from("svc")).layer(terminal);

        // hint "a" -> partition 1 -> cluster-p1
        let req = Request::builder().header("hint", "a").body(()).unwrap();
        svc.clone().oneshot(req).await.unwrap();
        assert_eq!(captured.lock().unwrap().as_deref(), Some("cluster-p1"));

        // hint "b" -> partition 2 -> cluster-p2
        let req = Request::builder().header("hint", "b").body(()).unwrap();
        svc.oneshot(req).await.unwrap();
        assert_eq!(captured.lock().unwrap().as_deref(), Some("cluster-p2"));
    }
}
