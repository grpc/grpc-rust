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

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Notify;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::Streaming;
use tonic::transport::Identity;
use tonic::transport::Server;
use tonic::transport::ServerTlsConfig;

use crate::generated::services::grpc::testing::Payload;
use crate::generated::services::grpc::testing::PayloadType;
use crate::generated::services::grpc::testing::ServerConfig;
use crate::generated::services::grpc::testing::ServerStats;
use crate::generated::services::grpc::testing::SimpleProtoParams;
use crate::generated::services::grpc::testing::SimpleRequest;
use crate::generated::services::grpc::testing::SimpleResponse;
use crate::generated::services::grpc::testing::benchmark_service_server::BenchmarkService;
use crate::generated::services::grpc::testing::benchmark_service_server::BenchmarkServiceServer;
use crate::generated::services::grpc::testing::payload_config::Payload::BytebufParams;
use crate::generated::services::grpc::testing::payload_config::Payload::ComplexParams;
use crate::generated::services::grpc::testing::payload_config::Payload::SimpleParams;
use crate::rusage::Rusage;

const SERVER_PEM: &[u8] = include_bytes!("../data/tls/server1.pem");
const SERVER_KEY: &[u8] = include_bytes!("../data/tls/server1.key");

pub struct BenchmarkServer {
    last_reset_time: Instant,
    last_rusage: Rusage,
    shutdown_notify: Arc<Notify>,
    port: u16,
}

impl BenchmarkServer {
    pub(crate) async fn start(config: ServerConfig) -> Result<Self, Status> {
        println!("Starting benchmark server with config: {:?}", config);

        let mut server_builder = Server::builder();
        // Parse security config.
        if let Some(security_params) = config.security_params {
            let tls_config = if security_params.use_test_ca {
                ServerTlsConfig::new().identity(Identity::from_pem(SERVER_PEM, SERVER_KEY))
            } else {
                ServerTlsConfig::new()
            };
            server_builder = server_builder.tls_config(tls_config).map_err(|err| {
                Status::invalid_argument(format!("failed to create TLS config: {err}"))
            })?;
        };

        // Parse payload config.
        let payload_type = match config.payload_config {
            Some(payload_config) => payload_config.payload.ok_or(Status::invalid_argument(
                "payload missing in payload_config",
            ))?,
            None => SimpleParams(SimpleProtoParams::default()),
        };

        let router = match payload_type {
            BytebufParams(_) | ComplexParams(_) => {
                return Err(Status::unimplemented("codec not implemented."));
            }
            SimpleParams(_) => {
                let server = BenchmarkServiceServer::new(ProtoServer {});
                server_builder.add_service(server)
            }
        };

        let shutdown_notify = Arc::new(Notify::new());
        let shutdown_notify_copy = shutdown_notify.clone();
        let port = if config.port > 0 {
            config.port as u16
        } else {
            0 // 0 means pick unused port
        };
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|err| Status::internal(format!("failed to bind to address {addr}: {err}")))?;
        let bound_addr = listener
            .local_addr()
            .map_err(|err| Status::internal(format!("failed to get local address: {err}")))?;
        let port = bound_addr.port();

        let incoming = TcpListenerStream::new(listener);

        tokio::spawn(router.serve_with_incoming_shutdown(incoming, async move {
            shutdown_notify_copy.notified().await;
            println!("BenchmarkServer is shutting down.")
        }));

        Ok(BenchmarkServer {
            last_reset_time: Instant::now(),
            last_rusage: Rusage::now().map_err(|err| {
                Status::internal(format!("failed to query system resource usage: {err}"))
            })?,
            shutdown_notify,
            port,
        })
    }

    pub(crate) fn get_stats(&mut self, reset: bool) -> Result<ServerStats, Status> {
        let now = Instant::now();
        let wall_time_elapsed = now.duration_since(self.last_reset_time);
        let latest_rusage = Rusage::now().map_err(|err| {
            Status::internal(format!("failed to query system resource usage: {err}"))
        })?;
        let user_time_ns = latest_rusage.user_time_nanos() - self.last_rusage.user_time_nanos();
        let system_time_ns =
            latest_rusage.system_time_nanos() - self.last_rusage.system_time_nanos();

        if reset {
            self.last_rusage = latest_rusage;
            self.last_reset_time = now;
        }

        Ok(ServerStats {
            time_elapsed: wall_time_elapsed.as_nanos() as f64 / 1e9,
            time_user: user_time_ns as f64 / 1e9,
            time_system: system_time_ns as f64 / 1e9,
            // The following fields are not set by Java and Go.
            idle_cpu_time: 0,
            cq_poll_count: 0,
            total_cpu_time: 0,
            core_stats: None,
        })
    }

    pub(crate) fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug)]
struct ProtoServer {}

#[tonic::async_trait]
impl BenchmarkService for ProtoServer {
    async fn unary_call(
        &self,
        request: Request<SimpleRequest>,
    ) -> Result<Response<SimpleResponse>, Status> {
        Ok(Response::new(SimpleResponse {
            payload: Some(Payload {
                r#type: PayloadType::Compressable as i32,
                body: vec![0; request.into_inner().response_size as usize],
            }),
            username: String::new(),
            oauth_scope: String::new(),
            server_id: String::new(),
            grpclb_route_type: 0,
            hostname: String::new(),
        }))
    }

    type StreamingCallStream =
        Pin<Box<dyn Stream<Item = Result<SimpleResponse, Status>> + Send + 'static>>;

    async fn streaming_call(
        &self,
        request: Request<Streaming<SimpleRequest>>,
    ) -> Result<Response<Self::StreamingCallStream>, Status> {
        let mut inbound = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(simple_request) = inbound.next().await {
                let request = simple_request?;
                yield SimpleResponse {
                    payload: Some(Payload {
                        r#type: PayloadType::Compressable as i32,
                        body: vec![0; request.response_size as usize],
                    }),
                    username: String::new(),
                    oauth_scope: String::new(),
                    server_id: String::new(),
                    grpclb_route_type: 0,
                    hostname: String::new(),
                };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::StreamingCallStream))
    }

    async fn streaming_from_client(
        &self,
        _request: tonic::Request<Streaming<SimpleRequest>>,
    ) -> Result<Response<SimpleResponse>, Status> {
        Err(Status::unimplemented("method unimplemented"))
    }

    type StreamingFromServerStream =
        Pin<Box<dyn Stream<Item = Result<SimpleResponse, Status>> + Send + 'static>>;

    async fn streaming_from_server(
        &self,
        _request: Request<SimpleRequest>,
    ) -> Result<Response<Self::StreamingFromServerStream>, Status> {
        Err(Status::unimplemented("method unimplemented"))
    }

    type StreamingBothWaysStream =
        Pin<Box<dyn Stream<Item = Result<SimpleResponse, Status>> + Send + 'static>>;

    async fn streaming_both_ways(
        &self,
        _request: Request<Streaming<SimpleRequest>>,
    ) -> Result<Response<Self::StreamingBothWaysStream>, Status> {
        Err(Status::unimplemented("method unimplemented"))
    }
}

impl Drop for BenchmarkServer {
    fn drop(&mut self) {
        self.shutdown_notify.notify_one();
    }
}
