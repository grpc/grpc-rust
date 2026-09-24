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

use std::fmt;

use bssl_tls::alpn::H2;
use bssl_tls::context::TlsContextBuilder;
use bssl_tls_tokio::{TlsAcceptor, TokioTlsExt};
use grpc::__unstable::Internal;
use grpc::__unstable::credentials::server::HandshakeOutput;
use grpc::__unstable::rt::{BoxEndpoint, EndpointIoStream, GrpcRuntime, StreamEndpoint};
use grpc::credentials::{ProtocolInfo, SecurityInfo, SecurityLevel, ServerCredentials};
use tonic::async_trait;

/// Server-side transport security credentials powered by BoringSSL (`bssl-tls`).
pub struct BsslServerCredentials {
    acceptor: TlsAcceptor,
    protocol_info: ProtocolInfo,
}

impl fmt::Debug for BsslServerCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BsslServerCredentials")
            .field("protocol_info", &self.protocol_info.security_protocol())
            .finish()
    }
}

impl BsslServerCredentials {
    /// Creates credentials with an existing [`TlsAcceptor`].
    pub fn new(acceptor: TlsAcceptor) -> Self {
        Self {
            acceptor,
            protocol_info: ProtocolInfo::new("tls"),
        }
    }

    /// Creates credentials from a [`TlsContextBuilder`], automatically configuring ALPN for HTTP/2 (`h2`).
    pub fn from_context_builder(
        mut builder: TlsContextBuilder<bssl_tls::context::TlsMode>,
    ) -> Result<Self, bssl_tls::errors::Error> {
        builder.set_alpn_protocols([H2])?;
        Ok(Self::new(builder.build_tokio_acceptor()))
    }
}

#[async_trait]
impl ServerCredentials for BsslServerCredentials {
    fn info(&self) -> &ProtocolInfo {
        &self.protocol_info
    }

    async fn accept(
        &self,
        source: BoxEndpoint,
        _runtime: GrpcRuntime,
        _token: Internal,
    ) -> Result<HandshakeOutput, String> {
        let local_addr: Box<str> = source.get_local_address().into();
        let peer_addr: Box<str> = source.get_peer_address().into();
        let network_type = source.get_network_type();

        let input_io = EndpointIoStream::new(source);

        let mut tls_stream = self
            .acceptor
            .accept(input_io)
            .await
            .map_err(|e| format!("BoringSSL TLS server accept failed: {e}"))?;

        let alpn = if let Some(est) = tls_stream.get_mut().established() {
            est.get_selected_alpn().map(|s| s.to_vec())
        } else {
            None
        };

        if let Some(selected) = alpn {
            if selected != H2 {
                return Err(format!(
                    "Client negotiated unexpected ALPN protocol: {:?}",
                    String::from_utf8_lossy(&selected)
                ));
            }
        } else {
            return Err("Client did not negotiate ALPN (h2 required)".into());
        }

        let endpoint: BoxEndpoint = Box::new(StreamEndpoint::new(
            tls_stream,
            local_addr,
            peer_addr,
            network_type,
        ));

        let security =
            SecurityInfo::new("tls").with_security_level(SecurityLevel::PrivacyAndIntegrity);

        Ok(HandshakeOutput { endpoint, security })
    }
}
