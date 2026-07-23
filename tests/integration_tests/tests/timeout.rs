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

use integration_tests::pb::{test_client, test_server, Input, Output};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tonic::{transport::Server, Code, Request, Response, Status};

#[tokio::test]
async fn cancelation_on_timeout() {
    let addr = run_service_in_background(Duration::from_secs(1), Duration::from_secs(100)).await;

    let mut client = test_client::TestClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let mut req = Request::new(Input {});
    req.metadata_mut()
        // 500 ms
        .insert("grpc-timeout", "500m".parse().unwrap());

    let res = client.unary_call(req).await;

    let err = res.unwrap_err();
    assert!(err.message().contains("Timeout expired"));
    assert_eq!(err.code(), Code::Cancelled);
}

#[tokio::test]
async fn picks_server_timeout_if_thats_sorter() {
    let addr = run_service_in_background(Duration::from_secs(1), Duration::from_millis(100)).await;

    let mut client = test_client::TestClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let mut req = Request::new(Input {});
    req.metadata_mut()
        // 10 hours
        .insert("grpc-timeout", "10H".parse().unwrap());

    let res = client.unary_call(req).await;
    let err = res.unwrap_err();
    assert!(err.message().contains("Timeout expired"));
    assert_eq!(err.code(), Code::Cancelled);
}

#[tokio::test]
async fn picks_client_timeout_if_thats_sorter() {
    let addr = run_service_in_background(Duration::from_secs(1), Duration::from_secs(100)).await;

    let mut client = test_client::TestClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let mut req = Request::new(Input {});
    req.metadata_mut()
        // 100 ms
        .insert("grpc-timeout", "100m".parse().unwrap());

    let res = client.unary_call(req).await;
    let err = res.unwrap_err();
    assert!(err.message().contains("Timeout expired"));
    assert_eq!(err.code(), Code::Cancelled);
}

async fn run_service_in_background(latency: Duration, server_timeout: Duration) -> SocketAddr {
    struct Svc {
        latency: Duration,
    }

    #[tonic::async_trait]
    impl test_server::Test for Svc {
        async fn unary_call(&self, _req: Request<Input>) -> Result<Response<Output>, Status> {
            tokio::time::sleep(self.latency).await;
            Ok(Response::new(Output {}))
        }
    }

    let svc = test_server::TestServer::new(Svc { latency });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .timeout(server_timeout)
            .add_service(svc)
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    addr
}
