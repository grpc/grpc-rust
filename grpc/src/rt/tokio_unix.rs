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

//! Unix domain socket support for the tokio runtime.

use std::path::PathBuf;

use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::net::unix::SocketAddr;

use crate::client::name_resolution::UNIX_NETWORK_TYPE;
use crate::private::UnstableToken;
use crate::rt::EndpointListener;
use crate::rt::GrpcEndpoint;
use crate::rt::StreamEndpoint;
use crate::server::ListenerAddress;
use crate::server::UnixListenerAddress;

pub(super) async fn connect(path: PathBuf) -> Result<Box<dyn GrpcEndpoint>, String> {
    let stream = UnixStream::connect(&path)
        .await
        .map_err(|err| err.to_string())?;
    let peer_addr = stream.peer_addr().map_err(|e| e.to_string())?;
    let local_addr = stream.local_addr().map_err(|e| e.to_string())?;
    Ok(endpoint(stream, &local_addr, &peer_addr))
}

pub(super) fn bind(path: PathBuf) -> Result<Box<dyn EndpointListener>, String> {
    let listener = UnixListener::bind(&path).map_err(|e| e.to_string())?;
    let local_addr = listener.local_addr().map_err(|e| e.to_string())?;
    let local_addr = UnixListenerAddress::new(address_string(&local_addr), UnstableToken);
    Ok(Box::new(TokioUnixListener {
        listener,
        local_addr,
    }))
}

fn endpoint(
    stream: UnixStream,
    local_addr: &SocketAddr,
    peer_addr: &SocketAddr,
) -> Box<dyn GrpcEndpoint> {
    Box::new(StreamEndpoint::new(
        stream,
        address_string(local_addr).into_boxed_str(),
        address_string(peer_addr).into_boxed_str(),
        UNIX_NETWORK_TYPE,
    ))
}

/// Formats a Unix socket address the same way as `grpc-tokio`.
fn address_string(addr: &SocketAddr) -> String {
    if let Some(path) = addr.as_pathname() {
        return path.to_string_lossy().into_owned();
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    if let Some(name) = addr.as_abstract_name() {
        return format!("\0{}", String::from_utf8_lossy(name));
    }
    String::new()
}

struct TokioUnixListener {
    listener: UnixListener,
    local_addr: UnixListenerAddress,
}

#[crate::async_trait]
impl EndpointListener for TokioUnixListener {
    async fn accept(&self) -> Result<Box<dyn GrpcEndpoint>, String> {
        let (stream, peer_addr) = self.listener.accept().await.map_err(|e| e.to_string())?;
        let local_addr = stream.local_addr().map_err(|e| e.to_string())?;
        Ok(endpoint(stream, &local_addr, &peer_addr))
    }

    fn local_addr(&self) -> &dyn ListenerAddress {
        &self.local_addr
    }
}
