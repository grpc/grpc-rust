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

//! Transport-agnostic retry utilities.
//!
//! The retry *decision* state (attempt cap, backoff, body cloning) lives in the
//! generic [`RetryPolicy`], while transport-specific decisions (which responses
//! are retryable, and any per-retry request mutation) live behind the
//! [`RetryClassifier`] seam. [`GrpcRetryClassifier`] is the default gRPC
//! implementation; [`GrpcRetryPolicy`] is the gRPC policy alias.

use std::fmt::Debug;
use std::io;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use backoff::ExponentialBackoffBuilder;
use backoff::backoff::Backoff;
use http::{Request, Response};
use http_body::Body;
use shared_http_body::{SharedBody, SharedBodyExt};
use tower::retry::Policy;
use tower::retry::Retry;
use tower::{Layer, Service};

use crate::client::circuit_breaking::is_local_circuit_breaker_drop;
use crate::client::route::RouteDecision;
use crate::xds::resource::route_config::RouteRetryConfig;

/// Check if an error's source chain contains a retryable connection-level error.
///
/// These are errors where the request was definitely **not** sent, making it safe to retry.
/// Walks the full error source chain via [`std::error::Error::source`].
pub(crate) fn is_retryable_connection_error(err: &(dyn std::error::Error + 'static)) -> bool {
    let mut current: Option<&(dyn std::error::Error + 'static)> = Some(err);
    while let Some(e) = current {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            match io_err.kind() {
                io::ErrorKind::ConnectionRefused
                | io::ErrorKind::NotConnected
                | io::ErrorKind::AddrInUse
                | io::ErrorKind::AddrNotAvailable => return true,
                _ => {}
            }
        }
        current = e.source();
    }
    false
}

/// Check if a gRPC status code is retryable according to the given policy.
pub(crate) fn is_retryable_grpc_status_code(
    code: tonic::Code,
    retryable_codes: &[tonic::Code],
) -> bool {
    code != tonic::Code::Ok && retryable_codes.contains(&code)
}

/// Transport-specific retry decisions. [`RetryPolicy`] owns everything else
/// (attempt cap, backoff, body cloning), so a classifier only decides *whether*
/// a response is retryable and optionally mutates the request before each retry.
///
/// This is the seam that lets non-gRPC transports (e.g. plain HTTP) reuse the
/// shared retry engine by supplying their own retryable-status logic without
/// duplicating any retry state machine.
pub(crate) trait RetryClassifier: Clone {
    /// Whether the request should be retried, given either the transport response
    /// or a connection-level error. Implementations typically retry on a retryable
    /// connection error (see [`is_retryable_connection_error`]) or a retryable
    /// transport status.
    fn is_retryable<Res>(&self, res: &Result<http::Response<Res>, tower::BoxError>) -> bool;

    /// Optional per-retry request mutation (e.g. stamping a retry-attempt header),
    /// called with the 1-based attempt number just before the retry is issued.
    /// Default: no-op.
    fn prepare_retry<Req>(&self, _req: &mut http::Request<Req>, _attempt: u32) {}
}

/// Maximum number of retry attempts allowed by the gRPC retry spec.
/// Any `num_retries` value that would result in more than 5 total attempts
/// is capped to `MAX_ATTEMPTS - 1 = 4`.
const MAX_ATTEMPTS: u32 = 5;

/// Minimum floor for backoff durations. Values below this are clamped up.
const MIN_BACKOFF: Duration = Duration::from_millis(1);

/// Backoff configuration for retries.
///
/// Build via [`RetryBackoffConfig::new`], which requires `base_interval`.
/// `max_interval` and `backoff_multiplier` are optional with sensible defaults.
///
/// # Guardrails
/// - `base_interval` and `max_interval` must be > 0; values < 1ms are treated as 1ms.
/// - `max_interval` defaults to `10 * base_interval`.
/// - `max_interval` must be >= `base_interval`; if not, it is clamped to `base_interval`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RetryBackoffConfig {
    pub(crate) base_interval: Duration,
    pub(crate) max_interval: Duration,
    pub(crate) backoff_multiplier: f64,
}

impl RetryBackoffConfig {
    /// Create a new backoff config with the given `base_interval`.
    /// `max_interval` defaults to `10 * base_interval`.
    /// `backoff_multiplier` defaults to `2.0`.
    pub(crate) fn new(base_interval: Duration) -> Self {
        let base_interval = base_interval.max(MIN_BACKOFF);
        Self {
            // `checked_mul` guards the (already range-checked) `base_interval`
            // against overflow; the fallback keeps `max_interval >= base`.
            max_interval: base_interval.checked_mul(10).unwrap_or(base_interval),
            base_interval,
            backoff_multiplier: 2.0,
        }
    }

    /// Set the maximum backoff interval.
    /// Values < 1ms are treated as 1ms. Values < `base_interval` are clamped to `base_interval`.
    pub(crate) fn max_interval(mut self, max_interval: Duration) -> Self {
        let max_interval = max_interval.max(MIN_BACKOFF);
        self.max_interval = max_interval.max(self.base_interval);
        self
    }

    /// Set the backoff multiplier (default: 2.0).
    pub(crate) fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }
}

impl Default for RetryBackoffConfig {
    fn default() -> Self {
        Self::new(Duration::from_millis(25)).max_interval(Duration::from_millis(250))
    }
}

/// Transport-agnostic retry knobs shared by every [`RetryClassifier`].
///
/// Built via [`RetryConfig::new`] with defaults, then customized via builder methods.
///
/// # Defaults
/// - `num_retries`: 1 (2 total attempts)
/// - `retry_backoff`: base_interval=25ms, max_interval=250ms, multiplier=2.0
///
/// # Guardrails
/// - `num_retries` must be >= 1. Values of 0 are clamped to 1.
/// - `num_retries` is capped so total attempts (num_retries + 1) never exceed 5.
#[derive(Debug, Clone)]
pub(crate) struct RetryConfig {
    pub(crate) num_retries: u32,
    pub(crate) retry_backoff: RetryBackoffConfig,
}

impl RetryConfig {
    /// Create a new retry config with defaults.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the number of retries (total attempts = num_retries + 1).
    /// Values of 0 are clamped to 1. Values that would exceed 5 total attempts are capped.
    pub(crate) fn num_retries(mut self, num_retries: u32) -> Self {
        // Safety: clamp panics if min > max. Here min=1, max=MAX_ATTEMPTS-1=4 (const).
        self.num_retries = num_retries.clamp(1, MAX_ATTEMPTS - 1);
        self
    }

    /// Set the backoff configuration.
    pub(crate) fn retry_backoff(mut self, backoff: RetryBackoffConfig) -> Self {
        self.retry_backoff = backoff;
        self
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            num_retries: 1,
            retry_backoff: RetryBackoffConfig::default(),
        }
    }
}

/// Default gRPC [`RetryClassifier`]: retries on a retryable connection error or a
/// retryable gRPC status code, and stamps the `grpc-previous-rpc-attempts` header
/// on each retry per the gRPC spec.
#[derive(Debug, Clone)]
pub(crate) struct GrpcRetryClassifier {
    /// gRPC status codes that should be retried.
    retry_on: Arc<[tonic::Code]>,
}

impl GrpcRetryClassifier {
    /// Create a classifier that retries the given gRPC status codes.
    pub(crate) fn new(retry_on: Vec<tonic::Code>) -> Self {
        Self {
            retry_on: retry_on.into(),
        }
    }
}

impl Default for GrpcRetryClassifier {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl RetryClassifier for GrpcRetryClassifier {
    fn is_retryable<Res>(&self, res: &Result<http::Response<Res>, tower::BoxError>) -> bool {
        match res {
            Err(err) => is_retryable_connection_error(err.as_ref()),
            Ok(response) if is_local_circuit_breaker_drop(response) => false,
            Ok(response) => match tonic::Status::from_header_map(response.headers()) {
                Some(status) => is_retryable_grpc_status_code(status.code(), &self.retry_on),
                // No grpc-status header means success.
                None => false,
            },
        }
    }

    fn prepare_retry<Req>(&self, req: &mut http::Request<Req>, attempt: u32) {
        // Per gRPC spec: advertise the number of previous attempts.
        req.headers_mut()
            .insert(GRPC_PREVIOUS_RPC_ATTEMPTS, http::HeaderValue::from(attempt));
    }
}

/// gRPC header for tracking retry attempts per the gRPC spec.
const GRPC_PREVIOUS_RPC_ATTEMPTS: &str = "grpc-previous-rpc-attempts";

/// Create a [`backoff::ExponentialBackoff`] from a [`RetryBackoffConfig`].
fn make_backoff(config: &RetryBackoffConfig) -> backoff::ExponentialBackoff {
    ExponentialBackoffBuilder::default()
        .with_initial_interval(config.base_interval)
        .with_max_interval(config.max_interval)
        .with_multiplier(config.backoff_multiplier)
        .with_randomization_factor(0.2)
        .with_max_elapsed_time(None)
        .build()
}

/// Immutable, shared retry configuration: the transport-agnostic knobs (attempt
/// cap, backoff) plus the transport-specific classifier `C`. Built once when a
/// `RouteConfiguration` is validated (see
/// [`GrpcRetrySharedConfig::from_route_retry`]) and shared across matching
/// requests via an [`Arc`].
///
/// Kept separate from the per-request retry state ([`RetryPolicy`]) so that
/// instantiating a policy is an `Arc` clone plus a zero-field init.
#[derive(Debug)]
pub(crate) struct RetrySharedConfig<C> {
    /// Attempt cap and backoff schedule.
    config: RetryConfig,
    /// Decides retryability and per-retry request mutation for the transport.
    classifier: C,
}

impl<C> RetrySharedConfig<C> {
    /// Create a shared retry config from a [`RetryConfig`] and a classifier.
    pub(crate) fn new(config: RetryConfig, classifier: C) -> Self {
        Self { config, classifier }
    }
}

impl<C: Default> Default for RetrySharedConfig<C> {
    fn default() -> Self {
        Self::new(RetryConfig::default(), C::default())
    }
}

/// Per-request retry *state*: a pointer to the shared, immutable
/// [`RetrySharedConfig`] plus the mutable state for one request (backoff cursor
/// and attempt counter). Implements [`tower::retry::Policy`]; tower's `Retry`
/// clones it per request, so the state is per-request while the config stays
/// shared behind the `Arc`.
#[derive(Clone, Debug)]
pub(crate) struct RetryPolicy<C> {
    /// Immutable config shared across all requests on this route.
    shared: Arc<RetrySharedConfig<C>>,
    /// Backoff state for the current request, created from config on first retry.
    backoff: Option<backoff::ExponentialBackoff>,
    /// Number of retry attempts made so far for the current request.
    attempts: u32,
}

impl<C> RetryPolicy<C> {
    /// Create a policy from a config and classifier, allocating the shared `Arc`.
    /// Prefer [`from_shared`](Self::from_shared) on the hot path.
    pub(crate) fn new(config: RetryConfig, classifier: C) -> Self {
        Self::from_shared(Arc::new(RetrySharedConfig::new(config, classifier)))
    }

    /// Instantiate per-request state from a shared config: a pointer clone plus
    /// a zero-field init.
    pub(crate) fn from_shared(shared: Arc<RetrySharedConfig<C>>) -> Self {
        Self {
            shared,
            backoff: None,
            attempts: 0,
        }
    }

    /// Consume the policy and return its shared config.
    pub(crate) fn into_shared(self) -> Arc<RetrySharedConfig<C>> {
        self.shared
    }

    /// Get or lazily create the backoff and advance it to the next delay. Only
    /// reached when a request is actually being retried.
    fn backoff_next(&mut self) -> Duration {
        let backoff_config = &self.shared.config.retry_backoff;
        let backoff = self
            .backoff
            .get_or_insert_with(|| make_backoff(backoff_config));
        backoff
            .next_backoff()
            .unwrap_or(backoff_config.max_interval)
    }
}

impl<C: Default> Default for RetryPolicy<C> {
    fn default() -> Self {
        Self::from_shared(Arc::new(RetrySharedConfig::default()))
    }
}

impl RetrySharedConfig<GrpcRetryClassifier> {
    /// Build a shared gRPC config from a route's [`RouteRetryConfig`], mapping
    /// `retry_on` to [`tonic::Code`]s; unset Envoy fields use [`RetryConfig`]
    /// defaults. Returns `None` when no condition maps to a gRPC code, so the
    /// route falls back to the layer default instead of masking connection
    /// retries (gRFC A44).
    pub(crate) fn from_route_retry(retry: &RouteRetryConfig) -> Option<Self> {
        let retry_on = grpc_retry_on_codes(&retry.retry_on);
        if retry_on.is_empty() {
            return None;
        }
        let mut config = RetryConfig::new();
        if let Some(num_retries) = retry.num_retries {
            config = config.num_retries(num_retries);
        }
        if let Some(base_interval) = retry.base_interval {
            let mut backoff = RetryBackoffConfig::new(base_interval);
            if let Some(max_interval) = retry.max_interval {
                backoff = backoff.max_interval(max_interval);
            }
            config = config.retry_backoff(backoff);
        }
        Some(Self::new(config, GrpcRetryClassifier::new(retry_on)))
    }
}

/// Map Envoy `retry_on` conditions (comma-separated) to gRPC [`tonic::Code`]s.
///
/// Only the gRPC-status conditions from gRFC A44 are recognized; non-gRPC tokens
/// (e.g. `5xx`, `gateway-error`, `reset`, `connect-failure`) are ignored because
/// connection-level retries are handled separately by
/// [`is_retryable_connection_error`].
pub(crate) fn grpc_retry_on_codes(retry_on: &str) -> Vec<tonic::Code> {
    use tonic::Code;
    retry_on
        .split(',')
        .filter_map(|token| match token.trim() {
            "cancelled" => Some(Code::Cancelled),
            "deadline-exceeded" => Some(Code::DeadlineExceeded),
            "internal" => Some(Code::Internal),
            "resource-exhausted" => Some(Code::ResourceExhausted),
            "unavailable" => Some(Code::Unavailable),
            _ => None,
        })
        .collect()
}

impl<C, Req, Res> Policy<Request<Req>, Response<Res>, tower::BoxError> for RetryPolicy<C>
where
    C: RetryClassifier,
    Req: Clone,
{
    type Future = tokio::time::Sleep;

    fn retry(
        &mut self,
        req: &mut Request<Req>,
        result: &mut Result<Response<Res>, tower::BoxError>,
    ) -> Option<Self::Future> {
        if self.attempts >= self.shared.config.num_retries {
            return None;
        }

        if !self.shared.classifier.is_retryable(result) {
            return None;
        }

        let delay = self.backoff_next();
        self.attempts += 1;

        // Let the classifier stamp any per-retry request state (e.g. gRPC's
        // grpc-previous-rpc-attempts header).
        self.shared.classifier.prepare_retry(req, self.attempts);

        Some(tokio::time::sleep(delay))
    }

    fn clone_request(&mut self, req: &Request<Req>) -> Option<Request<Req>> {
        Some(req.clone())
    }
}

/// Non-breaking alias: existing gRPC callers keep the same name and behavior.
pub(crate) type GrpcRetryPolicy = RetryPolicy<GrpcRetryClassifier>;

/// Shared, immutable gRPC retry config (attempt cap, backoff, retryable
/// [`tonic::Code`] set); the [`GrpcRetryClassifier`] specialization of
/// [`RetrySharedConfig`].
pub(crate) type GrpcRetrySharedConfig = RetrySharedConfig<GrpcRetryClassifier>;

/// Tower [`Layer`] that wraps a gRPC service with retry support.
///
/// Builds a fresh [`tower::retry::Retry`] per request, selecting the config from
/// the matched route's [`RouteDecision`] (stamped by the routing layer just
/// outside). Requests with no [`RouteDecision`] (non-xDS callers) or whose route
/// carries no retry policy use `fallback`.
#[derive(Clone)]
pub(crate) struct RetryLayer {
    /// Config used when a request carries no per-route retry config.
    fallback: Arc<GrpcRetrySharedConfig>,
}

impl RetryLayer {
    /// Create a layer with the given `fallback` config.
    pub(crate) fn new(fallback: Arc<GrpcRetrySharedConfig>) -> Self {
        Self { fallback }
    }
}

impl<S> Layer<S> for RetryLayer {
    type Service = RetryService<S>;

    fn layer(&self, service: S) -> Self::Service {
        RetryService {
            inner: service,
            fallback: Arc::clone(&self.fallback),
        }
    }
}

/// Service that converts request bodies to [`SharedBody`] and retries via
/// [`tower::retry::Retry`], selecting the per-request config from the matched
/// route's [`RouteDecision`] (see [`RetryLayer`]).
#[derive(Clone)]
pub(crate) struct RetryService<S> {
    inner: S,
    /// Config used when a request carries no per-route retry config.
    fallback: Arc<GrpcRetrySharedConfig>,
}

impl<S, B, Res> Service<Request<B>> for RetryService<S>
where
    GrpcRetryPolicy: Policy<Request<SharedBody<B>>, Response<Res>, S::Error>,
    <GrpcRetryPolicy as Policy<Request<SharedBody<B>>, Response<Res>, S::Error>>::Future: Send,
    S: Service<Request<SharedBody<B>>, Response = Response<Res>> + Clone + Send + 'static,
    S::Error: Debug + Send + 'static,
    S::Response: Send + 'static,
    S::Future: Send + 'static,
    B: Body + Unpin + Send + 'static,
    B::Data: Clone + Send + Sync,
    B::Error: Clone + Send + Sync,
    Res: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        // Config the routing layer stamped for this request's route, or the fallback.
        let shared = request
            .extensions()
            .get::<RouteDecision>()
            .and_then(|decision| decision.retry_config.clone())
            .unwrap_or_else(|| Arc::clone(&self.fallback));
        let policy = RetryPolicy::from_shared(shared);
        let mut retry_svc = Retry::new(policy, self.inner.clone());
        let shared_request = request.map(|b| b.into_shared());
        Box::pin(retry_svc.call(shared_request))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_retryable_connection_error tests ---

    #[test]
    fn test_connection_refused_is_retryable() {
        let err = io::Error::new(io::ErrorKind::ConnectionRefused, "refused");
        assert!(is_retryable_connection_error(&err));
    }

    #[test]
    fn test_not_connected_is_retryable() {
        let err = io::Error::new(io::ErrorKind::NotConnected, "not connected");
        assert!(is_retryable_connection_error(&err));
    }

    #[test]
    fn test_addr_in_use_is_retryable() {
        let err = io::Error::new(io::ErrorKind::AddrInUse, "addr in use");
        assert!(is_retryable_connection_error(&err));
    }

    #[test]
    fn test_addr_not_available_is_retryable() {
        let err = io::Error::new(io::ErrorKind::AddrNotAvailable, "addr not available");
        assert!(is_retryable_connection_error(&err));
    }

    #[test]
    fn test_connection_reset_is_not_retryable() {
        // Connection reset means the request may have been sent
        let err = io::Error::new(io::ErrorKind::ConnectionReset, "reset");
        assert!(!is_retryable_connection_error(&err));
    }

    #[test]
    fn test_timeout_is_not_retryable() {
        let err = io::Error::new(io::ErrorKind::TimedOut, "timed out");
        assert!(!is_retryable_connection_error(&err));
    }

    #[test]
    fn test_nested_connection_refused_is_retryable() {
        // tonic::Status wraps the inner error and exposes it via source()
        let inner = io::Error::new(io::ErrorKind::ConnectionRefused, "refused");
        let mut status = tonic::Status::unavailable("connection refused");
        status.set_source(Arc::new(inner));
        assert!(is_retryable_connection_error(&status));
    }

    #[test]
    fn test_non_io_error_is_not_retryable() {
        #[derive(Debug)]
        struct CustomError;
        impl std::fmt::Display for CustomError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "custom")
            }
        }
        impl std::error::Error for CustomError {}

        assert!(!is_retryable_connection_error(&CustomError));
    }

    // --- is_retryable_grpc_status_code tests ---

    #[test]
    fn test_unavailable_is_retryable() {
        let codes = vec![tonic::Code::Unavailable, tonic::Code::Cancelled];
        assert!(is_retryable_grpc_status_code(
            tonic::Code::Unavailable,
            &codes
        ));
    }

    #[test]
    fn test_ok_is_not_retryable() {
        let codes = vec![tonic::Code::Unavailable, tonic::Code::Cancelled];
        assert!(!is_retryable_grpc_status_code(tonic::Code::Ok, &codes));
    }

    #[test]
    fn test_ok_should_not_be_retried() {
        let codes = vec![tonic::Code::Ok];
        assert!(!is_retryable_grpc_status_code(tonic::Code::Ok, &codes))
    }

    #[test]
    fn test_empty_retryable_codes() {
        assert!(!is_retryable_grpc_status_code(
            tonic::Code::Unavailable,
            &[]
        ));
    }

    // --- GrpcRetryClassifier::is_retryable tests ---

    #[test]
    fn test_is_retryable_connection_error_via_result() {
        let classifier = GrpcRetryClassifier::default();
        let err: tower::BoxError =
            Box::new(io::Error::new(io::ErrorKind::ConnectionRefused, "refused"));
        let result: Result<http::Response<()>, tower::BoxError> = Err(err);
        assert!(classifier.is_retryable(&result));
    }

    #[test]
    fn test_is_retryable_grpc_status_via_result() {
        let classifier = GrpcRetryClassifier::new(vec![tonic::Code::Unavailable]);
        let response = http::Response::builder()
            .header("grpc-status", "14") // UNAVAILABLE
            .body(())
            .unwrap();
        let result: Result<http::Response<()>, tower::BoxError> = Ok(response);
        assert!(classifier.is_retryable(&result));
    }

    #[test]
    fn test_is_not_retryable_ok_response() {
        let classifier = GrpcRetryClassifier::new(vec![tonic::Code::Unavailable]);
        let response = http::Response::builder()
            .header("grpc-status", "0") // OK
            .body(())
            .unwrap();
        let result: Result<http::Response<()>, tower::BoxError> = Ok(response);
        assert!(!classifier.is_retryable(&result));
    }

    #[test]
    fn test_is_not_retryable_no_grpc_status_header() {
        let classifier = GrpcRetryClassifier::new(vec![tonic::Code::Unavailable]);
        let response = http::Response::builder().body(()).unwrap();
        let result: Result<http::Response<()>, tower::BoxError> = Ok(response);
        assert!(!classifier.is_retryable(&result));
    }

    // --- RetryBackoffConfig tests ---

    #[test]
    fn test_backoff_defaults() {
        let backoff = RetryBackoffConfig::default();
        assert_eq!(backoff.base_interval, Duration::from_millis(25));
        assert_eq!(backoff.max_interval, Duration::from_millis(250));
        assert_eq!(backoff.backoff_multiplier, 2.0);
    }

    #[test]
    fn test_backoff_new_sets_max_to_10x_base() {
        let backoff = RetryBackoffConfig::new(Duration::from_millis(100));
        assert_eq!(backoff.base_interval, Duration::from_millis(100));
        assert_eq!(backoff.max_interval, Duration::from_millis(1000));
    }

    #[test]
    fn test_backoff_base_interval_below_1ms_clamped() {
        let backoff = RetryBackoffConfig::new(Duration::from_micros(500));
        assert_eq!(backoff.base_interval, Duration::from_millis(1));
        assert_eq!(backoff.max_interval, Duration::from_millis(10));
    }

    #[test]
    fn test_backoff_max_interval_below_1ms_clamped() {
        let backoff = RetryBackoffConfig::new(Duration::from_millis(1))
            .max_interval(Duration::from_micros(100));
        assert_eq!(backoff.max_interval, Duration::from_millis(1));
    }

    #[test]
    fn test_backoff_max_interval_below_base_clamped() {
        let backoff = RetryBackoffConfig::new(Duration::from_millis(100))
            .max_interval(Duration::from_millis(50));
        assert_eq!(backoff.max_interval, Duration::from_millis(100));
    }

    #[test]
    fn test_backoff_custom_multiplier() {
        let backoff = RetryBackoffConfig::new(Duration::from_millis(25)).backoff_multiplier(1.5);
        assert_eq!(backoff.backoff_multiplier, 1.5);
    }

    // --- RetryConfig tests ---

    #[test]
    fn test_policy_defaults() {
        let config = RetryConfig::new();
        assert_eq!(config.num_retries, 1);
        assert_eq!(config.retry_backoff, RetryBackoffConfig::default());
    }

    #[test]
    fn test_policy_num_retries_zero_clamped_to_1() {
        let config = RetryConfig::new().num_retries(0);
        assert_eq!(config.num_retries, 1);
    }

    #[test]
    fn test_policy_num_retries_capped_at_4() {
        // max_attempts=5, so num_retries = max_attempts - 1 = 4
        let config = RetryConfig::new().num_retries(10);
        assert_eq!(config.num_retries, 4);
    }

    #[test]
    fn test_policy_num_retries_4_is_max() {
        let config = RetryConfig::new().num_retries(4);
        assert_eq!(config.num_retries, 4);
    }

    #[test]
    fn test_grpc_classifier_retry_on() {
        let classifier =
            GrpcRetryClassifier::new(vec![tonic::Code::Unavailable, tonic::Code::Cancelled]);
        assert_eq!(
            classifier.retry_on.as_ref(),
            [tonic::Code::Unavailable, tonic::Code::Cancelled]
        );
    }

    #[test]
    fn test_policy_custom_backoff() {
        let backoff = RetryBackoffConfig::new(Duration::from_millis(50))
            .max_interval(Duration::from_millis(500))
            .backoff_multiplier(3.0);
        let config = RetryConfig::new().retry_backoff(backoff.clone());
        assert_eq!(config.retry_backoff, backoff);
    }

    // --- from_route_retry tests ---

    #[test]
    fn test_from_route_retry_maps_fields() {
        let retry = RouteRetryConfig {
            retry_on: "unavailable".into(),
            num_retries: Some(3),
            base_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(1000)),
        };
        let shared = GrpcRetrySharedConfig::from_route_retry(&retry).expect("codes present");
        assert_eq!(shared.config.num_retries, 3);
        assert_eq!(
            shared.config.retry_backoff.base_interval,
            Duration::from_millis(100)
        );
        assert_eq!(
            shared.config.retry_backoff.max_interval,
            Duration::from_millis(1000)
        );
        assert_eq!(
            shared.classifier.retry_on.as_ref(),
            [tonic::Code::Unavailable]
        );
    }

    #[test]
    fn test_from_route_retry_unset_fields_use_defaults() {
        let retry = RouteRetryConfig {
            retry_on: "cancelled".into(),
            num_retries: None,
            base_interval: None,
            max_interval: None,
        };
        let shared = GrpcRetrySharedConfig::from_route_retry(&retry).expect("codes present");
        assert_eq!(shared.config.num_retries, 1);
        assert_eq!(shared.config.retry_backoff, RetryBackoffConfig::default());
        assert_eq!(
            shared.classifier.retry_on.as_ref(),
            [tonic::Code::Cancelled]
        );
    }

    #[test]
    fn test_from_route_retry_empty_codes_yields_none() {
        // `retry_on` with only non-gRPC tokens maps to no gRPC codes, so no
        // policy is produced and connection retries are not masked (gRFC A44).
        let retry = RouteRetryConfig {
            retry_on: "5xx,reset".into(),
            num_retries: Some(3),
            base_interval: None,
            max_interval: None,
        };
        assert!(GrpcRetrySharedConfig::from_route_retry(&retry).is_none());
    }

    // --- from_shared tests ---

    #[test]
    fn from_shared_instantiates_zeroed_state_sharing_config() {
        let shared = Arc::new(
            GrpcRetrySharedConfig::from_route_retry(&RouteRetryConfig {
                retry_on: "unavailable".into(),
                num_retries: Some(2),
                base_interval: None,
                max_interval: None,
            })
            .expect("codes present"),
        );
        let policy = RetryPolicy::from_shared(Arc::clone(&shared));

        assert_eq!(policy.attempts, 0);
        assert!(policy.backoff.is_none());
        assert!(Arc::ptr_eq(&policy.shared, &shared));
        assert_eq!(policy.shared.config.num_retries, 2);

        let policy2 = RetryPolicy::from_shared(Arc::clone(&shared));
        assert!(Arc::ptr_eq(&policy.shared, &policy2.shared));
    }

    /// Verify that two concurrent requests using the same policy get independent
    /// retry state (attempts counter and backoff). Tower's `Retry::call` clones
    /// the policy per request, so mutations from one request must not leak into another.
    #[tokio::test]
    async fn test_retry_state_is_per_request() {
        let policy = GrpcRetryPolicy::new(
            RetryConfig::new().num_retries(2),
            GrpcRetryClassifier::new(vec![tonic::Code::Unavailable]),
        );

        // Simulate two independent request sessions by cloning the policy
        // (this is what tower's Retry::call does per request).
        let mut policy_req1 = policy.clone();
        let mut policy_req2 = policy.clone();

        // Build two independent requests
        let mut req1 = http::Request::builder().body(()).unwrap();
        let mut req2 = http::Request::builder().body(()).unwrap();

        type TestResult = Result<http::Response<()>, tower::BoxError>;

        // Both should be able to clone their requests
        let _ = Policy::<_, http::Response<()>, tower::BoxError>::clone_request(
            &mut policy_req1,
            &req1,
        )
        .expect("clone_request should succeed");
        let _ = Policy::<_, http::Response<()>, tower::BoxError>::clone_request(
            &mut policy_req2,
            &req2,
        )
        .expect("clone_request should succeed");

        // Simulate UNAVAILABLE response for req1, trigger a retry
        let mut result1: TestResult = Ok(http::Response::builder()
            .header("grpc-status", "14")
            .body(())
            .unwrap());
        let retry1 = policy_req1.retry(&mut req1, &mut result1);
        assert!(retry1.is_some(), "req1 should retry on first UNAVAILABLE");

        // req1 has used one retry attempt. req2 should be unaffected — still
        // has all retries available.
        let mut result2: TestResult = Ok(http::Response::builder()
            .header("grpc-status", "14")
            .body(())
            .unwrap());
        let retry2 = policy_req2.retry(&mut req2, &mut result2);
        assert!(retry2.is_some(), "req2 should still be able to retry");

        // Retry req1 again — second retry
        let mut result1b: TestResult = Ok(http::Response::builder()
            .header("grpc-status", "14")
            .body(())
            .unwrap());
        let retry1b = policy_req1.retry(&mut req1, &mut result1b);
        assert!(retry1b.is_some(), "req1 should retry on second UNAVAILABLE");

        // req1 is now exhausted (2 retries used out of 2)
        let mut result1c: TestResult = Ok(http::Response::builder()
            .header("grpc-status", "14")
            .body(())
            .unwrap());
        let retry1c = policy_req1.retry(&mut req1, &mut result1c);
        assert!(retry1c.is_none(), "req1 should be exhausted");

        // req2 should still have its second retry available
        let mut result2b: TestResult = Ok(http::Response::builder()
            .header("grpc-status", "14")
            .body(())
            .unwrap());
        let retry2b = policy_req2.retry(&mut req2, &mut result2b);
        assert!(retry2b.is_some(), "req2 should still have retries left");
    }
}
