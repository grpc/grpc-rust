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

use std::sync::Arc;

use bytes::Bytes;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tonic::async_trait;

use crate::client::transport::ProxyOptions;
use crate::client::transport::http_connect::rewind::Rewind;
use crate::credentials::ChannelCredentials;
use crate::credentials::ProtocolInfo;
use crate::credentials::call::CallCredentials;
use crate::credentials::client::ClientHandshakeInfo;
use crate::credentials::client::HandshakeOutput;
use crate::credentials::common::Authority;
use crate::private;
use crate::rt::BoxEndpoint;
use crate::rt::EndpointIoStream;
use crate::rt::GrpcEndpoint;
use crate::rt::GrpcRuntime;
use crate::rt::StreamEndpoint;

mod rewind;

/// Performs the HTTP CONNECT handshake on the given endpoint.
///
/// This function sends an HTTP CONNECT request to the proxy specified in `opts`,
/// reads the response, and returns a new endpoint that yields any buffered data
/// read during the handshake before delegating to the original endpoint.
async fn do_connect_handshake<I: GrpcEndpoint>(
    input: I,
    opts: &ProxyOptions,
) -> Result<ProxyStream<I>, String> {
    let mut io = EndpointIoStream::new(input);

    let mut req = format!(
        "CONNECT {} HTTP/1.1\r\nHost: {}\r\n",
        opts.target_authority(),
        opts.target_authority()
    )
    .into_bytes();
    if let Some(creds) = opts.proxy_authorization_header() {
        req.extend_from_slice(b"Proxy-Authorization: ");
        req.extend_from_slice(creds.as_bytes());
        req.extend_from_slice(b"\r\n");
    }
    req.extend_from_slice(b"\r\n"); // headers end

    io.write_all(&req).await.map_err(|e| e.to_string())?;
    io.flush().await.map_err(|e| e.to_string())?;

    const READ_BUF_SIZE: usize = 8192;
    let mut buf = vec![0u8; READ_BUF_SIZE];
    let mut read = 0;

    // Read the response.
    loop {
        let n = io.read(&mut buf[read..]).await.map_err(|e| e.to_string())?;
        if n == 0 {
            return Err("Connection closed by proxy".to_string());
        }
        read += n;

        // Allocate space on the stack to read up to 16 headers from the proxy.
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut res = httparse::Response::new(&mut headers);
        match res.parse(&buf[..read]) {
            Ok(httparse::Status::Complete(len)) => {
                if res.code != Some(200) {
                    return Err(format!("Proxy returned status {}", res.code.unwrap_or(0)));
                }
                // Success!
                let remaining = read - len;
                let buffered_data = if remaining > 0 {
                    Some(Bytes::copy_from_slice(&buf[len..read]))
                } else {
                    None
                };

                let local_addr = io
                    .get_ref()
                    .get_local_address()
                    .to_string()
                    .into_boxed_str();
                let peer_addr = io.get_ref().get_peer_address().to_string().into_boxed_str();
                let network_type = io.get_ref().get_network_type();
                // Check for buffered data. In most cases, the buffer should be
                // empty as the server waits for the client to send the first
                // message, e.g. in TLS.
                let endpoint = if let Some(data) = buffered_data {
                    Rewind::new_buffered(io, data)
                } else {
                    Rewind::new_unbuffered(io)
                };
                return Ok(StreamEndpoint::new(
                    endpoint,
                    local_addr,
                    peer_addr,
                    network_type,
                ));
            }
            Ok(httparse::Status::Partial) => {
                if read >= READ_BUF_SIZE {
                    return Err("Response too large".to_string());
                }
            }
            Err(e) => {
                return Err(format!("Failed to parse HTTP response: {}", e));
            }
        }
    }
}

/// A credential wrapper that performs an HTTP CONNECT handshake before
/// delegating to an inner security credential (like TLS).
pub(crate) struct HttpConnectHandshaker {
    inner: Arc<dyn ChannelCredentials>,
    options: ProxyOptions,
}

impl HttpConnectHandshaker {
    /// Constructs a new `HttpConnectHandshaker` wrapping the inner credentials.
    pub(crate) fn new(inner: Arc<dyn ChannelCredentials>, options: &ProxyOptions) -> Self {
        Self {
            inner,
            options: options.clone(),
        }
    }
}

/// The I/O stream wrapper returned after the HTTP CONNECT handshake succeeds.
type ProxyStream<I> = StreamEndpoint<Rewind<EndpointIoStream<I>>>;

#[async_trait]
impl ChannelCredentials for HttpConnectHandshaker {
    fn info(&self) -> &ProtocolInfo {
        self.inner.info()
    }

    fn get_call_credentials(&self, token: private::Internal) -> Option<&Arc<dyn CallCredentials>> {
        self.inner.get_call_credentials(token)
    }

    async fn connect(
        &self,
        authority: &Authority,
        source: BoxEndpoint,
        info: &ClientHandshakeInfo,
        runtime: &GrpcRuntime,
        token: private::Internal,
    ) -> Result<HandshakeOutput, String> {
        // Perform the HTTP CONNECT handshake.
        let proxied_stream = do_connect_handshake(source, &self.options).await?;

        // Delegate the actual security handshake (e.g., TLS) to the wrapped
        // credentials.
        self.inner
            .connect(authority, Box::new(proxied_stream), info, runtime, token)
            .await
    }
}

#[cfg(test)]
mod test;
