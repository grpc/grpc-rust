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

//! JSON parsing and environment loading for [`BootstrapConfig`].

use std::collections::HashMap;
use std::env::VarError;
use std::path::Path;
use std::sync::Arc;

use grpc::credentials::ChannelCredentials;
use grpc_rustls::client::ClientTlsConfig;
use grpc_rustls::client::RustlsChannelCredentials;
use serde::Deserialize;
use xds_client::message::Locality;
use xds_client::message::MetadataValue;
use xds_client::message::Node;

use super::BootstrapConfig;
use super::XdsServerConfig;
use super::default_node;

/// Environment variable pointing to a bootstrap JSON file path.
const ENV_BOOTSTRAP_FILE: &str = "GRPC_XDS_BOOTSTRAP";
/// Environment variable containing inline bootstrap JSON.
const ENV_BOOTSTRAP_CONFIG: &str = "GRPC_XDS_BOOTSTRAP_CONFIG";

/// Errors that can occur when loading or validating bootstrap configuration.
#[derive(Debug)]
#[non_exhaustive]
pub enum BootstrapError {
    /// Neither `GRPC_XDS_BOOTSTRAP` nor `GRPC_XDS_BOOTSTRAP_CONFIG` environment variable is set.
    NotConfigured,
    /// Failed to read the bootstrap JSON file.
    ReadFile {
        /// File path that could not be read.
        path: String,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The environment variable content could not be converted to string.
    EnvNotUnicode,
    /// The JSON content could not be parsed.
    InvalidJson(serde_json::Error),
    /// The parsed config failed validation.
    Validation(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConfigured => write!(
                f,
                "neither {ENV_BOOTSTRAP_FILE} nor {ENV_BOOTSTRAP_CONFIG} environment variable is set"
            ),
            Self::ReadFile { path, source } => {
                write!(f, "failed to read bootstrap file '{path}': {source}")
            }
            Self::EnvNotUnicode => write!(
                f,
                "{ENV_BOOTSTRAP_CONFIG} environment variable did not contain UTF-8"
            ),
            Self::InvalidJson(err) => write!(f, "failed to parse bootstrap JSON: {err}"),
            Self::Validation(msg) => write!(f, "bootstrap config validation failed: {msg}"),
        }
    }
}

impl std::error::Error for BootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadFile { source, .. } => Some(source),
            Self::InvalidJson(err) => Some(err),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for BootstrapError {
    fn from(err: serde_json::Error) -> Self {
        Self::InvalidJson(err)
    }
}

impl BootstrapConfig {
    /// Loads bootstrap configuration from environment variables.
    ///
    /// Checks `GRPC_XDS_BOOTSTRAP` (file path) first, then falls back to
    /// `GRPC_XDS_BOOTSTRAP_CONFIG` (inline JSON).
    pub fn from_env() -> Result<Self, BootstrapError> {
        if let Some(path) = std::env::var_os(ENV_BOOTSTRAP_FILE) {
            return Self::from_file(path.as_ref());
        }

        match std::env::var(ENV_BOOTSTRAP_CONFIG) {
            Ok(json) => return Self::from_json(&json),
            Err(VarError::NotUnicode(_)) => return Err(BootstrapError::EnvNotUnicode),
            Err(VarError::NotPresent) => (),
        }

        Err(BootstrapError::NotConfigured)
    }

    /// Loads bootstrap configuration from a file path.
    pub fn from_file(path: &Path) -> Result<Self, BootstrapError> {
        let json = std::fs::read_to_string(path).map_err(|e| BootstrapError::ReadFile {
            path: path.to_string_lossy().into_owned(),
            source: e,
        })?;
        Self::from_json(&json)
    }

    /// Parses and validates bootstrap configuration from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, BootstrapError> {
        let raw: RawBootstrapConfig = serde_json::from_str(json)?;
        Self::try_from(raw)
    }
}

#[derive(Deserialize)]
struct RawBootstrapConfig {
    xds_servers: Vec<RawXdsServerConfig>,
    #[serde(default)]
    node: RawNodeConfig,
}

#[derive(Deserialize)]
struct RawXdsServerConfig {
    server_uri: String,
    channel_creds: Vec<RawChannelCredentialConfig>,
}

#[derive(Deserialize)]
struct RawChannelCredentialConfig {
    #[serde(rename = "type")]
    cred_type: String,
    #[serde(default)]
    config: Option<serde_json::Value>,
}

#[derive(Default, Deserialize)]
struct RawNodeConfig {
    #[serde(default)]
    id: String,
    #[serde(default)]
    cluster: String,
    #[serde(default)]
    locality: Option<RawLocalityConfig>,
    #[serde(default)]
    metadata: HashMap<String, serde_json::Value>,
}

#[derive(Default, Deserialize)]
struct RawLocalityConfig {
    #[serde(default)]
    region: String,
    #[serde(default)]
    zone: String,
    #[serde(default)]
    sub_zone: String,
}

impl TryFrom<RawBootstrapConfig> for BootstrapConfig {
    type Error = BootstrapError;

    fn try_from(raw: RawBootstrapConfig) -> Result<Self, Self::Error> {
        if raw.xds_servers.is_empty() {
            return Err(BootstrapError::Validation(
                "xds_servers must not be empty".into(),
            ));
        }

        let mut xds_servers = Vec::with_capacity(raw.xds_servers.len());
        for (i, raw_server) in raw.xds_servers.into_iter().enumerate() {
            if raw_server.server_uri.is_empty() {
                return Err(BootstrapError::Validation(format!(
                    "xds_servers[{i}].server_uri must not be empty"
                )));
            }
            let channel_credentials = select_channel_credentials(i, raw_server.channel_creds)?;
            xds_servers.push(XdsServerConfig::new(
                raw_server.server_uri,
                channel_credentials,
            ));
        }

        let node = Node::try_from(raw.node)?;

        Ok(Self::new(xds_servers).with_node(node))
    }
}

fn select_channel_credentials(
    server_index: usize,
    channel_creds: Vec<RawChannelCredentialConfig>,
) -> Result<Arc<dyn ChannelCredentials>, BootstrapError> {
    for cred in channel_creds {
        match cred.cred_type.as_str() {
            "tls" => {
                let tls_creds =
                RustlsChannelCredentials::new(ClientTlsConfig::new()).map_err(|e| {
                    BootstrapError::Validation(format!(
                        "xds_servers[{server_index}].channel_creds failed to initialize tls credentials: {e}"
                    ))
                })?;
                return Ok(Arc::new(tls_creds));
            }
            _ => {}
        }
    }

    Err(BootstrapError::Validation(format!(
        "xds_servers[{server_index}].channel_creds contains no supported credential types (supported: tls)"
    )))
}

impl TryFrom<RawNodeConfig> for Node {
    type Error = BootstrapError;

    fn try_from(config: RawNodeConfig) -> Result<Self, Self::Error> {
        let mut node = default_node();

        if !config.id.is_empty() {
            node = node.with_id(config.id);
        }
        if !config.cluster.is_empty() {
            node = node.with_cluster(config.cluster);
        }
        if let Some(locality) = config.locality {
            node = node.with_locality(Locality {
                region: locality.region,
                zone: locality.zone,
                sub_zone: locality.sub_zone,
            });
        }
        if !config.metadata.is_empty() {
            let mut metadata_map = HashMap::with_capacity(config.metadata.len());
            for (k, v) in config.metadata {
                let mv = json_value_to_metadata_value(v)?;
                metadata_map.insert(k, mv);
            }
            node = node.with_metadata(metadata_map);
        }

        Ok(node)
    }
}

fn json_value_to_metadata_value(val: serde_json::Value) -> Result<MetadataValue, BootstrapError> {
    Ok(match val {
        serde_json::Value::Null => MetadataValue::Null,
        serde_json::Value::Bool(b) => MetadataValue::Bool(b),
        serde_json::Value::Number(n) => MetadataValue::Number(n.as_f64().ok_or_else(|| {
            BootstrapError::Validation(format!("metadata number {n} not representable as f64"))
        })?),
        serde_json::Value::String(s) => MetadataValue::String(s),
        serde_json::Value::Array(arr) => {
            let mut items = Vec::with_capacity(arr.len());
            for elem in arr {
                items.push(json_value_to_metadata_value(elem)?);
            }
            MetadataValue::Array(items)
        }
        serde_json::Value::Object(obj) => {
            let mut fields = HashMap::with_capacity(obj.len());
            for (k, v) in obj {
                fields.insert(k, json_value_to_metadata_value(v)?);
            }
            MetadataValue::Object(fields)
        }
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::*;

    fn init_tls_provider() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let _ = rustls::crypto::ring::default_provider().install_default();
        });
    }

    fn sample_minimal_json() -> &'static str {
        r#"{
            "xds_servers": [{
                "server_uri": "xds.example.com:443",
                "channel_creds": [{"type": "tls"}]
            }]
        }"#
    }

    fn sample_full_json() -> &'static str {
        r#"{
            "xds_servers": [{
                "server_uri": "xds.example.com:443",
                "channel_creds": [
                    {"type": "google_default"},
                    {"type": "tls"}
                ],
                "server_features": ["ignore_resource_deletion"]
            }],
            "node": {
                "id": "node-123",
                "cluster": "cluster-abc",
                "locality": {
                    "region": "us-west1",
                    "zone": "us-west1-a",
                    "sub_zone": "rack-1"
                },
                "metadata": {
                    "env": "prod",
                    "version": 2
                }
            }
        }"#
    }

    #[test]
    fn parse_minimal_bootstrap() {
        init_tls_provider();
        let config = BootstrapConfig::from_json(sample_minimal_json()).unwrap();
        assert_eq!(config.xds_servers.len(), 1);
        assert_eq!(config.xds_servers[0].server_uri, "xds.example.com:443");
        assert_eq!(
            config.xds_servers[0]
                .channel_credentials
                .info()
                .security_protocol(),
            "tls"
        );
        assert!(config.node.id.is_none());
        assert_eq!(config.node.user_agent_name, "gRPC Rust");
    }

    #[test]
    fn parse_full_bootstrap() {
        init_tls_provider();
        let config = BootstrapConfig::from_json(sample_full_json()).unwrap();
        assert_eq!(config.xds_servers.len(), 1);
        assert_eq!(config.xds_servers[0].server_uri, "xds.example.com:443");
        assert_eq!(
            config.xds_servers[0]
                .channel_credentials
                .info()
                .security_protocol(),
            "tls"
        );
        assert_eq!(config.node.id.as_deref(), Some("node-123"));
        assert_eq!(config.node.cluster.as_deref(), Some("cluster-abc"));
        assert_eq!(config.node.user_agent_name, "gRPC Rust");

        let locality = config.node.locality.as_ref().unwrap();
        assert_eq!(locality.region, "us-west1");
        assert_eq!(locality.zone, "us-west1-a");
        assert_eq!(locality.sub_zone, "rack-1");

        assert_eq!(
            config.node.metadata.get("env"),
            Some(&MetadataValue::String("prod".into()))
        );
        assert_eq!(
            config.node.metadata.get("version"),
            Some(&MetadataValue::Number(2.0))
        );
    }

    #[test]
    fn validation_fails_empty_servers() {
        let json = r#"{"xds_servers": []}"#;
        let err = BootstrapConfig::from_json(json).unwrap_err();
        assert!(matches!(err, BootstrapError::Validation(_)));
        assert!(err.to_string().contains("xds_servers must not be empty"));
    }

    #[test]
    fn validation_fails_missing_channel_creds() {
        let json = r#"{"xds_servers": [{"server_uri": "localhost:50051"}]}"#;
        let err = BootstrapConfig::from_json(json).unwrap_err();
        assert!(matches!(err, BootstrapError::InvalidJson(_)));
        assert!(err.to_string().contains("missing field `channel_creds`"));
    }

    #[test]
    fn validation_fails_no_supported_channel_creds() {
        let json = r#"{
            "xds_servers": [{
                "server_uri": "localhost:50051",
                "channel_creds": [{"type": "google_default"}, {"type": "unknown"}]
            }]
        }"#;
        let err = BootstrapConfig::from_json(json).unwrap_err();
        assert!(matches!(err, BootstrapError::Validation(_)));
        assert!(
            err.to_string()
                .contains("contains no supported credential types")
        );
    }
}
