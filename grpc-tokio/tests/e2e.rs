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
use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;
use grpc::StatusCodeError;
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
use grpc::server::Http2Config;
use grpc::server::Listener as _;
use grpc::server::Server;
use grpc::server::TcpAddress;

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

/// Starts a server with no services on the Tokio runtime, connects to it with
/// a channel on the Tokio runtime, and verifies an RPC fails with
/// UNIMPLEMENTED.
#[tokio::test]
async fn rpc_to_unregistered_method_is_unimplemented() {
    let rt = grpc_tokio::runtime();
    let server = Server::builder(rt.clone()).build();
    let listener = Server::tcp_listener(
        "127.0.0.1:0".parse().unwrap(),
        Arc::new(LocalServerCredentials::new()),
        Http2Config::default(),
        rt.clone(),
    )
    .await
    .unwrap();
    let local_addr = listener.local_addr();
    let port = (local_addr as &dyn Any)
        .downcast_ref::<TcpAddress>()
        .expect("expected a TcpAddress")
        .0
        .port();
    let server_task = tokio::spawn(async move {
        let _ = server.serve(listener).await;
    });

    let channel = Channel::builder(
        format!("dns:///127.0.0.1:{port}"),
        Arc::new(LocalChannelCredentials::new()),
        grpc_tokio::runtime(),
    )
    .build();
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
    assert_eq!(status.unwrap_err().code(), StatusCodeError::Unimplemented);

    server_task.abort();
}
