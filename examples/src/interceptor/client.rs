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

use hello_world::HelloRequest;
use hello_world::greeter_client::GreeterClient;
use tonic::{
    Request, Status,
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, Endpoint},
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = GreeterClient::with_interceptor(channel, intercept);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={response:?}");

    Ok(())
}

/// This function will get called on each outbound request. Returning a
/// `Status` here will cancel the request and have that status returned to
/// the caller.
fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {req:?}");
    Ok(req)
}

// You can also use the `Interceptor` trait to create an interceptor type
// that is easy to name
struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        Ok(request)
    }
}

#[allow(dead_code, unused_variables)]
async fn using_named_interceptor() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let client: GreeterClient<InterceptedService<Channel, MyInterceptor>> =
        GreeterClient::with_interceptor(channel, MyInterceptor);

    Ok(())
}

// Using a function pointer type might also be possible if your interceptor is a
// bare function that doesn't capture any variables
#[allow(dead_code, unused_variables, clippy::type_complexity)]
async fn using_function_pointer_interceptro() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let client: GreeterClient<
        InterceptedService<Channel, fn(tonic::Request<()>) -> Result<tonic::Request<()>, Status>>,
    > = GreeterClient::with_interceptor(channel, intercept);

    Ok(())
}
