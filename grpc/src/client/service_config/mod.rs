/*
 *
 * Copyright 2026 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 */
use std::time::Duration;

pub(crate) mod json;
pub(crate) mod serde;

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
pub struct MethodConfig {
    /// List of methods this config applies to.
    pub(crate) name: Vec<MethodName>,
    pub(crate) wait_for_ready: Option<bool>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) retry_policy: Option<RetryPolicy>,
    pub(crate) hedging_policy: Option<HedgingPolicy>,
    pub max_request_message_bytes: Option<u32>,
    pub(crate) max_response_message_bytes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodName {
    /// The service name (fully qualified).
    /// e.g., for "grpc.examples.echo.Echo/Echo" this would be
    /// "grpc.examples.echo.Echo".
    pub(crate) service: String,
    /// If None, applies to all methods in the service.
    /// e.g., for "grpc.examples.echo.Echo/Echo" this would be "Echo".
    pub(crate) method: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetryThrottlingPolicy {
    pub(crate) max_tokens: u32,
    pub(crate) token_ratio: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetryPolicy {
    pub(crate) max_attempts: u32,
    pub(crate) initial_backoff: Duration,
    pub(crate) max_backoff: Duration,
    pub(crate) backoff_multiplier: f32,
    pub(crate) retryable_status_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HedgingPolicy {
    pub(crate) max_attempts: u32,
    pub(crate) hedging_delay: Duration,
    pub(crate) non_fatal_status_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HealthCheckConfig {
    pub(crate) service_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionScaling {
    pub(crate) max_connections_per_subchannel: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadBalancingConfig {
    pub(crate) name: String,
    pub(crate) config: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetryOrHedgingPolicy {
    Retry(RetryPolicy),
    Hedging(HedgingPolicy),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult(pub Result<ServiceConfig, String>);

impl ParseResult {
    pub fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }

    pub fn unwrap(self) -> ServiceConfig {
        self.0.unwrap()
    }

    pub fn unwrap_err(self) -> String {
        self.0.unwrap_err()
    }

    pub fn into_inner(self) -> Result<ServiceConfig, String> {
        self.0
    }
}

impl std::ops::Deref for ParseResult {
    type Target = Result<ServiceConfig, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ServiceConfig {
    /// Parses a service configuration from a JSON string.
    pub fn parse(config_json: &str) -> ParseResult {
        let config_serde: serde::ServiceConfigSerDe = match serde_json::from_str(config_json) {
            Ok(c) => c,
            Err(e) => {
                return ParseResult(Err(format!(
                    "failed to deserialize service config JSON: {}",
                    e
                )));
            }
        };
        let config: Self = config_serde.into();
        if let Err(e) = config.validate() {
            return ParseResult(Err(e));
        }
        ParseResult(Ok(config))
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
                if mc.retry_policy.is_some() && mc.hedging_policy.is_some() {
                    return Err(format!(
                        "method_config[{}] cannot have both retryPolicy and hedgingPolicy defined",
                        i
                    ));
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
                if let Some(ref hp) = mc.hedging_policy
                    && hp.max_attempts <= 1
                {
                    return Err(format!(
                        "method_config[{}].hedging_policy.max_attempts must be > 1",
                        i
                    ));
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
    use serde_json::json;

    use super::*;

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

        let mc2 = &method_configs[1];
        assert_eq!(mc2.wait_for_ready, Some(true));
        let hp = mc2.hedging_policy.as_ref().unwrap();
        assert_eq!(hp.max_attempts, 3);
        assert_eq!(hp.hedging_delay, Duration::from_millis(500));
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

        // Empty method name service.
        let json_data = json!({
            "methodConfig": [{
                "name": [{ "service": "" }]
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
}
