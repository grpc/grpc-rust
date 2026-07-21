//! `tonic` based transport implementation.
//!
//! This transport uses tonic's low-level `Grpc` client with a `BytesCodec`
//! to send and receive raw bytes, allowing the xDS client layer to handle
//! serialization/deserialization independently.

use crate::client::config::ServerConfig;
use crate::error::{Error, Result};
use crate::transport::{Transport, TransportBuilder, TransportStream};
use bytes::{Buf, BufMut, Bytes};
use http::uri::PathAndQuery;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tonic::client::Grpc;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::transport::{Channel, Endpoint};
use tonic::{Status, Streaming};

/// Per-stream call credentials for the ADS stream (e.g. a bearer token).
///
/// Attached on each (re)connect, only when the channel is secure.
#[tonic::async_trait]
pub trait TonicCallCredentials: Send + Sync + std::fmt::Debug + 'static {
    /// Generates the authentication metadata for a specific call.
    async fn get_request_metadata(
        &self,
        metadata: &mut tonic::metadata::MetadataMap,
    ) -> std::result::Result<(), Status>;

    /// Whether these credentials require a secure (TLS) transport.
    fn requires_secure_transport(&self) -> bool {
        // Note: a bool simplification of the `grpc` crate's
        // `CallCredentials::minimum_channel_security_level` (`SecurityLevel`).
        true
    }
}

/// The gRPC path for the ADS StreamAggregatedResources RPC.
const ADS_PATH: &str =
    "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources";

const ADS_CHANNEL_BUFFER_SIZE: usize = 16;

/// A codec that passes bytes through without serialization.
///
/// This allows us to handle serialization in the xDS client layer
/// rather than in the transport layer.
#[derive(Debug, Clone, Copy)]
struct BytesCodec;

impl Codec for BytesCodec {
    type Encode = Bytes;
    type Decode = Bytes;
    type Encoder = BytesEncoder;
    type Decoder = BytesDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        BytesEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        BytesDecoder
    }
}

#[derive(Debug)]
struct BytesEncoder;

impl Encoder for BytesEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut EncodeBuf<'_>,
    ) -> std::result::Result<(), Self::Error> {
        dst.put_slice(&item);
        Ok(())
    }
}

#[derive(Debug)]
struct BytesDecoder;

impl Decoder for BytesDecoder {
    type Item = Bytes;
    type Error = Status;

    fn decode(
        &mut self,
        src: &mut DecodeBuf<'_>,
    ) -> std::result::Result<Option<Self::Item>, Self::Error> {
        Ok(Some(src.copy_to_bytes(src.remaining())))
    }
}

/// Factory for creating ADS streams using tonic.
#[derive(Clone, Debug)]
pub struct TonicTransport {
    channel: Channel,
    call_creds: Option<Arc<dyn TonicCallCredentials>>,
}

impl TonicTransport {
    /// Create a transport from an existing tonic [`Channel`].
    ///
    /// Use this when you need custom channel configuration (e.g., TLS, timeouts).
    /// Call credentials are not supported here, since the channel's security
    /// cannot be verified; use [`TonicTransportBuilder`] for them.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tonic::transport::{Certificate, Channel, ClientTlsConfig};
    ///
    /// let tls = ClientTlsConfig::new()
    ///     .ca_certificate(Certificate::from_pem(ca_cert))
    ///     .domain_name("xds.example.com");
    ///
    /// let channel = Channel::from_static("https://xds.example.com:443")
    ///     .tls_config(tls)?
    ///     .connect()
    ///     .await?;
    ///
    /// let transport = TonicTransport::from_channel(channel);
    /// ```
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            channel,
            call_creds: None,
        }
    }

    /// Connect to an xDS server with default settings.
    ///
    /// For custom configuration (TLS, call credentials), use [`TonicTransportBuilder`];
    /// for a pre-built channel, use [`from_channel`](Self::from_channel).
    pub async fn connect(uri: impl Into<String>) -> Result<Self> {
        let server = ServerConfig::new(uri.into());
        TonicTransportBuilder::new().build(&server).await
    }
}

/// Builder for creating [`TonicTransport`] instances.
///
/// This implements [`TransportBuilder`] and can be used with
/// [`XdsClientBuilder`](crate::XdsClientBuilder) to enable server fallback support.
///
/// # Example
///
/// ```ignore
/// use xds_client::{ClientConfig, Node, TonicTransportBuilder, XdsClient};
///
/// let transport_builder = TonicTransportBuilder::new();
/// let config = ClientConfig::new(node, "http://xds.example.com:18000");
/// let client = XdsClient::builder(config, transport_builder, codec, runtime).build();
/// ```
///
/// # TLS
///
/// Enable the `tls-ring` or `tls-aws-lc` feature and call `with_tls_config`:
///
/// ```ignore
/// use tonic::transport::ClientTlsConfig;
/// use xds_client::TonicTransportBuilder;
///
/// let builder = TonicTransportBuilder::new()
///     .with_tls_config(ClientTlsConfig::new().with_enabled_roots());
/// ```
#[derive(Debug, Clone, Default)]
pub struct TonicTransportBuilder {
    // Future extensions:
    // - Connection timeout settings
    // - Keep-alive configuration
    // - Connection pooling settings
    // - Per-server credential overrides (via ServerConfig.extensions)
    #[cfg(any(feature = "tonic-tls-ring", feature = "tonic-tls-aws-lc"))]
    tls_config: Option<tonic::transport::ClientTlsConfig>,

    /// Per-stream call credentials for the ADS stream.
    call_creds: Option<Arc<dyn TonicCallCredentials>>,
}

impl TonicTransportBuilder {
    /// Create a new transport builder with default (plaintext) settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the TLS configuration for connections to the xDS server.
    ///
    /// When set, all connections created by this builder will use TLS
    /// with the provided configuration.
    #[cfg(any(feature = "tonic-tls-ring", feature = "tonic-tls-aws-lc"))]
    pub fn with_tls_config(mut self, tls_config: tonic::transport::ClientTlsConfig) -> Self {
        self.tls_config = Some(tls_config);
        self
    }

    /// Set per-stream call credentials for the ADS stream (e.g. `google_default`).
    ///
    /// Attached on each (re)connect, only over a secure channel; over an insecure
    /// channel, [`build`](TransportBuilder::build) fails. Not refreshed mid-stream.
    pub fn with_call_credentials(mut self, creds: Arc<dyn TonicCallCredentials>) -> Self {
        self.call_creds = Some(creds);
        self
    }

    /// Prepend `https://` to a scheme-less `server_uri` on the secure path.
    ///
    /// Bootstrap URIs like `trafficdirector.googleapis.com:443` parse with no scheme,
    /// so `Endpoint` won't negotiate TLS. A scheme lets it, and tonic derive SNI from
    /// `uri.host()`. Non-`http::Uri` inputs (`unix://`) and plaintext are left as-is.
    fn ensure_secure_server_uri(raw: &str, secure: bool) -> String {
        if secure
            && let Ok(uri) = raw.parse::<http::Uri>()
            && uri.scheme().is_none()
        {
            return format!("https://{raw}");
        }
        raw.to_string()
    }
}

impl TransportBuilder for TonicTransportBuilder {
    type Transport = TonicTransport;

    async fn build(&self, server: &ServerConfig) -> Result<Self::Transport> {
        // The channel is secure only when TLS is configured; with no TLS backend
        // compiled in, it can never be secure.
        #[cfg(any(feature = "tonic-tls-ring", feature = "tonic-tls-aws-lc"))]
        let secure = self.tls_config.is_some();
        #[cfg(not(any(feature = "tonic-tls-ring", feature = "tonic-tls-aws-lc")))]
        let secure = false;

        // Refuse before connecting: never send credentials over an insecure channel.
        if let Some(creds) = &self.call_creds
            && creds.requires_secure_transport()
            && !secure
        {
            return Err(Error::CallCredentials(
                "call credentials require a secure transport".into(),
            ));
        }

        // `Endpoint::from_shared` routes `unix://` URIs to tonic's UDS connector.
        // Required for control planes like Istio's grpc-agent that ship `unix:///etc/istio/proxy/XDS`.
        let endpoint = Endpoint::from_shared(Self::ensure_secure_server_uri(server.uri(), secure))
            .map_err(|e| Error::Connection(e.to_string()))?;

        #[cfg(any(feature = "tonic-tls-ring", feature = "tonic-tls-aws-lc"))]
        let endpoint = match &self.tls_config {
            Some(tls) => endpoint
                .tls_config(tls.clone())
                .map_err(|e| Error::Connection(e.to_string()))?,
            None => endpoint,
        };

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        Ok(TonicTransport {
            channel,
            call_creds: self.call_creds.clone(),
        })
    }
}

impl Transport for TonicTransport {
    type Stream = TonicAdsStream;

    async fn new_stream(&self, initial_requests: Vec<Bytes>) -> Result<Self::Stream> {
        let mut grpc = Grpc::new(self.channel.clone());

        grpc.ready()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let (tx, rx) = mpsc::channel::<Bytes>(ADS_CHANNEL_BUFFER_SIZE);

        // Create a stream that first yields initial requests, then reads from the channel.
        // This ensures data is available immediately when the stream is polled,
        // preventing deadlock with servers that don't send response headers
        // until they receive the first request message.
        let initial_stream = tokio_stream::iter(initial_requests);
        let channel_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        let request_stream = initial_stream.chain(channel_stream);

        let path = PathAndQuery::from_static(ADS_PATH);
        let mut request = tonic::Request::new(request_stream);

        // Inject the configured call credentials.
        if let Some(creds) = &self.call_creds {
            creds
                .get_request_metadata(request.metadata_mut())
                .await
                .map_err(|e| Error::CallCredentials(e.to_string()))?;
        }

        let response = grpc
            .streaming(request, path, BytesCodec)
            .await
            .map_err(Error::Stream)?;

        Ok(TonicAdsStream {
            sender: tx,
            receiver: response.into_inner(),
        })
    }
}

/// A bidirectional ADS stream backed by tonic.
#[derive(Debug)]
pub struct TonicAdsStream {
    sender: mpsc::Sender<Bytes>,
    receiver: Streaming<Bytes>,
}

impl TransportStream for TonicAdsStream {
    async fn send(&mut self, request: Bytes) -> Result<()> {
        self.sender
            .send(request)
            .await
            .map_err(|_| Error::StreamClosed)?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<Option<Bytes>> {
        match self.receiver.message().await {
            Ok(msg) => Ok(msg),
            Err(status) => Err(Error::Stream(status)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use envoy_types::pb::envoy::service::discovery::v3::{
        DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
        aggregated_discovery_service_server::{
            AggregatedDiscoveryService, AggregatedDiscoveryServiceServer,
        },
    };
    use prost::Message;
    use std::net::SocketAddr;
    use std::pin::Pin;
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio_stream::Stream;
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::{Request, Response, Status};

    /// Mock ADS server that echoes back a response for each request.
    #[derive(Default)]
    struct MockAdsServer {
        expected_auth: Option<String>,
    }

    #[tonic::async_trait]
    impl AggregatedDiscoveryService for MockAdsServer {
        type StreamAggregatedResourcesStream =
            Pin<Box<dyn Stream<Item = std::result::Result<DiscoveryResponse, Status>> + Send>>;

        async fn stream_aggregated_resources(
            &self,
            request: Request<tonic::Streaming<DiscoveryRequest>>,
        ) -> std::result::Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
            if let Some(expected) = &self.expected_auth {
                let got = request
                    .metadata()
                    .get("authorization")
                    .and_then(|v| v.to_str().ok());
                if got != Some(expected.as_str()) {
                    return Err(Status::unauthenticated(
                        "missing or unexpected authorization",
                    ));
                }
            }
            let mut inbound = request.into_inner();

            let outbound = async_stream::try_stream! {
                while let Some(req) = inbound.next().await {
                    let req = req?;
                    let response = DiscoveryResponse {
                        version_info: "1".to_string(),
                        type_url: req.type_url.clone(),
                        nonce: "nonce-1".to_string(),
                        resources: vec![],
                        ..Default::default()
                    };
                    yield response;
                }
            };

            Ok(Response::new(Box::pin(outbound)))
        }

        type DeltaAggregatedResourcesStream =
            Pin<Box<dyn Stream<Item = std::result::Result<DeltaDiscoveryResponse, Status>> + Send>>;

        async fn delta_aggregated_resources(
            &self,
            _request: Request<tonic::Streaming<DeltaDiscoveryRequest>>,
        ) -> std::result::Result<Response<Self::DeltaAggregatedResourcesStream>, Status> {
            Err(Status::unimplemented("delta not supported in mock"))
        }
    }

    async fn start_mock_server(expected_auth: Option<&str>) -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = MockAdsServer {
            expected_auth: expected_auth.map(str::to_owned),
        };

        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(AggregatedDiscoveryServiceServer::new(server))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        addr
    }

    #[derive(Debug)]
    struct MockCreds {
        pairs: Vec<(String, String)>,
        requires_secure: bool,
    }

    #[tonic::async_trait]
    impl TonicCallCredentials for MockCreds {
        async fn get_request_metadata(
            &self,
            metadata: &mut tonic::metadata::MetadataMap,
        ) -> std::result::Result<(), tonic::Status> {
            for (name, value) in &self.pairs {
                let key = tonic::metadata::AsciiMetadataKey::from_bytes(name.as_bytes())
                    .map_err(|e| Status::invalid_argument(e.to_string()))?;
                let val = tonic::metadata::AsciiMetadataValue::try_from(value)
                    .map_err(|e| Status::invalid_argument(e.to_string()))?;
                metadata.insert(key, val);
            }
            Ok(())
        }
        fn requires_secure_transport(&self) -> bool {
            self.requires_secure
        }
    }

    #[tokio::test]
    async fn call_creds_attach_metadata() {
        let addr = start_mock_server(Some("Bearer test-token")).await;
        let creds = Arc::new(MockCreds {
            pairs: vec![("authorization".into(), "Bearer test-token".into())],
            requires_secure: false,
        });
        let transport = TonicTransportBuilder::new()
            .with_call_credentials(creds)
            .build(&ServerConfig::new(format!("http://{addr}")))
            .await
            .unwrap();
        let request = DiscoveryRequest {
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
            ..Default::default()
        };
        let request_bytes: Bytes = request.encode_to_vec().into();
        let mut stream = transport.new_stream(vec![request_bytes]).await.unwrap();
        let response = stream.recv().await.unwrap().unwrap();
        let response = DiscoveryResponse::decode(response).unwrap();
        assert_eq!(response.version_info, "1");
    }

    #[tokio::test]
    async fn from_channel_connects_and_streams() {
        let addr = start_mock_server(None).await;
        let channel = Endpoint::from_shared(format!("http://{addr}"))
            .unwrap()
            .connect_lazy();
        let transport = TonicTransport::from_channel(channel);
        let request = DiscoveryRequest {
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
            ..Default::default()
        };
        let request_bytes: Bytes = request.encode_to_vec().into();
        let mut stream = transport.new_stream(vec![request_bytes]).await.unwrap();
        let response = stream.recv().await.unwrap().unwrap();
        let response = DiscoveryResponse::decode(response).unwrap();
        assert_eq!(response.version_info, "1");
    }

    #[tokio::test]
    async fn call_creds_require_secure_transport() {
        // The check runs before connecting, so no server is needed.
        let err = TonicTransportBuilder::new()
            .with_call_credentials(Arc::new(MockCreds {
                pairs: vec![],
                requires_secure: true,
            }))
            .build(&ServerConfig::new("http://127.0.0.1:1"))
            .await
            .unwrap_err();
        assert!(matches!(err, Error::CallCredentials(_)));
    }

    #[test]
    fn ensure_secure_server_uri_adds_scheme_only_when_needed() {
        assert_eq!(
            TonicTransportBuilder::ensure_secure_server_uri(
                "trafficdirector.googleapis.com:443",
                true
            ),
            "https://trafficdirector.googleapis.com:443",
        );
        assert_eq!(
            TonicTransportBuilder::ensure_secure_server_uri("https://xds.example.com:443", true),
            "https://xds.example.com:443"
        );
        assert_eq!(
            TonicTransportBuilder::ensure_secure_server_uri("unix:///etc/istio/proxy/XDS", true),
            "unix:///etc/istio/proxy/XDS"
        );
        assert_eq!(
            TonicTransportBuilder::ensure_secure_server_uri("127.0.0.1:18000", false),
            "127.0.0.1:18000"
        );
    }

    #[tokio::test]
    async fn test_tonic_transport_connect_and_stream() {
        let addr = start_mock_server(None).await;
        let uri = format!("http://{addr}");

        let transport = TonicTransport::connect(&uri).await.unwrap();

        let request = DiscoveryRequest {
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
            resource_names: vec!["listener-1".to_string()],
            ..Default::default()
        };
        let request_bytes: Bytes = request.encode_to_vec().into();

        let mut stream = transport.new_stream(vec![request_bytes]).await.unwrap();

        let response_bytes = stream.recv().await.unwrap().unwrap();
        let response = DiscoveryResponse::decode(response_bytes).unwrap();

        assert_eq!(response.version_info, "1");
        assert_eq!(
            response.type_url,
            "type.googleapis.com/envoy.config.listener.v3.Listener"
        );
        assert_eq!(response.nonce, "nonce-1");
    }
}
