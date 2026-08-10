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

use std::time::Duration;

use h2::server;
use h2::Reason;
use http::StatusCode;
use integration_tests::pb::test_bidi_stream_client::TestBidiStreamClient;
use integration_tests::pb::InputStream;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;

#[tokio::test]
async fn client_cancellation_sends_rst_stream() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = server::handshake(socket).await.unwrap();

        let (request, mut respond) = connection.accept().await.unwrap().unwrap();

        // Poll the H2 connection to drive progress.
        tokio::spawn(async move { while connection.accept().await.is_some() {} });

        // Send response headers to satisfy the client's await on the call
        let response = http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/grpc")
            .body(())
            .unwrap();
        let _respond_tx = respond.send_response(response, false).unwrap();

        let mut body = request.into_body();

        match body.data().await {
            Some(Ok(_)) => panic!("Expected error or EOF, got data"),
            Some(Err(err)) => {
                assert_eq!(err.reason(), Some(Reason::CANCEL));
            }
            None => panic!("Expected RST_STREAM, got clean close (EOS)"),
        };
    });

    let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut client = TestBidiStreamClient::new(channel);

    let (tx, rx) = mpsc::channel::<InputStream>(1);
    let stream = ReceiverStream::new(rx);
    let mut request = Request::new(stream);

    let cancel_handle = request.cancellation_handle();

    // Start the call. This will resolve when server sends headers.
    let response = client.bidi_call(request).await.unwrap();

    // Trigger cancellation
    cancel_handle.cancel();

    tokio::time::timeout(Duration::from_secs(5), server_handle)
        .await
        .expect("Test timed out waiting for server to verify reset")
        .unwrap();

    // Keep tx alive to prevent normal EOF.
    let _keep_tx = tx;
    let _keep_rx = response;
}
