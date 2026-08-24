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

//! Configuration for the xDS client.

use std::any::Any;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;

use crate::client::retry::RetryPolicy;
use crate::message::Node;

/// Configuration for an xDS management server.
///
/// Equality and hashing currently cover the URI and transport configuration.
/// This is not yet the complete gRFC A47 server definition: known server
/// features must also participate once they are modeled.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ServerConfig {
    uri: String,
    transport: Option<TransportConfig>,
    // Future extension:
    // - `ignore_resource_deletion: bool` (gRFC A53)
    //   - Field is deprecated in gRFC a88.
    // - known server features / capabilities.
    //   - gRFC A47 requires them to participate in equality and hashing.
}

/// Opaque, value-based transport configuration for an xDS server.
///
/// [`TransportBuilder`](crate::TransportBuilder) implementations can use this
/// to carry per-server credential selection or other connection settings. It
/// is transport-only configuration, not a general extension mechanism, and is
/// not passed to resource decoders.
///
/// The concrete type and value participate in [`ServerConfig`] equality and
/// hashing. Values must implement `Eq + Hash`, so non-hashable values are
/// rejected at compile time.
#[derive(Clone)]
pub struct TransportConfig {
    inner: Arc<dyn ErasedTransportConfig>,
}

impl TransportConfig {
    /// Wraps a transport-specific configuration value.
    pub fn new<T>(config: T) -> Self
    where
        T: Eq + Hash + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(config),
        }
    }

    /// Returns the concrete configuration when it has type `T`.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.as_any().downcast_ref()
    }
}

impl fmt::Debug for TransportConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TransportConfig")
            .field(&self.inner.type_name())
            .finish()
    }
}

impl PartialEq for TransportConfig {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner) || self.inner.erased_eq(&*other.inner)
    }
}

impl Eq for TransportConfig {}

impl Hash for TransportConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Match `erased_eq`, which considers values of different concrete
        // types unequal even when their value hashes happen to be identical.
        self.inner.as_any().type_id().hash(state);
        self.inner.erased_hash(state);
    }
}

// `Eq` and `Hash` cannot be used directly through a trait object: equality
// refers to `Self`, and `Hash::hash` is generic over the hasher. This
// object-safe adapter preserves the concrete value's equality and hashing
// after it is stored behind `Arc<dyn ErasedTransportConfig>`.
trait ErasedTransportConfig: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn erased_eq(&self, other: &dyn ErasedTransportConfig) -> bool;
    fn erased_hash(&self, state: &mut dyn Hasher);
}

impl<T> ErasedTransportConfig for T
where
    T: Eq + Hash + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn erased_eq(&self, other: &dyn ErasedTransportConfig) -> bool {
        other.as_any().downcast_ref::<T>() == Some(self)
    }

    fn erased_hash(&self, mut state: &mut dyn Hasher) {
        // Select `&mut dyn Hasher` as the sized generic hasher type expected by
        // `Hash::hash`; that requires passing a mutable reference to `state`.
        self.hash(&mut state);
    }
}

impl ServerConfig {
    /// Create a new server configuration with the given URI.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            transport: None,
        }
    }

    /// Returns the URI of the management server.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the transport-specific configuration, if present.
    pub fn transport_config(&self) -> Option<&TransportConfig> {
        self.transport.as_ref()
    }

    /// Sets value-based transport configuration for this server.
    ///
    /// The value's concrete type and contents participate in the current
    /// server key. Known server features are not modeled yet.
    pub fn with_transport_config<T>(mut self, config: T) -> Self
    where
        T: Eq + Hash + Send + Sync + 'static,
    {
        self.transport = Some(TransportConfig::new(config));
        self
    }
}

/// Default timeout for initial resource response (30 seconds per gRFC A57).
pub const DEFAULT_RESOURCE_INITIAL_TIMEOUT: Duration = Duration::from_secs(30);

/// Configuration for the xDS client.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ClientConfig {
    /// Node identification sent to the xDS server.
    pub(crate) node: Node,

    /// Retry policy for connection attempts.
    ///
    /// Controls the backoff behavior when reconnecting to the xDS server.
    pub(crate) retry_policy: RetryPolicy,

    /// Priority-ordered list of xDS management servers.
    ///
    /// The client will attempt to connect to servers in order, falling back
    /// to the next server if the current one is unavailable (per gRFC A71).
    /// Index 0 has the highest priority.
    pub(crate) servers: Vec<ServerConfig>,

    /// Timeout for initial resource response (gRFC A57).
    ///
    /// If a watched resource is not received within this duration after the watch
    /// is registered, watchers receive a `ResourceDoesNotExist` error.
    ///
    /// Default: 30 seconds. Set to `None` to disable the timeout.
    pub(crate) resource_initial_timeout: Option<Duration>,

    /// gRPC channel target this xDS client serves (per gRFC A78).
    ///
    /// Used as the `grpc.target` attribute on emitted metrics. This identifies
    /// the consumer-facing data-plane channel (e.g. `xds:///my-service`).
    ///
    /// Set this when constructing the client. When unset, the `grpc.target`
    /// attribute is emitted as an empty string.
    pub(crate) target: Option<String>,
    // Future extensions:
    // - `authorities: HashMap<String, AuthorityConfig>` for xDS federation (gRFC A47)
    // - Locality / zone information for locality-aware routing
}

impl ClientConfig {
    /// Create a new configuration with a single server.
    ///
    /// Uses the default retry policy.
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::{ClientConfig, Node};
    ///
    /// let node = Node::new("grpc", "1.0")
    ///     .with_id("my-node")
    ///     .with_cluster("my-cluster");
    ///
    /// let config = ClientConfig::new(node, "https://xds.example.com:443");
    /// ```
    pub fn new(node: Node, server_uri: impl Into<String>) -> Self {
        Self {
            node,
            retry_policy: RetryPolicy::default(),
            servers: vec![ServerConfig::new(server_uri)],
            resource_initial_timeout: Some(DEFAULT_RESOURCE_INITIAL_TIMEOUT),
            target: None,
        }
    }

    /// Create a new configuration with multiple servers for fallback.
    ///
    /// Servers are tried in order; index 0 has the highest priority.
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::{ClientConfig, Node, ServerConfig};
    ///
    /// let node = Node::new("grpc", "1.0");
    /// let config = ClientConfig::with_servers(node, vec![
    ///     ServerConfig::new("https://primary.xds.example.com:443"),
    ///     ServerConfig::new("https://backup.xds.example.com:443"),
    /// ]);
    /// ```
    pub fn with_servers(node: Node, servers: Vec<ServerConfig>) -> Self {
        Self {
            node,
            retry_policy: RetryPolicy::default(),
            servers,
            resource_initial_timeout: Some(DEFAULT_RESOURCE_INITIAL_TIMEOUT),
            target: None,
        }
    }

    /// Set the retry policy.
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::{ClientConfig, Node, RetryPolicy};
    /// use std::time::Duration;
    ///
    /// let node = Node::new("grpc", "1.0");
    /// let policy = RetryPolicy::default()
    ///     .with_initial_backoff(Duration::from_millis(500)).unwrap()
    ///     .with_max_backoff(Duration::from_secs(60)).unwrap();
    ///
    /// let config = ClientConfig::new(node, "https://xds.example.com:443")
    ///     .with_retry_policy(policy);
    /// ```
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Set the timeout for initial resource response (gRFC A57).
    ///
    /// If a watched resource is not received within this duration after the watch
    /// is registered, watchers receive a `ResourceDoesNotExist` error.
    ///
    /// Set to `None` to disable the timeout.
    ///
    /// # Example
    ///
    /// ```
    /// use xds_client::{ClientConfig, Node};
    /// use std::time::Duration;
    ///
    /// let node = Node::new("grpc", "1.0");
    ///
    /// // Use a custom timeout
    /// let config = ClientConfig::new(node.clone(), "https://xds.example.com:443")
    ///     .with_resource_initial_timeout(Some(Duration::from_secs(60)));
    ///
    /// // Disable the timeout
    /// let config = ClientConfig::new(node, "https://xds.example.com:443")
    ///     .with_resource_initial_timeout(None);
    /// ```
    pub fn with_resource_initial_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.resource_initial_timeout = timeout;
        self
    }

    /// Set the gRPC channel target name (per gRFC A78).
    ///
    /// Used as the `grpc.target` attribute on emitted metrics. This is the
    /// data-plane channel target (e.g. `xds:///my-service`).
    ///
    /// Consumers that wrap `xds-client` in a channel layer (e.g. tonic-xds)
    /// should set this to the channel target. When unset, the `grpc.target`
    /// attribute is emitted as an empty string.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, hash_map::DefaultHasher};

    use super::*;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct TestTransportConfig(&'static str);

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct OtherTransportConfig(&'static str);

    #[derive(Debug, PartialEq, Eq)]
    struct ComplexTransportConfig {
        credential_type: &'static str,
        options: HashMap<&'static str, &'static str>,
    }

    // `HashMap` does not implement `Hash`, because its iteration order is not
    // stable. Hash entries in key order so hashing agrees with the map's
    // order-independent equality.
    impl Hash for ComplexTransportConfig {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.credential_type.hash(state);
            let mut options: Vec<_> = self.options.iter().collect();
            options.sort_unstable_by_key(|(key, _)| *key);
            options.hash(state);
        }
    }

    fn hash(config: &ServerConfig) -> u64 {
        let mut hasher = DefaultHasher::new();
        config.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn server_key_is_value_based() {
        let first = ServerConfig::new("https://xds.example.com:443")
            .with_transport_config(TestTransportConfig("tls"));
        let second = ServerConfig::new("https://xds.example.com:443")
            .with_transport_config(TestTransportConfig("tls"));

        assert_eq!(first, second);
        assert_eq!(hash(&first), hash(&second));
    }

    #[test]
    fn server_key_includes_uri_and_transport() {
        let base = || {
            ServerConfig::new("https://xds.example.com:443")
                .with_transport_config(TestTransportConfig("tls"))
        };

        assert_ne!(
            base(),
            ServerConfig::new("https://other.example.com:443")
                .with_transport_config(TestTransportConfig("tls"))
        );
        assert_ne!(
            base(),
            ServerConfig::new("https://xds.example.com:443")
                .with_transport_config(TestTransportConfig("insecure"))
        );
        assert_ne!(
            base(),
            ServerConfig::new("https://xds.example.com:443")
                .with_transport_config(OtherTransportConfig("tls"))
        );
    }

    #[test]
    fn transport_config_can_be_downcast_without_debugging_its_value() {
        let config = ServerConfig::new("https://xds.example.com:443")
            .with_transport_config(TestTransportConfig("secret"));
        let transport = config.transport_config().unwrap();

        assert_eq!(
            transport.downcast_ref::<TestTransportConfig>(),
            Some(&TestTransportConfig("secret"))
        );
        assert!(!format!("{transport:?}").contains("secret"));
    }

    #[test]
    fn complex_transport_config_with_hash_map_is_value_based() {
        let first = ComplexTransportConfig {
            credential_type: "tls",
            options: HashMap::from([
                ("root_cert", "test-ca.pem"),
                ("server_name", "xds.example.com"),
            ]),
        };
        // Insert the same entries in the opposite order to ensure map
        // iteration order cannot affect the server key.
        let second = ComplexTransportConfig {
            credential_type: "tls",
            options: HashMap::from([
                ("server_name", "xds.example.com"),
                ("root_cert", "test-ca.pem"),
            ]),
        };

        let first = ServerConfig::new("https://xds.example.com:443").with_transport_config(first);
        let second = ServerConfig::new("https://xds.example.com:443").with_transport_config(second);

        assert_eq!(first, second);
        assert_eq!(hash(&first), hash(&second));
    }
}
