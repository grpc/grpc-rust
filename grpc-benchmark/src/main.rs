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
use std::sync::Arc;

use grpc_benchmark::generated::services::grpc::testing::worker_service_server::WorkerServiceServer;
use grpc_benchmark::worker::WorkerServer;
use tokio::sync::Notify;
use tonic::transport::Server;

pub async fn run_worker(worker_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), worker_port);
    let quit_notify = Arc::new(Notify::new());

    let svc = WorkerServiceServer::new(WorkerServer::new(quit_notify.clone()));

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, quit_notify.notified())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("default crypto provider already set.");
    // The default Tokio runtime uses 1 thread per logical processor. While the
    // testing framework supports specifying the thread count in the test config,
    // the tests that run on k8s use specific machine sizes and don't depend on
    // the clients/servers to restrict their resource usage. Tokio doesn't
    // support nested runtimes, so support for per test thread config is not
    // presently supported.

    let mut driver_port = None;

    // Skip the first argument (the binary name itself).
    for arg in std::env::args().skip(1) {
        if let Some(port_str) = arg.strip_prefix("--driver_port=") {
            driver_port = Some(port_str.parse::<u16>().unwrap_or_else(|_| {
                eprintln!("Error: --driver_port must be a valid u16 integer.");
                std::process::exit(1);
            }));
        } else {
            eprintln!("Warning: Unrecognized argument '{}'", arg);
        }
    }

    let Some(dp) = driver_port else {
        eprintln!("Usage: worker --driver_port=<port>");
        std::process::exit(1);
    };

    run_worker(dp).await?;

    Ok(())
}
