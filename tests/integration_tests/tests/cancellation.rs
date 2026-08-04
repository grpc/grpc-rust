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

use h2::Reason;
use integration_tests::pb::cancellation_test_client::CancellationTestClient;
use integration_tests::pb::Input;
use tokio::net::TcpListener;
use tokio_stream::wrappers::ReceiverStream;
use tonic::client::CancellationHandle;
use tonic::Request;

#[tokio::test]
async fn client_cancellation_sends_rst_stream() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::handshake(socket).await.unwrap();

        let (body_done_tx, mut body_done_rx) =
            tokio::sync::oneshot::channel::<Result<(), String>>();

        if let Some(result) = connection.accept().await {
            let (request, mut respond) = result.unwrap();

            // Send response headers to satisfy the client's await on the call
            let response = http::Response::builder()
                .status(http::StatusCode::OK)
                .header("content-type", "application/grpc")
                .body(())
                .unwrap();
            let _respond_tx = respond.send_response(response, false).unwrap();

            let mut body = request.into_body();

            tokio::spawn(async move {
                let res = match body.data().await {
                    Some(Ok(_)) => Err("Expected error or EOF, got data".to_string()),
                    Some(Err(err)) => {
                        if let Some(reason) = err.reason() {
                            if reason == Reason::CANCEL {
                                Ok(())
                            } else {
                                Err(format!("Expected CANCEL reason, got {:?}", reason))
                            }
                        } else {
                            Err(format!("Expected reset with reason, got: {:?}", err))
                        }
                    }
                    None => Err("Expected RST_STREAM, got clean close (EOS)".to_string()),
                };
                let _ = body_done_tx.send(res);
            });
        }

        // Drive the connection
        let mut drive_conn = true;
        let mut body_res = Err("Body task did not complete".to_string());
        while drive_conn {
            tokio::select! {
                res = connection.accept() => {
                    if res.is_none() {
                        drive_conn = false;
                    }
                }
                res = &mut body_done_rx => {
                    body_res = res.unwrap_or_else(|_| Err("Body task panicked".to_string()));
                    drive_conn = false;
                }
            }
        }

        body_res.unwrap();
    });

    let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut client = CancellationTestClient::new(channel);

    let (tx, rx) = tokio::sync::mpsc::channel::<Input>(1);
    let stream = ReceiverStream::new(rx);
    let mut request = Request::new(stream);

    let cancel_handle = CancellationHandle::new(request.extensions_mut());

    // Start the call. This will resolve when server sends headers.
    let response = client.bidi_stream(request).await.unwrap();

    // Trigger cancellation
    cancel_handle.cancel();

    // Keep tx alive to prevent normal EOF
    let _keep_tx = tx;
    let _keep_rx = response;

    tokio::time::timeout(Duration::from_secs(5), server_handle)
        .await
        .expect("Test timed out waiting for server to verify reset")
        .unwrap();
}
