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

use arc_swap::ArcSwap;
use backoff::ExponentialBackoffBuilder;
use backoff::backoff::Backoff;
use http::{Request, Response};
use http_body::Body;
use shared_http_body::{SharedBody, SharedBodyExt};
use tower::retry::Policy;
use tower::retry::Retry;
use tower::{Layer, Service};

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
            max_interval: base_interval * 10,
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
#[derive(Debug, Clone, Default)]
pub(crate) struct GrpcRetryClassifier {
    /// gRPC status codes that should be retried.
    pub(crate) retry_on: Vec<tonic::Code>,
}

impl RetryClassifier for GrpcRetryClassifier {
    fn is_retryable<Res>(&self, res: &Result<http::Response<Res>, tower::BoxError>) -> bool {
        match res {
            Err(err) => is_retryable_connection_error(err.as_ref()),
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

/// Retry *decision* state — attempt cap, backoff, and body cloning — shared
/// across transports. Transport-specific decisions live in the classifier `C`
/// (see [`RetryClassifier`]).
///
/// Wraps a [`RetryConfig`] behind an [`ArcSwap`] so that configuration can be
/// atomically updated (e.g. from xDS) without blocking in-flight requests.
///
/// Implements [`tower::retry::Policy`]. Tower's `Retry` service clones the policy
/// for each request, so `backoff` and `attempts` track per-request retry state
/// while the shared config is read from `ArcSwap` on each retry decision. The
/// retry *state machine* stays entirely in tower's `Retry`/`ResponseFuture`;
/// this type only implements the `Policy` trait, so no state machine is
/// reimplemented here.
#[derive(Clone, Debug)]
pub(crate) struct RetryPolicy<C> {
    config: Arc<ArcSwap<RetryConfig>>,
    /// Decides retryability and per-retry request mutation for the transport.
    classifier: C,
    /// Backoff state for the current request, created from config on first retry.
    backoff: Option<backoff::ExponentialBackoff>,
    /// Number of retry attempts made so far for the current request.
    attempts: u32,
}

impl<C> RetryPolicy<C> {
    /// Create a new retry policy with the given configuration and classifier.
    pub(crate) fn new(config: RetryConfig, classifier: C) -> Self {
        Self {
            config: Arc::new(ArcSwap::from(Arc::new(config))),
            classifier,
            backoff: None,
            attempts: 0,
        }
    }

    /// Atomically swap the configuration with a new one.
    pub(crate) fn update_config(&self, config: RetryConfig) {
        self.config.store(Arc::new(config));
    }

    /// Load the current configuration.
    pub(crate) fn load_config(&self) -> Arc<RetryConfig> {
        self.config.load_full()
    }

    /// Get or create the backoff, and advance it to the next delay.
    fn backoff_next(&mut self, backoff_config: &RetryBackoffConfig) -> Duration {
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
        Self::new(RetryConfig::default(), C::default())
    }
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
        let config = self.load_config();

        if self.attempts >= config.num_retries {
            return None;
        }

        if !self.classifier.is_retryable(result) {
            return None;
        }

        let delay = self.backoff_next(&config.retry_backoff);
        self.attempts += 1;

        // Let the classifier stamp any per-retry request state (e.g. gRPC's
        // grpc-previous-rpc-attempts header).
        self.classifier.prepare_retry(req, self.attempts);

        Some(tokio::time::sleep(delay))
    }

    fn clone_request(&mut self, req: &Request<Req>) -> Option<Request<Req>> {
        Some(req.clone())
    }
}

/// Non-breaking alias: existing gRPC callers keep the same name and behavior.
pub(crate) type GrpcRetryPolicy = RetryPolicy<GrpcRetryClassifier>;

/// Tower [`Layer`] that wraps a service with retry support.
///
/// Converts the request body into a [`SharedBody`] (cloneable) and constructs
/// a fresh [`tower::retry::Retry`] service per request so that each request
/// gets its own retry state.
///
/// This layer is generic over the retry policy — it is not tied to gRPC.
/// The gRPC-specific behavior lives in the [`Policy`] implementation
/// (e.g. [`GrpcRetryPolicy`]).
#[derive(Clone)]
pub(crate) struct RetryLayer<P> {
    policy: P,
}

impl<P> RetryLayer<P> {
    /// Create a new retry layer with the given policy.
    pub(crate) fn new(policy: P) -> Self {
        Self { policy }
    }
}

impl<P: Clone, S> Layer<S> for RetryLayer<P> {
    type Service = RetryService<P, S>;

    fn layer(&self, service: S) -> Self::Service {
        RetryService {
            inner: service,
            policy: self.policy.clone(),
        }
    }
}

/// Service that converts request bodies to [`SharedBody`] and retries via
/// [`tower::retry::Retry`] with the given policy.
#[derive(Clone)]
pub(crate) struct RetryService<P, S> {
    inner: S,
    policy: P,
}

impl<P, S, B, Res> Service<Request<B>> for RetryService<P, S>
where
    P: Policy<Request<SharedBody<B>>, Response<Res>, S::Error> + Clone + Send + 'static,
    P::Future: Send,
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
        let mut retry_svc = Retry::new(self.policy.clone(), self.inner.clone());
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
        let classifier = GrpcRetryClassifier {
            retry_on: vec![tonic::Code::Unavailable],
        };
        let response = http::Response::builder()
            .header("grpc-status", "14") // UNAVAILABLE
            .body(())
            .unwrap();
        let result: Result<http::Response<()>, tower::BoxError> = Ok(response);
        assert!(classifier.is_retryable(&result));
    }

    #[test]
    fn test_is_not_retryable_ok_response() {
        let classifier = GrpcRetryClassifier {
            retry_on: vec![tonic::Code::Unavailable],
        };
        let response = http::Response::builder()
            .header("grpc-status", "0") // OK
            .body(())
            .unwrap();
        let result: Result<http::Response<()>, tower::BoxError> = Ok(response);
        assert!(!classifier.is_retryable(&result));
    }

    #[test]
    fn test_is_not_retryable_no_grpc_status_header() {
        let classifier = GrpcRetryClassifier {
            retry_on: vec![tonic::Code::Unavailable],
        };
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
        let classifier = GrpcRetryClassifier {
            retry_on: vec![tonic::Code::Unavailable, tonic::Code::Cancelled],
        };
        assert_eq!(
            classifier.retry_on,
            vec![tonic::Code::Unavailable, tonic::Code::Cancelled]
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

    // --- RetryPolicy (ArcSwap wrapper) tests ---

    #[test]
    fn test_policy_load_config() {
        let policy = GrpcRetryPolicy::new(
            RetryConfig::new().num_retries(1),
            GrpcRetryClassifier {
                retry_on: vec![tonic::Code::Unavailable],
            },
        );
        let loaded = policy.load_config();
        assert_eq!(loaded.num_retries, 1);
    }

    #[test]
    fn test_policy_update_config() {
        let policy = GrpcRetryPolicy::default();
        assert_eq!(policy.load_config().num_retries, 1);

        policy.update_config(RetryConfig::new().num_retries(3));

        let loaded = policy.load_config();
        assert_eq!(loaded.num_retries, 3);
    }

    /// Verify that two concurrent requests using the same policy get independent
    /// retry state (attempts counter and backoff). Tower's `Retry::call` clones
    /// the policy per request, so mutations from one request must not leak into another.
    #[tokio::test]
    async fn test_retry_state_is_per_request() {
        let policy = GrpcRetryPolicy::new(
            RetryConfig::new().num_retries(2),
            GrpcRetryClassifier {
                retry_on: vec![tonic::Code::Unavailable],
            },
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
