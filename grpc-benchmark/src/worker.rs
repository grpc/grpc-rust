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

use std::pin::Pin;
use std::result::Result;
use std::sync::Arc;
use std::thread::available_parallelism;

use tokio::sync::Notify;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::Streaming;

use crate::client::BenchmarkClient;
use crate::generated::services::grpc::testing::ClientArgs;
use crate::generated::services::grpc::testing::ClientStatus;
use crate::generated::services::grpc::testing::CoreRequest;
use crate::generated::services::grpc::testing::CoreResponse;
use crate::generated::services::grpc::testing::ServerArgs;
use crate::generated::services::grpc::testing::ServerStatus;
use crate::generated::services::grpc::testing::Void;
use crate::generated::services::grpc::testing::client_args::Argtype as ClientArgType;
use crate::generated::services::grpc::testing::server_args::Argtype;
use crate::generated::services::grpc::testing::worker_service_server::WorkerService;
use crate::server::BenchmarkServer;

pub struct WorkerServer {
    quit_notify: Arc<Notify>,
}

impl WorkerServer {
    pub fn new(quit_notify: Arc<Notify>) -> Self {
        WorkerServer { quit_notify }
    }
}

fn core_count() -> Result<i32, Status> {
    let cores = available_parallelism()
        .map_err(|e| Status::internal(format!("failed to determine core count: {e}")))?
        .get() as i32;

    Ok(cores)
}

#[tonic::async_trait]
impl WorkerService for WorkerServer {
    // Server streaming response type for the RunServer method.
    type RunServerStream =
        Pin<Box<dyn Stream<Item = Result<ServerStatus, Status>> + Send + 'static>>;

    async fn run_server(
        &self,
        request: Request<Streaming<ServerArgs>>,
    ) -> Result<Response<Self::RunServerStream>, Status> {
        println!("Handling server stream.");
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            let mut benchmark_server: Option<BenchmarkServer> = None;

            while let Some(request) = stream.next().await {
                let request = request?;
                let mut reset_stats = false;

                let argtype = request.argtype
                    .ok_or_else(|| Status::invalid_argument("missing request.argtype"))?;

                match argtype {
                    Argtype::Setup(server_config) => {
                        println!("Server creation requested.");

                        if benchmark_server.is_some() {
                             Err(Status::already_exists("server already started"))?;
                        }

                        let server = BenchmarkServer::start(server_config).await.map_err(|status| {
                            println!("Error while creating server: {:?}", status);
                            status
                        })?;

                        benchmark_server = Some(server);
                    }
                    Argtype::Mark(mark) => {
                        println!("Server stats requested.");

                        benchmark_server.as_ref().ok_or_else(|| {
                            Status::invalid_argument("server does not exist when mark received")
                        })?;

                        reset_stats = mark.reset;
                    }
                };

                let server = benchmark_server.as_mut().unwrap();
                let stats = server.get_stats(reset_stats)?;

                yield ServerStatus {
                    stats: Some(stats),
                    cores: core_count()?,
                    port: server.port() as i32,
                };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RunServerStream))
    }

    type RunClientStream =
        Pin<Box<dyn Stream<Item = Result<ClientStatus, Status>> + Send + 'static>>;

    async fn run_client(
        &self,
        request: Request<Streaming<ClientArgs>>,
    ) -> Result<Response<Self::RunClientStream>, Status> {
        println!("Handling client stream.");
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            let mut benchmark_client: Option<BenchmarkClient> = None;
            while let Some(request) = stream.next().await {
                let request = request?;
                let mut reset_stats = false;
                let argtype = request.argtype
                    .ok_or(Status::invalid_argument("missing request.argtype"))?;
                match  argtype {
                    ClientArgType::Setup(client_config) => {
                        if benchmark_client.is_some() {
                             Err(Status::already_exists("client already started"))?;
                        }
                        match BenchmarkClient::start(client_config) {
                            Ok(client) => {
                                benchmark_client = Some(client);
                            },
                            Err(status) => {
                                println!("Error while creating client: {:?}", status);
                                Err(status)?;
                            }
                        }
                    },
                    ClientArgType::Mark(mark) => {
                        benchmark_client.as_ref()
                            .ok_or(Status::invalid_argument("client does not exist when mark received"))?;
                        reset_stats = mark.reset;
                    }
                };
                let stats = benchmark_client.as_mut().unwrap().get_stats(reset_stats).await?;
                yield ClientStatus {
                    stats: Some(stats),
                };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RunClientStream))
    }

    async fn core_count(
        &self,
        _request: Request<CoreRequest>,
    ) -> Result<Response<CoreResponse>, Status> {
        Ok(Response::new(CoreResponse {
            cores: core_count()?,
        }))
    }

    async fn quit_worker(&self, _request: Request<Void>) -> Result<Response<Void>, Status> {
        self.quit_notify.notify_one();
        Ok(Response::new(Void {}))
    }
}
