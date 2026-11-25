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

use crate::common::async_util::BoxFuture;
use crate::xds::cache::ClusterWatches;
use std::any::Any;
use std::fmt::{self, Debug};
use std::future::{Future, poll_fn};
use std::hash::Hash;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::task::{Context, Poll};
use tokio::sync::{Notify, mpsc, oneshot};
use tokio_util::sync::PollSender;
use tower::{BoxError, Service, balance::p2c::Balance, discover::Discover, load::Load};
use tracing::Span;

type RespFut<Resp> = BoxFuture<Result<Resp, BoxError>>;

const DEFAULT_BUFFER_CAPACITY: usize = 1024;

/// Tells a channel's cluster workers to connect ahead of traffic.
///
/// Until set, a worker polls its balancer only while a request waits. Once set,
/// by [`XdsChannel::connect`] or the channel's first request, it polls
/// continuously, so the balancer connects endpoints as they're discovered.
///
/// [`XdsChannel::connect`]: crate::client::channel::XdsChannel::connect
#[derive(Clone, Debug, Default)]
pub(crate) struct ConnectSignal(Arc<ConnectState>);

#[derive(Debug, Default)]
struct ConnectState {
    set: AtomicBool,
    notify: Notify,
}

impl ConnectSignal {
    /// Sets the signal and wakes every waiting worker. Idempotent.
    pub(crate) fn set(&self) {
        // Every request sets the signal; the load keeps that a shared read.
        if !self.is_set() && !self.0.set.swap(true, Ordering::AcqRel) {
            self.0.notify.notify_waiters();
        }
    }

    fn is_set(&self) -> bool {
        self.0.set.load(Ordering::Acquire)
    }

    /// Resolves once the signal is set.
    async fn wait(&self) {
        // Created before the check: `notify_waiters` completes any `Notified`
        // that already exists, even if it hasn't been polled yet.
        let notified = self.0.notify.notified();
        if !self.is_set() {
            notified.await;
        }
    }
}

/// `ClusterBalancer` is responsible for managing load balancing requests across multiple channels.
/// Currently, `ClusterBalancer` leverges `tower::balance::p2c` for doing P2C load balancing. In the future, we will
/// support more load balancing strategies as needed.
pub(crate) struct ClusterBalancer<D, Req>
where
    D: Discover,
    D::Key: Hash,
{
    balancer: Balance<D, Req>,
}

impl<D, Req> ClusterBalancer<D, Req>
where
    D: Discover,
    D::Key: Hash,
    D::Service: Service<Req>,
    <D::Service as Service<Req>>::Error: Into<BoxError>,
{
    /// Creates a new `ClusterBalancer` with provided service discovery.
    pub(crate) fn new(discover: D) -> Self {
        Self {
            balancer: Balance::new(discover),
        }
    }

    /// Returns the number of endpoints currently tracked by the balancer.
    /// This can be useful for monitoring and debugging purposes.
    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        self.balancer.len()
    }
}

impl<D, Req> Service<Req> for ClusterBalancer<D, Req>
where
    D: Discover + Unpin,
    D::Key: Hash + Clone,
    D::Error: Into<BoxError>,
    D::Service: Service<Req> + Load,
    <D::Service as Load>::Metric: std::fmt::Debug,
    <D::Service as Service<Req>>::Error: Into<BoxError> + 'static,
    <D::Service as Service<Req>>::Future: Send + 'static,
{
    type Response = <Balance<D, Req> as Service<Req>>::Response;
    type Error = <Balance<D, Req> as Service<Req>>::Error;
    type Future = RespFut<Self::Response>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.balancer.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        Box::pin(self.balancer.call(req))
    }
}

/// A cloneable handle that load-balances requests across a cluster's
/// endpoints, like `tonic::transport::Channel` for a whole xDS cluster.
///
/// Handles queue requests to a worker task that owns the balancer. The worker
/// runs until every handle is dropped or the balancer fails, and drives the
/// balancer ahead of traffic once its [`ConnectSignal`] is set.
pub(crate) struct ClusterChannel<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    tx: PollSender<Message<Req, Resp>>,
    /// Why the worker stopped, if its balancer failed.
    failure: Arc<OnceLock<BalancerError>>,
}

/// A request queued for the worker.
struct Message<Req, Resp> {
    request: Req,
    /// Receives the balancer's response future.
    tx: oneshot::Sender<RespFut<Resp>>,
    /// The caller's span, entered while the balancer dispatches the request.
    span: Span,
}

impl<Req, Resp> Clone for ClusterChannel<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            failure: Arc::clone(&self.failure),
        }
    }
}

impl<Req, Resp> ClusterChannel<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    /// Creates a channel whose worker dispatches requests to `svc`, the
    /// service that load-balances across the cluster's endpoints.
    pub(crate) fn from_service<B>(svc: B, connect: ConnectSignal) -> Self
    where
        B: Service<Req, Error = BoxError, Future = RespFut<Resp>> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel(DEFAULT_BUFFER_CAPACITY);
        let failure = Arc::default();
        tokio::spawn(serve(svc, rx, connect, Arc::clone(&failure)));
        Self {
            tx: PollSender::new(tx),
            failure,
        }
    }

    /// Creates a channel that P2C-balances across the endpoints yielded by
    /// `discover`.
    pub(crate) fn from_discover<D>(discover: D, connect: ConnectSignal) -> Self
    where
        D: Discover + Unpin + Send + 'static,
        D::Key: Hash + Clone + Send,
        D::Error: Into<BoxError>,
        D::Service: Service<Req, Response = Resp> + Load + Send + 'static,
        <D::Service as Load>::Metric: Debug,
        <D::Service as Service<Req>>::Error: Into<BoxError>,
        <D::Service as Service<Req>>::Future: Send + 'static,
    {
        Self::from_service(ClusterBalancer::new(discover), connect)
    }
}

/// Builds the load-balancing channel for a cluster when the route config starts
/// referencing it.
///
/// The seam between the router and the balancer: the router owns a client for
/// each cluster in the route config, built from the cluster's CDS/EDS watches,
/// without knowing how endpoints are connected or balanced.
pub(crate) trait ClusterChannelFactory<Req, Resp>: Send + Sync + 'static
where
    Req: Send + 'static,
    Resp: 'static,
{
    /// Builds the channel for `cluster_name`, fed by `watches`, which connects
    /// endpoints ahead of traffic once `connect` is set.
    fn make(
        &self,
        cluster_name: &str,
        watches: ClusterWatches,
        connect: ConnectSignal,
    ) -> ClusterChannel<Req, Resp>;
}

impl<Req, Resp> Service<Req> for ClusterChannel<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    type Response = Resp;
    type Error = BoxError;
    type Future = RespFut<Resp>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx
            .poll_reserve(cx)
            .map_err(|_| closed_error(&self.failure))
    }

    fn call(&mut self, request: Req) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let message = Message {
            request,
            tx,
            span: Span::current(),
        };
        // If the worker is gone, the message and its `tx` drop here, so `rx`
        // fails below.
        let _ = self.tx.send_item(message);
        let failure = Arc::clone(&self.failure);
        Box::pin(async move {
            match rx.await {
                Ok(response) => response.await,
                Err(_) => Err(closed_error(&failure)),
            }
        })
    }
}

/// Dispatches queued requests to `svc` until every handle is dropped or `svc`
/// fails. Once `connect` is set, also polls `svc` while no request waits, so
/// the balancer connects endpoints ahead of traffic.
async fn serve<S, Req, Resp>(
    mut svc: S,
    mut rx: mpsc::Receiver<Message<Req, Resp>>,
    connect: ConnectSignal,
    failure: Arc<OnceLock<BalancerError>>,
) where
    S: Service<Req, Error = BoxError, Future = RespFut<Resp>>,
{
    let mut connected = pin!(connect.wait());
    let mut active = false;
    // A request taken off the queue before `svc` was ready for it.
    let mut held: Option<Message<Req, Resp>> = None;
    let served = poll_fn(|cx| {
        loop {
            // Short-circuits so `connected` isn't polled after it completes.
            active = active || connected.as_mut().poll(cx).is_ready();
            let message = match held.take() {
                Some(message) => message,
                None => match rx.poll_recv(cx) {
                    Poll::Ready(Some(message)) => message,
                    Poll::Ready(None) => return Poll::Ready(Ok(())),
                    Poll::Pending => {
                        if active && let Poll::Ready(Err(e)) = svc.poll_ready(cx) {
                            return Poll::Ready(Err(e));
                        }
                        return Poll::Pending;
                    }
                },
            };
            if message.tx.is_closed() {
                continue; // The caller gave up waiting.
            }
            match svc.poll_ready(cx) {
                Poll::Ready(Ok(())) => {
                    let response = message.span.in_scope(|| svc.call(message.request));
                    let _ = message.tx.send(response);
                }
                poll => {
                    held = Some(message);
                    return poll;
                }
            }
        }
    })
    .await;

    let Err(error) = served else { return };
    tracing::debug!(%error, "cluster load balancer failed");
    // Recorded before any caller is released, so every caller reports it.
    let _ = failure.set(BalancerError(Arc::new(error)));
    drop(held);
    drop(svc);
    rx.close();
    // Release requests sent on permits reserved before the close.
    while rx.recv().await.is_some() {}
}

/// The error a failed balancer returned, shared by every caller it fails.
#[derive(Clone, Debug)]
struct BalancerError(Arc<BoxError>);

impl fmt::Display for BalancerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cluster load balancer failed: {}", self.0)
    }
}

impl std::error::Error for BalancerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&**self.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("cluster channel worker stopped")]
struct WorkerStopped;

/// The error for a request the worker can no longer serve.
fn closed_error(failure: &OnceLock<BalancerError>) -> BoxError {
    match failure.get() {
        Some(error) => error.clone().into(),
        None => WorkerStopped.into(),
    }
}

/// `ClusterClient` manages channels that load-balance for a xDS cluster.
pub(crate) struct ClusterClient<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    name: String,
    channel: ClusterChannel<Req, Resp>,
}

impl Debug for ClusterClient<(), ()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusterClient")
            .field("name", &self.name)
            .finish()
    }
}

impl<Req, Resp> ClusterClient<Req, Resp>
where
    Req: Send + 'static,
    Resp: 'static,
{
    /// Creates a `ClusterClient` serving `name` through an already-built
    /// load-balancing channel.
    pub(crate) fn from_channel(name: String, channel: ClusterChannel<Req, Resp>) -> Self {
        Self { name, channel }
    }

    /// Returns a channel that can be used to send RPCs to the cluster.
    pub(crate) fn channel(&self) -> ClusterChannel<Req, Resp> {
        self.channel.clone()
    }

    /// Returns the name of the cluster.
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

/// A shared, type-erased handle to a cluster's [`ClusterClient`].
///
/// The router holds one for each cluster the active route config references,
/// and attaches it to every [`RouteDecision`] for that cluster. A request
/// therefore reaches the client it was routed with, even if the cluster leaves
/// the route config while the request is in flight. The client is dropped once
/// the router and every such request let go of it.
///
/// The type is erased because the router is transport-agnostic.
/// [`XdsLbService`] recovers the typed client.
///
/// [`RouteDecision`]: crate::client::route::RouteDecision
/// [`XdsLbService`]: crate::client::lb::XdsLbService
#[derive(Clone)]
pub(crate) struct ClusterRef(Arc<dyn Any + Send + Sync>);

impl ClusterRef {
    pub(crate) fn new<Req, Resp>(client: ClusterClient<Req, Resp>) -> Self
    where
        Req: Send + 'static,
        Resp: 'static,
    {
        Self(Arc::new(client))
    }

    /// Wraps `channel` in a [`ClusterClient`] for `name` and erases its type.
    pub(crate) fn from_channel<Req, Resp>(
        name: impl Into<String>,
        channel: ClusterChannel<Req, Resp>,
    ) -> Self
    where
        Req: Send + 'static,
        Resp: 'static,
    {
        Self::new(ClusterClient::from_channel(name.into(), channel))
    }

    /// The client, or `None` if it serves other request or response types.
    pub(crate) fn client<Req, Resp>(&self) -> Option<&ClusterClient<Req, Resp>>
    where
        Req: Send + 'static,
        Resp: 'static,
    {
        self.0.downcast_ref()
    }

    /// Whether `self` and `other` share one client.
    #[cfg(test)]
    pub(crate) fn same_client(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Debug for ClusterRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusterRef").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::retry::is_retryable_connection_error;
    use std::io;
    use std::pin::Pin;
    use std::sync::atomic::AtomicUsize;
    use std::task::ready;
    use std::time::Duration;
    use tower::ServiceExt;

    /// A balancer that counts readiness polls and is always ready.
    struct Probe {
        polls: Arc<AtomicUsize>,
    }

    impl Service<()> for Probe {
        type Response = ();
        type Error = BoxError;
        type Future = RespFut<()>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), BoxError>> {
            self.polls.fetch_add(1, Ordering::Relaxed);
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, (): ()) -> Self::Future {
            Box::pin(async { Ok(()) })
        }
    }

    /// A balancer that is unready until `fail` fires, then fails with a
    /// connection error.
    struct FailOn {
        fail: oneshot::Receiver<()>,
        _alive: Arc<()>,
    }

    impl Service<()> for FailOn {
        type Response = ();
        type Error = BoxError;
        type Future = RespFut<()>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), BoxError>> {
            let _ = ready!(Pin::new(&mut self.fail).poll(cx));
            Poll::Ready(Err(io::Error::from(io::ErrorKind::ConnectionRefused).into()))
        }

        fn call(&mut self, (): ()) -> Self::Future {
            unreachable!("never ready")
        }
    }

    async fn assert_balancer_failed(caller: impl Future<Output = Result<(), BoxError>>) {
        let err = tokio::time::timeout(Duration::from_secs(1), caller)
            .await
            .expect("caller hung")
            .unwrap_err();
        assert!(err.to_string().contains("load balancer failed"), "{err}");
        assert!(is_retryable_connection_error(&*err), "{err}");
    }

    #[tokio::test(start_paused = true)]
    async fn polls_idle_balancer_once_connect_is_set() {
        let polls = Arc::new(AtomicUsize::new(0));
        let connect = ConnectSignal::default();
        let probe = Probe {
            polls: Arc::clone(&polls),
        };
        let _channel = ClusterChannel::<(), ()>::from_service(probe, connect.clone());

        tokio::time::sleep(Duration::from_secs(1)).await;
        assert_eq!(polls.load(Ordering::Relaxed), 0);

        connect.set();
        tokio::time::sleep(Duration::from_secs(1)).await;
        assert_eq!(polls.load(Ordering::Relaxed), 1);
    }

    /// Whether the balancer fails idle after connect, or with one request held
    /// and another queued, it is dropped at once and every caller gets its
    /// error, including one that reserved a slot before the failure.
    #[tokio::test(start_paused = true)]
    async fn balancer_failure_reaches_every_caller() {
        for connected in [false, true] {
            let (fail, fail_rx) = oneshot::channel();
            let alive = Arc::new(());
            let balancer = Arc::downgrade(&alive);
            let connect = ConnectSignal::default();
            if connected {
                connect.set();
            }
            let svc = FailOn {
                fail: fail_rx,
                _alive: alive,
            };
            let channel = ClusterChannel::<(), ()>::from_service(svc, connect);
            let waiting = if connected { 0 } else { 2 };
            let callers: Vec<_> = (0..waiting)
                .map(|_| tokio::spawn(channel.clone().oneshot(())))
                .collect();
            let mut reserved = channel.clone();
            reserved.ready().await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;

            fail.send(()).unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
            assert_eq!(balancer.strong_count(), 0, "failed balancer still alive");

            for caller in callers {
                assert_balancer_failed(async { caller.await.unwrap() }).await;
            }
            assert_balancer_failed(reserved.call(())).await;
            assert_balancer_failed(channel.clone().oneshot(())).await;
        }
    }
}
