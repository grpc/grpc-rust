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

//! A minimal `!Send` gRPC server.
//!
//! This is new code, not a mirror of `crate::transport::server`: it is a
//! deliberately small http2-only accept loop, with no TLS and no graceful
//! shutdown, so there is no "keep in sync" counterpart.
//!
//! Requires a tokio local context (a [`tokio::task::LocalSet`] on any tokio
//! 1.x, or a `tokio::runtime::LocalRuntime` on tokio >= 1.51): serving drives
//! each connection via `tokio::task::spawn_local`, which panics outside one.

use std::convert::Infallible;
use std::net::SocketAddr;

use http::{Request, Response};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_service::Service;

use crate::local::body::Body;
use crate::local::router::Routes;
use crate::local::transport::LocalExec;
use crate::server::NamedService;

/// Builder entry point for a `!Send` local gRPC server.
///
/// http2-only, no TLS, no graceful shutdown. Requires a tokio local context;
/// see the [module docs](self).
#[derive(Default, Debug)]
pub struct Server;

impl Server {
    /// Create a new server builder.
    pub fn builder() -> Self {
        Self
    }

    /// Add a service, returning a [`Router`] ready to serve it.
    pub fn add_service<S, ResBody>(self, svc: S) -> Router
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible>
            + NamedService
            + Clone
            + 'static,
        S::Future: 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
        ResBody::Error: Into<crate::BoxError>,
    {
        Router {
            routes: Routes::new(svc),
        }
    }
}

/// Accepts connections and serves the accumulated services over HTTP/2.
///
/// http2-only, no TLS, no graceful shutdown. Requires a tokio local context;
/// see the [module docs](self).
#[derive(Debug)]
pub struct Router {
    routes: Routes,
}

impl Router {
    /// Add another service.
    pub fn add_service<S, ResBody>(mut self, svc: S) -> Self
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = Infallible>
            + NamedService
            + Clone
            + 'static,
        S::Future: 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + 'static,
        ResBody::Error: Into<crate::BoxError>,
    {
        self.routes = self.routes.add_service(svc);
        self
    }

    /// Bind `addr` and serve forever: http2-only, no TLS, no graceful
    /// shutdown.
    ///
    /// Requires a tokio local context (a [`tokio::task::LocalSet`] on any
    /// tokio 1.x, or a `tokio::runtime::LocalRuntime` on tokio >= 1.51);
    /// panics outside one, via `tokio::task::spawn_local`.
    pub async fn serve(self, addr: SocketAddr) -> Result<(), crate::BoxError> {
        let listener = TcpListener::bind(addr).await?;
        self.serve_with_incoming(TcpListenerStream::new(listener))
            .await
    }

    /// Serve forever over an existing stream of accepted connections:
    /// http2-only, no TLS, no graceful shutdown.
    ///
    /// Requires a tokio local context (a [`tokio::task::LocalSet`] on any
    /// tokio 1.x, or a `tokio::runtime::LocalRuntime` on tokio >= 1.51);
    /// panics outside one, via `tokio::task::spawn_local`.
    pub async fn serve_with_incoming<I, IO, IE>(self, incoming: I) -> Result<(), crate::BoxError>
    where
        I: Stream<Item = Result<IO, IE>>,
        IO: AsyncRead + AsyncWrite + Unpin + 'static,
        IE: Into<crate::BoxError>,
    {
        let routes = self.routes;
        let mut incoming = std::pin::pin!(incoming);
        while let Some(io) = incoming.next().await {
            let io = TokioIo::new(io.map_err(Into::into)?);
            let svc = TowerToHyperService::new(routes.clone());
            tokio::task::spawn_local(async move {
                if let Err(err) = hyper::server::conn::http2::Builder::new(LocalExec)
                    .serve_connection(io, svc)
                    .await
                {
                    tracing::debug!("local server connection error: {err}");
                }
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::task::{Context, Poll};

    use http_body_util::BodyExt as _;
    use tokio::net::TcpStream;
    use tokio::task::LocalSet;

    use super::*;

    #[derive(Clone)]
    struct FakeSvc(Rc<Cell<u32>>);

    impl NamedService for FakeSvc {
        const NAME: &'static str = "test.Fake";
    }

    impl Service<Request<Body>> for FakeSvc {
        type Response = Response<Body>;
        type Error = Infallible;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: Request<Body>) -> Self::Future {
            self.0.set(self.0.get() + 1);
            let resp = Response::builder()
                .header("x-fake", "yes")
                .body(Body::empty())
                .unwrap();
            std::future::ready(Ok(resp))
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn serves_hand_rolled_service() {
        LocalSet::new()
            .run_until(async {
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                let hits = Rc::new(Cell::new(0));
                let router = Server::builder().add_service(FakeSvc(hits.clone()));

                tokio::task::spawn_local(async move {
                    let _ = router
                        .serve_with_incoming(TcpListenerStream::new(listener))
                        .await;
                });

                let stream = TcpStream::connect(addr).await.unwrap();
                let (mut sender, conn) =
                    hyper::client::conn::http2::handshake(LocalExec, TokioIo::new(stream))
                        .await
                        .unwrap();
                tokio::task::spawn_local(async move {
                    let _ = conn.await;
                });

                let req = Request::builder()
                    .uri("/test.Fake/Method")
                    .body(http_body_util::Empty::<bytes::Bytes>::new())
                    .unwrap();
                let resp = sender.send_request(req).await.unwrap();
                assert!(resp.status().is_success());
                assert_eq!(resp.headers().get("x-fake").unwrap(), "yes");
                let _ = resp.into_body().collect().await.unwrap();
                assert_eq!(hits.get(), 1);
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[should_panic(expected = "spawn_local` called from outside")]
    async fn panics_outside_local_context() {
        let hits = Rc::new(Cell::new(0));
        let router = Server::builder().add_service(FakeSvc(hits));
        let (server_io, _client_io) = tokio::io::duplex(1024);
        let _ = router
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server_io)))
            .await;
    }
}
