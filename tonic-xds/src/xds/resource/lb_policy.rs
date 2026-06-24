//! Cluster load-balancing policy (CDS) and ring-hash config validation.

use envoy_types::pb::envoy::config::cluster::v3::cluster::{
    self, ring_hash_lb_config::HashFunction,
};
use xds_client::Error;

/// Load balancing policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LbPolicy {
    RoundRobin,
    LeastRequest,
    /// Ring-hash (gRFC A42), carrying the validated ring bounds.
    RingHash(RingHashSettings),
}

/// Validated gRFC A42 ring-hash sizing parsed from `ring_hash_lb_config`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RingHashSettings {
    pub min_ring_size: u64,
    pub max_ring_size: u64,
}

/// gRFC A42 ring-hash sizing. `minimum_ring_size` defaults to 1024 and
/// `maximum_ring_size` to the local cap of 4096 when unset; both are clamped to
/// that cap, and any configured value above the 8M ceiling is rejected.
pub(crate) const RING_HASH_DEFAULT_MIN_SIZE: u64 = 1024;
pub(crate) const RING_HASH_SIZE_CAP: u64 = 4096;
pub(crate) const RING_HASH_SIZE_CEILING: u64 = 8 * 1024 * 1024;

impl RingHashSettings {
    /// Validate a Cluster's `lb_config` oneof as `ring_hash_lb_config` (gRFC
    /// A42).
    ///
    /// Rejects a `hash_function` other than `XX_HASH`, any ring size above the
    /// 8M ceiling, and `min_ring_size > max_ring_size`. Unset sizes take the
    /// defaults (1024 / 4096); the resolved bounds are then clamped to the
    /// local cap.
    pub(crate) fn validate(lb_config: Option<cluster::LbConfig>) -> xds_client::Result<Self> {
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
        // Checked on the resolved sizes before the cap is applied.
        if min > max {
            return Err(Error::Validation(format!(
                "ring_hash min_ring_size ({min}) is greater than max_ring_size ({max})"
            )));
        }

        Ok(RingHashSettings {
            min_ring_size: min.min(RING_HASH_SIZE_CAP),
            max_ring_size: max.min(RING_HASH_SIZE_CAP),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use envoy_types::pb::google::protobuf::UInt64Value;

    fn ring_size(value: u64) -> UInt64Value {
        UInt64Value { value }
    }

    fn lb_config(config: cluster::RingHashLbConfig) -> Option<cluster::LbConfig> {
        Some(cluster::LbConfig::RingHashLbConfig(config))
    }

    #[test]
    fn ring_hash_defaults() {
        // No ring_hash_lb_config → min 1024, max defaults to the cap; XX_HASH
        // is the default hash function.
        let settings = RingHashSettings::validate(None).unwrap();
        assert_eq!(
            settings,
            RingHashSettings {
                min_ring_size: 1024,
                max_ring_size: 4096,
            }
        );
    }

    #[test]
    fn ring_hash_custom_sizes_within_cap() {
        let settings = RingHashSettings::validate(lb_config(cluster::RingHashLbConfig {
            minimum_ring_size: Some(ring_size(2048)),
            maximum_ring_size: Some(ring_size(3000)),
            ..Default::default()
        }))
        .unwrap();
        assert_eq!(
            settings,
            RingHashSettings {
                min_ring_size: 2048,
                max_ring_size: 3000,
            }
        );
    }

    #[test]
    fn ring_hash_sizes_clamped_to_cap() {
        // Sizes within the 8M ceiling but above the local cap clamp to 4096.
        let settings = RingHashSettings::validate(lb_config(cluster::RingHashLbConfig {
            minimum_ring_size: Some(ring_size(100_000)),
            maximum_ring_size: Some(ring_size(100_000)),
            ..Default::default()
        }))
        .unwrap();
        assert_eq!(
            settings,
            RingHashSettings {
                min_ring_size: 4096,
                max_ring_size: 4096,
            }
        );
    }

    #[test]
    fn ring_hash_rejects_non_xx_hash() {
        let err = RingHashSettings::validate(lb_config(cluster::RingHashLbConfig {
            hash_function: cluster::ring_hash_lb_config::HashFunction::MurmurHash2 as i32,
            ..Default::default()
        }))
        .unwrap_err();
        assert!(err.to_string().contains("hash function"));
    }

    #[test]
    fn ring_hash_rejects_size_above_ceiling() {
        let err = RingHashSettings::validate(lb_config(cluster::RingHashLbConfig {
            maximum_ring_size: Some(ring_size(8 * 1024 * 1024 + 1)),
            ..Default::default()
        }))
        .unwrap_err();
        assert!(err.to_string().contains("exceeds the maximum"));
    }

    #[test]
    fn ring_hash_rejects_min_greater_than_max() {
        let err = RingHashSettings::validate(lb_config(cluster::RingHashLbConfig {
            minimum_ring_size: Some(ring_size(3000)),
            maximum_ring_size: Some(ring_size(2000)),
            ..Default::default()
        }))
        .unwrap_err();
        assert!(err.to_string().contains("greater than max_ring_size"));
    }
}
