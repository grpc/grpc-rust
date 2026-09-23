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

//! Converts snapshot-based endpoint updates into incremental [`Change`]
//! streams for Tower's load balancing infrastructure.
//!
//! Each update carries the complete address set for a cluster; this module
//! diffs consecutive updates and produces `Change::Insert` / `Change::Remove`
//! events that Tower's P2C balancer (or any other `Discover`-based balancer)
//! can consume.
//!
//! Which addresses belong in an update is the caller's decision. The xDS
//! client passes the endpoints that gRFC A27 considers usable; a caller with
//! different rules passes a different set.

use std::collections::HashSet;
use std::sync::Arc;

use arc_swap::ArcSwap;
use futures_core::Stream;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::ReceiverStream;
use tower::BoxError;
use tower::discover::Change;

use crate::client::endpoint::{Connector, EndpointAddress};
use crate::client::lb::BoxDiscover;
use crate::xds::cache::CacheWatch;
use crate::xds::resource::EndpointsResource;

/// Buffer capacity for the endpoint change channel between the diff loop
/// and Tower's load balancer.
const ENDPOINT_CHANNEL_CAPACITY: usize = 64;

/// An atomically-swappable [`Connector`] held by an [`EndpointManager`].
///
/// The diff loop calls `load_full()` on every new endpoint so each connection
/// picks up the latest connector. Existing endpoint channels keep their
/// `EndpointChannel` instance (and any in-flight TLS session) — only
/// freshly-discovered endpoints see a swapped value.
pub(crate) type ConnectorSwap<S> = Arc<ArcSwap<Arc<dyn Connector<Service = S> + Send + Sync>>>;

/// A [`Connector`] an [`EndpointManager`] may be given.
pub type SharedConnector<S> = Arc<dyn Connector<Service = S> + Send + Sync>;

/// Converts endpoint updates into incremental [`Change`] streams.
///
/// `EndpointManager` is a pure diff-and-connect component: the caller supplies
/// a stream of address sets and gets back the changes between consecutive
/// sets, with each new address already connected.
pub struct EndpointManager<S: Send + 'static> {
    connector: ConnectorSwap<S>,
}

impl<S: Send + 'static> std::fmt::Debug for EndpointManager<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EndpointManager").finish_non_exhaustive()
    }
}

impl<S: Send + 'static> EndpointManager<S> {
    /// Builds a manager that connects through `connector`.
    pub fn new(connector: SharedConnector<S>) -> Self {
        Self {
            connector: Arc::new(ArcSwap::from_pointee(connector)),
        }
    }

    /// Replaces the connector used for endpoints discovered from now on.
    ///
    /// A caller rebuilds the connector when the cluster's CDS resource changes.
    /// Endpoints already discovered keep the channels they were opened with.
    pub fn set_connector(&self, connector: SharedConnector<S>) {
        self.connector.store(Arc::new(connector));
    }

    /// Returns a stream of endpoint changes for the given address sets.
    ///
    /// Diffs each set against the previous one, emitting `Change::Insert` for
    /// addresses that appeared and `Change::Remove` for those that went. An
    /// address present in both is left alone, so its connection survives.
    ///
    /// The returned stream ends when `updates` ends or the consumer drops it.
    pub fn discover_endpoints<U>(&self, updates: U) -> BoxDiscover<EndpointAddress, S>
    where
        U: Stream<Item = HashSet<EndpointAddress>> + Send + 'static,
    {
        let connector = self.connector.clone();
        let (tx, rx) = mpsc::channel(ENDPOINT_CHANNEL_CAPACITY);

        tokio::spawn(diff_loop(updates, connector, tx));

        Box::pin(ReceiverStream::new(rx))
    }
}

/// Background task: diffs consecutive address sets into incremental changes.
async fn diff_loop<S, U>(
    updates: U,
    connector: ConnectorSwap<S>,
    tx: mpsc::Sender<Result<Change<EndpointAddress, S>, BoxError>>,
) where
    S: Send + 'static,
    U: Stream<Item = HashSet<EndpointAddress>> + Send + 'static,
{
    let mut active: HashSet<EndpointAddress> = HashSet::new();
    let mut updates = std::pin::pin!(updates);

    while let Some(new_set) = updates.next().await {
        for added in new_set.difference(&active) {
            let svc = connector.load_full().connect(added).await;
            if tx
                .send(Ok(Change::Insert(added.clone(), svc)))
                .await
                .is_err()
            {
                return;
            }
        }

        for removed in active.difference(&new_set) {
            if tx.send(Ok(Change::Remove(removed.clone()))).await.is_err() {
                return;
            }
        }

        active = new_set;
    }
}

/// The addresses of a cluster's endpoints that gRFC A27 considers usable.
///
/// An endpoint whose health is neither `HEALTHY` nor `UNKNOWN` is left out, so
/// the balancer never holds it. This is an adapter rather than something
/// [`EndpointManager`] applies, so a caller with different rules can feed its
/// own set.
pub(crate) fn healthy_addresses(
    watch: CacheWatch<EndpointsResource>,
) -> impl Stream<Item = HashSet<EndpointAddress>> + Send + 'static {
    futures_util::stream::unfold(watch, |mut watch| async move {
        let endpoints = watch.next().await?;
        let addresses = endpoints
            .healthy_endpoints()
            .map(|endpoint| endpoint.address.clone())
            .collect();
        Some((addresses, watch))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::async_util::BoxFuture;
    use crate::xds::cache::XdsCache;
    use crate::xds::resource::endpoints::{HealthStatus, LocalityEndpoints, ResolvedEndpoint};
    use tokio_stream::StreamExt;

    /// Test [`Connector`] that returns the address as its `Service` (just a
    /// `String`).
    struct StringConnector;

    impl Connector for StringConnector {
        type Service = String;
        fn connect(&self, addr: &EndpointAddress) -> BoxFuture<Self::Service> {
            let s = addr.to_string();
            Box::pin(async move { s })
        }
    }

    fn test_connector() -> SharedConnector<String> {
        Arc::new(StringConnector)
    }

    fn make_endpoints(cluster: &str, addrs: &[(&str, u16)]) -> Arc<EndpointsResource> {
        Arc::new(EndpointsResource {
            cluster_name: cluster.to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: addrs
                    .iter()
                    .map(|(host, port)| ResolvedEndpoint {
                        address: EndpointAddress::new(*host, *port),
                        health_status: HealthStatus::Healthy,
                        load_balancing_weight: 1,
                    })
                    .collect(),
                load_balancing_weight: 100,
                priority: 0,
            }],
        })
    }

    #[tokio::test]
    async fn initial_endpoints_emitted_as_inserts() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints(
            "c1",
            make_endpoints("c1", &[("10.0.0.1", 8080), ("10.0.0.2", 8080)]),
        );

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));

        let mut addrs: Vec<String> = Vec::new();
        for _ in 0..2 {
            match stream.next().await.unwrap().unwrap() {
                Change::Insert(addr, _svc) => addrs.push(addr.to_string()),
                Change::Remove(_) => panic!("expected Insert"),
            }
        }
        addrs.sort();
        assert_eq!(addrs, vec!["10.0.0.1:8080", "10.0.0.2:8080"]);
    }

    #[tokio::test]
    async fn added_endpoint_emits_insert() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        let _ = stream.next().await; // consume initial

        cache.update_endpoints(
            "c1",
            make_endpoints("c1", &[("10.0.0.1", 8080), ("10.0.0.2", 8080)]),
        );

        match stream.next().await.unwrap().unwrap() {
            Change::Insert(addr, _) => assert_eq!(addr.to_string(), "10.0.0.2:8080"),
            Change::Remove(_) => panic!("expected Insert for new endpoint"),
        }
    }

    #[tokio::test]
    async fn removed_endpoint_emits_remove() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints(
            "c1",
            make_endpoints("c1", &[("10.0.0.1", 8080), ("10.0.0.2", 8080)]),
        );

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        // Consume 2 initial inserts.
        let _ = stream.next().await;
        let _ = stream.next().await;

        // Shrink to one endpoint.
        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        match stream.next().await.unwrap().unwrap() {
            Change::Remove(addr) => assert_eq!(addr.to_string(), "10.0.0.2:8080"),
            Change::Insert(..) => panic!("expected Remove"),
        }
    }

    #[tokio::test]
    async fn unhealthy_endpoint_removed() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        let _ = stream.next().await; // consume initial insert

        let unhealthy = Arc::new(EndpointsResource {
            cluster_name: "c1".to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: vec![ResolvedEndpoint {
                    address: EndpointAddress::new("10.0.0.1", 8080),
                    health_status: HealthStatus::Unhealthy,
                    load_balancing_weight: 1,
                }],
                load_balancing_weight: 100,
                priority: 0,
            }],
        });
        cache.update_endpoints("c1", unhealthy);

        match stream.next().await.unwrap().unwrap() {
            Change::Remove(addr) => assert_eq!(addr.to_string(), "10.0.0.1:8080"),
            Change::Insert(..) => panic!("expected Remove for unhealthy endpoint"),
        }
    }

    #[tokio::test]
    async fn cache_removal_closes_stream() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        let _ = stream.next().await; // consume initial

        cache.remove_endpoints("c1");

        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn multiple_clusters_independent() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));
        cache.update_endpoints("c2", make_endpoints("c2", &[("10.0.0.2", 9090)]));

        let mut s1 = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        let mut s2 = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c2")));

        match s1.next().await.unwrap().unwrap() {
            Change::Insert(addr, _) => assert_eq!(addr.to_string(), "10.0.0.1:8080"),
            _ => panic!("expected Insert"),
        }
        match s2.next().await.unwrap().unwrap() {
            Change::Insert(addr, _) => assert_eq!(addr.to_string(), "10.0.0.2:9090"),
            _ => panic!("expected Insert"),
        }
    }

    #[tokio::test]
    async fn endpoint_swap_emits_insert_then_remove() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(healthy_addresses(cache.watch_endpoints("c1")));
        let _ = stream.next().await; // consume initial

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.2", 8080)]));

        let mut saw_remove = false;
        let mut saw_insert = false;
        for _ in 0..2 {
            match stream.next().await.unwrap().unwrap() {
                Change::Remove(addr) => {
                    assert_eq!(addr.to_string(), "10.0.0.1:8080");
                    saw_remove = true;
                }
                Change::Insert(addr, _) => {
                    assert_eq!(addr.to_string(), "10.0.0.2:8080");
                    saw_insert = true;
                }
            }
        }
        assert!(saw_remove, "should have removed old endpoint");
        assert!(saw_insert, "should have inserted new endpoint");
    }
}
