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
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::{transport::Server, Code, Request, Response, Status};

#[tokio::test]
async fn service_resource_exhausted() {
    let addr = run_service_in_background(0).await;

    let mut client = test_client::TestClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let req = Request::new(Input {});
    let res = client.unary_call(req).await;

    let err = res.unwrap_err();
    assert_eq!(err.code(), Code::ResourceExhausted);
}

#[tokio::test]
async fn service_resource_not_exhausted() {
    let addr = run_service_in_background(1).await;

    let mut client = test_client::TestClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let req = Request::new(Input {});
    let res = client.unary_call(req).await;

    assert!(res.is_ok());
}

async fn run_service_in_background(concurrency_limit: usize) -> SocketAddr {
    struct Svc;

    #[tonic::async_trait]
    impl test_server::Test for Svc {
        async fn unary_call(&self, _req: Request<Input>) -> Result<Response<Output>, Status> {
            Ok(Response::new(Output {}))
        }
    }

    let svc = test_server::TestServer::new(Svc {});

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .concurrency_limit_per_connection(concurrency_limit)
            .load_shed(true)
            .add_service(svc)
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    addr
}
