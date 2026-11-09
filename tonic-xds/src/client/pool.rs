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

//! xDS clients shared across channels, keyed as in gRFC A71.

use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::Duration;

use tonic::transport::Channel;
#[cfg(feature = "_tls-any")]
use xds_client::Error as XdsClientError;
use xds_client::{
    ClientConfig, MetricsRecorder, Node, ProstCodec, ServerConfig, TokioRuntime, TonicTransport,
    TonicTransportBuilder, TransportBuilder, XdsClient,
};

use crate::client::channel::BuildError;
use crate::csds::CsdsRegistry;
use crate::xds::bootstrap::BootstrapConfig;
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider::{
    CertProviderError, CertificateProvider, file_watcher::FileWatcherProvider,
};
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider_config::FileWatcherConfig;
use crate::{BoxFuture, ControlPlaneConfig, ControlPlaneConnector, TonicCallCredentials};

/// Shares xDS clients, and their ADS streams, between channels.
///
/// Every channel takes its xDS client from a pool, by key. Channels given the
/// same pool with [`XdsChannelBuilder::with_xds_client_pool`] share one client
/// per key, so they hold a single ADS stream and resources they have in common
/// are fetched once. The key defaults to the channel's target URI, matching
/// [gRFC A71]; give every channel the same key with
/// [`XdsChannelBuilder::with_xds_client_key`] to share one client across the
/// process. A channel built without a pool gets a pool of its own, configured
/// by its [`XdsChannelConfig`](crate::XdsChannelConfig), so its client is not
/// shared.
///
/// A client lives while any channel uses it. Once the last one is dropped, the
/// next channel with that key starts a new client; the old ADS stream may take
/// a moment to close.
///
/// A client runs on the Tokio runtime of the channel that starts it. If that
/// runtime shuts down, the client stops for every channel sharing it, so share
/// a pool only among channels on one long-lived runtime.
///
/// The pool's bootstrap and ADS settings apply to every client it starts.
/// Cloning the pool is cheap and shares its clients.
///
/// [`CsdsService`](crate::CsdsService) reports each client the pool starts,
/// with its key as the `client_scope`.
///
/// [`XdsChannelBuilder::with_xds_client_pool`]: crate::XdsChannelBuilder::with_xds_client_pool
/// [`XdsChannelBuilder::with_xds_client_key`]: crate::XdsChannelBuilder::with_xds_client_key
/// [gRFC A71]: https://github.com/grpc/proposal/blob/master/A71-xds-fallback.md
#[derive(Clone)]
pub struct XdsClientPool {
    inner: Arc<Inner>,
}

struct Inner {
    bootstrap: BootstrapConfig,
    ads: AdsClientSettings,
    recorder: Option<Arc<dyn MetricsRecorder>>,
    csds_registry: Arc<CsdsRegistry>,
    clients: Mutex<HashMap<String, Weak<XdsClient>>>,
}

impl Inner {
    /// Starts an xDS client for the bootstrap's server and reports it through
    /// CSDS while it runs. `key` becomes the gRFC A78 `grpc.target` metric
    /// label and the gRFC A71 CSDS `client_scope`.
    fn start_client(&self, key: &str) -> Result<Arc<XdsClient>, BuildError> {
        let bootstrap = &self.bootstrap;
        let ads = &self.ads;
        let connector: Arc<dyn ControlPlaneConnector> = match &ads.connector {
            Some(connector) => connector.clone(),
            None => Arc::new(GrpcControlPlaneConnector::new(bootstrap)?),
        };
        let mut options = TonicTransportBuilder::new();
        if let Some(creds) = ads.call_creds.clone() {
            options = options.with_call_credentials(creds);
        }
        if let Some(limit) = ads.max_decoding_message_size {
            options = options.with_max_decoding_message_size(limit);
        }
        if let Some(limit) = ads.max_encoding_message_size {
            options = options.with_max_encoding_message_size(limit);
        }
        let transport_builder = ControlPlaneTransportBuilder {
            connector,
            options,
            keep_alive: ads.keep_alive,
            require_secure: bootstrap.use_tls(),
        };
        let node = Node::try_from(bootstrap.node.clone())?;
        let client_config =
            ClientConfig::new(node.clone(), bootstrap.server_uri()).with_target(key);
        let mut builder =
            XdsClient::builder(client_config, transport_builder, ProstCodec, TokioRuntime);
        if let Some(recorder) = self.recorder.clone() {
            builder = builder.with_metrics_recorder(recorder);
        }
        let client = Arc::new(builder.build());
        self.csds_registry.register(key.to_owned(), &node, &client);
        Ok(client)
    }
}

/// ADS stream settings shared by every client a pool starts.
#[derive(Clone, Default)]
struct AdsClientSettings {
    connector: Option<Arc<dyn ControlPlaneConnector>>,
    call_creds: Option<Arc<dyn TonicCallCredentials>>,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
    keep_alive: Option<(Option<Duration>, Duration)>,
}

impl Debug for AdsClientSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdsClientSettings")
            .field("custom_control_plane_connector", &self.connector.is_some())
            .field("call_creds", &self.call_creds)
            .field("max_decoding_message_size", &self.max_decoding_message_size)
            .field("max_encoding_message_size", &self.max_encoding_message_size)
            .field("keep_alive", &self.keep_alive)
            .finish()
    }
}

struct ControlPlaneTransportBuilder {
    connector: Arc<dyn ControlPlaneConnector>,
    options: TonicTransportBuilder,
    keep_alive: Option<(Option<Duration>, Duration)>,
    require_secure: bool,
}

impl TransportBuilder for ControlPlaneTransportBuilder {
    type Transport = TonicTransport;

    async fn build(&self, server: &ServerConfig) -> xds_client::Result<Self::Transport> {
        let config = ControlPlaneConfig::new(server, self.keep_alive);
        let secure = self.connector.is_secure();
        let https = server
            .uri()
            .parse::<http::Uri>()
            .is_ok_and(|uri| uri.scheme_str() == Some("https"));
        if (self.require_secure || https) && !secure {
            return Err(xds_client::Error::Connection(format!(
                "server '{}' requires a secure transport, but the control-plane \
                 connector declares an insecure transport",
                server.uri(),
            )));
        }
        self.options
            .build_with_channel(async { self.connector.connect(config).await }, secure)
            .await
    }
}

/// The default gRPC control-plane connector.
#[derive(Debug)]
pub(crate) struct GrpcControlPlaneConnector {
    transport_builder: TonicTransportBuilder,
    secure: bool,
}

impl GrpcControlPlaneConnector {
    pub(crate) fn new(bootstrap: &BootstrapConfig) -> Result<Self, BuildError> {
        let (transport_builder, secure) = default_control_plane_transport(bootstrap)?;
        Ok(Self {
            transport_builder,
            secure,
        })
    }
}

impl ControlPlaneConnector for GrpcControlPlaneConnector {
    fn is_secure(&self) -> bool {
        self.secure
    }

    fn connect(&self, config: ControlPlaneConfig<'_>) -> BoxFuture<xds_client::Result<Channel>> {
        let mut transport_builder = self.transport_builder.clone();
        if let Some((interval, timeout)) = config.keep_alive() {
            transport_builder = transport_builder.with_keep_alive(interval, timeout);
        }
        let server = config.server().clone();
        Box::pin(async move { transport_builder.connect_channel(&server).await })
    }
}

fn default_control_plane_transport(
    bootstrap: &BootstrapConfig,
) -> Result<(TonicTransportBuilder, bool), BuildError> {
    #[cfg(feature = "_tls-any")]
    if let Some(tls_config) = bootstrap.tls_config()? {
        let provider = AdsTlsConfigProvider::new(tls_config).map_err(BuildError::AdsTls)?;
        let builder = TonicTransportBuilder::new()
            .with_tls_config_factory(move |_| provider.client_tls_config());
        return Ok((builder, true));
    }
    #[cfg(not(feature = "_tls-any"))]
    if bootstrap.use_tls() {
        return Err(BuildError::Bootstrap(
            crate::xds::bootstrap::BootstrapError::Validation(
                "TLS requested by bootstrap but no TLS feature enabled \
                 (enable tls-ring or tls-aws-lc, or supply the connection \
                 yourself with XdsClientPoolBuilder::with_control_plane_connector)"
                    .into(),
            ),
        ));
    }
    Ok((TonicTransportBuilder::new(), false))
}

#[cfg(feature = "_tls-any")]
#[derive(Clone)]
struct AdsTlsConfigProvider {
    file_watcher: Option<Arc<dyn CertificateProvider>>,
}

#[cfg(feature = "_tls-any")]
impl AdsTlsConfigProvider {
    fn new(config: FileWatcherConfig) -> Result<Self, CertProviderError> {
        let file_watcher = if config.has_certificate_files() {
            Some(Arc::new(FileWatcherProvider::new(config)?) as Arc<dyn CertificateProvider>)
        } else {
            None
        };
        Ok(Self { file_watcher })
    }

    fn client_tls_config(&self) -> xds_client::Result<tonic::transport::ClientTlsConfig> {
        let mut tls_config = tonic::transport::ClientTlsConfig::new();
        if let Some(provider) = &self.file_watcher {
            let data = provider
                .fetch()
                .map_err(|error| XdsClientError::TlsConfig(Box::new(error)))?;
            if let Some(roots) = data.roots() {
                tls_config =
                    tls_config.ca_certificate(tonic::transport::Certificate::from_pem(roots));
            } else {
                tls_config = tls_config.with_native_roots();
            }
            if let Some(identity) = data.identity() {
                tls_config = tls_config.identity(tonic::transport::Identity::from_pem(
                    identity.cert_chain(),
                    identity.key(),
                ));
            }
        } else {
            tls_config = tls_config.with_native_roots();
        }
        Ok(tls_config)
    }
}

impl Debug for XdsClientPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdsClientPool")
            .field("bootstrap", &self.inner.bootstrap)
            .field("ads", &self.inner.ads)
            .field("recorder", &recorder_name(self.inner.recorder.as_deref()))
            .finish_non_exhaustive()
    }
}

impl XdsClientPool {
    /// Returns a builder for configuring a pool.
    #[must_use]
    pub fn builder() -> XdsClientPoolBuilder {
        XdsClientPoolBuilder::default()
    }

    #[cfg(feature = "_tls-any")]
    pub(crate) fn bootstrap(&self) -> &BootstrapConfig {
        &self.inner.bootstrap
    }

    /// Returns the live client for `key`, starting one if there is none.
    pub(crate) fn get_or_create(&self, key: &str) -> Result<Arc<XdsClient>, BuildError> {
        let mut clients = self
            .inner
            .clients
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(client) = clients.get(key).and_then(Weak::upgrade) {
            return Ok(client);
        }
        let client = self.inner.start_client(key)?;
        clients.retain(|_, client| client.strong_count() > 0);
        clients.insert(key.to_owned(), Arc::downgrade(&client));
        Ok(client)
    }
}

/// Builder for an [`XdsClientPool`].
#[derive(Clone, Default)]
pub struct XdsClientPoolBuilder {
    bootstrap: Option<BootstrapConfig>,
    ads: AdsClientSettings,
    recorder: Option<Arc<dyn MetricsRecorder>>,
    /// Where started clients are reported through CSDS. The process-wide
    /// registry unless set.
    csds_registry: Option<Arc<CsdsRegistry>>,
}

impl Debug for XdsClientPoolBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdsClientPoolBuilder")
            .field("bootstrap", &self.bootstrap)
            .field("ads", &self.ads)
            .field("recorder", &recorder_name(self.recorder.as_deref()))
            .finish()
    }
}

impl XdsClientPoolBuilder {
    /// Sets the bootstrap configuration. If not set,
    /// [`build`](Self::build) loads it from the environment, like
    /// [`XdsChannelConfig::with_bootstrap`](crate::XdsChannelConfig::with_bootstrap).
    #[must_use]
    pub fn with_bootstrap(mut self, bootstrap: BootstrapConfig) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    /// Sets per-stream call credentials for the ADS stream. See
    /// [`XdsChannelConfig::with_call_credentials`](crate::XdsChannelConfig::with_call_credentials).
    #[must_use]
    pub fn with_call_credentials(mut self, creds: Arc<dyn TonicCallCredentials>) -> Self {
        self.ads.call_creds = Some(creds);
        self
    }

    /// Uses a custom connector for every xDS management server connection.
    /// See [`XdsChannelBuilder::with_control_plane_connector`](crate::XdsChannelBuilder::with_control_plane_connector)
    /// for the transport security contract.
    #[must_use]
    pub fn with_control_plane_connector<C>(mut self, connector: C) -> Self
    where
        C: ControlPlaneConnector,
    {
        self.ads.connector = Some(Arc::new(connector));
        self
    }

    /// Limits the size of a decoded ADS response, in bytes. See
    /// [`XdsChannelConfig::with_max_decoding_message_size`](crate::XdsChannelConfig::with_max_decoding_message_size).
    #[must_use]
    pub fn with_max_decoding_message_size(mut self, limit: usize) -> Self {
        self.ads.max_decoding_message_size = Some(limit);
        self
    }

    /// Limits the size of an encoded ADS request, in bytes. See
    /// [`XdsChannelConfig::with_max_encoding_message_size`](crate::XdsChannelConfig::with_max_encoding_message_size).
    #[must_use]
    pub fn with_max_encoding_message_size(mut self, limit: usize) -> Self {
        self.ads.max_encoding_message_size = Some(limit);
        self
    }

    /// Configures HTTP/2 keepalives on the ADS connection. See
    /// [`XdsChannelConfig::with_ads_keep_alive`](crate::XdsChannelConfig::with_ads_keep_alive).
    /// The override is passed to the connector through
    /// [`ControlPlaneConfig::keep_alive`].
    #[must_use]
    pub fn with_ads_keep_alive(mut self, interval: Option<Duration>, timeout: Duration) -> Self {
        self.ads.keep_alive = Some((interval, timeout));
        self
    }

    /// Sets the [`MetricsRecorder`] that receives the gRFC A78 xDS client
    /// metrics. Each client reports its pool key as the `grpc.target` label.
    #[must_use]
    pub fn with_metrics_recorder(mut self, recorder: Arc<dyn MetricsRecorder>) -> Self {
        self.recorder = Some(recorder);
        self
    }

    /// Emits the gRFC A78 xDS client metrics through an OpenTelemetry `Meter`.
    /// Requires the `otel` feature.
    #[cfg(feature = "otel")]
    #[must_use]
    pub fn with_otel_metrics(self, meter: opentelemetry::metrics::Meter) -> Self {
        self.with_metrics_recorder(Arc::new(
            xds_client_opentelemetry::OtelMetricsRecorder::new(meter),
        ))
    }

    /// Reports the pool's clients in `registry` instead of the process-wide
    /// one, so a test sees only its own clients.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn with_csds_registry(mut self, registry: Arc<CsdsRegistry>) -> Self {
        self.csds_registry = Some(registry);
        self
    }

    /// Builds the pool. Clients are started on demand, when channels are built.
    pub fn build(self) -> Result<XdsClientPool, BuildError> {
        let bootstrap = match self.bootstrap {
            Some(bootstrap) => bootstrap,
            None => BootstrapConfig::from_env()?,
        };
        Ok(XdsClientPool {
            inner: Arc::new(Inner {
                bootstrap,
                ads: self.ads,
                recorder: self.recorder,
                csds_registry: self.csds_registry.unwrap_or_else(CsdsRegistry::global),
                clients: Mutex::default(),
            }),
        })
    }
}

fn recorder_name(recorder: Option<&dyn MetricsRecorder>) -> &'static str {
    recorder.map_or("None", |r| std::any::type_name_of_val(r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    type KeepAliveOverride = Option<(Option<Duration>, Duration)>;
    type RecordedConfig = (String, KeepAliveOverride);

    #[derive(Default)]
    struct TestControlPlaneConnector {
        secure: bool,
        connects: AtomicUsize,
        configs: Mutex<Vec<RecordedConfig>>,
        reject_keep_alive: bool,
    }

    impl ControlPlaneConnector for TestControlPlaneConnector {
        fn is_secure(&self) -> bool {
            self.secure
        }

        fn connect(
            &self,
            config: ControlPlaneConfig<'_>,
        ) -> BoxFuture<xds_client::Result<Channel>> {
            self.configs
                .lock()
                .unwrap()
                .push((config.server().uri().to_owned(), config.keep_alive()));
            if self.reject_keep_alive && config.keep_alive().is_some() {
                return Box::pin(async {
                    Err(xds_client::Error::Connection(
                        "keepalive override unsupported".into(),
                    ))
                });
            }
            self.connects.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move { Ok(Channel::from_static("http://127.0.0.1:1").connect_lazy()) })
        }
    }

    #[derive(Debug)]
    struct SecureCallCredentials;

    #[tonic::async_trait]
    impl TonicCallCredentials for SecureCallCredentials {
        async fn get_request_metadata(
            &self,
            _metadata: &mut tonic::metadata::MetadataMap,
        ) -> Result<(), tonic::Status> {
            Ok(())
        }
    }

    fn test_transport_builder(
        secure: bool,
        require_secure: bool,
        options: TonicTransportBuilder,
    ) -> (ControlPlaneTransportBuilder, Arc<TestControlPlaneConnector>) {
        let connector = Arc::new(TestControlPlaneConnector {
            secure,
            ..Default::default()
        });
        (
            ControlPlaneTransportBuilder {
                connector: connector.clone(),
                options,
                keep_alive: None,
                require_secure,
            },
            connector,
        )
    }

    #[tokio::test]
    async fn connector_receives_config_on_each_connection() {
        let (mut builder, connector) =
            test_transport_builder(false, false, TonicTransportBuilder::new());
        let uris = [
            "http://127.0.0.1:1",
            "xds.example.com:443",
            "unix:///tmp/control-plane.sock",
        ];
        let overrides = [
            None,
            Some((None, Duration::from_secs(2))),
            Some((Some(Duration::from_secs(30)), Duration::from_secs(3))),
        ];
        let mut expected = Vec::new();
        for uri in uris {
            for keep_alive in overrides {
                builder.keep_alive = keep_alive;
                for _ in 0..2 {
                    builder.build(&ServerConfig::new(uri)).await.unwrap();
                    expected.push((uri.to_owned(), keep_alive));
                }
            }
        }
        assert_eq!(*connector.configs.lock().unwrap(), expected);
        assert_eq!(connector.connects.load(Ordering::SeqCst), expected.len());
    }

    #[tokio::test]
    async fn insecure_connector_rejected_for_bootstrap_and_https() {
        for (require_secure, uri) in [
            (true, "http://127.0.0.1:1"),
            (true, "xds.example.com:443"),
            (false, "https://127.0.0.1:1"),
        ] {
            let (builder, connector) =
                test_transport_builder(false, require_secure, TonicTransportBuilder::new());
            let error = builder.build(&ServerConfig::new(uri)).await.unwrap_err();
            assert!(matches!(error, xds_client::Error::Connection(_)));
            assert_eq!(connector.connects.load(Ordering::SeqCst), 0);
            assert!(connector.configs.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn call_credentials_checked_against_connector_security() {
        for secure in [false, true] {
            let options =
                TonicTransportBuilder::new().with_call_credentials(Arc::new(SecureCallCredentials));
            let (builder, connector) = test_transport_builder(secure, false, options);
            let result = builder
                .build(&ServerConfig::new("http://127.0.0.1:1"))
                .await;
            if secure {
                result.unwrap();
                assert_eq!(connector.connects.load(Ordering::SeqCst), 1);
            } else {
                assert!(matches!(result, Err(xds_client::Error::CallCredentials(_))));
                assert_eq!(connector.connects.load(Ordering::SeqCst), 0);
                assert!(connector.configs.lock().unwrap().is_empty());
            }
        }
    }

    #[tokio::test]
    async fn secure_connector_satisfies_bootstrap_and_https_requirements() {
        let (builder, connector) = test_transport_builder(true, true, TonicTransportBuilder::new());
        builder
            .build(&ServerConfig::new("https://xds.example.com:443"))
            .await
            .unwrap();
        assert_eq!(connector.connects.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn unsupported_keepalive_error_propagates_before_connecting() {
        let connector = Arc::new(TestControlPlaneConnector {
            reject_keep_alive: true,
            ..Default::default()
        });
        let builder = ControlPlaneTransportBuilder {
            connector: connector.clone(),
            options: TonicTransportBuilder::new(),
            keep_alive: Some((None, Duration::from_secs(1))),
            require_secure: false,
        };
        let error = builder
            .build(&ServerConfig::new("http://127.0.0.1:1"))
            .await
            .unwrap_err();
        assert!(
            matches!(error, xds_client::Error::Connection(message) if message == "keepalive override unsupported")
        );
        assert_eq!(connector.connects.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn default_connector_reports_insecure_bootstrap() {
        for uri in ["http://127.0.0.1:1", "unix:///tmp/control-plane.sock"] {
            let bootstrap = BootstrapConfig::builder(uri)
                .channel_creds([crate::ChannelCredentialType::Insecure])
                .build()
                .unwrap();
            let connector = GrpcControlPlaneConnector::new(&bootstrap).unwrap();
            assert!(!connector.is_secure(), "{uri}");
        }
    }

    /// A pool builder whose clients point at a port nothing listens on.
    fn test_pool_builder() -> XdsClientPoolBuilder {
        let bootstrap = BootstrapConfig::from_json(
            r#"{"xds_servers":[{"server_uri":"http://127.0.0.1:1","channel_creds":[{"type":"insecure"}]}],"node":{"id":"test"}}"#,
        )
        .unwrap();
        XdsClientPool::builder().with_bootstrap(bootstrap)
    }

    fn test_pool() -> XdsClientPool {
        test_pool_builder().build().unwrap()
    }

    #[test]
    fn clients_are_reported_in_the_global_csds_registry_by_default() {
        assert!(Arc::ptr_eq(
            &test_pool().inner.csds_registry,
            &CsdsRegistry::global()
        ));
    }

    /// A client shared under one key is reported once, until the last
    /// channel using it drops it.
    #[tokio::test]
    async fn csds_reports_each_client_once_while_it_is_used() {
        for custom_connector in [false, true] {
            let registry = Arc::new(CsdsRegistry::default());
            let mut builder = test_pool_builder().with_csds_registry(Arc::clone(&registry));
            if custom_connector {
                builder =
                    builder.with_control_plane_connector(TestControlPlaneConnector::default());
            }
            let pool = builder.build().unwrap();

            let a = pool.get_or_create("a").unwrap();
            let shared_a = pool.get_or_create("a").unwrap();
            let b = pool.get_or_create("b").unwrap();
            assert_eq!(registry.scopes(), ["a", "b"]);

            drop(a);
            assert_eq!(registry.scopes(), ["a", "b"]);
            drop(shared_a);
            assert_eq!(registry.scopes(), ["b"]);
            drop(b);
            assert!(registry.scopes().is_empty());
        }
    }

    #[tokio::test]
    async fn same_key_shares_a_client() {
        let pool = test_pool();
        let a = pool.get_or_create("a").unwrap();

        assert!(Arc::ptr_eq(&a, &pool.get_or_create("a").unwrap()));
        assert!(Arc::ptr_eq(&a, &pool.clone().get_or_create("a").unwrap()));
        assert!(!Arc::ptr_eq(&a, &pool.get_or_create("b").unwrap()));
    }

    #[tokio::test]
    async fn unused_client_is_replaced_and_pruned() {
        let pool = test_pool();
        let first = Arc::downgrade(&pool.get_or_create("a").unwrap());
        drop(pool.get_or_create("b").unwrap());
        assert!(first.upgrade().is_none());

        let a = pool.get_or_create("a").unwrap();
        let clients = pool.inner.clients.lock().unwrap();
        assert_eq!(clients.keys().collect::<Vec<_>>(), ["a"]);
        assert!(Arc::ptr_eq(&a, &clients["a"].upgrade().unwrap()));
    }

    #[cfg(feature = "_tls-any")]
    #[test]
    fn a65_empty_config_does_not_create_a_file_watcher() {
        let provider = AdsTlsConfigProvider::new(FileWatcherConfig::default()).unwrap();
        assert!(provider.file_watcher.is_none());
        provider.client_tls_config().unwrap();
    }
}
