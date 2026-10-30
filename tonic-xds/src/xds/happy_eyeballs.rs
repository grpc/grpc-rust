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

//! Happy Eyeballs connector for endpoints with multiple addresses (gRFC A61).
//!
//! An xDS endpoint may have several addresses, typically one IPv4 and one
//! IPv6. [`HappyEyeballsConnector`] races connections to them as described in
//! gRFC A61 and [RFC 8305]:
//!
//! - Addresses are de-duplicated, then interleaved by address family starting
//!   with the family of the first address (RFC 8305 §4, with a First Address
//!   Family Count of 1). Hostnames form a family of their own.
//! - Attempts start one at a time, in that order. The next one starts when
//!   [`CONNECTION_ATTEMPT_DELAY`] passes or as soon as the most recently
//!   started attempt fails, whichever comes first. Earlier attempts keep
//!   running.
//! - The first connection established wins; the other attempts are dropped.
//! - If every attempt fails, the error carries the most recent failure.
//!
//! With [`HappyEyeballsConnector::new`], an attempt covers only the TCP
//! connection: tonic layers TLS and HTTP/2 on top of the winning stream, so
//! a TLS or HTTP/2 failure does not fall back to the other addresses.
//! Reconnecting after a failure is also left to tonic, which retries on the
//! next request without backoff.
//!
//! [RFC 8305]: https://www.rfc-editor.org/rfc/rfc8305

use crate::client::endpoint::EndpointAddress;
use crate::common::async_util::BoxFuture;
use futures_util::future::poll_fn;
use futures_util::stream::{FuturesUnordered, StreamExt};
use http::Uri;
use hyper_util::client::legacy::connect::HttpConnector;
use std::collections::HashSet;
use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{Instant, Sleep};
use tower::{BoxError, Service};

/// How long to wait for an attempt before starting the next one in parallel
/// (the "Connection Attempt Delay" recommended in RFC 8305 §8).
pub(crate) const CONNECTION_ATTEMPT_DELAY: Duration = Duration::from_millis(250);

/// Error returned by [`HappyEyeballsConnector`].
#[derive(Debug, thiserror::Error)]
pub(crate) enum HappyEyeballsError {
    /// The endpoint has no addresses to connect to.
    #[error("no addresses to connect to")]
    NoAddresses,
    /// Every attempt failed; `error` is the most recent failure.
    ///
    /// The message includes `error` because tonic puts only the top-level
    /// message into the RPC status. It is also the `source()`, so the retry
    /// layer can still find the underlying I/O error.
    #[error("failed to connect to all addresses; last error: {addr}: {error}")]
    AllAttemptsFailed {
        addr: EndpointAddress,
        #[source]
        error: BoxError,
    },
}

/// A [`Service<Uri>`] that connects to one endpoint by racing its addresses.
///
/// The `Uri` passed to [`Service::call`] is ignored: each attempt dials
/// `http://{addr}` through the inner connector. It is meant to be used with
/// [`tonic::transport::Endpoint::connect_with_connector_lazy`], which adds TLS
/// (if configured) on top of the returned stream, using the `Endpoint` URI
/// for the authority and SNI.
#[derive(Clone, Debug)]
pub(crate) struct HappyEyeballsConnector<C = HttpConnector> {
    /// De-duplicated and interleaved; see [`sort_addresses`].
    addrs: Arc<[EndpointAddress]>,
    inner: C,
}

impl HappyEyeballsConnector {
    /// Creates a connector that dials `addrs` over TCP.
    pub(crate) fn new(addrs: impl IntoIterator<Item = EndpointAddress>) -> Self {
        // Same as tonic's default connector. No per-attempt timeout: tonic's
        // `connect_timeout` bounds the whole race.
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(true);
        Self::with_connector(addrs, http)
    }
}

impl<C> HappyEyeballsConnector<C> {
    /// Creates a connector that dials each of `addrs` through `inner`.
    pub(crate) fn with_connector(
        addrs: impl IntoIterator<Item = EndpointAddress>,
        inner: C,
    ) -> Self {
        Self {
            addrs: sort_addresses(addrs).into(),
            inner,
        }
    }
}

impl<C> Service<Uri> for HappyEyeballsConnector<C>
where
    C: Service<Uri> + Clone + Send + 'static,
    C::Response: Send + 'static,
    C::Error: Into<BoxError>,
    C::Future: Send + 'static,
{
    type Response = C::Response;
    type Error = BoxError;
    type Future = BoxFuture<Result<C::Response, BoxError>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Each attempt waits for its own clone of `inner` to be ready.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _dst: Uri) -> Self::Future {
        let inner = self.inner.clone();
        let addrs = self.addrs.clone();
        Box::pin(async move {
            let mut race = Race::new(inner, addrs);
            poll_fn(|cx| race.poll_connect(cx)).await
        })
    }
}

type Attempt<T> = BoxFuture<(usize, Result<T, BoxError>)>;

/// One pass over an endpoint's addresses.
struct Race<C: Service<Uri>> {
    inner: C,
    addrs: Arc<[EndpointAddress]>,
    /// Index of the next address to try. Attempts `0..next` have started.
    next: usize,
    attempts: FuturesUnordered<Attempt<C::Response>>,
    /// Fires [`CONNECTION_ATTEMPT_DELAY`] after the latest attempt started.
    timer: Pin<Box<Sleep>>,
    /// Index and error of the most recent failure.
    last_error: Option<(usize, BoxError)>,
}

impl<C> Race<C>
where
    C: Service<Uri> + Clone + Send + 'static,
    C::Response: Send + 'static,
    C::Error: Into<BoxError>,
    C::Future: Send + 'static,
{
    fn new(inner: C, addrs: Arc<[EndpointAddress]>) -> Self {
        let mut race = Self {
            inner,
            addrs,
            next: 0,
            attempts: FuturesUnordered::new(),
            timer: Box::pin(tokio::time::sleep(CONNECTION_ATTEMPT_DELAY)),
            last_error: None,
        };
        if !race.addrs.is_empty() {
            race.start_next_attempt();
        }
        race
    }

    fn start_next_attempt(&mut self) {
        let idx = self.next;
        self.next += 1;
        self.attempts
            .push(attempt(self.inner.clone(), idx, &self.addrs[idx]));
        self.timer
            .as_mut()
            .reset(Instant::now() + CONNECTION_ATTEMPT_DELAY);
    }

    fn poll_connect(&mut self, cx: &mut Context<'_>) -> Poll<Result<C::Response, BoxError>> {
        loop {
            match self.attempts.poll_next_unpin(cx) {
                Poll::Ready(Some((_, Ok(conn)))) => return Poll::Ready(Ok(conn)),
                Poll::Ready(Some((idx, Err(error)))) => {
                    tracing::debug!(addr = %self.addrs[idx], %error, "connection attempt failed");
                    self.last_error = Some((idx, error));
                    // Only a failure of the latest attempt starts the next one
                    // early.
                    if idx + 1 == self.next && self.next < self.addrs.len() {
                        self.start_next_attempt();
                    }
                    continue;
                }
                Poll::Ready(None) => {
                    // Every started attempt has failed. The latest attempt's
                    // failure always starts the next one, so none are left.
                    let error = match self.last_error.take() {
                        Some((idx, error)) => HappyEyeballsError::AllAttemptsFailed {
                            addr: self.addrs[idx].clone(),
                            error,
                        },
                        None => HappyEyeballsError::NoAddresses,
                    };
                    return Poll::Ready(Err(error.into()));
                }
                Poll::Pending => {}
            }
            if self.next < self.addrs.len() && self.timer.as_mut().poll(cx).is_ready() {
                self.start_next_attempt();
                continue;
            }
            return Poll::Pending;
        }
    }
}

/// Connects to `addr` through `inner`, tagging the result with `idx`.
fn attempt<C>(mut inner: C, idx: usize, addr: &EndpointAddress) -> Attempt<C::Response>
where
    C: Service<Uri> + Send + 'static,
    C::Error: Into<BoxError>,
    C::Future: Send + 'static,
{
    let uri = format!("http://{addr}").parse::<Uri>();
    Box::pin(async move {
        let result: Result<C::Response, BoxError> = async {
            let uri = uri?;
            poll_fn(|cx| inner.poll_ready(cx))
                .await
                .map_err(Into::into)?;
            inner.call(uri).await.map_err(Into::into)
        }
        .await;
        (idx, result)
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Family {
    V4,
    V6,
    Hostname,
}

fn family(addr: &EndpointAddress) -> Family {
    match addr.ip() {
        Some(IpAddr::V4(_)) => Family::V4,
        // IPv4-mapped IPv6 addresses connect over IPv4.
        Some(IpAddr::V6(ip)) if ip.to_ipv4_mapped().is_some() => Family::V4,
        Some(IpAddr::V6(_)) => Family::V6,
        None => Family::Hostname,
    }
}

/// De-duplicates `addrs`, keeping first occurrences, and interleaves them by
/// address family. Families take turns in order of first appearance.
fn sort_addresses(addrs: impl IntoIterator<Item = EndpointAddress>) -> Vec<EndpointAddress> {
    let mut seen = HashSet::new();
    let mut groups: Vec<(Family, Vec<EndpointAddress>)> = Vec::new();
    for addr in addrs.into_iter().filter(|addr| seen.insert(addr.clone())) {
        let f = family(&addr);
        match groups.iter_mut().find(|(g, _)| *g == f) {
            Some((_, group)) => group.push(addr),
            None => groups.push((f, vec![addr])),
        }
    }

    // The k-th address of every family, for k = 0, 1, ...
    // We want to interleave IPv4 and IPv6 addresses per RFC 8305
    let longest = groups.iter().map(|(_, g)| g.len()).max().unwrap_or(0);
    (0..longest)
        .flat_map(|k| groups.iter().filter_map(move |(_, group)| group.get(k)))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::retry::is_retryable_connection_error;
    use crate::testutil::grpc::{GreeterClient, HelloRequest, spawn_greeter_server};
    use std::net::SocketAddr;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::net::TcpListener;
    use tonic::transport::Endpoint;

    fn addr(s: &str) -> EndpointAddress {
        s.parse::<SocketAddr>().unwrap().into()
    }

    fn sorted(addrs: &[EndpointAddress]) -> Vec<String> {
        sort_addresses(addrs.iter().cloned())
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn interleaves_families_starting_with_first_address() {
        let addrs = [
            addr("[2001:db8::1]:80"),
            addr("[2001:db8::2]:80"),
            addr("[2001:db8::3]:80"),
            addr("10.0.0.1:80"),
            addr("10.0.0.2:80"),
        ];
        assert_eq!(
            sorted(&addrs),
            [
                "[2001:db8::1]:80",
                "10.0.0.1:80",
                "[2001:db8::2]:80",
                "10.0.0.2:80",
                "[2001:db8::3]:80",
            ]
        );
    }

    #[test]
    fn keeps_order_within_single_family() {
        let addrs = [
            addr("10.0.0.2:80"),
            addr("10.0.0.1:80"),
            addr("10.0.0.3:80"),
        ];
        assert_eq!(
            sorted(&addrs),
            ["10.0.0.2:80", "10.0.0.1:80", "10.0.0.3:80"]
        );
    }

    #[test]
    fn hostnames_are_their_own_family() {
        let addrs = [
            EndpointAddress::new("a.example.com", 80),
            EndpointAddress::new("b.example.com", 80),
            addr("10.0.0.1:80"),
            addr("10.0.0.2:80"),
            addr("[2001:db8::1]:80"),
        ];
        assert_eq!(
            sorted(&addrs),
            [
                "a.example.com:80",
                "10.0.0.1:80",
                "[2001:db8::1]:80",
                "b.example.com:80",
                "10.0.0.2:80",
            ]
        );
    }

    #[test]
    fn ipv4_mapped_ipv6_counts_as_ipv4() {
        let addrs = [
            addr("10.0.0.1:80"),
            addr("[::ffff:10.0.0.2]:80"),
            addr("[2001:db8::1]:80"),
        ];
        assert_eq!(
            sorted(&addrs),
            ["10.0.0.1:80", "[2001:db8::1]:80", "[::ffff:10.0.0.2]:80"]
        );
    }

    #[test]
    fn drops_duplicates_keeping_first_occurrence() {
        let addrs = [
            addr("10.0.0.1:80"),
            addr("[2001:db8::1]:80"),
            addr("10.0.0.1:80"),
            addr("10.0.0.2:80"),
            addr("[2001:db8::1]:80"),
        ];
        assert_eq!(
            sorted(&addrs),
            ["10.0.0.1:80", "[2001:db8::1]:80", "10.0.0.2:80"]
        );
    }

    /// Scripted result of a connection attempt.
    #[derive(Clone, Copy)]
    enum Outcome {
        /// Connects after this many milliseconds.
        Connect(u64),
        /// Fails after this many milliseconds.
        Fail(u64),
        /// Never completes.
        Hang,
    }
    use Outcome::*;

    type Script = &'static [(&'static str, Outcome)];

    /// Connector that plays back a script of per-address outcomes and records
    /// when each attempt starts.
    #[derive(Clone)]
    struct MockConnector {
        script: Script,
        started: Arc<Mutex<Vec<(&'static str, u64)>>>,
        in_flight: Arc<AtomicUsize>,
        t0: Instant,
    }

    impl MockConnector {
        fn new(script: Script) -> Self {
            Self {
                script,
                started: Default::default(),
                in_flight: Default::default(),
                t0: Instant::now(),
            }
        }

        /// Races the scripted addresses, in script order.
        async fn race(&self) -> Result<&'static str, BoxError> {
            let addrs = self.script.iter().map(|(a, _)| addr(a));
            HappyEyeballsConnector::with_connector(addrs, self.clone())
                .call(Uri::from_static("http://ignored"))
                .await
        }

        /// Address and start time in milliseconds of each attempt, in order.
        fn started(&self) -> Vec<(&'static str, u64)> {
            self.started.lock().unwrap().clone()
        }

        /// Attempts that have started and are still pending.
        fn in_flight(&self) -> usize {
            self.in_flight.load(Ordering::SeqCst)
        }
    }

    struct InFlightGuard(Arc<AtomicUsize>);

    impl Drop for InFlightGuard {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }

    impl Service<Uri> for MockConnector {
        type Response = &'static str;
        type Error = BoxError;
        type Future = BoxFuture<Result<&'static str, BoxError>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), BoxError>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, uri: Uri) -> Self::Future {
            let authority = uri.authority().unwrap().as_str();
            let (addr, outcome) = *self.script.iter().find(|(a, _)| *a == authority).unwrap();
            let elapsed = self.t0.elapsed().as_millis() as u64;
            self.started.lock().unwrap().push((addr, elapsed));
            self.in_flight.fetch_add(1, Ordering::SeqCst);
            let guard = InFlightGuard(self.in_flight.clone());
            Box::pin(async move {
                let _guard = guard;
                match outcome {
                    Connect(ms) => {
                        tokio::time::sleep(Duration::from_millis(ms)).await;
                        Ok(addr)
                    }
                    Fail(ms) => {
                        tokio::time::sleep(Duration::from_millis(ms)).await;
                        Err("connection refused".into())
                    }
                    Hang => std::future::pending().await,
                }
            })
        }
    }

    #[tokio::test(start_paused = true)]
    async fn first_address_connecting_in_time_is_the_only_attempt() {
        let mock =
            MockConnector::new(&[("10.0.0.1:80", Connect(200)), ("10.0.0.2:80", Connect(0))]);

        assert_eq!(mock.race().await.unwrap(), "10.0.0.1:80");
        assert_eq!(mock.started(), [("10.0.0.1:80", 0)]);
    }

    #[tokio::test(start_paused = true)]
    async fn starts_next_attempt_after_delay_and_drops_losers() {
        let mock = MockConnector::new(&[("10.0.0.1:80", Hang), ("10.0.0.2:80", Connect(10))]);

        assert_eq!(mock.race().await.unwrap(), "10.0.0.2:80");
        assert_eq!(mock.started(), [("10.0.0.1:80", 0), ("10.0.0.2:80", 250)]);
        assert_eq!(mock.in_flight(), 0, "losing attempt must be cancelled");
    }

    #[tokio::test(start_paused = true)]
    async fn latest_attempt_failing_starts_next_immediately() {
        let mock = MockConnector::new(&[
            ("10.0.0.1:80", Fail(10)),
            ("10.0.0.2:80", Hang),
            ("10.0.0.3:80", Fail(100)),
            ("10.0.0.4:80", Hang),
            ("10.0.0.5:80", Connect(0)),
        ]);

        assert_eq!(mock.race().await.unwrap(), "10.0.0.5:80");
        // .3 fails at 360 and starts .4 at once, which restarts the delay
        // timer for .5.
        assert_eq!(
            mock.started(),
            [
                ("10.0.0.1:80", 0),
                ("10.0.0.2:80", 10),
                ("10.0.0.3:80", 260),
                ("10.0.0.4:80", 360),
                ("10.0.0.5:80", 610),
            ]
        );
        assert_eq!(mock.in_flight(), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn earlier_attempt_failing_does_not_start_next() {
        let mock = MockConnector::new(&[
            ("10.0.0.1:80", Fail(300)),
            ("10.0.0.2:80", Hang),
            ("10.0.0.3:80", Connect(0)),
        ]);

        assert_eq!(mock.race().await.unwrap(), "10.0.0.3:80");
        assert_eq!(
            mock.started(),
            [
                ("10.0.0.1:80", 0),
                ("10.0.0.2:80", 250),
                ("10.0.0.3:80", 500)
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn earlier_attempt_can_still_win() {
        let mock = MockConnector::new(&[
            ("10.0.0.1:80", Connect(400)),
            ("10.0.0.2:80", Hang),
            ("10.0.0.3:80", Hang),
        ]);

        assert_eq!(mock.race().await.unwrap(), "10.0.0.1:80");
        assert_eq!(mock.started(), [("10.0.0.1:80", 0), ("10.0.0.2:80", 250)]);
        assert_eq!(mock.in_flight(), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn attempts_follow_interleaved_order() {
        let mock = MockConnector::new(&[
            ("[2001:db8::1]:80", Fail(0)),
            ("[2001:db8::2]:80", Connect(0)),
            ("10.0.0.1:80", Fail(0)),
        ]);

        assert_eq!(mock.race().await.unwrap(), "[2001:db8::2]:80");
        assert_eq!(
            mock.started(),
            [
                ("[2001:db8::1]:80", 0),
                ("10.0.0.1:80", 0),
                ("[2001:db8::2]:80", 0)
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn all_attempts_failing_reports_most_recent_failure() {
        let mock = MockConnector::new(&[("10.0.0.1:80", Fail(300)), ("10.0.0.2:80", Fail(10))]);

        let err = mock.race().await.unwrap_err();
        // .2 fails at 260, .1 at 300.
        assert_eq!(
            err.to_string(),
            "failed to connect to all addresses; last error: 10.0.0.1:80: connection refused"
        );
        let source = std::error::Error::source(err.as_ref()).unwrap();
        assert_eq!(source.to_string(), "connection refused");
        assert_eq!(mock.started(), [("10.0.0.1:80", 0), ("10.0.0.2:80", 250)]);
    }

    #[tokio::test(start_paused = true)]
    async fn no_addresses_fails() {
        let err = HappyEyeballsConnector::with_connector([], MockConnector::new(&[]))
            .call(Uri::from_static("http://ignored"))
            .await
            .unwrap_err();
        assert!(matches!(
            err.downcast_ref::<HappyEyeballsError>(),
            Some(HappyEyeballsError::NoAddresses)
        ));
    }

    /// Returns `N` distinct loopback addresses that fail to connect
    /// immediately.
    fn closed_addrs<const N: usize>() -> [SocketAddr; N] {
        std::array::from_fn(|i| SocketAddr::from(([127, 0, 0, i as u8 + 1], 0)))
    }

    #[tokio::test]
    async fn tcp_connects_past_closed_address() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let open = listener.local_addr().unwrap();
        let [closed] = closed_addrs();

        let conn = HappyEyeballsConnector::new([closed.into(), open.into()])
            .call(Uri::from_static("http://ignored"))
            .await
            .unwrap();
        assert_eq!(conn.inner().peer_addr().unwrap(), open);
    }

    #[tokio::test]
    async fn tcp_closed_everywhere_is_retryable() {
        let addrs = closed_addrs::<2>().map(Into::into);

        let err = HappyEyeballsConnector::new(addrs)
            .call(Uri::from_static("http://ignored"))
            .await
            .unwrap_err();
        assert!(
            is_retryable_connection_error(err.as_ref()),
            "the retry layer must still see the io::Error: {err:?}"
        );
    }

    #[tokio::test]
    async fn channel_serves_rpcs_through_reachable_address() {
        let server = spawn_greeter_server("hello", None, None).await.unwrap();
        let [closed] = closed_addrs();
        let connector = HappyEyeballsConnector::new([closed.into(), server.addr.into()]);
        let channel = Endpoint::from_shared(format!("http://{closed}"))
            .unwrap()
            .connect_with_connector_lazy(connector);

        let reply = GreeterClient::new(channel)
            .say_hello(HelloRequest { name: "xds".into() })
            .await
            .unwrap();
        assert_eq!(reply.into_inner().message, "hello: xds");
        let _ = server.shutdown.send(());
    }

    /// Attempts dial `http://{addr}`, but TLS is decided by the `Endpoint`:
    /// tonic runs the handshake on whichever stream wins the race.
    #[cfg(feature = "_tls-any")]
    #[tokio::test]
    async fn channel_runs_tls_over_winning_stream() {
        use tonic::transport::{Certificate, ClientTlsConfig, Identity, ServerTlsConfig};

        // With both TLS backends compiled in (`--all-features`), rustls
        // cannot pick a provider on its own.
        #[cfg(feature = "tls-ring")]
        let _ = rustls::crypto::ring::default_provider().install_default();
        #[cfg(all(not(feature = "tls-ring"), feature = "tls-aws-lc"))]
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let ca_key = rcgen::KeyPair::generate().unwrap();
        let mut ca_params = rcgen::CertificateParams::new(Vec::<String>::new()).unwrap();
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let ca_cert = ca_params.self_signed(&ca_key).unwrap();
        let key = rcgen::KeyPair::generate().unwrap();
        let cert = rcgen::CertificateParams::new(vec!["127.0.0.1".to_string()])
            .unwrap()
            .signed_by(&key, &rcgen::Issuer::from_params(&ca_params, &ca_key))
            .unwrap();
        let server_tls =
            ServerTlsConfig::new().identity(Identity::from_pem(cert.pem(), key.serialize_pem()));
        let client_tls =
            ClientTlsConfig::new().ca_certificate(Certificate::from_pem(ca_cert.pem()));

        let server = spawn_greeter_server("hello", Some(server_tls), Some(client_tls.clone()))
            .await
            .unwrap();
        let [closed] = closed_addrs();
        let connector = HappyEyeballsConnector::new([closed.into(), server.addr.into()]);
        let channel = Endpoint::from_shared(format!("https://{closed}"))
            .unwrap()
            .tls_config(client_tls)
            .unwrap()
            .connect_with_connector_lazy(connector);

        let reply = GreeterClient::new(channel)
            .say_hello(HelloRequest { name: "xds".into() })
            .await
            .unwrap();
        assert_eq!(reply.into_inner().message, "hello: xds");
        let _ = server.shutdown.send(());
    }

    #[tokio::test]
    async fn channel_reports_unavailable_when_all_addresses_fail() {
        let addrs = closed_addrs::<2>();
        let channel = Endpoint::from_shared(format!("http://{}", addrs[0]))
            .unwrap()
            .connect_with_connector_lazy(HappyEyeballsConnector::new(addrs.map(Into::into)));

        let status = GreeterClient::new(channel)
            .say_hello(HelloRequest { name: "xds".into() })
            .await
            .unwrap_err();
        assert_eq!(status.code(), tonic::Code::Unavailable);
        assert!(
            status
                .message()
                .starts_with("failed to connect to all addresses; last error: 127.0.0."),
            "{status:?}"
        );
    }
}
