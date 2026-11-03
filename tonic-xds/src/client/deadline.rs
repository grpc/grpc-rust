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

//! Per-call deadline enforcement (gRFC A31).
//!
//! The routing layer ([`XdsRoutingService`]) enforces each call's deadline with
//! a [`DeadlineTimer`], so the deadline counts from the start of the call. The
//! timer starts as the call enters, armed with the application's
//! `grpc-timeout`, and so also bounds the wait for the first route config. Once
//! a route is matched, it is tightened to the route's xDS max stream duration
//! ([`RouteDecision::max_stream_duration`]), also counted from the start of the
//! call. The earlier deadline wins and holds once for the whole call: every
//! retry attempt, the backoff between them, and (for gRPC) the response stream.
//! An expired call fails with `DEADLINE_EXCEEDED`.
//!
//! The deadline is also recorded on the request as a [`CallDeadline`] extension,
//! which the retry layer copies onto each attempt, so
//! [`set_grpc_timeout_header`] can advertise the remaining budget upstream.
//!
//! [`XdsRoutingService`]: crate::client::route::XdsRoutingService
//! [`RouteDecision::max_stream_duration`]: crate::client::route::RouteDecision::max_stream_duration

use std::future::{Future, poll_fn};
use std::pin::{Pin, pin};
use std::task::{Context, Poll, ready};
use std::time::Duration;

use bytes::Bytes;
use http::{HeaderValue, Request, Response};
use http_body::{Body, Frame, SizeHint};
use pin_project_lite::pin_project;
use tokio::time::{Instant, Sleep};
use tonic::body::Body as TonicBody;
use tower::BoxError;

use crate::common::grpc_timeout::{GRPC_TIMEOUT_HEADER, encode_grpc_timeout, extract_grpc_timeout};

/// The absolute deadline of a call, stamped on the request by
/// [`DeadlineTimer::apply_route`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CallDeadline(pub(crate) Instant);

/// Extends the call deadline over a response body.
type BodyDeadline<ResBody> = fn(ResBody, Pin<Box<Sleep>>) -> ResBody;

/// How much of the call deadline a channel enforces. The routing layer holds
/// one and starts a [`DeadlineTimer`] from it for each call.
pub(crate) struct DeadlinePolicy<ResBody> {
    /// Whether the application deadline is read from the `grpc-timeout` header.
    app_grpc_timeout: bool,
    /// Extends the deadline over the response body; `None` stops enforcing it
    /// once response headers arrive.
    body_deadline: Option<BodyDeadline<ResBody>>,
}

impl<ResBody> Clone for DeadlinePolicy<ResBody> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<ResBody> Copy for DeadlinePolicy<ResBody> {}

impl DeadlinePolicy<TonicBody> {
    /// The gRPC policy: honors the application's `grpc-timeout` and holds the
    /// deadline until the response stream ends.
    pub(crate) fn grpc() -> Self {
        Self {
            app_grpc_timeout: true,
            body_deadline: Some(grpc_body_deadline),
        }
    }
}

impl<ResBody> DeadlinePolicy<ResBody> {
    /// The policy for a transport-generic channel: enforces only the xDS max
    /// stream duration, and only until response headers arrive, since neither
    /// `grpc-timeout` nor the response body is understood here.
    pub(crate) fn headers_only() -> Self {
        Self {
            app_grpc_timeout: false,
            body_deadline: None,
        }
    }

    /// Starts timing a call that enters now, armed with the application
    /// deadline if this policy honors one.
    pub(crate) fn start<B>(&self, request: &Request<B>) -> DeadlineTimer<ResBody> {
        // A malformed `grpc-timeout` is treated as absent, as tonic does.
        let app = if self.app_grpc_timeout {
            extract_grpc_timeout(request).ok().flatten()
        } else {
            None
        };
        let mut timer = DeadlineTimer {
            start: Instant::now(),
            sleep: None,
            body_deadline: self.body_deadline,
        };
        timer.tighten(app);
        timer
    }
}

/// The deadline of one call, from when it enters the channel until its
/// response headers arrive or, for gRPC, its response stream ends.
pub(crate) struct DeadlineTimer<ResBody> {
    /// When the call started; every timeout counts from here.
    start: Instant,
    /// Fires at the deadline; `None` while the call has none.
    sleep: Option<Pin<Box<Sleep>>>,
    body_deadline: Option<BodyDeadline<ResBody>>,
}

impl<ResBody> DeadlineTimer<ResBody> {
    /// Moves the deadline to `timeout` after the start of the call, if that is
    /// earlier. A timeout too large to represent as an `Instant` is no deadline.
    fn tighten(&mut self, timeout: Option<Duration>) {
        let Some(deadline) = timeout.and_then(|timeout| self.start.checked_add(timeout)) else {
            return;
        };
        match &mut self.sleep {
            Some(sleep) if sleep.deadline() <= deadline => {}
            Some(sleep) => sleep.as_mut().reset(deadline),
            None => self.sleep = Some(Box::pin(tokio::time::sleep_until(deadline))),
        }
    }

    /// Awaits `fut`, failing with `DEADLINE_EXCEEDED` if the deadline passes
    /// first. Bounds the wait for a route config.
    pub(crate) async fn bound<T, E>(
        &mut self,
        fut: impl Future<Output = Result<T, E>>,
    ) -> Result<T, BoxError>
    where
        E: Into<BoxError>,
    {
        let mut fut = pin!(fut);
        poll_fn(|cx| {
            if let Poll::Ready(result) = fut.as_mut().poll(cx) {
                return Poll::Ready(result.map_err(Into::into));
            }
            match self.sleep.as_mut() {
                Some(sleep) => {
                    ready!(sleep.as_mut().poll(cx));
                    Poll::Ready(Err(deadline_exceeded()))
                }
                None => Poll::Pending,
            }
        })
        .await
    }

    /// Applies the matched route's xDS max stream duration, counted from the
    /// start of the call (gRFC A31), and records the resulting deadline on
    /// `request` as a [`CallDeadline`]. Fails with `DEADLINE_EXCEEDED` if the
    /// deadline has already passed, e.g. while waiting for the route config.
    pub(crate) fn apply_route<B>(
        &mut self,
        request: &mut Request<B>,
        max_stream_duration: Option<Duration>,
    ) -> Result<(), BoxError> {
        self.tighten(max_stream_duration);
        let Some(sleep) = &self.sleep else {
            return Ok(());
        };
        let deadline = sleep.deadline();
        if Instant::now() >= deadline {
            return Err(deadline_exceeded());
        }
        request.extensions_mut().insert(CallDeadline(deadline));
        Ok(())
    }

    /// Bounds the routed call `inner` by the deadline and, for gRPC, extends
    /// the deadline over its response stream.
    pub(crate) fn run<F>(self, inner: F) -> ResponseFuture<F, ResBody> {
        ResponseFuture {
            inner,
            sleep: self.sleep,
            body_deadline: self.body_deadline,
        }
    }
}

pin_project! {
    /// A routed call bounded by its deadline; see [`DeadlineTimer::run`].
    pub(crate) struct ResponseFuture<F, ResBody> {
        #[pin]
        inner: F,
        // `None` when the call has no deadline.
        sleep: Option<Pin<Box<Sleep>>>,
        body_deadline: Option<BodyDeadline<ResBody>>,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F, ResBody>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
    E: Into<BoxError>,
{
    type Output = Result<Response<ResBody>, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if let Poll::Ready(result) = this.inner.poll(cx) {
            let sleep = this.sleep.take();
            let expired = matches!(&sleep, Some(s) if Instant::now() >= s.deadline());
            return Poll::Ready(match (result, sleep) {
                // An attempt cut short by the deadline, by the endpoint's own
                // `grpc-timeout` timer (an error) or the server's (a CANCELLED
                // reply), is reported as the deadline it hit. tonic's endpoint
                // stack always runs that timer, but since the advertised
                // timeout is rounded up it fires on this deadline's tick at the
                // earliest. Any other result that is ready is kept, even if
                // the deadline is too.
                (Err(_), _) if expired => Err(deadline_exceeded()),
                (Ok(response), _) if expired && is_cancelled(&response) => Err(deadline_exceeded()),
                (Ok(response), Some(sleep)) => Ok(match *this.body_deadline {
                    Some(body_deadline) => response.map(|body| body_deadline(body, sleep)),
                    None => response,
                }),
                (result, _) => result.map_err(Into::into),
            });
        }
        if let Some(sleep) = this.sleep.as_mut() {
            ready!(sleep.as_mut().poll(cx));
            return Poll::Ready(Err(deadline_exceeded()));
        }
        Poll::Pending
    }
}

fn deadline_exceeded() -> BoxError {
    Box::new(tonic::Status::deadline_exceeded(
        "deadline exceeded (from grpc-timeout or xDS max stream duration)",
    ))
}

/// Whether `response` is a trailers-only gRPC reply with status CANCELLED.
fn is_cancelled<B>(response: &Response<B>) -> bool {
    tonic::Status::from_header_map(response.headers())
        .is_some_and(|status| status.code() == tonic::Code::Cancelled)
}

/// Sets the request's `grpc-timeout` to the time remaining until its
/// [`CallDeadline`], if it has one.
///
/// Called on each attempt, after the retry layer, so every attempt advertises
/// only the budget it has left rather than the original timeout.
pub(crate) fn set_grpc_timeout_header<B>(request: &mut Request<B>) {
    let Some(CallDeadline(deadline)) = request.extensions().get::<CallDeadline>().copied() else {
        return;
    };
    let remaining = deadline.saturating_duration_since(Instant::now());
    if let Ok(value) = HeaderValue::from_str(&encode_grpc_timeout(remaining)) {
        request.headers_mut().insert(GRPC_TIMEOUT_HEADER, value);
    }
}

fn grpc_body_deadline(body: TonicBody, sleep: Pin<Box<Sleep>>) -> TonicBody {
    TonicBody::new(GrpcDeadlineBody {
        inner: Some(body),
        sleep,
    })
}

/// A gRPC response body that fails with `DEADLINE_EXCEEDED` once the call
/// deadline passes, so the deadline holds for the whole stream.
struct GrpcDeadlineBody {
    // Dropped on expiry to cancel the underlying stream.
    inner: Option<TonicBody>,
    sleep: Pin<Box<Sleep>>,
}

impl Body for GrpcDeadlineBody {
    type Data = Bytes;
    type Error = tonic::Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = &mut *self;
        let Some(inner) = this.inner.as_mut() else {
            return Poll::Ready(None);
        };
        // Checked first so a stream that always has data ready still expires.
        if this.sleep.as_mut().poll(cx).is_ready() {
            this.inner = None;
            return Poll::Ready(Some(Err(tonic::Status::deadline_exceeded(
                "deadline exceeded while streaming the response",
            ))));
        }
        let frame = ready!(Pin::new(inner).poll_frame(cx));
        if !matches!(&frame, Some(Ok(frame)) if frame.is_data()) {
            // Trailers, an error, or the end: the stream is over, so later
            // polls must not report the deadline.
            this.inner = None;
        }
        Poll::Ready(frame)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.as_ref().is_none_or(Body::is_end_stream)
    }

    fn size_hint(&self) -> SizeHint {
        self.inner
            .as_ref()
            .map_or_else(SizeHint::default, Body::size_hint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::retry::{
        GrpcRetryClassifier, RetryBackoffConfig, RetryConfig, RetryLayer, RetrySharedConfig,
    };
    use crate::client::route::{
        AcquiredConfig, RouteDecision, RouteInput, Router, RoutingSnapshot, XdsRoutingLayer,
    };
    use crate::common::grpc_timeout::parse_grpc_timeout;
    use crate::xds::routing::RoutingError;
    use shared_http_body::SharedBody;
    use std::sync::{Arc, Mutex};
    use tower::{Service, ServiceBuilder, ServiceExt, service_fn};

    /// Which `grpc-timeout` each attempt reached the endpoint with.
    type SeenTimeouts = Arc<Mutex<Vec<Option<String>>>>;

    /// Routes every call to one cluster with the given xDS max stream duration,
    /// once a route config arrives `config_delay` after the call does.
    struct TestRouter {
        max_stream_duration: Option<Duration>,
        config_delay: Option<Duration>,
    }

    impl Router for TestRouter {
        fn acquire(&self) -> AcquiredConfig {
            match self.config_delay {
                None => AcquiredConfig::Ready(Arc::default()),
                Some(delay) => AcquiredConfig::Pending(Box::pin(async move {
                    tokio::time::sleep(delay).await;
                    Ok(Arc::default())
                })),
            }
        }

        fn route(
            &self,
            _input: &RouteInput<'_>,
            _config: &RoutingSnapshot,
        ) -> Result<RouteDecision, RoutingError> {
            Ok(RouteDecision {
                cluster: "c".into(),
                request_hash: None,
                retry_config: None,
                max_stream_duration: self.max_stream_duration,
            })
        }
    }

    /// A router whose config is already in effect, matching routes with `msd`.
    fn router(msd: Option<Duration>) -> TestRouter {
        TestRouter {
            max_stream_duration: msd,
            config_delay: None,
        }
    }

    fn routing<ResBody>(
        router: TestRouter,
        policy: DeadlinePolicy<ResBody>,
    ) -> XdsRoutingLayer<ResBody> {
        XdsRoutingLayer::new(Arc::new(router), None, Arc::from("svc"), policy)
    }

    /// A response body that yields one data frame and then never completes.
    struct StallingBody {
        sent_first: bool,
    }

    impl Body for StallingBody {
        type Data = Bytes;
        type Error = tonic::Status;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Bytes>, tonic::Status>>> {
            if std::mem::replace(&mut self.sent_first, true) {
                Poll::Pending
            } else {
                Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(b"first")))))
            }
        }
    }

    async fn next_frame(body: &mut TonicBody) -> Option<Result<Frame<Bytes>, tonic::Status>> {
        std::future::poll_fn(|cx| Pin::new(&mut *body).poll_frame(cx)).await
    }

    /// What the mock endpoint does on each attempt.
    #[derive(Clone, Copy)]
    enum Endpoint {
        /// Replies OK after the given delay.
        OkAfter(Duration),
        /// Replies UNAVAILABLE (retryable) after the given delay.
        UnavailableAfter(Duration),
        /// Gives up when its advertised `grpc-timeout` elapses: with an error
        /// like tonic's client-side endpoint timeout, or (`reply`) with a
        /// CANCELLED response like a tonic server.
        HonorsGrpcTimeout { reply: bool },
        /// Replies with headers at once, then streams one frame and stalls.
        StallingStream,
    }

    fn grpc_endpoint_request(mut req: Request<SharedBody<TonicBody>>) -> Request<TonicBody> {
        set_grpc_timeout_header(&mut req);
        req.map(TonicBody::new)
    }

    /// routing (with the gRPC deadline) -> retry (UNAVAILABLE, up to 4
    /// retries) -> endpoint mock.
    fn grpc_stack(
        endpoint: Endpoint,
        router: TestRouter,
    ) -> (
        impl Service<Request<TonicBody>, Response = Response<TonicBody>, Error = BoxError, Future: Send>
        + Clone,
        SeenTimeouts,
    ) {
        let seen = SeenTimeouts::default();
        let recorder = seen.clone();
        let mock = service_fn(move |req: Request<TonicBody>| {
            let timeout = req
                .headers()
                .get(GRPC_TIMEOUT_HEADER)
                .map(|v| v.to_str().unwrap().to_owned());
            recorder.lock().unwrap().push(timeout.clone());
            async move {
                match endpoint {
                    Endpoint::OkAfter(delay) => {
                        tokio::time::sleep(delay).await;
                        Ok(Response::new(TonicBody::empty()))
                    }
                    Endpoint::UnavailableAfter(delay) => {
                        tokio::time::sleep(delay).await;
                        Ok(tonic::Status::unavailable("try again").into_http())
                    }
                    Endpoint::HonorsGrpcTimeout { reply } => {
                        let timeout =
                            parse_grpc_timeout(timeout.as_deref().expect("grpc-timeout")).unwrap();
                        tokio::time::sleep(timeout).await;
                        if reply {
                            Ok(tonic::Status::cancelled("Timeout expired").into_http())
                        } else {
                            Err("timeout expired".into())
                        }
                    }
                    Endpoint::StallingStream => Ok(Response::new(TonicBody::new(StallingBody {
                        sent_first: false,
                    }))),
                }
            }
        });
        let retry = RetrySharedConfig::new(
            RetryConfig::new()
                .num_retries(4)
                .retry_backoff(RetryBackoffConfig::new(Duration::from_millis(10))),
            Arc::new(GrpcRetryClassifier::new(vec![tonic::Code::Unavailable])),
        );
        let stack = ServiceBuilder::new()
            .layer(routing(router, DeadlinePolicy::grpc()))
            .layer(RetryLayer::new(Arc::new(retry)))
            .map_request(grpc_endpoint_request)
            .service(mock);
        (stack, seen)
    }

    /// A request carrying, optionally, an app `grpc-timeout`.
    fn request(app_timeout: Option<&str>) -> Request<TonicBody> {
        let mut req = Request::new(TonicBody::empty());
        if let Some(timeout) = app_timeout {
            req.headers_mut()
                .insert(GRPC_TIMEOUT_HEADER, HeaderValue::from_str(timeout).unwrap());
        }
        req
    }

    /// Calls the service and returns the error code (or `Ok`) and elapsed time.
    async fn call<S>(svc: S, req: Request<TonicBody>) -> (Result<(), tonic::Code>, Duration)
    where
        S: Service<Request<TonicBody>, Response = Response<TonicBody>, Error = BoxError>,
    {
        let start = Instant::now();
        let result = svc.oneshot(req).await;
        let elapsed = start.elapsed();
        let result = result.map(drop).map_err(|err| {
            err.downcast::<tonic::Status>()
                .map(|status| status.code())
                .unwrap_or(tonic::Code::Unknown)
        });
        (result, elapsed)
    }

    fn secs(s: u64) -> Duration {
        Duration::from_secs(s)
    }

    /// gRFC A31: the effective deadline is min(app `grpc-timeout`, xDS value).
    /// It bounds the call and is advertised to the backend.
    #[tokio::test(start_paused = true)]
    async fn effective_deadline() {
        const DELAY: u64 = 30;
        // (xDS msd, app `grpc-timeout`, effective deadline); durations in seconds.
        let cases = [
            (None, None, None),
            (Some(10), None, Some(10)),
            (None, Some("20S"), Some(20)),
            (Some(10), Some("20S"), Some(10)),
            (Some(10), Some("5S"), Some(5)),
            (Some(40), None, Some(40)),
            // A reply ready on the same tick as the deadline is kept.
            (Some(DELAY), None, Some(DELAY)),
            // A malformed app `grpc-timeout` is treated as absent, as tonic does:
            // forwarded untouched, or replaced when an xDS deadline applies.
            (None, Some("bogus"), None),
            (Some(2), Some("bogus"), Some(2)),
        ];
        for (msd, app, deadline) in cases {
            let case = format!("msd={msd:?} app={app:?}");
            let (svc, seen) = grpc_stack(Endpoint::OkAfter(secs(DELAY)), router(msd.map(secs)));
            let outcome = call(svc, request(app)).await;
            let want = match deadline.filter(|&d| d < DELAY) {
                Some(d) => (Err(tonic::Code::DeadlineExceeded), secs(d)),
                None => (Ok(()), secs(DELAY)),
            };
            assert_eq!(outcome, want, "{case}");
            let header = deadline.map(|d| encode_grpc_timeout(secs(d)));
            let want_header = header.or(app.map(String::from));
            assert_eq!(*seen.lock().unwrap(), vec![want_header], "{case}");
        }
    }

    /// The deadline counts from the start of the call, so time spent waiting
    /// for the first route config uses it up too.
    #[tokio::test(start_paused = true)]
    async fn deadline_counts_from_call_start() {
        const CONFIG_DELAY: u64 = 4;
        // (xDS msd, app `grpc-timeout`, when the call fails, timeout the
        // endpoint is sent if it is called); durations in seconds.
        let cases = [
            // The app deadline cuts the wait for a route config short.
            (None, Some("1S"), 1, None),
            // The xDS limit is only known once a route is matched; if it has
            // passed by then, the endpoint is never called.
            (Some(3), None, CONFIG_DELAY, None),
            // Otherwise only what is left of it is sent on.
            (Some(10), None, 10, Some(6)),
            (Some(10), Some("8S"), 8, Some(4)),
        ];
        for (msd, app, fails_at, sent) in cases {
            let case = format!("msd={msd:?} app={app:?}");
            let router = TestRouter {
                max_stream_duration: msd.map(secs),
                config_delay: Some(secs(CONFIG_DELAY)),
            };
            let (svc, seen) = grpc_stack(Endpoint::OkAfter(secs(30)), router);
            let outcome = call(svc, request(app)).await;
            assert_eq!(
                outcome,
                (Err(tonic::Code::DeadlineExceeded), secs(fails_at)),
                "{case}"
            );
            let want_sent: Vec<_> = sent
                .map(|t| Some(encode_grpc_timeout(secs(t))))
                .into_iter()
                .collect();
            assert_eq!(*seen.lock().unwrap(), want_sent, "{case}");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_spans_retries() {
        // Each attempt takes 4s, so the 10s deadline lands in the third attempt
        // rather than restarting with each one.
        let (svc, seen) = grpc_stack(Endpoint::UnavailableAfter(secs(4)), router(Some(secs(10))));
        let (result, elapsed) = call(svc, request(None)).await;
        assert_eq!(result, Err(tonic::Code::DeadlineExceeded));
        assert_eq!(elapsed, secs(10));

        // Each attempt advertises only the time it has left.
        let seen: Vec<Duration> = seen
            .lock()
            .unwrap()
            .iter()
            .map(|t| parse_grpc_timeout(t.as_deref().unwrap()).unwrap())
            .collect();
        assert_eq!(seen.len(), 3, "{seen:?}");
        assert_eq!(seen[0], secs(10));
        assert!(seen[1] < secs(6) && seen[1] > secs(5), "{seen:?}");
        assert!(seen[2] < secs(2) && seen[2] > secs(1), "{seen:?}");
    }

    #[tokio::test(start_paused = true)]
    async fn endpoint_timeout_at_deadline_reports_deadline_exceeded() {
        for reply in [false, true] {
            let (svc, _) = grpc_stack(Endpoint::HonorsGrpcTimeout { reply }, router(Some(secs(3))));
            let (result, elapsed) = call(svc, request(None)).await;
            assert_eq!(result, Err(tonic::Code::DeadlineExceeded), "reply={reply}");
            assert_eq!(elapsed, secs(3), "reply={reply}");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_applies_to_response_stream() {
        let (svc, _) = grpc_stack(Endpoint::StallingStream, router(Some(secs(5))));
        let start = Instant::now();
        let response = svc.oneshot(request(None)).await.unwrap();
        let mut body = response.into_body();

        let first = next_frame(&mut body).await.unwrap().unwrap();
        assert_eq!(first.into_data().unwrap(), Bytes::from_static(b"first"));

        let status = next_frame(&mut body).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
        assert_eq!(start.elapsed(), secs(5));
        assert!(next_frame(&mut body).await.is_none());
    }

    /// A body that ends with trailers, then fails the test if polled again.
    struct TrailersBody {
        sent: bool,
    }

    impl Body for TrailersBody {
        type Data = Bytes;
        type Error = tonic::Status;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Bytes>, tonic::Status>>> {
            assert!(!self.sent, "polled after trailers");
            self.sent = true;
            Poll::Ready(Some(Ok(Frame::trailers(http::HeaderMap::new()))))
        }
    }

    #[tokio::test(start_paused = true)]
    async fn finished_response_stream_ignores_later_deadline() {
        let sleep = Box::pin(tokio::time::sleep(secs(5)));
        let mut body = grpc_body_deadline(TonicBody::new(TrailersBody { sent: false }), sleep);

        let trailers = next_frame(&mut body).await.unwrap().unwrap();
        assert!(trailers.is_trailers());

        tokio::time::advance(secs(10)).await;
        assert!(next_frame(&mut body).await.is_none());
        assert!(body.is_end_stream());
    }

    #[tokio::test(start_paused = true)]
    async fn headers_only_policy_ignores_grpc_timeout_and_body() {
        let svc = |msd| {
            ServiceBuilder::new()
                .layer(routing(router(msd), DeadlinePolicy::headers_only()))
                .service(service_fn(|_req: Request<TonicBody>| async {
                    tokio::time::sleep(secs(5)).await;
                    Ok::<_, BoxError>(Response::new(TonicBody::empty()))
                }))
        };

        // The app `grpc-timeout` is not consulted on a transport-generic channel.
        let (result, elapsed) = call(svc(None), request(Some("1S"))).await;
        assert_eq!(result, Ok(()));
        assert_eq!(elapsed, secs(5));

        // The xDS max stream duration still bounds the wait for response headers.
        let (result, elapsed) = call(svc(Some(secs(2))), request(None)).await;
        assert_eq!(result, Err(tonic::Code::DeadlineExceeded));
        assert_eq!(elapsed, secs(2));
    }
}
