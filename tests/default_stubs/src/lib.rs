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

use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};

tonic::include_proto!("test");
tonic::include_proto!("test_default");

#[derive(Debug, Default)]
pub struct Svc;

#[tonic::async_trait]
impl test_server::Test for Svc {
    type ServerStreamStream = Pin<Box<dyn Stream<Item = Result<(), Status>> + Send + 'static>>;
    type BidirectionalStreamStream =
        Pin<Box<dyn Stream<Item = Result<(), Status>> + Send + 'static>>;

    async fn unary(&self, _: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::permission_denied(""))
    }

    async fn server_stream(
        &self,
        _: Request<()>,
    ) -> Result<Response<Self::ServerStreamStream>, Status> {
        Err(Status::permission_denied(""))
    }

    async fn client_stream(&self, _: Request<Streaming<()>>) -> Result<Response<()>, Status> {
        Err(Status::permission_denied(""))
    }

    async fn bidirectional_stream(
        &self,
        _: Request<Streaming<()>>,
    ) -> Result<Response<Self::BidirectionalStreamStream>, Status> {
        Err(Status::permission_denied(""))
    }
}

#[tonic::async_trait]
impl test_default_server::TestDefault for Svc {
    // Default unimplemented stubs provided here.
}
