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

//! A minimal Tokio-based [`Runtime`] for this crate's unit tests.
//!
//! The production Tokio runtime is provided by the `grpc-tokio` crate. Unit
//! tests can't use it: `grpc-tokio` links against the non-test build of this
//! crate, so its types would not match the types seen by the tests.

#![allow(clippy::disallowed_methods)] // This is a Tokio runtime implementation.

use std::future::Future;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::Duration;
use std::time::Instant;

use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use crate::client::name_resolution::TCP_IP_NETWORK_TYPE;
use crate::private::UnstableTrait;
use crate::rt::BoxEndpoint;
use crate::rt::BoxFuture;
use crate::rt::BoxedTaskHandle;
use crate::rt::DnsResolver;
use crate::rt::EndpointListener;
use crate::rt::GrpcEndpoint;
use crate::rt::ResolverOptions;
use crate::rt::Runtime;
use crate::rt::Sleep;
use crate::rt::StreamEndpoint;
use crate::rt::TaskHandle;
use crate::rt::TcpOptions;
use crate::rt::UnixSocketOptions;
use crate::server::ListenerAddress;
use crate::server::TcpAddress;

#[derive(Debug, Default)]
pub(crate) struct TokioRuntime {
    _priv: (),
}

impl UnstableTrait for JoinHandle<()> {}

impl TaskHandle for JoinHandle<()> {
    fn abort(&self) {
        self.abort();
    }
}

impl UnstableTrait for tokio::time::Sleep {}

impl Sleep for tokio::time::Sleep {}

/// A DNS resolver that uses `tokio::net::lookup_host`. It only supports host
/// lookups using the system resolver.
struct TokioDnsResolver;

#[crate::async_trait]
impl DnsResolver for TokioDnsResolver {
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
        Err("TXT record lookup is not supported by the test runtime".to_string())
    }
}

impl Runtime for TokioRuntime {
    fn spawn(&self, task: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> BoxedTaskHandle {
        Box::new(tokio::spawn(task))
    }

    fn get_dns_resolver(&self, opts: ResolverOptions) -> Result<Box<dyn DnsResolver>, String> {
        if opts.server_addr.is_some() {
            return Err("custom DNS servers are not supported by the test runtime".to_string());
        }
        Ok(Box::new(TokioDnsResolver))
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>> {
        Box::pin(tokio::time::sleep(duration))
    }

    fn now(&self) -> Instant {
        // Use tokio's clock so that a paused/advanced test clock is respected.
        tokio::time::Instant::now().into_std()
    }

    fn tcp_stream(
        &self,
        target: SocketAddr,
        opts: TcpOptions,
    ) -> BoxFuture<Result<BoxEndpoint, String>> {
        Box::pin(async move {
            let stream = TcpStream::connect(target)
                .await
                .map_err(|err| err.to_string())?;
            stream
                .set_nodelay(opts.enable_nodelay)
                .map_err(|err| err.to_string())?;
            Ok(Box::new(StreamEndpoint::new_from_tcp(stream)?) as BoxEndpoint)
        })
    }

    #[cfg(unix)]
    fn unix_stream(
        &self,
        path: PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<BoxEndpoint, String>> {
        Box::pin(unix::connect(path))
    }

    fn tcp_listener(
        &self,
        addr: SocketAddr,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        Box::pin(async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| e.to_string())?;
            let local_addr = TcpAddress(listener.local_addr().map_err(|e| e.to_string())?);
            Ok(Box::new(TokioTcpListener {
                listener,
                local_addr,
            }) as Box<dyn EndpointListener>)
        })
    }

    #[cfg(unix)]
    fn unix_listener(
        &self,
        path: PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        Box::pin(async move { unix::bind(path) })
    }

    #[cfg(not(unix))]
    fn unix_listener(
        &self,
        _path: PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        Box::pin(
            async move { Err("Unix listeners are not supported on this platform".to_string()) },
        )
    }
}

impl StreamEndpoint<TcpStream> {
    pub(crate) fn new_from_tcp(stream: TcpStream) -> Result<Self, String> {
        let local_addr = stream.local_addr().map_err(|err| err.to_string())?;
        let peer_addr = stream.peer_addr().map_err(|err| err.to_string())?;
        Ok(StreamEndpoint::new(
            stream,
            local_addr.to_string().into_boxed_str(),
            peer_addr.to_string().into_boxed_str(),
            TCP_IP_NETWORK_TYPE,
        ))
    }
}

struct TokioTcpListener {
    listener: tokio::net::TcpListener,
    local_addr: TcpAddress,
}

#[crate::async_trait]
impl EndpointListener for TokioTcpListener {
    async fn accept(&self) -> Result<Box<dyn GrpcEndpoint>, String> {
        let (stream, peer_addr) = self.listener.accept().await.map_err(|e| e.to_string())?;
        let local_addr = stream
            .local_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_default();
        Ok(Box::new(StreamEndpoint::new(
            stream,
            local_addr.into_boxed_str(),
            peer_addr.to_string().into_boxed_str(),
            TCP_IP_NETWORK_TYPE,
        )))
    }

    fn local_addr(&self) -> &dyn ListenerAddress {
        &self.local_addr
    }
}

#[cfg(unix)]
#[path = "tokio_unix.rs"]
mod unix;
