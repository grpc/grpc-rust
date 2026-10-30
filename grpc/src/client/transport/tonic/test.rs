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
use std::pin::Pin;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;

use bytes::Buf;
use bytes::Bytes;
use h2::Reason;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
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
use crate::client::RecvStream as _;
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

struct FailingEncodeRequest;

impl SendMessage for FailingEncodeRequest {
    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
        Err("intentional encode failure".to_string())
    }
}

// Starts a raw h2 server that accepts one stream and asserts the client resets
// it with CANCEL.  The server never sends a status, so the client can only
// learn it from its own encode failure.
async fn spawn_cancel_expecting_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::handshake(socket).await.unwrap();
        let (request, _respond) = connection.accept().await.unwrap().unwrap();

        // Drive the connection so the RST_STREAM frame is processed.
        tokio::spawn(async move { while connection.accept().await.is_some() {} });

        match request.into_body().data().await {
            Some(Ok(_)) => panic!("Expected RST_STREAM, got data"),
            Some(Err(err)) => assert_eq!(err.reason(), Some(Reason::CANCEL)),
            None => panic!("Expected RST_STREAM, got clean close (EOS)"),
        }
    });
    (addr, handle)
}

// Starts a raw h2 server that accepts one stream, optionally responds with
// headers, and then holds the stream open without sending a status.
async fn spawn_stream_holding_server(send_headers: bool) -> SocketAddr {
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::handshake(socket).await.unwrap();
        let (_request, mut respond) = connection.accept().await.unwrap().unwrap();
        let _response_stream = send_headers.then(|| {
            let response = http::Response::builder()
                .status(200)
                .header("content-type", "application/grpc")
                .body(())
                .unwrap();
            respond.send_response(response, false).unwrap()
        });
        while connection.accept().await.is_some() {}
    });
    addr
}

// Connects to addr, returning the connection and its disconnection listener.
async fn connect_tonic_transport(
    addr: SocketAddr,
) -> (Box<dyn DynInvoke>, oneshot::Receiver<Result<(), String>>) {
    let builder = GLOBAL_TRANSPORT_REGISTRY
        .get_transport(TCP_IP_NETWORK_TYPE)
        .unwrap();
    let config = Arc::new(TransportOptions::default());
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
            &config,
        )
        .await
        .unwrap();
    (conn, disconnection_listener)
}

fn assert_encode_error_trailers(item: ResponseStreamItem) {
    let ResponseStreamItem::Trailers(trailers) = item else {
        panic!("Expected Trailers after encode failure, got {item:?}");
    };
    let status_err = trailers.status().as_ref().unwrap_err();
    assert_eq!(status_err.code(), StatusCodeError::Internal);
    assert!(
        status_err.message().contains("intentional encode failure"),
        "unexpected status message: {}",
        status_err.message()
    );
}

const ENCODE_ERROR_METHOD: &str = "/grpc.examples.echo.Echo/BidirectionalStreamingEcho";

#[tokio::test]
async fn tonic_transport_send_encode_error_cancels_stream() {
    super::reg();
    let (addr, server_handle) = spawn_cancel_expecting_server().await;
    let (conn, _disconnection_listener) = connect_tonic_transport(addr).await;
    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name(ENCODE_ERROR_METHOD),
            CallOptions::default(),
        )
        .await;

    assert!(
        tx.send(&FailingEncodeRequest, SendOptions::default())
            .await
            .is_err()
    );

    // The send stream has ended, so a message that encodes fine is rejected.
    let req = WrappedEchoRequest(EchoRequest::default());
    assert!(tx.send(&req, SendOptions::default()).await.is_err());

    // The send failure alone cancels the stream, before the application reads
    // the status.
    GrpcRuntime::default()
        .timeout(Duration::from_secs(5), server_handle)
        .await
        .expect("Test timed out waiting for server to observe RST_STREAM")
        .unwrap();

    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let item = GrpcRuntime::default()
        .timeout(Duration::from_secs(5), rx.recv(&mut resp))
        .await
        .expect("Test timed out waiting for trailers after encode failure");
    assert_encode_error_trailers(item);
    assert!(matches!(
        rx.recv(&mut resp).await,
        ResponseStreamItem::StreamClosed
    ));
}

#[tokio::test]
async fn tonic_transport_send_encode_error_fails_recv_awaiting_headers() {
    super::reg();
    let addr = spawn_stream_holding_server(false).await;
    let (conn, _disconnection_listener) = connect_tonic_transport(addr).await;
    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name(ENCODE_ERROR_METHOD),
            CallOptions::default(),
        )
        .await;

    // join! polls recv first, so it is already waiting for the response
    // headers when send fails.
    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let runtime = GrpcRuntime::default();
    let (item, send_result) = tokio::join!(
        runtime.timeout(Duration::from_secs(5), rx.recv(&mut resp)),
        tx.send(&FailingEncodeRequest, SendOptions::default()),
    );
    assert!(send_result.is_err());
    assert_encode_error_trailers(
        item.expect("Test timed out waiting for trailers after encode failure"),
    );
}

#[tokio::test]
async fn tonic_transport_send_encode_error_fails_recv_awaiting_message() {
    super::reg();
    let addr = spawn_stream_holding_server(true).await;
    let (conn, _disconnection_listener) = connect_tonic_transport(addr).await;
    let (mut tx, mut rx) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name(ENCODE_ERROR_METHOD),
            CallOptions::default(),
        )
        .await;

    let mut resp = WrappedEchoResponse(EchoResponse::default());
    let runtime = GrpcRuntime::default();
    let item = runtime
        .timeout(Duration::from_secs(5), rx.recv(&mut resp))
        .await
        .expect("Test timed out waiting for response headers");
    assert!(matches!(item, ResponseStreamItem::Headers(_)));

    // join! polls recv first, so it is already waiting for a message when send
    // fails.
    let (item, send_result) = tokio::join!(
        runtime.timeout(Duration::from_secs(5), rx.recv(&mut resp)),
        tx.send(&FailingEncodeRequest, SendOptions::default()),
    );
    assert!(send_result.is_err());
    assert_encode_error_trailers(
        item.expect("Test timed out waiting for trailers after encode failure"),
    );
}

// While the stream cannot be opened, tonic cannot act on the cancellation, so
// a pending recv must not depend on it.
#[tokio::test]
async fn tonic_transport_send_encode_error_fails_recv_while_waiting_for_stream_slot() {
    super::reg();
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::Builder::new()
            .max_concurrent_streams(1)
            .handshake::<_, Bytes>(socket)
            .await
            .unwrap();
        // Hold every stream open, and respond with headers so the client can
        // tell when its first stream is established.
        let mut streams = Vec::new();
        while let Some(Ok((request, mut respond))) = connection.accept().await {
            let response = http::Response::builder()
                .status(200)
                .header("content-type", "application/grpc")
                .body(())
                .unwrap();
            let response_stream = respond.send_response(response, false).unwrap();
            streams.push((request, response_stream));
        }
    });
    let (conn, _disconnection_listener) = connect_tonic_transport(addr).await;
    let runtime = GrpcRuntime::default();
    let mut resp = WrappedEchoResponse(EchoResponse::default());

    // The first call occupies the server's only stream slot.
    let (_tx1, mut rx1) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name(ENCODE_ERROR_METHOD),
            CallOptions::default(),
        )
        .await;
    let item = runtime
        .timeout(Duration::from_secs(5), rx1.recv(&mut resp))
        .await
        .expect("Test timed out waiting for response headers");
    assert!(matches!(item, ResponseStreamItem::Headers(_)));

    // The second call's stream cannot be opened until the first one ends.
    let (mut tx2, mut rx2) = conn
        .dyn_invoke(
            RequestHeaders::new().with_method_name(ENCODE_ERROR_METHOD),
            CallOptions::default(),
        )
        .await;
    // join! polls recv first, so it is already waiting when send fails.
    let (item, send_result) = tokio::join!(
        runtime.timeout(Duration::from_secs(5), rx2.recv(&mut resp)),
        tx2.send(&FailingEncodeRequest, SendOptions::default()),
    );
    assert!(send_result.is_err());
    assert_encode_error_trailers(
        item.expect("Test timed out waiting for trailers after encode failure"),
    );
}

// Builds a recv stream whose tonic call has already finished with `result`,
// after the call's send stream failed.  The status must already be received
// when the send fails, which cannot be arranged deterministically over a real
// connection, so the stream is built directly.
fn recv_stream_after_send_failure(
    result: Result<tonic::Response<tonic::Streaming<Bytes>>, TonicStatus>,
) -> super::TonicRecvStream {
    let (resp_tx, resp_rx) = oneshot::channel();
    resp_tx.send(result).unwrap();
    let call = Arc::new(super::CallState {
        cancel_tx: tonic::Request::new(()).cancellation_handle(),
        send_failure: Arc::default(),
    });
    call.fail_send(StatusError::new(
        StatusCodeError::Internal,
        "error encoding request: intentional encode failure",
    ));
    super::TonicRecvStream {
        state: super::StreamState::AwaitingHeaders(resp_rx),
        call: Some(call),
        connection_info: Some(crate::core::test_connection_info()),
    }
}

// As in gRPC-Go, a status the client already received from the server is
// reported even if the send stream fails before the application reads it.
#[tokio::test]
async fn tonic_recv_stream_reports_received_status_after_send_failure() {
    for code in [tonic::Code::NotFound, tonic::Code::Cancelled] {
        let mut rx = recv_stream_after_send_failure(Err(TonicStatus::new(code, "server status")));
        let mut resp = WrappedEchoResponse(EchoResponse::default());
        let ResponseStreamItem::Trailers(trailers) = rx.recv(&mut resp).await else {
            panic!("Expected Trailers");
        };
        let status_err = trailers.status().as_ref().unwrap_err();
        assert_eq!(status_err.code(), StatusCodeError::from(code as i32));
        assert_eq!(status_err.message(), "server status");
    }
}

// Records, each time the request body is woken, whether the send failure was
// already recorded.
struct SendFailureProbe {
    send_failure: Arc<super::SendFailure>,
    recorded_at_wake: std::sync::Mutex<Vec<bool>>,
}

impl std::task::Wake for SendFailureProbe {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let recorded = self.send_failure.get().is_some();
        self.recorded_at_wake.lock().unwrap().push(recorded);
    }
}

// A request body that ends normally lets the server treat a cut-short request
// as complete, so after a send failure the body must only end by cancellation.
// The race this guards against needs a concurrent poll of the body, so the
// test observes the body directly: the request channel wakes it when it
// closes, and the failure must be recorded by then.
#[tokio::test]
async fn tonic_send_encode_error_never_ends_request_body() {
    let new_streams = || {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let send_failure = Arc::new(super::SendFailure::default());
        let body = super::RequestStream {
            rx: ReceiverStream::new(rx),
            send_failure: send_failure.clone(),
        };
        let call = Arc::new(super::CallState {
            cancel_tx: tonic::Request::new(()).cancellation_handle(),
            send_failure: send_failure.clone(),
        });
        let send = super::TonicSendStream {
            state: super::SendState::Open { tx, call },
        };
        (send, body, send_failure)
    };

    // Without a send failure, the final message ends the body.
    let (mut send, mut body, _) = new_streams();
    let req = WrappedEchoRequest(EchoRequest::default());
    send.send(&req, SendOptions::default().with_final_msg(true))
        .await
        .unwrap();
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(matches!(
        Pin::new(&mut body).poll_next(&mut cx),
        std::task::Poll::Ready(Some(_))
    ));
    assert!(matches!(
        Pin::new(&mut body).poll_next(&mut cx),
        std::task::Poll::Ready(None)
    ));

    let (mut send, mut body, send_failure) = new_streams();
    let probe = Arc::new(SendFailureProbe {
        send_failure,
        recorded_at_wake: Default::default(),
    });
    let waker = std::task::Waker::from(probe.clone());
    let mut cx = std::task::Context::from_waker(&waker);
    assert!(Pin::new(&mut body).poll_next(&mut cx).is_pending());
    assert!(
        send.send(&FailingEncodeRequest, SendOptions::default())
            .await
            .is_err()
    );
    assert_eq!(*probe.recorded_at_wake.lock().unwrap(), vec![true]);
    assert!(Pin::new(&mut body).poll_next(&mut cx).is_pending());
}
