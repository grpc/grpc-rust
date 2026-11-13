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

//! Validated configuration types for [gRFC A50] outlier detection.
//!
//! [`OutlierDetectionConfig`] is the input to the outlier-detection
//! algorithm. The two sub-configs gate which ejection algorithms run.
//! The `child_policy` field from A50 is not modeled — `tonic-xds`
//! currently runs P2C as its only load balancer; revisit when
//! more land.
//!
//! [gRFC A50]: https://github.com/grpc/proposal/blob/master/A50-xds-outlier-detection.md

use std::time::Duration;

use envoy_types::pb::envoy::config::cluster::v3::OutlierDetection;
use envoy_types::pb::google::protobuf::{Duration as ProtoDuration, UInt32Value};
use xds_client::Error;

/// A50 defaults, applied when the corresponding CDS field is unset.
mod defaults {
    use std::time::Duration;

    pub(super) const INTERVAL: Duration = Duration::from_secs(10);
    pub(super) const BASE_EJECTION_TIME: Duration = Duration::from_secs(30);
    pub(super) const MAX_EJECTION_TIME: Duration = Duration::from_secs(300);
    pub(super) const MAX_EJECTION_PERCENT: u32 = 10;
    pub(super) const ENFORCING_SUCCESS_RATE: u32 = 100;
    pub(super) const SUCCESS_RATE_STDEV_FACTOR: u32 = 1900;
    pub(super) const SUCCESS_RATE_MINIMUM_HOSTS: u32 = 5;
    pub(super) const SUCCESS_RATE_REQUEST_VOLUME: u32 = 100;
    pub(super) const FAILURE_PERCENTAGE_THRESHOLD: u32 = 85;
    /// Zero, so failure-percentage ejection is off unless CDS opts in.
    pub(super) const ENFORCING_FAILURE_PERCENTAGE: u32 = 0;
    pub(super) const FAILURE_PERCENTAGE_MINIMUM_HOSTS: u32 = 5;
    pub(super) const FAILURE_PERCENTAGE_REQUEST_VOLUME: u32 = 50;
}

/// A 0–100 percentage, validated at construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Percentage(u8);

impl Percentage {
    /// Construct from a raw value, returning `Err` if it exceeds 100.
    pub(crate) fn new(value: u32) -> Result<Self, PercentageError> {
        if value > 100 {
            Err(PercentageError(value))
        } else {
            Ok(Self(value as u8))
        }
    }

    /// The contained value, in `0..=100`.
    pub(crate) fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("percentage must be in 0..=100, got {0}")]
pub(crate) struct PercentageError(u32);

/// Validated A50 outlier-detection configuration for a cluster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutlierDetectionConfig {
    /// How often the ejection sweep runs.
    pub interval: Duration,
    /// Base duration for a single ejection; actual ejection time is
    /// `base_ejection_time * multiplier`, capped by `max_ejection_time`.
    pub base_ejection_time: Duration,
    /// Upper bound on `base_ejection_time * multiplier`. The spec guarantees
    /// this is at least `base_ejection_time`.
    pub max_ejection_time: Duration,
    /// Maximum percentage of endpoints that may be ejected at any time.
    pub max_ejection_percent: Percentage,
    /// Success-rate ejection parameters. `None` if the algorithm is disabled.
    pub success_rate: Option<SuccessRateConfig>,
    /// Failure-percentage ejection parameters. `None` if the algorithm is
    /// disabled.
    pub failure_percentage: Option<FailurePercentageConfig>,
}

/// Success-rate ejection parameters (gRFC A50).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SuccessRateConfig {
    /// Ejection threshold factor, scaled by 1000 (so `1900` means `1.9`).
    /// An endpoint is a candidate for ejection when its success rate falls
    /// below `mean - stdev * (stdev_factor / 1000.0)`.
    pub stdev_factor: u32,
    /// Probability that a flagged candidate is actually ejected.
    /// Set to 0 to compute statistics without enforcing.
    pub enforcing_success_rate: Percentage,
    /// Minimum number of candidate endpoints required to run the algorithm.
    pub minimum_hosts: u32,
    /// Minimum number of requests an endpoint must have seen in the last
    /// interval to be considered a candidate.
    pub request_volume: u32,
}

/// Failure-percentage ejection parameters (gRFC A50).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FailurePercentageConfig {
    /// Failure rate at or above which an endpoint is a candidate for
    /// ejection.
    pub threshold: Percentage,
    /// Probability that a flagged candidate is actually ejected.
    /// Set to 0 to compute statistics without enforcing.
    pub enforcing_failure_percentage: Percentage,
    /// Minimum number of candidate endpoints required to run the algorithm.
    pub minimum_hosts: u32,
    /// Minimum number of requests an endpoint must have seen in the last
    /// interval to be considered a candidate.
    pub request_volume: u32,
}

impl OutlierDetectionConfig {
    /// True when at least one ejection algorithm is enabled.
    pub(crate) fn is_enabled(&self) -> bool {
        self.success_rate.is_some() || self.failure_percentage.is_some()
    }

    /// Build A50 outlier-detection config from a CDS `OutlierDetection`
    /// message. `None` — the `Cluster.outlier_detection` field is unset —
    /// disables outlier detection, which is [`Self::default`].
    ///
    /// Per A50 an algorithm's sub-config is present only when its
    /// `enforcing_*` percentage is non-zero. That distinction is
    /// load-bearing rather than cosmetic: a config with both sub-configs
    /// absent is a no-op that un-ejects every address and resets ejection
    /// multipliers, whereas a present-but-never-enforced sub-config keeps
    /// outlier detection enabled and leaves prior ejections standing.
    pub(crate) fn from_proto(od: Option<&OutlierDetection>) -> xds_client::Result<Self> {
        let Some(od) = od else {
            return Ok(Self::default());
        };

        let interval = duration_or(od.interval.as_ref(), defaults::INTERVAL, "interval")?;
        let base_ejection_time = duration_or(
            od.base_ejection_time.as_ref(),
            defaults::BASE_EJECTION_TIME,
            "base_ejection_time",
        )?;
        // Envoy/A50: when unset this is the larger of 300s and
        // `base_ejection_time`, so that a config raising the latter past the
        // default doesn't end up capped below a single base interval.
        let max_ejection_time = match od.max_ejection_time.as_ref() {
            Some(d) => proto_duration(d, "max_ejection_time")?,
            None => defaults::MAX_EJECTION_TIME.max(base_ejection_time),
        };

        // Every percentage is validated before it is used to gate an
        // algorithm, so an out-of-range value NACKs the resource even when
        // the algorithm it belongs to ends up disabled.
        let max_ejection_percent = percentage_or(
            od.max_ejection_percent.as_ref(),
            defaults::MAX_EJECTION_PERCENT,
            "max_ejection_percent",
        )?;
        let enforcing_success_rate = percentage_or(
            od.enforcing_success_rate.as_ref(),
            defaults::ENFORCING_SUCCESS_RATE,
            "enforcing_success_rate",
        )?;
        let failure_percentage_threshold = percentage_or(
            od.failure_percentage_threshold.as_ref(),
            defaults::FAILURE_PERCENTAGE_THRESHOLD,
            "failure_percentage_threshold",
        )?;
        let enforcing_failure_percentage = percentage_or(
            od.enforcing_failure_percentage.as_ref(),
            defaults::ENFORCING_FAILURE_PERCENTAGE,
            "enforcing_failure_percentage",
        )?;

        let success_rate = (enforcing_success_rate.get() != 0).then(|| SuccessRateConfig {
            stdev_factor: u32_or(
                od.success_rate_stdev_factor.as_ref(),
                defaults::SUCCESS_RATE_STDEV_FACTOR,
            ),
            enforcing_success_rate,
            minimum_hosts: u32_or(
                od.success_rate_minimum_hosts.as_ref(),
                defaults::SUCCESS_RATE_MINIMUM_HOSTS,
            ),
            request_volume: u32_or(
                od.success_rate_request_volume.as_ref(),
                defaults::SUCCESS_RATE_REQUEST_VOLUME,
            ),
        });

        let failure_percentage =
            (enforcing_failure_percentage.get() != 0).then(|| FailurePercentageConfig {
                threshold: failure_percentage_threshold,
                enforcing_failure_percentage,
                minimum_hosts: u32_or(
                    od.failure_percentage_minimum_hosts.as_ref(),
                    defaults::FAILURE_PERCENTAGE_MINIMUM_HOSTS,
                ),
                request_volume: u32_or(
                    od.failure_percentage_request_volume.as_ref(),
                    defaults::FAILURE_PERCENTAGE_REQUEST_VOLUME,
                ),
            });

        Ok(Self {
            interval,
            base_ejection_time,
            max_ejection_time,
            max_ejection_percent,
            success_rate,
            failure_percentage,
        })
    }
}

/// Convert a protobuf `Duration`, rejecting the negative values the type
/// permits but A50 has no meaning for.
fn proto_duration(d: &ProtoDuration, field: &str) -> xds_client::Result<Duration> {
    if d.seconds < 0 || d.nanos < 0 {
        return Err(Error::Validation(format!(
            "outlier_detection.{field} must not be negative, got {}s {}ns",
            d.seconds, d.nanos
        )));
    }
    Ok(Duration::new(d.seconds as u64, d.nanos as u32))
}

fn duration_or(
    proto: Option<&ProtoDuration>,
    default: Duration,
    field: &str,
) -> xds_client::Result<Duration> {
    match proto {
        Some(d) => proto_duration(d, field),
        None => Ok(default),
    }
}

fn percentage_or(
    proto: Option<&UInt32Value>,
    default: u32,
    field: &str,
) -> xds_client::Result<Percentage> {
    let value = proto.map(|v| v.value).unwrap_or(default);
    Percentage::new(value).map_err(|e| Error::Validation(format!("outlier_detection.{field}: {e}")))
}

fn u32_or(proto: Option<&UInt32Value>, default: u32) -> u32 {
    proto.map(|v| v.value).unwrap_or(default)
}

impl Default for OutlierDetectionConfig {
    /// Disabled by default: both ejection algorithms are `None`, so
    /// [`Self::is_enabled`] returns `false`. The remaining fields use
    /// A50's documented defaults — they're inert until an algorithm
    /// is enabled via config update.
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(10),
            base_ejection_time: Duration::from_secs(30),
            max_ejection_time: Duration::from_secs(300),
            max_ejection_percent: Percentage(10),
            success_rate: None,
            failure_percentage: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_config() -> OutlierDetectionConfig {
        OutlierDetectionConfig {
            interval: Duration::from_secs(10),
            base_ejection_time: Duration::from_secs(30),
            max_ejection_time: Duration::from_secs(300),
            max_ejection_percent: Percentage::new(10).unwrap(),
            success_rate: None,
            failure_percentage: None,
        }
    }

    #[test]
    fn is_enabled_false_when_both_algorithms_disabled() {
        assert!(!base_config().is_enabled());
    }

    #[test]
    fn is_enabled_true_when_success_rate_present() {
        let mut c = base_config();
        c.success_rate = Some(SuccessRateConfig {
            stdev_factor: 1900,
            enforcing_success_rate: Percentage::new(100).unwrap(),
            minimum_hosts: 5,
            request_volume: 100,
        });
        assert!(c.is_enabled());
    }

    #[test]
    fn is_enabled_true_when_failure_percentage_present() {
        let mut c = base_config();
        c.failure_percentage = Some(FailurePercentageConfig {
            threshold: Percentage::new(85).unwrap(),
            enforcing_failure_percentage: Percentage::new(100).unwrap(),
            minimum_hosts: 5,
            request_volume: 50,
        });
        assert!(c.is_enabled());
    }

    #[test]
    fn percentage_accepts_zero_to_one_hundred() {
        for v in [0, 1, 50, 99, 100] {
            assert_eq!(Percentage::new(v).unwrap().get() as u32, v);
        }
    }

    #[test]
    fn percentage_rejects_values_above_one_hundred() {
        assert_eq!(Percentage::new(101), Err(PercentageError(101)));
        assert_eq!(Percentage::new(u32::MAX), Err(PercentageError(u32::MAX)));
    }

    // -- `from_proto` --

    fn pct(value: u32) -> Option<UInt32Value> {
        Some(UInt32Value { value })
    }

    fn secs(seconds: i64) -> Option<ProtoDuration> {
        Some(ProtoDuration { seconds, nanos: 0 })
    }

    #[test]
    fn absent_outlier_detection_is_the_disabled_default() {
        let config = OutlierDetectionConfig::from_proto(None).unwrap();
        assert_eq!(config, OutlierDetectionConfig::default());
        assert!(!config.is_enabled());
    }

    /// An empty message is not the same as an absent one: A50 defaults
    /// `enforcing_success_rate` to 100, so success-rate ejection is on.
    #[test]
    fn empty_message_enables_success_rate_with_a50_defaults() {
        let config =
            OutlierDetectionConfig::from_proto(Some(&OutlierDetection::default())).unwrap();

        assert_eq!(config.interval, Duration::from_secs(10));
        assert_eq!(config.base_ejection_time, Duration::from_secs(30));
        assert_eq!(config.max_ejection_time, Duration::from_secs(300));
        assert_eq!(config.max_ejection_percent.get(), 10);
        assert_eq!(
            config.success_rate,
            Some(SuccessRateConfig {
                stdev_factor: 1900,
                enforcing_success_rate: Percentage::new(100).unwrap(),
                minimum_hosts: 5,
                request_volume: 100,
            })
        );
        // `enforcing_failure_percentage` defaults to 0, so the algorithm
        // is absent rather than present-but-unenforced.
        assert_eq!(config.failure_percentage, None);
        assert!(config.is_enabled());
    }

    /// A50 treats `enforcing_* == 0` as "sub-config absent", which is what
    /// makes `is_enabled` false and lets the sweep un-eject everything.
    #[test]
    fn zero_enforcing_percentage_drops_the_sub_config() {
        let od = OutlierDetection {
            enforcing_success_rate: pct(0),
            enforcing_failure_percentage: pct(0),
            ..Default::default()
        };

        let config = OutlierDetectionConfig::from_proto(Some(&od)).unwrap();

        assert_eq!(config.success_rate, None);
        assert_eq!(config.failure_percentage, None);
        assert!(!config.is_enabled());
    }

    #[test]
    fn non_zero_enforcing_failure_percentage_builds_the_sub_config() {
        let od = OutlierDetection {
            enforcing_success_rate: pct(0),
            enforcing_failure_percentage: pct(40),
            failure_percentage_threshold: pct(70),
            failure_percentage_minimum_hosts: Some(UInt32Value { value: 3 }),
            failure_percentage_request_volume: Some(UInt32Value { value: 25 }),
            ..Default::default()
        };

        let config = OutlierDetectionConfig::from_proto(Some(&od)).unwrap();

        assert_eq!(config.success_rate, None);
        assert_eq!(
            config.failure_percentage,
            Some(FailurePercentageConfig {
                threshold: Percentage::new(70).unwrap(),
                enforcing_failure_percentage: Percentage::new(40).unwrap(),
                minimum_hosts: 3,
                request_volume: 25,
            })
        );
        assert!(config.is_enabled());
    }

    /// Envoy/A50: an unset `max_ejection_time` resolves to the larger of
    /// 300s and `base_ejection_time`, so a long `base_ejection_time` is
    /// never silently capped below itself.
    #[test]
    fn unset_max_ejection_time_never_undercuts_base_ejection_time() {
        let od = OutlierDetection {
            base_ejection_time: secs(600),
            ..Default::default()
        };

        let config = OutlierDetectionConfig::from_proto(Some(&od)).unwrap();

        assert_eq!(config.base_ejection_time, Duration::from_secs(600));
        assert_eq!(config.max_ejection_time, Duration::from_secs(600));
    }

    #[test]
    fn explicit_max_ejection_time_is_taken_verbatim() {
        let od = OutlierDetection {
            base_ejection_time: secs(600),
            max_ejection_time: secs(90),
            ..Default::default()
        };

        let config = OutlierDetectionConfig::from_proto(Some(&od)).unwrap();

        assert_eq!(config.max_ejection_time, Duration::from_secs(90));
    }

    /// Out-of-range percentages must NACK even when they belong to an
    /// algorithm that this config leaves disabled — otherwise a bad
    /// resource would be accepted and only fail once someone enabled it.
    #[test]
    fn out_of_range_percentage_rejects_even_when_algorithm_is_disabled() {
        let od = OutlierDetection {
            enforcing_success_rate: pct(0),
            enforcing_failure_percentage: pct(0),
            failure_percentage_threshold: pct(150),
            ..Default::default()
        };

        let err = OutlierDetectionConfig::from_proto(Some(&od)).unwrap_err();

        assert!(
            err.to_string()
                .contains("outlier_detection.failure_percentage_threshold"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn out_of_range_max_ejection_percent_rejects() {
        let od = OutlierDetection {
            max_ejection_percent: pct(101),
            ..Default::default()
        };

        let err = OutlierDetectionConfig::from_proto(Some(&od)).unwrap_err();

        assert!(
            err.to_string()
                .contains("outlier_detection.max_ejection_percent"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn negative_duration_rejects() {
        let od = OutlierDetection {
            interval: Some(ProtoDuration {
                seconds: -1,
                nanos: 0,
            }),
            ..Default::default()
        };

        let err = OutlierDetectionConfig::from_proto(Some(&od)).unwrap_err();

        assert!(
            err.to_string().contains("outlier_detection.interval"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn sub_second_durations_round_trip() {
        let od = OutlierDetection {
            interval: Some(ProtoDuration {
                seconds: 1,
                nanos: 500_000_000,
            }),
            ..Default::default()
        };

        let config = OutlierDetectionConfig::from_proto(Some(&od)).unwrap();

        assert_eq!(config.interval, Duration::from_millis(1500));
    }
}
