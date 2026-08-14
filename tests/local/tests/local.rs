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

//! End-to-end tests for `tonic::local` (single-threaded, `!Send`) clients and
//! servers, driven entirely through codegen (#2790).

use std::cell::RefCell;
use std::net::SocketAddr;
use std::pin::Pin;
use std::rc::Rc;

use local_tests::pb::echo_client::EchoClient;
use local_tests::pb::echo_server::{self, Echo};
use local_tests::pb::{EchoRequest, EchoResponse};
use tokio::task::LocalSet;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::{Stream, StreamExt};
use tonic::codegen::http;
use tonic::codegen::http::uri::PathAndQuery;
use tonic::codegen::http::Uri;
use tonic::codegen::Service;
use tonic::local::transport::channel::Channel;
use tonic::local::transport::server::Server;
use tonic::{Request, Response, Status};

// Misuse negatives: `tokio::spawn` requires `Send`, so these `!Send` types cannot be
// spawned onto a multi-threaded runtime — the local stack is compile-time confined to a
// local context.
static_assertions::assert_not_impl_any!(tonic::local::Body: Send);
static_assertions::assert_not_impl_any!(tonic::local::Streaming<EchoResponse>: Send);
static_assertions::assert_not_impl_any!(tonic::local::Channel: Send);
static_assertions::assert_not_impl_any!(tonic::local::Routes: Send);
static_assertions::assert_not_impl_any!(tonic::local::transport::server::Router: Send);
static_assertions::assert_not_impl_any!(EchoClient<tonic::local::Channel>: Send);

/// A `!Send` handler: counts unary calls via an `Rc<RefCell<..>>` and, if the
/// request carries an `x-marker` metadata entry (set by an interceptor),
/// echoes it back on the response.
struct EchoService {
    calls: Rc<RefCell<u64>>,
}

#[tonic::async_trait(?Send)]
impl Echo for EchoService {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        *self.calls.borrow_mut() += 1;
        let marker = request.metadata().get("x-marker").cloned();
        let mut response = Response::new(EchoResponse {
            message: request.into_inner().message,
        });
        if let Some(marker) = marker {
            response.metadata_mut().insert("x-marker", marker);
        }
        Ok(response)
    }

    type ServerStreamingEchoStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>>>>;

    async fn server_streaming_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        let msg = request.into_inner().message;
        // Non-`Send` response stream: the `map` closure captures an `Rc`.
        let tag = Rc::new(RefCell::new(0u32));
        let stream = tokio_stream::iter(0..5usize).map(move |i| {
            *tag.borrow_mut() += 1;
            Ok(EchoResponse {
                message: format!("{msg}-{i}"),
            })
        });
        Ok(Response::new(Box::pin(stream)))
    }

    async fn client_streaming_echo(
        &self,
        request: Request<tonic::local::codec::Streaming<EchoRequest>>,
    ) -> Result<Response<EchoResponse>, Status> {
        let mut stream = request.into_inner();
        let mut parts = Vec::new();
        while let Some(msg) = stream.message().await? {
            parts.push(msg.message);
        }
        Ok(Response::new(EchoResponse {
            message: parts.join(","),
        }))
    }

    type BidirectionalStreamingEchoStream =
        Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>>>>;

    async fn bidirectional_streaming_echo(
        &self,
        request: Request<tonic::local::codec::Streaming<EchoRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        let stream = request.into_inner().map(|item| {
            item.map(|req| EchoResponse {
                message: req.message,
            })
        });
        Ok(Response::new(Box::pin(stream)))
    }
}

/// Bind an ephemeral port and serve `svc` in the background via
/// `spawn_local`, returning the bound address.
async fn spawn_server<S, ResBody>(svc: S) -> SocketAddr
where
    S: Service<
            http::Request<tonic::local::body::Body>,
            Response = http::Response<ResBody>,
            Error = std::convert::Infallible,
        > + tonic::server::NamedService
        + Clone
        + 'static,
    S::Future: 'static,
    ResBody: tonic::codegen::Body<Data = tonic::codegen::Bytes> + 'static,
    ResBody::Error: Into<tonic::codegen::StdError>,
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::task::spawn_local(async move {
        let _ = Server::builder()
            .add_service(svc)
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await;
    });
    addr
}

async fn connect(addr: SocketAddr) -> EchoClient<Channel> {
    let uri: Uri = format!("http://{addr}").parse().unwrap();
    let channel = Channel::connect(uri).await.unwrap();
    EchoClient::new(channel)
}

/// Bootstrap an `EchoService` behind a freshly bound server and a connected
/// client, returning the call counter alongside the client.
async fn spawn_echo() -> (Rc<RefCell<u64>>, EchoClient<Channel>) {
    let calls = Rc::new(RefCell::new(0u64));
    let svc = EchoService {
        calls: calls.clone(),
    };
    let addr = spawn_server(echo_server::EchoServer::new(svc)).await;
    let client = connect(addr).await;
    (calls, client)
}

#[tokio::test(flavor = "current_thread")]
async fn unary_echo_with_rc_state() {
    LocalSet::new()
        .run_until(async {
            let (calls, mut client) = spawn_echo().await;

            let response = client
                .unary_echo(EchoRequest {
                    message: "hello".into(),
                })
                .await
                .unwrap();
            assert_eq!(response.into_inner().message, "hello");

            let response = client
                .unary_echo(EchoRequest {
                    message: "world".into(),
                })
                .await
                .unwrap();
            assert_eq!(response.into_inner().message, "world");

            assert_eq!(*calls.borrow(), 2);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn server_streaming_echo() {
    LocalSet::new()
        .run_until(async {
            let (_calls, mut client) = spawn_echo().await;

            let response = client
                .server_streaming_echo(EchoRequest {
                    message: "tick".into(),
                })
                .await
                .unwrap();
            let mut inbound = response.into_inner();
            let mut received = Vec::new();
            while let Some(msg) = inbound.message().await.unwrap() {
                received.push(msg.message);
            }

            let expected: Vec<String> = (0..5).map(|i| format!("tick-{i}")).collect();
            assert_eq!(received, expected);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn client_streaming_echo() {
    LocalSet::new()
        .run_until(async {
            let (_calls, mut client) = spawn_echo().await;

            // Non-`Send` request stream: the `map` closure captures an `Rc`.
            let tag = Rc::new(RefCell::new(0u32));
            let stream =
                tokio_stream::iter(["one".to_string(), "two".to_string(), "three".to_string()])
                    .map(move |m| {
                        *tag.borrow_mut() += 1;
                        EchoRequest { message: m }
                    });

            let response = client.client_streaming_echo(stream).await.unwrap();
            assert_eq!(response.into_inner().message, "one,two,three");
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn bidi_echo() {
    LocalSet::new()
        .run_until(async {
            let (_calls, mut client) = spawn_echo().await;

            let tag = Rc::new(RefCell::new(0u32));
            let messages = ["a".to_string(), "b".to_string(), "c".to_string()];
            let stream = tokio_stream::iter(messages.clone()).map(move |m| {
                *tag.borrow_mut() += 1;
                EchoRequest { message: m }
            });

            let response = client.bidirectional_streaming_echo(stream).await.unwrap();
            let mut inbound = response.into_inner();
            let mut received = Vec::new();
            while let Some(msg) = inbound.message().await.unwrap() {
                received.push(msg.message);
            }

            assert_eq!(received, messages);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn unimplemented_path() {
    LocalSet::new()
        .run_until(async {
            let calls = Rc::new(RefCell::new(0u64));
            let svc = EchoService { calls };
            let addr = spawn_server(echo_server::EchoServer::new(svc)).await;

            let uri: Uri = format!("http://{addr}").parse().unwrap();
            let channel = Channel::connect(uri).await.unwrap();
            let mut grpc = tonic::local::client::Grpc::new(channel);
            grpc.ready().await.unwrap();

            let path: PathAndQuery = "/localtest.Echo/Nope".parse().unwrap();
            let codec = tonic_prost::ProstCodec::<EchoRequest, EchoResponse>::default();
            let status = grpc
                .unary(
                    Request::new(EchoRequest {
                        message: "x".into(),
                    }),
                    path,
                    codec,
                )
                .await
                .unwrap_err();

            assert_eq!(status.code(), tonic::Code::Unimplemented);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn interceptor_works() {
    LocalSet::new()
        .run_until(async {
            fn add_marker(mut req: Request<()>) -> Result<Request<()>, Status> {
                req.metadata_mut()
                    .insert("x-marker", "server-seen".parse().unwrap());
                Ok(req)
            }

            let calls = Rc::new(RefCell::new(0u64));
            let svc = EchoService { calls };
            let intercepted = echo_server::EchoServer::with_interceptor(svc, add_marker);
            let addr = spawn_server(intercepted).await;
            let mut client = connect(addr).await;

            let response = client
                .unary_echo(EchoRequest {
                    message: "hi".into(),
                })
                .await
                .unwrap();

            assert_eq!(response.metadata().get("x-marker").unwrap(), "server-seen");
        })
        .await;
}

#[test]
fn works_on_local_runtime() {
    let rt = tokio::runtime::LocalRuntime::new().unwrap();
    rt.block_on(async {
        let (calls, mut client) = spawn_echo().await;

        let response = client
            .unary_echo(EchoRequest {
                message: "no-localset".into(),
            })
            .await
            .unwrap();
        assert_eq!(response.into_inner().message, "no-localset");
        assert_eq!(*calls.borrow(), 1);
    });
}
