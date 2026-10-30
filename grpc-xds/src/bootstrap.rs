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

//! xDS bootstrap configuration for gRPC Rust

// TODO: remove once the xDS resolver / channel integrates bootstrap.
#![allow(dead_code, unused_imports)]

mod json;

use std::fmt;
use std::sync::Arc;

use grpc::__unstable::rt::GrpcRuntime;
use grpc::credentials::ChannelCredentials;
pub use json::BootstrapError;
use xds_client::message::Node;

use crate::transport::GrpcTransportBuilder;

/// Returns the default [`Node`] with client identity populated per gRFC A27.
///
/// Sets `user_agent_name` to `"gRPC Rust"` and `user_agent_version`
/// to the crate version.
pub fn default_node() -> Node {
    Node::new("gRPC Rust", env!("CARGO_PKG_VERSION"))
}

/// Validated xDS bootstrap configuration per gRFC A27 and A71.
#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    /// Priority-ordered xDS management servers to connect to.
    xds_servers: Vec<XdsServerConfig>,
    /// Node identity and metadata of this xds client.
    node: Node,
}

impl BootstrapConfig {
    /// Creates a new bootstrap configuration with the given priority-ordered xDS servers
    /// and a default [`Node`].
    pub fn new(xds_servers: Vec<XdsServerConfig>) -> Self {
        Self {
            xds_servers,
            node: default_node(),
        }
    }

    /// Sets the node identity sent to the xDS server.
    pub fn with_node(mut self, node: Node) -> Self {
        self.node = node;
        self
    }

    /// Converts this bootstrap configuration into an [`xds_client::ClientConfig`]
    /// configured with the first server in `xds_servers`.
    pub fn to_client_config(&self) -> xds_client::ClientConfig {
        xds_client::ClientConfig::new(self.node.clone(), &self.xds_servers[0].server_uri)
    }

    /// Creates a [`GrpcTransportBuilder`] configured with the first server's
    /// channel credentials.
    pub(crate) fn to_transport_builder(&self, runtime: GrpcRuntime) -> GrpcTransportBuilder {
        GrpcTransportBuilder::new(runtime)
            .with_credentials(self.xds_servers[0].channel_credentials.clone())
    }
}

/// Configuration for a single xDS management server.
#[derive(Clone)]
pub struct XdsServerConfig {
    /// URI of the xDS server (e.g., `"xds.example.com:443"` or `"unix:///path/to/socket"`).
    server_uri: String,
    /// The channel credentials to be used for this server.
    channel_credentials: Arc<dyn ChannelCredentials>,
}

impl fmt::Debug for XdsServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdsServerConfig")
            .field("server_uri", &self.server_uri)
            .field("channel_credentials", &self.channel_credentials.info())
            .finish()
    }
}

impl XdsServerConfig {
    /// Creates a new server configuration for the given URI and channel credentials.
    pub fn new(server_uri: String, channel_credentials: Arc<dyn ChannelCredentials>) -> Self {
        Self {
            server_uri: server_uri,
            channel_credentials,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_config_new_uses_default_node() {
        let config = BootstrapConfig::new(vec![]);
        let default = default_node();
        assert_eq!(config.node.user_agent_name, default.user_agent_name);
        assert_eq!(config.node.user_agent_version, default.user_agent_version);
    }
}
