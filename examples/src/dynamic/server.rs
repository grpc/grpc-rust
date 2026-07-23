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

use std::env;
use tonic::{Request, Response, Status, service::RoutesBuilder, transport::Server};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod echo {
    tonic::include_proto!("grpc.examples.unaryecho");
}

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct MyEcho {}

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        println!("Got an echo request from {:?}", request.remote_addr());

        let message = format!("you said: {}", request.into_inner().message);

        Ok(Response::new(EchoResponse { message }))
    }
}

fn init_echo(args: &[String], builder: &mut RoutesBuilder) {
    let enabled = args.iter().any(|arg| arg.as_str() == "echo");
    if enabled {
        println!("Adding Echo service...");
        let svc = EchoServer::new(MyEcho::default());
        builder.add_service(svc);
    }
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a greet request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

fn init_greeter(args: &[String], builder: &mut RoutesBuilder) {
    let enabled = args.iter().any(|arg| arg.as_str() == "greeter");

    if enabled {
        println!("Adding Greeter service...");
        let svc = GreeterServer::new(MyGreeter::default());
        builder.add_service(svc);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut routes_builder = RoutesBuilder::default();
    init_greeter(&args, &mut routes_builder);
    init_echo(&args, &mut routes_builder);

    let addr = "[::1]:50051".parse().unwrap();

    println!("Grpc server listening on {addr}");

    Server::builder()
        .add_routes(routes_builder.routes())
        .serve(addr)
        .await?;

    Ok(())
}
