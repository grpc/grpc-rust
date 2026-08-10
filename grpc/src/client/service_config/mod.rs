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

pub(crate) mod duration;
pub(crate) mod serde;

use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::ParsedJsonLbConfig;

pub type ParseResult = Result<ServiceConfig, String>;

/// An in-memory representation of a service config, provided to gRPC as a JSON
/// object.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfig {
    pub(crate) inner: serde::ServiceConfigSerde,
}

impl ServiceConfig {
    /// Parses a service configuration from a JSON string.
    pub fn parse(config_json: &str) -> ParseResult {
        let config_serde: serde::ServiceConfigSerde = match serde_json::from_str(config_json) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("failed to deserialize service config JSON: {e}"));
            }
        };
        config_serde.validate()?;
        Ok(Self {
            inner: config_serde,
        })
    }

    /// Extracts the load balancing configuration per gRPC specification rules.
    ///
    /// Evaluates `load_balancing_config` first, falling back to legacy
    /// `load_balancing_policy`.
    pub(crate) fn lb_config(&self) -> Option<ParsedJsonLbConfig> {
        if let Some(ref config) = self.inner.load_balancing_config {
            let mut map = serde_json::Map::new();
            map.insert(config.name.clone(), config.config.clone());
            return Some(ParsedJsonLbConfig::from_value(serde_json::Value::Array(
                vec![serde_json::Value::Object(map)],
            )));
        }

        if let Some(ref policy) = self.inner.load_balancing_policy
            && GLOBAL_LB_REGISTRY.get_policy(policy).is_some()
        {
            let mut map = serde_json::Map::new();
            map.insert(
                policy.clone(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
            return Some(ParsedJsonLbConfig::from_value(serde_json::Value::Array(
                vec![serde_json::Value::Object(map)],
            )));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use serde_json::json;

    use super::duration::GrpcDuration;
    use super::serde::RetryOrHedgingPolicySerde;
    use super::serde::SerdeF32;
    use super::serde::SerdeU32;
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
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
                },
                {
                    "name": [
                        { "service": "grpc.examples.echo.EchoHedging" }
                    ],
                    "waitForReady": true,
                    "hedgingPolicy": {
                        "maxAttempts": 3,
                        "hedgingDelay": "0.5s",
                        "nonFatalStatusCodes": ["UNAVAILABLE"]
                    }
                }
            ],
            "retryThrottling": {
                "maxTokens": 100,
                "tokenRatio": 0.1
            },
            "healthCheckConfig": {
                "serviceName": "grpc.health.v1.Health"
            },
            "connectionScaling": {
                "maxConnectionsPerSubchannel": 20
            }
        });

        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();

        // Verify Load Balancing Config.
        let lb_config = sc.inner.load_balancing_config.unwrap();
        assert_eq!(lb_config.name, "pick_first");
        assert_eq!(lb_config.config, json!({ "shuffleAddressList": true }));

        // Verify Method Config.
        let method_configs = sc.inner.method_config.unwrap();
        assert_eq!(method_configs.len(), 2);
        let mc = &method_configs[0];
        assert_eq!(mc.name.len(), 2);
        assert_eq!(mc.name[0].service, "grpc.examples.echo.Echo");
        assert_eq!(mc.name[0].method, "Echo");
        assert_eq!(mc.name[1].service, "grpc.examples.echo.Echo2");
        assert_eq!(mc.name[1].method, "");

        assert_eq!(
            mc.timeout,
            Some(GrpcDuration(Duration::new(1, 500_000_000)))
        );
        assert_eq!(mc.max_request_message_bytes, Some(SerdeU32(1024)));
        assert_eq!(mc.max_response_message_bytes, Some(SerdeU32(2048)));

        // Verify Retry Policy.
        let rp = mc.retry_policy.as_ref().unwrap();
        assert_eq!(rp.max_attempts, SerdeU32(3));
        assert_eq!(
            rp.initial_backoff,
            GrpcDuration(Duration::new(0, 100_000_000))
        );
        assert_eq!(rp.max_backoff, GrpcDuration(Duration::from_secs(1)));
        assert_eq!(rp.backoff_multiplier, SerdeF32(2.0));
        assert_eq!(rp.retryable_status_codes, vec!["UNAVAILABLE", "INTERNAL"]);

        let mc2 = &method_configs[1];
        assert_eq!(mc2.wait_for_ready, Some(true));
        let hp = mc2.hedging_policy.as_ref().unwrap();
        assert_eq!(hp.max_attempts, SerdeU32(3));
        assert_eq!(hp.hedging_delay, GrpcDuration(Duration::from_millis(500)));
        assert_eq!(
            hp.non_fatal_status_codes,
            Some(vec!["UNAVAILABLE".to_string()])
        );
        assert_eq!(
            mc2.retry_or_hedging_policy(),
            Some(RetryOrHedgingPolicySerde::Hedging(hp.clone()))
        );

        // Verify Retry Throttling.
        let rt = sc.inner.retry_throttling.unwrap();
        assert_eq!(rt.max_tokens, SerdeU32(100));
        assert_eq!(rt.token_ratio, SerdeF32(0.1));

        assert_eq!(
            sc.inner.health_check_config.as_ref().unwrap().service_name,
            Some("grpc.health.v1.Health".to_string())
        );
        assert_eq!(
            sc.inner
                .connection_scaling
                .as_ref()
                .unwrap()
                .max_connections_per_subchannel,
            SerdeU32(20)
        );
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

        // Empty service name with non-empty method name.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "", "method": "Echo" }]
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());

        // Empty retryable status codes array.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "foo" }],
                "retryPolicy": {
                    "maxAttempts": 2,
                    "initialBackoff": "1s",
                    "maxBackoff": "1s",
                    "backoffMultiplier": 2.0,
                    "retryableStatusCodes": [] // Invalid, must be non-empty.
                }
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());

        // Invalid status code in retryableStatusCodes.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "foo" }],
                "retryPolicy": {
                    "maxAttempts": 2,
                    "initialBackoff": "1s",
                    "maxBackoff": "1s",
                    "backoffMultiplier": 2.0,
                    "retryableStatusCodes": ["INVALID_STATUS_CODE_NAME"]
                }
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());

        // Duplicate method_config name entry across items.
        let json_data = json!({
            "methodConfig": [
                {
                    "name": [{ "service": "foo", "method": "Bar" }]
                },
                {
                    "name": [{ "service": "foo", "method": "Bar" }]
                }
            ]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());
    }

    #[test]
    fn test_legacy_lb_policy_fallback() {
        let json_data = json!({
            "loadBalancingPolicy": "round_robin"
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        assert_eq!(
            sc.inner.load_balancing_policy,
            Some("round_robin".to_string())
        );
        assert_eq!(sc.inner.load_balancing_config, None);
    }

    #[test]
    fn test_retry_hedging_mutual_exclusivity() {
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "foo" }],
                "retryPolicy": {
                    "maxAttempts": 2,
                    "initialBackoff": "1s",
                    "maxBackoff": "1s",
                    "backoffMultiplier": 2.0,
                    "retryableStatusCodes": []
                },
                "hedgingPolicy": {
                    "maxAttempts": 2,
                    "hedgingDelay": "1s"
                }
            }]
        });
        assert!(ServiceConfig::parse(&json_data.to_string()).is_err());
    }

    #[test]
    fn test_default_method_config_empty_name() {
        let json_data = json!({
            "methodConfig": [
                {
                    "name": [],
                    "timeout": "5s",
                    "waitForReady": true
                },
                {
                    "name": [{}],
                    "maxRequestMessageBytes": 4096
                }
            ]
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        let method_configs = sc.inner.method_config.unwrap();
        assert_eq!(method_configs.len(), 2);
        assert!(method_configs[0].name.is_empty());
        assert_eq!(
            method_configs[0].timeout,
            Some(GrpcDuration(Duration::from_secs(5)))
        );
        assert_eq!(method_configs[1].name.len(), 1);
        assert_eq!(method_configs[1].name[0].service, "");
        assert_eq!(method_configs[1].name[0].method, "");
    }

    #[test]
    fn test_service_level_vs_method_level_config() {
        let json_data = json!({
            "methodConfig": [
                {
                    "name": [{ "service": "grpc.examples.echo.Echo" }],
                    "retryPolicy": {
                        "maxAttempts": 3,
                        "initialBackoff": "0.1s",
                        "maxBackoff": "1s",
                        "backoffMultiplier": 2.0,
                        "retryableStatusCodes": ["UNAVAILABLE"]
                    }
                },
                {
                    "name": [{ "service": "grpc.examples.echo.Echo", "method": "SpecialCall" }],
                    "timeout": "0.5s"
                }
            ]
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        let method_configs = sc.inner.method_config.unwrap();
        assert_eq!(method_configs[0].name[0].method, "");
        assert!(method_configs[0].retry_policy.is_some());
        assert_eq!(method_configs[1].name[0].method, "SpecialCall");
        assert_eq!(
            method_configs[1].timeout,
            Some(GrpcDuration(Duration::from_millis(500)))
        );
    }

    #[test]
    fn test_minimal_lb_config_only() {
        let json_data = json!({
            "loadBalancingConfig": [
                { "round_robin": {} }
            ]
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        assert!(sc.inner.method_config.is_none());
        assert!(sc.inner.retry_throttling.is_none());
        assert!(sc.inner.health_check_config.is_none());
        assert!(sc.inner.connection_scaling.is_none());
        let lb_config = sc.inner.load_balancing_config.unwrap();
        assert_eq!(lb_config.name, "round_robin");
        assert_eq!(lb_config.config, json!({}));
    }

    #[test]
    fn test_lb_config_resolution() {
        // 1. Explicit loadBalancingConfig selects first supported candidate
        let json_data = json!({
            "loadBalancingConfig": [
                { "unsupported_lb_policy": { "foo": "bar" } },
                { "round_robin": {} },
                { "pick_first": { "shuffleAddressList": true } }
            ]
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        let lb_cfg = sc.lb_config().unwrap();
        let val: serde_json::Value = lb_cfg.convert_to().unwrap();
        assert_eq!(
            val,
            json!([
                { "round_robin": {} }
            ])
        );

        // 2. Legacy loadBalancingPolicy fallback
        let json_data = json!({
            "loadBalancingPolicy": "round_robin"
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        let lb_cfg = sc.lb_config().unwrap();
        let val: serde_json::Value = lb_cfg.convert_to().unwrap();
        assert_eq!(val, json!([{ "round_robin": {} }]));

        // 3. No supported LB config present
        let json_data = json!({
            "loadBalancingConfig": [
                { "unsupported_lb_policy": { "foo": "bar" } }
            ]
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        assert!(sc.lb_config().is_none());

        // 4. No LB config present
        let json_data = json!({});
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        assert!(sc.lb_config().is_none());
    }
}
