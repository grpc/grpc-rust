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

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;

use super::duration::GrpcDuration;
use crate::client::load_balancing::DynLbConfig;
use crate::client::load_balancing::DynLbPolicyBuilder;
use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::ParsedJsonLbConfig;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServiceConfigSerde {
    pub(crate) load_balancing_policy: Option<String>,
    #[serde(default)]
    pub(crate) load_balancing_config: LbConfigSerde,
    pub(crate) method_config: Option<Vec<MethodConfigSerde>>,
    pub(crate) retry_throttling: Option<RetryThrottlingPolicySerde>,
    pub(crate) health_check_config: Option<HealthCheckConfigSerde>,
    pub(crate) connection_scaling: Option<ConnectionScalingSerde>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MethodConfigSerde {
    #[serde(default)]
    pub(crate) name: Vec<MethodNameSerde>,
    pub(crate) wait_for_ready: Option<bool>,
    pub(crate) timeout: Option<GrpcDuration>,
    pub(crate) max_request_message_bytes: Option<SerdeU32>,
    pub(crate) max_response_message_bytes: Option<SerdeU32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct MethodNameSerde {
    #[serde(default)]
    pub(crate) service: String,
    #[serde(default)]
    pub(crate) method: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryThrottlingPolicySerde {
    pub(crate) max_tokens: SerdeU32,
    pub(crate) token_ratio: SerdeF32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealthCheckConfigSerde {
    pub(crate) service_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionScalingSerde {
    #[serde(default = "default_max_connections_per_subchannel")]
    pub(crate) max_connections_per_subchannel: SerdeU32,
}

fn default_max_connections_per_subchannel() -> SerdeU32 {
    SerdeU32(10)
}

#[derive(Debug, Clone)]
pub(crate) struct LbInnerConfig {
    pub(crate) builder: Arc<DynLbPolicyBuilder>,
    pub(crate) config: Option<DynLbConfig>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LbConfigSerde(Option<LbInnerConfig>);

impl LbConfigSerde {
    pub(crate) fn as_ref(&self) -> Option<&LbInnerConfig> {
        self.0.as_ref()
    }

    pub(crate) fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub(crate) fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl<'de> Deserialize<'de> for LbConfigSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_entries =
            Option::<Vec<HashMap<String, serde_json::Value>>>::deserialize(deserializer)?
                .unwrap_or_default();

        if raw_entries.is_empty() {
            return Ok(LbConfigSerde(None));
        }

        for map in raw_entries {
            let mut iter = map.into_iter();
            let (Some((name, raw_config)), None) = (iter.next(), iter.next()) else {
                return Err(serde::de::Error::custom(
                    "Each load balancing config entry must contain exactly one policy name.",
                ));
            };

            if let Some(builder) = GLOBAL_LB_REGISTRY.get_policy(&name) {
                let parsed_json = ParsedJsonLbConfig::from_value(raw_config);
                let parsed_config = builder
                    .parse_config(&parsed_json)
                    .map_err(serde::de::Error::custom)?;
                return Ok(LbConfigSerde(Some(LbInnerConfig {
                    builder,
                    config: parsed_config,
                })));
            }
        }

        Err(serde::de::Error::custom(
            "No supported load balancing policy found in config.",
        ))
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

impl MethodConfigSerde {
    fn validate(&self) -> Result<(), String> {
        for (j, name) in self.name.iter().enumerate() {
            if let Err(e) = name.validate() {
                return Err(format!("name[{j}] {e}"));
            }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn test_load_balancing_config_serde() {
        use crate::client::load_balancing::pick_first::PickFirstConfig;

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct TestConfig {
            #[serde(default)]
            load_balancing_config: LbConfigSerde,
        }

        // Single supported policy without config (round_robin)
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [{ "round_robin": {} }]
        }))
        .unwrap();
        let selected = val.load_balancing_config.as_ref().unwrap();
        assert_eq!(selected.builder.name(), "round_robin");
        assert!(selected.config.is_none());

        // Multiple policies; picks first supported with parsed config (pick_first)
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "unsupported_lb_1": { "key": "val" } },
                { "pick_first": { "shuffleAddressList": true } },
                { "round_robin": {} }
            ]
        }))
        .unwrap();
        let selected = val.load_balancing_config.as_ref().unwrap();
        assert_eq!(selected.builder.name(), "pick_first");
        let pf_cfg = selected
            .config
            .as_ref()
            .unwrap()
            .downcast_ref::<PickFirstConfig>()
            .unwrap();
        assert!(pf_cfg.shuffle_address_list);

        // Invalid config for supported policy fails deserialization
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [{ "pick_first": { "shuffleAddressList": "not_a_bool" } }]
        }));
        assert!(res.is_err());

        // Non-empty array with no supported policies -> fails deserialization
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "unsupported_1": {} },
                { "unsupported_2": {} }
            ]
        }));
        assert!(res.is_err());

        // Empty array -> collapses to None
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": []
        }))
        .unwrap();
        assert!(val.load_balancing_config.is_none());

        // Null or absent -> None
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": null
        }))
        .unwrap();
        assert!(val.load_balancing_config.is_none());

        let val: TestConfig = serde_json::from_value(json!({})).unwrap();
        assert!(val.load_balancing_config.is_none());

        // Multiple policies; trailing entries after first supported are ignored
        let val: TestConfig = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "pick_first": { "shuffleAddressList": true } },
                { "unsupported": { "invalid": 123 }, "other": {} },
                {}
            ]
        }))
        .unwrap();
        let selected = val.load_balancing_config.as_ref().unwrap();
        assert_eq!(selected.builder.name(), "pick_first");

        // Invalid entry with multiple keys in single object -> Error
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [
                { "round_robin": {}, "pick_first": {} }
            ]
        }));
        assert!(res.is_err());

        // Invalid entry with empty object -> Error
        let res: Result<TestConfig, _> = serde_json::from_value(json!({
            "loadBalancingConfig": [{}]
        }));
        assert!(res.is_err());
    }
}
