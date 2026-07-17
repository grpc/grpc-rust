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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use grpc::client::Channel;
    use grpc::credentials::LocalChannelCredentials;
    use protobuf::proto;
    use tokio::sync::Notify;
    use tokio::time::sleep;
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::transport::Server as TonicServer;

    use super::WorkerServer;
    use crate::generated::grpc::testing::ClientArgs;
    use crate::generated::grpc::testing::ClientConfig;
    use crate::generated::grpc::testing::ClientStatus;
    use crate::generated::grpc::testing::ClientType;
    use crate::generated::grpc::testing::ClosedLoopParams;
    use crate::generated::grpc::testing::HistogramParams;
    use crate::generated::grpc::testing::LoadParams;
    use crate::generated::grpc::testing::Mark;
    use crate::generated::grpc::testing::PayloadConfig;
    use crate::generated::grpc::testing::RpcType;
    use crate::generated::grpc::testing::ServerArgs;
    use crate::generated::grpc::testing::ServerConfig;
    use crate::generated::grpc::testing::ServerStatus;
    use crate::generated::grpc::testing::SimpleProtoParams;
    use crate::generated::grpc::testing::worker_service_client::WorkerServiceClient;
    use crate::generated::services::grpc::testing::worker_service_server::WorkerServiceServer;

    #[tokio::test]
    async fn test_smoke() {
        // Start Worker Server.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let worker_port = listener.local_addr().unwrap().port();
        let incoming = TcpListenerStream::new(listener);

        let quit_notify = Arc::new(Notify::new());
        let worker_svc = WorkerServer::new(quit_notify.clone());
        let svc = WorkerServiceServer::new(worker_svc);

        let shutdown_notify = Arc::new(Notify::new());
        let shutdown_notify_copy = shutdown_notify.clone();
        tokio::spawn(async move {
            TonicServer::builder()
                .add_service(svc)
                .serve_with_incoming_shutdown(incoming, shutdown_notify_copy.notified())
                .await
                .unwrap();
        });

        // Create Worker Client.
        let channel = Channel::builder(
            format!("dns:///127.0.0.1:{}", worker_port),
            LocalChannelCredentials::new_arc(),
        )
        .build();
        let client = WorkerServiceClient::new(channel);

        // Start Benchmark Server via Worker.
        let (mut server_tx, mut server_rx) = client.run_server().await;

        let server_config = proto!(ServerConfig {
            port: 0, // Dynamic port
        });

        server_tx
            .send(proto!(ServerArgs {
                setup: server_config,
            }))
            .await
            .unwrap();

        // Await server startup response.
        let mut server_status = ServerStatus::default();
        server_rx
            .recv_into(&mut server_status)
            .await
            .expect("failed to receive server status");

        let bound_port = server_status.as_view().port();
        assert!(bound_port > 0, "Bound port must be > 0");
        println!("Benchmark server bound to port: {}", bound_port);

        // Start Benchmark Client via Worker.
        let (mut client_tx, mut client_rx) = client.run_client().await;

        let target = format!("127.0.0.1:{}", bound_port);
        let client_config = proto!(ClientConfig {
            server_targets: vec![target.as_str()].into_iter(),
            client_type: ClientType::AsyncClient,
            client_channels: 1,
            outstanding_rpcs_per_channel: 1,
            rpc_type: RpcType::Unary,
            histogram_params: proto!(HistogramParams {
                resolution: 0.01,
                max_possible: 60e9, // 60s
            }),
            payload_config: proto!(PayloadConfig {
                simple_params: proto!(SimpleProtoParams {
                    req_size: 10,
                    resp_size: 10,
                }),
            }),
            load_params: proto!(LoadParams {
                closed_loop: proto!(ClosedLoopParams {}),
            }),
        });

        client_tx
            .send(proto!(ClientArgs {
                setup: client_config,
            }))
            .await
            .unwrap();

        // Await client startup response.
        let mut client_status = ClientStatus::default();
        client_rx
            .recv_into(&mut client_status)
            .await
            .expect("failed to receive client status");
        assert!(client_status.as_view().has_stats());

        // Query Stats until Latencies are Recorded.
        let result = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                let mark = proto!(Mark { reset: false });
                client_tx
                    .send(proto!(ClientArgs { mark: mark }))
                    .await
                    .unwrap();

                if let Ok(()) = client_rx.recv_into(&mut client_status).await {
                    let status_view = client_status.as_view();
                    if status_view.has_stats()
                        && status_view.stats().has_latencies()
                        && status_view.stats().latencies().count() > 0.0
                    {
                        break;
                    }
                }
                sleep(Duration::from_millis(10)).await;
            }
        })
        .await;

        assert!(result.is_ok(), "No stats recorded after timeout");

        drop(client_tx);
        drop(server_tx);

        assert!(client_rx.recv_into(&mut client_status).await.is_err());
        assert!(server_rx.recv_into(&mut server_status).await.is_err());

        shutdown_notify.notify_one();
    }
}
