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

use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use bytes::Buf;
use bytes::Bytes;
use h2::Reason;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::Response;
use tonic::Status as TonicStatus;
use tonic::metadata::MetadataMap as TonicMetadata;
use tonic::metadata::MetadataValue as TonicMetadataValue;
use tonic_prost::prost::Message as ProstMessage;

use crate::StatusCodeError;
use crate::StatusError;
use crate::async_trait;
use crate::attributes::Attributes;
use crate::client::CallOptions;
use crate::client::Channel;
use crate::client::DynInvoke;
use crate::client::Invoke as _;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream as _;
use crate::client::Trailers;
use crate::client::name_resolution::TCP_IP_NETWORK_TYPE;
use crate::client::transport::SecurityOpts;
use crate::client::transport::TransportOptions;
use crate::client::transport::registry::GLOBAL_TRANSPORT_REGISTRY;
use crate::core::Address;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::credentials::ChannelCredentials;
use crate::credentials::CompositeChannelCredentials;
use crate::credentials::LocalChannelCredentials;
use crate::credentials::ProtocolInfo;
use crate::credentials::SecurityInfo;
use crate::credentials::SecurityLevel;
use crate::credentials::call::CallCredentials;
use crate::credentials::call::CallDetails;
use crate::credentials::call::ClientConnectionSecurityInfo;
use crate::credentials::client::ClientHandshakeInfo;
use crate::credentials::client::HandshakeOutput;
use crate::credentials::client::ValidateAuthority;
use crate::credentials::common::Authority;
use crate::echo_pb::EchoRequest;
use crate::echo_pb::EchoResponse;
use crate::metadata::AsciiMetadataKey;
use crate::metadata::MetadataMap;
use crate::private;
use crate::rt::BoxEndpoint;
use crate::rt::GrpcRuntime;
use crate::rt::tokio::TokioRuntime;
use crate::stub_server::EchoStream;
use crate::stub_server::StubServer;
use crate::stub_server::echo_stream;

#[derive(Debug)]
struct MockCallCredentials {
    metadata: Vec<(&'static str, &'static str)>,
    min_security_level: SecurityLevel,
    should_fail: Option<crate::StatusError>,
}

#[async_trait]
impl CallCredentials for MockCallCredentials {
    async fn get_metadata(
        &self,
        _call_details: &CallDetails,
        _auth_info: &ClientConnectionSecurityInfo,
        metadata: &mut MetadataMap,
    ) -> Result<(), crate::StatusError> {
        if let Some(status) = &self.should_fail {
            return Err(status.clone());
        }
        for (key, val) in &self.metadata {
            metadata.insert(
                key.parse::<AsciiMetadataKey>().unwrap(),
                val.parse().unwrap(),
            );
        }
        Ok(())
    }

    fn minimum_channel_security_level(&self) -> SecurityLevel {
        self.min_security_level
    }
}

const DEFAULT_TEST_DURATION: Duration = Duration::from_secs(10);
const DEFAULT_TEST_SHORT_DURATION: Duration = Duration::from_millis(10);

// Tests the tonic transport by creating a bi-di stream with a tonic server.
#[tokio::test]
pub(crate) async fn tonic_transport_rpc() {
    super::reg();
    let server = StubServer::new().start().await;
    let addr = server.addr();
    println!("EchoServer listening on: {addr}");

    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let config = Arc::new(TransportOptions::default());
    let securty_opts = SecurityOpts {
        credentials: LocalChannelCredentials::new_arc(),
        authority: Authority::new("localhost".to_string(), None),
        handshake_info: ClientHandshakeInfo::default(),
    };
    let address = Address {
        network_type: TCP_IP_NETWORK_TYPE,
        address: addr.to_string().into(),
        attributes: Attributes::new(),
    };
    let (conn, _sec_info, mut disconnection_listener) = builder
        .dyn_connect(
            &address,
            GrpcRuntime::new(TokioRuntime::default()),
            &securty_opts,
            &config,
        )
        .await
        .unwrap();

    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new()
                .with_method_name("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            CallOptions::default(),
        )
        .await;

    // Spawn a sender task
    let client_handle = tokio::spawn(async move {
        let mut dummy_msg = WrappedEchoResponse(EchoResponse { message: "".into() });
        match rx.recv(&mut dummy_msg).await {
            ResponseStreamItem::Headers(_) => {
                println!("Got headers");
            }
            item => panic!("Expected headers, got {:?}", item),
        }

        for i in 0..5 {
            let message = format!("message {i}");
            let request = EchoRequest {
                message: message.clone(),
            };

            let req = WrappedEchoRequest(request);

            println!("Sent request: {:?}", req.0);
            assert!(
                tx.send(&req, SendOptions::default()).await.is_ok(),
                "Receiver dropped"
            );

            // Wait for the reply
            let mut recv_msg = WrappedEchoResponse(EchoResponse { message: "".into() });
            match rx.recv(&mut recv_msg).await {
                ResponseStreamItem::Message => {
                    let echo_response = recv_msg.0;
                    println!("Got response: {echo_response:?}");
                    assert_eq!(echo_response.message, message);
                }
                item => panic!("Expected message, got {:?}", item),
            }
        }
    });

    client_handle.await.unwrap();
    // The connection should break only after the server is stopped. Stopping
    // waits for the server to exit, so wait for the disconnection
    // concurrently.
    assert_eq!(
        disconnection_listener.try_recv(),
        Err(oneshot::error::TryRecvError::Empty),
    );
    let (res, ()) = tokio::join!(
        async {
            GrpcRuntime::default()
                .timeout(DEFAULT_TEST_DURATION, disconnection_listener)
                .await
                .unwrap()
                .unwrap()
        },
        server.stop(),
    );
    assert_eq!(res, Ok(()));
}

#[tokio::test]
async fn grpc_invoke_tonic_unary() {
    let server = StubServer::new().start().await;
    let addr = server.addr();

    // Create the channel.
    let target = format!("dns:///{}", addr);
    let channel = Channel::builder(&target, LocalChannelCredentials::new_arc()).build();

    let (headers, resp, trailers) = perform_unary_echo(&channel, "hello interop").await;
    assert_eq!(resp.message, "hello interop");

    let connection_info = headers.connection_info();
    assert_eq!(
        connection_info.local_address().network_type,
        TCP_IP_NETWORK_TYPE
    );
    let local_addr: SocketAddr = connection_info.local_address().address.parse().unwrap();
    assert_eq!(local_addr.ip(), addr.ip());
    assert_eq!(
        connection_info.remote_address().network_type,
        TCP_IP_NETWORK_TYPE
    );
    assert_eq!(
        connection_info.remote_address().address.to_string(),
        addr.to_string()
    );
    assert_eq!(connection_info.security_info().security_protocol(), "local");

    assert!(
        trailers.connection_info().is_none(),
        "trailers should not contain connection_info when headers were present; had {:?}",
        trailers.connection_info().as_ref().unwrap()
    );

    assert!(
        trailers.status().is_ok(),
        "RPC failed: {:?}",
        trailers.status()
    );

    server.stop().await;
}

#[cfg(unix)]
mod unix_tests {
    use std::path::Component;
    use std::path::Path;
    use std::path::PathBuf;

    use tempfile::tempdir;
    use tokio::net::UnixListener;
    use tokio_stream::wrappers::UnixListenerStream;

    use super::*;
    use crate::client::name_resolution::UNIX_NETWORK_TYPE;

    async fn run_unix_test(bind_path: &PathBuf, target: &str) {
        let listener = UnixListener::bind(bind_path).unwrap();
        let expected_remote_addr = bind_path.to_string_lossy().into_owned();
        let channel = Channel::builder(target, LocalChannelCredentials::new_arc()).build();

        let server = StubServer::new().start_with_incoming(UnixListenerStream::new(listener));

        let payload = "hello unix";
        let (headers, resp, trailers) = perform_unary_echo(&channel, payload).await;
        assert_eq!(resp.message, payload);
        assert!(trailers.status().is_ok());

        let connection_info = headers.connection_info();
        assert_eq!(
            connection_info.local_address().network_type,
            UNIX_NETWORK_TYPE
        );
        // The client end of a Unix connection is unnamed.
        assert_eq!(&*connection_info.local_address().address, "");
        assert_eq!(
            connection_info.remote_address().network_type,
            UNIX_NETWORK_TYPE
        );
        assert_eq!(
            connection_info.remote_address().address.to_string(),
            expected_remote_addr
        );
        assert_eq!(connection_info.security_info().security_protocol(), "local");
        assert!(
            trailers.connection_info().is_none(),
            "trailers should not contain connection_info when headers were present; had {:?}",
            trailers.connection_info().as_ref().unwrap()
        );

        server.stop().await;
    }

    #[tokio::test]
    async fn unix_absolute_path() {
        let dir = tempdir().expect("failed to create temp dir");
        let socket_path = dir.path().join("absolute.sock");
        let target = format!("unix://{}", socket_path.to_str().unwrap());

        run_unix_test(&socket_path, &target).await;
    }

    #[tokio::test]
    async fn unix_relative_path() {
        let dir = tempdir().expect("failed to create temp dir");
        let socket_name = "relative.sock";
        let socket_path = dir.path().join(socket_name);

        // We calculate the socket file's path relative to the current
        // directory to avoid changing the working directory and interfering
        // with other tests.
        let current_dir = std::env::current_dir().expect("failed to fetch current directory");
        let relative_path = get_relative_path(&socket_path, &current_dir).unwrap();
        let target = format!("unix:{}", relative_path.display());

        run_unix_test(&socket_path, &target).await;

        std::env::set_current_dir(current_dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn unix_abstract_socket() {
        let abstract_path = format!("grpc-test-abstract-socket-{}", rand::random::<u64>());
        let bind_path = format!("\0{}", abstract_path);
        let target = format!("unix-abstract:{}", abstract_path);

        run_unix_test(&PathBuf::from(bind_path), &target).await;
    }

    /// Calculates the relative path from a `base` directory to a `target` path.
    ///
    /// Both paths should be absolute. This operation is infallible on Unix
    /// systems due to the presence of a single root directory.
    fn get_relative_path(target: &Path, base: &Path) -> Result<PathBuf, String> {
        let mut target_components = target.components();
        let mut base_components = base.components();

        // Find the common prefix between the two paths.
        let mut common_components = 0;
        loop {
            match (
                target_components.clone().next(),
                base_components.clone().next(),
            ) {
                (Some(t), Some(b)) if t == b => {
                    target_components.next();
                    base_components.next();
                    common_components += 1;
                }
                _ => break,
            }
        }

        // If they share absolutely nothing (e.g., C:\\ vs D:\\ on Windows), we can't
        // make it relative.
        if common_components == 0 {
            return Err("no common ancestor".to_owned());
        }

        let mut relative_path = PathBuf::new();

        // For every component left in the base path, we need to go up one directory
        // ("..").
        for _ in base_components {
            relative_path.push(Component::ParentDir);
        }

        // Append the remaining components of the target path.
        for component in target_components {
            relative_path.push(component);
        }

        Ok(relative_path)
    }
}

#[tokio::test]
async fn grpc_invoke_failure_cases() {
    let server = StubServer::new().start().await;
    let addr = server.addr();

    let target = format!("dns:///{}", addr);

    // Security level mismatch (MockCallCredentials requires PrivacyAndIntegrity,
    // but LocalChannelCredentials provides NoSecurity over TCP).
    {
        let creds = LocalChannelCredentials::new();
        let call_creds = Arc::new(MockCallCredentials {
            metadata: vec![],
            min_security_level: SecurityLevel::PrivacyAndIntegrity,
            should_fail: None,
        });
        let composite_creds = CompositeChannelCredentials::new(creds, call_creds);
        let channel = Channel::builder(&target, Arc::new(composite_creds)).build();

        let trailers = perform_unary_echo_failure(&channel).await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            StatusCodeError::Unauthenticated
        );
        let connection_info = trailers
            .connection_info()
            .as_ref()
            .expect("connection_info should be present in trailers");
        assert_eq!(
            connection_info.remote_address().network_type,
            TCP_IP_NETWORK_TYPE
        );
        assert_eq!(
            connection_info.remote_address().address.to_string(),
            addr.to_string()
        );
        assert_eq!(connection_info.security_info().security_protocol(), "local");
    }

    // Call credentials return error
    {
        let creds = LocalChannelCredentials::new();
        let call_creds = Arc::new(MockCallCredentials {
            metadata: vec![],
            min_security_level: SecurityLevel::NoSecurity,
            should_fail: Some(crate::StatusError::new(
                StatusCodeError::PermissionDenied,
                "test message",
            )),
        });
        let composite_creds = CompositeChannelCredentials::new(creds, call_creds);
        let channel = Channel::builder(&target, Arc::new(composite_creds)).build();

        let trailers = perform_unary_echo_failure(&channel).await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            StatusCodeError::PermissionDenied
        );
        assert!(
            trailers
                .status()
                .as_ref()
                .unwrap_err()
                .message()
                .contains("test message")
        );
        let connection_info = trailers
            .connection_info()
            .as_ref()
            .expect("connection_info should be present in trailers");
        assert_eq!(
            connection_info.remote_address().network_type,
            TCP_IP_NETWORK_TYPE
        );
        assert_eq!(
            connection_info.remote_address().address.to_string(),
            addr.to_string()
        );
        assert_eq!(connection_info.security_info().security_protocol(), "local");
    }

    // Call credentials return restricted control plane code (mapped to Internal)
    {
        let creds = LocalChannelCredentials::new();
        let call_creds = Arc::new(MockCallCredentials {
            metadata: vec![],
            min_security_level: SecurityLevel::NoSecurity,
            should_fail: Some(StatusError::new(
                StatusCodeError::InvalidArgument,
                "test message",
            )),
        });
        let composite_creds = CompositeChannelCredentials::new(creds, call_creds);
        let channel = Channel::builder(&target, Arc::new(composite_creds)).build();

        let trailers = perform_unary_echo_failure(&channel).await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            StatusCodeError::Internal
        );
        assert!(
            trailers
                .status()
                .as_ref()
                .unwrap_err()
                .message()
                .contains("test message")
        );
        let connection_info = trailers
            .connection_info()
            .as_ref()
            .expect("connection_info should be present in trailers");
        assert_eq!(
            connection_info.remote_address().network_type,
            TCP_IP_NETWORK_TYPE
        );
        assert_eq!(
            connection_info.remote_address().address.to_string(),
            addr.to_string()
        );
        assert_eq!(connection_info.security_info().security_protocol(), "local");
    }

    server.stop().await;
}

async fn perform_unary_echo(
    channel: &Channel,
    message: &str,
) -> (ResponseHeaders, EchoResponse, Trailers) {
    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/UnaryEcho"),
            CallOptions::default(),
        )
        .await;

    let req = WrappedEchoRequest(EchoRequest {
        message: message.into(),
    });
    tx.send(
        &req,
        SendOptions {
            final_msg: true,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let mut resp = WrappedEchoResponse(EchoResponse::default());

    let ResponseStreamItem::Headers(headers) = rx.recv(&mut resp).await else {
        panic!("Expected Headers first");
    };

    let ResponseStreamItem::Message = rx.recv(&mut resp).await else {
        panic!("Expected Message after Headers");
    };
    let echo_resp = std::mem::take(&mut resp.0);

    let ResponseStreamItem::Trailers(trailers) = rx.recv(&mut resp).await else {
        panic!("Expected Trailers, got StreamClosed or other item");
    };

    (headers, echo_resp, trailers)
}

async fn perform_unary_echo_failure(channel: &Channel) -> Trailers {
    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/UnaryEcho"),
            CallOptions::default(),
        )
        .await;

    let req = WrappedEchoRequest(EchoRequest::default());
    _ = tx
        .send(
            &req,
            SendOptions {
                final_msg: true,
                ..Default::default()
            },
        )
        .await;

    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let ResponseStreamItem::Trailers(t) = rx.recv(&mut resp).await else {
        panic!("Expected Trailers due to failure");
    };
    t
}

#[tokio::test]
async fn tonic_transport_invalid_base64_headers() {
    super::reg();

    let server = StubServer::new()
        .with_bidirectional_streaming_echo(|request| async {
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("test-bin"),
                HeaderValue::from_static("invalid base64 data"),
            );
            let response_headers = TonicMetadata::from_headers(headers);
            let mut response = Response::new(echo_stream(request.into_inner()));
            *response.metadata_mut() = response_headers;
            Ok(response)
        })
        .start()
        .await;
    let addr = server.addr();

    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let config = Arc::new(TransportOptions::default());
    let securty_opts = SecurityOpts {
        credentials: LocalChannelCredentials::new_arc(),
        authority: Authority::new("localhost".to_string(), None),
        handshake_info: ClientHandshakeInfo::default(),
    };
    let address = Address {
        network_type: TCP_IP_NETWORK_TYPE,
        address: addr.to_string().into(),
        attributes: Attributes::new(),
    };
    let (conn, _sec_info, _disconnection_listener) = builder
        .dyn_connect(
            &address,
            GrpcRuntime::new(TokioRuntime::default()),
            &securty_opts,
            &config,
        )
        .await
        .unwrap();

    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new()
                .with_method_name("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            CallOptions::default(),
        )
        .await;

    let mut dummy_msg = WrappedEchoResponse(EchoResponse { message: "".into() });

    match rx.recv(&mut dummy_msg).await {
        ResponseStreamItem::Trailers(trailers) => {
            println!("Got trailers as expected due to invalid headers");
            let status = trailers.status().as_ref().unwrap_err();
            assert_eq!(status.code(), StatusCodeError::Internal);
        }
        item => panic!("Expected Trailers with error, got {:?}", item),
    }

    let request = EchoRequest {
        message: "hello".into(),
    };
    let req = WrappedEchoRequest(request);

    GrpcRuntime::default()
        .timeout(DEFAULT_TEST_DURATION, async {
            while tx.send(&req, SendOptions::default()).await.is_ok() {}
        })
        .await
        .expect("timed out waiting for stream to close");

    server.stop().await;
}

#[tokio::test]
async fn tonic_transport_recv_drop_cancels_send() {
    super::reg();
    let server = StubServer::new().start().await;
    let addr = server.addr();

    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let config = Arc::new(TransportOptions::default());
    let securty_opts = SecurityOpts {
        credentials: LocalChannelCredentials::new_arc(),
        authority: Authority::new("localhost".to_string(), None),
        handshake_info: ClientHandshakeInfo::default(),
    };
    let address = Address {
        network_type: TCP_IP_NETWORK_TYPE,
        address: addr.to_string().into(),
        attributes: Attributes::new(),
    };
    let (conn, _sec_info, _disconnection_listener) = builder
        .dyn_connect(
            &address,
            GrpcRuntime::new(TokioRuntime::default()),
            &securty_opts,
            &config,
        )
        .await
        .unwrap();

    let (mut tx, rx) = conn
        .dyn_invoke(
            RequestHeaders::new()
                .with_method_name("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            CallOptions::default(),
        )
        .await;

    drop(rx);

    let request = EchoRequest {
        message: "hello".into(),
    };
    let req = WrappedEchoRequest(request);

    GrpcRuntime::default()
        .timeout(DEFAULT_TEST_DURATION, async {
            while tx.send(&req, SendOptions::default()).await.is_ok() {}
        })
        .await
        .expect("timed out waiting for stream to close");

    server.stop().await;
}

#[derive(Debug, Clone)]
struct MockConnectionAuthorityValidator;
impl ValidateAuthority for MockConnectionAuthorityValidator {
    fn validate_authority(&self, _authority: &Authority) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
struct SlowChannelCredentials {
    sleep_duration: Duration,
}

impl SlowChannelCredentials {
    fn new_arc(sleep_duration: Duration) -> Arc<Self> {
        Arc::new(Self { sleep_duration })
    }
}

#[async_trait]
impl ChannelCredentials for SlowChannelCredentials {
    async fn connect(
        &self,
        _authority: &Authority,
        source: BoxEndpoint,
        _info: &ClientHandshakeInfo,
        runtime: &GrpcRuntime,
        _token: private::Internal,
    ) -> Result<HandshakeOutput, String> {
        runtime.sleep(self.sleep_duration).await;
        Ok(HandshakeOutput {
            endpoint: source,
            security_info: SecurityInfo::new("mock"),
            authority_validator: Box::new(MockConnectionAuthorityValidator),
        })
    }

    fn info(&self) -> &ProtocolInfo {
        static INFO: ProtocolInfo = ProtocolInfo::new("mock");
        &INFO
    }

    fn get_call_credentials(&self, _: private::Internal) -> Option<&Arc<dyn CallCredentials>> {
        None
    }
}

#[tokio::test(start_paused = true)]
async fn connect_timeout_exceeded() {
    let server = StubServer::new().start().await;
    let addr = server.addr();

    // Create the channel with SlowChannelCredentials (21s).
    // The default timeout is 20s.
    let target = format!("dns:///{}", addr);
    let channel = Channel::builder(
        &target,
        SlowChannelCredentials::new_arc(Duration::from_secs(21)),
    )
    .build();

    // Spawn the RPC call because it will block waiting for connection.
    let rpc_handle = tokio::spawn(async move { perform_unary_echo_failure(&channel).await });

    // Advance time to trigger the timeout in subchannel connect.
    GrpcRuntime::default().sleep(Duration::from_secs(21)).await;

    // The RPC should have failed with a timeout.
    let trailers = rpc_handle.await.unwrap();

    assert!(trailers.status().is_err());
    let status = trailers.status().as_ref().unwrap_err();
    assert_eq!(status.code(), StatusCodeError::Unavailable);

    server.stop().await;
}

#[tokio::test]
async fn trailers_only_metadata() {
    let server = StubServer::new()
        .with_unary_echo(|_| async {
            let mut metadata = TonicMetadata::new();
            metadata.insert(
                "x-custom-trailer",
                TonicMetadataValue::from_static("custom-value"),
            );

            let status = TonicStatus::with_metadata(
                tonic::Code::InvalidArgument,
                "test error message",
                metadata,
            );
            Err(status)
        })
        .start()
        .await;
    let addr = server.addr();

    let target = format!("dns:///{}", addr);
    let channel = Channel::builder(&target, LocalChannelCredentials::new_arc()).build();

    let trailers = perform_unary_echo_failure(&channel).await;

    let status_err = trailers.status().as_ref().unwrap_err();
    assert_eq!(status_err.code(), StatusCodeError::InvalidArgument);
    assert_eq!(status_err.message(), "test error message");

    let metadata_map = trailers.metadata();
    let value = metadata_map.get("x-custom-trailer").unwrap();
    assert_eq!(value, "custom-value");

    let connection_info = trailers
        .connection_info()
        .as_ref()
        .expect("trailers should contain connection_info in trailers-only response");
    assert_eq!(
        connection_info.remote_address().network_type,
        TCP_IP_NETWORK_TYPE
    );
    assert_eq!(
        connection_info.remote_address().address.to_string(),
        addr.to_string()
    );

    server.stop().await;
}

// Verifies that trailer metadata is preserved when the server returns an error
// status in trailers after sending headers and a message (i.e. not a
// Trailers-Only response).
#[tokio::test]
async fn error_trailers_after_headers_metadata() {
    // Echoes the request message once, then ends the stream with `status` in
    // the trailers. Unlike a unary error, this sends response headers first,
    // so the error is delivered in trailers after headers rather than as a
    // Trailers-Only response.
    let server = StubServer::new()
        .with_server_streaming_echo(|request| async {
            let mut metadata = TonicMetadata::new();
            metadata.insert(
                "x-custom-trailer",
                TonicMetadataValue::from_static("custom-value"),
            );
            let status = TonicStatus::with_metadata(
                tonic::Code::InvalidArgument,
                "test error message",
                metadata,
            );
            let response = EchoResponse {
                message: request.into_inner().message,
            };
            let stream = tokio_stream::iter([Ok(response), Err(status)]);
            Ok(Response::new(Box::pin(stream) as EchoStream))
        })
        .start()
        .await;
    let addr = server.addr();

    let target = format!("dns:///{}", addr);
    let channel = Channel::builder(&target, LocalChannelCredentials::new_arc()).build();

    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/ServerStreamingEcho"),
            CallOptions::default(),
        )
        .await;
    let req = WrappedEchoRequest(EchoRequest {
        message: "hello".into(),
    });
    tx.send(
        &req,
        SendOptions {
            final_msg: true,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let item = rx.recv(&mut resp).await;
    let ResponseStreamItem::Headers(_) = item else {
        panic!("Expected Headers first, got {item:?}");
    };
    let item = rx.recv(&mut resp).await;
    let ResponseStreamItem::Message = item else {
        panic!("Expected Message after Headers, got {item:?}");
    };
    assert_eq!(resp.0.message, "hello");
    let item = rx.recv(&mut resp).await;
    let ResponseStreamItem::Trailers(trailers) = item else {
        panic!("Expected Trailers after Message, got {item:?}");
    };

    let status_err = trailers.status().as_ref().unwrap_err();
    assert_eq!(status_err.code(), StatusCodeError::InvalidArgument);
    assert_eq!(status_err.message(), "test error message");

    let value = trailers
        .metadata()
        .get("x-custom-trailer")
        .expect("trailer metadata should be preserved on error status");
    assert_eq!(value, "custom-value");

    assert!(
        trailers.connection_info().is_none(),
        "trailers should not contain connection_info when headers were present"
    );

    server.stop().await;
}

struct WrappedEchoRequest(EchoRequest);
struct WrappedEchoResponse(EchoResponse);

impl SendMessage for WrappedEchoRequest {
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
        Ok(Box::new(Bytes::from(self.0.encode_to_vec())))
    }
}

impl RecvMessage for WrappedEchoResponse {
    fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String> {
        let buf = data.copy_to_bytes(data.remaining());
        self.0 = EchoResponse::decode(buf).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[tokio::test]
async fn tonic_transport_recv_drop_sends_rst_stream() {
    super::reg();
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::handshake(socket).await.unwrap();

        let result = connection.accept().await.unwrap();
        let (request, _respond) = result.unwrap();
        let mut body = request.into_body();

        match body.data().await {
            Some(Ok(_)) => {
                panic!("Expected error or EOF, got data");
            }
            Some(Err(err)) => {
                println!("Got expected error: {:?}", err);
                assert_eq!(err.reason(), Some(Reason::CANCEL));
            }
            None => {
                panic!("Expected RST_STREAM, got clean close (EOS)");
            }
        }
    });

    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let config = Arc::new(TransportOptions::default());
    let securty_opts = SecurityOpts {
        credentials: LocalChannelCredentials::new_arc(),
        authority: Authority::new("localhost".to_string(), None),
        handshake_info: ClientHandshakeInfo::default(),
    };
    let address = Address {
        network_type: TCP_IP_NETWORK_TYPE,
        address: addr.to_string().into(),
        attributes: Attributes::new(),
    };

    let (conn, _sec_info, _disconnection_listener) = builder
        .dyn_connect(
            &address,
            GrpcRuntime::new(TokioRuntime::default()),
            &securty_opts,
            &config,
        )
        .await
        .unwrap();

    let (mut tx, rx) = conn
        .dyn_invoke(
            RequestHeaders::new()
                .with_method_name("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            CallOptions::default(),
        )
        .await;

    // Drop the receiver to trigger cancellation.
    drop(rx);

    // Wait for the server to verify the reset.
    GrpcRuntime::default()
        .timeout(Duration::from_secs(5), server_handle)
        .await
        .expect("Test timed out waiting for server to verify reset")
        .unwrap();

    // Verify the send stream has ended.
    let req = WrappedEchoRequest(EchoRequest::default());
    assert!(tx.send(&req, SendOptions::default()).await.is_err());
}

/// Connects the tonic transport directly to `addr`, bypassing the channel.
/// Returns the connection and its disconnection listener.
async fn connect_transport(
    addr: SocketAddr,
) -> (Box<dyn DynInvoke>, oneshot::Receiver<Result<(), String>>) {
    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let security_opts = SecurityOpts {
        credentials: LocalChannelCredentials::new_arc(),
        authority: Authority::new("localhost".to_string(), None),
        handshake_info: ClientHandshakeInfo::default(),
    };
    let address = Address {
        network_type: TCP_IP_NETWORK_TYPE,
        address: addr.to_string().into(),
        attributes: Attributes::new(),
    };
    let (conn, _sec_info, disconnection_listener) = builder
        .dyn_connect(
            &address,
            GrpcRuntime::new(TokioRuntime::default()),
            &security_opts,
            &TransportOptions::default(),
        )
        .await
        .unwrap();
    (conn, disconnection_listener)
}

/// Call credentials that take `delay` to produce metadata.
#[derive(Debug)]
struct DelayedCallCredentials {
    delay: Duration,
}

#[async_trait]
impl CallCredentials for DelayedCallCredentials {
    async fn get_metadata(
        &self,
        _call_details: &CallDetails,
        _auth_info: &ClientConnectionSecurityInfo,
        _metadata: &mut MetadataMap,
    ) -> Result<(), crate::StatusError> {
        GrpcRuntime::default().sleep(self.delay).await;
        Ok(())
    }

    fn minimum_channel_security_level(&self) -> SecurityLevel {
        SecurityLevel::NoSecurity
    }
}

/// Responds to a raw h2 request with a trailers-only, successful gRPC response.
fn respond_ok(mut respond: h2::server::SendResponse<Bytes>) {
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "application/grpc")
        .header("grpc-status", "0")
        .body(())
        .unwrap();
    respond.send_response(response, true).unwrap();
}

type RawH2Request = (
    http::Request<h2::RecvStream>,
    h2::server::SendResponse<Bytes>,
);

/// A raw h2 server, for tests that need to observe or control the HTTP/2
/// stream directly.
///
/// It accepts a single connection and responds successfully to the first RPC on
/// it, which is expected to be from [`warm_up`].  Later requests are delivered
/// via [`RawH2Server::next_request`].
struct RawH2Server {
    addr: SocketAddr,
    requests: mpsc::UnboundedReceiver<RawH2Request>,
}

impl RawH2Server {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, requests) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let mut connection = h2::server::handshake(socket).await.unwrap();
            let Some(Ok((_, respond))) = connection.accept().await else {
                return;
            };
            respond_ok(respond);
            // Keep accepting, which also drives the connection, until the
            // client closes it.
            while let Some(Ok(request)) = connection.accept().await {
                let _ = tx.send(request);
            }
        });
        Self { addr, requests }
    }

    /// Returns the next request received after the warm-up RPC.
    async fn next_request(&mut self) -> RawH2Request {
        self.requests.recv().await.unwrap()
    }
}

/// Performs an RPC without a deadline to ensure `channel` is connected, so that
/// RPCs later in a test do not race with connection establishment.  (With a
/// paused clock, tokio may advance time to the RPC's deadline while waiting on
/// real I/O.)
///
/// The server must respond to the RPC successfully.
async fn warm_up(channel: &Channel) {
    let (_tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/UnaryEcho"),
            CallOptions::default(),
        )
        .await;
    let mut resp = WrappedEchoResponse(EchoResponse::default());
    loop {
        match rx.recv(&mut resp).await {
            ResponseStreamItem::Trailers(trailers) => {
                let status = trailers.status();
                assert!(status.is_ok(), "warm-up RPC failed: {status:?}");
                return;
            }
            ResponseStreamItem::StreamClosed => panic!("warm-up RPC ended without trailers"),
            ResponseStreamItem::Headers(_) | ResponseStreamItem::Message => {}
        }
    }
}

/// Returns the value of the grpc-timeout header in `request`.
fn grpc_timeout<T>(request: &http::Request<T>) -> &str {
    request
        .headers()
        .get("grpc-timeout")
        .expect("missing grpc-timeout")
        .to_str()
        .unwrap()
}

/// How the server observed the end of the client's request stream.
#[derive(Debug)]
enum RequestStreamEnd {
    /// The client half-closed the stream (END_STREAM).
    HalfClosed,
    /// The client reset the stream (RST_STREAM) with the given reason.
    Reset(Option<Reason>),
}

/// Reads `body` until it ends, and reports how it ended.
async fn read_request_stream_end(mut body: h2::RecvStream) -> RequestStreamEnd {
    loop {
        match body.data().await {
            None => return RequestStreamEnd::HalfClosed,
            Some(Err(e)) => return RequestStreamEnd::Reset(e.reason()),
            Some(Ok(data)) => {
                let _ = body.flow_control().release_capacity(data.len());
            }
        }
    }
}

/// Returns the current time according to the runtime's clock, which follows
/// tokio's paused clock in tests that use one.  RPC deadlines must be computed
/// from this so that they are consistent with the clock used by the channel.
fn now() -> Instant {
    GrpcRuntime::default().now()
}

fn call_options_with_deadline(deadline: Instant) -> CallOptions {
    let mut opts = CallOptions::new();
    opts.set_deadline(deadline);
    opts
}

/// Receives the next item from `rx` and asserts that it is trailers with a
/// DeadlineExceeded status.
async fn recv_deadline_exceeded(rx: &mut impl RecvStream) -> Trailers {
    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let item = rx.recv(&mut resp).await;
    let ResponseStreamItem::Trailers(trailers) = item else {
        panic!("expected trailers due to deadline exceeded, got {item:?}");
    };
    assert_eq!(
        trailers.status().as_ref().unwrap_err().code(),
        StatusCodeError::DeadlineExceeded
    );
    trailers
}

#[tokio::test(start_paused = true)]
async fn deadline_exceeded_waiting_for_connection() {
    let server = StubServer::new().start().await;

    // Connection handshake takes 5s, but RPC deadline is 1s.
    let target = format!("dns:///{}", server.addr());
    let channel = Channel::builder(
        &target,
        SlowChannelCredentials::new_arc(Duration::from_secs(5)),
    )
    .build();

    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/UnaryEcho"),
            call_options_with_deadline(now() + Duration::from_secs(1)),
        )
        .await;

    let req = WrappedEchoRequest(EchoRequest::default());
    assert!(tx.send(&req, SendOptions::default()).await.is_err());
    let trailers = recv_deadline_exceeded(&mut rx).await;
    assert!(trailers.connection_info().is_none());

    server.stop().await;
}

// Verifies that the transport fails a call whose deadline has already expired
// without starting it.  (The channel normally catches this first, but time may
// pass before the call reaches the transport.)
#[tokio::test]
async fn tonic_transport_deadline_already_expired() {
    super::reg();
    let server = StubServer::new().start().await;
    let (conn, _disconnection_listener) = connect_transport(server.addr()).await;

    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/UnaryEcho"),
            call_options_with_deadline(now() - Duration::from_secs(1)),
        )
        .await;

    let req = WrappedEchoRequest(EchoRequest::default());
    assert!(tx.send(&req, SendOptions::default()).await.is_err());
    recv_deadline_exceeded(&mut rx).await;

    server.stop().await;
}

// Verifies that a deadline expiring while awaiting the response cancels the
// stream, and that the grpc-timeout header is computed when the request headers
// are written, so that time spent before then is not included in the timeout
// sent to the server.
#[tokio::test(start_paused = true)]
async fn deadline_exceeded_during_recv() {
    super::reg();
    let mut server = RawH2Server::start().await;

    const CALL_CREDS_DELAY: Duration = Duration::from_millis(300);
    let composite_creds = CompositeChannelCredentials::new(
        LocalChannelCredentials::new(),
        Arc::new(DelayedCallCredentials {
            delay: CALL_CREDS_DELAY,
        }),
    );
    let target = format!("dns:///{}", server.addr);
    let channel = Channel::builder(&target, Arc::new(composite_creds)).build();
    // Connect first, so that only the call credentials delay the RPC below.
    warm_up(&channel).await;

    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new()
                .with_method_name("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            call_options_with_deadline(now() + Duration::from_secs(1)),
        )
        .await;
    let req = WrappedEchoRequest(EchoRequest {
        message: "hello".into(),
    });
    tx.send(&req, SendOptions::default()).await.unwrap();

    // The server never responds.
    let (request, _respond) = server.next_request().await;
    // The clock is paused, so exactly CALL_CREDS_DELAY elapses before the
    // request is sent.
    assert_eq!(grpc_timeout(&request), "700000u");

    recv_deadline_exceeded(&mut rx).await;
    // The stream stays closed after the trailers.
    let item = rx
        .recv(&mut WrappedEchoResponse(EchoResponse::default()))
        .await;
    assert!(
        matches!(item, ResponseStreamItem::StreamClosed),
        "expected StreamClosed after trailers, got {item:?}"
    );

    let end = read_request_stream_end(request.into_body()).await;
    assert!(
        matches!(end, RequestStreamEnd::Reset(Some(Reason::CANCEL))),
        "expected the request stream to be cancelled, got {end:?}"
    );
}

// Verifies that a send blocked on flow control times out on its own (without a
// concurrent recv), and that the stream is then reset rather than half-closed.
//
// Note that hyper does not poll the request body (where tonic observes
// cancellation) while the stream has no flow control window, so the reset is
// not sent until the server reads the request body.
#[tokio::test(start_paused = true)]
async fn deadline_exceeded_during_send_resets_instead_of_half_closing() {
    super::reg();
    let mut server = RawH2Server::start().await;
    let target = format!("dns:///{}", server.addr);
    let channel = Channel::builder(&target, LocalChannelCredentials::new_arc()).build();
    warm_up(&channel).await;

    let (mut tx, mut rx) = channel
        .invoke(
            RequestHeaders::new().with_method_name("/grpc.examples.echo.Echo/ClientStreamingEcho"),
            call_options_with_deadline(now() + Duration::from_secs(1)),
        )
        .await;

    let req = WrappedEchoRequest(EchoRequest {
        message: "x".repeat(1024),
    });
    // The server does not read the request body yet, so flow control
    // eventually blocks the sends until the deadline expires.  Don't call
    // rx.recv() concurrently.
    let mut sent = 0;
    while tx.send(&req, SendOptions::default()).await.is_ok() {
        sent += 1;
    }
    assert!(sent > 0, "no messages were sent");

    // Dropping the send stream after the deadline must not half-close the
    // request stream.
    drop(tx);
    // Start reading the request body, opening the flow control window so that
    // the client can make progress on the stream.
    let (request, _respond) = server.next_request().await;
    let end = tokio::spawn(read_request_stream_end(request.into_body()));
    // Give the client time to (incorrectly) half-close the stream before it is
    // cancelled below.  The clock is paused, so this returns once the client
    // and server are otherwise idle.
    GrpcRuntime::default()
        .sleep(Duration::from_millis(100))
        .await;

    // The deadline is enforced (and the call cancelled) once rx is used.
    recv_deadline_exceeded(&mut rx).await;

    let end = end.await.unwrap();
    assert!(
        matches!(end, RequestStreamEnd::Reset(Some(Reason::CANCEL))),
        "expected the request stream to be cancelled, got {end:?}"
    );
}
