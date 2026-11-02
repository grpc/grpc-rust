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

//! [`ChannelCredentials`] implementations used by the bootstrap config.

use std::sync::Arc;

use grpc::__unstable::UnstableToken;
use grpc::__unstable::credentials::client::ClientHandshakeInfo;
use grpc::__unstable::credentials::client::HandshakeOutput;
use grpc::__unstable::credentials::client::ValidateAuthority;
use grpc::__unstable::credentials::common::Authority;
use grpc::__unstable::rt::BoxEndpoint;
use grpc::__unstable::rt::GrpcRuntime;
use grpc::async_trait;
use grpc::credentials::ChannelCredentials;
use grpc::credentials::ProtocolInfo;
use grpc::credentials::SecurityInfo;
use grpc::credentials::SecurityLevel;
use grpc::credentials::call::CallCredentials;

const INSECURE_PROTOCOL_NAME: &str = "insecure";
static INSECURE_PROTO_INFO: ProtocolInfo = ProtocolInfo::unstable_new(INSECURE_PROTOCOL_NAME);

/// [`ChannelCredentials`] that perform no encryption or authentication, using
/// plaintext over the raw transport, regardless of whether the peer is local.
///
/// This is intentionally crate-private, as grpc-rust does not publicly support
/// unencrypted connections over the network. It exists to support the
/// `"insecure"` channel credential type in the xDS bootstrap config.
#[derive(Debug, Clone, Default)]
pub struct InsecureChannelCredentials;

impl InsecureChannelCredentials {
    pub fn new() -> Self {
        Self
    }
}

/// Allows any authority, as no peer identity is established.
#[derive(Debug)]
struct InsecureAuthorityValidator;

impl ValidateAuthority for InsecureAuthorityValidator {
    fn validate_authority(&self, _authority: &Authority) -> bool {
        true
    }
}

#[async_trait]
impl ChannelCredentials for InsecureChannelCredentials {
    async fn connect(
        &self,
        _authority: &Authority,
        source: BoxEndpoint,
        _info: &ClientHandshakeInfo,
        _runtime: &GrpcRuntime,
        _token: UnstableToken,
    ) -> Result<HandshakeOutput, String> {
        Ok(HandshakeOutput {
            endpoint: source,
            security_info: SecurityInfo::new(INSECURE_PROTOCOL_NAME)
                .with_security_level(SecurityLevel::NoSecurity),
            authority_validator: Box::new(InsecureAuthorityValidator),
        })
    }

    fn info(&self) -> &ProtocolInfo {
        &INSECURE_PROTO_INFO
    }

    fn get_call_credentials(&self, _: UnstableToken) -> Option<&Arc<dyn CallCredentials>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use grpc::__unstable::rt;
    use grpc::__unstable::rt::EndpointIoStream;
    use grpc::__unstable::rt::TcpOptions;
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    use super::*;

    #[tokio::test]
    async fn insecure_connect_passes_through_plaintext() {
        let creds = InsecureChannelCredentials::new();
        assert_eq!(creds.info().security_protocol(), "insecure");

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        let authority = Authority::new("example.com", Some(server_addr.port()));

        let runtime = rt::default_runtime();
        let endpoint = runtime
            .tcp_stream(server_addr, TcpOptions::default())
            .await
            .unwrap();

        let output = creds
            .connect(
                &authority,
                endpoint,
                &ClientHandshakeInfo::default(),
                &runtime,
                UnstableToken,
            )
            .await
            .unwrap();

        assert_eq!(output.security_info.security_protocol(), "insecure");
        assert_eq!(
            output.security_info.security_level(),
            SecurityLevel::NoSecurity
        );
        assert!(output.authority_validator.validate_authority(&authority));

        let (mut server_stream, _) = listener.accept().await.unwrap();
        let test_data = b"hello grpc";
        server_stream.write_all(test_data).await.unwrap();

        let mut buf = vec![0u8; test_data.len()];
        EndpointIoStream::new(output.endpoint)
            .read_exact(&mut buf)
            .await
            .unwrap();
        assert_eq!(buf, test_data);
    }
}
