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

//! xDS routing: route matching logic and [`XdsRouter`] implementation.
//!
//! This module contains both the route matching logic (domain → path → headers)
//! and the stateful [`XdsRouter`] that subscribes to cache updates and serves
//! routing decisions.
//!
//! Domain matching follows gRFC A27 priority:
//! 1. Exact match
//! 2. Suffix wildcard (`*.foo.com`)
//! 3. Prefix wildcard (`foo.*`)
//! 4. Universal wildcard `*`
//!
//! Within each category, the most specific (longest non-wildcard part) wins.

use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
use tokio::sync::watch;

use crate::client::cluster::ClusterRef;
use crate::client::retry::RetryClassifierFactory;
use crate::client::route::{AcquiredConfig, RouteDecision, RouteInput, Router, RoutingSnapshot};
use crate::common::async_util::AbortOnDrop;
use crate::xds::cache::{
    CacheSender, CacheWatch, ClusterError, ClusterSnapshot, ClusterWatches, XdsCache,
};
use crate::xds::resource::hash_policy::HashPolicyConfig;
use crate::xds::resource::route_config::{
    HeaderMatchSpecifierConfig, HeaderMatcherConfig, PathSpecifierConfig, RouteConfig,
    RouteConfigAction, RouteConfigMatch, RouteConfigResource, VirtualHostConfig, WeightedCluster,
};
use crate::xds::resource::string_matcher::{
    ends_with_ignore_ascii_case, starts_with_ignore_ascii_case,
};
use crate::xds::resource::{ClusterResource, EndpointsResource};

/// Default timeout for waiting for the initial route config on cold start.
pub(crate) const DEFAULT_READY_TIMEOUT: Duration = Duration::from_secs(30);

/// Resolved top-level router state published by [`XdsRouter`]'s watch task.
enum RouterState {
    Ready(Arc<RoutingSnapshot>),
    Failed(tonic::Status),
}

/// xDS-backed [`Router`] that resolves requests to clusters.
///
/// Subscribes to [`XdsConfig`](crate::xds::cache::XdsConfig) snapshots from
/// [`XdsCache`] via a background watch task and maintains a shared
/// [`ArcSwapOption`] for lock-free reads on the hot path. The watch task is
/// aborted when the router is dropped.
///
/// The watch task also owns a client and CDS/EDS watch senders for each healthy
/// cluster the route config references (see [`ClusterSet`]). Each
/// [`RouteDecision`] carries the client of its cluster, while routes targeting
/// a cluster with a [`ClusterError`] or arriving while top-level LDS/RDS has
/// failed fail immediately with `UNAVAILABLE`.
///
/// The first RPC blocks (up to `ready_timeout`, defaulting to
/// [`DEFAULT_READY_TIMEOUT`], or without an internal timeout when `None`) until
/// the initial config or top-level error is published, matching standard gRPC
/// behavior where RPCs wait for the resolver's first update. Subsequent RPCs
/// read the state lock-free.
pub(crate) struct XdsRouter {
    state: Arc<ArcSwapOption<RouterState>>,
    ready_rx: watch::Receiver<bool>,
    ready_timeout: Option<Duration>,
    _watch_task: AbortOnDrop,
}

impl XdsRouter {
    /// Creates a router that watches [`XdsConfig`](crate::xds::cache::XdsConfig)
    /// snapshots from `cache`, calling `provision` with router-owned
    /// [`ClusterWatches`] to build a client for each healthy cluster the route
    /// config references. `retry_factory` compiles per-route retry policies,
    /// letting a non-gRPC transport interpret `retry_on` for that transport.
    ///
    /// Spawns a background task that reconciles cluster feeds on every snapshot
    /// and rebuilds the local [`RoutingSnapshot`] whenever the route config or
    /// cluster outcomes change. The task is aborted when this router is dropped.
    pub(crate) fn new<F>(
        cache: &XdsCache,
        retry_factory: Arc<dyn RetryClassifierFactory>,
        ready_timeout: Option<Duration>,
        provision: F,
    ) -> Self
    where
        F: FnMut(&str, ClusterWatches) -> ClusterRef + Send + 'static,
    {
        let state = Arc::new(ArcSwapOption::empty());
        let (ready_tx, ready_rx) = watch::channel(false);
        let state_slot = state.clone();
        let mut watcher = cache.watch();
        let mut clusters = ClusterSet::new(provision);
        let handle = tokio::spawn(async move {
            let mut ready_tx = Some(ready_tx);
            let mut last_snapshot: Option<Arc<RoutingSnapshot>> = None;
            while let Some(result) = watcher.next().await {
                match result {
                    Ok(config) => {
                        let clusters_changed = clusters.reconcile(&config.clusters);
                        let reuse = !clusters_changed
                            && last_snapshot
                                .as_ref()
                                .is_some_and(|prev| prev.matches_config(&config));
                        if !reuse {
                            let snapshot = Arc::new(RoutingSnapshot::new(
                                Arc::clone(&config.route_config),
                                config.virtual_host_index,
                                retry_factory.as_ref(),
                                clusters.outcomes(&config.clusters),
                            ));
                            state_slot
                                .store(Some(Arc::new(RouterState::Ready(Arc::clone(&snapshot)))));
                            last_snapshot = Some(snapshot);
                        }
                    }
                    Err(err) => {
                        clusters.clear();
                        last_snapshot = None;
                        let status = tonic::Status::unavailable(format!(
                            "xDS configuration unavailable: {err}"
                        ));
                        state_slot.store(Some(Arc::new(RouterState::Failed(status))));
                    }
                }
                // Signal readiness on the first resolution, then drop the sender.
                if let Some(tx) = ready_tx.take() {
                    let _ = tx.send(true);
                }
            }
        });
        Self {
            state,
            ready_rx,
            ready_timeout,
            _watch_task: AbortOnDrop(handle),
        }
    }

    /// The route config currently in effect, or `None` if none has arrived yet
    /// or top-level configuration is currently in an error state.
    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> Option<Arc<RoutingSnapshot>> {
        match self.state.load().as_deref() {
            Some(RouterState::Ready(snapshot)) => Some(Arc::clone(snapshot)),
            Some(RouterState::Failed(_)) | None => None,
        }
    }
}

/// The clients and per-cluster CDS/EDS senders of the healthy clusters the
/// active route config references: those whose snapshot has endpoints.
///
/// The router's watch task reconciles this set against each
/// [`XdsConfig`](crate::xds::cache::XdsConfig) snapshot. Because snapshots are
/// completeness-gated, every provisioned cluster starts with both its initial
/// `ClusterResource` and `EndpointsResource` seeded on [`ClusterWatches`].
/// When a cluster leaves the route config, transitions to [`ClusterError`], or
/// has no endpoints because its EDS resource failed, the set drops both its
/// client and senders — closing downstream CDS/EDS watch streams
/// deterministically without touching [`XdsCache`]. Routes to a cluster without
/// a client still resolve, and the LB stage fails them with its error (gRFC
/// A74), which for a cluster without endpoints is its `resolution_note` (gRFC
/// A88). Requests already routed to a dropped client keep it alive until they
/// complete.
struct ClusterSet<F> {
    provision: F,
    clusters: HashMap<String, OwnedCluster>,
}

struct OwnedCluster {
    cluster: ClusterRef,
    cluster_tx: CacheSender<ClusterResource>,
    endpoints_tx: CacheSender<EndpointsResource>,
}

impl<F> ClusterSet<F>
where
    F: FnMut(&str, ClusterWatches) -> ClusterRef,
{
    fn new(provision: F) -> Self {
        Self {
            provision,
            clusters: HashMap::new(),
        }
    }

    /// Brings the set in line with `clusters`, forwarding CDS/EDS updates to
    /// retained healthy clusters and provisioning newly healthy ones.
    ///
    /// Returns `true` if any active cluster entry was added or removed.
    fn reconcile(
        &mut self,
        clusters: &HashMap<String, Result<ClusterSnapshot, ClusterError>>,
    ) -> bool {
        let mut changed = false;
        self.clusters.retain(|name, _| {
            let keep = matches!(
                clusters.get(name),
                Some(Ok(ClusterSnapshot {
                    endpoints: Some(_),
                    ..
                }))
            );
            if !keep {
                changed = true;
            }
            keep
        });
        for (name, res) in clusters {
            let Ok(ClusterSnapshot {
                cluster,
                endpoints: Some(endpoints),
                ..
            }) = res
            else {
                continue;
            };
            if let Some(existing) = self.clusters.get(name) {
                existing.cluster_tx.send(Arc::clone(cluster));
                existing.endpoints_tx.send(Arc::clone(endpoints));
                continue;
            }
            changed = true;
            let (cluster_tx, cluster_watch) = CacheWatch::channel(Some(Arc::clone(cluster)));
            let (endpoints_tx, endpoints_watch) = CacheWatch::channel(Some(Arc::clone(endpoints)));
            let watches = ClusterWatches {
                cluster: cluster_watch,
                endpoints: endpoints_watch,
            };
            let cluster = (self.provision)(name, watches);
            self.clusters.insert(
                name.clone(),
                OwnedCluster {
                    cluster,
                    cluster_tx,
                    endpoints_tx,
                },
            );
        }
        changed
    }

    fn outcomes(
        &self,
        clusters: &HashMap<String, Result<ClusterSnapshot, ClusterError>>,
    ) -> HashMap<String, Result<ClusterRef, ClusterError>> {
        clusters
            .iter()
            .map(|(name, res)| {
                let outcome = match res {
                    Ok(snap) => match self.clusters.get(name) {
                        Some(owned) => Ok(owned.cluster.clone()),
                        None => Err(ClusterError(snap.resolution_note.clone().unwrap_or_else(
                            || format!("cluster '{name}' has no endpoints").into(),
                        ))),
                    },
                    Err(err) => Err(err.clone()),
                };
                (name.clone(), outcome)
            })
            .collect()
    }

    fn clear(&mut self) {
        self.clusters.clear();
    }
}

impl Router for XdsRouter {
    fn acquire(&self) -> AcquiredConfig {
        match self.state.load().as_deref() {
            Some(RouterState::Ready(config)) => return AcquiredConfig::Ready(Arc::clone(config)),
            Some(RouterState::Failed(status)) => {
                return AcquiredConfig::Failed(RoutingError::Unavailable(status.clone()));
            }
            None => {}
        }
        // Wait for the initial resolution, matching standard gRPC behavior
        // where RPCs block until the resolver provides the first update.
        let state_ref = self.state.clone();
        let mut ready_rx = self.ready_rx.clone();
        let ready_timeout = self.ready_timeout;
        AcquiredConfig::Pending(Box::pin(async move {
            match ready_timeout {
                Some(timeout) => {
                    tokio::time::timeout(timeout, ready_rx.wait_for(|ready| *ready))
                        .await
                        .map_err(|_| RoutingError::NotReady)?
                        .map_err(|_| RoutingError::NotReady)?;
                }
                None => {
                    ready_rx
                        .wait_for(|ready| *ready)
                        .await
                        .map_err(|_| RoutingError::NotReady)?;
                }
            }
            match state_ref.load().as_deref() {
                Some(RouterState::Ready(config)) => Ok(Arc::clone(config)),
                Some(RouterState::Failed(status)) => Err(RoutingError::Unavailable(status.clone())),
                None => Err(RoutingError::NotReady),
            }
        }))
    }

    fn route(
        &self,
        input: &RouteInput<'_>,
        config: &RoutingSnapshot,
    ) -> Result<RouteDecision, RoutingError> {
        resolve_route(config, input.authority, input.path, input.headers)
    }
}

/// Resolve a route decision from the given snapshot, authority, path, and headers.
fn resolve_route(
    rc: &RoutingSnapshot,
    authority: &str,
    path: &str,
    headers: &http::HeaderMap,
) -> Result<RouteDecision, RoutingError> {
    let vh = rc
        .virtual_host()
        .ok_or_else(|| RoutingError::NoMatchingVirtualHost(authority.to_string()))?;
    let (route_idx, route) = vh
        .routes
        .iter()
        .enumerate()
        .find(|(_, route)| route_matches(route, path, headers))
        .ok_or_else(|| RoutingError::NoMatchingRoute(path.to_string()))?;
    let cluster = match &route.action {
        RouteConfigAction::Cluster(name) => name.clone(),
        RouteConfigAction::WeightedClusters(clusters) => select_weighted_cluster(clusters)
            .ok_or(RoutingError::EmptyWeightedClusters)?
            .to_string(),
    };

    // A failed cluster still routes; the LB stage fails the attempt (gRFC A74).
    let cluster_ref = rc
        .cluster_outcome(&cluster)
        .map(|outcome| outcome.cloned().map_err(Clone::clone));

    // Retry config for the matched route, pre-compiled at `route_idx` in the
    // same snapshot we routed on so retry and routing act on one RDS version.
    let retry_config = rc.retry_policy(route_idx);

    // gRFC A31: the route's max stream duration, which already includes the
    // listener's default. Zero means no limit.
    let max_stream_duration = route.max_stream_duration.filter(|d| !d.is_zero());

    // gRFC A42 ring-hash request hash. The policy list is empty for now, so
    // `request_hash` resolves to `None` and the ring-hash picker falls back to a
    // random hash.
    // TODO(madhurishgupta): populate hash policies from the route's RDS
    // `RouteAction.hash_policy` (later PR).
    let policies: &[HashPolicyConfig] = &[];
    let request_hash = HashPolicyConfig::request_hash(headers, policies);

    Ok(RouteDecision {
        cluster_ref,
        cluster,
        request_hash,
        retry_config,
        max_stream_duration,
    })
}

/// Error returned when routing fails.
#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum RoutingError {
    #[error("route config not yet available")]
    NotReady,
    #[error("no matching virtual host for authority '{0}'")]
    NoMatchingVirtualHost(String),
    #[error("no matching route in virtual host for path '{0}'")]
    NoMatchingRoute(String),
    #[error("weighted cluster selection failed (empty cluster list)")]
    EmptyWeightedClusters,
    /// Top-level (LDS/RDS/VirtualHost) xDS configuration failed, surfaced as
    /// `UNAVAILABLE`.
    #[error("{}", .0.message())]
    Unavailable(#[source] tonic::Status),
}

impl RouteConfigResource {
    /// Match a request and return the target cluster action.
    ///
    /// Performs domain matching on the authority, then walks routes in order
    /// to find the first match. Production routing uses [`RoutingSnapshot`]'s
    /// pre-selected virtual host instead.
    #[cfg(test)]
    pub(crate) fn route(
        &self,
        authority: &str,
        path: &str,
        headers: &http::HeaderMap,
    ) -> Result<&RouteConfigAction, RoutingError> {
        let idx = find_best_matching_virtual_host_index(authority, &self.virtual_hosts)
            .ok_or_else(|| RoutingError::NoMatchingVirtualHost(authority.to_string()))?;

        for route in &self.virtual_hosts[idx].routes {
            if route_matches(route, path, headers) {
                return Ok(&route.action);
            }
        }

        Err(RoutingError::NoMatchingRoute(path.to_string()))
    }
}

const WILDCARD: &str = "*";

/// Finds the index of the best-matching virtual host for the given authority.
pub(crate) fn find_best_matching_virtual_host_index(
    authority: &str,
    virtual_hosts: &[VirtualHostConfig],
) -> Option<usize> {
    virtual_hosts
        .iter()
        .enumerate()
        .filter_map(|(idx, vh)| {
            let best_score = vh
                .domains
                .iter()
                .filter_map(|d| match_domain(authority, d))
                .min()?;
            Some((best_score, idx))
        })
        .min_by_key(|(score, _)| *score)
        .map(|(_, idx)| idx)
}

/// How well a domain pattern matched an authority.
///
/// Sorts naturally so that better matches are smaller:
/// match type (Exact < Suffix < Prefix < Universal), then higher
/// specificity (more non-wildcard characters) breaks ties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DomainMatchScore(DomainMatchType, Reverse<usize>);

/// Domain match types ordered by priority (lower is better).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DomainMatchType {
    Exact = 0,
    Suffix = 1,
    Prefix = 2,
    Universal = 3,
}

fn match_domain(authority: &str, pattern: &str) -> Option<DomainMatchScore> {
    if pattern == WILDCARD {
        return Some(DomainMatchScore(DomainMatchType::Universal, Reverse(0)));
    }

    if authority.eq_ignore_ascii_case(pattern) {
        return Some(DomainMatchScore(
            DomainMatchType::Exact,
            Reverse(pattern.len()),
        ));
    }

    // The wildcard must absorb at least one authority byte, so the
    // authority has to be strictly longer than the pattern tail.
    if let Some(suffix) = pattern.strip_prefix(WILDCARD)
        && authority.len() > suffix.len()
        && ends_with_ignore_ascii_case(authority, suffix)
    {
        return Some(DomainMatchScore(
            DomainMatchType::Suffix,
            Reverse(suffix.len()),
        ));
    }

    if let Some(prefix) = pattern.strip_suffix(WILDCARD)
        && authority.len() > prefix.len()
        && starts_with_ignore_ascii_case(authority, prefix)
    {
        return Some(DomainMatchScore(
            DomainMatchType::Prefix,
            Reverse(prefix.len()),
        ));
    }

    None
}

fn route_matches(route: &RouteConfig, path: &str, headers: &http::HeaderMap) -> bool {
    match_path(&route.match_criteria, path)
        && match_headers(&route.match_criteria, headers)
        && match_fraction(route.match_criteria.match_fraction)
}

/// Probabilistic route gating per A28. Returns true if the route should be considered.
fn match_fraction(fraction: Option<u32>) -> bool {
    match fraction {
        None | Some(1_000_000..) => true,
        Some(0) => false,
        Some(n) => fastrand::u32(0..1_000_000) < n,
    }
}

/// Select a cluster from weighted clusters using accumulated weights and random selection.
///
/// Returns `None` if `clusters` is empty.
pub(crate) fn select_weighted_cluster(clusters: &[WeightedCluster]) -> Option<&str> {
    if clusters.is_empty() {
        return None;
    }

    let total: u64 = clusters.iter().map(|c| c.weight as u64).sum();
    if total == 0 {
        return Some(&clusters[fastrand::usize(0..clusters.len())].name);
    }

    let random = fastrand::u64(0..total);
    let mut acc = 0u64;
    for cluster in clusters {
        acc += cluster.weight as u64;
        if random < acc {
            return Some(&cluster.name);
        }
    }
    // random is in [0, total) and acc reaches total, so the loop always returns.
    unreachable!()
}

fn match_path(criteria: &RouteConfigMatch, path: &str) -> bool {
    match &criteria.path_specifier {
        PathSpecifierConfig::Prefix(prefix) => {
            if prefix.is_empty() {
                return true;
            }
            if criteria.case_sensitive {
                path.starts_with(prefix.as_str())
            } else {
                path.to_ascii_lowercase()
                    .starts_with(&prefix.to_ascii_lowercase())
            }
        }
        PathSpecifierConfig::Path(exact) => {
            if criteria.case_sensitive {
                path == exact
            } else {
                path.eq_ignore_ascii_case(exact)
            }
        }
        PathSpecifierConfig::SafeRegex(re) => re.is_match(path),
    }
}

fn match_headers(criteria: &RouteConfigMatch, headers: &http::HeaderMap) -> bool {
    criteria.headers.iter().all(|m| {
        let result = match_header(m, headers);
        if m.invert_match { !result } else { result }
    })
}

const DEFAULT_CONTENT_TYPE: &str = "application/grpc";

fn match_header(hm: &HeaderMatcherConfig, headers: &http::HeaderMap) -> bool {
    // Per A28: if content-type is not present, assume "application/grpc".
    let value = headers
        .get(&hm.name)
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            if hm.name.eq_ignore_ascii_case("content-type") {
                Some(DEFAULT_CONTENT_TYPE)
            } else {
                None
            }
        });

    match &hm.match_specifier {
        HeaderMatchSpecifierConfig::Present => value.is_some(),
        HeaderMatchSpecifierConfig::Absent => value.is_none(),
        HeaderMatchSpecifierConfig::String(sm) => value.is_some_and(|v| sm.is_match(v)),
        HeaderMatchSpecifierConfig::Range { start, end } => {
            value.is_some_and(|v| v.parse::<i64>().is_ok_and(|n| n >= *start && n < *end))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::retry::GrpcRetryClassifierFactory;
    use crate::xds::cache::{XdsCache, XdsConfig, XdsConfigError};
    use crate::xds::resource::route_config::{
        RouteConfig, RouteConfigAction, RouteConfigMatch, VirtualHostConfig,
    };
    use crate::xds::resource::safe_regex::SafeRegex;
    use crate::xds::resource::string_matcher::StringMatcher;

    fn simple_route(prefix: &str, cluster: &str) -> RouteConfig {
        RouteConfig {
            match_criteria: RouteConfigMatch {
                path_specifier: PathSpecifierConfig::Prefix(prefix.into()),
                headers: vec![],
                case_sensitive: true,
                match_fraction: None,
            },
            action: RouteConfigAction::Cluster(cluster.into()),
            retry_config: None,
            max_stream_duration: None,
        }
    }

    fn simple_rc(virtual_hosts: Vec<VirtualHostConfig>) -> RouteConfigResource {
        RouteConfigResource {
            name: "test-rc".into(),
            virtual_hosts,
            metadata: Default::default(),
        }
    }

    #[test]
    fn domain_exact() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["foo.com".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        assert!(rc.route("foo.com", "/", &http::HeaderMap::new()).is_ok());
    }

    #[test]
    fn domain_case_insensitive() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["FOO.COM".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        assert!(rc.route("foo.com", "/", &http::HeaderMap::new()).is_ok());
    }

    #[test]
    fn domain_suffix_wildcard() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*.foo.com".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        let h = http::HeaderMap::new();
        assert!(rc.route("bar.foo.com", "/", &h).is_ok());
        assert!(rc.route("foo.com", "/", &h).is_err());
    }

    #[test]
    fn domain_prefix_wildcard() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["foo.*".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        let h = http::HeaderMap::new();
        assert!(rc.route("foo.bar", "/", &h).is_ok());
        assert!(rc.route("bar.foo", "/", &h).is_err());
    }

    #[test]
    fn domain_suffix_wildcard_case_insensitive() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*.FOO.com".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        let h = http::HeaderMap::new();
        assert!(rc.route("bar.foo.com", "/", &h).is_ok());
        assert!(rc.route("BAR.foo.COM", "/", &h).is_ok());
        assert!(rc.route("foo.com", "/", &h).is_err());
    }

    #[test]
    fn domain_prefix_wildcard_case_insensitive() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["FOO.*".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        let h = http::HeaderMap::new();
        assert!(rc.route("foo.bar", "/", &h).is_ok());
        assert!(rc.route("FOO.bar", "/", &h).is_ok());
        assert!(rc.route("FOO.", "/", &h).is_err());
    }

    #[test]
    fn domain_universal() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        assert!(
            rc.route("anything.com", "/", &http::HeaderMap::new())
                .is_ok()
        );
    }

    #[test]
    fn domain_exact_beats_suffix() {
        let rc = simple_rc(vec![
            VirtualHostConfig {
                name: "vh-suffix".into(),
                domains: vec!["*.foo.com".into()],
                routes: vec![simple_route("/", "cluster-suffix")],
            },
            VirtualHostConfig {
                name: "vh-exact".into(),
                domains: vec!["bar.foo.com".into()],
                routes: vec![simple_route("/", "cluster-exact")],
            },
        ]);
        let action = rc
            .route("bar.foo.com", "/", &http::HeaderMap::new())
            .unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-exact"));
    }

    #[test]
    fn domain_suffix_beats_universal() {
        let rc = simple_rc(vec![
            VirtualHostConfig {
                name: "vh-universal".into(),
                domains: vec!["*".into()],
                routes: vec![simple_route("/", "cluster-universal")],
            },
            VirtualHostConfig {
                name: "vh-suffix".into(),
                domains: vec!["*.foo.com".into()],
                routes: vec![simple_route("/", "cluster-suffix")],
            },
        ]);
        let action = rc
            .route("bar.foo.com", "/", &http::HeaderMap::new())
            .unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-suffix"));
    }

    #[test]
    fn domain_longer_suffix_wins() {
        let rc = simple_rc(vec![
            VirtualHostConfig {
                name: "vh-short".into(),
                domains: vec!["*.com".into()],
                routes: vec![simple_route("/", "cluster-short")],
            },
            VirtualHostConfig {
                name: "vh-long".into(),
                domains: vec!["*.foo.com".into()],
                routes: vec![simple_route("/", "cluster-long")],
            },
        ]);
        let action = rc
            .route("bar.foo.com", "/", &http::HeaderMap::new())
            .unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-long"));
    }

    #[test]
    fn domain_no_match() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["foo.com".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        assert!(rc.route("bar.com", "/", &http::HeaderMap::new()).is_err());
    }

    #[test]
    fn basic_routing() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![simple_route("/", "cluster-1")],
        }]);
        let headers = http::HeaderMap::new();

        let action = rc.route("any.host", "/foo", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-1"));
    }

    #[test]
    fn domain_selects_virtual_host() {
        let rc = simple_rc(vec![
            VirtualHostConfig {
                name: "vh-foo".into(),
                domains: vec!["foo.com".into()],
                routes: vec![simple_route("/", "cluster-foo")],
            },
            VirtualHostConfig {
                name: "vh-bar".into(),
                domains: vec!["bar.com".into()],
                routes: vec![simple_route("/", "cluster-bar")],
            },
        ]);
        let headers = http::HeaderMap::new();

        let action = rc.route("foo.com", "/x", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-foo"));

        let action = rc.route("bar.com", "/x", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-bar"));
    }

    #[test]
    fn no_matching_virtual_host() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["foo.com".into()],
            routes: vec![simple_route("/", "c1")],
        }]);
        let headers = http::HeaderMap::new();

        let err = rc.route("unknown.com", "/", &headers).unwrap_err();
        assert!(matches!(err, RoutingError::NoMatchingVirtualHost(_)));
    }

    #[test]
    fn first_matching_route_wins() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![
                simple_route("/svc/", "cluster-svc"),
                simple_route("/", "cluster-default"),
            ],
        }]);
        let headers = http::HeaderMap::new();

        let action = rc.route("host", "/svc/Method", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-svc"));

        let action = rc.route("host", "/other", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-default"));
    }

    #[test]
    fn no_matching_route() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![simple_route("/svc/", "c1")],
        }]);
        let headers = http::HeaderMap::new();

        let err = rc.route("host", "/other", &headers).unwrap_err();
        assert!(matches!(err, RoutingError::NoMatchingRoute(_)));
    }

    #[test]
    fn exact_path_match() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::Path("/svc/Method".into()),
                    headers: vec![],
                    case_sensitive: true,
                    match_fraction: None,
                },
                action: RouteConfigAction::Cluster("c1".into()),
                retry_config: None,
                max_stream_duration: None,
            }],
        }]);
        let headers = http::HeaderMap::new();

        assert!(rc.route("host", "/svc/Method", &headers).is_ok());
        assert!(rc.route("host", "/svc/Other", &headers).is_err());
    }

    #[test]
    fn regex_path_match() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::SafeRegex(
                        SafeRegex::new("/svc/.*").unwrap(),
                    ),
                    headers: vec![],
                    case_sensitive: true,
                    match_fraction: None,
                },
                action: RouteConfigAction::Cluster("c1".into()),
                retry_config: None,
                max_stream_duration: None,
            }],
        }]);
        let headers = http::HeaderMap::new();

        assert!(rc.route("host", "/svc/Anything", &headers).is_ok());
        assert!(rc.route("host", "/other", &headers).is_err());
    }

    #[test]
    fn header_matcher_filters_routes() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![HeaderMatcherConfig {
                            name: "x-env".into(),
                            match_specifier: HeaderMatchSpecifierConfig::String(
                                StringMatcher::Exact {
                                    value: "prod".into(),
                                    ignore_case: false,
                                },
                            ),
                            invert_match: false,
                        }],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("cluster-prod".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "cluster-default"),
            ],
        }]);

        let mut prod_headers = http::HeaderMap::new();
        prod_headers.insert("x-env", "prod".parse().unwrap());
        let action = rc.route("host", "/", &prod_headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-prod"));

        let action = rc.route("host", "/", &http::HeaderMap::new()).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "cluster-default"));
    }

    #[test]
    fn weighted_clusters_passed_through() {
        use crate::xds::resource::route_config::WeightedCluster;
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::Prefix("/".into()),
                    headers: vec![],
                    case_sensitive: true,
                    match_fraction: None,
                },
                action: RouteConfigAction::WeightedClusters(vec![
                    WeightedCluster {
                        name: "c1".into(),
                        weight: 70,
                    },
                    WeightedCluster {
                        name: "c2".into(),
                        weight: 30,
                    },
                ]),
                retry_config: None,
                max_stream_duration: None,
            }],
        }]);
        let action = rc.route("host", "/", &http::HeaderMap::new()).unwrap();
        assert!(matches!(action, RouteConfigAction::WeightedClusters(wcs) if wcs.len() == 2));
    }

    #[test]
    fn match_fraction_zero_never_matches() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![],
                        case_sensitive: true,
                        match_fraction: Some(0),
                    },
                    action: RouteConfigAction::Cluster("never".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "fallback"),
            ],
        }]);
        for _ in 0..100 {
            let action = rc.route("host", "/foo", &http::HeaderMap::new()).unwrap();
            assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "fallback"));
        }
    }

    #[test]
    fn match_fraction_million_always_matches() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::Prefix("/".into()),
                    headers: vec![],
                    case_sensitive: true,
                    match_fraction: Some(1_000_000),
                },
                action: RouteConfigAction::Cluster("always".into()),
                retry_config: None,
                max_stream_duration: None,
            }],
        }]);
        for _ in 0..100 {
            let action = rc.route("host", "/foo", &http::HeaderMap::new()).unwrap();
            assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "always"));
        }
    }

    #[test]
    fn range_match_header() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![HeaderMatcherConfig {
                            name: "x-version".into(),
                            match_specifier: HeaderMatchSpecifierConfig::Range {
                                start: 1,
                                end: 10,
                            },
                            invert_match: false,
                        }],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("versioned".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "default"),
            ],
        }]);

        let mut headers = http::HeaderMap::new();
        headers.insert("x-version", "5".parse().unwrap());
        let action = rc.route("host", "/", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "versioned"));

        // end is exclusive
        headers.insert("x-version", "10".parse().unwrap());
        let action = rc.route("host", "/", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "default"));

        // non-integer falls through
        headers.insert("x-version", "abc".parse().unwrap());
        let action = rc.route("host", "/", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "default"));

        // missing header falls through
        let action = rc.route("host", "/", &http::HeaderMap::new()).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "default"));
    }

    #[test]
    fn select_weighted_cluster_empty() {
        assert_eq!(select_weighted_cluster(&[]), None);
    }

    #[test]
    fn select_weighted_cluster_single() {
        let clusters = vec![WeightedCluster {
            name: "only".into(),
            weight: 100,
        }];
        assert_eq!(select_weighted_cluster(&clusters).unwrap(), "only");
    }

    #[test]
    fn select_weighted_cluster_zero_weights() {
        let clusters = vec![
            WeightedCluster {
                name: "a".into(),
                weight: 0,
            },
            WeightedCluster {
                name: "b".into(),
                weight: 0,
            },
        ];
        let name = select_weighted_cluster(&clusters).unwrap();
        assert!(name == "a" || name == "b");
    }

    #[test]
    fn select_weighted_cluster_weight_1_vs_0() {
        let clusters = vec![
            WeightedCluster {
                name: "winner".into(),
                weight: 1,
            },
            WeightedCluster {
                name: "loser".into(),
                weight: 0,
            },
        ];
        for _ in 0..1000 {
            assert_eq!(select_weighted_cluster(&clusters).unwrap(), "winner");
        }
    }

    #[test]
    fn content_type_defaults_to_grpc() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh1".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![HeaderMatcherConfig {
                            name: "content-type".into(),
                            match_specifier: HeaderMatchSpecifierConfig::String(
                                StringMatcher::Exact {
                                    value: "application/grpc".into(),
                                    ignore_case: false,
                                },
                            ),
                            invert_match: false,
                        }],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("grpc".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "fallback"),
            ],
        }]);
        // Per A28: content-type defaults to "application/grpc" when absent.
        let action = rc.route("host", "/", &http::HeaderMap::new()).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "grpc"));

        // Non-grpc content-type should not match the first route.
        let mut headers = http::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        let action = rc.route("host", "/", &headers).unwrap();
        assert!(matches!(action, RouteConfigAction::Cluster(c) if c == "fallback"));
    }

    #[test]
    fn ignore_case_exact_match() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh".into(),
            domains: vec!["*".into()],
            routes: vec![RouteConfig {
                match_criteria: RouteConfigMatch {
                    path_specifier: PathSpecifierConfig::Prefix("/".into()),
                    headers: vec![HeaderMatcherConfig {
                        name: "x-env".into(),
                        match_specifier: HeaderMatchSpecifierConfig::String(StringMatcher::Exact {
                            value: "Prod".into(),
                            ignore_case: true,
                        }),
                        invert_match: false,
                    }],
                    case_sensitive: true,
                    match_fraction: None,
                },
                action: RouteConfigAction::Cluster("matched".into()),
                retry_config: None,
                max_stream_duration: None,
            }],
        }]);

        let mut headers = http::HeaderMap::new();
        headers.insert("x-env", "prod".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        headers.insert("x-env", "PROD".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        headers.insert("x-env", "staging".parse().unwrap());
        assert!(rc.route("host", "/", &headers).is_err());
    }

    #[test]
    fn ignore_case_prefix_suffix_contains() {
        let make_route = |specifier: HeaderMatchSpecifierConfig| -> RouteConfigResource {
            simple_rc(vec![VirtualHostConfig {
                name: "vh".into(),
                domains: vec!["*".into()],
                routes: vec![
                    RouteConfig {
                        match_criteria: RouteConfigMatch {
                            path_specifier: PathSpecifierConfig::Prefix("/".into()),
                            headers: vec![HeaderMatcherConfig {
                                name: "x-tag".into(),
                                match_specifier: specifier,
                                invert_match: false,
                            }],
                            case_sensitive: true,
                            match_fraction: None,
                        },
                        action: RouteConfigAction::Cluster("matched".into()),
                        retry_config: None,
                        max_stream_duration: None,
                    },
                    simple_route("/", "fallback"),
                ],
            }])
        };

        let mut headers = http::HeaderMap::new();

        let rc = make_route(HeaderMatchSpecifierConfig::String(StringMatcher::Prefix {
            value: "App".into(),
            ignore_case: true,
        }));
        headers.insert("x-tag", "APPLICATION/JSON".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        let rc = make_route(HeaderMatchSpecifierConfig::String(StringMatcher::Suffix {
            value: "JSON".into(),
            ignore_case: true,
        }));
        headers.insert("x-tag", "application/json".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        let rc = make_route(HeaderMatchSpecifierConfig::String(
            StringMatcher::Contains {
                value: "Grpc".into(),
                ignore_case: true,
            },
        ));
        headers.insert("x-tag", "APPLICATION/GRPC+PROTO".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );
    }

    #[test]
    fn safe_regex_header_match() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![HeaderMatcherConfig {
                            name: "x-tag".into(),
                            match_specifier: HeaderMatchSpecifierConfig::String(
                                StringMatcher::SafeRegex(SafeRegex::new("v[0-9]+").unwrap()),
                            ),
                            invert_match: false,
                        }],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("matched".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "fallback"),
            ],
        }]);

        let mut headers = http::HeaderMap::new();

        headers.insert("x-tag", "v123".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        headers.insert("x-tag", "latest".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "fallback")
        );
    }

    #[test]
    fn ignore_case_false_is_case_sensitive() {
        let rc = simple_rc(vec![VirtualHostConfig {
            name: "vh".into(),
            domains: vec!["*".into()],
            routes: vec![
                RouteConfig {
                    match_criteria: RouteConfigMatch {
                        path_specifier: PathSpecifierConfig::Prefix("/".into()),
                        headers: vec![HeaderMatcherConfig {
                            name: "x-env".into(),
                            match_specifier: HeaderMatchSpecifierConfig::String(
                                StringMatcher::Exact {
                                    value: "Prod".into(),
                                    ignore_case: false,
                                },
                            ),
                            invert_match: false,
                        }],
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteConfigAction::Cluster("matched".into()),
                    retry_config: None,
                    max_stream_duration: None,
                },
                simple_route("/", "fallback"),
            ],
        }]);

        let mut headers = http::HeaderMap::new();
        headers.insert("x-env", "Prod".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "matched")
        );

        headers.insert("x-env", "prod".parse().unwrap());
        assert!(
            matches!(rc.route("host", "/", &headers).unwrap(), RouteConfigAction::Cluster(c) if c == "fallback")
        );
    }

    fn make_route_config(cluster: &str) -> Arc<RouteConfigResource> {
        Arc::new(simple_rc(vec![VirtualHostConfig {
            name: "vh".into(),
            domains: vec!["*".into()],
            routes: vec![simple_route("/", cluster)],
        }]))
    }

    #[tokio::test]
    async fn xds_router_routes_to_correct_cluster() {
        let cache = Arc::new(XdsCache::new());
        cache.publish_route_config_for_test(make_route_config("my-cluster"));

        let router = XdsRouter::for_test(&cache);
        tokio::task::yield_now().await; // let watch task propagate

        let headers = http::HeaderMap::new();
        let input = RouteInput {
            authority: "my-service",
            path: "/",
            headers: &headers,
        };
        let config = router.snapshot().expect("config");
        let decision = router.route(&input, &config).unwrap();
        assert_eq!(decision.cluster, "my-cluster");
    }

    #[tokio::test]
    async fn xds_router_updates_on_config_change() {
        let cache = Arc::new(XdsCache::new());
        cache.publish_route_config_for_test(make_route_config("cluster-a"));

        let router = XdsRouter::for_test(&cache);
        tokio::task::yield_now().await;

        let headers = http::HeaderMap::new();
        let input = RouteInput {
            authority: "svc",
            path: "/",
            headers: &headers,
        };

        let config = router.snapshot().expect("config");
        let decision = router.route(&input, &config).unwrap();
        assert_eq!(decision.cluster, "cluster-a");

        cache.publish_route_config_for_test(make_route_config("cluster-b"));
        tokio::task::yield_now().await;

        let config = router.snapshot().expect("config");
        let decision = router.route(&input, &config).unwrap();
        assert_eq!(decision.cluster, "cluster-b");
    }

    /// Builds a no-op cluster client handle for routing tests.
    fn noop_cluster_client(cluster_name: &str, _watches: ClusterWatches) -> ClusterRef {
        use crate::client::cluster::{ClusterChannel, ConnectSignal};
        use crate::common::async_util::BoxFuture;

        let svc = tower::service_fn(|_: ()| -> BoxFuture<Result<(), tower::BoxError>> {
            Box::pin(async { Ok(()) })
        });
        ClusterRef::from_channel(
            cluster_name,
            ClusterChannel::from_service(svc, ConnectSignal::default()),
        )
    }

    type ProvisionedClusters = Arc<std::sync::Mutex<Vec<String>>>;

    /// Returns a provisioning closure that records each cluster it builds a
    /// (no-op) client for.
    fn recording_provisioner() -> (
        impl FnMut(&str, ClusterWatches) -> ClusterRef + Send + 'static,
        ProvisionedClusters,
    ) {
        let provisioned = Arc::new(std::sync::Mutex::new(Vec::new()));
        let recorded = provisioned.clone();
        let provision = move |cluster_name: &str, watches: ClusterWatches| {
            recorded.lock().unwrap().push(cluster_name.to_string());
            noop_cluster_client(cluster_name, watches)
        };
        (provision, provisioned)
    }

    fn router_with_clusters(cache: &Arc<XdsCache>) -> (XdsRouter, ProvisionedClusters) {
        let (provision, provisioned) = recording_provisioner();
        let router = XdsRouter::new(
            cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            provision,
        );
        (router, provisioned)
    }

    impl XdsRouter {
        /// A router for tests that only check route matching: it compiles gRPC
        /// retry policies, and its cluster clients are no-ops.
        pub(crate) fn for_test(cache: &Arc<XdsCache>) -> Self {
            Self::new(
                cache,
                Arc::new(GrpcRetryClassifierFactory),
                Some(DEFAULT_READY_TIMEOUT),
                noop_cluster_client,
            )
        }
    }

    /// Routes a request and returns the cluster client its decision carries.
    fn routed_client(router: &XdsRouter) -> ClusterRef {
        let headers = http::HeaderMap::new();
        let input = RouteInput {
            authority: "svc",
            path: "/",
            headers: &headers,
        };
        let config = router.snapshot().expect("config");
        let decision = router.route(&input, &config).unwrap();
        let client = decision
            .cluster_ref
            .and_then(Result::ok)
            .expect("decision carries a client");
        assert!(client.same_client(config.cluster(&decision.cluster).unwrap()));
        client
    }

    #[tokio::test]
    async fn xds_router_owns_clients_of_referenced_clusters() {
        let cache = Arc::new(XdsCache::new());
        cache.publish_route_config_for_test(make_route_config("cluster-a"));
        let (router, provisioned) = router_with_clusters(&cache);
        tokio::task::yield_now().await;
        let first_a = routed_client(&router);

        // A new route config that still references the cluster keeps its client.
        cache.publish_route_config_for_test(make_route_config("cluster-a"));
        tokio::task::yield_now().await;
        assert!(routed_client(&router).same_client(&first_a));

        // A route config that stops referencing the cluster drops its client.
        cache.publish_route_config_for_test(make_route_config("cluster-b"));
        tokio::task::yield_now().await;
        routed_client(&router);
        let config = router.snapshot().unwrap();
        assert!(config.cluster("cluster-a").is_none());

        // Re-adding the cluster builds a fresh client.
        cache.publish_route_config_for_test(make_route_config("cluster-a"));
        tokio::task::yield_now().await;
        assert!(!routed_client(&router).same_client(&first_a));
        assert!(router.snapshot().unwrap().cluster("cluster-b").is_none());

        assert_eq!(
            *provisioned.lock().unwrap(),
            ["cluster-a", "cluster-b", "cluster-a"]
        );
    }

    #[tokio::test]
    async fn xds_router_updates_watches_when_readded_cluster_coalesces() {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        let make_cluster = || {
            Arc::new(ClusterResource {
                name: "cluster-a".into(),
                eds_service_name: None,
                lb_policy: LbPolicy::RoundRobin,
                security: None,
                circuit_breaking: CircuitBreakingConfig::default(),
                outlier_detection: Default::default(),
            })
        };
        let make_endpoints = || {
            Arc::new(EndpointsResource {
                cluster_name: "cluster-a".into(),
                localities: vec![],
            })
        };

        let cache = Arc::new(XdsCache::new());
        let c1 = make_cluster();
        let ep1 = make_endpoints();
        cache.publish(Arc::new(XdsConfig {
            route_config: make_route_config("cluster-a"),
            virtual_host_index: 0,
            clusters: HashMap::from([(
                "cluster-a".to_string(),
                Ok(ClusterSnapshot {
                    cluster: Arc::clone(&c1),
                    endpoints: Some(Arc::clone(&ep1)),
                    resolution_note: None,
                }),
            )]),
        }));

        let (watches_tx, mut watches_rx) = tokio::sync::mpsc::unbounded_channel();
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            move |name, watches| {
                let _ = watches_tx.send(watches);
                noop_cluster_client(name, ClusterWatches::closed())
            },
        );
        tokio::task::yield_now().await;
        let first = routed_client(&router);
        let mut watches = watches_rx.recv().await.expect("initial cluster watches");
        assert!(Arc::ptr_eq(&watches.cluster.next().await.unwrap(), &c1));
        assert!(Arc::ptr_eq(&watches.endpoints.next().await.unwrap(), &ep1));

        // The cluster leaves and rejoins with complete v2 resources before the
        // router wakes, so the watch coalesces directly to the second snapshot.
        // Because every published snapshot is complete, keeping the warm client
        // and forwarding v2 over `ClusterWatches` converges without a rebuild.
        cache.publish_route_config_for_test(make_route_config("cluster-b"));
        let c2 = make_cluster();
        let ep2 = make_endpoints();
        cache.publish(Arc::new(XdsConfig {
            route_config: make_route_config("cluster-a"),
            virtual_host_index: 0,
            clusters: HashMap::from([(
                "cluster-a".to_string(),
                Ok(ClusterSnapshot {
                    cluster: Arc::clone(&c2),
                    endpoints: Some(Arc::clone(&ep2)),
                    resolution_note: None,
                }),
            )]),
        }));
        tokio::task::yield_now().await;

        assert!(routed_client(&router).same_client(&first));
        assert!(watches_rx.try_recv().is_err());
        assert!(Arc::ptr_eq(&watches.cluster.next().await.unwrap(), &c2));
        assert!(Arc::ptr_eq(&watches.endpoints.next().await.unwrap(), &ep2));
    }

    #[tokio::test]
    async fn xds_router_forwards_cds_and_eds_without_rebuilding_routing_snapshot() {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        let rc = make_route_config("cluster-a");
        let make_cluster = || {
            Arc::new(ClusterResource {
                name: "cluster-a".into(),
                eds_service_name: None,
                lb_policy: LbPolicy::RoundRobin,
                security: None,
                circuit_breaking: CircuitBreakingConfig::default(),
                outlier_detection: Default::default(),
            })
        };
        let make_endpoints = || {
            Arc::new(EndpointsResource {
                cluster_name: "cluster-a".into(),
                localities: vec![],
            })
        };
        let publish = |cache: &XdsCache,
                       cluster: &Arc<ClusterResource>,
                       endpoints: &Arc<EndpointsResource>,
                       resolution_note: Option<&str>| {
            cache.publish(Arc::new(XdsConfig {
                route_config: Arc::clone(&rc),
                virtual_host_index: 0,
                clusters: HashMap::from([(
                    "cluster-a".to_string(),
                    Ok(ClusterSnapshot {
                        cluster: Arc::clone(cluster),
                        endpoints: Some(Arc::clone(endpoints)),
                        resolution_note: resolution_note.map(Into::into),
                    }),
                )]),
            }));
        };

        let c0 = make_cluster();
        let ep0 = make_endpoints();
        let cache = Arc::new(XdsCache::new());
        publish(&cache, &c0, &ep0, None);

        let (watches_tx, mut watches_rx) = tokio::sync::mpsc::unbounded_channel();
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            move |name, watches| {
                let _ = watches_tx.send((name.to_string(), watches));
                noop_cluster_client(name, ClusterWatches::closed())
            },
        );
        tokio::task::yield_now().await;

        let initial_snapshot = router.snapshot().expect("initial routing snapshot");
        let (name, mut watches) = watches_rx.recv().await.expect("cluster provisioned");
        assert_eq!(name, "cluster-a");
        assert!(Arc::ptr_eq(&watches.cluster.next().await.unwrap(), &c0));
        assert!(Arc::ptr_eq(&watches.endpoints.next().await.unwrap(), &ep0));

        // Pure CDS and EDS updates forward to ClusterWatches without rebuilding RoutingSnapshot.
        let c1 = make_cluster();
        publish(&cache, &c1, &ep0, None);
        tokio::task::yield_now().await;
        assert!(Arc::ptr_eq(&watches.cluster.next().await.unwrap(), &c1));
        assert!(Arc::ptr_eq(&router.snapshot().unwrap(), &initial_snapshot));

        let ep1 = make_endpoints();
        publish(&cache, &c1, &ep1, None);
        tokio::task::yield_now().await;
        assert!(Arc::ptr_eq(&watches.endpoints.next().await.unwrap(), &ep1));
        assert!(Arc::ptr_eq(&router.snapshot().unwrap(), &initial_snapshot));

        // A new note alone, e.g. from an ambient error, leaves the cluster as is.
        publish(&cache, &c1, &ep1, Some("LDS resource 'listener': NACKed"));
        tokio::task::yield_now().await;
        let update = tokio::time::timeout(Duration::from_millis(20), async {
            tokio::select! {
                _ = watches.cluster.next() => {}
                _ = watches.endpoints.next() => {}
            }
        })
        .await;
        assert!(update.is_err(), "unchanged resources must not be forwarded");
        assert!(Arc::ptr_eq(&router.snapshot().unwrap(), &initial_snapshot));
        assert!(
            watches_rx.try_recv().is_err(),
            "cluster must not be rebuilt"
        );

        // Removing the cluster from the route config drops its senders and closes both watches.
        cache.publish_route_config_for_test(make_route_config("cluster-b"));
        tokio::task::yield_now().await;
        assert!(watches.cluster.next().await.is_none());
        assert!(watches.endpoints.next().await.is_none());
    }

    #[tokio::test]
    async fn xds_router_surfaces_top_level_config_error_and_recovers() {
        let cache = Arc::new(XdsCache::new());
        let (watches_tx, mut watches_rx) = tokio::sync::mpsc::unbounded_channel();
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            move |name, watches| {
                let _ = watches_tx.send(watches);
                noop_cluster_client(name, ClusterWatches::closed())
            },
        );

        // An RPC racing startup wakes immediately on a top-level LDS/RDS error
        // with UNAVAILABLE instead of blocking until DEFAULT_READY_TIMEOUT.
        let pending = router.acquire();
        assert!(matches!(pending, AcquiredConfig::Pending(_)));
        cache.publish_error(XdsConfigError("LDS resource 'svc': does not exist".into()));
        let err = pending.get().await.unwrap_err();
        assert!(matches!(err, RoutingError::Unavailable(_)));
        let status = tonic::Status::from_error(Box::new(err));
        assert_eq!(status.code(), tonic::Code::Unavailable);
        assert!(status.message().contains("LDS resource 'svc'"));

        // Steady-state calls while failed return AcquiredConfig::Failed directly.
        assert!(matches!(router.acquire(), AcquiredConfig::Failed(_)));

        // Publishing a complete snapshot recovers routing and provisions the cluster.
        cache.publish_route_config_for_test(make_route_config("cluster-a"));
        tokio::task::yield_now().await;
        let mut watches = watches_rx.recv().await.expect("cluster provisioned");
        assert!(watches.cluster.next().await.is_some());
        assert!(watches.endpoints.next().await.is_some());
        routed_client(&router);
        assert!(router.snapshot().unwrap().cluster("cluster-a").is_some());

        // A subsequent top-level failure tears down active cluster watches.
        cache.publish_error(XdsConfigError("RDS resource 'rc': does not exist".into()));
        tokio::task::yield_now().await;
        assert!(router.snapshot().is_none());
        assert!(watches.cluster.next().await.is_none());
        assert!(watches.endpoints.next().await.is_none());
    }

    #[tokio::test]
    async fn xds_router_surfaces_per_cluster_error_without_affecting_healthy_clusters() {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        let rc = Arc::new(simple_rc(vec![VirtualHostConfig {
            name: "vh".into(),
            domains: vec!["*".into()],
            routes: vec![
                simple_route("/ok", "healthy"),
                simple_route("/bad", "broken"),
            ],
        }]));
        let healthy_snap = || ClusterSnapshot {
            cluster: Arc::new(ClusterResource {
                name: "healthy".into(),
                eds_service_name: None,
                lb_policy: LbPolicy::RoundRobin,
                security: None,
                circuit_breaking: CircuitBreakingConfig::default(),
                outlier_detection: Default::default(),
            }),
            endpoints: Some(Arc::new(EndpointsResource {
                cluster_name: "healthy".into(),
                localities: vec![],
            })),
            resolution_note: None,
        };

        let cache = Arc::new(XdsCache::new());
        cache.publish(Arc::new(XdsConfig {
            route_config: Arc::clone(&rc),
            virtual_host_index: 0,
            clusters: HashMap::from([
                ("healthy".to_string(), Ok(healthy_snap())),
                (
                    "broken".to_string(),
                    Err(ClusterError("CDS resource 'broken': does not exist".into())),
                ),
            ]),
        }));

        let (watches_tx, mut watches_rx) = tokio::sync::mpsc::unbounded_channel();
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            move |name, watches| {
                let _ = watches_tx.send((name.to_string(), watches));
                noop_cluster_client(name, ClusterWatches::closed())
            },
        );
        tokio::task::yield_now().await;

        // Only the healthy cluster is provisioned.
        let (prov_name, mut healthy_watches) = watches_rx.recv().await.unwrap();
        assert_eq!(prov_name, "healthy");
        assert!(watches_rx.try_recv().is_err());
        assert!(healthy_watches.cluster.next().await.is_some());
        assert!(healthy_watches.endpoints.next().await.is_some());

        let snapshot = router.snapshot().expect("snapshot available");
        let headers = http::HeaderMap::new();
        let ok_decision = router
            .route(
                &RouteInput {
                    authority: "svc",
                    path: "/ok",
                    headers: &headers,
                },
                &snapshot,
            )
            .expect("healthy route succeeds");
        assert_eq!(ok_decision.cluster, "healthy");
        assert!(matches!(ok_decision.cluster_ref, Some(Ok(_))));

        // A failed cluster still routes; the decision carries its error for the
        // LB stage to fail the attempt with.
        let bad_decision = router
            .route(
                &RouteInput {
                    authority: "svc",
                    path: "/bad",
                    headers: &headers,
                },
                &snapshot,
            )
            .expect("route to a failed cluster resolves");
        let Some(Err(err)) = bad_decision.cluster_ref else {
            panic!("decision carries the cluster error");
        };
        assert_eq!(err.to_string(), "CDS resource 'broken': does not exist");

        // If the healthy cluster later fails, its watches close cleanly.
        cache.publish(Arc::new(XdsConfig {
            route_config: Arc::clone(&rc),
            virtual_host_index: 0,
            clusters: HashMap::from([
                (
                    "healthy".to_string(),
                    Err(ClusterError(
                        "CDS resource 'healthy': does not exist".into(),
                    )),
                ),
                (
                    "broken".to_string(),
                    Err(ClusterError("CDS resource 'broken': does not exist".into())),
                ),
            ]),
        }));
        tokio::task::yield_now().await;
        assert!(healthy_watches.cluster.next().await.is_none());
        assert!(healthy_watches.endpoints.next().await.is_none());
    }

    #[tokio::test]
    async fn xds_router_carries_eds_failure_until_eds_recovers() {
        use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
        use crate::xds::resource::cluster::LbPolicy;

        let rc = make_route_config("cluster-a");
        let cluster = Arc::new(ClusterResource {
            name: "cluster-a".into(),
            eds_service_name: None,
            lb_policy: LbPolicy::RoundRobin,
            security: None,
            circuit_breaking: CircuitBreakingConfig::default(),
            outlier_detection: Default::default(),
        });
        let publish = |cache: &XdsCache, endpoints: Option<Arc<EndpointsResource>>| {
            cache.publish(Arc::new(XdsConfig {
                route_config: Arc::clone(&rc),
                virtual_host_index: 0,
                clusters: HashMap::from([(
                    "cluster-a".to_string(),
                    Ok(ClusterSnapshot {
                        cluster: Arc::clone(&cluster),
                        resolution_note: endpoints
                            .is_none()
                            .then(|| "EDS resource 'cluster-a': does not exist".into()),
                        endpoints,
                    }),
                )]),
            }));
        };

        let cache = Arc::new(XdsCache::new());
        publish(&cache, None);
        let (watches_tx, mut watches_rx) = tokio::sync::mpsc::unbounded_channel();
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(DEFAULT_READY_TIMEOUT),
            move |name, watches| {
                let _ = watches_tx.send(watches);
                noop_cluster_client(name, ClusterWatches::closed())
            },
        );
        tokio::task::yield_now().await;

        let headers = http::HeaderMap::new();
        let input = RouteInput {
            authority: "svc",
            path: "/",
            headers: &headers,
        };
        let assert_carries_note = || {
            let snapshot = router.snapshot().expect("snapshot available");
            let decision = router.route(&input, &snapshot).expect("route resolves");
            let Some(Err(err)) = decision.cluster_ref else {
                panic!("decision carries the EDS error");
            };
            assert!(err.to_string().contains("EDS resource 'cluster-a'"));
        };

        // Without endpoints the cluster is not provisioned, but still routes,
        // carrying the note for the LB stage to fail the attempt with.
        assert!(watches_rx.try_recv().is_err());
        assert_carries_note();

        // Once EDS resolves, the cluster is provisioned with those endpoints.
        let endpoints = Arc::new(EndpointsResource {
            cluster_name: "cluster-a".into(),
            localities: vec![],
        });
        publish(&cache, Some(Arc::clone(&endpoints)));
        tokio::task::yield_now().await;
        let mut watches = watches_rx.recv().await.expect("cluster provisioned");
        assert!(Arc::ptr_eq(
            &watches.endpoints.next().await.unwrap(),
            &endpoints
        ));
        routed_client(&router);

        // A later EDS failure drops the provisioned cluster, closing its
        // watches, and routes carry the note until EDS recovers.
        publish(&cache, None);
        tokio::task::yield_now().await;
        assert!(watches.endpoints.next().await.is_none());
        assert_carries_note();

        publish(&cache, Some(Arc::clone(&endpoints)));
        tokio::task::yield_now().await;
        watches_rx.recv().await.expect("cluster provisioned again");
        routed_client(&router);
    }

    #[tokio::test(start_paused = true)]
    async fn xds_router_returns_not_ready_without_config() {
        let cache = Arc::new(XdsCache::new());
        let router = XdsRouter::for_test(&cache);

        // With no config yet, `acquire` yields the waiting variant, which
        // blocks and then reports NotReady.
        assert!(router.snapshot().is_none());
        assert!(matches!(router.acquire(), AcquiredConfig::Pending(_)));

        let start = tokio::time::Instant::now();
        let result = router.acquire().get().await;

        assert!(
            matches!(result, Err(RoutingError::NotReady)),
            "expected NotReady, got {result:?}",
        );
        assert!(
            start.elapsed() >= DEFAULT_READY_TIMEOUT,
            "expected the wait to span the full ready timeout, took {:?}",
            start.elapsed(),
        );
    }

    #[tokio::test(start_paused = true)]
    async fn xds_router_respects_custom_ready_timeout() {
        let cache = Arc::new(XdsCache::new());
        let custom_timeout = Duration::from_secs(2);
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            Some(custom_timeout),
            noop_cluster_client,
        );

        let start = tokio::time::Instant::now();
        let result = router.acquire().get().await;

        assert!(
            matches!(result, Err(RoutingError::NotReady)),
            "expected NotReady, got {result:?}",
        );
        assert_eq!(start.elapsed(), custom_timeout);
    }

    #[tokio::test(start_paused = true)]
    async fn xds_router_without_ready_timeout_waits_past_default_until_config_arrives() {
        let cache = Arc::new(XdsCache::new());
        let router = XdsRouter::new(
            &cache,
            Arc::new(GrpcRetryClassifierFactory),
            None,
            noop_cluster_client,
        );

        let fut = router.acquire().get();
        let mut fut = std::pin::pin!(fut);

        // Advance well beyond DEFAULT_READY_TIMEOUT (30s): with ready_timeout = None,
        // the router must remain pending until a route config arrives.
        assert!(
            tokio::time::timeout(Duration::from_secs(120), &mut fut)
                .await
                .is_err(),
            "acquire with None ready_timeout should not expire on its own",
        );

        cache.publish_route_config_for_test(make_route_config("late-cluster"));

        let snapshot = fut
            .await
            .expect("should resolve once route config arrives after >30s");
        let headers = http::HeaderMap::new();
        let input = RouteInput {
            authority: "svc",
            path: "/",
            headers: &headers,
        };
        let decision = router.route(&input, &snapshot).unwrap();
        assert_eq!(decision.cluster, "late-cluster");
    }

    #[test]
    fn max_stream_duration_resolution() {
        // (route value, decision); zero means no limit.
        for (route_msd, want) in [
            (None, None),
            (Some(Duration::from_secs(2)), Some(Duration::from_secs(2))),
            (Some(Duration::ZERO), None),
        ] {
            let mut route = simple_route("/", "c");
            route.max_stream_duration = route_msd;
            let rc = Arc::new(simple_rc(vec![VirtualHostConfig {
                name: "vh".into(),
                domains: vec!["*".into()],
                routes: vec![route],
            }]));
            let snapshot = RoutingSnapshot::new(rc, 0, &GrpcRetryClassifierFactory, HashMap::new());
            let decision = resolve_route(&snapshot, "svc", "/", &http::HeaderMap::new()).unwrap();
            assert_eq!(decision.max_stream_duration, want, "{route_msd:?}");
        }
    }
}
