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

//! Example: standalone gRPC greeter server for testing xDS.
//!
//! Starts a greeter backend on a given port. Used together with the
//! `xds_server` and `channel` examples.
//!
//! # Quick start
//!
//! ```sh
//! ./tonic-xds/examples/run_xds_example.sh
//! ```
//!
//! # Running individually
//!
//! ```sh
//! # Start on port 50051 (default):
//! cargo run -p tonic-xds --example greeter_server --features testutil
//!
//! # Custom port:
//! PORT=50052 cargo run -p tonic-xds --example greeter_server --features testutil
//! ```

use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_xds::testutil::proto::helloworld::{
    HelloReply, HelloRequest,
    greeter_server::{Greeter, GreeterServer},
};

struct MyGreeter {
    addr: String,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        println!("Received request: name={name}");
        Ok(Response::new(HelloReply {
            message: format!("Hello {name} from {}", self.addr),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr: std::net::SocketAddr = format!("0.0.0.0:{port}").parse()?;

    println!("Greeter server listening on {addr}");

    Server::builder()
        .add_service(GreeterServer::new(MyGreeter {
            addr: addr.to_string(),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
