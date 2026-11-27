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

//! The runtime used by gRPC for I/O, timers, and background tasks.
//!
//! Runtime implementations are provided by separate crates, e.g. `grpc-tokio`.

use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
#[cfg(test)]
use std::time::Duration;
use std::time::Instant;

use crate::private::UnstableToken;
use crate::private::UnstableTrait;

pub(crate) use unstable::*;

pub(crate) mod hyper_wrapper;
/// A minimal Tokio-based runtime used by this crate's unit tests. The real
/// Tokio runtime lives in the `grpc-tokio` crate, which can't be used here
/// because it depends on this crate.
#[cfg(any(test, feature = "__internal-test-utils"))]
pub(crate) mod tokio;

/// A boxed handle to a task spawned with [`GrpcRuntime::spawn`].
pub type BoxedTaskHandle = Box<dyn TaskHandle>;

/// A future that resolves after a specified duration.
///
/// Only runtime implementations, which must use `grpc::__unstable`, may
/// implement this trait.
pub trait Sleep: UnstableTrait + Send + Sync + Future<Output = ()> {}

/// A handle to a spawned task.
///
/// Only runtime implementations, which must use `grpc::__unstable`, may
/// implement this trait.
pub trait TaskHandle: UnstableTrait + Send + Sync {
    /// Abort the associated task.
    fn abort(&self);
}

/// Returns the runtime used by unit tests in this crate and other gRPC crates.
///
/// Unlike `grpc_tokio::runtime()`, this can be created outside a Tokio
/// runtime: each operation uses the Tokio runtime that is current when it is
/// called (e.g. via `tokio::spawn`), and panics if there is none. This lets
/// synchronous tests construct components that require a runtime but never
/// use it.
#[cfg(any(test, feature = "__internal-test-utils"))]
#[doc(hidden)]
pub fn test_runtime() -> GrpcRuntime {
    GrpcRuntime::new(tokio::TokioRuntime::default(), UnstableToken)
}

/// A handle to the runtime used by gRPC for I/O, timers, and background
/// tasks.
///
/// Cloning a `GrpcRuntime` is cheap; clones share the same underlying runtime.
///
/// The underlying runtime must keep running for as long as anything using the
/// `GrpcRuntime` exists, e.g. every channel and server created with it. Using
/// it after the underlying runtime has stopped is a programming error: it may
/// panic, cause RPCs to fail, or cause them to hang.
#[derive(Clone, Debug)]
pub struct GrpcRuntime {
    inner: Arc<dyn Runtime>,
}

/// Error returned by [`GrpcRuntime::timeout`] when the deadline elapses.
#[cfg(test)]
#[derive(Debug)]
pub(crate) struct Elapsed;

impl GrpcRuntime {
    /// Creates a `GrpcRuntime` from a runtime implementation.
    pub fn new<T: Runtime + 'static>(runtime: T, _: UnstableToken) -> Self {
        GrpcRuntime {
            inner: Arc::new(runtime),
        }
    }

    /// Spawns the given task to run in the background.
    pub fn spawn(
        &self,
        task: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) -> BoxedTaskHandle {
        self.inner.spawn(task)
    }

    pub(crate) fn get_dns_resolver(
        &self,
        opts: ResolverOptions,
    ) -> Result<Box<dyn DnsResolver>, String> {
        self.inner.get_dns_resolver(opts)
    }

    /// Returns a future that completes after `duration`.
    pub fn sleep(&self, duration: std::time::Duration) -> Pin<Box<dyn Sleep>> {
        self.inner.sleep(duration)
    }

    /// Returns the current time according to this runtime's clock.
    pub fn now(&self) -> Instant {
        self.inner.now()
    }

    /// Awaits `fut`, returning `Err(Elapsed)` if it does not complete within
    /// `duration` according to this runtime's clock.
    #[cfg(test)]
    pub(crate) async fn timeout<F: Future>(
        &self,
        duration: Duration,
        fut: F,
    ) -> Result<F::Output, Elapsed> {
        let sleep = self.sleep(duration);
        ::tokio::select! {
            biased;
            out = fut => Ok(out),
            _ = sleep => Err(Elapsed),
        }
    }

    pub(crate) fn tcp_stream(
        &self,
        target: SocketAddr,
        opts: TcpOptions,
    ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>> {
        self.inner.tcp_stream(target, opts)
    }

    pub(crate) fn unix_stream(
        &self,
        path: PathBuf,
        opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>> {
        self.inner.unix_stream(path, opts)
    }

    pub(crate) fn tcp_listener(
        &self,
        addr: SocketAddr,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        self.inner.tcp_listener(addr)
    }

    pub(crate) fn unix_listener(
        &self,
        path: PathBuf,
        opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        self.inner.unix_listener(path, opts)
    }
}

/// Runtime APIs that are not yet stable.
///
/// These are exported via `grpc::__unstable::rt` for use by runtime
/// implementations (e.g. `grpc-tokio`) and other grpc-authored crates.
pub(crate) mod unstable {
    use std::fmt::Debug;
    use std::future::Future;
    use std::io;
    use std::io::IoSlice;
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use std::pin::Pin;
    use std::task::Context;
    use std::task::Poll;
    use std::time::Duration;
    use std::time::Instant;

    use ::tokio::io::AsyncRead;
    use ::tokio::io::AsyncWrite;
    use ::tokio::io::ReadBuf;

    use crate::private;
    use crate::rt::BoxedTaskHandle;
    use crate::rt::Sleep;
    use crate::server::ListenerAddress;

    pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
    pub type BoxEndpoint = Box<dyn GrpcEndpoint>;

    /// A server-side listening socket that yields incoming connections.
    #[crate::async_trait]
    pub trait EndpointListener: Send + Sync + 'static {
        /// Accepts the next incoming connection.
        ///
        /// An error stops the server, so return one only when the listener itself
        /// can't continue. A failure that affects only one connection must not be
        /// returned as an error.
        async fn accept(&self) -> Result<Box<dyn GrpcEndpoint>, String>;

        /// Returns the local address this listener is bound to.
        fn local_addr(&self) -> &dyn ListenerAddress;
    }

    /// An abstraction over an asynchronous runtime.
    ///
    /// The `Runtime` trait defines the core functionality required for
    /// executing asynchronous tasks, creating DNS resolvers, and performing
    /// time-based operations such as sleeping. It provides a uniform interface
    /// that can be implemented for various async runtimes, enabling pluggable
    /// and testable infrastructure.
    ///
    /// # Lifetime of the underlying runtime
    ///
    /// Implementations are typically bound to an underlying executor or
    /// reactor (e.g. a specific Tokio runtime). That executor must keep running
    /// for as long as anything using this `Runtime` exists, e.g. every channel
    /// and server created with it. If it stops, implementations need not
    /// report an error: they may panic (e.g. when `sleep` is called or its
    /// result is polled) or silently drop spawned tasks, so RPCs may fail or
    /// hang. Methods that perform I/O return errors as usual.
    pub trait Runtime: Send + Sync + Debug {
        /// Spawns the given asynchronous task to run in the background.
        fn spawn(
            &self,
            task: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
        ) -> BoxedTaskHandle;

        /// Creates and returns an instance of a DNSResolver, optionally
        /// configured by the ResolverOptions struct. This method may return an
        /// error if it fails to create the DNSResolver.
        fn get_dns_resolver(&self, opts: ResolverOptions) -> Result<Box<dyn DnsResolver>, String>;

        /// Returns a future that completes after the specified duration.
        fn sleep(&self, duration: std::time::Duration) -> Pin<Box<dyn Sleep>>;

        /// Returns the current time according to this runtime's clock.
        fn now(&self) -> Instant;

        /// Establishes a TCP connection to the given `target` address with the
        /// specified `opts`.
        fn tcp_stream(
            &self,
            target: SocketAddr,
            opts: TcpOptions,
        ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>>;

        /// Establishes a Unix connection to the given `path` with the specified
        /// `opts`.
        fn unix_stream(
            &self,
            path: PathBuf,
            opts: UnixSocketOptions,
        ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>> {
            Box::pin(async move {
                Err("Unix sockets are not supported by this runtime on this platform".to_string())
            })
        }

        /// Binds a TCP listener to the given address.
        fn tcp_listener(
            &self,
            addr: SocketAddr,
        ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>>;

        /// Binds a Unix listener to the given path.
        fn unix_listener(
            &self,
            path: PathBuf,
            opts: UnixSocketOptions,
        ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>>;
    }

    /// A trait for asynchronous DNS resolution.
    #[crate::async_trait]
    pub trait DnsResolver: Send + Sync {
        /// Resolve an address
        async fn lookup_host_name(&self, name: &str) -> Result<Vec<std::net::IpAddr>, String>;
        /// Perform a TXT record lookup. If a txt record contains multiple strings,
        /// they are concatenated.
        async fn lookup_txt(&self, name: &str) -> Result<Vec<String>, String>;
    }

    #[derive(Default)]
    pub struct ResolverOptions {
        /// The address of the DNS server in "IP:port" format. If None, the
        /// system's default DNS server will be used.
        pub(crate) server_addr: Option<std::net::SocketAddr>,
    }

    impl ResolverOptions {
        /// Sets the address of the DNS server to use. If `None`, the system's
        /// default DNS server will be used.
        pub fn with_server_addr(mut self, server_addr: Option<SocketAddr>) -> Self {
            self.server_addr = server_addr;
            self
        }

        /// Returns the address of the DNS server to use, or `None` if the system's
        /// default DNS server should be used.
        pub fn server_addr(&self) -> Option<SocketAddr> {
            self.server_addr
        }
    }

    #[derive(Default)]
    pub struct TcpOptions {
        pub(crate) enable_nodelay: bool,
        pub(crate) keepalive: Option<Duration>,
    }

    impl TcpOptions {
        /// Returns whether `TCP_NODELAY` should be set on the socket.
        pub fn enable_nodelay(&self) -> bool {
            self.enable_nodelay
        }

        /// Returns the TCP keepalive idle time to configure on the socket, or
        /// `None` if TCP keepalives should not be enabled.
        pub fn keepalive(&self) -> Option<Duration> {
            self.keepalive
        }
    }

    #[derive(Default)]
    pub struct UnixSocketOptions {
        _priv: (),
    }

    /// GrpcEndpoint is a generic stream-oriented network connection.
    // This trait is sealed since we may need to change the read and write
    // methods to align closely with the gRPC C++ implementations. For example,
    // the read method may be responsible for allocating the buffer and
    // returning it to enable in-place decryption. Since the libraries used
    // for http2 and channel credentials use AsyncRead, designing such an API
    // today would require adapters which would incur an extra copy, affecting
    // performance.
    pub trait GrpcEndpoint: Send + Unpin + 'static {
        /// Returns the local address that this stream is bound to.
        fn get_local_address(&self) -> &str;

        /// Returns the remote address that this stream is connected to.
        fn get_peer_address(&self) -> &str;

        /// Returns the network type of the connection (e.g., "tcp", "unix").
        fn get_network_type(&self) -> &'static str;

        #[doc(hidden)]
        fn poll_read_private(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>>;

        #[doc(hidden)]
        fn poll_write_private(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
            token: private::SealedToken,
        ) -> Poll<io::Result<usize>>;

        #[doc(hidden)]
        fn poll_flush_private(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>>;

        #[doc(hidden)]
        fn poll_shutdown_private(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>>;

        #[doc(hidden)]
        fn poll_write_vectored_private(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[IoSlice<'_>],
            token: private::SealedToken,
        ) -> Poll<io::Result<usize>> {
            let buf = bufs
                .iter()
                .find(|b| !b.is_empty())
                .map_or(&[][..], |b| &**b);
            self.poll_write_private(cx, buf, token)
        }

        #[doc(hidden)]
        fn is_write_vectored_private(&self, _: private::SealedToken) -> bool {
            false
        }
    }

    /// An adapter that exposes `AsyncRead` and `AsyncWrite` functionality for
    /// interfacing with `hyper` and `rustls`. This type is kept private to avoid
    /// exposing its read and write methods to external crates.
    pub struct EndpointIoStream<T> {
        inner: T,
    }

    impl<T: GrpcEndpoint> EndpointIoStream<T> {
        pub fn new(inner: T) -> Self {
            Self { inner }
        }

        pub fn get_ref(&self) -> &T {
            &self.inner
        }
    }

    impl<T: GrpcEndpoint> AsyncRead for EndpointIoStream<T> {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_read_private(cx, buf, private::SealedToken)
        }
    }

    impl<T: GrpcEndpoint> AsyncWrite for EndpointIoStream<T> {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.inner).poll_write_private(cx, buf, private::SealedToken)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_flush_private(cx, private::SealedToken)
        }

        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_shutdown_private(cx, private::SealedToken)
        }

        fn poll_write_vectored(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[IoSlice<'_>],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.inner).poll_write_vectored_private(cx, bufs, private::SealedToken)
        }

        fn is_write_vectored(&self) -> bool {
            self.inner.is_write_vectored_private(private::SealedToken)
        }
    }

    /// A wrapper that implements [GrpcEndpoint] for an asynchronous I/O stream.
    pub struct StreamEndpoint<T> {
        inner: T,
        peer_addr: Box<str>,
        local_addr: Box<str>,
        network_type: &'static str,
    }

    impl<T> StreamEndpoint<T> {
        pub fn new(
            inner: T,
            local_addr: Box<str>,
            peer_addr: Box<str>,
            network_type: &'static str,
        ) -> Self {
            Self {
                inner,
                peer_addr,
                local_addr,
                network_type,
            }
        }
    }

    impl<T: AsyncRead + AsyncWrite + Unpin + Send + 'static> GrpcEndpoint for StreamEndpoint<T> {
        fn get_local_address(&self) -> &str {
            &self.local_addr
        }

        fn get_peer_address(&self) -> &str {
            &self.peer_addr
        }

        fn get_network_type(&self) -> &'static str {
            self.network_type
        }

        fn poll_read_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
            _token: private::SealedToken,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_read(cx, buf)
        }

        fn poll_write_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
            _token: private::SealedToken,
        ) -> Poll<Result<usize, io::Error>> {
            Pin::new(&mut self.inner).poll_write(cx, buf)
        }

        fn poll_write_vectored_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[io::IoSlice<'_>],
            _token: private::SealedToken,
        ) -> Poll<Result<usize, io::Error>> {
            Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
        }

        fn is_write_vectored_private(&self, _token: private::SealedToken) -> bool {
            self.inner.is_write_vectored()
        }

        fn poll_flush_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            _token: private::SealedToken,
        ) -> Poll<Result<(), io::Error>> {
            Pin::new(&mut self.inner).poll_flush(cx)
        }

        fn poll_shutdown_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            _token: private::SealedToken,
        ) -> Poll<Result<(), io::Error>> {
            Pin::new(&mut self.inner).poll_shutdown(cx)
        }
    }

    impl GrpcEndpoint for Box<dyn GrpcEndpoint> {
        fn get_local_address(&self) -> &str {
            (**self).get_local_address()
        }

        fn get_peer_address(&self) -> &str {
            (**self).get_peer_address()
        }

        fn get_network_type(&self) -> &'static str {
            (**self).get_network_type()
        }

        fn poll_read_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut **self).poll_read_private(cx, buf, token)
        }

        fn poll_write_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
            token: private::SealedToken,
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_write_private(cx, buf, token)
        }

        fn poll_flush_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut **self).poll_flush_private(cx, token)
        }

        fn poll_shutdown_private(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            token: private::SealedToken,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut **self).poll_shutdown_private(cx, token)
        }
    }
}
