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

//! Implementation of the `xds_wrr_locality_experimental` Load Balancing policy.
//!
//! Used in xDS to distribute RPCs across localities using weighted round-robin
//! before delegating endpoint selection within the chosen locality to a
//! configured `childPolicy` (such as `round_robin`).
//!
//! Because xDS provides locality weights dynamically via EDS endpoint
//! attributes rather than static service config, this policy extracts those
//! per-locality weights from each resolver update and builds a
//! `weighted_target_experimental` child policy configuration.
//!
//! Specifications:
//! - [gRFC A52: xDS Custom LB Policies](https://github.com/grpc/proposal/blob/master/A52-xds-custom-lb-policies.md)
//! - [gRFC A28: xDS Traffic Splitting and Routing](https://github.com/grpc/proposal/blob/master/A28-xds-traffic-splitting-and-routing.md#weighted-target-lb-policy)
//! - [gRFC A78: gRPC Metrics: WRR, Pick First, and xDS](https://github.com/grpc/proposal/blob/master/A78-grpc-metrics-wrr-pf-xds.md#optional-xds-locality-label)

use std::sync::Arc;

use grpc::__unstable::client::load_balancing::ChannelController;
use grpc::__unstable::client::load_balancing::FailingPicker;
use grpc::__unstable::client::load_balancing::LbConfigJson;
use grpc::__unstable::client::load_balancing::LbPolicy;
use grpc::__unstable::client::load_balancing::LbPolicyBuilder;
use grpc::__unstable::client::load_balancing::LbPolicyOptions;
use grpc::__unstable::client::load_balancing::LbState;
use grpc::__unstable::client::load_balancing::ParsedLbConfig;
use grpc::__unstable::client::load_balancing::WorkData;
use grpc::__unstable::client::load_balancing::WorkScheduler;
use grpc::__unstable::client::load_balancing::child_manager::Child;
use grpc::__unstable::client::load_balancing::registry::GLOBAL_LB_REGISTRY;
use grpc::__unstable::client::name_resolution::Endpoint;
use grpc::__unstable::client::name_resolution::ResolverUpdate;
use grpc::__unstable::rt::GrpcRuntime;
use grpc::client::ConnectivityState;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

use crate::resource::Locality;

static POLICY_NAME: &str = "xds_wrr_locality_experimental";
static CHILD_POLICY_NAME: &str = "weighted_target_experimental";

#[derive(Debug, PartialEq, Eq)]
struct LocalityWeight(u32);

/// Sets the xDS locality load-balancing weight on `endpoint`.
#[allow(dead_code, reason = "no caller until there is an xDS resolver.")]
pub(crate) fn set_locality_weight(mut endpoint: Endpoint, weight: u32) -> Endpoint {
    endpoint.attributes = endpoint.attributes.add(LocalityWeight(weight));
    endpoint
}

#[derive(Debug, Deserialize)]
struct WrrLocalityConfig {
    /// Prototype LB policy configuration for picking endpoints within a
    /// locality. Kept as raw JSON so it can be replicated into each locality
    /// entry when constructing the `weighted_target_experimental` config on
    /// resolver updates.
    #[serde(rename = "childPolicy")]
    child_policy: Vec<Value>,
}

#[derive(Debug)]
struct WrrLocalityBuilder;

impl LbPolicyBuilder for WrrLocalityBuilder {
    type LbPolicy = WrrLocalityPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        WrrLocalityPolicy {
            child: None,
            runtime: options.runtime,
            work_scheduler: options.work_scheduler,
            child_policy_name: CHILD_POLICY_NAME,
        }
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &LbConfigJson) -> Result<WrrLocalityConfig, String> {
        let parsed: WrrLocalityConfig = config
            .convert_to()
            .map_err(|e| format!("{POLICY_NAME}: invalid config: {e}"))?;

        serde_json::from_value::<ParsedLbConfig>(Value::Array(parsed.child_policy.clone()))
            .map_err(|e| format!("{POLICY_NAME}: invalid childPolicy: {e}"))?;

        Ok(parsed)
    }
}

#[derive(Debug)]
struct WrrLocalityPolicy {
    child: Option<(Child, ParsedLbConfig)>,
    runtime: GrpcRuntime,
    work_scheduler: Arc<dyn WorkScheduler>,
    child_policy_name: &'static str,
}

impl WrrLocalityPolicy {
    /// Drops any active child policy and reports TRANSIENT_FAILURE with a
    /// picker that fails RPCs with `error`, followed by the resolver's note if
    /// there is one. Returns the error the picker reports.
    fn fail_update(
        &mut self,
        error: String,
        resolution_note: Option<&str>,
        channel_controller: &mut dyn ChannelController,
    ) -> String {
        let error = match resolution_note {
            Some(note) if !note.is_empty() => format!("{error} ({note})"),
            _ => error,
        };
        self.child = None;
        channel_controller.update_picker(LbState {
            connectivity_state: ConnectivityState::TransientFailure,
            picker: Arc::new(FailingPicker {
                error: error.clone(),
            }),
        });
        error
    }

    /// Builds and parses the child LB policy configuration from per-locality
    /// endpoint weights. Per gRFC A52, if a locality appears with conflicting
    /// weights across endpoints, the first weight encountered is used.
    fn parse_child_config(
        &self,
        endpoints: &[Endpoint],
        child_policy: &[Value],
    ) -> Result<ParsedLbConfig, String> {
        let mut targets = serde_json::Map::new();
        for (locality, weight) in endpoints.iter().filter_map(|endpoint| {
            let locality = endpoint.attributes.get::<Locality>()?;
            let &LocalityWeight(weight) = endpoint.attributes.get::<LocalityWeight>()?;
            (weight > 0).then_some((locality, weight))
        }) {
            targets.entry(locality.to_string()).or_insert_with(|| {
                json!({
                    "weight": weight,
                    "childPolicy": child_policy,
                })
            });
        }

        let child_policy_json = json!([{
            self.child_policy_name: {
                "targets": targets,
            }
        }]);
        serde_json::from_value(child_policy_json)
            .map_err(|err| format!("{POLICY_NAME}: failed to parse generated child config: {err}"))
    }

    /// Returns the child policy and its config, creating the child on first
    /// use. `child_config` always replaces the stored config, even when the
    /// child already exists.
    fn get_or_insert_child(
        &mut self,
        child_config: ParsedLbConfig,
    ) -> &mut (Child, ParsedLbConfig) {
        let child = match self.child.take() {
            Some((child, _old_config)) => child,
            None => Child::new(
                &*child_config.builder,
                LbPolicyOptions {
                    work_scheduler: self.work_scheduler.clone(),
                    runtime: self.runtime.clone(),
                },
            ),
        };
        self.child.insert((child, child_config))
    }

    /// Use whenever forwarding a resolver update to an active child so that any
    /// picker intercepted by `Child` is published to the channel.
    fn update_child(
        child: &mut Child,
        config: &ParsedLbConfig,
        update: ResolverUpdate,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let result = child.resolver_update(update, &config.config, channel_controller);
        if child.take_updated() {
            channel_controller.update_picker(child.state().clone());
        }
        result.map_err(|e| format!("{POLICY_NAME}: failed to update child policy: {e}"))
    }
}

impl LbPolicy for WrrLocalityPolicy {
    type LbConfig = WrrLocalityConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let endpoints = match &update.endpoints {
            Ok(endpoints) => endpoints,
            Err(err) => {
                let Some((child, last_config)) = &mut self.child else {
                    return Err(self.fail_update(
                        format!("{POLICY_NAME}: resolver error: {err}"),
                        update.resolution_note.as_deref(),
                        channel_controller,
                    ));
                };

                return Self::update_child(child, last_config, update, channel_controller);
            }
        };

        let child_config = match self.parse_child_config(endpoints, &config.child_policy) {
            Ok(parsed) => parsed,
            Err(err) => {
                return Err(self.fail_update(
                    err,
                    update.resolution_note.as_deref(),
                    channel_controller,
                ));
            }
        };
        let (child, last_config) = self.get_or_insert_child(child_config);
        Self::update_child(child, last_config, update, channel_controller)
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        let Some(data) = data else {
            debug_assert!(false, "{POLICY_NAME}: work called with None value");
            return;
        };
        let Some((child, _)) = &mut self.child else {
            return;
        };
        if child.try_work(data, channel_controller).is_ok() && child.take_updated() {
            channel_controller.update_picker(child.state().clone());
        }
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        let Some((child, _)) = &mut self.child else {
            return;
        };
        child.exit_idle(channel_controller);
        if child.take_updated() {
            channel_controller.update_picker(child.state().clone());
        }
    }
}

/// Register WRR locality as an LbPolicy.
pub(crate) fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(WrrLocalityBuilder);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    use grpc::__unstable::client::load_balancing::QueuingPicker;
    use grpc::__unstable::client::load_balancing::endpoint_filtering::set_path_in_endpoint;
    use grpc::__unstable::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use grpc::__unstable::client::load_balancing::subchannel::SubchannelState;
    use grpc::__unstable::client::load_balancing::test_utils;
    use grpc::__unstable::client::load_balancing::test_utils::StubPolicyFuncs;
    use grpc::__unstable::client::load_balancing::test_utils::TestEnv;
    use grpc::__unstable::rt::default_runtime;
    use grpc::core::Address;

    use super::*;

    #[test]
    fn parse_config() {
        reg();
        let top_level = serde_json::from_value::<ParsedLbConfig>(json!([{
            POLICY_NAME: {
                "childPolicy": [{ RR_POLICY_NAME: {} }]
            }
        }]))
        .expect("registered policy should parse through GLOBAL_LB_REGISTRY");
        assert_eq!(top_level.builder.name(), "xds_wrr_locality_experimental");

        let builder = WrrLocalityBuilder;
        assert_eq!(builder.name(), "xds_wrr_locality_experimental");

        let parse =
            |json: Value| builder.parse_config(&LbConfigJson::new(&json.to_string()).unwrap());

        let valid = parse(json!({
            "childPolicy": [{ RR_POLICY_NAME: {} }]
        }))
        .expect("valid config should parse");
        assert_eq!(valid.child_policy, vec![json!({ RR_POLICY_NAME: {} })]);

        let fallback = parse(json!({
            "childPolicy": [
                { "unregistered_policy_name": {} },
                { RR_POLICY_NAME: {} }
            ]
        }))
        .expect("fallback config with a supported policy should parse");
        assert_eq!(fallback.child_policy.len(), 2);

        let err = |json| parse(json).expect_err("invalid config should fail to parse");
        assert!(err(json!({})).contains("childPolicy"));
        assert!(err(json!({ "childPolicy": null })).contains("invalid config"));
        assert!(err(json!({ "childPolicy": {} })).contains("invalid config"));
        assert!(err(json!({ "childPolicy": [] })).contains("must not be empty"));
        assert!(err(json!({ "childPolicy": [{ "": {} }] })).contains("No supported"));
        assert!(
            err(json!({ "childPolicy": [{ "unregistered_policy_name": {} }] }))
                .contains("No supported")
        );
        assert!(
            err(json!({
                "childPolicy": [{ "pick_first": { "shuffleAddressList": "not_a_bool" } }]
            }))
            .contains("invalid childPolicy")
        );
    }

    #[test]
    fn resolver_update_builds_weighted_targets_and_forwards_update() {
        let (mut t, received) = new_recording_env("mock_wrr_builds_targets");
        let endpoints = vec![
            locality_endpoint("region-1", "zone-1", "subzone-1", 20),
            locality_endpoint("region-2", "zone-2", "subzone-2", 80),
        ];
        let mut update = ResolverUpdate::default();
        update.attributes = update.attributes.add("custom_attr");
        update.endpoints = Ok(endpoints.clone());
        update.resolution_note = Some("eds_note".to_string());

        t.policy
            .resolver_update(update, &WrrLocalityConfig::default(), &mut t.tcc)
            .expect("resolver update should succeed");
        t.expect_queuing_picker();
        t.expect_no_events();

        let (received_update, targets) = received.lock().unwrap().take().unwrap();
        assert_eq!(
            targets,
            BTreeMap::from([
                (
                    "{region=\"region-1\", zone=\"zone-1\", sub_zone=\"subzone-1\"}".to_string(),
                    20,
                ),
                (
                    "{region=\"region-2\", zone=\"zone-2\", sub_zone=\"subzone-2\"}".to_string(),
                    80,
                ),
            ])
        );
        assert_eq!(received_update.endpoints, Ok(endpoints));
        assert_eq!(received_update.attributes.get(), Some(&"custom_attr"));
        assert_eq!(received_update.resolution_note.as_deref(), Some("eds_note"));
    }

    #[test]
    fn conflicting_weights_for_same_locality_uses_first_weight() {
        let endpoints = vec![
            locality_endpoint("r1", "z1", "sz1", 35),
            locality_endpoint("r1", "z1", "sz1", 35),
            locality_endpoint("r1", "z1", "sz1", 90),
        ];

        assert_eq!(
            targets_from_update("mock_wrr_conflicting_weights", endpoints),
            BTreeMap::from([(
                "{region=\"r1\", zone=\"z1\", sub_zone=\"sz1\"}".to_string(),
                35,
            )])
        );
    }

    #[test]
    fn endpoints_without_locality_or_positive_weight_are_skipped() {
        let mut missing_weight = Endpoint::default();
        missing_weight.attributes = missing_weight.attributes.add(Locality {
            region: "r1".to_string(),
            zone: "z1".to_string(),
            sub_zone: "sz1".to_string(),
        });
        let missing_locality = set_locality_weight(Endpoint::default(), 10);

        let endpoints = vec![
            missing_weight,
            missing_locality,
            locality_endpoint("r2", "z2", "sz2", 0),
            locality_endpoint("r3", "z3", "sz3", 50),
        ];

        assert_eq!(
            targets_from_update("mock_wrr_skips_unweighted", endpoints),
            BTreeMap::from([(
                "{region=\"r3\", zone=\"z3\", sub_zone=\"sz3\"}".to_string(),
                50,
            )])
        );
    }

    #[test]
    fn dynamic_locality_updates_refresh_targets() {
        let (mut t, received) = new_recording_env("mock_wrr_dynamic_updates");

        t.send_resolver_update(vec![
            locality_endpoint("r1", "z1", "sz1", 10),
            locality_endpoint("r2", "z2", "sz2", 20),
        ])
        .expect("initial update should succeed");
        t.expect_queuing_picker();
        assert_eq!(received.lock().unwrap().take().unwrap().1.len(), 2);

        t.send_resolver_update(vec![locality_endpoint("r2", "z2", "sz2", 75)])
            .expect("second update should succeed");
        t.expect_queuing_picker();
        t.expect_no_events();
        assert_eq!(
            received.lock().unwrap().take().unwrap().1,
            BTreeMap::from([(
                "{region=\"r2\", zone=\"z2\", sub_zone=\"sz2\"}".to_string(),
                75,
            )])
        );
    }

    #[tokio::test]
    async fn weighted_locality_endpoints_become_ready_through_child_policies() {
        let mut t = new_env();
        let mut endpoint = locality_endpoint("r1", "z1", "sz1", 10);
        let mut address = Address::default();
        address.address = "addr1".to_string().into();
        endpoint.addresses = vec![address];
        // weighted_target gives each endpoint to the target named by the first
        // element of its hierarchical path.
        let endpoint = set_path_in_endpoint(
            endpoint,
            vec!["{region=\"r1\", zone=\"z1\", sub_zone=\"sz1\"}".to_string()],
        );

        t.send_resolver_update(vec![endpoint])
            .expect("resolver update should succeed");
        let subchannel = t.expect_new_subchannel_for("addr1");
        t.expect_connect_to("addr1");
        t.expect_queuing_picker();

        t.send_subchannel_update(&subchannel, &SubchannelState::ready());
        t.expect_ready_picker(&subchannel);
        t.expect_no_events();
    }

    #[test]
    fn empty_or_unweighted_endpoints_fail_picks_in_child() {
        let mut t = new_env();

        // An endpoint without a locality weight leaves no targets. The update
        // succeeds and the picker fails with weighted_target's error, so the
        // empty config reached the child instead of failing here. The
        // resolver's note reaches the picker too.
        let mut unweighted = Endpoint::default();
        unweighted.attributes = unweighted.attributes.add(Locality {
            region: "r1".to_string(),
            zone: "z1".to_string(),
            sub_zone: "sz1".to_string(),
        });
        send_update_with_note(&mut t, Ok(vec![unweighted]), "eds note")
            .expect("update without weighted localities should succeed");
        t.expect_failing_picker("weighted_target_experimental: no targets (eds note)");
        t.expect_no_events();

        // An empty endpoint list also reaches the child.
        t.send_resolver_update(Vec::new())
            .expect("empty endpoint update should succeed");
        t.expect_failing_picker("weighted_target_experimental: no targets");
        t.expect_no_events();
    }

    #[test]
    fn resolver_error_without_child_transitions_to_transient_failure() {
        let mut t = new_env();

        let err = send_update_with_note(&mut t, Err("eds stream closed".to_string()), "eds note")
            .expect_err("resolver error without child should fail update");
        // The resolver's note follows the error.
        assert_eq!(
            err,
            "xds_wrr_locality_experimental: resolver error: eds stream closed (eds note)"
        );

        t.expect_failing_picker(&err);
        t.expect_no_events();
    }

    #[test]
    fn resolver_error_with_existing_child_forwards_to_child() {
        let (mut t, received) = new_recording_env("mock_wrr_resolver_error_forwarded");
        t.send_resolver_update(vec![locality_endpoint("r1", "z1", "sz1", 40)])
            .expect("initial update should succeed");
        t.expect_queuing_picker();

        t.send_resolver_error("eds stream closed".to_string())
            .expect("resolver error should be forwarded to existing child");
        t.expect_queuing_picker();
        t.expect_no_events();

        let (received_update, targets) = received.lock().unwrap().take().unwrap();
        assert_eq!(
            targets,
            BTreeMap::from([(
                "{region=\"r1\", zone=\"z1\", sub_zone=\"sz1\"}".to_string(),
                40,
            )])
        );
        assert_eq!(
            received_update.endpoints,
            Err("eds stream closed".to_string())
        );
    }

    #[test]
    fn generated_child_config_parse_error_transitions_to_transient_failure() {
        let mut t = new_env_with_child("unregistered_child_policy");

        let err = send_update_with_note(
            &mut t,
            Ok(vec![locality_endpoint("r1", "z1", "sz1", 10)]),
            "eds note",
        )
        .expect_err("unregistered child policy should fail update");
        // The resolver's note follows the parse error.
        assert!(
            err.starts_with(
                "xds_wrr_locality_experimental: failed to parse generated child config:"
            ) && err.ends_with(" (eds note)"),
            "unexpected error: {err}"
        );

        t.expect_failing_picker(&err);
        t.expect_no_events();
    }

    #[test]
    fn child_error_still_publishes_picker() {
        test_utils::reg_stub_policy(
            "stub_wrr_child_error",
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(|_data, _update, _cfg, controller| {
                    controller.update_picker(LbState::initial());
                    Err("mock child failure".to_string())
                })),
                ..Default::default()
            },
        );
        let mut t = new_env_with_child("stub_wrr_child_error");

        let err = t
            .send_resolver_update(vec![locality_endpoint("r1", "z1", "sz1", 10)])
            .expect_err("child failure should return error");
        assert_eq!(
            err,
            "xds_wrr_locality_experimental: failed to update child policy: mock child failure"
        );

        t.expect_queuing_picker();
        t.expect_no_events();
    }

    #[test]
    fn work_and_exit_idle_delegate_to_child_and_republish_on_update() {
        test_utils::reg_stub_policy(
            "stub_wrr_work_and_exit_idle",
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(|_data, _update, _cfg, controller| {
                    controller.update_picker(LbState {
                        connectivity_state: ConnectivityState::Idle,
                        picker: Arc::new(QueuingPicker),
                    });
                    Ok(())
                })),
                exit_idle: Some(Arc::new(|data, controller| {
                    if data.test_data.replace(Box::new(())).is_none() {
                        controller.update_picker(LbState::initial());
                        data.lb_policy_options.work_scheduler.schedule_work(None);
                    }
                })),
                work: Some(Arc::new(|_data, _work_data, controller| {
                    controller.update_picker(LbState {
                        connectivity_state: ConnectivityState::Ready,
                        picker: Arc::new(QueuingPicker),
                    });
                })),
            },
        );
        let mut t = new_env_with_child("stub_wrr_work_and_exit_idle");

        // `exit_idle` before any child exists is a no-op.
        t.policy.exit_idle(&mut t.tcc);
        t.expect_no_events();

        // Child starts IDLE.
        t.send_resolver_update(vec![locality_endpoint("r1", "z1", "sz1", 10)])
            .expect("update should succeed");
        t.expect_picker_state(ConnectivityState::Idle);

        // `exit_idle` moves child to CONNECTING and schedules work.
        t.policy.exit_idle(&mut t.tcc);
        t.expect_queuing_picker();

        // Running scheduled work moves child to READY.
        t.run_work();
        t.expect_picker_state(ConnectivityState::Ready);

        // Calling `exit_idle` when child does not update does not republish a
        // picker.
        t.policy.exit_idle(&mut t.tcc);
        t.expect_no_events();
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "xds_wrr_locality_experimental: work called with None value")
    )]
    fn work_with_none_panics_in_debug() {
        let mut t = new_env();
        t.policy.work(None, &mut t.tcc);
    }

    impl Default for WrrLocalityConfig {
        fn default() -> Self {
            Self {
                child_policy: vec![json!({ RR_POLICY_NAME: {} })],
            }
        }
    }

    type RecordedUpdate = (ResolverUpdate, BTreeMap<String, u32>);

    fn new_env() -> TestEnv<WrrLocalityPolicy> {
        test_utils::register_default_policies();
        new_env_with_child(CHILD_POLICY_NAME)
    }

    fn new_env_with_child(child_policy_name: &'static str) -> TestEnv<WrrLocalityPolicy> {
        TestEnv::new(|work_scheduler| WrrLocalityPolicy {
            child_policy_name,
            ..WrrLocalityBuilder.build(LbPolicyOptions {
                work_scheduler,
                runtime: default_runtime(),
            })
        })
    }

    fn new_recording_env(
        child_policy_name: &'static str,
    ) -> (
        TestEnv<WrrLocalityPolicy>,
        Arc<Mutex<Option<RecordedUpdate>>>,
    ) {
        let received: Arc<Mutex<Option<RecordedUpdate>>> = Arc::default();
        GLOBAL_LB_REGISTRY.add_builder(MockChildBuilder {
            name: child_policy_name,
            received: received.clone(),
        });
        (new_env_with_child(child_policy_name), received)
    }

    fn targets_from_update(
        child_policy_name: &'static str,
        endpoints: Vec<Endpoint>,
    ) -> BTreeMap<String, u32> {
        let (mut t, received) = new_recording_env(child_policy_name);
        t.send_resolver_update(endpoints)
            .expect("resolver update should succeed");
        t.expect_queuing_picker();
        t.expect_no_events();
        received.lock().unwrap().take().unwrap().1
    }

    fn send_update_with_note(
        t: &mut TestEnv<WrrLocalityPolicy>,
        endpoints: Result<Vec<Endpoint>, String>,
        note: &str,
    ) -> Result<(), String> {
        let mut update = ResolverUpdate::default();
        update.endpoints = endpoints;
        update.resolution_note = Some(note.to_string());
        t.policy
            .resolver_update(update, &WrrLocalityConfig::default(), &mut t.tcc)
    }

    fn locality_endpoint(region: &str, zone: &str, sub_zone: &str, weight: u32) -> Endpoint {
        let mut endpoint = Endpoint::default();
        endpoint.attributes = endpoint.attributes.add(Locality {
            region: region.to_string(),
            zone: zone.to_string(),
            sub_zone: sub_zone.to_string(),
        });
        set_locality_weight(endpoint, weight)
    }

    #[derive(Debug, Deserialize)]
    struct MockTargetConfig {
        weight: u32,
        #[serde(rename = "childPolicy")]
        child_policy: ParsedLbConfig,
    }

    #[derive(Debug, Deserialize)]
    struct MockChildConfig {
        targets: BTreeMap<String, MockTargetConfig>,
    }

    #[derive(Clone, Debug)]
    struct MockChildBuilder {
        name: &'static str,
        received: Arc<Mutex<Option<RecordedUpdate>>>,
    }

    impl LbPolicyBuilder for MockChildBuilder {
        type LbPolicy = Self;

        fn build(&self, _options: LbPolicyOptions) -> Self::LbPolicy {
            self.clone()
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn parse_config(&self, config: &LbConfigJson) -> Result<MockChildConfig, String> {
            config.convert_to().map_err(|e| e.to_string())
        }
    }

    impl LbPolicy for MockChildBuilder {
        type LbConfig = MockChildConfig;

        fn resolver_update(
            &mut self,
            update: ResolverUpdate,
            config: &Self::LbConfig,
            channel_controller: &mut dyn ChannelController,
        ) -> Result<(), String> {
            let targets = config
                .targets
                .iter()
                .map(|(name, target)| {
                    assert_eq!(target.child_policy.builder.name(), RR_POLICY_NAME);
                    (name.clone(), target.weight)
                })
                .collect();
            *self.received.lock().unwrap() = Some((update, targets));
            channel_controller.update_picker(LbState::initial());
            Ok(())
        }

        fn work(
            &mut self,
            _data: Option<WorkData>,
            _channel_controller: &mut dyn ChannelController,
        ) {
        }

        fn exit_idle(&mut self, _channel_controller: &mut dyn ChannelController) {}
    }
}
