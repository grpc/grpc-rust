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

#[allow(unused)]
mod generated {
    pub mod helloworld {
        grpc::include_generated_proto!("generated/helloworld", "helloworld");
    }
}

use std::env;
use std::sync::Arc;

use generated::helloworld::HelloRequest;
use generated::helloworld::greeter_client::GreeterClient;
use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use protobuf::proto;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let name = if args.len() > 1 {
        args[1].clone()
    } else {
        "Rust World".to_owned()
    };

    // Create a new gRPC channel:
    let channel = Channel::builder(
        "dns:///[::1]:50051",
        Arc::new(LocalChannelCredentials::new()),
    )
    .build();
    let client = GreeterClient::new(channel);

    // Send the request and print the response:
    let request = proto!(HelloRequest { name });
    let response = client
        .say_hello(request.as_view())
        .await
        .expect("RPC error");

    println!("Greeting: {:}", response.message());
}
