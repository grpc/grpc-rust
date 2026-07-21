use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use super::ConnectionScaling;
use super::HealthCheckConfig;
use super::HedgingPolicy;
use super::LoadBalancingConfig;
use super::MethodConfig;
use super::MethodName;
use super::RetryPolicy;
use super::RetryThrottlingPolicy;
use super::ServiceConfig;
use super::json;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServiceConfigSerDe {
    pub(crate) load_balancing_policy: Option<String>,
    pub(crate) load_balancing_config: Option<Vec<LoadBalancingConfigSerDe>>,
    pub(crate) method_config: Option<Vec<MethodConfigSerDe>>,
    pub(crate) retry_throttling: Option<RetryThrottlingPolicySerDe>,
    pub(crate) health_check_config: Option<HealthCheckConfigSerDe>,
    pub(crate) connection_scaling: Option<ConnectionScalingSerDe>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MethodConfigSerDe {
    pub(crate) name: Vec<MethodNameSerDe>,
    pub(crate) wait_for_ready: Option<bool>,
    #[serde(default, deserialize_with = "json::deserialize_duration_opt")]
    pub(crate) timeout: Option<Duration>,
    pub(crate) retry_policy: Option<RetryPolicySerDe>,
    pub(crate) hedging_policy: Option<HedgingPolicySerDe>,
    #[serde(default, deserialize_with = "json::deserialize_uint32_opt")]
    pub(crate) max_request_message_bytes: Option<u32>,
    #[serde(default, deserialize_with = "json::deserialize_uint32_opt")]
    pub(crate) max_response_message_bytes: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct MethodNameSerDe {
    pub(crate) service: String,
    pub(crate) method: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryThrottlingPolicySerDe {
    pub(crate) max_tokens: u32,
    pub(crate) token_ratio: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryPolicySerDe {
    pub(crate) max_attempts: u32,
    #[serde(deserialize_with = "json::deserialize_duration")]
    pub(crate) initial_backoff: Duration,
    #[serde(deserialize_with = "json::deserialize_duration")]
    pub(crate) max_backoff: Duration,
    pub(crate) backoff_multiplier: f32,
    pub(crate) retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HedgingPolicySerDe {
    pub(crate) max_attempts: u32,
    #[serde(deserialize_with = "json::deserialize_duration")]
    pub(crate) hedging_delay: Duration,
    #[serde(default)]
    pub(crate) non_fatal_status_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealthCheckConfigSerDe {
    #[serde(rename = "serviceName", alias = "ServiceName")]
    pub(crate) service_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionScalingSerDe {
    #[serde(default = "default_max_connections_per_subchannel")]
    pub(crate) max_connections_per_subchannel: u32,
}

fn default_max_connections_per_subchannel() -> u32 {
    10
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadBalancingConfigSerDe {
    pub(crate) name: String,
    pub(crate) config: serde_json::Value,
}

impl<'de> Deserialize<'de> for LoadBalancingConfigSerDe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancingConfigSerDe;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "a map with a single key-value pair representing a load balancing policy",
                )
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                if let Some(name) = map.next_key::<String>()? {
                    let config = map.next_value::<serde_json::Value>()?;
                    if map.next_key::<String>()?.is_some() {
                        return Err(serde::de::Error::custom("map has more than one key"));
                    }
                    Ok(LoadBalancingConfigSerDe { name, config })
                } else {
                    Err(serde::de::Error::custom("map is empty"))
                }
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

impl Serialize for LoadBalancingConfigSerDe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &self.config)?;
        map.end()
    }
}

impl From<ServiceConfigSerDe> for ServiceConfig {
    fn from(dto: ServiceConfigSerDe) -> Self {
        let mut lb_config = dto
            .load_balancing_config
            .map(|v| v.into_iter().map(Into::into).collect());
        if lb_config.is_none()
            && let Some(ref lb_policy) = dto.load_balancing_policy
        {
            lb_config = Some(vec![LoadBalancingConfig {
                name: lb_policy.clone(),
                config: serde_json::Value::Object(serde_json::Map::new()),
            }]);
        }
        Self {
            load_balancing_policy: dto.load_balancing_policy,
            load_balancing_config: lb_config,
            method_config: dto
                .method_config
                .map(|v| v.into_iter().map(Into::into).collect()),
            retry_throttling: dto.retry_throttling.map(Into::into),
            health_check_config: dto.health_check_config.map(Into::into),
            connection_scaling: dto.connection_scaling.map(Into::into),
        }
    }
}

impl From<MethodConfigSerDe> for MethodConfig {
    fn from(dto: MethodConfigSerDe) -> Self {
        Self {
            name: dto.name.into_iter().map(Into::into).collect(),
            wait_for_ready: dto.wait_for_ready,
            timeout: dto.timeout,
            retry_policy: dto.retry_policy.map(Into::into),
            hedging_policy: dto.hedging_policy.map(Into::into),
            max_request_message_bytes: dto.max_request_message_bytes,
            max_response_message_bytes: dto.max_response_message_bytes,
        }
    }
}

impl From<MethodNameSerDe> for MethodName {
    fn from(dto: MethodNameSerDe) -> Self {
        Self {
            service: dto.service,
            method: dto.method,
        }
    }
}

impl From<RetryThrottlingPolicySerDe> for RetryThrottlingPolicy {
    fn from(dto: RetryThrottlingPolicySerDe) -> Self {
        Self {
            max_tokens: dto.max_tokens,
            token_ratio: dto.token_ratio,
        }
    }
}

impl From<RetryPolicySerDe> for RetryPolicy {
    fn from(dto: RetryPolicySerDe) -> Self {
        Self {
            max_attempts: dto.max_attempts,
            initial_backoff: dto.initial_backoff,
            max_backoff: dto.max_backoff,
            backoff_multiplier: dto.backoff_multiplier,
            retryable_status_codes: dto.retryable_status_codes,
        }
    }
}

impl From<HedgingPolicySerDe> for HedgingPolicy {
    fn from(dto: HedgingPolicySerDe) -> Self {
        Self {
            max_attempts: dto.max_attempts,
            hedging_delay: dto.hedging_delay,
            non_fatal_status_codes: dto.non_fatal_status_codes,
        }
    }
}

impl From<HealthCheckConfigSerDe> for HealthCheckConfig {
    fn from(dto: HealthCheckConfigSerDe) -> Self {
        Self {
            service_name: dto.service_name,
        }
    }
}

impl From<ConnectionScalingSerDe> for ConnectionScaling {
    fn from(dto: ConnectionScalingSerDe) -> Self {
        Self {
            max_connections_per_subchannel: dto.max_connections_per_subchannel,
        }
    }
}

impl From<LoadBalancingConfigSerDe> for LoadBalancingConfig {
    fn from(dto: LoadBalancingConfigSerDe) -> Self {
        Self {
            name: dto.name,
            config: dto.config,
        }
    }
}
