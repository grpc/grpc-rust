//! Validated Cluster resource (CDS).

use bytes::Bytes;
use envoy_types::pb::envoy::config::cluster::v3::{Cluster, cluster};
use prost::Message;
use xds_client::resource::TypeUrl;
use xds_client::{Error, Resource};

use super::security::{ClusterSecurityConfig, parse_transport_socket};

/// Validated Cluster resource.
#[derive(Debug, Clone)]
pub(crate) struct ClusterResource {
    pub name: String,
    /// The EDS service name for endpoint discovery.
    /// If not set, the cluster name is used.
    pub eds_service_name: Option<String>,
    /// The load balancing policy for this cluster.
    pub lb_policy: LbPolicy,
    /// TLS security config parsed from `transport_socket`. `None` means the
    /// cluster uses plaintext connections.
    pub security: Option<ClusterSecurityConfig>,
}

/// Load balancing policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LbPolicy {
    RoundRobin,
    LeastRequest,
    /// Ring-hash (gRFC A42). Carries the validated ring bounds parsed from
    /// `ring_hash_lb_config`.
    RingHash {
        min_ring_size: u64,
        max_ring_size: u64,
    },
}

/// gRFC A42 ring-hash sizing. `minimum_ring_size` defaults to 1024 and
/// `maximum_ring_size` to the local cap of 4096 when unset; both are clamped to
/// that cap, and any configured value above the 8M ceiling is rejected.
const RING_HASH_DEFAULT_MIN_SIZE: u64 = 1024;
const RING_HASH_SIZE_CAP: u64 = 4096;
const RING_HASH_SIZE_CEILING: u64 = 8 * 1024 * 1024;

impl Resource for ClusterResource {
    type Message = Cluster;

    const TYPE_URL: TypeUrl = TypeUrl::new("type.googleapis.com/envoy.config.cluster.v3.Cluster");

    const ALL_RESOURCES_REQUIRED_IN_SOTW: bool = true;

    fn deserialize(bytes: Bytes) -> xds_client::Result<Self::Message> {
        Cluster::decode(bytes).map_err(Into::into)
    }

    fn name(message: &Self::Message) -> &str {
        &message.name
    }

    fn validate(message: Self::Message) -> xds_client::Result<Self> {
        let name = message.name;
        if name.is_empty() {
            return Err(Error::Validation("cluster name is empty".into()));
        }

        let eds_service_name = message
            .eds_cluster_config
            .map(|eds| eds.service_name)
            .filter(|s| !s.is_empty());

        let lb_policy = match cluster::LbPolicy::try_from(message.lb_policy) {
            Ok(cluster::LbPolicy::RoundRobin) => LbPolicy::RoundRobin,
            Ok(cluster::LbPolicy::LeastRequest) => LbPolicy::LeastRequest,
            Ok(cluster::LbPolicy::RingHash) => parse_ring_hash(message.lb_config)?,
            _ => {
                return Err(Error::Validation(format!(
                    "unsupported load balancing policy: {}",
                    message.lb_policy
                )));
            }
        };

        let security = parse_transport_socket(message.transport_socket)?;

        Ok(ClusterResource {
            name,
            eds_service_name,
            lb_policy,
            security,
        })
    }
}

impl ClusterResource {
    /// Returns the EDS service name for cascading EDS subscriptions.
    /// Falls back to the cluster name if no EDS service name is set.
    pub(crate) fn eds_service_name(&self) -> &str {
        self.eds_service_name.as_deref().unwrap_or(&self.name)
    }
}

/// Parse and validate `ring_hash_lb_config` (gRFC A42) from a Cluster's
/// `lb_config` oneof into [`LbPolicy::RingHash`].
///
/// Rejects a `hash_function` other than `XX_HASH` and any ring size above the
/// 8M ceiling. Unset sizes take the xDS defaults (1024 / 8M); the resolved
/// bounds are then clamped to the local cap.
fn parse_ring_hash(lb_config: Option<cluster::LbConfig>) -> xds_client::Result<LbPolicy> {
    use cluster::ring_hash_lb_config::HashFunction;

    let (min_field, max_field, hash_function) = match lb_config {
        Some(cluster::LbConfig::RingHashLbConfig(c)) => {
            (c.minimum_ring_size, c.maximum_ring_size, c.hash_function)
        }
        // No ring_hash_lb_config → defaults (XX_HASH is the proto default).
        _ => (None, None, HashFunction::XxHash as i32),
    };

    match HashFunction::try_from(hash_function) {
        Ok(HashFunction::XxHash) => {}
        Ok(other) => {
            return Err(Error::Validation(format!(
                "unsupported ring_hash hash function: {}",
                other.as_str_name()
            )));
        }
        Err(_) => {
            return Err(Error::Validation(format!(
                "unknown ring_hash hash function: {hash_function}"
            )));
        }
    }

    let min = min_field.map_or(RING_HASH_DEFAULT_MIN_SIZE, |v| v.value);
    let max = max_field.map_or(RING_HASH_SIZE_CAP, |v| v.value);
    if min > RING_HASH_SIZE_CEILING || max > RING_HASH_SIZE_CEILING {
        return Err(Error::Validation(format!(
            "ring_hash ring size exceeds the maximum of {RING_HASH_SIZE_CEILING} \
             (min_ring_size={min}, max_ring_size={max})"
        )));
    }
    // Checked on the resolved sizes before the cap is applied, matching grpc-go.
    if min > max {
        return Err(Error::Validation(format!(
            "ring_hash min_ring_size ({min}) is greater than max_ring_size ({max})"
        )));
    }

    Ok(LbPolicy::RingHash {
        min_ring_size: min.min(RING_HASH_SIZE_CAP),
        max_ring_size: max.min(RING_HASH_SIZE_CAP),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use envoy_types::pb::envoy::config::cluster::v3::cluster::EdsClusterConfig;
    use envoy_types::pb::google::protobuf::UInt64Value;

    fn ring_size(value: u64) -> UInt64Value {
        UInt64Value { value }
    }

    fn make_cluster(name: &str) -> Cluster {
        Cluster {
            name: name.to_string(),
            lb_policy: cluster::LbPolicy::RoundRobin as i32,
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_basic() {
        let cluster = make_cluster("my-cluster");
        let validated = ClusterResource::validate(cluster).expect("should validate");
        assert_eq!(validated.name, "my-cluster");
        assert_eq!(validated.lb_policy, LbPolicy::RoundRobin);
        assert!(validated.eds_service_name.is_none());
    }

    #[test]
    fn test_eds_service_name_defaults_to_cluster_name() {
        let cluster = make_cluster("my-cluster");
        let validated = ClusterResource::validate(cluster).unwrap();
        assert_eq!(validated.eds_service_name(), "my-cluster");
    }

    #[test]
    fn test_eds_service_name() {
        let cluster = Cluster {
            name: "my-cluster".to_string(),
            eds_cluster_config: Some(EdsClusterConfig {
                service_name: "eds-svc".to_string(),
                ..Default::default()
            }),
            lb_policy: cluster::LbPolicy::RoundRobin as i32,
            ..Default::default()
        };
        let validated = ClusterResource::validate(cluster).unwrap();
        assert_eq!(validated.eds_service_name.as_deref(), Some("eds-svc"));
        assert_eq!(validated.eds_service_name(), "eds-svc");
    }

    #[test]
    fn test_least_request_lb_policy() {
        let cluster = Cluster {
            name: "lr-cluster".to_string(),
            lb_policy: cluster::LbPolicy::LeastRequest as i32,
            ..Default::default()
        };
        let validated = ClusterResource::validate(cluster).unwrap();
        assert_eq!(validated.lb_policy, LbPolicy::LeastRequest);
    }

    #[test]
    fn test_unsupported_lb_policy_is_rejected() {
        // A policy we don't support (e.g. MAGLEV) is still NACKed.
        let cluster = Cluster {
            name: "mg-cluster".to_string(),
            lb_policy: cluster::LbPolicy::Maglev as i32,
            ..Default::default()
        };
        let err = ClusterResource::validate(cluster).unwrap_err();
        assert!(
            err.to_string()
                .contains("unsupported load balancing policy")
        );
    }

    /// Build a RING_HASH cluster, optionally carrying a `ring_hash_lb_config`.
    fn ring_hash_cluster(config: Option<cluster::RingHashLbConfig>) -> Cluster {
        Cluster {
            name: "rh-cluster".to_string(),
            lb_policy: cluster::LbPolicy::RingHash as i32,
            lb_config: config.map(cluster::LbConfig::RingHashLbConfig),
            ..Default::default()
        }
    }

    #[test]
    fn test_ring_hash_defaults() {
        // No ring_hash_lb_config → min 1024, max defaults to 8M then clamps to
        // the local cap of 4096; XX_HASH is the default hash function.
        let validated = ClusterResource::validate(ring_hash_cluster(None)).unwrap();
        assert_eq!(
            validated.lb_policy,
            LbPolicy::RingHash {
                min_ring_size: 1024,
                max_ring_size: 4096,
            }
        );
    }

    #[test]
    fn test_ring_hash_custom_sizes_within_cap() {
        let validated =
            ClusterResource::validate(ring_hash_cluster(Some(cluster::RingHashLbConfig {
                minimum_ring_size: Some(ring_size(2048)),
                maximum_ring_size: Some(ring_size(3000)),
                ..Default::default()
            })))
            .unwrap();
        assert_eq!(
            validated.lb_policy,
            LbPolicy::RingHash {
                min_ring_size: 2048,
                max_ring_size: 3000,
            }
        );
    }

    #[test]
    fn test_ring_hash_sizes_clamped_to_local_cap() {
        // Sizes within the 8M ceiling but above the local cap clamp to 4096.
        let validated =
            ClusterResource::validate(ring_hash_cluster(Some(cluster::RingHashLbConfig {
                minimum_ring_size: Some(ring_size(100_000)),
                maximum_ring_size: Some(ring_size(100_000)),
                ..Default::default()
            })))
            .unwrap();
        assert_eq!(
            validated.lb_policy,
            LbPolicy::RingHash {
                min_ring_size: 4096,
                max_ring_size: 4096,
            }
        );
    }

    #[test]
    fn test_ring_hash_rejects_non_xx_hash() {
        let cluster = ring_hash_cluster(Some(cluster::RingHashLbConfig {
            hash_function: cluster::ring_hash_lb_config::HashFunction::MurmurHash2 as i32,
            ..Default::default()
        }));
        let err = ClusterResource::validate(cluster).unwrap_err();
        assert!(err.to_string().contains("hash function"));
    }

    #[test]
    fn test_ring_hash_rejects_size_above_ceiling() {
        let cluster = ring_hash_cluster(Some(cluster::RingHashLbConfig {
            maximum_ring_size: Some(ring_size(8 * 1024 * 1024 + 1)),
            ..Default::default()
        }));
        let err = ClusterResource::validate(cluster).unwrap_err();
        assert!(err.to_string().contains("exceeds the maximum"));
    }

    #[test]
    fn test_ring_hash_rejects_min_greater_than_max() {
        let cluster = ring_hash_cluster(Some(cluster::RingHashLbConfig {
            minimum_ring_size: Some(ring_size(3000)),
            maximum_ring_size: Some(ring_size(2000)),
            ..Default::default()
        }));
        let err = ClusterResource::validate(cluster).unwrap_err();
        assert!(err.to_string().contains("greater than max_ring_size"));
    }

    #[test]
    fn test_validate_empty_name() {
        let cluster = make_cluster("");
        let err = ClusterResource::validate(cluster).unwrap_err();
        assert!(err.to_string().contains("cluster name is empty"));
    }

    #[test]
    fn test_all_resources_required() {
        assert!(ClusterResource::ALL_RESOURCES_REQUIRED_IN_SOTW);
    }

    #[test]
    fn test_deserialize_roundtrip() {
        let cluster = make_cluster("test");
        let bytes = cluster.encode_to_vec();
        let deserialized = ClusterResource::deserialize(Bytes::from(bytes)).unwrap();
        assert_eq!(ClusterResource::name(&deserialized), "test");
    }
}
