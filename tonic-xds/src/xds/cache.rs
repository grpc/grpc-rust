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

//! Shared xDS configuration snapshot channel and watch primitives.
//!
//! [`XdsResourceManager`](crate::xds::resource_manager::XdsResourceManager)
//! publishes validated, self-consistent [`XdsConfig`] snapshots into
//! [`XdsCache`]. [`XdsRouter`](crate::xds::routing::XdsRouter) subscribes to
//! [`XdsCache::watch`] and fans per-cluster CDS/EDS updates out over
//! router-owned [`CacheSender`] / [`CacheWatch`] channels bundled in
//! [`ClusterWatches`]. Applications watch the same snapshots through
//! [`XdsChannel::xds_config`](crate::XdsChannel::xds_config).

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::watch;

use crate::client::endpoint::XdsEndpoint;
use crate::xds::resource::{ClusterResource, EndpointsResource, RouteConfigResource};

/// A resolved cluster within an [`XdsConfig`]: its CDS resource is valid and
/// its EDS resource has resolved.
///
/// Per gRFC A74, an EDS failure still resolves the cluster, with no endpoints
/// and the reason in [`resolution_note`](Self::resolution_note). RPCs routed
/// to such a cluster fail with that note.
#[derive(Clone, Debug)]
pub struct ClusterSnapshot {
    pub(crate) cluster: Arc<ClusterResource>,
    /// `None` if the EDS resource failed.
    pub(crate) endpoints: Option<Arc<EndpointsResource>>,
    /// The EDS failure and ambient errors affecting this cluster (gRFC A88).
    pub(crate) resolution_note: Option<Arc<str>>,
}

impl ClusterSnapshot {
    /// The healthy endpoints the cluster's load balancer connects to, once
    /// per address, in EDS order.
    ///
    /// `None` if the EDS resource failed; see
    /// [`resolution_note`](Self::resolution_note).
    pub fn endpoints(&self) -> Option<impl Iterator<Item = &XdsEndpoint>> {
        let endpoints = self.endpoints.as_ref()?;
        Some(endpoints.unique_healthy_endpoints().map(|ep| &**ep))
    }

    /// Why [`endpoints`](Self::endpoints) is `None`, and any ambient errors
    /// (such as a rejected update, which keeps the last valid resource) of
    /// this cluster's xDS resources or the Listener and RouteConfiguration
    /// above it, joined with `"; "` (gRFC A88).
    pub fn resolution_note(&self) -> Option<&str> {
        self.resolution_note.as_deref()
    }

    /// The error RPCs routed to this cluster fail with while it has no
    /// endpoints: its resolution note.
    pub(crate) fn no_endpoints_error(&self, name: &str) -> ClusterError {
        ClusterError(
            self.resolution_note
                .clone()
                .unwrap_or_else(|| format!("cluster '{name}' has no endpoints").into()),
        )
    }

    fn ptr_eq(&self, other: &Self) -> bool {
        let endpoints_eq = match (&self.endpoints, &other.endpoints) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        Arc::ptr_eq(&self.cluster, &other.cluster)
            && endpoints_eq
            && self.resolution_note == other.resolution_note
    }
}

/// Why a cluster in an [`XdsConfig`] failed to resolve: its CDS resource
/// failed without a cached fallback.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ClusterError(pub(crate) Arc<str>);

/// Why the xDS configuration failed to resolve: the Listener (LDS),
/// RouteConfiguration (RDS), or virtual host for the channel's authority
/// failed without a cached fallback.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct XdsConfigError(pub(crate) Arc<str>);

/// Atomic, completeness-gated snapshot of the active route configuration and
/// all clusters referenced by the matched virtual host, aligned with gRFC A74
/// `XdsConfig`.
///
/// A snapshot is published only once every referenced cluster has resolved or
/// failed.
#[derive(Clone, Debug)]
pub struct XdsConfig {
    pub(crate) route_config: Arc<RouteConfigResource>,
    /// Index into `route_config.virtual_hosts` matching the channel's target authority.
    pub(crate) virtual_host_index: usize,
    /// Keyed by cluster name; contains an entry (`Ok` or `Err`) for every
    /// cluster referenced by the matched virtual host. Never contains pending
    /// clusters — publication is gated until every referenced cluster resolves.
    pub(crate) clusters: HashMap<String, Result<ClusterSnapshot, ClusterError>>,
}

impl XdsConfig {
    /// Every cluster the matched virtual host references, by name, in
    /// arbitrary order.
    pub fn clusters(
        &self,
    ) -> impl Iterator<Item = (&str, Result<&ClusterSnapshot, &ClusterError>)> {
        self.clusters
            .iter()
            .map(|(name, cluster)| (name.as_str(), cluster.as_ref()))
    }

    fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.route_config, &other.route_config)
            && self.virtual_host_index == other.virtual_host_index
            && self.clusters.len() == other.clusters.len()
            && self
                .clusters
                .iter()
                .all(|(name, res)| match (res, other.clusters.get(name)) {
                    (Ok(a), Some(Ok(b))) => a.ptr_eq(b),
                    (Err(a), Some(Err(b))) => a == b,
                    _ => false,
                })
    }
}

/// The latest xDS configuration, or why it failed to resolve.
pub type XdsConfigResult = Result<Arc<XdsConfig>, XdsConfigError>;

/// Per-cluster resource watches supplied by [`XdsRouter`](crate::xds::routing::XdsRouter)
/// when provisioning a cluster's load-balanced channel.
pub(crate) struct ClusterWatches {
    pub(crate) cluster: CacheWatch<Arc<ClusterResource>>,
    pub(crate) endpoints: CacheWatch<Arc<EndpointsResource>>,
}

impl ClusterWatches {
    /// Creates connected CDS and EDS sender/receiver pairs for tests.
    #[cfg(test)]
    pub(crate) fn channel(
        cluster: Option<Arc<ClusterResource>>,
        endpoints: Option<Arc<EndpointsResource>>,
    ) -> (
        CacheSender<ClusterResource>,
        CacheSender<EndpointsResource>,
        Self,
    ) {
        let (cluster_tx, cluster) = CacheWatch::channel(cluster);
        let (endpoints_tx, endpoints) = CacheWatch::channel(endpoints);
        (cluster_tx, endpoints_tx, Self { cluster, endpoints })
    }

    /// Already-closed watches for mock/test callers without live xDS feeds.
    #[cfg(test)]
    pub(crate) fn closed() -> Self {
        Self::channel(None, None).2
    }
}

/// Sending half of a single-resource watch channel paired with [`CacheWatch`].
///
/// Dropping the sender closes all associated [`CacheWatch`] receivers.
pub(crate) struct CacheSender<T> {
    tx: watch::Sender<Option<Arc<T>>>,
}

impl<T> CacheSender<T> {
    /// Publishes `value` unless the channel already holds the same [`Arc`].
    pub(crate) fn send(&self, value: Arc<T>) {
        self.tx.send_if_modified(|slot| {
            if slot.as_ref().is_some_and(|prev| Arc::ptr_eq(prev, &value)) {
                false
            } else {
                *slot = Some(value);
                true
            }
        });
    }
}

/// Watches a value for updates.
///
/// Each [`next`](Self::next) call waits for a value newer than the last one it
/// returned; the first call returns the current value, if there is one.
/// Intermediate updates may be skipped.
#[derive(Debug)]
pub struct CacheWatch<T> {
    rx: watch::Receiver<Option<T>>,
}

impl<T: Clone> CacheWatch<T> {
    fn from_receiver(mut rx: watch::Receiver<Option<T>>) -> Self {
        if rx.borrow().is_some() {
            // Ensure late watchers see the existing value on first next().
            rx.mark_changed();
        }
        Self { rx }
    }

    /// Waits for the next update and returns it, or `None` once no more
    /// updates can arrive.
    pub async fn next(&mut self) -> Option<T> {
        loop {
            if self.rx.changed().await.is_err() {
                return None;
            }
            let val = self.rx.borrow_and_update().clone();
            if val.is_some() {
                return val;
            }
        }
    }
}

impl<T> CacheWatch<Arc<T>> {
    /// Creates a connected sender/receiver pair with an optional initial value.
    ///
    /// When `initial` is `Some`, the returned [`CacheWatch`] yields it on the
    /// first [`next`](Self::next) call even if no further updates are sent.
    pub(crate) fn channel(initial: Option<Arc<T>>) -> (CacheSender<T>, Self) {
        let (tx, rx) = watch::channel(initial);
        (CacheSender { tx }, Self::from_receiver(rx))
    }
}

/// Latest validated [`XdsConfig`] snapshot channel published by the resource
/// manager and watched by [`XdsRouter`](crate::xds::routing::XdsRouter).
pub(crate) struct XdsCache {
    config_tx: watch::Sender<Option<XdsConfigResult>>,
}

impl XdsCache {
    /// Creates a new empty cache with no published snapshot.
    pub(crate) fn new() -> Self {
        let (config_tx, _) = watch::channel(None);
        Self { config_tx }
    }

    /// Publishes a new atomic [`XdsConfig`] snapshot unless an identical
    /// pointer-equal snapshot is already active.
    pub(crate) fn publish(&self, config: Arc<XdsConfig>) {
        self.config_tx.send_if_modified(|slot| {
            if matches!(slot, Some(Ok(prev)) if prev.ptr_eq(&config)) {
                false
            } else {
                *slot = Some(Ok(config));
                true
            }
        });
    }

    /// Publishes a top-level [`XdsConfigError`] unless the same error is
    /// already active.
    pub(crate) fn publish_error(&self, err: XdsConfigError) {
        self.config_tx.send_if_modified(|slot| {
            if matches!(slot, Some(Err(prev)) if prev == &err) {
                false
            } else {
                *slot = Some(Err(err));
                true
            }
        });
    }

    /// Watches [`XdsConfigResult`] updates.
    pub(crate) fn watch(&self) -> CacheWatch<XdsConfigResult> {
        CacheWatch::from_receiver(self.config_tx.subscribe())
    }

    /// Returns the currently published [`XdsConfig`], if `Ok`.
    #[cfg(test)]
    pub(crate) fn current(&self) -> Option<Arc<XdsConfig>> {
        match self.config_tx.borrow().as_ref()? {
            Ok(cfg) => Some(Arc::clone(cfg)),
            Err(_) => None,
        }
    }

    /// Returns the currently published [`XdsConfigResult`], if any.
    #[cfg(test)]
    pub(crate) fn current_result(&self) -> Option<XdsConfigResult> {
        self.config_tx.borrow().clone()
    }

    /// Publishes `route_config` immediately for pure route-matching / mock
    /// tests, pinning virtual host `0` and populating stub CDS + empty EDS
    /// resources for every referenced cluster.
    #[cfg(test)]
    pub(crate) fn publish_route_config_for_test(&self, route_config: Arc<RouteConfigResource>) {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        let clusters = route_config.virtual_hosts[0]
            .cluster_names()
            .into_iter()
            .map(|name| {
                (
                    name.to_string(),
                    Ok(ClusterSnapshot {
                        cluster: Arc::new(ClusterResource {
                            name: name.to_string(),
                            eds_service_name: None,
                            lb_policy: LbPolicy::RoundRobin,
                            security: None,
                            circuit_breaking: CircuitBreakingConfig::default(),
                            outlier_detection: Default::default(),
                        }),
                        endpoints: Some(Arc::new(EndpointsResource {
                            cluster_name: name.to_string(),
                            localities: vec![],
                        })),
                        resolution_note: None,
                    }),
                )
            })
            .collect();
        self.publish(Arc::new(XdsConfig {
            route_config,
            virtual_host_index: 0,
            clusters,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xds::resource::cluster::LbPolicy;

    fn make_route_config(name: &str) -> Arc<RouteConfigResource> {
        use crate::xds::resource::route_config::{
            PathSpecifierConfig, RouteConfig, RouteConfigAction, RouteConfigMatch,
            VirtualHostConfig,
        };
        Arc::new(RouteConfigResource {
            name: name.to_string(),
            virtual_hosts: vec![VirtualHostConfig {
                name: "vh1".to_string(),
                domains: vec!["*".to_string()],
                routes: vec![RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".to_string()),
                        headers: vec![],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("cluster-1".to_string()),
                    retry_config: None,
                    max_stream_duration: None,
                }],
            }],
            metadata: Default::default(),
        })
    }

    fn make_cluster(name: &str, lb: LbPolicy) -> Arc<ClusterResource> {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;

        Arc::new(ClusterResource {
            name: name.to_string(),
            eds_service_name: None,
            lb_policy: lb,
            security: None,
            circuit_breaking: CircuitBreakingConfig::default(),
            outlier_detection: Default::default(),
        })
    }

    fn make_endpoints(cluster_name: &str) -> Arc<EndpointsResource> {
        use crate::client::endpoint::EndpointAddress;
        use crate::xds::resource::endpoints::{HealthStatus, LocalityEndpoints, ResolvedEndpoint};
        Arc::new(EndpointsResource {
            cluster_name: cluster_name.to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: vec![ResolvedEndpoint {
                    endpoint: Arc::new(EndpointAddress::new("10.0.0.1", 8080).into()),
                    health_status: HealthStatus::Healthy,
                    load_balancing_weight: 1,
                }],
                load_balancing_weight: 100,
                priority: 0,
            }],
        })
    }

    #[tokio::test]
    async fn late_watcher_sees_existing_snapshot_and_error() {
        let cache = XdsCache::new();
        cache.publish_route_config_for_test(make_route_config("rc-1"));

        let snapshot = cache.watch().next().await.unwrap().unwrap();
        assert_eq!(snapshot.route_config.name, "rc-1");
        assert!(snapshot.clusters["cluster-1"].is_ok());

        cache.publish_error(XdsConfigError("lds missing".into()));
        assert_eq!(
            cache.watch().next().await.unwrap().unwrap_err(),
            XdsConfigError("lds missing".into())
        );
    }

    #[tokio::test]
    async fn channel_delivers_initial_and_subsequent_updates() {
        let initial = make_cluster("c1", LbPolicy::RoundRobin);
        let (tx, mut watch) = CacheWatch::channel(Some(Arc::clone(&initial)));

        let first = watch.next().await.unwrap();
        assert!(Arc::ptr_eq(&first, &initial));

        let updated = make_cluster("c1", LbPolicy::LeastRequest);
        tx.send(Arc::clone(&updated));
        let second = watch.next().await.unwrap();
        assert!(Arc::ptr_eq(&second, &updated));
    }

    #[tokio::test]
    async fn channel_dedupes_identical_arc_without_waking_receiver() {
        let c1 = make_cluster("c1", LbPolicy::RoundRobin);
        let (tx, mut watch) = CacheWatch::channel(Some(Arc::clone(&c1)));
        let _ = watch.next().await.unwrap();

        // Re-sending the exact same Arc must not wake the watcher.
        tx.send(Arc::clone(&c1));
        let woken = tokio::time::timeout(std::time::Duration::from_millis(20), watch.next()).await;
        assert!(woken.is_err(), "unchanged Arc should not notify watcher");

        // Sending a distinct Arc wakes the watcher immediately.
        let c2 = make_cluster("c1", LbPolicy::LeastRequest);
        tx.send(Arc::clone(&c2));
        let next = watch.next().await.unwrap();
        assert!(Arc::ptr_eq(&next, &c2));
    }

    #[tokio::test]
    async fn channel_closes_when_sender_dropped() {
        let initial = make_cluster("c1", LbPolicy::RoundRobin);
        let (tx, mut watch) = CacheWatch::channel(Some(Arc::clone(&initial)));
        drop(tx);

        // Unseen initial value is still delivered before reporting closure.
        let seen = watch.next().await.unwrap();
        assert!(Arc::ptr_eq(&seen, &initial));
        assert!(watch.next().await.is_none());

        let mut closed = ClusterWatches::closed();
        assert!(closed.cluster.next().await.is_none());
        assert!(closed.endpoints.next().await.is_none());
    }

    #[tokio::test]
    async fn config_publish_dedupes_and_handles_errors() {
        let cache = XdsCache::new();
        let mut watch = cache.watch();

        let rc = make_route_config("rc-1");
        let c1 = make_cluster("cluster-1", LbPolicy::RoundRobin);
        let ep1 = make_endpoints("cluster-1");
        let cfg = Arc::new(XdsConfig {
            route_config: Arc::clone(&rc),
            virtual_host_index: 0,
            clusters: HashMap::from([(
                "cluster-1".to_string(),
                Ok(ClusterSnapshot {
                    cluster: Arc::clone(&c1),
                    endpoints: Some(Arc::clone(&ep1)),
                    resolution_note: None,
                }),
            )]),
        });

        cache.publish(Arc::clone(&cfg));
        let received = watch.next().await.unwrap().unwrap();
        assert!(Arc::ptr_eq(&received, &cfg));

        // Identical pointer-equal snapshot must not wake watcher.
        let duplicate_cfg = Arc::new(XdsConfig {
            route_config: rc,
            virtual_host_index: 0,
            clusters: HashMap::from([(
                "cluster-1".to_string(),
                Ok(ClusterSnapshot {
                    cluster: c1,
                    endpoints: Some(ep1),
                    resolution_note: None,
                }),
            )]),
        });
        cache.publish(duplicate_cfg);
        let dup = tokio::time::timeout(std::time::Duration::from_millis(20), watch.next()).await;
        assert!(dup.is_err(), "pointer-equal snapshot must not notify");

        // Top-level errors wake existing watchers and dedupe consecutive repeats.
        cache.publish_error(XdsConfigError("lds missing".into()));
        assert_eq!(
            watch.next().await.unwrap().unwrap_err(),
            XdsConfigError("lds missing".into())
        );
        cache.publish_error(XdsConfigError("lds missing".into()));
        let dup_err =
            tokio::time::timeout(std::time::Duration::from_millis(20), watch.next()).await;
        assert!(dup_err.is_err(), "duplicate error must not notify");
    }

    #[test]
    fn config_view_lists_clusters_and_connectable_endpoints() {
        use crate::client::endpoint::EndpointAddress;
        use crate::xds::resource::endpoints::{HealthStatus, LocalityEndpoints, ResolvedEndpoint};

        let endpoint = |host: &str, health_status| ResolvedEndpoint {
            endpoint: Arc::new(EndpointAddress::new(host, 8080).into()),
            health_status,
            load_balancing_weight: 1,
        };
        let endpoints = Arc::new(EndpointsResource {
            cluster_name: "ok".to_string(),
            localities: vec![LocalityEndpoints {
                locality: None,
                endpoints: vec![
                    endpoint("10.0.0.1", HealthStatus::Healthy),
                    endpoint("10.0.0.2", HealthStatus::Unhealthy),
                    endpoint("10.0.0.3", HealthStatus::Unknown),
                    endpoint("10.0.0.1", HealthStatus::Healthy),
                ],
                load_balancing_weight: 100,
                priority: 0,
            }],
        });
        let snapshot = |name, endpoints, resolution_note: Option<&str>| {
            Ok(ClusterSnapshot {
                cluster: make_cluster(name, LbPolicy::RoundRobin),
                endpoints,
                resolution_note: resolution_note.map(Into::into),
            })
        };
        let config = XdsConfig {
            route_config: make_route_config("rc-1"),
            virtual_host_index: 0,
            clusters: HashMap::from([
                ("ok".to_string(), snapshot("ok", Some(endpoints), None)),
                (
                    "no-eds".to_string(),
                    snapshot("no-eds", None, Some("eds failed")),
                ),
                ("no-cds".to_string(), Err(ClusterError("cds failed".into()))),
            ]),
        };

        let clusters: HashMap<_, _> = config.clusters().collect();
        assert_eq!(clusters.len(), 3);

        let ok = clusters["ok"].unwrap();
        let addresses: Vec<_> = ok.endpoints().unwrap().map(|ep| ep.address()).collect();
        assert_eq!(
            addresses,
            [
                &EndpointAddress::new("10.0.0.1", 8080),
                &EndpointAddress::new("10.0.0.3", 8080),
            ]
        );
        assert_eq!(ok.resolution_note(), None);

        let no_eds = clusters["no-eds"].unwrap();
        assert!(no_eds.endpoints().is_none());
        assert_eq!(no_eds.resolution_note(), Some("eds failed"));

        assert_eq!(clusters["no-cds"].unwrap_err().to_string(), "cds failed");
    }
}
