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

//! Shared XDS cache between the resource manager and Tower service layers.
//!
//! The resource manager writes validated resources into this cache, and consumers
//! use [`CacheWatch`] to receive notifications. Some consumers (e.g. the routing
//! layer) maintain their own local copies for hot-path performance.

use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::watch;

use crate::xds::resource::{ClusterResource, EndpointsResource, RouteConfigResource};

/// Validated per-cluster resources within an [`XdsConfig`] snapshot.
///
/// A cluster enters [`XdsConfig::clusters`] as `Ok(ClusterSnapshot)` only once
/// both its CDS [`ClusterResource`] and EDS [`EndpointsResource`] have resolved.
#[derive(Clone, Debug)]
pub(crate) struct ClusterSnapshot {
    pub(crate) cluster: Arc<ClusterResource>,
    pub(crate) endpoints: Arc<EndpointsResource>,
}

impl ClusterSnapshot {
    fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.cluster, &other.cluster) && Arc::ptr_eq(&self.endpoints, &other.endpoints)
    }
}

/// Per-cluster resolution failure recorded in [`XdsConfig::clusters`] when a
/// referenced cluster's CDS or EDS resource fails without a cached fallback.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub(crate) struct ClusterError(pub(crate) Arc<str>);

/// Top-level xDS configuration failure published when the Listener (LDS),
/// RouteConfiguration (RDS), or authority virtual-host selection fails without
/// a cached fallback.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub(crate) struct XdsConfigError(pub(crate) Arc<str>);

/// Atomic, completeness-gated snapshot of the active route configuration and
/// all clusters referenced by the matched virtual host, aligned with gRFC A74
/// `XdsConfig`.
#[derive(Clone, Debug)]
pub(crate) struct XdsConfig {
    pub(crate) route_config: Arc<RouteConfigResource>,
    /// Index into `route_config.virtual_hosts` matching the channel's target authority.
    pub(crate) virtual_host_index: usize,
    /// Keyed by cluster name; contains an entry (`Ok` or `Err`) for every
    /// cluster referenced by the matched virtual host. Never contains pending
    /// clusters — publication is gated until every referenced cluster resolves.
    pub(crate) clusters: HashMap<String, Result<ClusterSnapshot, ClusterError>>,
}

impl XdsConfig {
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

/// Top-level watch payload published by [`XdsCache`].
pub(crate) type XdsConfigResult = Result<Arc<XdsConfig>, XdsConfigError>;

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
#[cfg(test)]
pub(crate) struct CacheSender<T> {
    tx: watch::Sender<Option<Arc<T>>>,
}

#[cfg(test)]
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

/// A wrapper around [`watch::Receiver`] that exposes only a single `next()`
/// method, preventing misuse of the raw watch API.
pub(crate) struct CacheWatch<T> {
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

    /// Clones this watch so the returned receiver observes the current value
    /// (if any) on its first [`next`](Self::next) call and tracks the same
    /// underlying sender.
    pub(crate) fn resubscribe(&self) -> Self {
        Self::from_receiver(self.rx.clone())
    }

    /// Waits for the next update and returns it.
    ///
    /// Returns `None` if the sender was dropped.
    pub(crate) async fn next(&mut self) -> Option<T> {
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

    /// Whether the watched entry has been removed from the cache.
    ///
    /// A later update for the same key creates a new entry, which this watch
    /// does not follow.
    pub(crate) fn is_closed(&self) -> bool {
        self.rx.has_changed().is_err()
    }
}

impl<T> CacheWatch<Arc<T>> {
    /// Creates a connected sender/receiver pair with an optional initial value.
    ///
    /// When `initial` is `Some`, the returned [`CacheWatch`] yields it on the
    /// first [`next`](Self::next) call even if no further updates are sent.
    #[cfg(test)]
    pub(crate) fn channel(initial: Option<Arc<T>>) -> (CacheSender<T>, Self) {
        let (tx, rx) = watch::channel(initial);
        (CacheSender { tx }, Self::from_receiver(rx))
    }
}

/// A keyed collection of [`watch`] channels for a single xDS resource type.
///
/// Each entry is lazily created on first access (watch or update) and
/// starts with `None`. Writers call
/// [`update_if_changed`](Self::update_if_changed) to set the value; consumers
/// call [`watch`](Self::watch) to receive changes.
struct WatchMap<T> {
    inner: DashMap<String, watch::Sender<Option<Arc<T>>>>,
}

impl<T> WatchMap<T> {
    fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    /// Updates the resource for the given key and notifies watchers.
    ///
    /// Lazily creates the watch channel if this is the first access for the key.
    #[cfg(test)]
    fn update(&self, key: &str, value: Arc<T>) {
        let tx = self.ensure(key);
        tx.send_replace(Some(value));
    }

    /// Updates the resource for the given key only when the `Arc` pointer changed.
    fn update_if_changed(&self, key: &str, value: Arc<T>) {
        let tx = self.ensure(key);
        tx.send_if_modified(|slot| {
            if slot.as_ref().is_some_and(|prev| Arc::ptr_eq(prev, &value)) {
                false
            } else {
                *slot = Some(value);
                true
            }
        });
    }

    /// Watches resource changes for the given key.
    ///
    /// Lazily creates the watch channel if it doesn't exist yet.
    fn watch(&self, key: &str) -> CacheWatch<Arc<T>> {
        let tx = self.ensure(key);
        CacheWatch::from_receiver(tx.subscribe())
    }

    /// Removes the watch channel for the given key.
    ///
    /// Dropping the sender closes all watcher receivers.
    #[cfg(test)]
    fn remove(&self, key: &str) {
        self.inner.remove(key);
    }

    /// Retains only watch channels whose key satisfies `keep`.
    fn retain(&self, mut keep: impl FnMut(&str) -> bool) {
        self.inner.retain(|k, _| keep(k));
    }

    /// Returns the sender for `key`, creating the watch channel if needed.
    fn ensure(&self, key: &str) -> watch::Sender<Option<Arc<T>>> {
        self.inner
            .entry(key.to_string())
            .or_insert_with(|| watch::channel(None).0)
            .value()
            .clone()
    }
}

/// Central cache for validated xDS resources.
///
/// The resource manager writes into this cache as LDS/RDS/CDS/EDS updates arrive.
/// Consumers use [`CacheWatch`] to receive notifications:
///
/// - **Routing layer**: calls [`watch_route_config`](Self::watch_route_config),
///   owns its own `ArcSwap`, and subscribes to [`watch_cluster`](Self::watch_cluster)
///   and [`watch_endpoints`](Self::watch_endpoints) to pass [`ClusterWatches`]
///   into each provisioned cluster channel.
pub(crate) struct XdsCache {
    /// Atomic completeness-gated snapshot stream published by `XdsResourceManager`.
    config_tx: watch::Sender<Option<XdsConfigResult>>,

    /// Active route configuration (from LDS inline or RDS).
    route_config_tx: watch::Sender<Option<Arc<RouteConfigResource>>>,

    /// Per-cluster CDS state with readiness gating.
    clusters: WatchMap<ClusterResource>,

    /// Per-cluster EDS endpoint snapshots with readiness gating.
    endpoints: WatchMap<EndpointsResource>,
}

impl XdsCache {
    /// Creates a new empty cache with no resources.
    pub(crate) fn new() -> Self {
        let (config_tx, _) = watch::channel(None);
        let (route_config_tx, _) = watch::channel(None);
        Self {
            config_tx,
            route_config_tx,
            clusters: WatchMap::new(),
            endpoints: WatchMap::new(),
        }
    }

    /// Publishes a new atomic [`XdsConfig`] snapshot unless an identical
    /// pointer-equal snapshot is already active.
    pub(crate) fn publish(&self, config: Arc<XdsConfig>) {
        let changed = self.config_tx.send_if_modified(|slot| {
            if matches!(slot, Some(Ok(prev)) if prev.ptr_eq(&config)) {
                false
            } else {
                *slot = Some(Ok(Arc::clone(&config)));
                true
            }
        });
        if !changed {
            return;
        }

        // Mirror into per-resource watch channels until XdsRouter switches to
        // `XdsCache::watch` in the follow-up PR.
        for (name, state) in &config.clusters {
            if let Ok(snap) = state {
                self.clusters
                    .update_if_changed(name, Arc::clone(&snap.cluster));
                self.endpoints
                    .update_if_changed(name, Arc::clone(&snap.endpoints));
            }
        }
        self.clusters
            .retain(|name| matches!(config.clusters.get(name), Some(Ok(_))));
        self.endpoints
            .retain(|name| matches!(config.clusters.get(name), Some(Ok(_))));
        self.update_route_config(Arc::clone(&config.route_config));
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
    #[cfg_attr(not(test), allow(dead_code))]
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

    /// Updates the active route configuration and notifies all watchers.
    pub(crate) fn update_route_config(&self, config: Arc<RouteConfigResource>) {
        self.route_config_tx.send_replace(Some(config));
    }

    /// Watches route configuration changes.
    pub(crate) fn watch_route_config(&self) -> CacheWatch<Arc<RouteConfigResource>> {
        CacheWatch::from_receiver(self.route_config_tx.subscribe())
    }

    /// Updates a cluster resource and notifies watchers.
    #[cfg(test)]
    pub(crate) fn update_cluster(&self, name: &str, cluster: Arc<ClusterResource>) {
        self.clusters.update(name, cluster);
    }

    /// Removes a cluster resource and its watch channel.
    #[cfg(test)]
    pub(crate) fn remove_cluster(&self, name: &str) {
        self.clusters.remove(name);
    }

    /// Watches cluster resource changes for a specific cluster.
    pub(crate) fn watch_cluster(&self, name: &str) -> CacheWatch<Arc<ClusterResource>> {
        self.clusters.watch(name)
    }

    /// Updates the endpoint resource for a cluster and notifies watchers.
    #[cfg(test)]
    pub(crate) fn update_endpoints(&self, cluster_name: &str, endpoints: Arc<EndpointsResource>) {
        self.endpoints.update(cluster_name, endpoints);
    }

    /// Watches endpoint changes for a cluster.
    pub(crate) fn watch_endpoints(&self, cluster_name: &str) -> CacheWatch<Arc<EndpointsResource>> {
        self.endpoints.watch(cluster_name)
    }

    /// Removes the endpoint resource and its watch channel.
    #[cfg(test)]
    pub(crate) fn remove_endpoints(&self, cluster_name: &str) {
        self.endpoints.remove(cluster_name);
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
    async fn route_config_update_notifies_watcher() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_route_config();

        cache.update_route_config(make_route_config("rc-1"));

        let val = watch.next().await.unwrap();
        assert_eq!(val.name, "rc-1");
    }

    #[tokio::test]
    async fn route_config_readiness_via_next() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_route_config();

        let handle = tokio::spawn(async move {
            let val = watch.next().await.unwrap();
            val.name.clone()
        });

        cache.update_route_config(make_route_config("rc-delayed"));
        let name = handle.await.unwrap();
        assert_eq!(name, "rc-delayed");
    }

    #[tokio::test]
    async fn cluster_watch_notifies_on_update() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_cluster("c1");

        cache.update_cluster("c1", make_cluster("c1", LbPolicy::LeastRequest));

        let val = watch.next().await.unwrap();
        assert_eq!(val.lb_policy, LbPolicy::LeastRequest);
    }

    #[tokio::test]
    async fn cluster_remove_closes_watcher() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_cluster("c1");
        cache.update_cluster("c1", make_cluster("c1", LbPolicy::RoundRobin));
        watch.next().await; // consume

        cache.remove_cluster("c1");
        assert!(watch.next().await.is_none());
    }

    #[tokio::test]
    async fn endpoint_update_notifies_watcher() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_endpoints("c1");

        cache.update_endpoints("c1", make_endpoints("c1"));

        let val = watch.next().await.unwrap();
        assert_eq!(val.cluster_name, "c1");
        assert_eq!(val.localities.len(), 1);
    }

    #[tokio::test]
    async fn multiple_endpoint_watchers_see_same_update() {
        let cache = XdsCache::new();
        let mut watch1 = cache.watch_endpoints("c1");
        let mut watch2 = cache.watch_endpoints("c1");

        cache.update_endpoints("c1", make_endpoints("c1"));

        assert_eq!(watch1.next().await.unwrap().cluster_name, "c1");
        assert_eq!(watch2.next().await.unwrap().cluster_name, "c1");
    }

    #[tokio::test]
    async fn remove_endpoints_closes_watchers() {
        let cache = XdsCache::new();
        let mut watch = cache.watch_endpoints("c1");
        cache.update_endpoints("c1", make_endpoints("c1"));
        watch.next().await; // consume

        cache.remove_endpoints("c1");
        assert!(watch.next().await.is_none());
    }

    #[tokio::test]
    async fn cascade_cds_then_eds_readiness() {
        let cache = XdsCache::new();

        let handle = tokio::spawn({
            let mut cluster_watch = cache.watch_cluster("c1");
            let mut endpoint_watch = cache.watch_endpoints("c1");
            async move {
                let cluster = cluster_watch.next().await.unwrap();
                assert_eq!(cluster.lb_policy, LbPolicy::RoundRobin);

                let eps = endpoint_watch.next().await.unwrap();
                assert_eq!(eps.cluster_name, "c1");
            }
        });

        cache.update_cluster("c1", make_cluster("c1", LbPolicy::RoundRobin));
        cache.update_endpoints("c1", make_endpoints("c1"));

        handle.await.unwrap();

        let cluster = cache.watch_cluster("c1").next().await.unwrap();
        assert_eq!(cluster.name, "c1");
        assert_eq!(cluster.lb_policy, LbPolicy::RoundRobin);

        let eps = cache.watch_endpoints("c1").next().await.unwrap();
        assert_eq!(eps.cluster_name, "c1");
        assert_eq!(eps.localities.len(), 1);
    }

    #[tokio::test]
    async fn late_watcher_sees_existing_value() {
        let cache = XdsCache::new();
        cache.update_route_config(make_route_config("rc-1"));
        cache.update_cluster("c1", make_cluster("c1", LbPolicy::RoundRobin));
        cache.update_endpoints("c1", make_endpoints("c1"));

        assert!(cache.watch_route_config().next().await.is_some());
        assert!(cache.watch_cluster("c1").next().await.is_some());
        assert!(cache.watch_endpoints("c1").next().await.is_some());
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
    async fn resubscribe_yields_current_value_and_tracks_same_sender() {
        let cache = XdsCache::new();
        let entry = cache.watch_cluster("c1");
        let mut sub = entry.resubscribe();

        let c1 = make_cluster("c1", LbPolicy::RoundRobin);
        cache.update_cluster("c1", Arc::clone(&c1));
        assert!(Arc::ptr_eq(&sub.next().await.unwrap(), &c1));

        // Late resubscribe sees existing value immediately and closes with entry.
        let mut late_sub = entry.resubscribe();
        assert!(Arc::ptr_eq(&late_sub.next().await.unwrap(), &c1));

        cache.remove_cluster("c1");
        assert!(entry.is_closed());
        assert!(sub.next().await.is_none());
        assert!(late_sub.next().await.is_none());
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
                    endpoints: Arc::clone(&ep1),
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
                    endpoints: ep1,
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
}
