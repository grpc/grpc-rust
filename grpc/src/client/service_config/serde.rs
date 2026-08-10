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

use serde::Deserialize;
use serde::Serialize;

use super::duration::GrpcDuration;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServiceConfigSerde {
    pub(crate) load_balancing_policy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "load_balancing_config_serde"
    )]
    pub(crate) load_balancing_config: Option<LoadBalancingConfigSerde>,
    pub(crate) method_config: Option<Vec<MethodConfigSerde>>,
    pub(crate) retry_throttling: Option<RetryThrottlingPolicySerde>,
    pub(crate) health_check_config: Option<HealthCheckConfigSerde>,
    pub(crate) connection_scaling: Option<ConnectionScalingSerde>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MethodConfigSerde {
    #[serde(default)]
    pub(crate) name: Vec<MethodNameSerde>,
    pub(crate) wait_for_ready: Option<bool>,
    pub(crate) timeout: Option<GrpcDuration>,
    pub(crate) retry_policy: Option<RetryPolicySerde>,
    pub(crate) hedging_policy: Option<HedgingPolicySerde>,
    pub(crate) max_request_message_bytes: Option<SerdeU32>,
    pub(crate) max_response_message_bytes: Option<SerdeU32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct MethodNameSerde {
    #[serde(default)]
    pub(crate) service: String,
    #[serde(default)]
    pub(crate) method: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryThrottlingPolicySerde {
    pub(crate) max_tokens: SerdeU32,
    pub(crate) token_ratio: SerdeF32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryPolicySerde {
    pub(crate) max_attempts: SerdeU32,
    pub(crate) initial_backoff: GrpcDuration,
    pub(crate) max_backoff: GrpcDuration,
    pub(crate) backoff_multiplier: SerdeF32,
    pub(crate) retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HedgingPolicySerde {
    pub(crate) max_attempts: SerdeU32,
    pub(crate) hedging_delay: GrpcDuration,
    pub(crate) non_fatal_status_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealthCheckConfigSerde {
    pub(crate) service_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionScalingSerde {
    #[serde(default = "default_max_connections_per_subchannel")]
    pub(crate) max_connections_per_subchannel: SerdeU32,
}

fn default_max_connections_per_subchannel() -> SerdeU32 {
    SerdeU32(10)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadBalancingConfigSerde {
    pub(crate) name: String,
    pub(crate) config: serde_json::Value,
}

impl Serialize for LoadBalancingConfigSerde {
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

#[derive(Debug, Clone)]
struct RawLbConfig {
    name: String,
    config: serde_json::Value,
}

impl<'de> Deserialize<'de> for RawLbConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RawLbConfig;
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
                    Ok(RawLbConfig { name, config })
                } else {
                    Err(serde::de::Error::custom("map is empty"))
                }
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

pub(crate) mod load_balancing_config_serde {
    use super::LoadBalancingConfigSerde;
    use super::RawLbConfig;
    use crate::client::load_balancing::GLOBAL_LB_REGISTRY;

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<LoadBalancingConfigSerde>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Option<LoadBalancingConfigSerde>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a list of load balancing policy configurations")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let mut selected = None;
                while let Some(candidate) = seq.next_element::<RawLbConfig>()? {
                    if selected.is_none()
                        && GLOBAL_LB_REGISTRY.get_policy(&candidate.name).is_some()
                    {
                        selected = Some(LoadBalancingConfigSerde {
                            name: candidate.name,
                            config: candidate.config,
                        });
                    }
                }
                Ok(selected)
            }
        }

        deserializer.deserialize_any(Visitor)
    }

    #[allow(clippy::ref_option)]
    pub(crate) fn serialize<S>(
        value: &Option<LoadBalancingConfigSerde>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        match value {
            Some(lb) => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(lb)?;
                seq.end()
            }
            None => serializer.serialize_none(),
        }
    }
}

impl MethodNameSerde {
    fn validate(&self) -> Result<(), String> {
        if self.service.is_empty() && !self.method.is_empty() {
            return Err("has empty service name with non-empty method name".to_string());
        }
        Ok(())
    }
}

#[rustfmt::skip]
const VALID_STATUS_CODES: &[&str] = &[
    "OK", "0",
    "CANCELLED", "1",
    "UNKNOWN", "2",
    "INVALID_ARGUMENT", "3",
    "DEADLINE_EXCEEDED", "4",
    "NOT_FOUND", "5",
    "ALREADY_EXISTS", "6",
    "PERMISSION_DENIED", "7",
    "RESOURCE_EXHAUSTED", "8",
    "FAILED_PRECONDITION", "9",
    "ABORTED", "10",
    "OUT_OF_RANGE", "11",
    "UNIMPLEMENTED", "12",
    "INTERNAL", "13",
    "UNAVAILABLE", "14",
    "DATA_LOSS", "15",
    "UNAUTHENTICATED", "16",
];

fn validate_status_code(code_str: &str) -> Result<(), String> {
    if VALID_STATUS_CODES.contains(&code_str) {
        Ok(())
    } else {
        Err(format!("invalid status code '{code_str}'"))
    }
}

impl RetryPolicySerde {
    fn validate(&self) -> Result<(), String> {
        if self.max_attempts.0 <= 1 {
            return Err("max_attempts must be > 1".to_string());
        }
        if self.initial_backoff.as_nanos() == 0 {
            return Err("initial_backoff must be > 0".to_string());
        }
        if self.max_backoff.as_nanos() == 0 {
            return Err("max_backoff must be > 0".to_string());
        }
        if self.backoff_multiplier.0 <= 0.0 {
            return Err("backoff_multiplier must be > 0".to_string());
        }
        if self.retryable_status_codes.is_empty() {
            return Err("retryable_status_codes must be non-empty".to_string());
        }
        for code in &self.retryable_status_codes {
            validate_status_code(code)?;
        }
        Ok(())
    }
}

impl HedgingPolicySerde {
    fn validate(&self) -> Result<(), String> {
        if self.max_attempts.0 <= 1 {
            return Err("max_attempts must be > 1".to_string());
        }
        if let Some(ref codes) = self.non_fatal_status_codes {
            for code in codes {
                validate_status_code(code)?;
            }
        }
        Ok(())
    }
}

impl RetryThrottlingPolicySerde {
    fn validate(&self) -> Result<(), String> {
        if self.max_tokens.0 == 0 || self.max_tokens.0 > 1000 {
            return Err("max_tokens must be between 1 and 1000".to_string());
        }
        if self.token_ratio.0 <= 0.0 {
            return Err("token_ratio must be > 0".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RetryOrHedgingPolicySerde {
    Retry(RetryPolicySerde),
    Hedging(HedgingPolicySerde),
}

impl MethodConfigSerde {
    pub(crate) fn retry_or_hedging_policy(&self) -> Option<RetryOrHedgingPolicySerde> {
        self.retry_policy
            .as_ref()
            .map(|rp| RetryOrHedgingPolicySerde::Retry(rp.clone()))
            .or_else(|| {
                self.hedging_policy
                    .as_ref()
                    .map(|hp| RetryOrHedgingPolicySerde::Hedging(hp.clone()))
            })
    }

    fn validate(&self) -> Result<(), String> {
        for (j, name) in self.name.iter().enumerate() {
            if let Err(e) = name.validate() {
                return Err(format!("name[{j}] {e}"));
            }
        }
        if self.retry_policy.is_some() && self.hedging_policy.is_some() {
            return Err("cannot have both retryPolicy and hedgingPolicy defined".to_string());
        }
        if let Some(ref rp) = self.retry_policy
            && let Err(e) = rp.validate()
        {
            return Err(format!("retry_policy.{e}"));
        }
        if let Some(ref hp) = self.hedging_policy
            && let Err(e) = hp.validate()
        {
            return Err(format!("hedging_policy.{e}"));
        }
        Ok(())
    }
}

impl ServiceConfigSerde {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if let Some(ref method_configs) = self.method_config {
            let mut seen_names = std::collections::HashSet::new();
            for (i, mc) in method_configs.iter().enumerate() {
                if let Err(e) = mc.validate() {
                    return Err(format!("method_config[{i}].{e}"));
                }
                for name in &mc.name {
                    let key = (&name.service, &name.method);
                    if !seen_names.insert(key) {
                        return Err(format!(
                            "duplicate method_config name entry: service='{}', method='{:?}'",
                            name.service, name.method
                        ));
                    }
                }
            }
        }

        if let Some(ref rt) = self.retry_throttling
            && let Err(e) = rt.validate()
        {
            return Err(format!("retry_throttling.{e}"));
        }

        Ok(())
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

// Wraps an f32 to provide custom serialization and deserialization.
// Specifically supports the deserialization of f32 values that may be
// represented as strings or numbers in JSON.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub(crate) struct SerdeF32(pub(crate) f32);

impl From<SerdeF32> for f32 {
    fn from(v: SerdeF32) -> Self {
        v.0
    }
}

impl<'de> Deserialize<'de> for SerdeF32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = SerdeF32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an f32 or a string representing an f32")
            }

            // gRPC spec specifies float (f32) precision for these fields, so precision
            // loss/truncation from f64/integers is expected and safe.
            #[allow(clippy::cast_possible_truncation)]
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SerdeF32(v as f32))
            }

            // gRPC spec specifies float (f32) precision for these fields, so precision
            // loss/truncation from f64/integers is expected and safe.
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SerdeF32(v as f32))
            }

            // gRPC spec specifies float (f32) precision for these fields, so precision
            // loss/truncation from f64/integers is expected and safe.
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SerdeF32(v as f32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map(SerdeF32).map_err(serde::de::Error::custom)
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

    #[test]
    fn test_serde_f32() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            val: Option<SerdeF32>,
        }

        let val: TestStruct = serde_json::from_value(json!({ "val": 0.1 })).unwrap();
        assert_eq!(val.val, Some(SerdeF32(0.1)));

        let val: TestStruct = serde_json::from_value(json!({ "val": "0.1" })).unwrap();
        assert_eq!(val.val, Some(SerdeF32(0.1)));

        let val: TestStruct = serde_json::from_value(json!({ "val": 1 })).unwrap();
        assert_eq!(val.val, Some(SerdeF32(1.0)));

        let val: TestStruct = serde_json::from_value(json!({ "val": null })).unwrap();
        assert_eq!(val.val, None);

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "val": "invalid" }));
        assert!(res.is_err());
    }

    #[test]
    fn test_validate_status_codes() {
        for &code in VALID_STATUS_CODES {
            assert!(validate_status_code(code).is_ok());
        }
        assert!(validate_status_code("INVALID_CODE").is_err());
        assert!(validate_status_code("17").is_err());
        assert!(validate_status_code("-1").is_err());
    }

    #[test]
    fn test_load_balancing_config_serde() {
        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(rename_all = "camelCase")]
        struct TestConfig {
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "load_balancing_config_serde"
            )]
            load_balancing_config: Option<LoadBalancingConfigSerde>,
        }

        // 1. Single supported policy
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [{ "round_robin": {} }]
        }))
        .unwrap();
        assert_eq!(
            val.load_balancing_config,
            Some(LoadBalancingConfigSerde {
                name: "round_robin".to_string(),
                config: json!({}),
            })
        );

        // 2. Multiple policies; picks first supported
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "unsupported_lb_1": { "key": "val" } },
                { "pick_first": { "shuffleAddressList": true } },
                { "round_robin": {} }
            ]
        }))
        .unwrap();
        assert_eq!(
            val.load_balancing_config,
            Some(LoadBalancingConfigSerde {
                name: "pick_first".to_string(),
                config: json!({ "shuffleAddressList": true }),
            })
        );

        // 3. No supported policies -> collapses to None
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "unsupported_1": {} },
                { "unsupported_2": {} }
            ]
        }))
        .unwrap();
        assert_eq!(val.load_balancing_config, None);

        // 4. Empty array -> collapses to None
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": []
        }))
        .unwrap();
        assert_eq!(val.load_balancing_config, None);

        // 5. Null or absent -> None
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": null
        }))
        .unwrap();
        assert_eq!(val.load_balancing_config, None);

        let val: TestConfig = serde_json::from_value(json!({})).unwrap();
        assert_eq!(val.load_balancing_config, None);

        // 6. Invalid entry with multiple keys in single object -> Error
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "round_robin": {}, "pick_first": {} }
            ]
        }));
        assert!(res.is_err());

        // 7. Invalid entry with empty object -> Error
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [{}]
        }));
        assert!(res.is_err());
    }
}
