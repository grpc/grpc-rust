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

//! Converts snapshot-based endpoint cache updates into incremental
//! [`Change`] streams for Tower's load balancing infrastructure.
//!
//! The resource manager writes [`EndpointsResource`] snapshots into the
//! `XdsCache`; this module diffs consecutive snapshots and produces
//! `Change::Insert` / `Change::Remove` events that Tower's P2C balancer
//! (or any other `Discover`-based balancer) can consume.
//!
//! Two flavours share one diff loop, differing only in what an added endpoint
//! is turned into: [`EndpointManager::discover_endpoints`] connects it through
//! a [`Connector`], while [`discover_addresses`] emits a bare [`IdleChannel`]
//! for balancers that own the connection lifecycle themselves.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tower::BoxError;
use tower::discover::Change;

use crate::client::endpoint::{Connector, EndpointAddress, XdsEndpoint};
use crate::client::lb::BoxDiscover;
use crate::client::loadbalance::channel_state::IdleChannel;
use crate::common::async_util::BoxFuture;
use crate::xds::cache::CacheWatch;
use crate::xds::resource::EndpointsResource;

/// Buffer capacity for the endpoint change channel between the diff loop
/// and Tower's load balancer.
const ENDPOINT_CHANNEL_CAPACITY: usize = 64;

/// Converts endpoint cache watches into incremental [`Change`] streams.
///
/// `EndpointManager` is a pure diff-and-connect component: the caller
/// (typically `XdsClusterDiscovery`) obtains a [`CacheWatch`] from the
/// [`XdsCache`](crate::xds::cache::XdsCache) and passes it here, plus the
/// [`Connector`] used to establish each newly-discovered endpoint. Keeping the
/// connector current across CDS updates is the connector's own concern (see
/// [`ClusterConnector`](crate::xds::connector::ClusterConnector)).
pub(crate) struct EndpointManager<S: Send + 'static> {
    connector: Arc<dyn Connector<Service = S> + Send + Sync>,
}

impl<S: Send + 'static> EndpointManager<S> {
    pub(crate) fn new(connector: Arc<dyn Connector<Service = S> + Send + Sync>) -> Self {
        Self { connector }
    }

    /// Returns a stream of endpoint changes for the given cache watch.
    ///
    /// Diffs each snapshot against the previous set of healthy endpoints,
    /// emitting `Change::Insert` for new or changed endpoints and
    /// `Change::Remove` for removed ones.
    ///
    /// Each inserted endpoint is connected through the [`Connector`], so it
    /// reaches the balancer already established. A CDS update therefore only
    /// reaches endpoints discovered after it: channels already handed out keep
    /// the connection (and any TLS session) they were created with.
    pub(crate) fn discover_endpoints(
        &self,
        watch: CacheWatch<EndpointsResource>,
    ) -> BoxDiscover<EndpointAddress, S> {
        let connector = self.connector.clone();
        spawn_diff_loop(watch, move |endpoint| connector.connect(endpoint))
    }
}

/// Returns a stream of endpoint changes carrying unconnected endpoints.
///
/// Each added endpoint is emitted as an [`IdleChannel`] — the initial state of
/// the load balancer's channel state machine — without connecting. For
/// balancers that drive connect / eject / reconnect themselves and so need the
/// endpoint rather than a live channel.
pub(crate) fn discover_addresses(
    watch: CacheWatch<EndpointsResource>,
) -> BoxDiscover<EndpointAddress, IdleChannel> {
    spawn_diff_loop(watch, |endpoint| {
        let channel = IdleChannel::from(Arc::clone(endpoint));
        Box::pin(async move { channel })
    })
}

/// Spawns the shared diff loop and returns the change stream it feeds.
///
/// The spawned task exits naturally when either:
/// - The `CacheWatch` closes (`cache.remove_endpoints()` drops the watch sender)
/// - The receiver is dropped (consumer no longer reading `Change` events)
fn spawn_diff_loop<S, F>(
    watch: CacheWatch<EndpointsResource>,
    make_service: F,
) -> BoxDiscover<EndpointAddress, S>
where
    S: Send + 'static,
    F: Fn(&Arc<XdsEndpoint>) -> BoxFuture<S> + Send + 'static,
{
    let (tx, rx) = mpsc::channel(ENDPOINT_CHANNEL_CAPACITY);
    tokio::spawn(diff_loop(watch, make_service, tx));
    Box::pin(ReceiverStream::new(rx))
}

/// Background task: watches endpoint snapshots and emits incremental changes.
///
/// Each time a new [`EndpointsResource`] arrives from the cache, we diff
/// `healthy_endpoints()` against the previous snapshot, keyed by primary
/// address, and emit `Insert` for new endpoints followed by `Remove` for gone
/// ones. An endpoint whose primary address is unchanged but whose additional
/// addresses or hostname changed is re-emitted as an `Insert` under the same
/// key, which the balancers treat as replacing the service there, so the
/// connector sees the updated endpoint.
async fn diff_loop<S, F>(
    mut watch: CacheWatch<EndpointsResource>,
    make_service: F,
    tx: mpsc::Sender<Result<Change<EndpointAddress, S>, BoxError>>,
) where
    S: Send + 'static,
    F: Fn(&Arc<XdsEndpoint>) -> BoxFuture<S>,
{
    let mut active: HashMap<EndpointAddress, Arc<XdsEndpoint>> = HashMap::new();

    while let Some(endpoints) = watch.next().await {
        let mut latest: HashMap<EndpointAddress, Arc<XdsEndpoint>> = HashMap::new();
        for ep in endpoints.healthy_endpoints() {
            // Endpoints sharing a primary address collapse to the first.
            latest
                .entry(ep.endpoint.address().clone())
                .or_insert_with(|| Arc::clone(&ep.endpoint));
        }

        for (addr, endpoint) in &latest {
            if active.get(addr) == Some(endpoint) {
                continue;
            }
            let svc = make_service(endpoint).await;
            if tx
                .send(Ok(Change::Insert(addr.clone(), svc)))
                .await
                .is_err()
            {
                return;
            }
        }

        for removed in active.keys().filter(|addr| !latest.contains_key(*addr)) {
            if tx.send(Ok(Change::Remove(removed.clone()))).await.is_err() {
                return;
            }
        }

        active = latest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::async_util::BoxFuture;
    use crate::xds::cache::XdsCache;
    use crate::xds::resource::endpoints::{HealthStatus, LocalityEndpoints, ResolvedEndpoint};
    use tokio_stream::StreamExt;

    /// Test [`Connector`] whose `Service` is a `String` describing the
    /// endpoint it was asked to connect: every address, then the hostname.
    struct StringConnector;

    impl Connector for StringConnector {
        type Service = String;
        fn connect(&self, endpoint: &XdsEndpoint) -> BoxFuture<Self::Service> {
            let addrs: Vec<String> = endpoint.addresses().map(ToString::to_string).collect();
            let s = match endpoint.hostname() {
                Some(hostname) => format!("{} ({hostname})", addrs.join(",")),
                None => addrs.join(","),
            };
            Box::pin(async move { s })
        }
    }

    fn test_connector() -> Arc<dyn Connector<Service = String> + Send + Sync> {
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
                        endpoint: Arc::new(EndpointAddress::new(*host, *port).into()),
                        health_status: HealthStatus::Healthy,
                        load_balancing_weight: 1,
                    })
                    .collect(),
                load_balancing_weight: 100,
                priority: 0,
            }],
        })
    }

    /// A single-endpoint resource for `10.0.0.1:8080` with the given extras.
    fn make_single_endpoint(
        additional: &[&str],
        hostname: Option<&str>,
        health_status: HealthStatus,
    ) -> Arc<EndpointsResource> {
        Arc::new(EndpointsResource {
            cluster_name: "c1".to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: vec![ResolvedEndpoint {
                    endpoint: Arc::new(
                        XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080))
                            .with_additional_addresses(
                                additional
                                    .iter()
                                    .map(|host| EndpointAddress::new(*host, 8080)),
                            )
                            .with_hostname(hostname.unwrap_or_default()),
                    ),
                    health_status,
                    load_balancing_weight: 1,
                }],
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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));

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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
        let _ = stream.next().await; // consume initial insert

        let unhealthy = Arc::new(EndpointsResource {
            cluster_name: "c1".to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: vec![ResolvedEndpoint {
                    endpoint: Arc::new(EndpointAddress::new("10.0.0.1", 8080).into()),
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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
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

        let mut s1 = manager.discover_endpoints(cache.watch_endpoints("c1"));
        let mut s2 = manager.discover_endpoints(cache.watch_endpoints("c2"));

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

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
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

    /// A connector sees every field of the endpoint, and a change to any
    /// field but the primary address reconnects under the same key.
    #[tokio::test]
    async fn changed_endpoint_reinserted_under_same_key() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints("c1", make_single_endpoint(&[], None, HealthStatus::Healthy));
        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
        match stream.next().await.unwrap().unwrap() {
            Change::Insert(_, svc) => assert_eq!(svc, "10.0.0.1:8080"),
            Change::Remove(_) => panic!("expected Insert"),
        }

        let updates = [
            (
                make_single_endpoint(&["2001:db8::1"], None, HealthStatus::Healthy),
                "10.0.0.1:8080,[2001:db8::1]:8080",
            ),
            (
                make_single_endpoint(&["2001:db8::1"], Some("backend-1"), HealthStatus::Healthy),
                "10.0.0.1:8080,[2001:db8::1]:8080 (backend-1)",
            ),
        ];
        for (update, expected) in updates {
            cache.update_endpoints("c1", update);
            match stream.next().await.unwrap().unwrap() {
                Change::Insert(addr, svc) => {
                    assert_eq!(addr, EndpointAddress::new("10.0.0.1", 8080));
                    assert_eq!(svc, expected);
                }
                Change::Remove(_) => panic!("expected Insert for changed endpoint"),
            }
        }
    }

    /// Re-delivering an endpoint with the same addresses and hostname, even
    /// under a different usable health status, does not reconnect it.
    #[tokio::test]
    async fn unchanged_endpoint_not_reinserted() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector());

        cache.update_endpoints(
            "c1",
            make_single_endpoint(&["2001:db8::1"], Some("backend-1"), HealthStatus::Healthy),
        );
        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
        let _ = stream.next().await; // consume initial

        cache.update_endpoints(
            "c1",
            make_single_endpoint(&["2001:db8::1"], Some("backend-1"), HealthStatus::Unknown),
        );
        // The cache is a watch channel, so let the diff loop see the
        // re-delivery before the next update would coalesce it away.
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        // A later, real change must be the next item: nothing was emitted for
        // the re-delivery.
        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.2", 8080)]));

        let mut changes = Vec::new();
        for _ in 0..2 {
            changes.push(match stream.next().await.unwrap().unwrap() {
                Change::Insert(addr, _) => format!("insert {addr}"),
                Change::Remove(addr) => format!("remove {addr}"),
            });
        }
        assert_eq!(changes, ["insert 10.0.0.2:8080", "remove 10.0.0.1:8080"]);
    }
}
