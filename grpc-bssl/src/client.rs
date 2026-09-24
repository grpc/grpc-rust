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
use std::sync::Arc;

use bssl_tls::alpn::H2;
use bssl_tls::context::TlsContextBuilder;
use bssl_tls_tokio::{TlsConnector, TokioTlsExt};
use grpc::__unstable::Internal;
use grpc::__unstable::credentials::client::{
    ClientHandshakeInfo, HandshakeOutput, ValidateAuthority,
};
use grpc::__unstable::credentials::common::Authority;
use grpc::__unstable::rt::{BoxEndpoint, EndpointIoStream, GrpcRuntime, StreamEndpoint};
use grpc::credentials::call::CallCredentials;
use grpc::credentials::{ChannelCredentials, ProtocolInfo, SecurityInfo, SecurityLevel};
use tonic::async_trait;

/// Authority validator for BoringSSL-based channel credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsslAuthorityValidator {
    expected_host: String,
}

impl BsslAuthorityValidator {
    /// Creates a new validator with the expected server authority host.
    pub fn new(expected_host: impl Into<String>) -> Self {
        Self {
            expected_host: expected_host.into(),
        }
    }
}

impl ValidateAuthority for BsslAuthorityValidator {
    fn validate_authority(&self, authority: &Authority) -> bool {
        self.expected_host == authority.host()
    }
}

/// Client-side transport security credentials powered by BoringSSL (`bssl-tls`).
pub struct BsslChannelCredentials {
    connector: TlsConnector,
    protocol_info: ProtocolInfo,
}

impl fmt::Debug for BsslChannelCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BsslChannelCredentials")
            .field("protocol_info", &self.protocol_info.security_protocol())
            .finish()
    }
}

impl BsslChannelCredentials {
    /// Creates credentials with an existing [`TlsConnector`].
    pub fn new(connector: TlsConnector) -> Self {
        Self {
            connector,
            protocol_info: ProtocolInfo::new("tls"),
        }
    }

    /// Creates credentials from a [`TlsContextBuilder`], automatically configuring ALPN for HTTP/2 (`h2`).
    pub fn from_context_builder(
        mut builder: TlsContextBuilder<bssl_tls::context::TlsMode>,
    ) -> Result<Self, bssl_tls::errors::Error> {
        builder.set_alpn_protocols([H2])?;
        Ok(Self::new(builder.build_tokio_connector()))
    }
}

#[async_trait]
impl ChannelCredentials for BsslChannelCredentials {
    fn info(&self) -> &ProtocolInfo {
        &self.protocol_info
    }

    fn get_call_credentials(&self, _token: Internal) -> Option<&Arc<dyn CallCredentials>> {
        None
    }

    async fn connect(
        &self,
        authority: &Authority,
        source: BoxEndpoint,
        _info: &ClientHandshakeInfo,
        _runtime: &GrpcRuntime,
        _token: Internal,
    ) -> Result<HandshakeOutput, String> {
        let local_addr: Box<str> = source.get_local_address().into();
        let peer_addr: Box<str> = source.get_peer_address().into();
        let network_type = source.get_network_type();

        let host = authority.host();
        let input_io = EndpointIoStream::new(source);

        let mut tls_stream = self
            .connector
            .connect(host, input_io)
            .await
            .map_err(|e| format!("BoringSSL TLS client handshake failed: {e}"))?;

        let alpn = if let Some(est) = tls_stream.get_mut().established() {
            est.get_selected_alpn().map(|s| s.to_vec())
        } else {
            None
        };

        if let Some(selected) = alpn {
            if selected != H2 {
                return Err(format!(
                    "Server negotiated unexpected ALPN protocol: {:?}",
                    String::from_utf8_lossy(&selected)
                ));
            }
        } else {
            return Err("Server did not negotiate ALPN (h2 required)".into());
        }

        let endpoint: BoxEndpoint = Box::new(StreamEndpoint::new(
            tls_stream,
            local_addr,
            peer_addr,
            network_type,
        ));

        let security_info =
            SecurityInfo::new("tls").with_security_level(SecurityLevel::PrivacyAndIntegrity);

        Ok(HandshakeOutput {
            endpoint,
            security_info,
            authority_validator: Box::new(BsslAuthorityValidator::new(host)),
        })
    }
}
