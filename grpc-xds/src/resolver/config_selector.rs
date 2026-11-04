/*
 *
 * Copyright 2026 gRPC authors.
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

//! The xDS config selector, which routes each RPC to a cluster (gRFC A28).

use std::fmt::Debug;
use std::sync::Arc;

use grpc::__unstable::client::load_balancing::sampling::WeightedSampler;
use grpc::__unstable::client::name_resolution::config_selector::ConfigSelector;
use grpc::__unstable::client::name_resolution::config_selector::DynIntercept;
use grpc::__unstable::client::name_resolution::config_selector::RpcConfig;
use grpc::__unstable::client::name_resolution::config_selector::SelectConfigOptions;
use grpc::StatusCodeError;
use grpc::StatusError;
use grpc::async_trait;
use grpc::client::CallOptions;
use grpc::client::DynInvoke;
use grpc::client::DynRecvStream;
use grpc::client::DynSendStream;
use grpc::client::RequestHeaders;

use crate::resolver::PickedCluster;
use crate::resource::RouteMatch;

/// The interceptor for the RPCs routed to a cluster. Stores the cluster's
/// [`PickedCluster`] in each RPC's `CallOptions`, which tells the cluster
/// manager picker which child to pick from.
///
/// A config selector has one per cluster, shared by every route that names
/// the cluster, so the selector holds a ref to each cluster its routes can
/// send RPCs to. Each RPC holds the interceptor of the cluster it was routed
/// to, through its `RpcConfig`, until the channel's `invoke` for the RPC
/// returns, i.e. until the RPC is committed. So an RPC keeps only its own
/// cluster in use.
#[derive(Debug)]
pub(super) struct ClusterInterceptor {
    picked: Arc<PickedCluster>,
}

impl ClusterInterceptor {
    /// Creates the interceptor for the cluster whose cluster manager child is
    /// named `child_name`, which keeps `cluster_ref` alive.
    pub(super) fn new(child_name: &str, cluster_ref: Arc<()>) -> Self {
        Self {
            picked: Arc::new(PickedCluster {
                child_name: child_name.to_owned(),
                _cluster_ref: cluster_ref,
            }),
        }
    }
}

#[async_trait]
impl DynIntercept for ClusterInterceptor {
    async fn dyn_intercept(
        &self,
        headers: RequestHeaders,
        mut options: CallOptions,
        next: Arc<dyn DynInvoke>,
    ) -> (Box<dyn DynSendStream>, Box<dyn DynRecvStream>) {
        options.set_option(self.picked.clone());
        next.dyn_invoke(headers, options).await
    }
}

/// Routes each RPC to a cluster using the virtual host's routes (gRFC A28).
#[derive(Debug)]
pub(super) struct XdsConfigSelector {
    /// In the virtual host's order; the first matching route wins.
    routes: Vec<SelectorRoute>,
}

/// A route of an [`XdsConfigSelector`].
#[derive(Debug)]
pub(super) struct SelectorRoute {
    pub(super) route_match: RouteMatch,
    /// The clusters the route sends the RPCs that match it to, by weight. A
    /// route to a single cluster has one entry.
    pub(super) clusters: WeightedSampler<Arc<ClusterInterceptor>>,
}

impl XdsConfigSelector {
    pub(super) fn new(routes: Vec<SelectorRoute>) -> Self {
        Self { routes }
    }
}

impl ConfigSelector for XdsConfigSelector {
    fn select_config(&self, options: SelectConfigOptions<'_>) -> Result<RpcConfig, StatusError> {
        let headers = options.request_headers;
        let route = self
            .routes
            .iter()
            .find(|route| {
                route
                    .route_match
                    .is_match(headers.method_name(), headers.metadata())
            })
            .ok_or_else(|| {
                StatusError::new(StatusCodeError::Unavailable, "no matching route found")
            })?;
        let cluster = route.clusters.next().ok_or_else(|| {
            // Validation rejects routes whose clusters have no total weight.
            debug_assert!(false, "route has no cluster with a non-zero weight");
            StatusError::new(
                StatusCodeError::Internal,
                "route has no cluster with a non-zero weight",
            )
        })?;
        Ok(RpcConfig::default().with_interceptor(cluster.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use grpc::__unstable::client::load_balancing::sampling::WeightedSamplerBuilder;
    use grpc::client::RecvStream;
    use grpc::client::ResponseStreamItem;
    use grpc::client::SendOptions;
    use grpc::client::SendStream;
    use grpc::core::RecvMessage;
    use grpc::core::SendMessage;
    use grpc::core::StreamEnded;
    use grpc::metadata::MetadataMap;

    use super::*;
    use crate::resolver::picked_child_name;
    use crate::resource::HeaderMatchSpecifier;
    use crate::resource::HeaderMatcher;
    use crate::resource::PathSpecifier;
    use crate::resource::StringMatcher;

    const METHOD: &str = "/pkg.Service/Method";

    /// The streams of an RPC that ended without a response.
    struct ClosedStream;

    impl SendStream for ClosedStream {
        async fn send(
            &mut self,
            _msg: &dyn SendMessage,
            _options: SendOptions,
        ) -> Result<(), StreamEnded> {
            Err(StreamEnded)
        }
    }

    impl RecvStream for ClosedStream {
        async fn recv(&mut self, _msg: &mut dyn RecvMessage) -> ResponseStreamItem {
            ResponseStreamItem::StreamClosed
        }
    }

    /// The end of an interceptor chain. Records the cluster manager child
    /// picked for each RPC that reaches it.
    #[derive(Default)]
    struct RecordingInvoker {
        /// The child picked for each RPC, or `None` if the RPC's call options
        /// have no pick.
        picked_children: Mutex<Vec<Option<String>>>,
    }

    #[async_trait]
    impl DynInvoke for RecordingInvoker {
        async fn dyn_invoke(
            &self,
            _headers: RequestHeaders,
            options: CallOptions,
        ) -> (Box<dyn DynSendStream>, Box<dyn DynRecvStream>) {
            let picked = picked_child_name(&options).map(str::to_owned);
            self.picked_children.lock().unwrap().push(picked);
            (Box::new(ClosedStream), Box::new(ClosedStream))
        }
    }

    /// Returns the interceptor for the cluster whose cluster manager child is
    /// named `child_name`, with a cluster ref that nothing tracks.
    fn cluster(child_name: &str) -> Arc<ClusterInterceptor> {
        Arc::new(ClusterInterceptor::new(child_name, Arc::new(())))
    }

    /// Returns a route match for paths starting with `prefix`.
    fn prefix_match(prefix: &str) -> RouteMatch {
        RouteMatch {
            path_specifier: PathSpecifier::Prefix(prefix.to_string()),
            headers: Vec::new(),
            case_sensitive: true,
            match_fraction: None,
        }
    }

    /// Returns a route that sends the RPCs that match `route_match` to
    /// `clusters`, which are `(cluster, weight)` pairs.
    fn route(
        route_match: RouteMatch,
        clusters: impl IntoIterator<Item = (Arc<ClusterInterceptor>, u32)>,
    ) -> SelectorRoute {
        SelectorRoute {
            route_match,
            clusters: WeightedSamplerBuilder::default().build(clusters),
        }
    }

    /// Returns the headers of an RPC to `METHOD` with `metadata`.
    fn headers(metadata: MetadataMap) -> RequestHeaders {
        RequestHeaders::new()
            .with_method_name(METHOD)
            .with_metadata(metadata)
    }

    /// Selects the config for an RPC to `METHOD` without metadata.
    fn select(selector: &XdsConfigSelector) -> Result<RpcConfig, StatusError> {
        selector.select_config(SelectConfigOptions::new(&headers(MetadataMap::new())))
    }

    /// Starts an RPC with `headers` as a channel would: selects its config
    /// with `selector` and runs it through the config's interceptor. Returns
    /// the cluster manager child the RPC was routed to, or the status the
    /// selector failed the RPC with.
    async fn route_rpc(
        selector: &XdsConfigSelector,
        headers: RequestHeaders,
    ) -> Result<String, StatusError> {
        let config = selector.select_config(SelectConfigOptions::new(&headers))?;
        let interceptor = config
            .interceptor
            .expect("selector should set an interceptor");
        let invoker = Arc::new(RecordingInvoker::default());
        interceptor
            .dyn_intercept(headers, CallOptions::new(), invoker.clone())
            .await;
        let picked_children = invoker.picked_children.lock().unwrap().clone();
        match picked_children.as_slice() {
            [Some(child)] => Ok(child.clone()),
            other => panic!("expected the RPC to reach the invoker with a pick, got {other:?}"),
        }
    }

    /// Returns the child picked for an RPC to `METHOD` without metadata.
    /// Panics if the selector fails the RPC.
    async fn picked_child(selector: &XdsConfigSelector) -> String {
        route_rpc(selector, headers(MetadataMap::new()))
            .await
            .expect("selector should pick a cluster")
    }

    #[tokio::test]
    async fn first_matching_route_wins() {
        let selector = XdsConfigSelector::new(vec![
            route(prefix_match("/other."), [(cluster("x"), 1)]),
            route(prefix_match("/pkg."), [(cluster("a"), 1)]),
            route(prefix_match(""), [(cluster("b"), 1)]),
        ]);

        assert_eq!(picked_child(&selector).await, "a");
    }

    #[tokio::test]
    async fn route_matches_on_headers() {
        let mut canary_match = prefix_match("");
        canary_match.headers.push(HeaderMatcher {
            name: "x-env".to_string(),
            match_specifier: HeaderMatchSpecifier::String(StringMatcher::exact("canary", false)),
            invert_match: false,
        });
        let selector = XdsConfigSelector::new(vec![
            route(canary_match, [(cluster("canary"), 1)]),
            route(prefix_match(""), [(cluster("prod"), 1)]),
        ]);

        let mut metadata = MetadataMap::new();
        metadata.insert("x-env", "canary".parse().unwrap());
        assert_eq!(
            route_rpc(&selector, headers(metadata)).await.unwrap(),
            "canary"
        );

        assert_eq!(picked_child(&selector).await, "prod");
    }

    #[test]
    fn no_matching_route_fails_unavailable() {
        let selector =
            XdsConfigSelector::new(vec![route(prefix_match("/other."), [(cluster("a"), 1)])]);

        let Err(status) = select(&selector) else {
            panic!("selector should fail the RPC");
        };

        assert_eq!(status.code(), StatusCodeError::Unavailable);
        assert_eq!(status.message(), "no matching route found");
    }

    #[tokio::test]
    async fn weighted_clusters_skip_zero_weights() {
        let selector = XdsConfigSelector::new(vec![route(
            prefix_match(""),
            [(cluster("a"), 0), (cluster("b"), 3)],
        )]);

        for _ in 0..3 {
            assert_eq!(picked_child(&selector).await, "b");
        }
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "route has no cluster with a non-zero weight")
    )]
    fn weighted_clusters_without_weight_fail_internal() {
        let selector = XdsConfigSelector::new(vec![route(
            prefix_match(""),
            [(cluster("a"), 0), (cluster("b"), 0)],
        )]);

        let Err(status) = select(&selector) else {
            panic!("selector should fail the RPC");
        };

        assert_eq!(status.code(), StatusCodeError::Internal);
        assert_eq!(
            status.message(),
            "route has no cluster with a non-zero weight"
        );
    }

    #[test]
    fn rpc_holds_only_picked_cluster() {
        let a_ref = Arc::new(());
        let b_ref = Arc::new(());
        let a = Arc::new(ClusterInterceptor::new("a", a_ref.clone()));
        let b = Arc::new(ClusterInterceptor::new("b", b_ref.clone()));
        // Weak, so the test can count the interceptors' holders without being
        // one.
        let a_interceptor = Arc::downgrade(&a);
        let b_interceptor = Arc::downgrade(&b);
        let selector = XdsConfigSelector::new(vec![
            route(prefix_match("/pkg."), [(a, 1)]),
            route(prefix_match(""), [(b, 1)]),
        ]);
        // Each interceptor is held by the selector, and holds its cluster's
        // ref, which the test also holds.
        assert_eq!(a_interceptor.strong_count(), 1);
        assert_eq!(b_interceptor.strong_count(), 1);
        assert_eq!(Arc::strong_count(&a_ref), 2);
        assert_eq!(Arc::strong_count(&b_ref), 2);

        // The channel holds an RPC's config until the RPC's `invoke` returns.
        // Both RPCs match the "/pkg." route, so each config holds "a"'s
        // interceptor.
        let rpc_config1 = select(&selector).unwrap();
        let rpc_config2 = select(&selector).unwrap();
        assert_eq!(a_interceptor.strong_count(), 3);
        assert_eq!(b_interceptor.strong_count(), 1);

        // Dropping the selector releases "b", but the RPCs keep "a".
        drop(selector);
        assert_eq!(a_interceptor.strong_count(), 2);
        assert_eq!(b_interceptor.strong_count(), 0);
        assert_eq!(Arc::strong_count(&a_ref), 2);
        assert_eq!(Arc::strong_count(&b_ref), 1);

        // "a" stays in use until the last RPC releases its config.
        drop(rpc_config1);
        assert_eq!(a_interceptor.strong_count(), 1);
        assert_eq!(Arc::strong_count(&a_ref), 2);
        drop(rpc_config2);
        assert_eq!(a_interceptor.strong_count(), 0);
        assert_eq!(Arc::strong_count(&a_ref), 1);
    }
}
