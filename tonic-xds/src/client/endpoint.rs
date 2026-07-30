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
use crate::xds::resource::cluster::ClusterResource;
use crate::xds::resource::security::ClusterSecurityConfig;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::AtomicU64, atomic::Ordering};
use std::task::{Context, Poll};
use tower::{BoxError, Service, load::Load};

/// Represents the host part of an endpoint address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EndpointHost {
    Ipv4(std::net::Ipv4Addr),
    Ipv6(std::net::Ipv6Addr),
    Hostname(String),
}

impl From<String> for EndpointHost {
    fn from(s: String) -> Self {
        if let Ok(ipv4) = s.parse::<std::net::Ipv4Addr>() {
            EndpointHost::Ipv4(ipv4)
        } else if let Ok(ipv6) = s.parse::<std::net::Ipv6Addr>() {
            EndpointHost::Ipv6(ipv6)
        } else {
            EndpointHost::Hostname(s)
        }
    }
}

/// Represents a validated endpoint address extracted from xDS
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndpointAddress {
    /// The IP address or hostname
    host: EndpointHost,
    /// The port number
    port: u16,
}

impl EndpointAddress {
    /// Creates a new `EndpointAddress` from a host string and port.
    ///
    /// Attempts to parse the host as an IP address; falls back to hostname.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: EndpointHost::from(host.into()),
            port,
        }
    }
}

impl std::fmt::Display for EndpointAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.host {
            EndpointHost::Ipv4(ip) => write!(f, "{ip}:{}", self.port),
            EndpointHost::Ipv6(ip) => write!(f, "[{ip}]:{}", self.port),
            EndpointHost::Hostname(h) => write!(f, "{h}:{}", self.port),
        }
    }
}

impl From<SocketAddr> for EndpointAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(v4_addr) => Self {
                host: EndpointHost::Ipv4(*v4_addr.ip()),
                port: v4_addr.port(),
            },
            SocketAddr::V6(v6_addr) => Self {
                host: EndpointHost::Ipv6(*v6_addr.ip()),
                port: v6_addr.port(),
            },
        }
    }
}

/// RAII tracker for in-flight requests.
/// This is mainly used to implement endpoint load reporting for load balancing purposes.
#[derive(Clone, Debug, Default)]
struct InFlightTracker {
    in_flight: Arc<AtomicU64>,
}

impl InFlightTracker {
    fn new(in_flight: Arc<AtomicU64>) -> Self {
        in_flight.fetch_add(1, Ordering::Relaxed);
        Self { in_flight }
    }
}

impl Drop for InFlightTracker {
    fn drop(&mut self) {
        self.in_flight.fetch_sub(1, Ordering::Relaxed);
    }
}

/// An endpoint channel for communicating with a single gRPC endpoint, with load reporting support for load balancing.
pub struct EndpointChannel<S> {
    inner: S,
    in_flight: Arc<AtomicU64>,
}

impl<S> EndpointChannel<S> {
    /// Creates a new `EndpointChannel`.
    /// This should be used by xDS implementations to construct channels to individual endpoints.
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            in_flight: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl<S> std::fmt::Debug for EndpointChannel<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EndpointChannel")
            .field("in_flight", &self.in_flight.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl<S> Clone for EndpointChannel<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            in_flight: self.in_flight.clone(),
        }
    }
}

impl<S, Req> Service<Req> for EndpointChannel<S>
where
    S: Service<Req> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<Result<S::Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let in_flight = InFlightTracker::new(self.in_flight.clone());
        let fut = self.inner.call(req);

        // -1 when the inner future completes
        Box::pin(async move {
            let _in_flight_guard = in_flight;

            fut.await
        })
    }
}

impl<S> Load for EndpointChannel<S> {
    type Metric = u64;
    fn load(&self) -> Self::Metric {
        self.in_flight.load(Ordering::Relaxed)
    }
}

/// Factory for creating connections to endpoints.
///
/// Implementations capture cluster-level config (TLS, HTTP/2 settings, timeouts)
/// at construction time. The implementation handles retries and concurrency
/// internally — the returned future resolves when a connection is established
/// (or is cancelled by dropping).
pub trait Connector {
    /// The service type produced by this connector.
    type Service;

    /// Connect to the given endpoint address.
    fn connect(
        &self,
        addr: &EndpointAddress,
    ) -> crate::common::async_util::BoxFuture<Self::Service>;
}

/// A read-only view of a cluster's parsed xDS configuration, handed to
/// [`MakeConnector::make_connector`] so a factory can build a connector
/// tailored to the cluster.
///
/// The view is intentionally opaque: it currently exposes only the cluster
/// name. Read access to the parsed TLS/security settings a custom connector
/// needs for gRFC A29 parity is exposed separately (and is not required to
/// build a plaintext connector).
pub struct ClusterConfig<'a> {
    name: &'a str,
    /// Parsed TLS config for the cluster (`None` = plaintext). Crate-internal
    /// for now; consumed by the built-in gRPC connector factory.
    pub(crate) security: Option<&'a ClusterSecurityConfig>,
}

impl<'a> ClusterConfig<'a> {
    /// Builds a view over a validated [`ClusterResource`].
    pub(crate) fn from_resource(cluster: &'a ClusterResource) -> Self {
        Self {
            name: &cluster.name,
            security: cluster.security.as_ref(),
        }
    }

    /// The cluster name.
    pub fn name(&self) -> &str {
        self.name
    }
}

impl std::fmt::Debug for ClusterConfig<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusterConfig")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// Factory for creating per-cluster [`Connector`]s.
///
/// Given a [`ClusterConfig`] view, the implementation builds a [`Connector`]
/// tailored to that cluster (e.g. selecting TLS vs plaintext and wiring the
/// gRFC A29 cert providers). Returning an `Err` rejects the current cluster
/// update; the caller keeps the previously built connector.
///
/// The connector is returned type-erased as `Arc<dyn Connector>` so an
/// implementation can keep its concrete connector type(s) private and hand
/// back different connectors for different clusters without wrapping them in
/// a single enum.
pub trait MakeConnector: Send + Sync + 'static {
    /// The service type produced by the connectors.
    ///
    /// Must be `Send + 'static` because discovery drives it from a spawned
    /// task and streams endpoint changes carrying it across threads.
    type Service: Send + 'static;

    /// Build a connector for the given cluster.
    fn make_connector(
        &self,
        cluster: ClusterConfig<'_>,
    ) -> Result<Arc<dyn Connector<Service = Self::Service> + Send + Sync>, BoxError>;
}
