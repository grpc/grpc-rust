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

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub(crate) mod json;

/// An in-memory representation of a service config, provided to gRPC as a JSON object.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    /// Ordered list of LB policies. The first supported one is used.
    pub load_balancing_config: Option<Vec<LoadBalancingConfig>>,
    /// Per-method configuration overrides.
    pub method_config: Option<Vec<MethodConfig>>,
    /// Global retry throttling parameters.
    pub retry_throttling: Option<RetryThrottlingPolicy>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MethodConfig {
    /// List of methods this config applies to.
    pub name: Vec<MethodName>,
    #[serde(default, deserialize_with = "json::deserialize_duration_opt")]
    pub timeout: Option<Duration>,
    pub retry_policy: Option<RetryPolicy>,
    #[serde(default, deserialize_with = "json::deserialize_uint32_opt")]
    pub max_request_message_bytes: Option<u32>,
    #[serde(default, deserialize_with = "json::deserialize_uint32_opt")]
    pub max_response_message_bytes: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MethodName {
    /// e.g., "grpc.examples.echo.Echo" or "grpc.examples.echo.Echo/Echo".
    /// The service name (fully qualified).
    pub service: String,
    /// If None, applies to all methods in the service.
    pub method: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RetryThrottlingPolicy {
    pub max_tokens: u32,
    pub token_ratio: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    pub max_attempts: u32,
    #[serde(deserialize_with = "json::deserialize_duration")]
    pub initial_backoff: Duration,
    #[serde(deserialize_with = "json::deserialize_duration")]
    pub max_backoff: Duration,
    pub backoff_multiplier: f32,
    pub retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadBalancingConfig {
    pub name: String,
    pub config: serde_json::Value,
}

impl<'de> Deserialize<'de> for LoadBalancingConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancingConfig;
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
                    Ok(LoadBalancingConfig { name, config })
                } else {
                    Err(serde::de::Error::custom("map is empty"))
                }
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

impl Serialize for LoadBalancingConfig {
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

impl ServiceConfig {
    /// Parses a service configuration from a JSON string.
    pub fn parse(config_json: &str) -> Result<Self, String> {
        let config: Self = serde_json::from_str(config_json)
            .map_err(|e| format!("failed to deserialize service config JSON: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(ref method_configs) = self.method_config {
            for (i, mc) in method_configs.iter().enumerate() {
                if mc.name.is_empty() {
                    return Err(format!("method_config[{}] has no names defined", i));
                }
                for (j, name) in mc.name.iter().enumerate() {
                    if name.service.is_empty() {
                        return Err(format!(
                            "method_config[{}].name[{}] has empty service name",
                            i, j
                        ));
                    }
                }
                if let Some(ref rp) = mc.retry_policy {
                    if rp.max_attempts <= 1 {
                        return Err(format!(
                            "method_config[{}].retry_policy.max_attempts must be > 1",
                            i
                        ));
                    }
                    if rp.initial_backoff.as_nanos() == 0 {
                        return Err(format!(
                            "method_config[{}].retry_policy.initial_backoff must be > 0",
                            i
                        ));
                    }
                    if rp.max_backoff.as_nanos() == 0 {
                        return Err(format!(
                            "method_config[{}].retry_policy.max_backoff must be > 0",
                            i
                        ));
                    }
                    if rp.backoff_multiplier <= 0.0 {
                        return Err(format!(
                            "method_config[{}].retry_policy.backoff_multiplier must be > 0",
                            i
                        ));
                    }
                }
            }
        }

        if let Some(ref rt) = self.retry_throttling {
            if rt.max_tokens == 0 || rt.max_tokens > 1000 {
                return Err("retry_throttling.max_tokens must be between 1 and 1000".to_string());
            }
            if rt.token_ratio <= 0.0 {
                return Err("retry_throttling.token_ratio must be > 0".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_service_config_parsing() {
        let json_data = json!({
            "loadBalancingConfig": [
                { "pick_first": { "shuffleAddressList": true } },
                { "round_robin": {} }
            ],
            "methodConfig": [
                {
                    "name": [
                        { "service": "grpc.examples.echo.Echo", "method": "Echo" },
                        { "service": "grpc.examples.echo.Echo2" }
                    ],
                    "timeout": "1.5s",
                    "retryPolicy": {
                        "maxAttempts": 3,
                        "initialBackoff": "0.1s",
                        "maxBackoff": "1s",
                        "backoffMultiplier": 2.0,
                        "retryableStatusCodes": ["UNAVAILABLE", "INTERNAL"]
                    },
                    "maxRequestMessageBytes": 1024,
                    "maxResponseMessageBytes": "2048"
                }
            ],
            "retryThrottling": {
                "maxTokens": 100,
                "tokenRatio": 0.1
            }
        });

        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();

        // Verify Load Balancing Config.
        let lb_configs = sc.load_balancing_config.unwrap();
        assert_eq!(lb_configs.len(), 2);
        assert_eq!(lb_configs[0].name, "pick_first");
        assert_eq!(lb_configs[0].config, json!({ "shuffleAddressList": true }));
        assert_eq!(lb_configs[1].name, "round_robin");
        assert_eq!(lb_configs[1].config, json!({}));

        // Verify Method Config.
        let method_configs = sc.method_config.unwrap();
        assert_eq!(method_configs.len(), 1);
        let mc = &method_configs[0];
        assert_eq!(mc.name.len(), 2);
        assert_eq!(mc.name[0].service, "grpc.examples.echo.Echo");
        assert_eq!(mc.name[0].method, Some("Echo".to_string()));
        assert_eq!(mc.name[1].service, "grpc.examples.echo.Echo2");
        assert_eq!(mc.name[1].method, None);

        assert_eq!(mc.timeout, Some(Duration::new(1, 500_000_000)));
        assert_eq!(mc.max_request_message_bytes, Some(1024));
        assert_eq!(mc.max_response_message_bytes, Some(2048));

        // Verify Retry Policy.
        let rp = mc.retry_policy.as_ref().unwrap();
        assert_eq!(rp.max_attempts, 3);
        assert_eq!(rp.initial_backoff, Duration::new(0, 100_000_000));
        assert_eq!(rp.max_backoff, Duration::from_secs(1));
        assert_eq!(rp.backoff_multiplier, 2.0);
        assert_eq!(rp.retryable_status_codes, vec!["UNAVAILABLE", "INTERNAL"]);

        // Verify Retry Throttling.
        let rt = sc.retry_throttling.unwrap();
        assert_eq!(rt.max_tokens, 100);
        assert_eq!(rt.token_ratio, 0.1);
    }

    #[test]
    fn test_invalid_service_config_parsing() {
        // Bad JSON formatting.
        assert!(ServiceConfig::parse("{").is_err());

        // Invalid max attempts.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "foo" }],
                "retryPolicy": {
                    "maxAttempts": 1, // Invalid, must be > 1.
                    "initialBackoff": "1s",
                    "maxBackoff": "1s",
                    "backoffMultiplier": 2.0,
                    "retryableStatusCodes": []
                }
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());

        // Invalid max tokens.
        let json_data = json!({
            "retryThrottling": {
                "maxTokens": 1001, // Invalid, max is 1000.
                "tokenRatio": 0.1
            }
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());

        // Empty method name service.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "" }]
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());
    }
}
