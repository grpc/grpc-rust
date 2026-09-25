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

//! Child policy wrapper for the `priority_experimental` load balancing policy.
//!
//! Each priority in the priority policy is represented by an instance of
//! [`ChildPolicy`], constructed via [`ChildBuilder`].
//!
//! [`ChildPolicy`] wraps a [`GracefulSwitchPolicy`] to support dynamic
//! switching between child LB policies (e.g., transitioning from `round_robin`
//! to `pick_first`) without dropping in-flight RPCs or abruptly terminating
//! connections.
//!
//! It also implements re-resolution filtering as specified in [gRFC A37] and
//! [gRFC A56]: if `ignore_reresolution_requests` is set to `true` in
//! [`PriorityChildConfig`], re-resolution requests triggered by this child
//! (e.g., when it enters `TRANSIENT_FAILURE`) are intercepted and suppressed
//! via [`WrappedController`], preventing redundant name resolution churn.
//!
//! [gRFC A37]:
//!   https://github.com/grpc/proposal/blob/master/A37-xds-aggregate-and-logical-dns-clusters.md
//! [gRFC A56]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md

use std::sync::Arc;

use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::DynLbConfig;
use crate::client::load_balancing::DynLbPolicyBuilder;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::ParsedJsonLbConfig;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::WorkScheduler;
use crate::client::load_balancing::graceful_switch::GracefulSwitchLbConfig;
use crate::client::load_balancing::graceful_switch::GracefulSwitchPolicy;
use crate::client::load_balancing::subchannel::Subchannel;
use crate::client::load_balancing::subchannel::SubchannelState;
use crate::client::name_resolution::ResolverUpdate;
use crate::client::service_config::serde_bindings::LbConfigSerde;
use crate::client::service_config::serde_bindings::LbInnerConfig;
use crate::core::Address;

/// Configuration for an individual child under the `priority_experimental`
/// LB policy.
///
/// Corresponds to the protobuf message
/// `PriorityLoadBalancingPolicyConfig.Child` defined in
/// [gRFC A56 (Section LB Policy Configuration)]:
///
/// ```proto
/// message Child {
///   repeated LoadBalancingConfig config = 1;
///   bool ignore_reresolution_requests = 2;
/// }
/// ```
///
/// [gRFC A56 (Section LB Policy Configuration)]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#lb-policy-configuration
#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct PriorityChildConfig {
    /// The child load balancing policy configuration, specifying the policy
    /// to instantiate (e.g., `round_robin`, `pick_first`, `weighted_target`)
    /// and its policy-specific configuration.
    config: ChildPolicyConfig,

    /// If `true`, re-resolution requests from this child policy will be ignored
    /// and not forwarded to the channel controller.
    ///
    /// This prevents lower-priority or failover children from triggering
    /// unnecessary name resolution when they enter `TRANSIENT_FAILURE`.
    /// See [gRFC A37] and [gRFC A56] for details.
    ///
    /// [gRFC A37]:
    ///   https://github.com/grpc/proposal/blob/master/A37-xds-aggregate-and-logical-dns-clusters.md
    /// [gRFC A56]:
    ///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md
    #[serde(default)]
    ignore_reresolution_requests: bool,
}

/// Parsed load balancing policy configuration for a child balancer.
///
/// Deserializes using [`LbConfigSerde`] and ensures that a supported child
/// load balancing policy is present (i.e. `as_ref()` is `Some`), failing
/// deserialization otherwise.
#[derive(Debug, Clone)]
struct ChildPolicyConfig {
    builder: Arc<DynLbPolicyBuilder>,
    lb_config: DynLbConfig,
}

impl<'de> serde::Deserialize<'de> for ChildPolicyConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let lb_config = LbConfigSerde::deserialize(deserializer)?;
        let inner = lb_config.as_ref().ok_or_else(|| {
            serde::de::Error::custom("child load balancing policy config must not be empty")
        })?;
        let LbInnerConfig { builder, config } = inner.clone();
        Ok(Self {
            builder,
            lb_config: config,
        })
    }
}

/// Builder for [`ChildPolicy`].
///
/// Produces wrapped LB policy instances that wrap a [`GracefulSwitchPolicy`]
/// and filter re-resolution requests if
/// [`PriorityChildConfig::ignore_reresolution_requests`] is set to `true`.
///
/// This builder is used internally by the priority policy to instantiate
/// children managed by [`ChildManager`], and is not registered in
/// [`GLOBAL_LB_REGISTRY`].
///
/// [`ChildManager`]: crate::client::load_balancing::child_manager::ChildManager
/// [`GLOBAL_LB_REGISTRY`]: crate::client::load_balancing::GLOBAL_LB_REGISTRY
#[derive(Debug)]
pub(super) struct ChildBuilder {}

impl LbPolicyBuilder for ChildBuilder {
    type LbPolicy = ChildPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        let graceful_switch = GracefulSwitchPolicy::new(options.runtime, options.work_scheduler);
        ChildPolicy {
            graceful_switch,
            ignore_reresolution_requests: false,
        }
    }

    fn name(&self) -> &'static str {
        "priority_child_lb"
    }

    /// Config parsing is a no-op here because child configurations are
    /// deserialized and validated as part of `PriorityConfig` in the parent
    /// priority policy.
    fn parse_config(
        &self,
        _config: &ParsedJsonLbConfig,
    ) -> Result<<Self::LbPolicy as LbPolicy>::LbConfig, String> {
        unreachable!("config should be parsed through the priority_experimental builder")
    }
}

/// A child load balancing policy wrapper for the priority balancer.
///
/// It delegates load balancing duties to an inner [`GracefulSwitchPolicy`]
/// while intercepting control operations from the child to the channel
/// controller. Specifically, if `ignore_reresolution_requests` is enabled in
/// the active [`PriorityChildConfig`], re-resolution requests from the child
/// are filtered out to prevent unnecessary DNS/resolver queries during
/// priority failovers.
#[derive(Debug)]
pub(super) struct ChildPolicy {
    graceful_switch: GracefulSwitchPolicy,
    ignore_reresolution_requests: bool,
}

impl LbPolicy for ChildPolicy {
    type LbConfig = PriorityChildConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        priority_child_cfg: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        self.ignore_reresolution_requests = priority_child_cfg.ignore_reresolution_requests;
        let mut wrapped_controller =
            WrappedController::new(channel_controller, self.ignore_reresolution_requests);
        let gs_cfg = GracefulSwitchLbConfig::new(
            priority_child_cfg.config.builder.clone(),
            priority_child_cfg.config.lb_config.clone(),
        );
        self.graceful_switch
            .resolver_update(update, &gs_cfg, &mut wrapped_controller)
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        let mut wrapped_controller =
            WrappedController::new(channel_controller, self.ignore_reresolution_requests);
        self.graceful_switch.work(data, &mut wrapped_controller);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        let mut wrapped_controller =
            WrappedController::new(channel_controller, self.ignore_reresolution_requests);
        self.graceful_switch.exit_idle(&mut wrapped_controller);
    }
}

/// A [`ChannelController`] proxy that conditionally filters re-resolution
/// requests.
struct WrappedController<'a> {
    channel_controller: &'a mut dyn ChannelController,
    ignore_reresolution_requests: bool,
}

impl<'a> WrappedController<'a> {
    fn new(
        channel_controller: &'a mut dyn ChannelController,
        ignore_reresolution_requests: bool,
    ) -> Self {
        Self {
            ignore_reresolution_requests,
            channel_controller,
        }
    }
}

impl ChannelController for WrappedController<'_> {
    fn new_subchannel(
        &mut self,
        address: &Address,
        work_scheduer: Arc<dyn WorkScheduler>,
    ) -> (Arc<dyn Subchannel>, SubchannelState) {
        self.channel_controller
            .new_subchannel(address, work_scheduer)
    }

    fn update_picker(&mut self, update: LbState) {
        self.channel_controller.update_picker(update);
    }

    fn request_resolution(&mut self) {
        if !self.ignore_reresolution_requests {
            self.channel_controller.request_resolution();
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::sync::mpsc;

    use super::*;
    use crate::client::load_balancing::pick_first::PickFirstConfig;
    use crate::client::load_balancing::test_utils;
    use crate::client::load_balancing::test_utils::TestChannelController;
    use crate::client::load_balancing::test_utils::TestEvent;
    use crate::client::load_balancing::test_utils::TestWorkScheduler;
    use crate::client::name_resolution::Endpoint;
    use crate::rt::default_runtime;

    #[test]
    fn test_child_lb_config_empty_fails() {
        let json = r#"[]"#;
        let res: Result<ChildPolicyConfig, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn test_child_lb_config_unsupported_fails() {
        let json = r#"[{"unsupported_policy": {}}]"#;
        let res: Result<ChildPolicyConfig, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn test_child_config_empty_policy_fails() {
        let json = r#"{
            "config": [],
            "ignoreReresolutionRequests": false
        }"#;
        let res: Result<PriorityChildConfig, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    /// Verifies that a child config without `ignoreReresolutionRequests`
    /// defaults the flag to `false`, and that a child policy without its own
    /// configuration parses into a builder with no config.
    #[test]
    fn test_child_config_defaults() {
        let json = r#"{"config": [{"round_robin": {}}]}"#;
        let cfg: PriorityChildConfig = serde_json::from_str(json).unwrap();

        assert!(!cfg.ignore_reresolution_requests);
        assert_eq!(cfg.config.builder.name(), "round_robin");
    }

    /// Verifies that `ignoreReresolutionRequests` and the policy-specific
    /// configuration of the child policy are parsed.
    #[test]
    fn test_child_config_with_policy_config() {
        let json = r#"{
            "config": [{"pick_first": {"shuffleAddressList": true}}],
            "ignoreReresolutionRequests": true
        }"#;
        let cfg: PriorityChildConfig = serde_json::from_str(json).unwrap();

        assert!(cfg.ignore_reresolution_requests);
        assert_eq!(cfg.config.builder.name(), "pick_first");
        let pf_cfg = cfg
            .config
            .lb_config
            .as_ref()
            .downcast_ref::<PickFirstConfig>()
            .expect("expected a PickFirstConfig");
    }

    /// Verifies that the first supported policy in the list is selected, as
    /// specified in gRFC A24 (Section Service Config Changes).
    #[test]
    fn test_child_config_picks_first_supported_policy() {
        let json = r#"{
            "config": [{"unsupported_policy": {}}, {"round_robin": {}}]
        }"#;
        let cfg: PriorityChildConfig = serde_json::from_str(json).unwrap();

        assert_eq!(cfg.config.builder.name(), "round_robin");
    }

    /// Runs a test using a [`ChildPolicy`] wrapping a `pick_first` child. Sends
    /// a resolver update with a single endpoint, triggers a failure on the
    /// created subchannel, and checks whether `RequestResolution` was forwarded
    /// to the channel controller.
    fn test_pick_first_child_resolution_request(ignore: bool) -> bool {
        let (tx_events, rx_events) = mpsc::channel();
        let mut tcc = TestChannelController {
            tx_events: tx_events.clone(),
        };
        let work_scheduler = Arc::new(TestWorkScheduler { tx_events });
        let rt = default_runtime();

        let mut child_lb = ChildBuilder {}.build(LbPolicyOptions {
            runtime: rt,
            work_scheduler,
        });

        let json = format!(
            r#"{{
              "config": [{{"pick_first": {{"shuffleAddressList": false}}}}],
              "ignoreReresolutionRequests": {ignore}
            }}"#
        );
        let cfg: PriorityChildConfig = serde_json::from_str(&json).unwrap();

        let endpoint = Endpoint {
            addresses: vec![Address {
                address: "127.0.0.1:8000".to_string().into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let update = ResolverUpdate {
            endpoints: Ok(vec![endpoint]),
            ..Default::default()
        };

        child_lb.resolver_update(update, &cfg, &mut tcc).unwrap();

        let mut subchannel = None;
        while let Ok(event) = rx_events.try_recv() {
            if let TestEvent::NewSubchannel(sc) = event {
                subchannel = Some(sc);
                break;
            }
        }
        let subchannel = subchannel.expect("expected NewSubchannel event");

        // Drain any initial events (e.g. connect, update_picker).
        while rx_events.try_recv().is_ok() {}

        // Fail the single subchannel. Since all addresses in pick_first fail,
        // it enters TRANSIENT_FAILURE and requests re-resolution.
        test_utils::schedule_subchannel_update(
            &subchannel,
            SubchannelState::transient_failure("connection refused"),
        );
        let mut work = Vec::new();
        while let Ok(event) = rx_events.try_recv() {
            if let TestEvent::ScheduleWork(data) = event {
                work.push(data);
            }
        }
        for data in work {
            child_lb.work(data, &mut tcc);
        }

        // Check whether RequestResolution was received by the channel.
        let mut requested_resolution = false;
        while let Ok(event) = rx_events.try_recv() {
            if matches!(event, TestEvent::RequestResolution) {
                requested_resolution = true;
            }
        }
        requested_resolution
    }

    /// Verifies that [`ChildPolicy`] (ChildLb) wrapping a `pick_first` child
    /// forwards re-resolution requests when `ignore_reresolution_requests` is
    /// disabled, and suppresses them when enabled (per gRFC A37 / gRFC A56).
    #[tokio::test]
    async fn wrapped_controller_ignore_resolve_now() {
        // When ignore_reresolution_requests is false, pick_first failing its
        // single subchannel triggers a re-resolution request that is detected.
        assert!(
            test_pick_first_child_resolution_request(false),
            "expected pick_first child to request resolution when ignore is false"
        );

        // When ignore_reresolution_requests is true, the re-resolution request
        // is intercepted and suppressed by WrappedController.
        assert!(
            !test_pick_first_child_resolution_request(true),
            "expected pick_first resolution request to be suppressed when ignore is true"
        );
    }
}
