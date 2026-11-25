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

use std::future::Future;
use std::io;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use grpc::__unstable::UnstableTrait;
use grpc::__unstable::client::name_resolution::TCP_IP_NETWORK_TYPE;
use grpc::__unstable::rt::BoxEndpoint;
use grpc::__unstable::rt::BoxFuture;
use grpc::__unstable::rt::DnsResolver;
use grpc::__unstable::rt::EndpointListener;
use grpc::__unstable::rt::GrpcEndpoint;
use grpc::__unstable::rt::ResolverOptions;
use grpc::__unstable::rt::Runtime;
use grpc::__unstable::rt::StreamEndpoint;
use grpc::__unstable::rt::TcpOptions;
use grpc::__unstable::rt::UnixSocketOptions;
use grpc::rt::BoxedTaskHandle;
use grpc::rt::Sleep;
use grpc::rt::TaskHandle;
use grpc::server::ListenerAddress;
use grpc::server::TcpAddress;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[cfg(feature = "dns")]
use crate::hickory_resolver;
#[cfg(unix)]
use crate::unix;

/// A DNS resolver that uses tokio::net::lookup_host for resolution. It only
/// supports host lookups.
#[cfg(any(test, not(feature = "dns")))]
pub(crate) struct TokioDefaultDnsResolver {
    _priv: (),
}

#[cfg(any(test, not(feature = "dns")))]
#[grpc::async_trait]
impl DnsResolver for TokioDefaultDnsResolver {
    async fn lookup_host_name(&self, name: &str) -> Result<Vec<IpAddr>, String> {
        let name_with_port = match name.parse::<IpAddr>() {
            Ok(ip) => SocketAddr::new(ip, 0).to_string(),
            Err(_) => format!("{name}:0"),
        };
        let ips = tokio::net::lookup_host(name_with_port)
            .await
            .map_err(|err| err.to_string())?
            .map(|socket_addr| socket_addr.ip())
            .collect();
        Ok(ips)
    }

    async fn lookup_txt(&self, _name: &str) -> Result<Vec<String>, String> {
        Err("TXT record lookup unavailable. Enable the optional 'dns' feature to enable service config lookups.".to_string())
    }
}

#[cfg(any(test, not(feature = "dns")))]
impl TokioDefaultDnsResolver {
    pub(crate) fn new(opts: ResolverOptions) -> Result<Self, String> {
        if opts.server_addr().is_some() {
            return Err("Custom DNS servers are not supported, enable optional feature 'dns' to enable support.".to_string());
        }
        Ok(TokioDefaultDnsResolver { _priv: () })
    }
}

/// A gRPC [`Runtime`] backed by a specific [Tokio](https://tokio.rs) runtime.
///
/// All tasks, timers, and I/O are bound to the runtime identified by `handle`,
/// regardless of which thread or executor calls into gRPC. The runtime must
/// have the IO and time drivers enabled.
#[derive(Debug)]
pub(crate) struct TokioRuntime {
    handle: Handle,
}

impl TokioRuntime {
    pub(crate) fn new(handle: Handle) -> Self {
        Self { handle }
    }
}

pin_project_lite::pin_project! {
    /// Polls `fut` within the context of the Tokio runtime identified by
    /// `handle`.
    ///
    /// Tokio registers sockets and timers with the runtime that is current
    /// when they are created or first polled, and some libraries (e.g.
    /// hickory) spawn tasks onto the current runtime. Wrapping a future in
    /// `InRuntime` binds all of that to `handle`'s runtime, so the future can
    /// be polled from any thread or executor.
    pub(crate) struct InRuntime<F> {
        handle: Handle,
        #[pin]
        fut: F,
    }
}

impl<F> InRuntime<F> {
    pub(crate) fn new(handle: Handle, fut: F) -> Self {
        Self { handle, fut }
    }
}

impl<F: Future> Future for InRuntime<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let this = self.project();
        let _guard = this.handle.enter();
        this.fut.poll(cx)
    }
}

/// Runs a [`DnsResolver`]'s lookups within the context of a specific Tokio
/// runtime.
struct InRuntimeDnsResolver<R> {
    handle: Handle,
    inner: R,
}

#[grpc::async_trait]
impl<R: DnsResolver> DnsResolver for InRuntimeDnsResolver<R> {
    async fn lookup_host_name(&self, name: &str) -> Result<Vec<IpAddr>, String> {
        InRuntime::new(self.handle.clone(), self.inner.lookup_host_name(name)).await
    }

    async fn lookup_txt(&self, name: &str) -> Result<Vec<String>, String> {
        InRuntime::new(self.handle.clone(), self.inner.lookup_txt(name)).await
    }
}

/// Wraps a Tokio [`JoinHandle`] as a [`TaskHandle`].
struct TokioTaskHandle(JoinHandle<()>);

impl UnstableTrait for TokioTaskHandle {}

impl TaskHandle for TokioTaskHandle {
    fn abort(&self) {
        self.0.abort();
    }
}

pin_project_lite::pin_project! {
    /// Wraps a Tokio [`Sleep`](tokio::time::Sleep) future as a [`Sleep`].
    struct TokioSleep {
        #[pin]
        inner: tokio::time::Sleep,
    }
}

impl Future for TokioSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.project().inner.poll(cx)
    }
}

impl UnstableTrait for TokioSleep {}

impl Sleep for TokioSleep {}

impl Runtime for TokioRuntime {
    fn spawn(&self, task: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> BoxedTaskHandle {
        Box::new(TokioTaskHandle(self.handle.spawn(task)))
    }

    fn get_dns_resolver(&self, opts: ResolverOptions) -> Result<Box<dyn DnsResolver>, String> {
        let _guard = self.handle.enter();
        #[cfg(feature = "dns")]
        let inner = hickory_resolver::DnsResolver::new(opts)?;
        #[cfg(not(feature = "dns"))]
        let inner = TokioDefaultDnsResolver::new(opts)?;
        Ok(Box::new(InRuntimeDnsResolver {
            handle: self.handle.clone(),
            inner,
        }))
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>> {
        // A Tokio timer is bound to the runtime that is current when it is
        // created.
        let _guard = self.handle.enter();
        Box::pin(TokioSleep {
            inner: tokio::time::sleep(duration),
        })
    }

    fn now(&self) -> Instant {
        // Use tokio's clock so that a paused/advanced test clock is respected.
        let _guard = self.handle.enter();
        tokio::time::Instant::now().into_std()
    }

    fn tcp_stream(
        &self,
        target: SocketAddr,
        opts: TcpOptions,
    ) -> BoxFuture<Result<BoxEndpoint, String>> {
        Box::pin(InRuntime::new(self.handle.clone(), async move {
            let stream = TcpStream::connect(target)
                .await
                .map_err(|err| err.to_string())?;
            stream
                .set_nodelay(opts.enable_nodelay())
                .map_err(|err| err.to_string())?;
            if let Some(duration) = opts.keepalive() {
                let sock_ref = socket2::SockRef::from(&stream);
                let mut ka = socket2::TcpKeepalive::new();
                ka = ka.with_time(duration);
                sock_ref
                    .set_tcp_keepalive(&ka)
                    .map_err(|err| err.to_string())?;
            }
            let stream: Box<dyn GrpcEndpoint> = Box::new(tcp_endpoint(stream)?);
            Ok(stream)
        }))
    }

    #[cfg(unix)]
    fn unix_stream(
        &self,
        path: std::path::PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>> {
        Box::pin(InRuntime::new(self.handle.clone(), unix::connect(path)))
    }

    fn tcp_listener(
        &self,
        addr: SocketAddr,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        let handle = self.handle.clone();
        Box::pin(InRuntime::new(self.handle.clone(), async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| e.to_string())?;
            let local_addr = TcpAddress(listener.local_addr().map_err(|e| e.to_string())?);
            Ok(Box::new(TokioTcpListener {
                listener,
                local_addr,
                handle,
            }) as Box<dyn EndpointListener>)
        }))
    }

    #[cfg(unix)]
    fn unix_listener(
        &self,
        path: std::path::PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        let handle = self.handle.clone();
        Box::pin(InRuntime::new(self.handle.clone(), async move {
            unix::bind(path, handle)
        }))
    }

    #[cfg(not(unix))]
    fn unix_listener(
        &self,
        _path: std::path::PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        Box::pin(
            async move { Err("Unix listeners are not supported on this platform".to_string()) },
        )
    }
}

/// Creates an endpoint for a connected TCP stream.
fn tcp_endpoint(stream: TcpStream) -> Result<StreamEndpoint<TcpStream>, String> {
    let local_addr = stream
        .local_addr()
        .map_err(|err| err.to_string())?
        .to_string()
        .into_boxed_str();
    let peer_addr = stream
        .peer_addr()
        .map_err(|err| err.to_string())?
        .to_string()
        .into_boxed_str();
    Ok(StreamEndpoint::new(
        stream,
        local_addr,
        peer_addr,
        TCP_IP_NETWORK_TYPE,
    ))
}

/// Creates an endpoint for a stream returned by `accept`, which also returned
/// `peer_addr`. Looking the peer address up again could fail if the client has
/// already reset the connection. If the local address can't be read, it is left
/// empty.
fn accepted_tcp_endpoint(stream: TcpStream, peer_addr: SocketAddr) -> StreamEndpoint<TcpStream> {
    let local_addr = stream
        .local_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_default()
        .into_boxed_str();
    StreamEndpoint::new(
        stream,
        local_addr,
        peer_addr.to_string().into_boxed_str(),
        TCP_IP_NETWORK_TYPE,
    )
}

/// How [`accept_with_retry`] backs off after accept failures that aren't
/// per-connection.
pub(crate) struct AcceptRetryPolicy {
    /// Delay after the first such failure.
    initial_backoff: Duration,
    /// Upper bound on the delay between retries.
    max_backoff: Duration,
}

/// Policy used by the tokio TCP and Unix listeners.
pub(crate) const TOKIO_ACCEPT_RETRY_POLICY: AcceptRetryPolicy = AcceptRetryPolicy {
    initial_backoff: Duration::from_millis(5),
    max_backoff: Duration::from_secs(1),
};

/// Reports whether `err` affects only one pending connection, leaving the
/// listener usable.
///
/// Uses the same classification as axum:
/// <https://github.com/tokio-rs/axum/blob/42d6fdc80d8933ced5ff4ae858f43a3581106c75/axum/src/serve/listener.rs#L266-L274>.
fn is_connection_error(err: &io::Error) -> bool {
    matches!(
        err.kind(),
        io::ErrorKind::ConnectionAborted
            | io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionReset
    )
}

/// Calls `accept` until it succeeds.
///
/// Per-connection errors are retried immediately. Other errors are retried with
/// exponential back-off, as configured by `policy`, without limit.
// TODO: Every accept error is retried, because there's no simple cross-platform way
// to tell retryable errors from permanent ones. Following other Rust servers is our
// best guess for now.
pub(crate) async fn accept_with_retry<T, F, Fut>(policy: &AcceptRetryPolicy, mut accept: F) -> T
where
    F: FnMut() -> Fut,
    Fut: Future<Output = io::Result<T>>,
{
    let mut backoff = policy.initial_backoff;
    loop {
        let err = match accept().await {
            Ok(accepted) => return accepted,
            Err(err) => err,
        };
        if is_connection_error(&err) {
            // A per-connection error means the kernel is still handing us
            // connections, so the listener is healthy: reset the back-off.
            backoff = policy.initial_backoff;
            continue;
        }
        sleep(backoff).await;
        backoff = backoff.saturating_mul(2).min(policy.max_backoff);
    }
}

/// Wraps `tokio::net::TcpListener` as an [`EndpointListener`](EndpointListener).
struct TokioTcpListener {
    listener: tokio::net::TcpListener,
    /// Determined when binding, so that binding fails if it can't be read.
    local_addr: TcpAddress,
    /// The runtime the listener was bound in, which `accept` runs within.
    handle: Handle,
}

#[grpc::async_trait]
impl EndpointListener for TokioTcpListener {
    async fn accept(&self) -> Result<Box<dyn GrpcEndpoint>, String> {
        let accept = accept_with_retry(&TOKIO_ACCEPT_RETRY_POLICY, || self.listener.accept());
        let (stream, peer_addr) = InRuntime::new(self.handle.clone(), accept).await;
        Ok(Box::new(accepted_tcp_endpoint(stream, peer_addr)))
    }

    fn local_addr(&self) -> &dyn ListenerAddress {
        &self.local_addr
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::future::Ready;
    use std::future::ready;
    use std::io;
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::time::Duration;

    use grpc::__unstable::rt::TcpOptions;
    use tokio::runtime::Handle;

    use super::AcceptRetryPolicy;
    use super::DnsResolver;
    use super::ResolverOptions;
    use super::Runtime;
    use super::TokioDefaultDnsResolver;
    use super::TokioRuntime;
    use super::accept_with_retry;

    const TEST_POLICY: AcceptRetryPolicy = AcceptRetryPolicy {
        initial_backoff: Duration::from_millis(10),
        max_backoff: Duration::from_millis(40),
    };

    /// Per-connection error, retried immediately.
    fn aborted() -> io::Result<u32> {
        Err(io::Error::from(io::ErrorKind::ConnectionAborted))
    }

    /// Any other error, retried with back-off.
    fn other() -> io::Result<u32> {
        Err(io::Error::other("too many open files"))
    }

    /// Runs `accept_with_retry` with `TEST_POLICY` over `script` and returns the
    /// result, the number of `accept` calls, and the virtual time that passed.
    async fn run_script(script: Vec<io::Result<u32>>) -> (u32, usize, Duration) {
        let mut script = VecDeque::from(script);
        let mut calls = 0;
        let runtime = TokioRuntime::new(Handle::current());
        let start = runtime.now();
        let result = accept_with_retry(&TEST_POLICY, || -> Ready<io::Result<u32>> {
            calls += 1;
            ready(script.pop_front().expect("script exhausted"))
        })
        .await;
        (result, calls, runtime.now() - start)
    }

    #[tokio::test(start_paused = true)]
    async fn accept_with_retry_returns_first_success() {
        let (result, calls, elapsed) = run_script(vec![Ok(1)]).await;

        assert_eq!(result, 1);
        assert_eq!(calls, 1);
        assert_eq!(elapsed, Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn accept_with_retry_retries_connection_errors_immediately() {
        let script = vec![
            aborted(),
            Err(io::Error::from(io::ErrorKind::ConnectionReset)),
            Err(io::Error::from(io::ErrorKind::ConnectionRefused)),
            Ok(1),
        ];

        let (result, calls, elapsed) = run_script(script).await;

        assert_eq!(result, 1);
        assert_eq!(calls, 4);
        assert_eq!(elapsed, Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn accept_with_retry_backs_off_exponentially() {
        let (result, calls, elapsed) = run_script(vec![other(), other(), Ok(1)]).await;

        assert_eq!(result, 1);
        assert_eq!(calls, 3);
        assert_eq!(elapsed, Duration::from_millis(10 + 20));
    }

    #[tokio::test(start_paused = true)]
    async fn accept_with_retry_caps_backoff() {
        let (result, calls, elapsed) =
            run_script(vec![other(), other(), other(), other(), Ok(1)]).await;

        assert_eq!(result, 1);
        assert_eq!(calls, 5);
        assert_eq!(elapsed, Duration::from_millis(10 + 20 + 40 + 40));
    }

    #[tokio::test(start_paused = true)]
    async fn accept_with_retry_connection_error_resets_backoff() {
        let script = vec![other(), other(), aborted(), other(), Ok(1)];

        let (result, calls, elapsed) = run_script(script).await;

        assert_eq!(result, 1);
        assert_eq!(calls, 5);
        assert_eq!(elapsed, Duration::from_millis(10 + 20 + 10));
    }

    #[tokio::test]
    async fn lookup_hostname() {
        let runtime = TokioRuntime::new(Handle::current());

        let dns = runtime
            .get_dns_resolver(ResolverOptions::default())
            .unwrap();
        let ips = dns.lookup_host_name("localhost").await.unwrap();
        assert!(
            !ips.is_empty(),
            "Expect localhost to resolve to more than 1 IPs."
        );
    }

    #[tokio::test]
    async fn default_resolver_txt_fails() {
        let default_resolver = TokioDefaultDnsResolver::new(ResolverOptions::default()).unwrap();

        let txt = default_resolver.lookup_txt("google.com").await;
        assert!(txt.is_err());
    }

    #[tokio::test]
    async fn default_resolver_custom_authority() {
        let opts = ResolverOptions::default().with_server_addr(Some("8.8.8.8:53".parse().unwrap()));
        let default_resolver = TokioDefaultDnsResolver::new(opts);
        assert!(default_resolver.is_err());
    }

    #[tokio::test(start_paused = true)]
    async fn now_reflects_paused_clock() {
        let runtime = TokioRuntime::new(Handle::current());
        let start = runtime.now();

        tokio::time::advance(Duration::from_secs(3600)).await;
        assert_eq!(runtime.now() - start, Duration::from_secs(3600));

        runtime.sleep(Duration::from_secs(60)).await;
        assert_eq!(runtime.now() - start, Duration::from_secs(3660));
    }

    /// Builds a multi-threaded Tokio runtime. Tests that use it run outside
    /// any Tokio runtime context themselves, as a non-Tokio caller would.
    fn multi_thread_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap()
    }

    #[test]
    fn spawn_from_outside_runtime() {
        let rt = multi_thread_runtime();
        let runtime = TokioRuntime::new(rt.handle().clone());
        assert!(Handle::try_current().is_err());

        let (tx, rx) = mpsc::channel();
        runtime.spawn(Box::pin(async move {
            tx.send(()).unwrap();
        }));
        rx.recv_timeout(Duration::from_secs(5))
            .expect("spawned task did not run");
    }

    #[test]
    fn sleep_created_and_polled_outside_runtime() {
        let rt = multi_thread_runtime();
        let runtime = TokioRuntime::new(rt.handle().clone());

        // Both creating and polling the timer happen outside any Tokio
        // runtime context; the timer is driven by `rt`.
        let sleep = runtime.sleep(Duration::from_millis(10));
        futures_executor::block_on(sleep);
    }

    #[test]
    fn tcp_stream_polled_outside_runtime() {
        let rt = multi_thread_runtime();
        let runtime = TokioRuntime::new(rt.handle().clone());
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let endpoint =
            futures_executor::block_on(runtime.tcp_stream(addr, TcpOptions::default())).unwrap();
        assert_eq!(endpoint.get_peer_address(), addr.to_string());
    }

    #[test]
    fn dns_lookup_polled_outside_runtime() {
        let rt = multi_thread_runtime();
        let runtime = TokioRuntime::new(rt.handle().clone());

        let dns = runtime
            .get_dns_resolver(ResolverOptions::default())
            .unwrap();
        let ips = futures_executor::block_on(dns.lookup_host_name("localhost")).unwrap();
        assert!(!ips.is_empty());
    }
}
