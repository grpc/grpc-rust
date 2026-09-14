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

use std::process;
use std::sync::Arc;
use std::time::Duration;

use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use grpc_benchmark::generated::grpc::testing::ClientArgs;
use grpc_benchmark::generated::grpc::testing::ClientConfig;
use grpc_benchmark::generated::grpc::testing::ClientStatus;
use grpc_benchmark::generated::grpc::testing::ClientType;
use grpc_benchmark::generated::grpc::testing::ClosedLoopParams;
use grpc_benchmark::generated::grpc::testing::HistogramDataView;
use grpc_benchmark::generated::grpc::testing::HistogramParams;
use grpc_benchmark::generated::grpc::testing::LoadParams;
use grpc_benchmark::generated::grpc::testing::Mark;
use grpc_benchmark::generated::grpc::testing::PayloadConfig;
use grpc_benchmark::generated::grpc::testing::RpcType;
use grpc_benchmark::generated::grpc::testing::SecurityParams;
use grpc_benchmark::generated::grpc::testing::ServerArgs;
use grpc_benchmark::generated::grpc::testing::ServerConfig;
use grpc_benchmark::generated::grpc::testing::ServerStatus;
use grpc_benchmark::generated::grpc::testing::SimpleProtoParams;
use grpc_benchmark::generated::grpc::testing::worker_service_client::WorkerServiceClient;
use grpc_benchmark::generated::services::grpc::testing::worker_service_server::WorkerServiceServer;
use grpc_benchmark::worker::WorkerServer;
use pico_args::Arguments;
use protobuf::proto;
use tokio::sync::Notify;
use tokio::time::sleep;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server as TonicServer;

struct Args {
    duration_secs: u64,
    channels: i32,
    rpcs_per_channel: i32,
    rpc_type: String,
    secure: bool,
    req_size: i32,
    resp_size: i32,
}

fn parse_args(mut pargs: Arguments) -> Result<Args, String> {
    let args = Args {
        duration_secs: pargs
            .value_from_str(["-d", "--duration"])
            .map_err(|e| format!("missing or invalid --duration: {e}"))?,
        channels: pargs
            .opt_value_from_str("--channels")
            .map_err(|e| format!("invalid --channels: {e}"))?
            .unwrap_or(1),
        rpcs_per_channel: pargs
            .value_from_str("--rpcs-per-channel")
            .map_err(|e| format!("missing or invalid --rpcs-per-channel: {e}"))?,
        rpc_type: {
            let t: String = pargs
                .value_from_str("--rpc-type")
                .map_err(|e| format!("missing or invalid --rpc-type: {e}"))?;
            if t != "unary" && t != "streaming" {
                return Err("rpc-type must be 'unary' or 'streaming'".to_string());
            }
            t
        },
        secure: pargs.contains("--secure"),
        req_size: pargs
            .opt_value_from_str("--req-size")
            .map_err(|e| format!("invalid --req-size: {e}"))?
            .unwrap_or(1),
        resp_size: pargs
            .opt_value_from_str("--resp-size")
            .map_err(|e| format!("invalid --resp-size: {e}"))?
            .unwrap_or(1),
    };

    let unused = pargs.finish();
    if !unused.is_empty() {
        return Err(format!("Unused arguments: {:?}", unused));
    }

    Ok(args)
}

fn calculate_percentile(
    histogram: &HistogramDataView<'_>,
    percentile: f64,
    resolution: f64,
) -> f64 {
    let count = histogram.count();
    if count == 0.0 {
        return 0.0;
    }
    let target = percentile * count;
    let mut accumulated = 0.0;
    let resolution_multiplier = 1.0 + resolution;
    let mut lower_bound = 1.0;

    for b in histogram.bucket() {
        let bucket_count = b as f64;
        let upper_bound = lower_bound * resolution_multiplier;
        if accumulated + bucket_count >= target {
            let fraction = (target - accumulated) / bucket_count;
            return lower_bound + fraction * (upper_bound - lower_bound);
        }
        accumulated += bucket_count;
        lower_bound = upper_bound;
    }
    lower_bound
}

#[tokio::main]
async fn main() {
    let mut pargs = Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        println!(
            "Usage: bench --duration <secs> --channels <num> --rpcs-per-channel <num> \
             --rpc-type <unary|streaming> [--secure] [--req-size <bytes>] [--resp-size <bytes>]"
        );
        return;
    }

    let args = match parse_args(pargs) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            process::exit(1);
        }
    };

    println!("Benchmark Config:");
    println!("  Duration: {} s", args.duration_secs);
    println!("  Channels: {}", args.channels);
    println!("  RPCs per channel: {}", args.rpcs_per_channel);
    println!("  RPC type: {}", args.rpc_type);
    println!(
        "  Security: {}",
        if args.secure { "secure" } else { "insecure" }
    );
    println!("  Request size: {} bytes", args.req_size);
    println!("  Response size: {} bytes", args.resp_size);

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

    let mut server_config = proto!(ServerConfig {
        port: 0, // Dynamic port
    });
    if args.secure {
        server_config.set_security_params(proto!(SecurityParams { use_test_ca: true }));
    }

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
    let rpc_type = match args.rpc_type.as_str() {
        "unary" => RpcType::Unary,
        "streaming" => RpcType::Streaming,
        _ => unreachable!(),
    };

    let mut client_config = proto!(ClientConfig {
        server_targets: vec![target.as_str()].into_iter(),
        client_type: ClientType::AsyncClient,
        client_channels: args.channels,
        outstanding_rpcs_per_channel: args.rpcs_per_channel,
        rpc_type: rpc_type,
        histogram_params: proto!(HistogramParams {
            resolution: 0.01,
            max_possible: 60e9, // 60s
        }),
        payload_config: proto!(PayloadConfig {
            simple_params: proto!(SimpleProtoParams {
                req_size: args.req_size,
                resp_size: args.resp_size,
            }),
        }),
        load_params: proto!(LoadParams {
            closed_loop: proto!(ClosedLoopParams {}),
        }),
    });
    if args.secure {
        client_config.set_security_params(proto!(SecurityParams {
            use_test_ca: true,
            server_host_override: "foo.test.google.fr".to_string(),
        }));
    }

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

    // Warmup
    println!("Warming up for 5 seconds...");
    sleep(Duration::from_secs(5)).await;

    // Reset stats after warmup
    let mark = proto!(Mark { reset: true });
    client_tx
        .send(proto!(ClientArgs { mark: mark }))
        .await
        .unwrap();
    // Await stats reset response.
    client_rx
        .recv_into(&mut client_status)
        .await
        .expect("failed to receive client status after reset");

    println!("Running benchmark for {} seconds...", args.duration_secs);
    sleep(Duration::from_secs(args.duration_secs)).await;

    // Get final stats
    let mark = proto!(Mark { reset: false });
    client_tx
        .send(proto!(ClientArgs { mark: mark }))
        .await
        .unwrap();

    if let Ok(()) = client_rx.recv_into(&mut client_status).await {
        let status_view = client_status.as_view();
        if status_view.has_stats() && status_view.stats().has_latencies() {
            let stats = status_view.stats();
            let latencies = stats.latencies();
            let count = latencies.count();
            if count > 0.0 {
                let time_elapsed = stats.time_elapsed(); // in seconds
                let qps = count / time_elapsed;
                let avg_lat = latencies.sum() / count / 1e6; // ms

                let p50 = calculate_percentile(&latencies, 0.50, 0.01) / 1e6; // ms
                let p90 = calculate_percentile(&latencies, 0.90, 0.01) / 1e6; // ms
                let p99 = calculate_percentile(&latencies, 0.99, 0.01) / 1e6; // ms

                println!("\nReport:");
                println!("  QPS: {:.2}", qps);
                println!("  Avg Latency: {:.4} ms", avg_lat);
                println!("  50th Percentile Latency: {:.4} ms", p50);
                println!("  90th Percentile Latency: {:.4} ms", p90);
                println!("  99th Percentile Latency: {:.4} ms", p99);
            } else {
                println!("No RPCs completed during the test.");
            }
        } else {
            println!("Failed to get stats: stats or latencies missing.");
        }
    } else {
        println!("Failed to receive stats from client.");
    }

    drop(client_tx);
    drop(server_tx);

    assert!(client_rx.recv_into(&mut client_status).await.is_err());
    assert!(server_rx.recv_into(&mut server_status).await.is_err());

    shutdown_notify.notify_one();
}
