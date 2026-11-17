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

use crate::client::cluster::{ClusterChannel, ClusterChannelFactory};
use crate::client::route::RouteDecision;
use crate::common::async_util::BoxFuture;
use crate::xds::cache::ClusterWatches;
use http::Request;
use std::hash::Hash;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::ServiceExt;
use tower::{BoxError, Service, discover::Change, load::Load};

/// A pinned, boxed stream of endpoint changes for Tower's `Discover`-based
/// load balancers.
pub(crate) type BoxDiscover<Endpoint, S> =
    Pin<Box<dyn futures_core::Stream<Item = Result<Change<Endpoint, S>, BoxError>> + Send>>;

/// Trait for discovering cluster endpoints.
///
/// Implementations resolve a cluster and its router-owned [`ClusterWatches`]
/// into a stream of endpoint changes (`Change::Insert` / `Change::Remove`). The
/// xDS-backed implementation is
/// [`XdsClusterDiscovery`](crate::xds::cluster_discovery::XdsClusterDiscovery).
pub(crate) trait ClusterDiscovery<Endpoint, S>: Send + Sync + 'static {
    fn discover_cluster(
        &self,
        cluster_name: &str,
        watches: ClusterWatches,
    ) -> BoxDiscover<Endpoint, S>;
}

/// Errors that can occur during load balancing.
#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum LoadBalancingError {
    #[error("No routing decision extension from the routing layer available")]
    NoRoutingDecision,
    /// The routing decision carries no client for its cluster. The xDS router
    /// attaches one for every cluster in the route config, so this indicates a
    /// router that provisions no clients.
    #[error("routing decision carries no client for cluster '{0}'")]
    NoClusterClient(String),
}

/// A Tower Service that sends each request to the cluster client carried by its
/// [`RouteDecision`].
pub(crate) struct XdsLbService<Req, Resp> {
    _types: PhantomData<fn(Req) -> Resp>,
}

impl<Req, Resp> XdsLbService<Req, Resp> {
    pub(crate) fn new() -> Self {
        Self {
            _types: PhantomData,
        }
    }
}

impl<Req, Resp> Clone for XdsLbService<Req, Resp> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<B, Resp> Service<Request<B>> for XdsLbService<Request<B>, Resp>
where
    Request<B>: Send + 'static,
    Resp: Send + 'static,
{
    type Response = Resp;
    type Error = BoxError;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Under xDS, the destination cluster is decided by the routing layer, which takes
        // the request as an input. Therefore, we cannot determine readiness without
        // knowing the target cluster, which is tied to the request.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        let Some(decision) = request.extensions_mut().get_mut::<RouteDecision>() else {
            return Box::pin(async move { Err(LoadBalancingError::NoRoutingDecision.into()) });
        };
        // Taken rather than cloned so the request, once queued in the cluster's
        // channel, does not keep that channel's client alive. Each retry attempt
        // starts from the retry layer's copy of the request, which still has it.
        let cluster_ref = decision.cluster_ref.take();
        let Some(cluster_client) = cluster_ref
            .as_ref()
            .and_then(|cluster| cluster.client::<Request<B>, Resp>())
        else {
            let err = LoadBalancingError::NoClusterClient(decision.cluster.clone());
            return Box::pin(async move { Err(err.into()) });
        };

        // The actual load-balancing will be performed by the cluster's balancer.
        let mut channel = cluster_client.channel();

        Box::pin(async move {
            // This will block until the first endpoint is available.
            channel.ready().await?;
            channel.call(request).await
        })
    }
}

/// [`ClusterChannelFactory`] backed by a [`ClusterDiscovery`], balancing each
/// cluster's endpoints with Tower's P2C balancer.
pub(crate) struct TowerP2cClusterChannelFactory<Endpoint, S> {
    discovery: Arc<dyn ClusterDiscovery<Endpoint, S>>,
}

impl<Endpoint, S> TowerP2cClusterChannelFactory<Endpoint, S> {
    pub(crate) fn new(discovery: Arc<dyn ClusterDiscovery<Endpoint, S>>) -> Self {
        Self { discovery }
    }
}

impl<Req, Endpoint, S> ClusterChannelFactory<Req, S::Response>
    for TowerP2cClusterChannelFactory<Endpoint, S>
where
    Req: Send + 'static,
    Endpoint: Hash + Eq + Clone + Send + 'static,
    S: Service<Req> + Load + Send + 'static,
    S::Response: Send + 'static,
    S::Error: Into<BoxError>,
    S::Future: Send + 'static,
    <S as Load>::Metric: std::fmt::Debug,
{
    fn make(
        &self,
        cluster_name: &str,
        watches: ClusterWatches,
    ) -> ClusterChannel<Req, S::Response> {
        ClusterChannel::from_discover(self.discovery.discover_cluster(cluster_name, watches))
    }
}
