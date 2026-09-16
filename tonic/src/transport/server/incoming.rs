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

use std::{
    collections::HashSet,
    fmt,
    net::{IpAddr, SocketAddr, TcpListener as StdTcpListener},
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
    time::Duration,
};

use socket2::TcpKeepalive;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{Stream, wrappers::TcpListenerStream};
use tracing::{trace, warn};

/// A filter consulted for every newly accepted TCP connection, before any
/// TLS handshake or service dispatch. Returning `false` closes the
/// connection immediately.
pub(crate) type AcceptFilter = Arc<dyn Fn(SocketAddr) -> bool + Send + Sync>;

/// A runtime-mutable allowlist of peer IP addresses, usable as an
/// [`Server::accept_filter`](super::Server::accept_filter).
///
/// Unlike a plain closure, the set of allowed IPs can be changed while the
/// server is running (e.g. from a signal handler starting a graceful drain,
/// or from an admin endpoint), with updates taking effect on the very next
/// accepted connection.
///
/// # Example
///
/// ```
/// # use tonic::transport::server::DynamicAllowlist;
/// let allowlist = DynamicAllowlist::new(["10.0.0.5".parse().unwrap()]);
///
/// // Elsewhere at runtime, e.g. on SIGTERM:
/// allowlist.insert("10.0.0.6".parse().unwrap());
/// allowlist.remove("10.0.0.5".parse().unwrap());
/// ```
#[derive(Clone, Default)]
pub struct DynamicAllowlist(Arc<RwLock<HashSet<IpAddr>>>);

impl fmt::Debug for DynamicAllowlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynamicAllowlist")
            .field(&self.0.read().unwrap())
            .finish()
    }
}

impl DynamicAllowlist {
    /// Creates an allowlist containing the given initial set of IPs.
    pub fn new(initial: impl IntoIterator<Item = IpAddr>) -> Self {
        Self(Arc::new(RwLock::new(initial.into_iter().collect())))
    }

    /// Replaces the allowlist with the given set of IPs.
    pub fn set(&self, ips: impl IntoIterator<Item = IpAddr>) {
        *self.0.write().unwrap() = ips.into_iter().collect();
    }

    /// Adds a single IP to the allowlist.
    pub fn insert(&self, ip: IpAddr) {
        self.0.write().unwrap().insert(ip);
    }

    /// Removes a single IP from the allowlist.
    pub fn remove(&self, ip: IpAddr) {
        self.0.write().unwrap().remove(&ip);
    }

    /// Returns `true` if `addr`'s IP is currently in the allowlist.
    pub fn allows(&self, addr: SocketAddr) -> bool {
        self.0.read().unwrap().contains(&addr.ip())
    }

    /// Turns this allowlist into a filter closure usable with
    /// [`Server::accept_filter`](super::Server::accept_filter).
    pub fn into_filter(self) -> impl Fn(SocketAddr) -> bool + Send + Sync + 'static {
        move |addr| self.allows(addr)
    }
}

/// Binds a socket address for a [Router](super::Router)
///
/// An incoming stream, usable with [Router::serve_with_incoming](super::Router::serve_with_incoming),
/// of `AsyncRead + AsyncWrite` that communicate with clients that connect to a socket address.
pub struct TcpIncoming {
    inner: TcpListenerStream,
    nodelay: Option<bool>,
    keepalive: Option<TcpKeepalive>,
    keepalive_time: Option<Duration>,
    keepalive_interval: Option<Duration>,
    keepalive_retries: Option<u32>,
    accept_filter: Option<AcceptFilter>,
}

impl fmt::Debug for TcpIncoming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpIncoming")
            .field("inner", &self.inner)
            .field("nodelay", &self.nodelay)
            .field("keepalive", &self.keepalive)
            .field("keepalive_time", &self.keepalive_time)
            .field("keepalive_interval", &self.keepalive_interval)
            .field("keepalive_retries", &self.keepalive_retries)
            .field("accept_filter", &self.accept_filter.as_ref().map(|_| ".."))
            .finish()
    }
}

impl TcpIncoming {
    /// Creates an instance by binding (opening) the specified socket address.
    ///
    /// Returns a TcpIncoming if the socket address was successfully bound.
    ///
    /// # Examples
    /// ```no_run
    /// # use tower_service::Service;
    /// # use http::{request::Request, response::Response};
    /// # use tonic::{body::Body, server::NamedService, transport::{Server, server::TcpIncoming}};
    /// # use core::convert::Infallible;
    /// # use std::error::Error;
    /// # fn main() { }  // Cannot have type parameters, hence instead define:
    /// # fn run<S>(some_service: S) -> Result<(), Box<dyn Error + Send + Sync>>
    /// # where
    /// #   S: Service<Request<Body>, Response = Response<Body>, Error = Infallible> + NamedService + Clone + Send + Sync + 'static,
    /// #   S::Future: Send + 'static,
    /// # {
    /// // Find a free port
    /// let mut port = 1322;
    /// let tinc = loop {
    ///    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    ///    match TcpIncoming::bind(addr) {
    ///       Ok(t) => break t,
    ///       Err(_) => port += 1
    ///    }
    /// };
    /// Server::builder()
    ///    .add_service(some_service)
    ///    .serve_with_incoming(tinc);
    /// # Ok(())
    /// # }
    pub fn bind(addr: SocketAddr) -> std::io::Result<Self> {
        let std_listener = StdTcpListener::bind(addr)?;
        std_listener.set_nonblocking(true)?;

        Ok(TcpListener::from_std(std_listener)?.into())
    }

    /// Sets the `TCP_NODELAY` option on the accepted connection.
    pub fn with_nodelay(self, nodelay: Option<bool>) -> Self {
        Self { nodelay, ..self }
    }

    /// Sets the `TCP_KEEPALIVE` option on the accepted connection.
    pub fn with_keepalive(self, keepalive_time: Option<Duration>) -> Self {
        Self {
            keepalive_time,
            keepalive: make_keepalive(
                keepalive_time,
                self.keepalive_interval,
                self.keepalive_retries,
            ),
            ..self
        }
    }

    /// Sets the `TCP_KEEPINTVL` option on the accepted connection.
    pub fn with_keepalive_interval(self, keepalive_interval: Option<Duration>) -> Self {
        Self {
            keepalive_interval,
            keepalive: make_keepalive(
                self.keepalive_time,
                keepalive_interval,
                self.keepalive_retries,
            ),
            ..self
        }
    }

    /// Sets the `TCP_KEEPCNT` option on the accepted connection.
    pub fn with_keepalive_retries(self, keepalive_retries: Option<u32>) -> Self {
        Self {
            keepalive_retries,
            keepalive: make_keepalive(
                self.keepalive_time,
                self.keepalive_interval,
                keepalive_retries,
            ),
            ..self
        }
    }

    /// Returns the local address that this tcp incoming is bound to.
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.as_ref().local_addr()
    }

    /// Only accept connections from peers for which `filter` returns `true`.
    /// Rejected connections are closed immediately, before any TLS handshake
    /// or service dispatch.
    pub(crate) fn with_accept_filter(self, filter: Option<AcceptFilter>) -> Self {
        Self {
            accept_filter: filter,
            ..self
        }
    }
}

impl From<TcpListener> for TcpIncoming {
    fn from(listener: TcpListener) -> Self {
        Self {
            inner: TcpListenerStream::new(listener),
            nodelay: None,
            keepalive: None,
            keepalive_time: None,
            keepalive_interval: None,
            keepalive_retries: None,
            accept_filter: None,
        }
    }
}

impl Stream for TcpIncoming {
    type Item = std::io::Result<TcpStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let polled = Pin::new(&mut self.inner).poll_next(cx);

            let Poll::Ready(Some(Ok(stream))) = &polled else {
                return polled;
            };

            if let Some(filter) = &self.accept_filter
                && let Ok(peer_addr) = stream.peer_addr()
                && !filter(peer_addr)
            {
                trace!("rejecting connection from {peer_addr} via accept_filter");
                continue;
            }

            set_accepted_socket_options(stream, self.nodelay, &self.keepalive);
            return polled;
        }
    }
}

// Consistent with hyper-0.14, this function does not return an error.
fn set_accepted_socket_options(
    stream: &TcpStream,
    nodelay: Option<bool>,
    keepalive: &Option<TcpKeepalive>,
) {
    if let Some(nodelay) = nodelay
        && let Err(e) = stream.set_nodelay(nodelay)
    {
        warn!("error trying to set TCP_NODELAY: {e}");
    }

    if let Some(keepalive) = keepalive {
        let sock_ref = socket2::SockRef::from(&stream);
        if let Err(e) = sock_ref.set_tcp_keepalive(keepalive) {
            warn!("error trying to set TCP_KEEPALIVE: {e}");
        }
    }
}

fn make_keepalive(
    keepalive_time: Option<Duration>,
    keepalive_interval: Option<Duration>,
    keepalive_retries: Option<u32>,
) -> Option<TcpKeepalive> {
    let mut dirty = false;
    let mut keepalive = TcpKeepalive::new();
    if let Some(t) = keepalive_time {
        keepalive = keepalive.with_time(t);
        dirty = true;
    }

    #[cfg(
        // See https://docs.rs/socket2/0.5.8/src/socket2/lib.rs.html#511-525
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "visionos",
            target_os = "linux",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "windows",
        )
    )]
    if let Some(t) = keepalive_interval {
        keepalive = keepalive.with_interval(t);
        dirty = true;
    }

    #[cfg(
        // See https://docs.rs/socket2/0.5.8/src/socket2/lib.rs.html#557-570
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "visionos",
            target_os = "linux",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    if let Some(r) = keepalive_retries {
        keepalive = keepalive.with_retries(r);
        dirty = true;
    }

    // avoid clippy errors for targets that do not use these fields.
    let _ = keepalive_retries;
    let _ = keepalive_interval;

    dirty.then_some(keepalive)
}

#[cfg(test)]
mod tests {
    use super::DynamicAllowlist;
    use crate::transport::server::TcpIncoming;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_stream::StreamExt as _;

    #[tokio::test]
    async fn one_tcpincoming_at_a_time() {
        let addr = "127.0.0.1:1322".parse().unwrap();
        {
            let _t1 = TcpIncoming::bind(addr).unwrap();
            let _t2 = TcpIncoming::bind(addr).unwrap_err();
        }
        let _t3 = TcpIncoming::bind(addr).unwrap();
    }

    #[tokio::test]
    async fn accept_filter_rejects_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let mut incoming =
            TcpIncoming::from(listener).with_accept_filter(Some(std::sync::Arc::new(|_| false)));

        let _client = TcpStream::connect(addr).await.unwrap();

        // The connection is dropped by the filter, so the listener keeps
        // waiting for a connection that never arrives (within the timeout).
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(200), incoming.next()).await;
        assert!(
            result.is_err(),
            "filtered-out connection should not be yielded"
        );
    }

    #[tokio::test]
    async fn accept_filter_allows_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let mut incoming =
            TcpIncoming::from(listener).with_accept_filter(Some(std::sync::Arc::new(|_| true)));

        let _client = TcpStream::connect(addr).await.unwrap();

        let result =
            tokio::time::timeout(std::time::Duration::from_millis(200), incoming.next()).await;
        assert!(result.unwrap().unwrap().is_ok());
    }

    #[tokio::test]
    async fn dynamic_allowlist_updates_take_effect_immediately() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let allowlist = DynamicAllowlist::new(["127.0.0.1".parse().unwrap()]);
        let mut incoming =
            TcpIncoming::from(listener).with_accept_filter(Some(std::sync::Arc::new({
                let allowlist = allowlist.clone();
                move |peer| allowlist.allows(peer)
            })));

        let _client1 = TcpStream::connect(addr).await.unwrap();
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(200), incoming.next()).await;
        assert!(
            result.unwrap().unwrap().is_ok(),
            "127.0.0.1 should be allowed initially"
        );

        allowlist.remove("127.0.0.1".parse().unwrap());

        let _client2 = TcpStream::connect(addr).await.unwrap();
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(200), incoming.next()).await;
        assert!(
            result.is_err(),
            "127.0.0.1 should be rejected after removal from the allowlist"
        );
    }
}
