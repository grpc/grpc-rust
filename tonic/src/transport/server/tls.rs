use std::{fmt, sync::Arc, time::Duration};

use tokio_rustls::rustls::crypto::CryptoProvider;

use super::service::TlsAcceptor;
use crate::transport::tls::{Certificate, Identity};

/// Configures TLS settings for servers.
#[derive(Clone, Default)]
pub struct ServerTlsConfig {
    identity: Option<Identity>,
    client_ca_root: Option<Certificate>,
    client_auth_optional: bool,
    ignore_client_order: bool,
    use_key_log: bool,
    timeout: Option<Duration>,
    provider: Option<Arc<CryptoProvider>>,
}

impl fmt::Debug for ServerTlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerTlsConfig").finish()
    }
}

impl ServerTlsConfig {
    /// Creates a new `ServerTlsConfig`.
    pub fn new() -> Self {
        ServerTlsConfig::default()
    }

    /// Sets the [`Identity`] of the server.
    pub fn identity(self, identity: Identity) -> Self {
        ServerTlsConfig {
            identity: Some(identity),
            ..self
        }
    }

    /// Sets a certificate against which to validate client TLS certificates.
    pub fn client_ca_root(self, cert: Certificate) -> Self {
        ServerTlsConfig {
            client_ca_root: Some(cert),
            ..self
        }
    }

    /// Sets whether client certificate verification is optional.
    ///
    /// This option has effect only if CA certificate is set.
    ///
    /// # Default
    /// By default, this option is set to `false`.
    pub fn client_auth_optional(self, optional: bool) -> Self {
        ServerTlsConfig {
            client_auth_optional: optional,
            ..self
        }
    }

    /// Sets whether the server's cipher preferences are followed instead of the client's.
    ///
    /// # Default
    /// By default, this option is set to `false`.
    pub fn ignore_client_order(self, ignore_client_order: bool) -> Self {
        ServerTlsConfig {
            ignore_client_order,
            ..self
        }
    }

    /// Use key log as specified by the `SSLKEYLOGFILE` environment variable.
    pub fn use_key_log(self) -> Self {
        ServerTlsConfig {
            use_key_log: true,
            ..self
        }
    }

    /// Sets the timeout for the TLS handshake.
    pub fn timeout(self, timeout: Duration) -> Self {
        ServerTlsConfig {
            timeout: Some(timeout),
            ..self
        }
    }

    /// Sets a custom rustls [`CryptoProvider`] used to build this server's TLS
    /// configuration.
    ///
    /// When unset, the process-wide default installed via
    /// `CryptoProvider::install_default` is used, falling back to the provider
    /// selected by the `tls-ring` or `tls-aws-lc` feature.
    pub fn with_provider(self, provider: Arc<CryptoProvider>) -> Self {
        ServerTlsConfig {
            provider: Some(provider),
            ..self
        }
    }

    pub(crate) fn tls_acceptor(&self) -> Result<TlsAcceptor, crate::BoxError> {
        TlsAcceptor::new(
            self.identity.as_ref().unwrap(),
            self.client_ca_root.as_ref(),
            self.client_auth_optional,
            self.ignore_client_order,
            self.use_key_log,
            self.timeout,
            self.provider.clone(),
        )
    }
}
