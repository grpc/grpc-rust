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

use bssl_tls::alpn::H2;
use bssl_tls::context::TlsContextBuilder;
use bssl_tls::credentials::{Certificate, CertificateVerificationMode, TlsCredentialBuilder};
use bssl_tls_tokio::TokioTlsExt;
use bssl_x509::certificates::X509Certificate;
use bssl_x509::keys::PrivateKey;
use bssl_x509::params::Trust;
use bssl_x509::store::X509StoreBuilder;
use grpc::__unstable::Internal;
use grpc::__unstable::credentials::client::{ClientHandshakeInfo, ValidateAuthority};
use grpc::__unstable::credentials::common::Authority;
use grpc::__unstable::rt::{BoxEndpoint, EndpointIoStream, GrpcRuntime, StreamEndpoint};
use grpc::credentials::{ChannelCredentials, SecurityLevel, ServerCredentials};
use grpc_bssl::{BsslAuthorityValidator, BsslChannelCredentials, BsslServerCredentials};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CA_PEM: &[u8] = include_bytes!("../../examples/data/tls/ca.pem");
const SERVER_CERT_PEM: &[u8] = include_bytes!("../../examples/data/tls/server.pem");
const SERVER_KEY_PEM: &[u8] = include_bytes!("../../examples/data/tls/server.key");
const CLIENT_CA_PEM: &[u8] = include_bytes!("../../examples/data/tls/client_ca.pem");
const CLIENT1_CERT_PEM: &[u8] = include_bytes!("../../examples/data/tls/client1.pem");
const CLIENT1_KEY_PEM: &[u8] = include_bytes!("../../examples/data/tls/client1.key");

fn server_context_builder() -> TlsContextBuilder<bssl_tls::context::TlsMode> {
    let ca = Certificate::parse_one_from_pem(CA_PEM, None).unwrap();
    let server_cert = Certificate::parse_one_from_pem(SERVER_CERT_PEM, None).unwrap();
    let server_key = PrivateKey::from_pem(SERVER_KEY_PEM, || unreachable!()).unwrap();

    let mut cred_builder = TlsCredentialBuilder::new();
    cred_builder
        .with_certificate_chain(&[server_cert, ca])
        .unwrap()
        .with_private_key(server_key)
        .unwrap();

    let mut ctx_builder = TlsContextBuilder::new_tls();
    ctx_builder
        .with_credential(cred_builder.build().unwrap())
        .unwrap();
    ctx_builder
}

fn client_context_builder() -> TlsContextBuilder<bssl_tls::context::TlsMode> {
    let mut cert_store = X509StoreBuilder::new();
    cert_store
        .set_trust(Trust::SslServer)
        .unwrap()
        .add_cert(X509Certificate::parse_one_from_pem(CA_PEM).unwrap())
        .unwrap();

    let cert_store = cert_store.build();
    let mut ctx_builder = TlsContextBuilder::new_tls();
    ctx_builder.with_certificate_store(&cert_store);
    ctx_builder
}

fn server_mtls_context_builder() -> TlsContextBuilder<bssl_tls::context::TlsMode> {
    let mut ctx_builder = server_context_builder();
    let mut client_ca_store = X509StoreBuilder::new();
    client_ca_store
        .set_trust(Trust::SslClient)
        .unwrap()
        .add_cert(X509Certificate::parse_one_from_pem(CLIENT_CA_PEM).unwrap())
        .unwrap();

    let client_ca_store = client_ca_store.build();
    ctx_builder
        .with_certificate_store(&client_ca_store)
        .with_certificate_verification_mode(CertificateVerificationMode::PeerCertMandatory);
    ctx_builder
}

fn client_mtls_context_builder() -> TlsContextBuilder<bssl_tls::context::TlsMode> {
    let mut ctx_builder = client_context_builder();
    let client_ca = Certificate::parse_one_from_pem(CLIENT_CA_PEM, None).unwrap();
    let client_cert = Certificate::parse_one_from_pem(CLIENT1_CERT_PEM, None).unwrap();
    let client_key = PrivateKey::from_pem(CLIENT1_KEY_PEM, || unreachable!()).unwrap();

    let mut cred_builder = TlsCredentialBuilder::new();
    cred_builder
        .with_certificate_chain(&[client_cert, client_ca])
        .unwrap()
        .with_private_key(client_key)
        .unwrap();

    ctx_builder
        .with_credential(cred_builder.build().unwrap())
        .unwrap();
    ctx_builder
}

async fn run_handshake_pair(
    server_creds: Arc<BsslServerCredentials>,
    client_creds: Arc<BsslChannelCredentials>,
    authority_host: &str,
) -> (
    Result<grpc::__unstable::credentials::server::HandshakeOutput, String>,
    Result<grpc::__unstable::credentials::client::HandshakeOutput, String>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (tcp_stream, peer_addr) = listener.accept().await.unwrap();
        let local_addr = tcp_stream.local_addr().unwrap();
        let raw_endpoint: BoxEndpoint = Box::new(StreamEndpoint::new(
            tcp_stream,
            local_addr.to_string().into_boxed_str(),
            peer_addr.to_string().into_boxed_str(),
            "tcp",
        ));
        server_creds
            .accept(raw_endpoint, GrpcRuntime::default(), Internal)
            .await
    });

    let authority = Authority::new(authority_host, Some(addr.port()));
    let client_task = tokio::spawn(async move {
        let tcp_stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let local_addr = tcp_stream.local_addr().unwrap();
        let peer_addr = tcp_stream.peer_addr().unwrap();
        let raw_endpoint: BoxEndpoint = Box::new(StreamEndpoint::new(
            tcp_stream,
            local_addr.to_string().into_boxed_str(),
            peer_addr.to_string().into_boxed_str(),
            "tcp",
        ));
        let handshake_info = ClientHandshakeInfo::default();
        client_creds
            .connect(
                &authority,
                raw_endpoint,
                &handshake_info,
                &GrpcRuntime::default(),
                Internal,
            )
            .await
    });

    let (s_res, c_res) = tokio::join!(server_task, client_task);
    (s_res.unwrap(), c_res.unwrap())
}

#[tokio::test]
async fn test_tls_handshake_roundtrip() {
    let server_creds =
        Arc::new(BsslServerCredentials::from_context_builder(server_context_builder()).unwrap());
    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    assert_eq!(server_creds.info().security_protocol(), "tls");
    assert_eq!(client_creds.info().security_protocol(), "tls");

    let (server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let server_handshake = server_result.expect("server handshake should succeed");
    let client_handshake = client_result.expect("client handshake should succeed");

    assert_eq!(server_handshake.security.security_protocol(), "tls");
    assert_eq!(
        server_handshake.security.security_level(),
        SecurityLevel::PrivacyAndIntegrity
    );

    assert_eq!(client_handshake.security_info.security_protocol(), "tls");
    assert_eq!(
        client_handshake.security_info.security_level(),
        SecurityLevel::PrivacyAndIntegrity
    );
    assert!(
        client_handshake
            .authority_validator
            .validate_authority(&Authority::new("localhost", None))
    );

    // Verify mutual I/O over encrypted streams
    let mut server_io = EndpointIoStream::new(server_handshake.endpoint);
    let mut client_io = EndpointIoStream::new(client_handshake.endpoint);

    client_io.write_all(b"ping!").await.unwrap();
    client_io.flush().await.unwrap();

    let mut buf = [0u8; 5];
    server_io.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"ping!");

    server_io.write_all(b"pong!").await.unwrap();
    server_io.flush().await.unwrap();

    let mut resp_buf = [0u8; 5];
    client_io.read_exact(&mut resp_buf).await.unwrap();
    assert_eq!(&resp_buf, b"pong!");
}

#[tokio::test]
async fn test_tls_client_handshake_no_alpn() {
    // Server provides NO ALPN
    let server_ctx = server_context_builder();
    let server_creds = Arc::new(BsslServerCredentials::new(
        server_ctx.build_tokio_acceptor(),
    ));

    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    let (_server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let err = match client_result {
        Ok(_) => panic!("client handshake should fail when server offers no ALPN"),
        Err(e) => e,
    };
    assert!(
        err.contains("Server did not negotiate ALPN")
            || err.contains("NoApplicationProtocol")
            || err.contains("Tlsv1AlertNoApplicationProtocol"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn test_tls_client_handshake_bad_alpn() {
    // Server provides only http/1.1
    let mut server_ctx = server_context_builder();
    server_ctx.set_alpn_protocols([&b"http/1.1"[..]]).unwrap();
    let server_creds = Arc::new(BsslServerCredentials::new(
        server_ctx.build_tokio_acceptor(),
    ));

    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    let (_server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let err = match client_result {
        Ok(_) => panic!("client handshake should fail when server offers bad ALPN"),
        Err(e) => e,
    };
    assert!(
        err.contains("unexpected ALPN protocol")
            || err.contains("NoApplicationProtocol")
            || err.contains("Tlsv1AlertNoApplicationProtocol"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn test_tls_client_handshake_alpn_h1_and_h2() {
    // Server offers http/1.1 and h2; client negotiates h2
    let mut server_ctx = server_context_builder();
    server_ctx
        .set_alpn_protocols([&b"http/1.1"[..], H2])
        .unwrap();
    let server_creds = Arc::new(BsslServerCredentials::new(
        server_ctx.build_tokio_acceptor(),
    ));

    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    let (server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    assert!(server_result.is_ok(), "server handshake should succeed");
    assert!(
        client_result.is_ok(),
        "client handshake should succeed with h2"
    );
}

#[tokio::test]
async fn test_tls_server_handshake_no_alpn() {
    // Server requires h2; client offers NO ALPN
    let server_creds =
        Arc::new(BsslServerCredentials::from_context_builder(server_context_builder()).unwrap());

    let client_ctx = client_context_builder();
    let client_creds = Arc::new(BsslChannelCredentials::new(
        client_ctx.build_tokio_connector(),
    ));

    let (server_result, _client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let err = match server_result {
        Ok(_) => panic!("server handshake should fail when client offers no ALPN"),
        Err(e) => e,
    };
    assert!(
        err.contains("Client did not negotiate ALPN")
            || err.contains("NoApplicationProtocol")
            || err.contains("Tlsv1AlertNoApplicationProtocol"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn test_tls_server_handshake_bad_alpn() {
    // Server requires h2; client offers only http/1.1
    let server_creds =
        Arc::new(BsslServerCredentials::from_context_builder(server_context_builder()).unwrap());

    let mut client_ctx = client_context_builder();
    client_ctx.set_alpn_protocols([&b"http/1.1"[..]]).unwrap();
    let client_creds = Arc::new(BsslChannelCredentials::new(
        client_ctx.build_tokio_connector(),
    ));

    let (server_result, _client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let err = match server_result {
        Ok(_) => panic!("server handshake should fail when client offers bad ALPN"),
        Err(e) => e,
    };
    assert!(
        err.contains("unexpected ALPN protocol")
            || err.contains("NoApplicationProtocol")
            || err.contains("Tlsv1AlertNoApplicationProtocol"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn test_tls_handshake_wrong_server_name() {
    // Certificate is valid for localhost and example.com, but NOT for wrong.host.invalid
    let server_creds =
        Arc::new(BsslServerCredentials::from_context_builder(server_context_builder()).unwrap());
    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    let (_server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "wrong.host.invalid").await;

    let err = match client_result {
        Ok(_) => panic!("client should fail hostname verification"),
        Err(e) => e,
    };
    assert!(
        err.contains("handshake failed") || err.contains("BoringSSL TLS client handshake failed"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn test_tls_validate_authority() {
    let validator = BsslAuthorityValidator::new("localhost");
    assert!(validator.validate_authority(&Authority::new("localhost", None)));
    assert!(validator.validate_authority(&Authority::new("localhost", Some(50051))));
    assert!(!validator.validate_authority(&Authority::new("other.host", None)));
}

#[tokio::test]
async fn test_tls_server_mtls_require_fail() {
    // Server requires client certificate (PeerCertMandatory), but client sends none
    let server_creds = Arc::new(
        BsslServerCredentials::from_context_builder(server_mtls_context_builder()).unwrap(),
    );
    let client_creds =
        Arc::new(BsslChannelCredentials::from_context_builder(client_context_builder()).unwrap());

    let (server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    assert!(
        server_result.is_err() || client_result.is_err(),
        "handshake should fail when required client certificate is omitted"
    );
}

#[tokio::test]
async fn test_tls_server_mtls_success() {
    // Server requires client certificate, and client provides trusted client1 cert
    let server_creds = Arc::new(
        BsslServerCredentials::from_context_builder(server_mtls_context_builder()).unwrap(),
    );
    let client_creds = Arc::new(
        BsslChannelCredentials::from_context_builder(client_mtls_context_builder()).unwrap(),
    );

    let (server_result, client_result) =
        run_handshake_pair(server_creds, client_creds, "localhost").await;

    let server_handshake = server_result.expect("server mTLS handshake should succeed");
    let client_handshake = client_result.expect("client mTLS handshake should succeed");

    // Verify mutual I/O over mTLS
    let mut server_io = EndpointIoStream::new(server_handshake.endpoint);
    let mut client_io = EndpointIoStream::new(client_handshake.endpoint);

    client_io.write_all(b"mtls!").await.unwrap();
    client_io.flush().await.unwrap();

    let mut buf = [0u8; 5];
    server_io.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"mtls!");

    server_io.write_all(b"done!").await.unwrap();
    server_io.flush().await.unwrap();

    let mut resp_buf = [0u8; 5];
    client_io.read_exact(&mut resp_buf).await.unwrap();
    assert_eq!(&resp_buf, b"done!");
}
