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

//! Implementation of the `weighted_target_experimental` Load Balancing policy.
//!
//! Specifications:
//! - [gRFC A28: xDS Traffic Splitting and Routing](https://github.com/grpc/proposal/blob/master/A28-xds-traffic-splitting-and-routing.md#weighted-target-lb-policy)
//! - [gRFC A56: Priority LB Policy](https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#hierarchical-addresses)
//! - [gRFC A78: gRPC Metrics: WRR, Pick First, and xDS](https://github.com/grpc/proposal/blob/master/A78-grpc-metrics-wrr-pf-xds.md#weighted-target-lb-policy)
//!
//! The Weighted Target LB policy manages a collection of weighted child
//! policies. Picks are distributed across the children in the aggregate state
//! in proportion to their configured weights. Children that report IDLE are
//! asked to exit idle immediately.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use serde::Deserialize;

use crate::StatusCodeError;
use crate::StatusError;
use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::LbConfigJson;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::ParsedLbConfig;
use crate::client::load_balancing::PickOptions;
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::child_manager::ChildManager;
use crate::client::load_balancing::child_manager::ChildUpdate;
use crate::client::load_balancing::endpoint_filtering;
use crate::client::load_balancing::sampling::WeightedSampler;
use crate::client::load_balancing::sampling::WeightedSamplerBuilder;
use crate::client::name_resolution::ResolverUpdate;

static POLICY_NAME: &str = "weighted_target_experimental";

/// Resolver attribute carrying the name of the target a child serves.
///
/// Defined in [gRFC A78](https://github.com/grpc/proposal/blob/master/A78-grpc-metrics-wrr-pf-xds.md#weighted-target-lb-policy).
#[derive(Debug, PartialEq, Eq)]
struct TargetName(Arc<str>);

#[derive(Debug, Deserialize)]
struct TargetConfig {
    weight: u32,
    #[serde(rename = "childPolicy")]
    child_policy: ParsedLbConfig,
}

#[derive(Debug, Deserialize)]
pub(super) struct WeightedTargetConfig {
    targets: BTreeMap<String, TargetConfig>,
}

#[derive(Debug, Default)]
pub(super) struct WeightedTargetBuilder {
    sampler: WeightedSamplerBuilder,
}

impl LbPolicyBuilder for WeightedTargetBuilder {
    type LbPolicy = WeightedTargetPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        WeightedTargetPolicy {
            child_manager: ChildManager::new(options.runtime, options.work_scheduler),
            weights: HashMap::new(),
            sampler: self.sampler.clone(),
        }
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &LbConfigJson) -> Result<WeightedTargetConfig, String> {
        let parsed: WeightedTargetConfig = config
            .convert_to()
            .map_err(|e| format!("{POLICY_NAME}: invalid config: {e}"))?;

        if parsed.targets.is_empty() {
            return Err(format!(
                "{POLICY_NAME}: invalid config: 'targets' must be non-empty"
            ));
        }

        for (target_name, target_cfg) in &parsed.targets {
            if target_cfg.weight == 0 {
                return Err(format!(
                    "{POLICY_NAME}: invalid config: target '{target_name}': weight must be greater than 0"
                ));
            }
        }

        Ok(parsed)
    }
}

#[derive(Debug)]
pub(super) struct WeightedTargetPolicy {
    child_manager: ChildManager<String>,
    weights: HashMap<String, u32>,
    sampler: WeightedSamplerBuilder,
}

impl WeightedTargetPolicy {
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController) {
        // Wake IDLE children now: the picker below excludes them whenever
        // another child is READY or CONNECTING, so no pick would reach them.
        self.child_manager.exit_idle(channel_controller);
        // Reset the flag so the next `work()`/`exit_idle()` republishes only if
        // a child updates its picker after this point.
        let _ = self.child_manager.child_updated();

        let connectivity_state = self.child_manager.aggregate_states();
        let items = self
            .child_manager
            .children()
            .filter(|(_, child)| child.state().connectivity_state == connectivity_state)
            .map(|(target, child)| {
                let weight = *self
                    .weights
                    .get(target)
                    .expect("every child should have a weight");
                (child.state().picker.clone(), weight)
            });

        let sampler = self.sampler.build(items);
        channel_controller.update_picker(LbState {
            connectivity_state,
            picker: Arc::new(WeightedPicker { sampler }),
        });
    }
}

impl LbPolicy for WeightedTargetPolicy {
    type LbConfig = WeightedTargetConfig;

    fn resolver_update(
        &mut self,
        mut update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let mut grouped_endpoints = std::mem::replace(&mut update.endpoints, Ok(Vec::new()))
            .map(endpoint_filtering::group_by_path);

        let child_updates = config.targets.iter().map(|(target_name, target_config)| {
            let endpoints = match &mut grouped_endpoints {
                Ok(grouped) => Ok(grouped.remove(target_name).unwrap_or_default()),
                Err(err) => Err(err.clone()),
            };
            let mut child_update = update.with_endpoints(endpoints);
            child_update.attributes = child_update
                .attributes
                .add(TargetName(Arc::from(target_name.as_str())));

            ChildUpdate {
                child_identifier: target_name.clone(),
                child_policy_builder: target_config.child_policy.builder.clone(),
                child_update: Some((child_update, &target_config.child_policy.config)),
            }
        });

        let result = self.child_manager.update(child_updates, channel_controller);

        self.weights = config
            .targets
            .iter()
            .map(|(name, cfg)| (name.clone(), cfg.weight))
            .collect();

        // Always publish: weight changes and removed targets alter the picker
        // even if no child reported a new state.
        self.update_picker(channel_controller);
        result.map_err(|e| format!("{POLICY_NAME}: failed to update children: {e}"))
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        self.child_manager.work(data, channel_controller);
        if self.child_manager.child_updated() {
            self.update_picker(channel_controller);
        }
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager.exit_idle(channel_controller);
        if self.child_manager.child_updated() {
            self.update_picker(channel_controller);
        }
    }
}

/// Picker that delegates each pick to a child picker selected by [`WeightedSampler`].
#[derive(Debug)]
struct WeightedPicker {
    sampler: WeightedSampler<Arc<dyn Picker>>,
}

impl Picker for WeightedPicker {
    fn pick(&self, options: PickOptions<'_>) -> PickResult {
        let Some(child_picker) = self.sampler.next() else {
            debug_assert!(
                false,
                "WeightedPicker constructed with empty pickers or zero total_weight"
            );
            return PickResult::Drop(StatusError::new(
                StatusCodeError::Internal,
                format!("{POLICY_NAME}: picker has no weighted children"),
            ));
        };

        child_picker.pick(options)
    }
}

/// Register weighted target as an LbPolicy.
pub(crate) fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(WeightedTargetBuilder::default());
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use serde_json::json;

    use super::*;
    use crate::attributes::Attributes;
    use crate::client::CallOptions;
    use crate::client::ConnectivityState;
    use crate::client::load_balancing::FixedResultPicker;
    use crate::client::load_balancing::Pick;
    use crate::client::load_balancing::endpoint_filtering::set_path_in_endpoint;
    use crate::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use crate::client::load_balancing::sampling::test_utils::sampler_builder;
    use crate::client::load_balancing::subchannel::Subchannel;
    use crate::client::load_balancing::subchannel::SubchannelState;
    use crate::client::load_balancing::subchannel::SubchannelUpdate;
    use crate::client::load_balancing::test_utils;
    use crate::client::load_balancing::test_utils::StubPolicyFuncs;
    use crate::client::load_balancing::test_utils::TestEnv;
    use crate::client::name_resolution::Endpoint;
    use crate::core::Address;
    use crate::metadata::MetadataMap;
    use crate::rt::default_runtime;

    struct StubState {
        subchannel: Arc<dyn Subchannel>,
        state: ConnectivityState,
    }

    /// Registers a stub child LB policy named `policy_name`.
    ///
    /// On its first resolver update, the stub creates a subchannel whose
    /// address is the `TargetName` attribute injected by `weighted_target`. On
    /// every subchannel update, it reports that subchannel's connectivity state
    /// with a state-appropriate `FixedResultPicker` (`PickResult::Pick` when
    /// `Ready`, `PickResult::Fail` when `TransientFailure`, `PickResult::Queue`
    /// otherwise). When `exit_idle` is called while in `Idle`, it calls
    /// `connect()` on its subchannel and transitions to `Connecting`.
    fn reg_stub(policy_name: &'static str) {
        let funcs = StubPolicyFuncs {
            resolver_update: Some(Arc::new(|data, update, _cfg, controller| {
                let TargetName(target) = update
                    .attributes
                    .get::<TargetName>()
                    .expect("parent should inject TargetName");
                if data.test_data.is_none() {
                    let address = Address {
                        address: target.to_string().into(),
                        ..Default::default()
                    };
                    let (subchannel, _) = controller
                        .new_subchannel(&address, data.lb_policy_options.work_scheduler.clone());
                    data.test_data = Some(Box::new(StubState {
                        subchannel,
                        state: ConnectivityState::Connecting,
                    }));
                }
                Ok(())
            })),
            work: Some(Arc::new(|data, work_data, controller| {
                let update = work_data
                    .expect("expected work data")
                    .downcast::<SubchannelUpdate>()
                    .expect("expected SubchannelUpdate");
                let stub = data
                    .test_data
                    .as_mut()
                    .and_then(|d| d.downcast_mut::<StubState>())
                    .expect("stub state should be initialized");
                let state = update.state.connectivity_state;
                stub.state = state;
                let picker: Arc<dyn Picker> = Arc::new(FixedResultPicker(match state {
                    ConnectivityState::Ready => PickResult::Pick(Pick {
                        subchannel: update.subchannel,
                        metadata: MetadataMap::new(),
                        on_complete: None,
                    }),
                    ConnectivityState::TransientFailure => PickResult::Fail(StatusError::new(
                        StatusCodeError::Unavailable,
                        update.state.last_connection_error.unwrap_or_default(),
                    )),
                    ConnectivityState::Connecting | ConnectivityState::Idle => PickResult::Queue,
                }));
                controller.update_picker(LbState {
                    connectivity_state: state,
                    picker,
                });
            })),
            exit_idle: Some(Arc::new(|data, controller| {
                let stub = data
                    .test_data
                    .as_mut()
                    .and_then(|d| d.downcast_mut::<StubState>())
                    .expect("stub state should be initialized");
                if stub.state == ConnectivityState::Idle {
                    stub.state = ConnectivityState::Connecting;
                    stub.subchannel.connect();
                    controller.update_picker(LbState::initial());
                }
            })),
        };
        test_utils::reg_stub_policy(policy_name, funcs);
    }

    /// Registers a stub child policy that records the `(ResolverUpdate, config)`
    /// received by each target, keyed by the injected `TargetName` attribute.
    fn reg_recording_stub(
        policy_name: &'static str,
    ) -> Arc<Mutex<HashMap<String, (ResolverUpdate, String)>>> {
        let received: Arc<Mutex<HashMap<String, (ResolverUpdate, String)>>> = Arc::default();
        let received_clone = received.clone();
        test_utils::reg_stub_policy(
            policy_name,
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(move |_data, update, config, _controller| {
                    let TargetName(target) = update
                        .attributes
                        .get::<TargetName>()
                        .expect("parent should inject TargetName");
                    let config = config
                        .downcast_ref::<String>()
                        .expect("child config should be a String")
                        .clone();
                    received_clone
                        .lock()
                        .unwrap()
                        .insert(target.to_string(), (update, config));
                    Ok(())
                })),
                ..Default::default()
            },
        );
        received
    }

    fn new_env() -> TestEnv<WeightedTargetPolicy> {
        TestEnv::new(|work_scheduler| {
            WeightedTargetBuilder::default().build(LbPolicyOptions {
                work_scheduler,
                runtime: default_runtime(),
            })
        })
    }

    /// Creates a `TestEnv` for a `WeightedTargetPolicy` whose sampler draws
    /// `keys` in order when selecting among multiple children with distinct
    /// weights.
    ///
    /// A key selects the child whose range of cumulative weight, in target-name
    /// order, contains it: with weights 30/70, keys 0..30 select the first
    /// child and keys 30..100 the second.
    fn new_env_with_keys(keys: &[u64]) -> TestEnv<WeightedTargetPolicy> {
        TestEnv::new(|work_scheduler| {
            WeightedTargetBuilder {
                sampler: sampler_builder(keys),
            }
            .build(LbPolicyOptions {
                work_scheduler,
                runtime: default_runtime(),
            })
        })
    }

    impl TestEnv<WeightedTargetPolicy> {
        /// Sends a config with one `policy_name` child per `(target, weight)`
        /// and no endpoints. Checks that each new child creates its subchannel
        /// and that the parent publishes an initial `Connecting` picker, and
        /// returns the subchannels in `targets` order. `targets` must be in
        /// name order, which is the order in which the parent creates children.
        fn apply<const N: usize>(
            &mut self,
            policy_name: &str,
            targets: [(&str, u32); N],
        ) -> [Arc<dyn Subchannel>; N] {
            self.send_resolver_update_with_config(Vec::new(), &config(policy_name, &targets))
                .expect("resolver update should succeed");
            let subchannels = targets.map(|(target, _)| self.expect_new_subchannel_for(target));
            self.expect_picker_state(ConnectivityState::Connecting);
            self.expect_no_events();
            subchannels
        }
    }

    /// Returns a `WeightedTargetConfig` with one `policy_name` child per
    /// `(target, weight)` whose child config is `"<target> config"`.
    fn config(policy_name: &str, targets: &[(&str, u32)]) -> WeightedTargetConfig {
        let builder = GLOBAL_LB_REGISTRY
            .get_policy(policy_name)
            .expect("stub should be registered");
        let targets = targets
            .iter()
            .map(|&(target, weight)| {
                let child_policy = ParsedLbConfig {
                    builder: builder.clone(),
                    config: Arc::new(format!("{target} config")),
                };
                (
                    target.to_string(),
                    TargetConfig {
                        weight,
                        child_policy,
                    },
                )
            })
            .collect();
        WeightedTargetConfig { targets }
    }

    /// Asserts that a pick returns `subchannel`.
    fn assert_picks(picker: &dyn Picker, subchannel: &Arc<dyn Subchannel>) {
        let pick = picker
            .pick(PickOptions::new(
                &test_utils::new_request_headers(),
                &CallOptions::new(),
            ))
            .unwrap_pick();
        assert_eq!(&pick.subchannel, subchannel);
    }

    fn endpoint(path: &[&str]) -> Endpoint {
        set_path_in_endpoint(
            Endpoint::default(),
            path.iter().map(|p| p.to_string()).collect(),
        )
    }

    #[test]
    fn parse_config() {
        let parse = |json: serde_json::Value| {
            WeightedTargetBuilder::default()
                .parse_config(&LbConfigJson::new(&json.to_string()).unwrap())
        };
        let rr = json!([{ RR_POLICY_NAME: {} }]);

        let cfg = parse(json!({ "targets": {
            "t1": { "weight": 10, "childPolicy": rr },
            "t2": { "weight": 90, "childPolicy": rr }
        }}))
        .expect("valid config should parse");
        let targets: Vec<_> = cfg
            .targets
            .iter()
            .map(|(name, target)| {
                (
                    name.as_str(),
                    target.weight,
                    target.child_policy.builder.name(),
                )
            })
            .collect();
        assert_eq!(
            targets,
            [("t1", 10, RR_POLICY_NAME), ("t2", 90, RR_POLICY_NAME)]
        );

        let err = |json| parse(json).expect_err("invalid config should fail to parse");
        let missing = err(json!({}));
        assert!(
            missing.starts_with("weighted_target_experimental: invalid config: ")
                && missing.contains("targets"),
            "unexpected error: {missing}"
        );
        assert_eq!(
            err(json!({ "targets": {} })),
            "weighted_target_experimental: invalid config: 'targets' must be non-empty"
        );
        assert_eq!(
            err(json!({ "targets": { "t1": { "weight": 0, "childPolicy": rr } } })),
            "weighted_target_experimental: invalid config: target 't1': weight must be greater than 0"
        );
    }

    #[test]
    fn resolver_update_routes_endpoints_and_config_by_target() {
        let received = reg_recording_stub("stub_wt_routes");
        let mut t = new_env();
        let endpoints = vec![
            endpoint(&["a", "a1"]),
            endpoint(&["b", "b1"]),
            endpoint(&["x", "x1"]),
        ];
        t.send_resolver_update_with_config(
            endpoints,
            &config("stub_wt_routes", &[("a", 30), ("b", 30), ("c", 40)]),
        )
        .unwrap();
        t.expect_queuing_picker();
        t.expect_no_events();

        // Stub children record under the injected `TargetName`, so these
        // checks also verify that each child's `TargetName` matches its target.
        // Each child gets only the endpoints under its target, with that path
        // element removed: "c" has none, and "x" is not a target, so its
        // endpoint goes nowhere.
        let received = received.lock().unwrap();
        assert_eq!(received.len(), 3);
        assert_eq!(received["a"].0.endpoints, Ok(vec![endpoint(&["a1"])]));
        assert_eq!(received["b"].0.endpoints, Ok(vec![endpoint(&["b1"])]));
        assert_eq!(received["c"].0.endpoints, Ok(vec![]));
        for target in ["a", "b", "c"] {
            assert_eq!(received[target].1, format!("{target} config"));
        }
    }

    #[test]
    fn resolver_update_forwards_attributes_errors_and_note_to_children() {
        let received = reg_recording_stub("stub_wt_forwards");
        let mut t = new_env();
        let update = ResolverUpdate {
            attributes: Attributes::new().add("parent attribute"),
            endpoints: Err("endpoints error".to_string()),
            service_config: Err("service config error".to_string()),
            resolution_note: Some("note".to_string()),
        };
        t.policy
            .resolver_update(
                update,
                &config("stub_wt_forwards", &[("a", 30), ("b", 70)]),
                &mut t.tcc,
            )
            .unwrap();
        t.expect_queuing_picker();
        t.expect_no_events();

        let received = received.lock().unwrap();
        for target in ["a", "b"] {
            let (update, _) = &received[target];
            assert_eq!(update.attributes.get(), Some(&"parent attribute"));
            assert_eq!(update.endpoints, Err("endpoints error".to_string()));
            assert_eq!(
                update.service_config.as_ref().unwrap_err(),
                "service config error"
            );
            assert_eq!(update.resolution_note.as_deref(), Some("note"));
        }
    }

    #[test]
    fn child_error_still_publishes_picker() {
        test_utils::reg_stub_policy(
            "stub_wt_child_error",
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(|_data, _update, _cfg, _controller| {
                    Err("mock failure".to_string())
                })),
                ..Default::default()
            },
        );
        let mut t = new_env();
        let err = t
            .send_resolver_update_with_config(
                Vec::new(),
                &config("stub_wt_child_error", &[("a", 30), ("b", 70)]),
            )
            .unwrap_err();
        assert!(
            err.starts_with("weighted_target_experimental: failed to update children: ")
                && err.contains("mock failure"),
            "unexpected error: {err}"
        );

        t.expect_queuing_picker();
        t.expect_no_events();
    }

    #[test]
    fn config_update_republishes_picker() {
        reg_stub("stub_wt_config_update");
        let mut t = new_env_with_keys(&[29, 30, 79, 80, 0]);
        let [a, b] = t.apply("stub_wt_config_update", [("a", 30), ("b", 70)]);
        t.send_subchannel_update(&a, &SubchannelState::ready());
        t.expect_picker_state(ConnectivityState::Ready);
        t.send_subchannel_update(&b, &SubchannelState::ready());
        let picker = t.expect_picker_state(ConnectivityState::Ready);
        assert_picks(&*picker, &a);
        assert_picks(&*picker, &b);

        // The children's states are unchanged in the next two updates, so
        // only the new weights and the removal of "a" change the picker.
        t.send_resolver_update_with_config(
            Vec::new(),
            &config("stub_wt_config_update", &[("a", 80), ("b", 20)]),
        )
        .unwrap();
        let picker = t.expect_picker_state(ConnectivityState::Ready);
        assert_picks(&*picker, &a);
        assert_picks(&*picker, &b);

        t.send_resolver_update_with_config(
            Vec::new(),
            &config("stub_wt_config_update", &[("b", 20)]),
        )
        .unwrap();
        t.expect_ready_picker(&b);
        t.expect_no_events();
    }

    #[test]
    fn ready_picker_excludes_non_ready_children() {
        reg_stub("stub_wt_ready_excludes");
        // With "a" (10) and "b" (20) excluded, "c" (30) and "d" (70) map keys
        // 0..30 to "c" and 30..100 to "d".
        let mut t = new_env_with_keys(&[29, 30]);
        let [_, b, c, d] = t.apply(
            "stub_wt_ready_excludes",
            [("a", 10), ("b", 20), ("c", 30), ("d", 70)],
        );
        t.send_subchannel_update(&b, &SubchannelState::transient_failure("err"));
        t.expect_picker_state(ConnectivityState::Connecting);
        t.send_subchannel_update(&c, &SubchannelState::ready());
        t.expect_picker_state(ConnectivityState::Ready);
        t.send_subchannel_update(&d, &SubchannelState::ready());
        let picker = t.expect_picker_state(ConnectivityState::Ready);
        t.expect_no_events();
        assert_picks(&*picker, &c);
        assert_picks(&*picker, &d);
    }

    #[test]
    fn transient_failure_picker_spans_failing_children() {
        reg_stub("stub_wt_transient_failure");
        let mut t = new_env_with_keys(&[29, 30]);
        let [a, b] = t.apply("stub_wt_transient_failure", [("a", 30), ("b", 70)]);
        t.send_subchannel_update(&a, &SubchannelState::transient_failure("err a"));
        t.expect_picker_state(ConnectivityState::Connecting);
        t.send_subchannel_update(&b, &SubchannelState::transient_failure("err b"));
        let picker = t.expect_failing_picker("err a");
        t.expect_no_events();
        let PickResult::Fail(status) = picker.pick(PickOptions::new(
            &test_utils::new_request_headers(),
            &CallOptions::new(),
        )) else {
            panic!("expected Fail result");
        };
        assert_eq!(status.code(), StatusCodeError::Unavailable);
        assert_eq!(status.message(), "err b");
    }

    #[test]
    fn connecting_picker_excludes_non_connecting_children() {
        reg_stub("stub_wt_connecting_excludes");
        // Key 0 would select "a" if non-connecting children are not excluded.
        let mut t = new_env_with_keys(&[0]);
        let [a, _] = t.apply("stub_wt_connecting_excludes", [("a", 30), ("b", 70)]);
        t.send_subchannel_update(&a, &SubchannelState::transient_failure("err"));
        t.expect_queuing_picker();
        t.expect_no_events();
    }

    #[test]
    fn child_dropping_to_idle_while_others_ready_is_asked_to_exit_idle() {
        reg_stub("stub_wt_child_dropping_to_idle");
        let mut t = new_env();
        let [a, b] = t.apply("stub_wt_child_dropping_to_idle", [("a", 50), ("b", 50)]);
        t.send_subchannel_update(&a, &SubchannelState::ready());
        t.expect_ready_picker(&a);
        t.send_subchannel_update(&b, &SubchannelState::ready());
        t.expect_picker_state(ConnectivityState::Ready);

        // The READY picker excludes "b" once it is IDLE, so no pick would wake
        // it; the parent has to ask it to exit idle.
        t.send_subchannel_update(&b, &SubchannelState::idle());
        assert_eq!(&*t.expect_connect().address, "b");
        t.expect_ready_picker(&a);
        t.expect_no_events();
    }

    #[test]
    fn all_children_dropping_to_idle_are_asked_to_exit_idle() {
        reg_stub("stub_wt_all_children_dropping_to_idle");
        let mut t = new_env();
        let [a] = t.apply("stub_wt_all_children_dropping_to_idle", [("a", 100)]);
        t.send_subchannel_update(&a, &SubchannelState::ready());
        t.expect_ready_picker(&a);

        // "a" drops to IDLE; `update_picker` calls `exit_idle` before
        // aggregating, so "a" connects and reports CONNECTING, and the parent
        // never publishes IDLE.
        t.send_subchannel_update(&a, &SubchannelState::idle());
        assert_eq!(&*t.expect_connect().address, "a");
        t.expect_queuing_picker();
        t.expect_no_events();
    }

    #[test]
    fn exit_idle_forwards_and_republishes_child_updates() {
        let ignore_first = Mutex::new(true);
        test_utils::reg_stub_policy(
            "stub_wt_exit_idle_republishes",
            StubPolicyFuncs {
                exit_idle: Some(Arc::new(move |_data, controller| {
                    // Ignore the `exit_idle` call triggered by `resolver_update`. The first call will be
                    // when the child is in CONNECTING (during setup), and the leaf node should ignore it.
                    if std::mem::replace(&mut *ignore_first.lock().unwrap(), false) {
                        return;
                    }
                    controller.update_picker(LbState {
                        connectivity_state: ConnectivityState::TransientFailure,
                        picker: Arc::new(FixedResultPicker(PickResult::Fail(StatusError::new(
                            StatusCodeError::Unavailable,
                            "woken",
                        )))),
                    });
                })),
                ..Default::default()
            },
        );
        let mut t = new_env();
        t.send_resolver_update_with_config(
            Vec::new(),
            &config("stub_wt_exit_idle_republishes", &[("a", 100)]),
        )
        .unwrap();
        t.expect_queuing_picker();
        t.expect_no_events();

        t.policy.exit_idle(&mut t.tcc);
        t.expect_failing_picker("woken");
        t.expect_no_events();
    }

    #[test]
    fn work_and_exit_idle_without_child_updates_do_not_republish() {
        test_utils::reg_stub_policy(
            "stub_wt_no_child_update",
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(|data, _update, _cfg, controller| {
                    controller.new_subchannel(
                        &Address::default(),
                        data.lb_policy_options.work_scheduler.clone(),
                    );
                    Ok(())
                })),
                ..Default::default()
            },
        );
        let mut t = new_env();
        t.send_resolver_update_with_config(
            Vec::new(),
            &config("stub_wt_no_child_update", &[("a", 100)]),
        )
        .unwrap();
        let sc = t.expect_new_subchannel();
        t.expect_queuing_picker();
        t.expect_no_events();

        t.send_subchannel_update(&sc, &SubchannelState::ready());
        t.policy.exit_idle(&mut t.tcc);
        t.expect_no_events();
    }
}
