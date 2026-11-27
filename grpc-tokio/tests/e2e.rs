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

//! End-to-end tests for channels and servers running on the Tokio runtime.

use std::any::Any;
use std::future::Future;
use std::mem;
use std::panic;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::Duration;

use bytes::Buf;
use bytes::Bytes;
use grpc::StatusCodeError;
use grpc::StatusError;
use grpc::client::CallOptions;
use grpc::client::Channel;
use grpc::client::Invoke as _;
use grpc::client::RecvStream as _;
use grpc::client::RequestHeaders;
use grpc::client::ResponseStreamItem;
use grpc::client::SendOptions;
use grpc::client::SendStream as _;
use grpc::core::RecvMessage;
use grpc::core::SendMessage;
use grpc::credentials::LocalChannelCredentials;
use grpc::credentials::LocalServerCredentials;
use grpc::rt::GrpcRuntime;
use grpc::server::CallOptions as ServerCallOptions;
use grpc::server::DynHandle;
use grpc::server::Handle;
use grpc::server::Http2Config;
use grpc::server::Listener as _;
use grpc::server::RecvStream as ServerRecvStream;
use grpc::server::RequestHeaders as ServerRequestHeaders;
use grpc::server::ResponseHeaders as ServerResponseHeaders;
use grpc::server::ResponseStreamItem as ServerResponseStreamItem;
use grpc::server::SendOptions as ServerSendOptions;
use grpc::server::SendStream as ServerSendStream;
use grpc::server::Server;
use grpc::server::TcpAddress;
use grpc::server::Trailers as ServerTrailers;
use grpc::server::descriptor::MethodDescriptor;
use grpc::server::descriptor::ServiceDescriptor;
use grpc::server::service::Service;
use tokio::runtime::Builder;

struct Empty;

impl SendMessage for Empty {
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
        Ok(Box::new(Bytes::new()))
    }
}

impl RecvMessage for Empty {
    fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String> {
        data.advance(data.remaining());
        Ok(())
    }
}

/// The full path of the echo service's only method.
const ECHO_METHOD: &str = "/test.Echo/Echo";

/// A message holding raw bytes, used for both echo requests and responses.
#[derive(Default)]
struct BytesMessage(Bytes);

impl SendMessage for BytesMessage {
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
        Ok(Box::new(self.0.clone()))
    }
}

impl RecvMessage for BytesMessage {
    fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String> {
        self.0 = data.copy_to_bytes(data.remaining());
        Ok(())
    }
}

/// Handles the echo method by sending the client's request message back.
struct EchoHandler;

impl Handle for EchoHandler {
    async fn handle(
        &self,
        _headers: ServerRequestHeaders,
        _options: ServerCallOptions,
        tx: &mut impl ServerSendStream,
        mut rx: impl ServerRecvStream + 'static,
    ) -> ServerTrailers {
        let mut request = BytesMessage::default();
        if !matches!(rx.next(&mut request).await, Some(Ok(()))) {
            return ServerTrailers::new(Err(StatusError::new(
                StatusCodeError::InvalidArgument,
                "expected a request message",
            )));
        }
        let headers = ServerResponseStreamItem::Headers(ServerResponseHeaders::new());
        let message = ServerResponseStreamItem::Message(&request);
        if tx
            .send(headers, ServerSendOptions::default())
            .await
            .is_err()
            || tx
                .send(message, ServerSendOptions::default())
                .await
                .is_err()
        {
            return ServerTrailers::new(Err(StatusError::new(
                StatusCodeError::Internal,
                "failed to send the response",
            )));
        }
        ServerTrailers::new(Ok(()))
    }
}

/// A service with one method, [`ECHO_METHOD`], handled by [`EchoHandler`].
struct EchoService;

impl Service for EchoService {
    fn descriptor(&self) -> ServiceDescriptor {
        ServiceDescriptor::new("test.Echo", vec![MethodDescriptor::new(ECHO_METHOD)])
    }

    fn register_methods(self) -> Vec<(String, Arc<dyn DynHandle>)> {
        vec![(ECHO_METHOD.to_string(), Arc::new(EchoHandler))]
    }
}

/// Binds a server running [`EchoService`] to a localhost port using `rt`.
/// Returns the port and a future that serves until it fails or is dropped.
async fn bind_server(rt: GrpcRuntime) -> (u16, impl Future<Output = ()> + Send) {
    let server = Server::builder(rt.clone()).add_service(EchoService).build();
    let listener = Server::tcp_listener(
        "127.0.0.1:0".parse().unwrap(),
        Arc::new(LocalServerCredentials::new()),
        Http2Config::default(),
        rt,
    )
    .await
    .unwrap();
    let port = (listener.local_addr() as &dyn Any)
        .downcast_ref::<TcpAddress>()
        .expect("expected a TcpAddress")
        .0
        .port();
    let serve = async move {
        let _ = server.serve(listener).await;
    };
    (port, serve)
}

/// Makes an RPC on `channel` to a method the server doesn't implement, and
/// returns its status code.
async fn unimplemented_rpc_status(channel: &Channel) -> StatusCodeError {
    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/test.Service/Method"),
            CallOptions::default(),
        )
        .await;
    // The send may fail if the server has already rejected the RPC.
    let _ = tx
        .send(&Empty, SendOptions::new().with_final_msg(true))
        .await;

    let status = loop {
        match rx.recv(&mut Empty).await {
            ResponseStreamItem::Trailers(trailers) => break trailers.status().clone(),
            ResponseStreamItem::StreamClosed => panic!("stream closed without trailers"),
            _ => {}
        }
    };
    status.unwrap_err().code()
}

/// Makes an echo RPC on `channel` with `payload`, asserts that it succeeds, and
/// returns the echoed payload.
async fn echo_rpc(channel: &Channel, payload: &'static [u8]) -> Bytes {
    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name(ECHO_METHOD),
            CallOptions::default(),
        )
        .await;
    tx.send(
        &BytesMessage(Bytes::from_static(payload)),
        SendOptions::new().with_final_msg(true),
    )
    .await
    .expect("failed to send the request");

    let mut response = BytesMessage::default();
    let mut echoed = None;
    loop {
        match rx.recv(&mut response).await {
            ResponseStreamItem::Headers(_) => {}
            ResponseStreamItem::Message => echoed = Some(mem::take(&mut response.0)),
            ResponseStreamItem::Trailers(trailers) => {
                if let Err(status) = trailers.status() {
                    panic!("echo RPC failed: {status:?}");
                }
                break;
            }
            ResponseStreamItem::StreamClosed => panic!("stream closed without trailers"),
        }
    }
    echoed.expect("no response message")
}

/// Runs `f` on a new thread, outside any Tokio runtime context, and returns
/// its result. Fails the test if `f` panics or doesn't finish in time.
fn run_on_thread<T: Send + 'static>(what: &str, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    let thread = thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(result) => {
            thread.join().unwrap();
            result
        }
        Err(RecvTimeoutError::Disconnected) => {
            panic::resume_unwind(thread.join().unwrap_err());
        }
        Err(RecvTimeoutError::Timeout) => panic!("{what} did not finish in time"),
    }
}

/// Starts a server running [`EchoService`] on the Tokio runtime, connects to it
/// with a channel on the Tokio runtime, and verifies an RPC to a method the
/// server doesn't implement fails with UNIMPLEMENTED.
#[tokio::test]
async fn rpc_to_unregistered_method_is_unimplemented() {
    let rt = grpc_tokio::runtime();
    let (port, serve) = bind_server(rt.clone()).await;
    let server_task = tokio::spawn(serve);

    let channel = Channel::builder(
        format!("dns:///127.0.0.1:{port}"),
        Arc::new(LocalChannelCredentials::new()),
        rt,
    )
    .build();
    assert_eq!(
        unimplemented_rpc_status(&channel).await,
        StatusCodeError::Unimplemented
    );

    server_task.abort();
}

/// A channel runs on the Tokio runtime it was created with, not on whichever
/// runtime is current when RPCs are made.
///
/// The channel is bound to runtime A (which also runs the server), but its
/// first RPC, which is when it connects, is made from a separate runtime B.
/// After B is shut down, the channel must still work, so a second RPC is made
/// from outside any Tokio runtime. Both RPCs must succeed. If the channel had placed its connection or
/// any other background work on B, that work would have been lost when B shut
/// down.
#[test]
fn channel_is_bound_to_its_runtime_not_the_callers() {
    // Runtime A: runs the server and the channel's background work.
    let runtime_a = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let rt = grpc_tokio::runtime_with_handle(runtime_a.handle().clone());
    let (port, serve) = runtime_a.block_on(bind_server(rt.clone()));
    runtime_a.spawn(serve);

    // Create the channel outside any Tokio runtime context.
    let channel = Channel::builder(
        format!("dns:///127.0.0.1:{port}"),
        Arc::new(LocalChannelCredentials::new()),
        rt,
    )
    .build();

    // First RPC: from runtime B, which is dropped (shut down) when the thread
    // finishes.
    let ch = channel.clone();
    let echoed = run_on_thread("RPC from runtime B", move || {
        let runtime_b = Builder::new_current_thread().enable_all().build().unwrap();
        runtime_b.block_on(echo_rpc(&ch, b"from runtime B"))
    });
    assert_eq!(echoed, &b"from runtime B"[..]);

    // Second RPC: from outside any Tokio runtime, after B has shut down.
    let echoed = run_on_thread("RPC after runtime B shut down", move || {
        futures_executor::block_on(echo_rpc(&channel, b"after runtime B"))
    });
    assert_eq!(echoed, &b"after runtime B"[..]);
}
