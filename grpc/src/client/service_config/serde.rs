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
use super::duration::GrpcDuration;

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
    pub(crate) timeout: Option<GrpcDuration>,
    pub(crate) retry_policy: Option<RetryPolicySerDe>,
    pub(crate) hedging_policy: Option<HedgingPolicySerDe>,
    pub(crate) max_request_message_bytes: Option<SerdeU32>,
    pub(crate) max_response_message_bytes: Option<SerdeU32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct MethodNameSerDe {
    pub(crate) service: String,
    pub(crate) method: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryThrottlingPolicySerDe {
    pub(crate) max_tokens: SerdeU32,
    pub(crate) token_ratio: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryPolicySerDe {
    pub(crate) max_attempts: SerdeU32,
    pub(crate) initial_backoff: GrpcDuration,
    pub(crate) max_backoff: GrpcDuration,
    pub(crate) backoff_multiplier: f32,
    pub(crate) retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HedgingPolicySerDe {
    pub(crate) max_attempts: SerdeU32,
    pub(crate) hedging_delay: GrpcDuration,
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
    pub(crate) max_connections_per_subchannel: SerdeU32,
}

fn default_max_connections_per_subchannel() -> SerdeU32 {
    SerdeU32(10)
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
            max_request_message_bytes: dto.max_request_message_bytes.map(Into::into),
            max_response_message_bytes: dto.max_response_message_bytes.map(Into::into),
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
            max_tokens: dto.max_tokens.into(),
            token_ratio: dto.token_ratio,
        }
    }
}

impl From<RetryPolicySerDe> for RetryPolicy {
    fn from(dto: RetryPolicySerDe) -> Self {
        Self {
            max_attempts: dto.max_attempts.into(),
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
            max_attempts: dto.max_attempts.into(),
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
            max_connections_per_subchannel: dto.max_connections_per_subchannel.into(),
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

// Wraps a u32 to provide custom serialization and deserialization.
// Specifically supports the deserialization of u32 values that may be
// represented as strings in JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct SerdeU32(pub(crate) u32);

impl From<SerdeU32> for u32 {
    fn from(v: SerdeU32) -> Self {
        v.0
    }
}

impl<'de> Deserialize<'de> for SerdeU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = SerdeU32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a u32 or a string representing a u32")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.try_into().map(SerdeU32).map_err(serde::de::Error::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map(SerdeU32).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serde_u32() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            val: Option<SerdeU32>,
        }

        let val: TestStruct = serde_json::from_value(json!({ "val": 123 })).unwrap();
        assert_eq!(val.val, Some(SerdeU32(123)));

        let val: TestStruct = serde_json::from_value(json!({ "val": "456" })).unwrap();
        assert_eq!(val.val, Some(SerdeU32(456)));

        let val: TestStruct = serde_json::from_value(json!({ "val": null })).unwrap();
        assert_eq!(val.val, None);

        let val: TestStruct = serde_json::from_value(json!({})).unwrap();
        assert_eq!(val.val, None);

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "val": "invalid" }));
        assert!(res.is_err());

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "val": -1 }));
        assert!(res.is_err());
    }
}
