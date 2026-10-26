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

//! A configurable Echo server for tests, modelled after grpc-go's
//! `internal/stubserver`.
//!
//! Each Echo method can be overridden with a `with_*` method. Methods that
//! aren't overridden echo the request, except for client streaming, which
//! returns `Unimplemented`.

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::Streaming;
use tonic::transport::Server;
use tonic::transport::server::Connected;

use crate::async_trait;
use crate::echo_pb::EchoRequest;
use crate::echo_pb::EchoResponse;
use crate::echo_pb::echo_server::Echo;
use crate::echo_pb::echo_server::EchoServer;

/// The response stream type used by the streaming Echo handlers.
pub(crate) type EchoStream =
    Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + 'static>>;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
type Handler<Req, Resp> =
    Arc<dyn Fn(Request<Req>) -> BoxFuture<Result<Response<Resp>, Status>> + Send + Sync>;

/// Boxes `f` and the futures it returns into a [`Handler`].
fn box_handler<Req, Resp, F, Fut>(f: F) -> Handler<Req, Resp>
where
    F: Fn(Request<Req>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response<Resp>, Status>> + Send + 'static,
{
    Arc::new(move |req| Box::pin(f(req)))
}

/// An Echo service whose handlers can be customized by each test.
#[derive(Clone, Default)]
pub(crate) struct StubServer {
    unary_echo: Option<Handler<EchoRequest, EchoResponse>>,
    server_streaming_echo: Option<Handler<EchoRequest, EchoStream>>,
    client_streaming_echo: Option<Handler<Streaming<EchoRequest>, EchoResponse>>,
    bidirectional_streaming_echo: Option<Handler<Streaming<EchoRequest>, EchoStream>>,
}

impl StubServer {
    /// Returns a server whose handlers all use the default behavior.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Overrides the `UnaryEcho` handler.
    pub(crate) fn with_unary_echo<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Request<EchoRequest>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<EchoResponse>, Status>> + Send + 'static,
    {
        self.unary_echo = Some(box_handler(f));
        self
    }

    /// Overrides the `ServerStreamingEcho` handler.
    pub(crate) fn with_server_streaming_echo<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Request<EchoRequest>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<EchoStream>, Status>> + Send + 'static,
    {
        self.server_streaming_echo = Some(box_handler(f));
        self
    }

    /// Overrides the `ClientStreamingEcho` handler.
    #[allow(dead_code)]
    pub(crate) fn with_client_streaming_echo<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Request<Streaming<EchoRequest>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<EchoResponse>, Status>> + Send + 'static,
    {
        self.client_streaming_echo = Some(box_handler(f));
        self
    }

    /// Overrides the `BidirectionalStreamingEcho` handler.
    pub(crate) fn with_bidirectional_streaming_echo<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Request<Streaming<EchoRequest>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<EchoStream>, Status>> + Send + 'static,
    {
        self.bidirectional_streaming_echo = Some(box_handler(f));
        self
    }

    /// Starts serving on a new TCP listener bound to `127.0.0.1:0`.
    pub(crate) async fn start(self) -> RunningStubServer {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let mut server = self.start_with_incoming(TcpListenerStream::new(listener));
        server.addr = Some(addr);
        server
    }

    /// Starts serving the connections produced by `incoming`, e.g. a
    /// `UnixListenerStream`. [`RunningStubServer::addr`] is unavailable for
    /// servers started this way.
    pub(crate) fn start_with_incoming<I, IO, IE>(self, incoming: I) -> RunningStubServer
    where
        I: Stream<Item = Result<IO, IE>> + Send + 'static,
        IO: AsyncRead + AsyncWrite + Connected + Unpin + Send + 'static,
        IE: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let serve_task = tokio::spawn(async move {
            let _ = Server::builder()
                .add_service(EchoServer::new(self))
                .serve_with_incoming_shutdown(incoming, async {
                    // Resolves when the sender is used or dropped.
                    let _ = shutdown_rx.await;
                })
                .await;
        });
        RunningStubServer {
            addr: None,
            shutdown_tx,
            serve_task,
        }
    }
}

/// A running [`StubServer`]. The server shuts down when this is dropped.
pub(crate) struct RunningStubServer {
    addr: Option<SocketAddr>,
    // Dropping the sender triggers the server's graceful shutdown.
    shutdown_tx: oneshot::Sender<()>,
    serve_task: JoinHandle<()>,
}

impl RunningStubServer {
    /// Returns the TCP address the server listens on.
    ///
    /// # Panics
    ///
    /// Panics if the server was started with
    /// [`StubServer::start_with_incoming`].
    pub(crate) fn addr(&self) -> SocketAddr {
        self.addr
            .expect("addr() is unavailable for servers started with start_with_incoming()")
    }

    /// Gracefully shuts the server down and waits for it to exit.
    pub(crate) async fn stop(self) {
        drop(self.shutdown_tx);
        self.serve_task.await.unwrap();
    }
}

/// Returns a stream that echoes each request received on `inbound`. This is
/// the default `BidirectionalStreamingEcho` behavior.
pub(crate) fn echo_stream(inbound: Streaming<EchoRequest>) -> EchoStream {
    Box::pin(inbound.map(|req| {
        req.map(|req| EchoResponse {
            message: req.message,
        })
    }))
}

#[async_trait]
impl Echo for StubServer {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        if let Some(handler) = &self.unary_echo {
            return handler(request).await;
        }
        Ok(Response::new(EchoResponse {
            message: request.into_inner().message,
        }))
    }

    type ServerStreamingEchoStream = EchoStream;

    async fn server_streaming_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoStream>, Status> {
        if let Some(handler) = &self.server_streaming_echo {
            return handler(request).await;
        }
        let response = EchoResponse {
            message: request.into_inner().message,
        };
        Ok(Response::new(Box::pin(tokio_stream::once(Ok(response)))))
    }

    async fn client_streaming_echo(
        &self,
        request: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<EchoResponse>, Status> {
        if let Some(handler) = &self.client_streaming_echo {
            return handler(request).await;
        }
        Err(Status::unimplemented("StubServer: ClientStreamingEcho"))
    }

    type BidirectionalStreamingEchoStream = EchoStream;

    async fn bidirectional_streaming_echo(
        &self,
        request: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<EchoStream>, Status> {
        if let Some(handler) = &self.bidirectional_streaming_echo {
            return handler(request).await;
        }
        Ok(Response::new(echo_stream(request.into_inner())))
    }
}
