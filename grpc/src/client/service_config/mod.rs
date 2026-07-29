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

use duration::GrpcDuration;

pub type ParseResult = Result<ServiceConfig, String>;

/// An in-memory representation of a service config, provided to gRPC as a JSON
/// object.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfig {
    pub(crate) load_balancing_policy: Option<String>,
    /// Ordered list of LB policies. The first supported one is used.
    pub(crate) load_balancing_config: Option<Vec<LoadBalancingConfig>>,
    /// Per-method configuration overrides.
    pub(crate) method_config: Option<Vec<MethodConfig>>,
    /// Global retry throttling parameters.
    pub(crate) retry_throttling: Option<RetryThrottlingPolicy>,
    /// Health check configuration.
    pub(crate) health_check_config: Option<HealthCheckConfig>,
    /// Connection scaling configuration.
    pub(crate) connection_scaling: Option<ConnectionScaling>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MethodConfig {
    /// List of methods this config applies to.
    pub(crate) name: Vec<MethodName>,
    pub(crate) wait_for_ready: Option<bool>,
    pub(crate) timeout: Option<GrpcDuration>,
    pub(crate) retry_policy: Option<RetryPolicy>,
    pub(crate) hedging_policy: Option<HedgingPolicy>,
    pub(crate) max_request_message_bytes: Option<u32>,
    pub(crate) max_response_message_bytes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MethodName {
    /// The service name (fully qualified).
    /// e.g., for "grpc.examples.echo.Echo/Echo" this would be
    /// "grpc.examples.echo.Echo".
    pub(crate) service: String,
    /// If None, applies to all methods in the service.
    /// e.g., for "grpc.examples.echo.Echo/Echo" this would be "Echo".
    pub(crate) method: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RetryThrottlingPolicy {
    pub(crate) max_tokens: u32,
    pub(crate) token_ratio: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RetryPolicy {
    pub(crate) max_attempts: u32,
    pub(crate) initial_backoff: GrpcDuration,
    pub(crate) max_backoff: GrpcDuration,
    pub(crate) backoff_multiplier: f32,
    pub(crate) retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HedgingPolicy {
    pub(crate) max_attempts: u32,
    pub(crate) hedging_delay: GrpcDuration,
    pub(crate) non_fatal_status_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HealthCheckConfig {
    pub(crate) service_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectionScaling {
    pub(crate) max_connections_per_subchannel: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadBalancingConfig {
    pub(crate) name: String,
    pub(crate) config: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RetryOrHedgingPolicy {
    Retry(RetryPolicy),
    Hedging(HedgingPolicy),
}

impl MethodName {
    fn validate(&self) -> Result<(), String> {
        if self.service.is_empty() && self.method.as_ref().is_some_and(|m| !m.is_empty()) {
            return Err("has empty service name with non-empty method name".to_string());
        }
        Ok(())
    }
}

impl RetryPolicy {
    fn validate(&self) -> Result<(), String> {
        if self.max_attempts <= 1 {
            return Err("max_attempts must be > 1".to_string());
        }
        if self.initial_backoff.as_nanos() == 0 {
            return Err("initial_backoff must be > 0".to_string());
        }
        if self.max_backoff.as_nanos() == 0 {
            return Err("max_backoff must be > 0".to_string());
        }
        if self.backoff_multiplier <= 0.0 {
            return Err("backoff_multiplier must be > 0".to_string());
        }
        Ok(())
    }
}

impl HedgingPolicy {
    fn validate(&self) -> Result<(), String> {
        if self.max_attempts <= 1 {
            return Err("max_attempts must be > 1".to_string());
        }
        Ok(())
    }
}

impl RetryThrottlingPolicy {
    fn validate(&self) -> Result<(), String> {
        if self.max_tokens == 0 || self.max_tokens > 1000 {
            return Err("max_tokens must be between 1 and 1000".to_string());
        }
        if self.token_ratio <= 0.0 {
            return Err("token_ratio must be > 0".to_string());
        }
        Ok(())
    }
}

impl MethodConfig {
    pub fn retry_or_hedging_policy(&self) -> Option<RetryOrHedgingPolicy> {
        self.retry_policy
            .as_ref()
            .map(|rp| RetryOrHedgingPolicy::Retry(rp.clone()))
            .or_else(|| {
                self.hedging_policy
                    .as_ref()
                    .map(|hp| RetryOrHedgingPolicy::Hedging(hp.clone()))
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

impl ServiceConfig {
    /// Parses a service configuration from a JSON string.
    pub fn parse(config_json: &str) -> ParseResult {
        let config_serde: serde::ServiceConfigSerde = match serde_json::from_str(config_json) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("failed to deserialize service config JSON: {e}"));
            }
        };
        let config: Self = config_serde.into();
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(ref method_configs) = self.method_config {
            for (i, mc) in method_configs.iter().enumerate() {
                if let Err(e) = mc.validate() {
                    return Err(format!("method_config[{i}].{e}"));
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

#[cfg(test)]
mod test {
    use std::time::Duration;

    use serde_json::json;

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
        let lb_configs = sc.load_balancing_config.unwrap();
        assert_eq!(lb_configs.len(), 2);
        assert_eq!(lb_configs[0].name, "pick_first");
        assert_eq!(lb_configs[0].config, json!({ "shuffleAddressList": true }));
        assert_eq!(lb_configs[1].name, "round_robin");
        assert_eq!(lb_configs[1].config, json!({}));

        // Verify Method Config.
        let method_configs = sc.method_config.unwrap();
        assert_eq!(method_configs.len(), 2);
        let mc = &method_configs[0];
        assert_eq!(mc.name.len(), 2);
        assert_eq!(mc.name[0].service, "grpc.examples.echo.Echo");
        assert_eq!(mc.name[0].method, Some("Echo".to_string()));
        assert_eq!(mc.name[1].service, "grpc.examples.echo.Echo2");
        assert_eq!(mc.name[1].method, None);

        assert_eq!(
            mc.timeout,
            Some(GrpcDuration(Duration::new(1, 500_000_000)))
        );
        assert_eq!(mc.max_request_message_bytes, Some(1024));
        assert_eq!(mc.max_response_message_bytes, Some(2048));

        // Verify Retry Policy.
        let rp = mc.retry_policy.as_ref().unwrap();
        assert_eq!(rp.max_attempts, 3);
        assert_eq!(
            rp.initial_backoff,
            GrpcDuration(Duration::new(0, 100_000_000))
        );
        assert_eq!(rp.max_backoff, GrpcDuration(Duration::from_secs(1)));
        assert_eq!(rp.backoff_multiplier, 2.0);
        assert_eq!(rp.retryable_status_codes, vec!["UNAVAILABLE", "INTERNAL"]);

        let mc2 = &method_configs[1];
        assert_eq!(mc2.wait_for_ready, Some(true));
        let hp = mc2.hedging_policy.as_ref().unwrap();
        assert_eq!(hp.max_attempts, 3);
        assert_eq!(hp.hedging_delay, GrpcDuration(Duration::from_millis(500)));
        assert_eq!(
            hp.non_fatal_status_codes,
            Some(vec!["UNAVAILABLE".to_string()])
        );
        assert_eq!(
            mc2.retry_or_hedging_policy(),
            Some(RetryOrHedgingPolicy::Hedging(hp.clone()))
        );

        // Verify Retry Throttling.
        let rt = sc.retry_throttling.unwrap();
        assert_eq!(rt.max_tokens, 100);
        assert_eq!(rt.token_ratio, 0.1);

        assert_eq!(
            sc.health_check_config.as_ref().unwrap().service_name,
            Some("grpc.health.v1.Health".to_string())
        );
        assert_eq!(
            sc.connection_scaling
                .as_ref()
                .unwrap()
                .max_connections_per_subchannel,
            20
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
    }

    #[test]
    fn test_legacy_lb_policy_fallback() {
        let json_data = json!({
            "loadBalancingPolicy": "round_robin"
        });
        let sc = ServiceConfig::parse(&json_data.to_string()).unwrap();
        assert_eq!(sc.load_balancing_policy, Some("round_robin".to_string()));
        let lb_configs = sc.load_balancing_config.unwrap();
        assert_eq!(lb_configs.len(), 1);
        assert_eq!(lb_configs[0].name, "round_robin");
        assert_eq!(lb_configs[0].config, json!({}));
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
        let method_configs = sc.method_config.unwrap();
        assert_eq!(method_configs.len(), 2);
        assert!(method_configs[0].name.is_empty());
        assert_eq!(
            method_configs[0].timeout,
            Some(GrpcDuration(Duration::from_secs(5)))
        );
        assert_eq!(method_configs[1].name.len(), 1);
        assert_eq!(method_configs[1].name[0].service, "");
        assert_eq!(method_configs[1].name[0].method, None);
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
        let method_configs = sc.method_config.unwrap();
        assert_eq!(method_configs[0].name[0].method, None);
        assert!(method_configs[0].retry_policy.is_some());
        assert_eq!(
            method_configs[1].name[0].method,
            Some("SpecialCall".to_string())
        );
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
        assert!(sc.method_config.is_none());
        assert!(sc.retry_throttling.is_none());
        assert!(sc.health_check_config.is_none());
        assert!(sc.connection_scaling.is_none());
        let lb_configs = sc.load_balancing_config.unwrap();
        assert_eq!(lb_configs.len(), 1);
        assert_eq!(lb_configs[0].name, "round_robin");
    }
}
