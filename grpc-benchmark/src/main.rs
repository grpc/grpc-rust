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

use std::env;
use std::process;
use std::time::Duration;

use grpc_benchmark::generated::services::grpc::testing::worker_service_server::WorkerServiceServer;
use grpc_benchmark::worker::WorkerServer;
use tokio::sync::mpsc;
use tokio::time;
use tonic::transport::Server;

#[derive(Debug)]
struct Args {
    /// Port to expose grpc.testing.WorkerService, Used by driver to initiate
    /// work.
    driver_port: u16,
}

async fn run_worker(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{}", args.driver_port).parse().unwrap();
    let (tx, mut rx) = mpsc::channel(1);

    let svc = WorkerServiceServer::new(WorkerServer::new(tx));

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, async {
            rx.recv().await;
            // Wait for the quit_worker response to be sent.
            time::sleep(Duration::from_secs(1)).await;
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The default Tokio runtime uses 1 thread per logical processor. While the
    // testing framework supports specifying the thread count in the test config,
    // the tests that run on k8s use specific machine sizes and don't depend on
    // the clients/servers to restrict their resource usage. Tokio doesn't
    // support nested runtimes, so adding support for per test thread config
    // is not presently supported.

    let mut driver_port = None;

    // Skip the first argument (the binary name itself).
    for arg in env::args().skip(1) {
        if let Some(port_str) = arg.strip_prefix("--driver_port=") {
            driver_port = Some(port_str.parse::<u16>().unwrap_or_else(|_| {
                eprintln!("Error: --driver_port must be a valid u16 integer.");
                process::exit(1);
            }));
        } else {
            eprintln!("Warning: Unrecognized argument '{}'", arg);
        }
    }

    let Some(dp) = driver_port else {
        eprintln!("Usage: worker --driver_port=<port>");
        process::exit(1);
    };

    let args = Args { driver_port: dp };

    println!("{:?}", args);
    run_worker(args).await?;

    Ok(())
}
