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

use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::ReceiverStream;
use tower::BoxError;
use tower::discover::Change;

use crate::client::endpoint::{Connector, EndpointAddress};
use crate::client::lb::BoxDiscover;
use crate::xds::cache::{CacheEvent, CacheWatch};
use crate::xds::resource::EndpointsResource;

/// Buffer capacity for the endpoint change channel between the diff loop
/// and Tower's load balancer.
const ENDPOINT_CHANNEL_CAPACITY: usize = 64;

pub(crate) struct ConnectorState<S> {
    generation: u64,
    connector: Arc<dyn Connector<Service = S> + Send + Sync>,
}

impl<S> ConnectorState<S> {
    pub(crate) fn new(
        generation: u64,
        connector: Arc<dyn Connector<Service = S> + Send + Sync>,
    ) -> Self {
        Self {
            generation,
            connector,
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }
}

pub(crate) type ConnectorWatch<S> = watch::Receiver<Arc<ConnectorState<S>>>;

/// Converts endpoint cache watches into incremental [`Change`] streams.
///
/// `EndpointManager` is a pure diff-and-connect component: the caller
/// (typically `XdsClusterDiscovery`) obtains a [`CacheWatch`] from the
/// [`XdsCache`](crate::xds::cache::XdsCache) and passes it here, plus a
/// [`ConnectorWatch`] updated from CDS.
pub(crate) struct EndpointManager<S: Send + 'static> {
    connector: ConnectorWatch<S>,
}

impl<S: Send + 'static> EndpointManager<S> {
    pub(crate) fn new(connector: ConnectorWatch<S>) -> Self {
        Self { connector }
    }

    /// Returns a stream of endpoint changes for the given cache watch.
    ///
    /// Diffs each snapshot against the previous set of healthy endpoints,
    /// emitting `Change::Insert` for new endpoints and `Change::Remove`
    /// for removed ones.
    pub(crate) fn discover_endpoints(
        &self,
        watch: CacheWatch<EndpointsResource>,
    ) -> BoxDiscover<EndpointAddress, S> {
        let connector = self.connector.clone();
        let (tx, rx) = mpsc::channel(ENDPOINT_CHANNEL_CAPACITY);

        // The spawned task exits when the cache or the consumer is dropped.
        tokio::spawn(diff_loop(watch, connector, tx));

        Box::pin(ReceiverStream::new(rx))
    }
}

/// Background task: watches endpoint snapshots and emits incremental changes.
///
/// Each time a new [`EndpointsResource`] arrives from the cache, we diff
/// `healthy_endpoints()` against the previous set and emit `Insert` for
/// new endpoints followed by `Remove` for gone ones.
async fn diff_loop<S: Send + 'static>(
    mut watch: CacheWatch<EndpointsResource>,
    mut connector: ConnectorWatch<S>,
    tx: mpsc::Sender<Result<Change<EndpointAddress, S>, BoxError>>,
) {
    let mut active: HashSet<EndpointAddress> = HashSet::new();

    'events: loop {
        let event = tokio::select! {
            _ = tx.closed() => return,
            event = watch.next_event() => event,
        };
        let Some(event) = event else {
            return;
        };
        let CacheEvent::Resource {
            generation,
            revision,
            resource: endpoints,
        } = event
        else {
            for removed in active.drain() {
                if tx.send(Ok(Change::Remove(removed))).await.is_err() {
                    return;
                }
            }
            continue;
        };
        let connector = tokio::select! {
            _ = tx.closed() => return,
            connector = connector_for_generation(&mut connector, generation) => connector,
        };
        let Some(connector) = connector else {
            continue;
        };
        if watch.current_revision() != revision {
            continue;
        }

        let new_set: HashSet<EndpointAddress> = endpoints
            .healthy_endpoints()
            .map(|ep| ep.address.clone())
            .collect();
        let added: Vec<_> = new_set.difference(&active).cloned().collect();
        let removed: Vec<_> = active.difference(&new_set).cloned().collect();

        for added in added {
            if watch.current_revision() != revision {
                continue 'events;
            }
            let svc = connector.connect(&added).await;
            if watch.current_revision() != revision {
                continue 'events;
            }
            if tx
                .send(Ok(Change::Insert(added.clone(), svc)))
                .await
                .is_err()
            {
                return;
            }
            active.insert(added);
        }

        for removed in removed {
            if watch.current_revision() != revision {
                continue 'events;
            }
            if tx.send(Ok(Change::Remove(removed.clone()))).await.is_err() {
                return;
            }
            active.remove(&removed);
        }
    }
}

async fn connector_for_generation<S>(
    connector: &mut ConnectorWatch<S>,
    generation: u64,
) -> Option<Arc<dyn Connector<Service = S> + Send + Sync>> {
    loop {
        let current = connector.borrow().clone();
        match current.generation.cmp(&generation) {
            std::cmp::Ordering::Equal => return Some(Arc::clone(&current.connector)),
            std::cmp::Ordering::Greater => return None,
            std::cmp::Ordering::Less => {}
        }
        if connector.changed().await.is_err() {
            return None;
        }
    }
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

    fn test_connector_channel(
        generation: u64,
    ) -> (
        watch::Sender<Arc<ConnectorState<String>>>,
        ConnectorWatch<String>,
    ) {
        let conn: Arc<dyn Connector<Service = String> + Send + Sync> = Arc::new(StringConnector);
        watch::channel(Arc::new(ConnectorState::new(generation, conn)))
    }

    fn test_connector_watch() -> ConnectorWatch<String> {
        test_connector_channel(0).1
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
        let manager = EndpointManager::new(test_connector_watch());

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
        let manager = EndpointManager::new(test_connector_watch());

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
        let manager = EndpointManager::new(test_connector_watch());

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
        let manager = EndpointManager::new(test_connector_watch());

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
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
    async fn cache_removal_clears_endpoints_and_readd_recovers() {
        let cache = XdsCache::new();
        let (connector_tx, connector_rx) = test_connector_channel(0);
        let manager = EndpointManager::new(connector_rx);

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));

        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
        let _ = stream.next().await; // consume initial

        cache.remove_endpoints("c1");

        match stream.next().await.unwrap().unwrap() {
            Change::Remove(addr) => assert_eq!(addr.to_string(), "10.0.0.1:8080"),
            Change::Insert(..) => panic!("expected Remove after cache removal"),
        }

        let connector: Arc<dyn Connector<Service = String> + Send + Sync> =
            Arc::new(StringConnector);
        connector_tx.send_replace(Arc::new(ConnectorState::new(1, connector)));
        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));
        match stream.next().await.unwrap().unwrap() {
            Change::Insert(addr, _) => assert_eq!(addr.to_string(), "10.0.0.1:8080"),
            Change::Remove(..) => panic!("expected Insert after cache re-add"),
        }
    }

    #[tokio::test]
    async fn readded_endpoints_wait_for_matching_connector_generation() {
        let cache = XdsCache::new();
        let (connector_tx, connector_rx) = test_connector_channel(0);
        let manager = EndpointManager::new(connector_rx);
        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.1", 8080)]));
        let mut stream = manager.discover_endpoints(cache.watch_endpoints("c1"));
        let _ = stream.next().await; // consume initial insert

        cache.remove_endpoints("c1");
        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.2", 8080)]));

        match stream.next().await.unwrap().unwrap() {
            Change::Remove(addr) => assert_eq!(addr.to_string(), "10.0.0.1:8080"),
            Change::Insert(..) => panic!("expected old endpoint removal"),
        }
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(20), stream.next())
                .await
                .is_err(),
            "re-added endpoints must wait for their CDS connector generation",
        );

        cache.update_endpoints("c1", make_endpoints("c1", &[("10.0.0.3", 8080)]));
        let connector: Arc<dyn Connector<Service = String> + Send + Sync> =
            Arc::new(StringConnector);
        connector_tx.send_replace(Arc::new(ConnectorState::new(1, connector)));
        match stream.next().await.unwrap().unwrap() {
            Change::Insert(addr, _) => assert_eq!(addr.to_string(), "10.0.0.3:8080"),
            Change::Remove(..) => panic!("expected re-added endpoint insert"),
        }
    }

    #[tokio::test]
    async fn multiple_clusters_independent() {
        let cache = XdsCache::new();
        let manager = EndpointManager::new(test_connector_watch());

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
        let manager = EndpointManager::new(test_connector_watch());

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
}
