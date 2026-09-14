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

//! A minimal `!Send` client channel over HTTP/2.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use http::Uri;
use hyper_util::rt::TokioIo;

use crate::local::body::Body;
use crate::local::transport::LocalExec;

/// A minimal `!Send` gRPC channel: one HTTP/2 connection over TCP.
///
/// Unlike [`crate::transport::Channel`] there is no TLS, no load balancing and
/// **no reconnect** — if the connection dies, every subsequent call fails and
/// a new `Channel` must be connected.
///
/// Requires a tokio local context (a [`tokio::task::LocalSet`], or a
/// `tokio::runtime::LocalRuntime` on tokio >= 1.51); [`Channel::connect`]
/// panics outside of one because it drives the connection via
/// `tokio::task::spawn_local`.
#[derive(Debug)]
pub struct Channel {
    tx: hyper::client::conn::http2::SendRequest<Body>,
    origin: Uri,
}

impl Channel {
    /// Connect to `dst`, which must be an `http` URI with an explicit
    /// `host:port` authority (e.g. `http://127.0.0.1:50051`).
    pub async fn connect(dst: Uri) -> Result<Self, crate::BoxError> {
        if dst.scheme_str() != Some("http") {
            return Err(
                "local::Channel requires an http scheme in the URI (no TLS support)".into(),
            );
        }
        let authority = dst
            .authority()
            .ok_or("local::Channel requires a host:port authority in the URI")?;
        let port = authority
            .port_u16()
            .ok_or("local::Channel requires an explicit port in the URI")?;

        let stream = tokio::net::TcpStream::connect((authority.host(), port)).await?;
        let (tx, conn) =
            hyper::client::conn::http2::handshake(LocalExec, TokioIo::new(stream)).await?;
        tokio::task::spawn_local(async move {
            if let Err(e) = conn.await {
                tracing::debug!("local channel connection error: {e}");
            }
        });

        Ok(Self { tx, origin: dst })
    }
}

impl tower_service::Service<http::Request<Body>> for Channel {
    type Response = http::Response<hyper::body::Incoming>;
    type Error = crate::BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.tx.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        // Update the request URI with the origin's scheme and authority,
        // mirroring transport/channel/service/add_origin.rs.
        let (mut head, body) = req.into_parts();
        let mut uri: http::uri::Parts = head.uri.into();
        let origin: http::uri::Parts = self.origin.clone().into();
        uri.scheme = origin.scheme;
        uri.authority = origin.authority;
        head.uri = match Uri::from_parts(uri) {
            Ok(uri) => uri,
            Err(e) => return Box::pin(std::future::ready(Err(e.into()))),
        };
        let req = http::Request::from_parts(head, body);

        let fut = self.tx.send_request(req);
        Box::pin(async move { fut.await.map_err(Into::into) })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use http_body_util::BodyExt;
    use hyper::service::service_fn;
    use tokio::task::LocalSet;
    use tower_service::Service as _;

    use super::*;

    /// Serve one HTTP/2 connection with a hand-rolled echo service; returns
    /// the bound address and a handle capturing the seen request path.
    async fn spawn_echo_server() -> (std::net::SocketAddr, Rc<Cell<bool>>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let addr_str = addr.to_string();
        let seen = Rc::new(Cell::new(false));
        let seen2 = seen.clone();

        tokio::task::spawn_local(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let svc = service_fn(move |req: http::Request<hyper::body::Incoming>| {
                let seen = seen2.clone();
                let addr_str = addr_str.clone();
                async move {
                    assert_eq!(req.uri().path(), "/echo/path");
                    assert_eq!(
                        req.uri().authority().map(|a| a.as_str()),
                        Some(addr_str.as_str())
                    );
                    seen.set(true);
                    let collected = req.into_body().collect().await.unwrap().to_bytes();
                    Ok::<_, std::convert::Infallible>(http::Response::new(
                        http_body_util::Full::new(collected),
                    ))
                }
            });
            let _ = hyper::server::conn::http2::Builder::new(LocalExec)
                .serve_connection(TokioIo::new(stream), svc)
                .await;
        });

        (addr, seen)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn channel_sends_request() {
        LocalSet::new()
            .run_until(async {
                let (addr, seen) = spawn_echo_server().await;

                let uri: Uri = format!("http://{addr}").parse().unwrap();
                let mut channel = Channel::connect(uri).await.unwrap();

                let payload = bytes::Bytes::from_static(b"ping");
                let req = http::Request::builder()
                    .method(http::Method::POST)
                    .uri("/echo/path")
                    .body(Body::new(http_body_util::Full::new(payload.clone())))
                    .unwrap();

                std::future::poll_fn(|cx| channel.poll_ready(cx))
                    .await
                    .unwrap();
                let resp = channel.call(req).await.unwrap();
                assert!(resp.status().is_success());
                let body = resp.into_body().collect().await.unwrap().to_bytes();
                assert_eq!(body, payload);
                assert!(seen.get());
            })
            .await;
    }

    #[tokio::test(flavor = "current_thread")]
    async fn connect_requires_port() {
        LocalSet::new()
            .run_until(async {
                let err = Channel::connect(Uri::from_static("http://localhost"))
                    .await
                    .unwrap_err();
                assert!(err.to_string().contains("port"));
            })
            .await;
    }

    #[tokio::test(flavor = "current_thread")]
    async fn connect_requires_http_scheme() {
        LocalSet::new()
            .run_until(async {
                let err = Channel::connect(Uri::from_static("127.0.0.1:5000"))
                    .await
                    .unwrap_err();
                assert!(err.to_string().contains("scheme"));
            })
            .await;
    }
}
