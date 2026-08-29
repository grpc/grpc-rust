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

#![cfg_attr(not(test), allow(dead_code))]

use std::fmt;
use std::sync::{
    Arc, Mutex, OnceLock,
    atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
};
use std::task::{Context, Poll};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use dashmap::DashMap;
use futures_util::StreamExt as _;
use http::{Request, Response};
use http_body::{Body, Frame};
use pin_project_lite::pin_project;
use tokio::sync::watch;
use tonic::body::Body as TonicBody;
use tower::Layer;
use tower::discover::Change;
use tower::load::Load;
use tower::{BoxError, Service};

use crate::client::lb::{BoxDiscover, ClusterDiscovery};
use crate::client::route::RouteDecision;
use crate::common::async_util::{AbortOnDrop, BoxFuture};
use crate::xds::cache::{CacheEvent, XdsCache};
use crate::xds::resource::ClusterResource;
use crate::xds::resource::circuit_breaking::{CircuitBreakingConfig, DEFAULT_MAX_REQUESTS};

static GLOBAL_COUNTERS: OnceLock<Arc<ClusterRequestCounterState>> = OnceLock::new();

/// Shared circuit-breaking state for xDS clusters.
#[derive(Clone, Debug)]
pub(crate) struct ClusterCircuitBreakerRegistry {
    inner: Arc<ClusterCircuitBreakerRegistryInner>,
}

#[derive(Debug)]
struct ClusterCircuitBreakerRegistryInner {
    configs: DashMap<String, Arc<ClusterCircuitBreakerState>>,
    counters: ClusterRequestCounters,
}

impl Drop for ClusterCircuitBreakerRegistryInner {
    fn drop(&mut self) {
        for state in self.configs.iter() {
            if let Some(previous) = state.config.swap(None) {
                let counter_key = previous.counter_key.clone();
                previous.counter.deactivate();
                drop(previous);
                self.counters.cleanup_if_unused(&counter_key);
            }
        }
    }
}

impl ClusterCircuitBreakerRegistry {
    pub(crate) fn new() -> Self {
        Self::with_counters(ClusterRequestCounters::global())
    }

    fn with_counters(counters: ClusterRequestCounters) -> Self {
        Self {
            inner: Arc::new(ClusterCircuitBreakerRegistryInner {
                configs: DashMap::new(),
                counters,
            }),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_config(&self, cluster: impl Into<String>, config: CircuitBreakingConfig) {
        let cluster = cluster.into();
        self.set_cluster_config(cluster.clone(), cluster, config);
    }

    pub(crate) fn set_cluster_config(
        &self,
        cluster: impl Into<String>,
        eds_service_name: impl Into<String>,
        config: CircuitBreakingConfig,
    ) {
        let cluster = cluster.into();
        let eds_service_name = eds_service_name.into();
        let counter_key = CounterKey::new(cluster.as_str(), eds_service_name.as_str());
        let counter = self.inner.counters.counter(&counter_key);
        let state = self.ensure_state(&cluster);
        self.update_state_config(
            &state,
            CircuitBreakerRuntimeConfig {
                max_requests: config.max_requests,
                counter_key,
                counter,
            },
            0,
        );
    }

    fn ensure_cluster_watch(
        &self,
        cache: &Arc<XdsCache>,
        cluster: &str,
        state: &Arc<ClusterCircuitBreakerState>,
    ) -> u64 {
        let mut lifecycle = state
            .watch_lifecycle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if lifecycle.running {
            return lifecycle.generation;
        }

        let mut cluster_watch = cache.watch_cluster(cluster);
        lifecycle.generation = lifecycle.generation.wrapping_add(1).max(1);
        let generation = lifecycle.generation;
        lifecycle.running = true;
        state
            .active_watch_generation
            .store(generation, Ordering::Release);

        let weak_registry = Arc::downgrade(&self.inner);
        let weak_state = Arc::downgrade(state);
        let task = tokio::spawn(async move {
            while let Some(event) = cluster_watch.next_event().await {
                let CacheEvent::Resource {
                    resource: cluster_resource,
                    ..
                } = event
                else {
                    break;
                };
                let Some(inner) = weak_registry.upgrade() else {
                    return;
                };
                let Some(state) = weak_state.upgrade() else {
                    return;
                };
                let circuit_breakers = ClusterCircuitBreakerRegistry { inner };
                let config = CircuitBreakerRuntimeConfig::from_cluster(
                    &cluster_resource,
                    &circuit_breakers.inner.counters,
                );
                circuit_breakers.update_state_config(&state, config, generation);
            }

            let Some(inner) = weak_registry.upgrade() else {
                return;
            };
            let Some(state) = weak_state.upgrade() else {
                return;
            };
            ClusterCircuitBreakerRegistry { inner }.finish_cluster_watch(&state, generation);
        });
        lifecycle._task = Some(AbortOnDrop(task));
        generation
    }

    fn ensure_state(&self, cluster: &str) -> Arc<ClusterCircuitBreakerState> {
        if let Some(state) = self.inner.configs.get(cluster) {
            return state.clone();
        }

        self.inner
            .configs
            .entry(cluster.to_string())
            .or_insert_with(|| Arc::new(ClusterCircuitBreakerState::new()))
            .clone()
    }

    fn cluster_breaker(&self, cluster: &str) -> Arc<ClusterCircuitBreaker> {
        self.cluster_breaker_with_optional_cache(cluster, None)
    }

    fn cluster_breaker_with_cache(
        &self,
        cluster: &str,
        cluster_cache: Arc<XdsCache>,
    ) -> Arc<ClusterCircuitBreaker> {
        self.cluster_breaker_with_optional_cache(cluster, Some(cluster_cache))
    }

    fn cluster_breaker_with_optional_cache(
        &self,
        cluster: &str,
        cluster_cache: Option<Arc<XdsCache>>,
    ) -> Arc<ClusterCircuitBreaker> {
        let state = self.ensure_state(cluster);
        let cluster: Arc<str> = Arc::from(cluster);
        let default_counter_key = CounterKey::same_cluster(cluster.clone());
        let breaker = Arc::new(ClusterCircuitBreaker {
            cluster,
            state,
            counters: self.inner.counters.clone(),
            default_counter_key,
            registry: self.clone(),
            cluster_cache,
        });
        if let Some(cache) = breaker.cluster_cache.as_ref() {
            self.ensure_cluster_watch(cache, &breaker.cluster, &breaker.state);
        }
        breaker
    }

    fn update_state_config(
        &self,
        state: &ClusterCircuitBreakerState,
        config: CircuitBreakerRuntimeConfig,
        watch_generation: u64,
    ) {
        let _update_guard = state
            .update_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if watch_generation != 0
            && state.active_watch_generation.load(Ordering::Acquire) != watch_generation
        {
            return;
        }
        let previous = state.current_config();
        if previous.as_deref() == Some(&config) {
            state
                .config_watch_generation
                .store(watch_generation, Ordering::Release);
            state.notify_config_changed();
            return;
        }

        let counter_key_changed = previous
            .as_ref()
            .is_none_or(|previous| previous.counter_key != config.counter_key);
        if counter_key_changed {
            config.counter.activate();
        }

        drop(previous);
        let previous = state.config.swap(Some(Arc::new(config)));
        if counter_key_changed && let Some(previous) = previous {
            self.deactivate_config(previous);
        }
        state
            .config_watch_generation
            .store(watch_generation, Ordering::Release);
        state.notify_config_changed();
    }

    fn finish_cluster_watch(&self, state: &Arc<ClusterCircuitBreakerState>, generation: u64) {
        let mut lifecycle = state
            .watch_lifecycle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if lifecycle.generation != generation {
            return;
        }

        self.clear_state(state);
        state
            .finished_watch_generation
            .store(generation, Ordering::Release);
        lifecycle.running = false;
        state.notify_config_changed();
    }

    fn clear_state(&self, state: &ClusterCircuitBreakerState) {
        let _update_guard = state
            .update_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(previous) = state.config.swap(None) {
            self.deactivate_config(previous);
        }
        state.config_watch_generation.store(0, Ordering::Release);
    }

    fn deactivate_config(&self, config: Arc<CircuitBreakerRuntimeConfig>) {
        let counter_key = config.counter_key.clone();
        config.counter.deactivate();
        drop(config);
        self.inner.counters.cleanup_if_unused(&counter_key);
    }

    fn acquire(&self, cluster: &str) -> Result<CircuitBreakerPermit, CircuitBreakerLimit> {
        self.cluster_breaker(cluster).acquire()
    }

    #[cfg(test)]
    fn in_flight(&self, cluster: &str) -> u32 {
        let breaker = self.cluster_breaker(cluster);
        self.inner
            .counters
            .in_flight(&breaker.current_counter_key())
    }

    #[cfg(test)]
    fn dropped_requests(&self, cluster: &str) -> u64 {
        self.ensure_state(cluster).dropped_requests()
    }

    #[cfg(test)]
    fn counter_count(&self) -> usize {
        self.inner.counters.counter_count()
    }

    #[cfg(test)]
    fn clear_cluster_config(&self, cluster: &str) {
        let state = self.inner.configs.get(cluster).map(|state| state.clone());
        if let Some(state) = state {
            self.clear_state(&state);
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self::with_counters(ClusterRequestCounters::isolated())
    }
}

impl Default for ClusterCircuitBreakerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
struct CircuitBreakerLimit {
    max_requests: u32,
}

#[derive(Clone, Debug)]
struct CircuitBreakerRuntimeConfig {
    max_requests: u32,
    counter_key: CounterKey,
    counter: Arc<InFlightCounter>,
}

impl PartialEq for CircuitBreakerRuntimeConfig {
    fn eq(&self, other: &Self) -> bool {
        self.max_requests == other.max_requests && self.counter_key == other.counter_key
    }
}

impl Eq for CircuitBreakerRuntimeConfig {}

impl CircuitBreakerRuntimeConfig {
    fn from_cluster(cluster: &ClusterResource, counters: &ClusterRequestCounters) -> Self {
        let counter_key = CounterKey::new(cluster.name.as_str(), cluster.eds_service_name());
        let counter = counters.counter(&counter_key);
        Self {
            max_requests: cluster.circuit_breaking.max_requests,
            counter_key,
            counter,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CounterKey {
    cluster: Arc<str>,
    eds_service_name: Arc<str>,
}

impl CounterKey {
    fn new(cluster: impl Into<Arc<str>>, eds_service_name: impl Into<Arc<str>>) -> Self {
        Self {
            cluster: cluster.into(),
            eds_service_name: eds_service_name.into(),
        }
    }

    fn same_cluster(cluster: Arc<str>) -> Self {
        Self {
            cluster: cluster.clone(),
            eds_service_name: cluster,
        }
    }
}

struct ClusterCircuitBreakerState {
    config: ArcSwapOption<CircuitBreakerRuntimeConfig>,
    update_lock: Mutex<()>,
    dropped_requests: AtomicU64,
    watch_lifecycle: Mutex<ClusterWatchLifecycle>,
    active_watch_generation: AtomicU64,
    config_watch_generation: AtomicU64,
    finished_watch_generation: AtomicU64,
    config_version: watch::Sender<u64>,
}

#[derive(Default)]
struct ClusterWatchLifecycle {
    generation: u64,
    running: bool,
    _task: Option<AbortOnDrop>,
}

impl fmt::Debug for ClusterCircuitBreakerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let watch_running = self
            .watch_lifecycle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .running;
        f.debug_struct("ClusterCircuitBreakerState")
            .field("current_config", &self.current_config())
            .field(
                "dropped_requests",
                &self.dropped_requests.load(Ordering::Acquire),
            )
            .field("watch_running", &watch_running)
            .field(
                "active_watch_generation",
                &self.active_watch_generation.load(Ordering::Acquire),
            )
            .field(
                "config_watch_generation",
                &self.config_watch_generation.load(Ordering::Acquire),
            )
            .field(
                "finished_watch_generation",
                &self.finished_watch_generation.load(Ordering::Acquire),
            )
            .finish()
    }
}

impl ClusterCircuitBreakerState {
    fn new() -> Self {
        let (config_version, _) = watch::channel(0);
        Self {
            config: ArcSwapOption::empty(),
            update_lock: Mutex::new(()),
            dropped_requests: AtomicU64::new(0),
            watch_lifecycle: Mutex::new(ClusterWatchLifecycle::default()),
            active_watch_generation: AtomicU64::new(0),
            config_watch_generation: AtomicU64::new(0),
            finished_watch_generation: AtomicU64::new(0),
            config_version,
        }
    }

    fn current_config(&self) -> Option<Arc<CircuitBreakerRuntimeConfig>> {
        self.config.load_full()
    }

    fn record_drop(&self) {
        self.dropped_requests.fetch_add(1, Ordering::AcqRel);
    }

    fn notify_config_changed(&self) {
        self.config_version
            .send_modify(|version| *version = version.wrapping_add(1));
    }

    #[cfg(test)]
    fn dropped_requests(&self) -> u64 {
        self.dropped_requests.load(Ordering::Acquire)
    }
}

struct ClusterCircuitBreaker {
    cluster: Arc<str>,
    state: Arc<ClusterCircuitBreakerState>,
    counters: ClusterRequestCounters,
    default_counter_key: CounterKey,
    registry: ClusterCircuitBreakerRegistry,
    cluster_cache: Option<Arc<XdsCache>>,
}

impl fmt::Debug for ClusterCircuitBreaker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterCircuitBreaker")
            .field("cluster", &self.cluster)
            .field("state", &self.state)
            .field("watching_cluster_cache", &self.cluster_cache.is_some())
            .finish()
    }
}

impl ClusterCircuitBreaker {
    fn acquire(&self) -> Result<CircuitBreakerPermit, CircuitBreakerLimit> {
        if let Some(config) = self.state.current_config() {
            return self.acquire_with_config(
                config.counter_key.clone(),
                config.counter.clone(),
                config.max_requests,
            );
        }

        let counter = self.counters.counter(&self.default_counter_key);
        self.acquire_with_config(
            self.default_counter_key.clone(),
            counter,
            DEFAULT_MAX_REQUESTS,
        )
    }

    async fn acquire_when_ready(&self) -> Result<CircuitBreakerPermit, Response<TonicBody>> {
        let Some(cache) = self.cluster_cache.as_ref() else {
            return self
                .acquire()
                .map_err(|limit| limit_exceeded_response(&self.cluster, limit));
        };

        if let Some(config) = self.current_watched_config() {
            return self
                .acquire_with_config(
                    config.counter_key.clone(),
                    config.counter.clone(),
                    config.max_requests,
                )
                .map_err(|limit| limit_exceeded_response(&self.cluster, limit));
        }

        let generation = self
            .registry
            .ensure_cluster_watch(cache, &self.cluster, &self.state);
        let Some(config) = self.wait_for_config(generation).await else {
            return Err(cluster_unavailable_response(&self.cluster));
        };

        self.acquire_with_config(
            config.counter_key.clone(),
            config.counter.clone(),
            config.max_requests,
        )
        .map_err(|limit| limit_exceeded_response(&self.cluster, limit))
    }

    fn current_watched_config(&self) -> Option<Arc<CircuitBreakerRuntimeConfig>> {
        let generation = self.state.active_watch_generation.load(Ordering::Acquire);
        if generation == 0 {
            return None;
        }
        self.watched_config_for_generation(generation)
    }

    fn watched_config_for_generation(
        &self,
        generation: u64,
    ) -> Option<Arc<CircuitBreakerRuntimeConfig>> {
        if self.state.config_watch_generation.load(Ordering::Acquire) != generation {
            return None;
        }
        let config = self.state.current_config();
        if self.state.active_watch_generation.load(Ordering::Acquire) == generation
            && self.state.config_watch_generation.load(Ordering::Acquire) == generation
        {
            config
        } else {
            None
        }
    }

    async fn wait_for_config(&self, generation: u64) -> Option<Arc<CircuitBreakerRuntimeConfig>> {
        let mut config_version = self.state.config_version.subscribe();
        loop {
            if let Some(config) = self.watched_config_for_generation(generation) {
                return Some(config);
            }
            if self.state.finished_watch_generation.load(Ordering::Acquire) == generation
                || self.state.active_watch_generation.load(Ordering::Acquire) != generation
            {
                return None;
            }
            if config_version.changed().await.is_err() {
                return None;
            }
        }
    }

    fn acquire_with_config(
        &self,
        counter_key: CounterKey,
        counter: Arc<InFlightCounter>,
        max_requests: u32,
    ) -> Result<CircuitBreakerPermit, CircuitBreakerLimit> {
        let limit = CircuitBreakerLimit { max_requests };
        match self.counters.acquire(counter_key, counter, max_requests) {
            Some(permit) => Ok(permit),
            None => {
                self.state.record_drop();
                Err(limit)
            }
        }
    }

    fn current_counter_key(&self) -> CounterKey {
        self.state
            .current_config()
            .map(|config| config.counter_key.clone())
            .unwrap_or_else(|| self.default_counter_key.clone())
    }
}

#[derive(Clone, Debug)]
struct ClusterRequestCounters {
    inner: Arc<ClusterRequestCounterState>,
}

#[derive(Debug, Default)]
struct ClusterRequestCounterState {
    counters: DashMap<CounterKey, Arc<InFlightCounter>>,
}

impl ClusterRequestCounters {
    fn global() -> Self {
        Self {
            inner: GLOBAL_COUNTERS
                .get_or_init(|| Arc::new(ClusterRequestCounterState::default()))
                .clone(),
        }
    }

    #[cfg(test)]
    fn isolated() -> Self {
        Self {
            inner: Arc::new(ClusterRequestCounterState::default()),
        }
    }

    fn acquire(
        &self,
        counter_key: CounterKey,
        counter: Arc<InFlightCounter>,
        limit: u32,
    ) -> Option<CircuitBreakerPermit> {
        if counter.try_acquire(limit) {
            Some(CircuitBreakerPermit {
                counter: Some(counter),
                counter_key,
                counters: self.clone(),
            })
        } else {
            let should_cleanup = counter.is_unused();
            drop(counter);
            if should_cleanup {
                self.cleanup_if_unused(&counter_key);
            }
            None
        }
    }

    fn counter(&self, counter_key: &CounterKey) -> Arc<InFlightCounter> {
        self.inner
            .counters
            .entry(counter_key.clone())
            .or_insert_with(|| Arc::new(InFlightCounter::default()))
            .clone()
    }

    fn cleanup_if_unused(&self, counter_key: &CounterKey) {
        self.inner.counters.remove_if(counter_key, |_, counter| {
            counter.is_unused() && Arc::strong_count(counter) == 1
        });
    }

    #[cfg(test)]
    fn in_flight(&self, counter_key: &CounterKey) -> u32 {
        self.inner
            .counters
            .get(counter_key)
            .map(|counter| counter.in_flight())
            .unwrap_or(0)
    }

    #[cfg(test)]
    fn counter_count(&self) -> usize {
        self.inner.counters.len()
    }
}

#[derive(Debug, Default)]
struct InFlightCounter {
    in_flight: AtomicU32,
    active_refs: AtomicUsize,
}

impl InFlightCounter {
    fn activate(&self) {
        self.active_refs.fetch_add(1, Ordering::AcqRel);
    }

    fn deactivate(&self) {
        let result = self
            .active_refs
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
        assert!(
            result.is_ok(),
            "attempted to deactivate an inactive circuit breaker counter"
        );
    }

    fn try_acquire(&self, limit: u32) -> bool {
        loop {
            let current = self.in_flight.load(Ordering::Acquire);
            if current >= limit {
                return false;
            }

            if self
                .in_flight
                .compare_exchange_weak(current, current + 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
        }
    }

    fn in_flight(&self) -> u32 {
        self.in_flight.load(Ordering::Acquire)
    }

    fn release(&self) {
        let previous = self.in_flight.fetch_sub(1, Ordering::AcqRel);
        assert!(
            previous > 0,
            "attempted to release an inactive circuit breaker permit"
        );
    }

    fn is_unused(&self) -> bool {
        self.in_flight() == 0 && self.active_refs.load(Ordering::Acquire) == 0
    }
}

#[derive(Debug)]
pub(crate) struct CircuitBreakerPermit {
    counter: Option<Arc<InFlightCounter>>,
    counter_key: CounterKey,
    counters: ClusterRequestCounters,
}

impl Drop for CircuitBreakerPermit {
    fn drop(&mut self) {
        if let Some(counter) = self.counter.take() {
            counter.release();
            let should_cleanup = counter.is_unused();
            drop(counter);
            if should_cleanup {
                self.counters.cleanup_if_unused(&self.counter_key);
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct CircuitBreakingClusterDiscovery<Endpoint, S> {
    inner: Arc<dyn ClusterDiscovery<Endpoint, S>>,
    circuit_breakers: ClusterCircuitBreakerRegistry,
    cluster_cache: Option<Arc<XdsCache>>,
}

impl<Endpoint, S> CircuitBreakingClusterDiscovery<Endpoint, S> {
    pub(crate) fn new(
        inner: Arc<dyn ClusterDiscovery<Endpoint, S>>,
        circuit_breakers: ClusterCircuitBreakerRegistry,
    ) -> Self {
        Self {
            inner,
            circuit_breakers,
            cluster_cache: None,
        }
    }

    pub(crate) fn with_cluster_cache(mut self, cache: Arc<XdsCache>) -> Self {
        self.cluster_cache = Some(cache);
        self
    }
}

impl<Endpoint, S> fmt::Debug for CircuitBreakingClusterDiscovery<Endpoint, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingClusterDiscovery")
            .field("circuit_breakers", &self.circuit_breakers)
            .field("watching_cluster_cache", &self.cluster_cache.is_some())
            .finish()
    }
}

impl<Endpoint, S> ClusterDiscovery<Endpoint, CircuitBreakingEndpointService<S>>
    for CircuitBreakingClusterDiscovery<Endpoint, S>
where
    Endpoint: Send + 'static,
    S: Send + 'static,
{
    fn discover_cluster(
        &self,
        cluster_name: &str,
    ) -> BoxDiscover<Endpoint, CircuitBreakingEndpointService<S>> {
        let breaker = match self.cluster_cache.clone() {
            Some(cache) => self
                .circuit_breakers
                .cluster_breaker_with_cache(cluster_name, cache),
            None => self.circuit_breakers.cluster_breaker(cluster_name),
        };
        Box::pin(
            self.inner
                .discover_cluster(cluster_name)
                .map(move |change| {
                    change.map(|change| match change {
                        Change::Insert(endpoint, service) => Change::Insert(
                            endpoint,
                            CircuitBreakingEndpointService::new(service, breaker.clone()),
                        ),
                        Change::Remove(endpoint) => Change::Remove(endpoint),
                    })
                }),
        )
    }
}

#[derive(Clone)]
pub(crate) struct CircuitBreakingEndpointService<S> {
    inner: S,
    breaker: Arc<ClusterCircuitBreaker>,
}

impl<S> CircuitBreakingEndpointService<S> {
    fn new(inner: S, breaker: Arc<ClusterCircuitBreaker>) -> Self {
        Self { inner, breaker }
    }
}

impl<S: fmt::Debug> fmt::Debug for CircuitBreakingEndpointService<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingEndpointService")
            .field("inner", &self.inner)
            .field("breaker", &self.breaker)
            .finish()
    }
}

impl<S, B> Service<Request<B>> for CircuitBreakingEndpointService<S>
where
    S: Service<Request<B>, Response = Response<TonicBody>, Error: Into<BoxError>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response<TonicBody>;
    type Error = BoxError;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let breaker = self.breaker.clone();
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let permit = match breaker.acquire_when_ready().await {
                Ok(permit) => permit,
                Err(response) => return Ok(response),
            };

            let response = inner.call(request).await.map_err(Into::into)?;
            Ok(hold_permit(response, permit))
        })
    }
}

impl<S: Load> Load for CircuitBreakingEndpointService<S> {
    type Metric = S::Metric;

    fn load(&self) -> Self::Metric {
        self.inner.load()
    }
}

pub(crate) fn hold_permit(
    response: Response<TonicBody>,
    permit: CircuitBreakerPermit,
) -> Response<TonicBody> {
    response.map(|body| TonicBody::new(PermitBody::new(body, permit)))
}

/// Tower layer that enforces A32 max in-flight requests per xDS cluster.
///
/// This layer must wrap the ready per-cluster dispatch service inside retries so
/// each admitted call represents one upstream attempt rather than queued work.
#[derive(Clone)]
pub(crate) struct CircuitBreakingLayer {
    circuit_breakers: ClusterCircuitBreakerRegistry,
    breaker_cache: Arc<DashMap<String, Arc<ClusterCircuitBreaker>>>,
}

impl CircuitBreakingLayer {
    pub(crate) fn new(circuit_breakers: ClusterCircuitBreakerRegistry) -> Self {
        Self {
            circuit_breakers,
            breaker_cache: Arc::new(DashMap::new()),
        }
    }
}

impl fmt::Debug for CircuitBreakingLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingLayer")
            .field("circuit_breakers", &self.circuit_breakers)
            .field("cached_clusters", &self.breaker_cache.len())
            .finish()
    }
}

impl<S> Layer<S> for CircuitBreakingLayer {
    type Service = CircuitBreakingService<S>;

    fn layer(&self, service: S) -> Self::Service {
        CircuitBreakingService {
            inner: service,
            circuit_breakers: self.circuit_breakers.clone(),
            breaker_cache: self.breaker_cache.clone(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct CircuitBreakingService<S> {
    inner: S,
    circuit_breakers: ClusterCircuitBreakerRegistry,
    breaker_cache: Arc<DashMap<String, Arc<ClusterCircuitBreaker>>>,
}

impl<S: fmt::Debug> fmt::Debug for CircuitBreakingService<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingService")
            .field("inner", &self.inner)
            .field("circuit_breakers", &self.circuit_breakers)
            .field("cached_clusters", &self.breaker_cache.len())
            .finish()
    }
}

impl<S> CircuitBreakingService<S> {
    fn breaker_for_cluster(&self, cluster: &str) -> Arc<ClusterCircuitBreaker> {
        if let Some(breaker) = self.breaker_cache.get(cluster) {
            return breaker.clone();
        }

        self.breaker_cache
            .entry(cluster.to_string())
            .or_insert_with(|| self.circuit_breakers.cluster_breaker(cluster))
            .clone()
    }
}

impl<S, B> Service<Request<B>> for CircuitBreakingService<S>
where
    S: Service<Request<B>, Response = Response<TonicBody>, Error: Into<BoxError>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response<TonicBody>;
    type Error = BoxError;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let Some(cluster) = request
            .extensions()
            .get::<RouteDecision>()
            .map(|route_decision| route_decision.cluster.as_str())
        else {
            return Box::pin(async {
                Ok(status_response(tonic::Status::internal(
                    CircuitBreakingError::NoRoutingDecision.to_string(),
                )))
            });
        };

        let breaker = self.breaker_for_cluster(cluster);
        let permit = match breaker.acquire() {
            Ok(permit) => permit,
            Err(limit) => {
                return Box::pin(std::future::ready(Ok(limit_exceeded_response(
                    &breaker.cluster,
                    limit,
                ))));
            }
        };
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let response = inner.call(request).await.map_err(Into::into)?;
            Ok(response.map(|body| TonicBody::new(PermitBody::new(body, permit))))
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum CircuitBreakingError {
    #[error("No routing decision extension from the routing layer available")]
    NoRoutingDecision,
}

/// Marks responses rejected by the local circuit breaker before reaching an endpoint.
///
/// The retry layer uses this extension to distinguish local `UNAVAILABLE` drops
/// from retryable responses returned by an upstream service.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalCircuitBreakerDrop;

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalPreEndpointResponse;

pub(crate) fn is_local_circuit_breaker_drop<B>(response: &Response<B>) -> bool {
    response
        .extensions()
        .get::<LocalCircuitBreakerDrop>()
        .is_some()
}

pub(crate) fn is_local_pre_endpoint_response<B>(response: &Response<B>) -> bool {
    response
        .extensions()
        .get::<LocalPreEndpointResponse>()
        .is_some()
}

fn limit_exceeded_response(cluster: &str, limit: CircuitBreakerLimit) -> Response<TonicBody> {
    let mut response = status_response(tonic::Status::unavailable(format!(
        "circuit breaker open for cluster '{cluster}': max_requests limit {} reached",
        limit.max_requests,
    )));
    response.extensions_mut().insert(LocalCircuitBreakerDrop);
    response.extensions_mut().insert(LocalPreEndpointResponse);
    response
}

fn cluster_unavailable_response(cluster: &str) -> Response<TonicBody> {
    let mut response = status_response(tonic::Status::unavailable(format!(
        "cluster '{cluster}' is no longer available",
    )));
    response.extensions_mut().insert(LocalPreEndpointResponse);
    response
}

fn status_response(status: tonic::Status) -> Response<TonicBody> {
    status.into_http::<TonicBody>()
}

pin_project! {
    #[derive(Debug)]
    struct PermitBody<B> {
        #[pin]
        inner: B,
        permit: Option<CircuitBreakerPermit>,
    }
}

impl<B> PermitBody<B> {
    fn new(inner: B, permit: CircuitBreakerPermit) -> Self {
        Self {
            inner,
            permit: Some(permit),
        }
    }
}

impl<B> Body for PermitBody<B>
where
    B: Body<Data = Bytes>,
{
    type Data = Bytes;
    type Error = B::Error;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let mut this = self.project();
        match this.inner.as_mut().poll_frame(cx) {
            Poll::Ready(None) => {
                this.permit.take();
                Poll::Ready(None)
            }
            Poll::Ready(Some(Ok(frame))) => {
                if frame.is_trailers() {
                    this.permit.take();
                }
                Poll::Ready(Some(Ok(frame)))
            }
            Poll::Ready(Some(Err(err))) => {
                this.permit.take();
                Poll::Ready(Some(Err(err)))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::task::{Context, Poll};
    use std::time::Duration;

    use bytes::Bytes;
    use http::{HeaderMap, Request, Response};
    use http_body::{Body, Frame};
    use tonic::Code;
    use tower::Layer;
    use tower::ServiceExt;
    use tower::service_fn;

    use crate::client::retry::{
        GrpcRetryClassifier, RetryBackoffConfig, RetryConfig, RetryLayer, RetryPolicy,
    };

    use super::*;

    const CLUSTER: &str = "cluster-a";

    fn request() -> Request<TonicBody> {
        let mut request = Request::new(TonicBody::empty());
        request.extensions_mut().insert(RouteDecision {
            cluster: CLUSTER.to_string(),
            request_hash: None,
            retry_config: None,
        });
        request
    }

    fn configured_breakers(max_requests: u32) -> ClusterCircuitBreakerRegistry {
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests });
        breakers
    }

    fn cluster_resource(max_requests: u32) -> Arc<ClusterResource> {
        Arc::new(ClusterResource {
            name: CLUSTER.to_string(),
            eds_service_name: None,
            lb_policy: crate::xds::resource::cluster::LbPolicy::RoundRobin,
            security: None,
            circuit_breaking: CircuitBreakingConfig { max_requests },
        })
    }

    #[tokio::test]
    async fn rejects_requests_when_cluster_limit_is_reached() {
        let breakers = configured_breakers(1);
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();
        let service = service_fn(move |_request: Request<TonicBody>| {
            call_counter.fetch_add(1, Ordering::SeqCst);
            async { Ok::<_, BoxError>(Response::new(TonicBody::new(PendingBody))) }
        });
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let first = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        let second = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        let status = tonic::Status::from_header_map(second.headers()).unwrap();
        assert_eq!(status.code(), Code::Unavailable);
        assert!(status.message().contains("max_requests limit 1"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(breakers.dropped_requests(CLUSTER), 1);

        drop(first);
        assert_eq!(breakers.in_flight(CLUSTER), 0);

        let _third = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn cached_service_applies_live_limit_updates_without_resetting_in_flight() {
        let breakers = configured_breakers(2);
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();
        let service = service_fn(move |_request: Request<TonicBody>| {
            call_counter.fetch_add(1, Ordering::SeqCst);
            async { Ok::<_, BoxError>(Response::new(TonicBody::new(PendingBody))) }
        });
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let first = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        let second = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 2);

        breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests: 1 });

        let third = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(
            tonic::Status::from_header_map(third.headers())
                .unwrap()
                .code(),
            Code::Unavailable
        );

        drop(first);
        assert_eq!(breakers.in_flight(CLUSTER), 1);
        let fourth = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(
            tonic::Status::from_header_map(fourth.headers())
                .unwrap()
                .code(),
            Code::Unavailable
        );

        drop(second);
        assert_eq!(breakers.in_flight(CLUSTER), 0);
        let _fifth = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 3);
        assert_eq!(breakers.dropped_requests(CLUSTER), 2);
    }

    #[tokio::test]
    async fn releases_permit_when_response_body_reaches_trailers() {
        let breakers = configured_breakers(1);
        let service = service_fn(|_request: Request<TonicBody>| async {
            Ok::<_, BoxError>(Response::new(TonicBody::new(DataThenTrailersBody {
                state: BodyState::Data,
            })))
        });
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let response = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        let mut body = response.into_body();
        let data_frame = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        assert!(data_frame.unwrap().unwrap().is_data());
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        let trailers_frame = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        assert!(trailers_frame.unwrap().unwrap().is_trailers());
        assert_eq!(breakers.in_flight(CLUSTER), 0);
    }

    #[tokio::test]
    async fn releases_permit_when_response_body_returns_error() {
        let breakers = configured_breakers(1);
        let service = service_fn(|_request: Request<TonicBody>| async {
            Ok::<_, BoxError>(Response::new(TonicBody::new(ErrorBody { emitted: false })))
        });
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let response = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        let mut body = response.into_body();
        let frame = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        assert!(frame.unwrap().is_err());
        assert_eq!(breakers.in_flight(CLUSTER), 0);
    }

    #[tokio::test]
    async fn releases_permit_when_response_future_is_dropped() {
        let breakers = configured_breakers(1);
        let service = service_fn(|_request: Request<TonicBody>| async {
            std::future::pending::<Result<Response<TonicBody>, BoxError>>().await
        });
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let mut future = service.ready().await.unwrap().call(request());
        std::future::poll_fn(|cx| match Future::poll(Pin::new(&mut future), cx) {
            Poll::Pending => Poll::Ready(()),
            Poll::Ready(_) => panic!("inner service should remain pending"),
        })
        .await;
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        drop(future);
        assert_eq!(breakers.in_flight(CLUSTER), 0);
    }

    #[tokio::test]
    async fn reports_missing_route_decision_as_grpc_status() {
        let service = service_fn(|_request: Request<TonicBody>| async {
            Ok::<_, BoxError>(Response::new(TonicBody::empty()))
        });
        let mut service =
            CircuitBreakingLayer::new(ClusterCircuitBreakerRegistry::new_for_test()).layer(service);

        let response = service
            .ready()
            .await
            .unwrap()
            .call(Request::new(TonicBody::empty()))
            .await
            .unwrap();
        let status = tonic::Status::from_header_map(response.headers()).unwrap();
        assert_eq!(status.code(), Code::Internal);
        assert!(status.message().contains("No routing decision"));
    }

    #[tokio::test]
    async fn oneshot_honors_config_after_consuming_service() {
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();
        let service = service_fn(move |_request: Request<TonicBody>| {
            call_counter.fetch_add(1, Ordering::SeqCst);
            async { Ok::<_, BoxError>(Response::new(TonicBody::empty())) }
        });
        let service = CircuitBreakingLayer::new(configured_breakers(0)).layer(service);

        let response = service.oneshot(request()).await.unwrap();
        let status = tonic::Status::from_header_map(response.headers()).unwrap();

        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn limit_responses_do_not_enter_retry_policy() {
        let breakers = configured_breakers(0);
        let policy = RetryPolicy::new(
            RetryConfig::new().num_retries(4).retry_backoff(
                RetryBackoffConfig::new(Duration::from_millis(1))
                    .max_interval(Duration::from_millis(1)),
            ),
            Arc::new(GrpcRetryClassifier::new(vec![Code::Unavailable])),
        );
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();

        let service = service_fn(
            move |_request: Request<shared_http_body::SharedBody<TonicBody>>| {
                call_counter.fetch_add(1, Ordering::SeqCst);
                async { Ok::<_, BoxError>(Response::new(TonicBody::empty())) }
            },
        );
        let mut service = tower::ServiceBuilder::new()
            .layer(RetryLayer::new(policy.into_shared()))
            .layer(CircuitBreakingLayer::new(breakers.clone()))
            .service(service);

        let response = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        let status = tonic::Status::from_header_map(response.headers()).unwrap();

        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(breakers.dropped_requests(CLUSTER), 1);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn shared_counters_enforce_process_limit_with_per_channel_drop_counts() {
        let counters = ClusterRequestCounters::isolated();
        let first_breakers = ClusterCircuitBreakerRegistry::with_counters(counters.clone());
        let second_breakers = ClusterCircuitBreakerRegistry::with_counters(counters.clone());
        first_breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests: 1 });
        second_breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests: 1 });

        let first = first_breakers.acquire(CLUSTER).unwrap();
        assert!(second_breakers.acquire(CLUSTER).is_err());
        assert_eq!(first_breakers.dropped_requests(CLUSTER), 0);
        assert_eq!(second_breakers.dropped_requests(CLUSTER), 1);

        drop(first);
        let second = second_breakers.acquire(CLUSTER).unwrap();
        drop(second);
        drop(first_breakers);
        drop(second_breakers);
        assert_eq!(counters.counter_count(), 0);
    }

    #[test]
    fn default_limit_rejects_the_1025th_request() {
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        let breaker = breakers.cluster_breaker(CLUSTER);
        let permits: Vec<_> = (0..DEFAULT_MAX_REQUESTS)
            .map(|_| breaker.acquire().unwrap())
            .collect();

        assert!(breaker.acquire().is_err());
        assert_eq!(breakers.dropped_requests(CLUSTER), 1);

        drop(permits);
        assert_eq!(breakers.counter_count(), 0);
    }

    #[tokio::test]
    async fn endpoint_waiting_for_ready_does_not_hold_permit() {
        let breakers = configured_breakers(1);
        let service = BackpressuredService {
            ready_budget: Arc::new(AtomicU32::new(0)),
            calls: Arc::new(AtomicU32::new(0)),
        };
        let mut service =
            CircuitBreakingEndpointService::new(service, breakers.cluster_breaker(CLUSTER));

        let early =
            tokio::time::timeout(tokio::time::Duration::from_millis(20), service.ready()).await;

        assert!(
            early.is_err(),
            "endpoint wrapper should wait for inner endpoint readiness",
        );
        assert_eq!(breakers.in_flight(CLUSTER), 0);
    }

    #[tokio::test]
    async fn endpoint_limit_responses_are_not_retried() {
        use crate::client::retry::{GrpcRetryClassifier, RetryConfig};

        let breakers = configured_breakers(0);
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();
        let service = service_fn(
            move |_request: Request<shared_http_body::SharedBody<TonicBody>>| {
                call_counter.fetch_add(1, Ordering::SeqCst);
                async { Ok::<_, BoxError>(Response::new(TonicBody::new(PendingBody))) }
            },
        );
        let service =
            CircuitBreakingEndpointService::new(service, breakers.cluster_breaker(CLUSTER));
        let retry_policy = RetryPolicy::new(
            RetryConfig::new().num_retries(1),
            Arc::new(GrpcRetryClassifier::new(vec![tonic::Code::Unavailable])),
        );
        let mut service = tower::ServiceBuilder::new()
            .layer(RetryLayer::new(retry_policy.into_shared()))
            .service(service);

        let response = service
            .ready()
            .await
            .unwrap()
            .call(Request::new(TonicBody::empty()))
            .await
            .unwrap();
        let status = tonic::Status::from_header_map(response.headers()).unwrap();

        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(breakers.dropped_requests(CLUSTER), 1);
    }

    #[test]
    fn eds_service_name_change_uses_independent_counter() {
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        breakers.set_cluster_config(CLUSTER, "eds-a", CircuitBreakingConfig { max_requests: 1 });
        let first = breakers.acquire(CLUSTER).unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        breakers.set_cluster_config(CLUSTER, "eds-b", CircuitBreakingConfig { max_requests: 1 });
        assert_eq!(breakers.in_flight(CLUSTER), 0);
        let second = breakers.acquire(CLUSTER).unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);
        assert!(breakers.acquire(CLUSTER).is_err());

        drop(second);
        assert_eq!(breakers.in_flight(CLUSTER), 0);
        drop(first);
        assert_eq!(breakers.counter_count(), 1);
    }

    #[test]
    fn idle_eds_service_name_change_cleans_up_previous_counter() {
        let counters = ClusterRequestCounters::isolated();
        let breakers = ClusterCircuitBreakerRegistry::with_counters(counters.clone());
        breakers.set_cluster_config(CLUSTER, "eds-a", CircuitBreakingConfig { max_requests: 1 });
        assert_eq!(counters.counter_count(), 1);

        breakers.set_cluster_config(CLUSTER, "eds-b", CircuitBreakingConfig { max_requests: 1 });
        assert_eq!(counters.counter_count(), 1);

        drop(breakers);
        assert_eq!(counters.counter_count(), 0);
    }

    #[test]
    fn cluster_removal_cleans_up_counter_after_in_flight_requests_finish() {
        let breakers = configured_breakers(1);
        let permit = breakers.acquire(CLUSTER).unwrap();
        assert_eq!(breakers.counter_count(), 1);

        breakers.clear_cluster_config(CLUSTER);
        assert_eq!(breakers.counter_count(), 1);

        drop(permit);
        assert_eq!(breakers.counter_count(), 0);
    }

    #[tokio::test]
    async fn cached_breaker_observes_cluster_removal_and_recreation() {
        let cache = Arc::new(XdsCache::new());
        cache.update_cluster(CLUSTER, cluster_resource(1));
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        let breaker = breakers.cluster_breaker_with_cache(CLUSTER, cache.clone());

        let first = breaker.acquire_when_ready().await.unwrap();
        drop(first);

        let generation = breaker
            .state
            .active_watch_generation
            .load(Ordering::Acquire);
        cache.remove_cluster(CLUSTER);
        tokio::time::timeout(Duration::from_secs(1), async {
            while breaker
                .state
                .finished_watch_generation
                .load(Ordering::Acquire)
                != generation
            {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();

        cache.update_cluster(CLUSTER, cluster_resource(1));
        let second = tokio::time::timeout(Duration::from_secs(1), breaker.acquire_when_ready())
            .await
            .expect("re-added cluster should wake the cached breaker")
            .expect("re-added cluster should be available");
        drop(second);
    }

    #[tokio::test]
    async fn removal_before_watch_task_runs_wakes_waiting_request() {
        let cache = Arc::new(XdsCache::new());
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        let breaker = breakers.cluster_breaker_with_cache(CLUSTER, cache.clone());

        cache.remove_cluster(CLUSTER);
        let response = tokio::time::timeout(Duration::from_secs(1), breaker.acquire_when_ready())
            .await
            .expect("cluster removal should wake the request")
            .expect_err("removed cluster should be unavailable");
        let status = tonic::Status::from_header_map(response.headers()).unwrap();
        assert_eq!(status.code(), Code::Unavailable);
        assert!(is_local_pre_endpoint_response(&response));
        assert!(!is_local_circuit_breaker_drop(&response));
    }

    #[tokio::test]
    async fn waiting_request_does_not_cross_watch_generations() {
        let cache = Arc::new(XdsCache::new());
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        let breaker = breakers.cluster_breaker_with_cache(CLUSTER, cache.clone());
        let old_generation = breaker
            .state
            .active_watch_generation
            .load(Ordering::Acquire);
        let mut old_request = Box::pin(breaker.acquire_when_ready());

        std::future::poll_fn(|cx| {
            assert!(matches!(old_request.as_mut().poll(cx), Poll::Pending));
            Poll::Ready(())
        })
        .await;

        cache.remove_cluster(CLUSTER);
        cache.update_cluster(CLUSTER, cluster_resource(1));
        tokio::time::timeout(Duration::from_secs(1), async {
            while breaker
                .state
                .finished_watch_generation
                .load(Ordering::Acquire)
                != old_generation
            {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();

        let new_permit = breaker.acquire_when_ready().await.unwrap();
        drop(new_permit);

        let response = old_request
            .await
            .expect_err("old request must not consume the re-added cluster config");
        assert_eq!(
            tonic::Status::from_header_map(response.headers())
                .unwrap()
                .code(),
            Code::Unavailable,
        );
    }

    #[tokio::test]
    async fn dropping_cache_backed_breaker_releases_watcher_owners() {
        let cache = Arc::new(XdsCache::new());
        cache.update_cluster(CLUSTER, cluster_resource(1));
        let breakers = ClusterCircuitBreakerRegistry::new_for_test();
        let breaker = breakers.cluster_breaker_with_cache(CLUSTER, cache.clone());
        let permit = breaker.acquire_when_ready().await.unwrap();
        drop(permit);

        let weak_cache = Arc::downgrade(&cache);
        let weak_registry = Arc::downgrade(&breakers.inner);
        drop(breaker);
        drop(breakers);
        drop(cache);
        tokio::task::yield_now().await;

        assert!(weak_cache.upgrade().is_none());
        assert!(weak_registry.upgrade().is_none());
    }

    #[tokio::test]
    async fn endpoint_holds_positive_limit_until_response_body_drops() {
        let breakers = configured_breakers(1);
        let calls = Arc::new(AtomicU32::new(0));
        let call_counter = calls.clone();
        let service = service_fn(move |_request: Request<TonicBody>| {
            call_counter.fetch_add(1, Ordering::SeqCst);
            async { Ok::<_, BoxError>(Response::new(TonicBody::new(PendingBody))) }
        });
        let mut service =
            CircuitBreakingEndpointService::new(service, breakers.cluster_breaker(CLUSTER));

        let first = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);

        let second = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        let status = tonic::Status::from_header_map(second.headers()).unwrap();
        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        drop(first);
        assert_eq!(breakers.in_flight(CLUSTER), 0);

        let third = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        drop(third);
    }

    #[test]
    fn dropping_breakers_releases_config_counter_ref() {
        let counters = ClusterRequestCounters::isolated();
        let breakers = ClusterCircuitBreakerRegistry::with_counters(counters.clone());
        breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests: 1 });
        let permit = breakers.acquire(CLUSTER).unwrap();
        drop(permit);
        assert_eq!(counters.counter_count(), 1);

        drop(breakers);

        assert_eq!(counters.counter_count(), 0);
    }

    #[test]
    fn cleanup_keeps_counter_with_outstanding_clone() {
        let counters = ClusterRequestCounters::isolated();
        let counter_key = CounterKey::new(CLUSTER, CLUSTER);
        let counter = counters.counter(&counter_key);

        counters.cleanup_if_unused(&counter_key);
        assert_eq!(counters.counter_count(), 1);

        drop(counter);
        counters.cleanup_if_unused(&counter_key);
        assert_eq!(counters.counter_count(), 0);
    }

    #[test]
    fn structured_counter_keys_do_not_collide_on_embedded_delimiters() {
        let counters = ClusterRequestCounters::isolated();
        let first_key = CounterKey::new("cluster\0eds", "service");
        let second_key = CounterKey::new("cluster", "eds\0service");

        let first = counters.counter(&first_key);
        let second = counters.counter(&second_key);

        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(counters.counter_count(), 2);
    }

    #[test]
    fn cached_cluster_breaker_does_not_pin_default_counter() {
        let counters = ClusterRequestCounters::isolated();
        let breakers = ClusterCircuitBreakerRegistry::with_counters(counters.clone());
        let breaker = breakers.cluster_breaker(CLUSTER);
        let permit = breaker.acquire().unwrap();
        assert_eq!(counters.counter_count(), 1);

        drop(permit);
        assert_eq!(counters.counter_count(), 0);
        assert_eq!(breaker.cluster.as_ref(), CLUSTER);
    }

    #[tokio::test]
    async fn waiting_for_inner_readiness_does_not_acquire_permit() {
        let breakers = configured_breakers(1);
        let calls = Arc::new(AtomicU32::new(0));
        let service = BackpressuredService {
            ready_budget: Arc::new(AtomicU32::new(0)),
            calls: calls.clone(),
        };
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let mut ready = Box::pin(service.ready());
        std::future::poll_fn(|cx| match ready.as_mut().poll(cx) {
            Poll::Pending => Poll::Ready(()),
            Poll::Ready(_) => panic!("inner service should remain backpressured"),
        })
        .await;

        assert_eq!(breakers.in_flight(CLUSTER), 0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[derive(Clone, Debug)]
    struct BackpressuredService {
        ready_budget: Arc<AtomicU32>,
        calls: Arc<AtomicU32>,
    }

    impl Service<Request<TonicBody>> for BackpressuredService {
        type Response = Response<TonicBody>;
        type Error = BoxError;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            if self
                .ready_budget
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |remaining| {
                    remaining.checked_sub(1)
                })
                .is_ok()
            {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        }

        fn call(&mut self, _request: Request<TonicBody>) -> Self::Future {
            self.calls.fetch_add(1, Ordering::SeqCst);
            std::future::ready(Ok(Response::new(TonicBody::new(PendingBody))))
        }
    }

    #[derive(Debug)]
    struct PendingBody;

    impl Body for PendingBody {
        type Data = Bytes;
        type Error = Infallible;

        fn poll_frame(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            Poll::Pending
        }
    }

    #[derive(Debug)]
    struct ErrorBody {
        emitted: bool,
    }

    impl Body for ErrorBody {
        type Data = Bytes;
        type Error = tonic::Status;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            if self.emitted {
                Poll::Ready(None)
            } else {
                self.emitted = true;
                Poll::Ready(Some(Err(tonic::Status::internal("body failed"))))
            }
        }
    }

    #[derive(Debug)]
    enum BodyState {
        Data,
        Trailers,
        Done,
    }

    #[derive(Debug)]
    struct DataThenTrailersBody {
        state: BodyState,
    }

    impl Body for DataThenTrailersBody {
        type Data = Bytes;
        type Error = Infallible;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            match self.state {
                BodyState::Data => {
                    self.state = BodyState::Trailers;
                    Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(b"hello")))))
                }
                BodyState::Trailers => {
                    self.state = BodyState::Done;
                    Poll::Ready(Some(Ok(Frame::trailers(HeaderMap::new()))))
                }
                BodyState::Done => Poll::Ready(None),
            }
        }
    }
}
