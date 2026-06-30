#![cfg_attr(not(test), allow(dead_code))]

use std::fmt;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
};
use std::task::{Context, Poll};

use bytes::Bytes;
use dashmap::DashMap;
use http::{Request, Response};
use http_body::{Body, Frame};
use pin_project_lite::pin_project;
use tokio::sync::watch;
use tonic::body::Body as TonicBody;
use tower::{BoxError, Layer, Service};

use crate::client::route::RouteDecision;
use crate::common::async_util::BoxFuture;
use crate::xds::resource::circuit_breaking::{CircuitBreakingConfig, DEFAULT_MAX_REQUESTS};

static GLOBAL_COUNTERS: OnceLock<Arc<ClusterRequestCounterState>> = OnceLock::new();

/// Shared circuit-breaking state for xDS clusters.
#[derive(Clone, Debug)]
pub(crate) struct ClusterCircuitBreakers {
    inner: Arc<ClusterCircuitBreakersInner>,
}

#[derive(Debug)]
struct ClusterCircuitBreakersInner {
    configs: DashMap<String, Arc<ClusterCircuitBreakerState>>,
    counters: ClusterRequestCounters,
    default_max_requests: u32,
}

impl Drop for ClusterCircuitBreakersInner {
    fn drop(&mut self) {
        for state in self.configs.iter() {
            if let Some(previous) = state.config_tx.send_replace(None) {
                let counter_key = previous.counter_key.clone();
                drop(previous);
                self.counters.deactivate(&counter_key);
            }
        }
    }
}

impl ClusterCircuitBreakers {
    pub(crate) fn new() -> Self {
        Self::with_counters(ClusterRequestCounters::global())
    }

    fn with_counters(counters: ClusterRequestCounters) -> Self {
        Self {
            inner: Arc::new(ClusterCircuitBreakersInner {
                configs: DashMap::new(),
                counters,
                default_max_requests: DEFAULT_MAX_REQUESTS,
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
        let counter_key = counter_key(&cluster, &eds_service_name);
        let counter = self.inner.counters.counter(&counter_key);
        let state = self.ensure_state(&cluster);
        self.update_state_config(
            &state,
            CircuitBreakerRuntimeConfig {
                max_requests: config.max_requests,
                counter_key: Arc::from(counter_key),
                counter,
            },
        );
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

    fn update_state_config(
        &self,
        state: &ClusterCircuitBreakerState,
        config: CircuitBreakerRuntimeConfig,
    ) {
        let previous = state.current_config();
        if previous.as_ref() == Some(&config) {
            return;
        }

        let counter_key_changed = previous
            .as_ref()
            .is_none_or(|previous| previous.counter_key != config.counter_key);
        if counter_key_changed {
            self.inner.counters.activate(&config.counter_key);
        }

        let previous = state.config_tx.send_replace(Some(config));
        if counter_key_changed && let Some(previous) = previous {
            self.deactivate_config(previous);
        }
    }

    fn clear_state(&self, state: &ClusterCircuitBreakerState) {
        if let Some(previous) = state.config_tx.send_replace(None) {
            self.deactivate_config(previous);
        }
    }

    fn deactivate_config(&self, config: CircuitBreakerRuntimeConfig) {
        let counter_key = config.counter_key.clone();
        drop(config);
        self.inner.counters.deactivate(&counter_key);
    }

    fn acquire(&self, cluster: &str) -> Result<CircuitBreakerPermit, CircuitBreakerLimit> {
        self.acquire_with_config(self.runtime_config_or_default(cluster))
    }

    fn acquire_with_config(
        &self,
        runtime_config: CircuitBreakerRuntimeConfig,
    ) -> Result<CircuitBreakerPermit, CircuitBreakerLimit> {
        let limit = CircuitBreakerLimit {
            max_requests: runtime_config.max_requests,
        };
        self.inner
            .counters
            .acquire(
                runtime_config.counter_key,
                runtime_config.counter,
                limit.max_requests,
            )
            .ok_or(limit)
    }

    fn runtime_config_or_default(&self, cluster: &str) -> CircuitBreakerRuntimeConfig {
        self.inner
            .configs
            .get(cluster)
            .and_then(|state| state.current_config())
            .unwrap_or_else(|| {
                let counter_key = counter_key(cluster, cluster);
                let counter = self.inner.counters.counter(&counter_key);
                CircuitBreakerRuntimeConfig {
                    max_requests: self.inner.default_max_requests,
                    counter_key: Arc::from(counter_key),
                    counter,
                }
            })
    }

    #[cfg(test)]
    fn in_flight(&self, cluster: &str) -> u32 {
        let runtime_config = self.runtime_config_or_default(cluster);
        self.inner.counters.in_flight(&runtime_config.counter_key)
    }

    #[cfg(test)]
    fn dropped_requests(&self, cluster: &str) -> u64 {
        let runtime_config = self.runtime_config_or_default(cluster);
        self.inner
            .counters
            .dropped_requests(&runtime_config.counter_key)
    }

    #[cfg(test)]
    fn counter_count(&self) -> usize {
        self.inner.counters.counter_count()
    }

    #[cfg(test)]
    fn clear_cluster_config(&self, cluster: &str) {
        if let Some((_, state)) = self.inner.configs.remove(cluster) {
            self.clear_state(&state);
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self::with_counters(ClusterRequestCounters::isolated())
    }
}

impl Default for ClusterCircuitBreakers {
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
    counter_key: Arc<str>,
    counter: Arc<InFlightCounter>,
}

impl PartialEq for CircuitBreakerRuntimeConfig {
    fn eq(&self, other: &Self) -> bool {
        self.max_requests == other.max_requests && self.counter_key == other.counter_key
    }
}

impl Eq for CircuitBreakerRuntimeConfig {}

fn counter_key(cluster: &str, eds_service_name: &str) -> String {
    format!("{cluster}\0{eds_service_name}")
}

#[derive(Debug)]
struct ClusterCircuitBreakerState {
    config_tx: watch::Sender<Option<CircuitBreakerRuntimeConfig>>,
}

impl ClusterCircuitBreakerState {
    fn new() -> Self {
        let (config_tx, _) = watch::channel(None);
        Self { config_tx }
    }

    fn current_config(&self) -> Option<CircuitBreakerRuntimeConfig> {
        self.config_tx.borrow().clone()
    }
}

#[derive(Clone, Debug)]
struct ClusterRequestCounters {
    inner: Arc<ClusterRequestCounterState>,
}

#[derive(Debug, Default)]
struct ClusterRequestCounterState {
    counters: DashMap<String, Arc<InFlightCounter>>,
    active_refs: DashMap<String, Arc<AtomicUsize>>,
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

    fn activate(&self, counter_key: &str) {
        self.inner
            .active_refs
            .entry(counter_key.to_string())
            .or_insert_with(|| Arc::new(AtomicUsize::new(0)))
            .fetch_add(1, Ordering::AcqRel);
    }

    fn deactivate(&self, counter_key: &str) {
        let should_cleanup = self
            .inner
            .active_refs
            .get(counter_key)
            .and_then(|active_refs| {
                active_refs
                    .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                        count.checked_sub(1)
                    })
                    .ok()
            })
            .is_some_and(|previous| previous <= 1);

        if should_cleanup {
            self.cleanup_if_unused(counter_key);
        }
    }

    fn acquire(
        &self,
        counter_key: Arc<str>,
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
            counter.record_drop();
            self.cleanup_if_unused(&counter_key);
            None
        }
    }

    fn counter(&self, counter_key: &str) -> Arc<InFlightCounter> {
        self.inner
            .counters
            .entry(counter_key.to_string())
            .or_insert_with(|| Arc::new(InFlightCounter::default()))
            .clone()
    }

    fn cleanup_if_unused(&self, counter_key: &str) {
        let active_refs = self.active_refs(counter_key);
        if active_refs != 0 {
            return;
        }

        self.inner.counters.remove_if(counter_key, |_, counter| {
            counter.in_flight() == 0 && Arc::strong_count(counter) == 1
        });
        self.inner
            .active_refs
            .remove_if(counter_key, |_, refs| refs.load(Ordering::Acquire) == 0);
    }

    fn active_refs(&self, counter_key: &str) -> usize {
        self.inner
            .active_refs
            .get(counter_key)
            .map(|refs| refs.load(Ordering::Acquire))
            .unwrap_or(0)
    }

    #[cfg(test)]
    fn in_flight(&self, counter_key: &str) -> u32 {
        self.inner
            .counters
            .get(counter_key)
            .map(|counter| counter.in_flight())
            .unwrap_or(0)
    }

    #[cfg(test)]
    fn dropped_requests(&self, counter_key: &str) -> u64 {
        self.inner
            .counters
            .get(counter_key)
            .map(|counter| counter.dropped_requests())
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
    /// Local A32 drop accounting, kept with the global counter so future LRS
    /// support can export `total_dropped_requests` without changing enforcement.
    dropped_requests: AtomicU64,
}

impl InFlightCounter {
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

    fn record_drop(&self) {
        self.dropped_requests.fetch_add(1, Ordering::AcqRel);
    }

    #[cfg(test)]
    fn dropped_requests(&self) -> u64 {
        self.dropped_requests.load(Ordering::Acquire)
    }
}

#[derive(Debug)]
struct CircuitBreakerPermit {
    counter: Option<Arc<InFlightCounter>>,
    counter_key: Arc<str>,
    counters: ClusterRequestCounters,
}

impl Drop for CircuitBreakerPermit {
    fn drop(&mut self) {
        if let Some(counter) = self.counter.take() {
            counter.in_flight.fetch_sub(1, Ordering::AcqRel);
        }
        self.counters.cleanup_if_unused(&self.counter_key);
    }
}

/// Tower layer that enforces A32 max in-flight requests per xDS cluster.
#[derive(Clone)]
pub(crate) struct CircuitBreakingLayer {
    circuit_breakers: ClusterCircuitBreakers,
}

impl CircuitBreakingLayer {
    pub(crate) fn new(circuit_breakers: ClusterCircuitBreakers) -> Self {
        Self { circuit_breakers }
    }
}

impl fmt::Debug for CircuitBreakingLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingLayer")
            .field("circuit_breakers", &self.circuit_breakers)
            .finish()
    }
}

impl<S> Layer<S> for CircuitBreakingLayer {
    type Service = CircuitBreakingService<S>;

    fn layer(&self, service: S) -> Self::Service {
        CircuitBreakingService {
            inner: service,
            circuit_breakers: self.circuit_breakers.clone(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct CircuitBreakingService<S> {
    inner: S,
    circuit_breakers: ClusterCircuitBreakers,
}

impl<S: fmt::Debug> fmt::Debug for CircuitBreakingService<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakingService")
            .field("inner", &self.inner)
            .field("circuit_breakers", &self.circuit_breakers)
            .finish()
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

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let Some(route_decision) = request.extensions().get::<RouteDecision>().cloned() else {
            return Box::pin(async {
                Ok(status_response(tonic::Status::internal(
                    CircuitBreakingError::NoRoutingDecision.to_string(),
                )))
            });
        };

        let cluster = route_decision.cluster;
        let circuit_breakers = self.circuit_breakers.clone();
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let permit = match circuit_breakers.acquire(&cluster) {
                Ok(permit) => permit,
                Err(limit) => return Ok(limit_exceeded_response(&cluster, limit)),
            };

            std::future::poll_fn(|cx| inner.poll_ready(cx))
                .await
                .map_err(Into::into)?;
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

fn limit_exceeded_response(cluster: &str, limit: CircuitBreakerLimit) -> Response<TonicBody> {
    status_response(tonic::Status::unavailable(format!(
        "circuit breaker open for cluster '{cluster}': max_requests limit {} reached",
        limit.max_requests,
    )))
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

    use bytes::Bytes;
    use http::{HeaderMap, Request, Response};
    use http_body::{Body, Frame};
    use tonic::Code;
    use tower::Layer;
    use tower::ServiceExt;
    use tower::retry::Policy;
    use tower::service_fn;

    use crate::client::retry::RetryLayer;

    use super::*;

    const CLUSTER: &str = "cluster-a";

    fn request() -> Request<TonicBody> {
        let mut request = Request::new(TonicBody::empty());
        request.extensions_mut().insert(RouteDecision {
            cluster: CLUSTER.to_string(),
            request_hash: None,
        });
        request
    }

    fn configured_breakers(max_requests: u32) -> ClusterCircuitBreakers {
        let breakers = ClusterCircuitBreakers::new_for_test();
        breakers.set_config(CLUSTER, CircuitBreakingConfig { max_requests });
        breakers
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
            CircuitBreakingLayer::new(ClusterCircuitBreakers::new_for_test()).layer(service);

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
    async fn limit_responses_do_not_enter_retry_policy() {
        let breakers = configured_breakers(1);
        let retry_observations = Arc::new(AtomicU32::new(0));
        let policy = CountingUnavailablePolicy {
            retry_observations: retry_observations.clone(),
        };

        let service = service_fn(
            |_request: Request<shared_http_body::SharedBody<TonicBody>>| async {
                Ok::<_, BoxError>(Response::new(TonicBody::new(PendingBody)))
            },
        );
        let mut service = tower::ServiceBuilder::new()
            .layer(CircuitBreakingLayer::new(breakers))
            .layer(RetryLayer::new(policy))
            .service(service);

        let _first = service
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
        let status = tonic::Status::from_header_map(second.headers()).unwrap();

        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(retry_observations.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn eds_service_name_change_uses_independent_counter() {
        let breakers = ClusterCircuitBreakers::new_for_test();
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
    fn cluster_removal_cleans_up_counter_after_in_flight_requests_finish() {
        let breakers = configured_breakers(1);
        let permit = breakers.acquire(CLUSTER).unwrap();
        assert_eq!(breakers.counter_count(), 1);

        breakers.clear_cluster_config(CLUSTER);
        assert_eq!(breakers.counter_count(), 1);

        drop(permit);
        assert_eq!(breakers.counter_count(), 0);
    }

    #[test]
    fn dropping_breakers_releases_config_counter_ref() {
        let counters = ClusterRequestCounters::isolated();
        let breakers = ClusterCircuitBreakers::with_counters(counters.clone());
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
        let counter_key = counter_key(CLUSTER, CLUSTER);
        let counter = counters.counter(&counter_key);

        counters.cleanup_if_unused(&counter_key);
        assert_eq!(counters.counter_count(), 1);

        drop(counter);
        counters.cleanup_if_unused(&counter_key);
        assert_eq!(counters.counter_count(), 0);
    }

    #[tokio::test]
    async fn rejects_over_limit_without_waiting_for_inner_ready() {
        let breakers = configured_breakers(1);
        let calls = Arc::new(AtomicU32::new(0));
        let service = BackpressuredService {
            ready_budget: Arc::new(AtomicU32::new(1)),
            calls: calls.clone(),
        };
        let mut service = CircuitBreakingLayer::new(breakers.clone()).layer(service);

        let first = service
            .ready()
            .await
            .unwrap()
            .call(request())
            .await
            .unwrap();
        assert_eq!(breakers.in_flight(CLUSTER), 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let second = tokio::time::timeout(
            tokio::time::Duration::from_millis(50),
            service.ready().await.unwrap().call(request()),
        )
        .await
        .expect("over-limit request should not wait for inner readiness")
        .unwrap();
        let status = tonic::Status::from_header_map(second.headers()).unwrap();
        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        drop(first);
    }

    #[derive(Clone, Debug)]
    struct CountingUnavailablePolicy {
        retry_observations: Arc<AtomicU32>,
    }

    impl Policy<Request<shared_http_body::SharedBody<TonicBody>>, Response<TonicBody>, BoxError>
        for CountingUnavailablePolicy
    {
        type Future = std::future::Ready<()>;

        fn retry(
            &mut self,
            _req: &mut Request<shared_http_body::SharedBody<TonicBody>>,
            result: &mut Result<Response<TonicBody>, BoxError>,
        ) -> Option<Self::Future> {
            if let Ok(response) = result
                && tonic::Status::from_header_map(response.headers())
                    .is_some_and(|status| status.code() == Code::Unavailable)
            {
                self.retry_observations.fetch_add(1, Ordering::SeqCst);
            }
            None
        }

        fn clone_request(
            &mut self,
            req: &Request<shared_http_body::SharedBody<TonicBody>>,
        ) -> Option<Request<shared_http_body::SharedBody<TonicBody>>> {
            Some(req.clone())
        }
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
