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

#[cfg(feature = "_tls-any")]
use xds_client::Error as XdsClientError;
use xds_client::{
    ClientConfig, MetricsRecorder, Node, ProstCodec, TokioRuntime, TonicTransportBuilder, XdsClient,
};

use crate::TonicCallCredentials;
use crate::client::channel::BuildError;
use crate::xds::bootstrap::BootstrapConfig;
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider::{
    CertProviderError, CertificateProvider, file_watcher::FileWatcherProvider,
};
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider_config::FileWatcherConfig;

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
    clients: Mutex<HashMap<String, Weak<XdsClient>>>,
}

impl Inner {
    /// Starts an xDS client for the bootstrap's server. `key` becomes the
    /// gRFC A78 `grpc.target` metric label.
    fn start_client(&self, key: &str) -> Result<XdsClient, BuildError> {
        let bootstrap = &self.bootstrap;
        #[allow(unused_mut)]
        let mut transport_builder = TonicTransportBuilder::new();
        #[cfg(feature = "_tls-any")]
        if let Some(tls_config) = bootstrap.tls_config()? {
            let provider = AdsTlsConfigProvider::new(tls_config).map_err(BuildError::AdsTls)?;
            transport_builder =
                transport_builder.with_tls_config_factory(move |_| provider.client_tls_config());
        }
        #[cfg(not(feature = "_tls-any"))]
        if bootstrap.use_tls() {
            return Err(BuildError::Bootstrap(
                crate::xds::bootstrap::BootstrapError::Validation(
                    "TLS requested by bootstrap but no TLS feature enabled \
                     (enable tls-ring or tls-aws-lc)"
                        .into(),
                ),
            ));
        }

        let ads = &self.ads;
        if let Some(creds) = ads.call_creds.clone() {
            transport_builder = transport_builder.with_call_credentials(creds);
        }
        if let Some(limit) = ads.max_decoding_message_size {
            transport_builder = transport_builder.with_max_decoding_message_size(limit);
        }
        if let Some(limit) = ads.max_encoding_message_size {
            transport_builder = transport_builder.with_max_encoding_message_size(limit);
        }
        if let Some((interval, timeout)) = ads.keep_alive {
            transport_builder = transport_builder.with_keep_alive(interval, timeout);
        }

        let node = Node::try_from(bootstrap.node.clone())?;
        let client_config = ClientConfig::new(node, bootstrap.server_uri()).with_target(key);
        let mut client_builder =
            XdsClient::builder(client_config, transport_builder, ProstCodec, TokioRuntime);
        if let Some(recorder) = self.recorder.clone() {
            client_builder = client_builder.with_metrics_recorder(recorder);
        }
        Ok(client_builder.build())
    }
}

/// ADS stream settings shared by every client a pool starts.
#[derive(Clone, Debug, Default)]
struct AdsClientSettings {
    call_creds: Option<Arc<dyn TonicCallCredentials>>,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
    keep_alive: Option<(Option<Duration>, Duration)>,
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
        let client = Arc::new(self.inner.start_client(key)?);
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

    /// A pool whose clients point at a port nothing listens on.
    fn test_pool() -> XdsClientPool {
        let bootstrap = BootstrapConfig::from_json(
            r#"{"xds_servers":[{"server_uri":"http://127.0.0.1:1","channel_creds":[{"type":"insecure"}]}],"node":{"id":"test"}}"#,
        )
        .unwrap();
        XdsClientPool::builder()
            .with_bootstrap(bootstrap)
            .build()
            .unwrap()
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
