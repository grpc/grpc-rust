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

//! xDS resource manager: LDS -> RDS -> VirtualHost -> CDS -> EDS cascade.
//!
//! The [`XdsResourceManager`] bridges the xDS client (ADS protocol layer) to the
//! [`XdsCache`]. It watches resources via [`XdsClient::watch()`], scopes child
//! subscriptions to the channel authority's matching virtual host, deduplicates
//! EDS watches by `eds_service_name`, coalesces bursts of ready watcher events,
//! and publishes completeness-gated atomic [`XdsConfig`] snapshots (or top-level
//! [`XdsConfigError`]s) into the cache.
//!
//! # Cascade
//!
//! ```text
//! LDS -> RDS (or inline) -> matched VirtualHost -> CDS (per cluster) -> EDS (per eds_service_name)
//! ```
//!
//! Each level determines the subscriptions for the next. Publication is gated
//! until the active route configuration and every cluster referenced by the
//! matched virtual host have resolved (either `Ok` with both CDS and EDS, or
//! `Err` with a per-cluster [`ClusterError`]), matching gRFC A74
//! `XdsDependencyManager` semantics.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use xds_client::{ProcessingDone, Resource, ResourceEvent, ResourceWatcher, XdsClient};

use crate::common::async_util::AbortOnDrop;
use crate::xds::cache::{ClusterError, ClusterSnapshot, XdsCache, XdsConfig, XdsConfigError};
use crate::xds::resource::listener::RouteSource;
use crate::xds::resource::{
    ClusterResource, EndpointsResource, ListenerResource, RouteConfigResource,
};
use crate::xds::routing::find_best_matching_virtual_host_index;

/// Manages the LDS -> RDS -> VirtualHost -> CDS -> EDS cascade.
///
/// Subscribes to xDS resources via [`XdsClient::watch()`] and writes validated
/// resources into [`XdsCache`]. Dropping the manager aborts the background task.
pub(crate) struct XdsResourceManager {
    _task: AbortOnDrop,
}

impl XdsResourceManager {
    /// Creates a new resource manager and starts the cascade.
    ///
    /// # Arguments
    /// * `xds_client` - The xDS client for creating resource watches
    /// * `cache` - The shared cache to write resources into
    /// * `listener_name` - The LDS resource name to watch (from target URI)
    /// * `authority` - Channel authority used to select the active `VirtualHost`
    pub(crate) fn new(
        xds_client: XdsClient,
        cache: Arc<XdsCache>,
        listener_name: String,
        authority: impl Into<Arc<str>>,
    ) -> Self {
        let state = CascadeState::new(&xds_client, listener_name, authority.into());
        let handle = tokio::spawn(state.run(xds_client, cache));
        Self {
            _task: AbortOnDrop(handle),
        }
    }
}

/// Resolution state of an individual watched xDS resource.
enum WatchState<T> {
    /// Subscribed, awaiting the first response from `xds-client`.
    Pending,
    /// Validated resource received (and retained across [`ResourceEvent::AmbientError`]s).
    Ready(Arc<T>),
    /// Resource invalidated (`ResourceDoesNotExist` or initial validation failure
    /// with no cached fallback).
    Failed(Arc<str>),
}

impl<T> WatchState<T> {
    /// Applies a [`ResourceEvent`] and returns its [`ProcessingDone`] token so
    /// the caller can register any cascading child watches before ACKing.
    fn apply(
        &mut self,
        event: ResourceEvent<T>,
        describe: impl FnOnce(xds_client::Error) -> Arc<str>,
    ) -> ProcessingDone {
        match event {
            ResourceEvent::ResourceChanged {
                result: Ok(resource),
                done,
            } => {
                *self = Self::Ready(resource);
                done
            }
            ResourceEvent::ResourceChanged {
                result: Err(err),
                done,
            } => {
                // Per `xds-client`'s `ResourceEvent` contract, `ResourceChanged(Err(_))`
                // invalidates any previously cached resource, whereas non-invalidating
                // errors arrive as `AmbientError` below (gRFC A88).
                *self = Self::Failed(describe(err));
                done
            }
            ResourceEvent::AmbientError { done, .. } => done,
        }
    }
}

/// Active subscription paired with its latest [`WatchState`].
struct Watch<T: Resource> {
    watcher: ResourceWatcher<T>,
    state: WatchState<T>,
}

impl<T: Resource> Watch<T> {
    fn new(xds_client: &XdsClient, name: &str) -> Self {
        Self {
            watcher: xds_client.watch::<T>(name),
            state: WatchState::Pending,
        }
    }

    fn poll_drain(
        &mut self,
        cx: &mut Context<'_>,
        kind: &str,
        name: &str,
        dones: &mut Vec<ProcessingDone>,
        closed: &mut bool,
    ) {
        loop {
            match self.watcher.poll_next(cx) {
                Poll::Ready(Some(ev)) => dones.push(
                    self.state
                        .apply(ev, |err| format!("{kind} resource '{name}': {err}").into()),
                ),
                Poll::Ready(None) => {
                    *closed = true;
                    break;
                }
                Poll::Pending => break,
            }
        }
    }
}

struct WatchedRds {
    name: String,
    watch: Watch<RouteConfigResource>,
}

struct CachedEffectiveRoute {
    raw_rc: Arc<RouteConfigResource>,
    listener_msd: Option<Duration>,
    effective_rc: Arc<RouteConfigResource>,
}

/// Diffs `map` against `desired`, dropping removed watches (unsubscribing) and
/// starting [`WatchState::Pending`] watches for newly referenced names.
fn reconcile<T: Resource>(
    xds_client: &XdsClient,
    map: &mut HashMap<String, Watch<T>>,
    desired: &HashSet<&str>,
) {
    map.retain(|name, _| desired.contains(name.as_str()));
    for &name in desired {
        if !map.contains_key(name) {
            map.insert(name.to_string(), Watch::new(xds_client, name));
        }
    }
}

/// Mutable state for the entire LDS -> RDS -> VirtualHost -> CDS -> EDS cascade.
///
/// All four resource levels are polled in a single task via [`run`](Self::run).
struct CascadeState {
    listener_name: String,
    authority: Arc<str>,
    lds: Watch<ListenerResource>,
    /// Active RDS subscription — `None` if the listener uses inline routes.
    rds: Option<WatchedRds>,
    /// Memoized route config with the listener's default max stream duration applied.
    cached_effective_route: Option<CachedEffectiveRoute>,
    /// Active CDS watches for clusters referenced by the matched virtual host,
    /// keyed by cluster name.
    clusters: HashMap<String, Watch<ClusterResource>>,
    /// Active EDS watches for ready clusters, keyed by `eds_service_name` so
    /// clusters sharing an EDS resource share a single subscription.
    endpoints: HashMap<String, Watch<EndpointsResource>>,
}

impl CascadeState {
    fn new(xds_client: &XdsClient, listener_name: String, authority: Arc<str>) -> Self {
        let lds = Watch::new(xds_client, &listener_name);
        Self {
            listener_name,
            authority,
            lds,
            rds: None,
            cached_effective_route: None,
            clusters: HashMap::new(),
            endpoints: HashMap::new(),
        }
    }

    /// Runs the cascade loop, draining all ready watcher events per wakeup,
    /// resolving the dependency graph once per batch, and then dropping all
    /// [`ProcessingDone`] tokens so `xds-client` ACKs after cascading child
    /// watches are registered.
    async fn run(mut self, xds_client: XdsClient, cache: Arc<XdsCache>) {
        let mut dones = Vec::new();
        while std::future::poll_fn(|cx| self.poll_batch(cx, &mut dones)).await {
            self.resolve(&xds_client, &cache);
            dones.clear();
        }
    }

    /// Drains all currently ready events across LDS, RDS, CDS, and EDS in a
    /// single `O(C + E)` pass so SotW/batched ADS responses coalesce into one
    /// [`resolve`](Self::resolve) pass.
    fn poll_batch(&mut self, cx: &mut Context<'_>, dones: &mut Vec<ProcessingDone>) -> Poll<bool> {
        let mut closed = false;

        let lds_before = dones.len();
        self.lds
            .poll_drain(cx, "LDS", &self.listener_name, dones, &mut closed);
        if dones.len() > lds_before {
            self.prune_stale_rds();
        }

        if let Some(rds) = &mut self.rds {
            rds.watch
                .poll_drain(cx, "RDS", &rds.name, dones, &mut closed);
        }

        for (name, watch) in &mut self.clusters {
            watch.poll_drain(cx, "CDS", name, dones, &mut closed);
        }

        for (eds_name, watch) in &mut self.endpoints {
            watch.poll_drain(cx, "EDS", eds_name, dones, &mut closed);
        }

        if !dones.is_empty() {
            Poll::Ready(true)
        } else if closed {
            Poll::Ready(false)
        } else {
            Poll::Pending
        }
    }

    fn prune_stale_rds(&mut self) {
        let keep_rds = match &self.lds.state {
            WatchState::Ready(listener) => match &listener.route_source {
                RouteSource::Rds(rds_name) => {
                    self.rds.as_ref().is_some_and(|rds| rds.name == *rds_name)
                }
                RouteSource::Inline(_) => false,
            },
            WatchState::Pending | WatchState::Failed(_) => false,
        };
        if !keep_rds {
            self.rds = None;
        }
    }

    /// Evaluates the full dependency graph and publishes either a complete
    /// [`XdsConfig`] snapshot or a top-level [`XdsConfigError`], or holds
    /// publication if any required resource is still [`WatchState::Pending`].
    fn resolve(&mut self, xds_client: &XdsClient, cache: &XdsCache) {
        let listener = match &self.lds.state {
            WatchState::Pending => return,
            WatchState::Failed(reason) => {
                let err = XdsConfigError(Arc::clone(reason));
                self.rds = None;
                self.clear_clusters();
                cache.publish_error(err);
                return;
            }
            WatchState::Ready(listener) => Arc::clone(listener),
        };

        let raw_rc = match &listener.route_source {
            RouteSource::Inline(rc) => {
                self.rds = None;
                Arc::clone(rc)
            }
            RouteSource::Rds(rds_name) => {
                if self.rds.as_ref().is_none_or(|rds| rds.name != *rds_name) {
                    self.rds = Some(WatchedRds {
                        name: rds_name.clone(),
                        watch: Watch::new(xds_client, rds_name),
                    });
                }
                match &self.rds.as_ref().expect("set above").watch.state {
                    WatchState::Pending => return,
                    WatchState::Failed(reason) => {
                        let err = XdsConfigError(Arc::clone(reason));
                        self.clear_clusters();
                        cache.publish_error(err);
                        return;
                    }
                    WatchState::Ready(rc) => Arc::clone(rc),
                }
            }
        };

        let Some(virtual_host_index) =
            find_best_matching_virtual_host_index(&self.authority, &raw_rc.virtual_hosts)
        else {
            self.clear_clusters();
            cache.publish_error(XdsConfigError(
                format!(
                    "no matching virtual host for authority '{}' in route configuration '{}'",
                    self.authority, raw_rc.name
                )
                .into(),
            ));
            return;
        };

        let required_clusters = raw_rc.virtual_hosts[virtual_host_index].cluster_names();
        reconcile(xds_client, &mut self.clusters, &required_clusters);

        let required_eds: HashSet<&str> = self
            .clusters
            .values()
            .filter_map(|watch| match &watch.state {
                WatchState::Ready(cluster) => Some(cluster.eds_service_name()),
                WatchState::Pending | WatchState::Failed(_) => None,
            })
            .collect();
        reconcile(xds_client, &mut self.endpoints, &required_eds);

        let mut clusters = HashMap::with_capacity(self.clusters.len());
        for (name, watch) in &self.clusters {
            let outcome = match &watch.state {
                WatchState::Pending => return,
                WatchState::Failed(reason) => Err(ClusterError(Arc::clone(reason))),
                WatchState::Ready(cluster) => {
                    let eds_name = cluster.eds_service_name();
                    match &self
                        .endpoints
                        .get(eds_name)
                        .expect("reconciled from Ready clusters above")
                        .state
                    {
                        WatchState::Pending => return,
                        WatchState::Failed(reason) => Err(ClusterError(Arc::clone(reason))),
                        WatchState::Ready(endpoints) => Ok(ClusterSnapshot {
                            cluster: Arc::clone(cluster),
                            endpoints: Arc::clone(endpoints),
                        }),
                    }
                }
            };
            clusters.insert(name.clone(), outcome);
        }

        let effective_rc = self.effective_route_config(&raw_rc, listener.max_stream_duration);
        cache.publish(Arc::new(XdsConfig {
            route_config: effective_rc,
            virtual_host_index,
            clusters,
        }));
    }

    /// Returns `rc` with the listener's default max stream duration applied to
    /// routes that set none (gRFC A31), memoizing the derived [`Arc`] so
    /// unrelated CDS/EDS or unchanged LDS updates preserve pointer identity.
    fn effective_route_config(
        &mut self,
        raw_rc: &Arc<RouteConfigResource>,
        listener_msd: Option<Duration>,
    ) -> Arc<RouteConfigResource> {
        // Normalize Some(0) to None since zero means no limit, matching unset.
        let normalized_msd = listener_msd.filter(|d| !d.is_zero());
        if let Some(cached) = &self.cached_effective_route
            && Arc::ptr_eq(&cached.raw_rc, raw_rc)
            && cached.listener_msd == normalized_msd
        {
            return Arc::clone(&cached.effective_rc);
        }
        let effective_rc = match normalized_msd {
            Some(default) => Arc::new((**raw_rc).clone().with_default_max_stream_duration(default)),
            None => Arc::clone(raw_rc),
        };
        self.cached_effective_route = Some(CachedEffectiveRoute {
            raw_rc: Arc::clone(raw_rc),
            listener_msd: normalized_msd,
            effective_rc: Arc::clone(&effective_rc),
        });
        effective_rc
    }

    /// Cancels all active CDS and EDS watches when the parent Listener,
    /// RouteConfiguration, or virtual-host lookup enters an error state.
    fn clear_clusters(&mut self) {
        self.clusters.clear();
        self.endpoints.clear();
    }

    #[cfg(test)]
    fn handle_lds(
        &mut self,
        event: ResourceEvent<ListenerResource>,
        xds_client: &XdsClient,
        cache: &XdsCache,
    ) {
        let name = &self.listener_name;
        let _done = self
            .lds
            .state
            .apply(event, |err| format!("LDS resource '{name}': {err}").into());
        self.prune_stale_rds();
        self.resolve(xds_client, cache);
    }

    #[cfg(test)]
    fn handle_rds(
        &mut self,
        event: ResourceEvent<RouteConfigResource>,
        xds_client: &XdsClient,
        cache: &XdsCache,
    ) {
        let Some(rds) = &mut self.rds else {
            return;
        };
        let name = &rds.name;
        let _done = rds
            .watch
            .state
            .apply(event, |err| format!("RDS resource '{name}': {err}").into());
        self.resolve(xds_client, cache);
    }

    #[cfg(test)]
    fn handle_cds(
        &mut self,
        cluster_name: &str,
        event: ResourceEvent<ClusterResource>,
        xds_client: &XdsClient,
        cache: &XdsCache,
    ) {
        let Some(watch) = self.clusters.get_mut(cluster_name) else {
            return;
        };
        let _done = watch.state.apply(event, |err| {
            format!("CDS resource '{cluster_name}': {err}").into()
        });
        self.resolve(xds_client, cache);
    }

    #[cfg(test)]
    fn handle_eds(
        &mut self,
        eds_name: &str,
        event: ResourceEvent<EndpointsResource>,
        xds_client: &XdsClient,
        cache: &XdsCache,
    ) {
        let Some(watch) = self.endpoints.get_mut(eds_name) else {
            return;
        };
        let _done = watch.state.apply(event, |err| {
            format!("EDS resource '{eds_name}': {err}").into()
        });
        self.resolve(xds_client, cache);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xds::resource::route_config::{
        PathSpecifierConfig, RouteConfig, RouteConfigAction, RouteConfigMatch, VirtualHostConfig,
    };

    fn test_client() -> XdsClient {
        XdsClient::disconnected()
    }

    fn test_cache() -> Arc<XdsCache> {
        Arc::new(XdsCache::new())
    }

    fn new_state(client: &XdsClient) -> CascadeState {
        CascadeState::new(client, "listener".into(), "listener".into())
    }

    fn new_state_with_authority(client: &XdsClient, authority: &str) -> CascadeState {
        CascadeState::new(client, "listener".into(), authority.into())
    }

    fn make_vhost(name: &str, domains: &[&str], clusters: &[&str]) -> VirtualHostConfig {
        VirtualHostConfig {
            name: name.into(),
            domains: domains.iter().map(|d| (*d).into()).collect(),
            routes: clusters
                .iter()
                .map(|c| RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster((*c).into()),
                    retry_config: None,
                    max_stream_duration: None,
                })
                .collect(),
        }
    }

    fn make_route_config(name: &str, clusters: &[&str]) -> Arc<RouteConfigResource> {
        Arc::new(RouteConfigResource {
            name: name.into(),
            virtual_hosts: vec![make_vhost("vh", &["*"], clusters)],
            metadata: Default::default(),
        })
    }

    fn make_listener_inline(clusters: &[&str]) -> Arc<ListenerResource> {
        Arc::new(ListenerResource {
            name: "listener".into(),
            route_source: RouteSource::Inline(Arc::new(RouteConfigResource {
                name: "inline-rc".into(),
                virtual_hosts: vec![make_vhost("vh", &["*"], clusters)],
                metadata: Default::default(),
            })),
            max_stream_duration: None,
        })
    }

    fn make_listener_rds(rds_name: &str) -> Arc<ListenerResource> {
        make_listener_rds_with_msd(rds_name, None)
    }

    fn make_listener_rds_with_msd(
        rds_name: &str,
        max_stream_duration: Option<Duration>,
    ) -> Arc<ListenerResource> {
        Arc::new(ListenerResource {
            name: "listener".into(),
            route_source: RouteSource::Rds(rds_name.into()),
            max_stream_duration,
        })
    }

    fn make_cluster_resource(name: &str, eds_service_name: Option<&str>) -> Arc<ClusterResource> {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        Arc::new(ClusterResource {
            name: name.into(),
            eds_service_name: eds_service_name.map(Into::into),
            lb_policy: LbPolicy::RoundRobin,
            security: None,
            circuit_breaking: CircuitBreakingConfig::default(),
            outlier_detection: Default::default(),
        })
    }

    fn make_endpoints_resource(cluster_name: &str) -> Arc<EndpointsResource> {
        Arc::new(EndpointsResource {
            cluster_name: cluster_name.into(),
            localities: vec![],
        })
    }

    fn ok_event<T>(resource: Arc<T>) -> ResourceEvent<T> {
        ResourceEvent::ResourceChanged {
            result: Ok(resource),
            done: ProcessingDone::detached(),
        }
    }

    fn does_not_exist_event<T>() -> ResourceEvent<T> {
        ResourceEvent::ResourceChanged {
            result: Err(xds_client::Error::ResourceDoesNotExist),
            done: ProcessingDone::detached(),
        }
    }

    fn validation_err_event<T>() -> ResourceEvent<T> {
        ResourceEvent::ResourceChanged {
            result: Err(xds_client::Error::Validation("invalid resource".into())),
            done: ProcessingDone::detached(),
        }
    }

    fn ambient_event<T>() -> ResourceEvent<T> {
        ResourceEvent::AmbientError {
            error: xds_client::Error::Validation("post-ACK NACK".into()),
            done: ProcessingDone::detached(),
        }
    }

    fn resolve_cluster(state: &mut CascadeState, name: &str, client: &XdsClient, cache: &XdsCache) {
        state.handle_cds(
            name,
            ok_event(make_cluster_resource(name, None)),
            client,
            cache,
        );
        state.handle_eds(name, ok_event(make_endpoints_resource(name)), client, cache);
    }

    #[test]
    fn reconcile_adds_new_clusters() {
        let client = test_client();
        let mut state = new_state(&client);

        let desired = HashSet::from(["a", "b"]);
        reconcile(&client, &mut state.clusters, &desired);

        assert!(state.clusters.contains_key("a"));
        assert!(state.clusters.contains_key("b"));
        assert_eq!(state.clusters.len(), 2);
    }

    #[test]
    fn reconcile_removes_old_clusters() {
        let client = test_client();
        let mut state = new_state(&client);

        let d1 = HashSet::from(["a", "b"]);
        reconcile(&client, &mut state.clusters, &d1);

        let d2 = HashSet::from(["b", "c"]);
        reconcile(&client, &mut state.clusters, &d2);

        assert!(!state.clusters.contains_key("a"));
        assert!(state.clusters.contains_key("b"));
        assert!(state.clusters.contains_key("c"));
        assert_eq!(state.clusters.len(), 2);
    }

    #[test]
    fn reconcile_to_empty_removes_all() {
        let client = test_client();
        let mut state = new_state(&client);

        let d1 = HashSet::from(["a"]);
        reconcile(&client, &mut state.clusters, &d1);
        assert_eq!(state.clusters.len(), 1);

        reconcile(&client, &mut state.clusters, &HashSet::new());
        assert!(state.clusters.is_empty());
    }

    #[tokio::test]
    async fn handle_rds_gates_until_all_referenced_clusters_resolve() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc-1")), &client, &cache);
        let rc = make_route_config("rc-1", &["cluster-a", "cluster-b"]);
        state.handle_rds(ok_event(rc), &client, &cache);

        // CDS watchers start immediately, but publication is gated until both
        // clusters complete CDS + EDS.
        assert!(state.clusters.contains_key("cluster-a"));
        assert!(state.clusters.contains_key("cluster-b"));
        assert!(cache.current_result().is_none());

        resolve_cluster(&mut state, "cluster-a", &client, &cache);
        assert!(
            cache.current_result().is_none(),
            "cluster-b is still pending"
        );

        resolve_cluster(&mut state, "cluster-b", &client, &cache);
        let snapshot = cache.watch().next().await.unwrap().unwrap();
        assert_eq!(snapshot.route_config.name, "rc-1");
        assert!(snapshot.clusters["cluster-a"].is_ok());
        assert!(snapshot.clusters["cluster-b"].is_ok());
    }

    #[test]
    fn resource_changed_validation_error_invalidates_state() {
        // `xds-client` emits `ResourceChanged(Err(Validation))` only when no
        // valid cached fallback exists; `WatchState::apply` must treat every
        // `ResourceChanged(Err(_))` as resource invalidation rather than keeping
        // stale `Ready` state.
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        state.handle_rds(
            ok_event(make_route_config("rc", &["c1", "c2"])),
            &client,
            &cache,
        );
        resolve_cluster(&mut state, "c1", &client, &cache);
        resolve_cluster(&mut state, "c2", &client, &cache);
        assert!(cache.current().is_some());

        // Cluster-level `ResourceChanged(Err(Validation))` transitions that
        // cluster to `Err(ClusterError)` and drops its EDS subscription while
        // keeping healthy siblings active.
        state.handle_cds("c1", validation_err_event(), &client, &cache);
        let snap = cache.current().unwrap();
        let c1_err = snap.clusters["c1"].as_ref().unwrap_err();
        assert!(c1_err.0.contains("CDS resource 'c1'"));
        assert!(c1_err.0.contains("invalid resource"));
        assert!(snap.clusters["c2"].is_ok());
        assert!(!state.endpoints.contains_key("c1"));
        assert!(state.endpoints.contains_key("c2"));

        // EDS `ResourceChanged(Err(Validation))` marks c2 failed.
        state.handle_eds("c2", validation_err_event(), &client, &cache);
        let snap = cache.current().unwrap();
        let c2_err = snap.clusters["c2"].as_ref().unwrap_err();
        assert!(c2_err.0.contains("EDS resource 'c2'"));

        // RDS `ResourceChanged(Err(Validation))` publishes top-level error and
        // cancels descendant CDS/EDS watches.
        state.handle_rds(validation_err_event(), &client, &cache);
        let err = cache.current_result().unwrap().unwrap_err();
        assert!(err.0.contains("RDS resource 'rc'"));
        assert!(state.rds.is_some());
        assert!(state.clusters.is_empty());
        assert!(state.endpoints.is_empty());

        // LDS `ResourceChanged(Err(Validation))` publishes top-level error and
        // cancels RDS as well.
        state.handle_lds(validation_err_event(), &client, &cache);
        let err = cache.current_result().unwrap().unwrap_err();
        assert!(err.0.contains("LDS resource 'listener'"));
        assert!(state.rds.is_none());
    }

    #[test]
    fn ambient_errors_preserve_state() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        state.handle_rds(ok_event(make_route_config("rc", &["c1"])), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);
        let initial = cache.current().unwrap();

        state.handle_lds(ambient_event(), &client, &cache);
        state.handle_rds(ambient_event(), &client, &cache);
        state.handle_cds("c1", ambient_event(), &client, &cache);
        state.handle_eds("c1", ambient_event(), &client, &cache);

        assert!(Arc::ptr_eq(&cache.current().unwrap(), &initial));
        assert!(state.rds.is_some());
        assert_eq!(state.clusters.len(), 1);
        assert_eq!(state.endpoints.len(), 1);
    }

    #[test]
    fn lds_and_rds_failures_publish_top_level_error_and_recover() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        // Initial LDS missing -> top-level XdsConfigError.
        state.handle_lds(does_not_exist_event(), &client, &cache);
        let err = cache.current_result().unwrap().unwrap_err();
        assert!(
            err.0.contains("LDS resource 'listener'"),
            "unexpected error: {err}"
        );

        // LDS arrives pointing at RDS, RDS missing -> top-level XdsConfigError.
        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        state.handle_rds(does_not_exist_event(), &client, &cache);
        let err = cache.current_result().unwrap().unwrap_err();
        assert!(
            err.0.contains("RDS resource 'rc'"),
            "unexpected error: {err}"
        );

        // RDS + cluster recover -> publishes Ok(XdsConfig).
        state.handle_rds(ok_event(make_route_config("rc", &["c1"])), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);
        assert_eq!(cache.current().unwrap().route_config.name, "rc");

        // Subsequent RDS deletion cancels child CDS/EDS watches while keeping
        // the RDS watch itself active so RDS recreation can recover.
        state.handle_rds(does_not_exist_event(), &client, &cache);
        assert!(cache.current_result().unwrap().is_err());
        assert!(state.rds.is_some());
        assert!(state.clusters.is_empty());
        assert!(state.endpoints.is_empty());

        // RDS returns and cluster resolves -> recovers again.
        state.handle_rds(ok_event(make_route_config("rc", &["c1"])), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);
        assert_eq!(cache.current().unwrap().route_config.name, "rc");

        // Subsequent LDS deletion cancels RDS and all descendant CDS/EDS watches.
        state.handle_lds(does_not_exist_event(), &client, &cache);
        assert!(cache.current_result().unwrap().is_err());
        assert!(state.rds.is_none());
        assert!(state.clusters.is_empty());
        assert!(state.endpoints.is_empty());
    }

    #[test]
    fn partial_cluster_failure_publishes_ok_and_err_entries_together() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(
            ok_event(make_listener_inline(&["healthy", "bad-cds", "bad-eds"])),
            &client,
            &cache,
        );
        resolve_cluster(&mut state, "healthy", &client, &cache);
        state.handle_cds("bad-cds", does_not_exist_event(), &client, &cache);
        assert!(
            cache.current_result().is_none(),
            "bad-eds is still pending, so whole snapshot must wait"
        );

        state.handle_cds(
            "bad-eds",
            ok_event(make_cluster_resource("bad-eds", Some("eds-missing"))),
            &client,
            &cache,
        );
        assert!(
            cache.current_result().is_none(),
            "bad-eds EDS has not resolved yet"
        );

        state.handle_eds("eds-missing", does_not_exist_event(), &client, &cache);
        let snap = cache.current().expect("all clusters have resolved");
        assert!(snap.clusters["healthy"].is_ok());
        let cds_err = snap.clusters["bad-cds"].as_ref().unwrap_err();
        assert!(cds_err.0.contains("CDS resource 'bad-cds'"));
        let eds_err = snap.clusters["bad-eds"].as_ref().unwrap_err();
        assert!(eds_err.0.contains("EDS resource 'eds-missing'"));

        // When bad-cds later recovers, the snapshot updates its entry to Ok.
        resolve_cluster(&mut state, "bad-cds", &client, &cache);
        let snap = cache.current().unwrap();
        assert!(snap.clusters["bad-cds"].is_ok());
    }

    #[tokio::test]
    async fn handle_lds_inline_writes_route_config_once_clusters_resolve() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_inline(&["c1"])), &client, &cache);
        assert!(state.rds.is_none());
        assert!(state.clusters.contains_key("c1"));
        assert!(cache.current_result().is_none());

        resolve_cluster(&mut state, "c1", &client, &cache);
        let snapshot = cache.watch().next().await.unwrap().unwrap();
        assert_eq!(snapshot.route_config.name, "inline-rc");
        assert!(snapshot.clusters["c1"].is_ok());
    }

    #[test]
    fn handle_lds_inline_clears_existing_rds() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc-1")), &client, &cache);
        assert_eq!(state.rds.as_ref().map(|r| r.name.as_str()), Some("rc-1"));

        state.handle_lds(ok_event(make_listener_inline(&[])), &client, &cache);
        assert!(state.rds.is_none());
        assert_eq!(cache.current().unwrap().route_config.name, "inline-rc");
    }

    #[test]
    fn handle_lds_rds_sets_reuses_and_replaces_watcher() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc-1")), &client, &cache);
        assert_eq!(state.rds.as_ref().map(|r| r.name.as_str()), Some("rc-1"));

        state.handle_lds(ok_event(make_listener_rds("rc-1")), &client, &cache);
        assert_eq!(state.rds.as_ref().map(|r| r.name.as_str()), Some("rc-1"));

        state.handle_lds(ok_event(make_listener_rds("rc-2")), &client, &cache);
        assert_eq!(state.rds.as_ref().map(|r| r.name.as_str()), Some("rc-2"));
    }

    /// The route config currently published by `cache`.
    async fn published(cache: &XdsCache) -> Arc<RouteConfigResource> {
        Arc::clone(&cache.watch().next().await.unwrap().unwrap().route_config)
    }

    /// The max stream duration of the published config's first route.
    async fn published_msd(cache: &XdsCache) -> Option<Duration> {
        published(cache).await.virtual_hosts[0].routes[0].max_stream_duration
    }

    #[tokio::test]
    async fn handle_lds_inline_applies_listener_max_stream_duration() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        let mut listener = Arc::into_inner(make_listener_inline(&["c1"])).unwrap();
        listener.max_stream_duration = Some(Duration::from_secs(4));
        state.handle_lds(ok_event(Arc::new(listener)), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);

        assert_eq!(published_msd(&cache).await, Some(Duration::from_secs(4)));
    }

    #[tokio::test]
    async fn rds_route_config_follows_listener_max_stream_duration() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);
        let secs = |s| Some(Duration::from_secs(s));

        // The RDS config is published with the listener's default applied once c1 resolves.
        let listener = make_listener_rds_with_msd("rc", secs(7));
        state.handle_lds(ok_event(Arc::clone(&listener)), &client, &cache);
        let rc = make_route_config("rc", &["c1"]);
        state.handle_rds(ok_event(Arc::clone(&rc)), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);
        let config = published(&cache).await;
        assert_eq!(config.name, "rc");
        assert_eq!(published_msd(&cache).await, secs(7));

        // An unchanged listener does not republish.
        state.handle_lds(ok_event(listener), &client, &cache);
        assert!(Arc::ptr_eq(&published(&cache).await, &config));

        // A listener-only change republishes the route config with the new
        // default, applied to the config as received rather than the old one.
        let listener = make_listener_rds_with_msd("rc", secs(3));
        state.handle_lds(ok_event(listener), &client, &cache);
        assert_eq!(published_msd(&cache).await, secs(3));
        let listener = make_listener_rds_with_msd("rc", None);
        state.handle_lds(ok_event(listener), &client, &cache);
        assert!(Arc::ptr_eq(&published(&cache).await, &rc));

        // A new RDS name keeps serving the old config until the new one arrives,
        // which is then published with the new default.
        let listener = make_listener_rds_with_msd("rc-2", secs(9));
        state.handle_lds(ok_event(listener), &client, &cache);
        assert!(Arc::ptr_eq(&published(&cache).await, &rc));

        state.handle_rds(
            ok_event(make_route_config("rc-2", &["c1"])),
            &client,
            &cache,
        );
        assert_eq!(published(&cache).await.name, "rc-2");
        assert_eq!(published_msd(&cache).await, secs(9));
    }

    /// An RDS update registers all referenced clusters through the real cascade
    /// path while holding its `ProcessingDone` token.
    #[tokio::test]
    async fn rds_referencing_many_clusters_reconciles_without_deadlock() {
        use xds_client::{
            ClientConfig, Node, ProstCodec, TokioRuntime, TonicTransportBuilder,
            XdsClient as RealXdsClient,
        };
        use xds_test_util::{XdsTestControlPlaneService, config};

        const CLUSTER_COUNT: usize = 100;

        let control_plane = XdsTestControlPlaneService::new()
            .start()
            .await
            .expect("control plane failed to start");
        control_plane.get_service().set_xds_config(
            &config::AdsTypeUrl::Rds,
            HashMap::from([(
                "rc-many".to_string(),
                config::build_route_config_with_cluster_count("rc-many", CLUSTER_COUNT),
            )]),
        );

        let client_config = ClientConfig::new(
            Node::new("test", "0"),
            format!("http://{}", control_plane.addr()),
        );
        let xds_client: RealXdsClient = RealXdsClient::builder(
            client_config,
            TonicTransportBuilder::new(),
            ProstCodec,
            TokioRuntime,
        )
        .build();

        let cache = test_cache();
        let disconnected = test_client();
        let mut state = new_state(&disconnected);
        state.handle_lds(ok_event(make_listener_rds("rc-many")), &xds_client, &cache);

        let event = tokio::time::timeout(
            Duration::from_secs(10),
            state.rds.as_mut().unwrap().watch.watcher.next(),
        )
        .await
        .expect("timed out waiting for the RDS update")
        .expect("watcher closed");

        state.handle_rds(event, &xds_client, &cache);

        assert_eq!(state.clusters.len(), CLUSTER_COUNT);
    }

    #[tokio::test]
    async fn route_change_adding_cluster_holds_previous_snapshot_until_new_cluster_resolves() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        state.handle_rds(ok_event(make_route_config("rc", &["c1"])), &client, &cache);
        let c1 = make_cluster_resource("c1", None);
        let ep1 = make_endpoints_resource("c1");
        state.handle_cds("c1", ok_event(Arc::clone(&c1)), &client, &cache);
        state.handle_eds("c1", ok_event(Arc::clone(&ep1)), &client, &cache);

        let initial = cache.current().unwrap();
        assert_eq!(initial.clusters.len(), 1);

        // Route config now also references c2: old snapshot remains active until
        // c2 completes both CDS and EDS.
        state.handle_rds(
            ok_event(make_route_config("rc-v2", &["c1", "c2"])),
            &client,
            &cache,
        );
        assert!(Arc::ptr_eq(&cache.current().unwrap(), &initial));

        let c2 = make_cluster_resource("c2", None);
        state.handle_cds("c2", ok_event(Arc::clone(&c2)), &client, &cache);
        assert!(Arc::ptr_eq(&cache.current().unwrap(), &initial));

        let ep2 = make_endpoints_resource("c2");
        state.handle_eds("c2", ok_event(Arc::clone(&ep2)), &client, &cache);
        let updated = cache.current().unwrap();
        assert_eq!(updated.route_config.name, "rc-v2");
        assert!(Arc::ptr_eq(
            &updated.clusters["c1"].as_ref().unwrap().cluster,
            &c1
        ));
        assert!(Arc::ptr_eq(
            &updated.clusters["c2"].as_ref().unwrap().cluster,
            &c2
        ));

        // Removing c1 publishes immediately because c2 is already resolved.
        state.handle_rds(
            ok_event(make_route_config("rc-v3", &["c2"])),
            &client,
            &cache,
        );
        let pruned = cache.current().unwrap();
        assert_eq!(pruned.route_config.name, "rc-v3");
        assert!(!pruned.clusters.contains_key("c1"));
        assert!(pruned.clusters.contains_key("c2"));
    }

    #[test]
    fn eds_service_name_change_gates_until_new_eds_resolves() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        state.handle_rds(ok_event(make_route_config("rc", &["c1"])), &client, &cache);
        state.handle_cds(
            "c1",
            ok_event(make_cluster_resource("c1", Some("eds-v1"))),
            &client,
            &cache,
        );
        let ep_v1 = make_endpoints_resource("eds-v1");
        state.handle_eds("eds-v1", ok_event(Arc::clone(&ep_v1)), &client, &cache);
        let snap_v1 = cache.current().unwrap();
        assert!(Arc::ptr_eq(
            &snap_v1.clusters["c1"].as_ref().unwrap().endpoints,
            &ep_v1
        ));

        // Changing eds_service_name replaces the EDS watch and holds the previous
        // complete snapshot rather than pairing cds-v2 with stale eds-v1.
        let cds_v2 = make_cluster_resource("c1", Some("eds-v2"));
        state.handle_cds("c1", ok_event(Arc::clone(&cds_v2)), &client, &cache);
        assert!(Arc::ptr_eq(&cache.current().unwrap(), &snap_v1));
        assert!(!state.endpoints.contains_key("eds-v1"));
        assert!(state.endpoints.contains_key("eds-v2"));

        let ep_v2 = make_endpoints_resource("eds-v2");
        state.handle_eds("eds-v2", ok_event(Arc::clone(&ep_v2)), &client, &cache);
        let snap_v2 = cache.current().unwrap();
        assert!(Arc::ptr_eq(
            &snap_v2.clusters["c1"].as_ref().unwrap().cluster,
            &cds_v2
        ));
        assert!(Arc::ptr_eq(
            &snap_v2.clusters["c1"].as_ref().unwrap().endpoints,
            &ep_v2
        ));
    }

    #[test]
    fn shared_eds_service_name_deduplicates_watch_and_updates_atomically() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state(&client);

        state.handle_lds(
            ok_event(make_listener_inline(&["cluster-a", "cluster-b"])),
            &client,
            &cache,
        );
        let cds_a = make_cluster_resource("cluster-a", Some("shared-eds"));
        let cds_b = make_cluster_resource("cluster-b", Some("shared-eds"));
        state.handle_cds("cluster-a", ok_event(Arc::clone(&cds_a)), &client, &cache);
        state.handle_cds("cluster-b", ok_event(Arc::clone(&cds_b)), &client, &cache);

        // Both clusters point to `shared-eds` -> exactly one EDS subscription.
        assert_eq!(state.endpoints.len(), 1);
        assert!(state.endpoints.contains_key("shared-eds"));
        assert!(cache.current_result().is_none());

        // A single EDS update resolves both clusters in the same snapshot.
        let ep_v1 = make_endpoints_resource("shared-eds");
        state.handle_eds("shared-eds", ok_event(Arc::clone(&ep_v1)), &client, &cache);
        let snap = cache.current().expect("both clusters resolved");
        assert!(Arc::ptr_eq(
            &snap.clusters["cluster-a"].as_ref().unwrap().endpoints,
            &ep_v1
        ));
        assert!(Arc::ptr_eq(
            &snap.clusters["cluster-b"].as_ref().unwrap().endpoints,
            &ep_v1
        ));

        // Removing `cluster-a` retains `shared-eds` because `cluster-b` still uses it.
        state.handle_lds(
            ok_event(make_listener_inline(&["cluster-b"])),
            &client,
            &cache,
        );
        assert_eq!(state.clusters.len(), 1);
        assert_eq!(state.endpoints.len(), 1);
        assert!(state.endpoints.contains_key("shared-eds"));
        let pruned = cache.current().unwrap();
        assert!(!pruned.clusters.contains_key("cluster-a"));
        assert!(Arc::ptr_eq(
            &pruned.clusters["cluster-b"].as_ref().unwrap().endpoints,
            &ep_v1
        ));
    }

    #[test]
    fn virtual_host_scoping_ignores_unmatched_vhost_clusters() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state_with_authority(&client, "svc-a.default.svc.cluster.local");

        state.handle_lds(ok_event(make_listener_rds("shared-rc")), &client, &cache);
        let rc = Arc::new(RouteConfigResource {
            name: "shared-rc".into(),
            virtual_hosts: vec![
                make_vhost("vh-a", &["svc-a.default.svc.cluster.local"], &["cluster-a"]),
                make_vhost(
                    "vh-b",
                    &["svc-b.default.svc.cluster.local"],
                    &["unmatched-broken-cluster"],
                ),
            ],
            metadata: Default::default(),
        });
        state.handle_rds(ok_event(rc), &client, &cache);

        // Only `cluster-a` from the matched virtual host is watched; the
        // unmatched virtual host's cluster is neither subscribed nor gated on.
        assert_eq!(state.clusters.len(), 1);
        assert!(state.clusters.contains_key("cluster-a"));
        assert!(!state.clusters.contains_key("unmatched-broken-cluster"));

        resolve_cluster(&mut state, "cluster-a", &client, &cache);
        let snap = cache
            .current()
            .expect("matched vhost cluster resolved without waiting on unmatched vhost");
        assert_eq!(snap.clusters.len(), 1);
        assert!(snap.clusters["cluster-a"].is_ok());
    }

    #[test]
    fn no_matching_virtual_host_publishes_top_level_error_and_clears_clusters() {
        let cache = test_cache();
        let client = test_client();
        let mut state = new_state_with_authority(&client, "svc-a.example.com");

        state.handle_lds(ok_event(make_listener_rds("rc")), &client, &cache);
        let matching_rc = Arc::new(RouteConfigResource {
            name: "rc".into(),
            virtual_hosts: vec![make_vhost("vh-a", &["svc-a.example.com"], &["c1"])],
            metadata: Default::default(),
        });
        state.handle_rds(ok_event(matching_rc), &client, &cache);
        resolve_cluster(&mut state, "c1", &client, &cache);
        assert!(cache.current().is_some());
        assert_eq!(state.clusters.len(), 1);
        assert_eq!(state.endpoints.len(), 1);

        // RDS update drops `svc-a.example.com` without a wildcard fallback ->
        // publishes top-level XdsConfigError and cancels child CDS/EDS watches.
        let non_matching_rc = Arc::new(RouteConfigResource {
            name: "rc".into(),
            virtual_hosts: vec![make_vhost("vh-other", &["other.example.com"], &["c2"])],
            metadata: Default::default(),
        });
        state.handle_rds(ok_event(non_matching_rc), &client, &cache);

        let err = cache.current_result().unwrap().unwrap_err();
        assert!(
            err.0
                .contains("no matching virtual host for authority 'svc-a.example.com'"),
            "unexpected error: {err}"
        );
        assert!(state.clusters.is_empty());
        assert!(state.endpoints.is_empty());
    }

    #[tokio::test]
    async fn poll_batch_coalesces_multi_cluster_sotw_response() {
        use xds_client::{
            ClientConfig, Node, ProstCodec, TokioRuntime, TonicTransportBuilder,
            XdsClient as RealXdsClient,
        };
        use xds_test_util::{XdsTestControlPlaneService, config};

        let control_plane = XdsTestControlPlaneService::new()
            .start()
            .await
            .expect("control plane failed to start");
        let svc = control_plane.get_service();
        let clusters = ["c1".to_string(), "c2".to_string(), "c3".to_string()];
        svc.set_xds_config(
            &config::AdsTypeUrl::Lds,
            HashMap::from([(
                "listener".into(),
                config::build_rds_listener("listener", "rc"),
            )]),
        );
        svc.set_xds_config(
            &config::AdsTypeUrl::Rds,
            HashMap::from([(
                "rc".into(),
                config::build_route_config_with_clusters("rc", &clusters),
            )]),
        );
        svc.set_xds_config(
            &config::AdsTypeUrl::Cds,
            HashMap::from([
                ("c1".into(), config::build_cluster("c1")),
                ("c2".into(), config::build_cluster("c2")),
                ("c3".into(), config::build_cluster("c3")),
            ]),
        );
        svc.set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([
                ("c1".into(), config::build_cla("c1", &[])),
                ("c2".into(), config::build_cla("c2", &[])),
                ("c3".into(), config::build_cla("c3", &[])),
            ]),
        );

        let xds_client: RealXdsClient = RealXdsClient::builder(
            ClientConfig::new(
                Node::new("test", "0"),
                format!("http://{}", control_plane.addr()),
            ),
            TonicTransportBuilder::new(),
            ProstCodec,
            TokioRuntime,
        )
        .build();

        let cache = test_cache();
        let mut state = new_state(&xds_client);

        let mut dones = Vec::new();
        while cache.current_result().is_none() {
            assert!(std::future::poll_fn(|cx| state.poll_batch(cx, &mut dones)).await);
            state.resolve(&xds_client, &cache);
            dones.clear();
        }

        // Register a canary watcher after `"c3"` is already subscribed and
        // drain its immediate cached replay so its next event reflects the new
        // SotW broadcast below.
        let mut eds_canary = xds_client.watch::<EndpointsResource>("c3");
        tokio::time::timeout(Duration::from_secs(10), eds_canary.next())
            .await
            .expect("timed out draining cached EDS")
            .expect("canary closed");

        // Push one SotW EDS update touching all three already-subscribed
        // clusters at once; `xds-client` dispatches all three watcher events
        // synchronously from the single DiscoveryResponse.
        let ep = [("127.0.0.1".to_string(), 8080)];
        svc.set_xds_config(
            &config::AdsTypeUrl::Eds,
            HashMap::from([
                ("c1".into(), config::build_cla("c1", &ep)),
                ("c2".into(), config::build_cla("c2", &ep)),
                ("c3".into(), config::build_cla("c3", &ep)),
            ]),
        );
        tokio::time::timeout(Duration::from_secs(10), eds_canary.next())
            .await
            .expect("timed out waiting for SotW EDS broadcast")
            .expect("canary closed");

        let mut watch = cache.watch();
        let _initial = watch.next().await.unwrap().unwrap();
        assert!(std::future::poll_fn(|cx| state.poll_batch(cx, &mut dones)).await);
        assert_eq!(
            dones.len(),
            3,
            "all three SotW EDS events drained in one batch"
        );
        state.resolve(&xds_client, &cache);
        dones.clear();

        let snap = watch.next().await.unwrap().unwrap();
        assert_eq!(snap.clusters.len(), 3);
        assert!(snap.clusters.values().all(|res| {
            res.as_ref().unwrap().endpoints.localities[0]
                .endpoints
                .len()
                == 1
        }));
    }

    #[tokio::test]
    async fn poll_batch_terminates_when_child_watcher_closes() {
        use xds_client::{
            ClientConfig, Node, ProstCodec, TokioRuntime, TonicTransportBuilder,
            XdsClient as RealXdsClient,
        };

        // Keep `lds` backed by a live client (`Poll::Pending`) while a child
        // watcher (`rds`) comes from a disconnected client (`Poll::Ready(None)`).
        let live_client: RealXdsClient = RealXdsClient::builder(
            ClientConfig::new(Node::new("test", "0"), "http://127.0.0.1:1"),
            TonicTransportBuilder::new(),
            ProstCodec,
            TokioRuntime,
        )
        .build();
        let mut state = new_state(&live_client);
        state.rds = Some(WatchedRds {
            name: "rc".into(),
            watch: Watch::new(&test_client(), "rc"),
        });

        let mut dones = Vec::new();
        let keep_running = tokio::time::timeout(
            Duration::from_secs(1),
            std::future::poll_fn(|cx| state.poll_batch(cx, &mut dones)),
        )
        .await
        .expect("poll_batch should not hang when a child watcher closes");
        assert!(!keep_running);
    }
}
