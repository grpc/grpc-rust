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

//! Implementation of the `xds_cluster_manager_experimental` Load Balancing policy.
//!
//! The Cluster Manager LB policy ([gRFC A31](https://github.com/grpc/proposal/blob/master/A31-xds-timeout-support-and-config-selector.md))
//! manages a list of child LB policies, each corresponding to an xDS cluster.
//! This policy selects a child policy (and therefore cluster) to route each RPC
//! to.
//!
//! Clusters removed from the configuration are shut down immediately.
//!
//! TODO: gRFC A31 does not specify retention behaviour for removed clusters,
//! and existing implementations in other langauges disagree. Revisit
//! once there is cross-language agreement on the intended behaviour or we
//! implement subchannel caching, which serves the same purpose.

use std::collections::HashMap;
use std::sync::Arc;

use grpc::__unstable::client::load_balancing::ChannelController;
use grpc::__unstable::client::load_balancing::DynLbConfig;
use grpc::__unstable::client::load_balancing::DynLbPolicyBuilder;
use grpc::__unstable::client::load_balancing::GLOBAL_LB_REGISTRY;
use grpc::__unstable::client::load_balancing::LbPolicy;
use grpc::__unstable::client::load_balancing::LbPolicyBuilder;
use grpc::__unstable::client::load_balancing::LbPolicyOptions;
use grpc::__unstable::client::load_balancing::LbState;
use grpc::__unstable::client::load_balancing::ParsedJsonLbConfig;
use grpc::__unstable::client::load_balancing::PickOptions;
use grpc::__unstable::client::load_balancing::PickResult;
use grpc::__unstable::client::load_balancing::Picker;
use grpc::__unstable::client::load_balancing::WorkData;
use grpc::__unstable::client::load_balancing::child_manager::ChildManager;
use grpc::__unstable::client::load_balancing::child_manager::ChildUpdate;
use grpc::__unstable::client::name_resolution::ResolverUpdate;
use grpc::StatusCodeError;
use grpc::StatusError;
use grpc::call_attributes::CallAttributes;
use serde::Deserialize;
use serde::Serialize;

pub(crate) static POLICY_NAME: &str = "xds_cluster_manager_experimental";

// Target cluster attribute for an RPC, keyed by its `TypeId` in the
// per-call attribute map.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct XdsCluster(pub(crate) String);

// TODO: deduplicate this with LbInnerConfig.
#[derive(Clone, Debug)]
struct ChildConfig {
    builder: Arc<DynLbPolicyBuilder>,
    config: DynLbConfig,
}

// Validated configuration for `xds_cluster_manager_experimental`.
// Maps a cluster name to a load balancing configuration for that cluster.
#[derive(Clone, Debug)]
pub(crate) struct ClusterManagerConfig {
    // Todo: Hashmap or BTreeMap?
    children: HashMap<String, ChildConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClusterManagerConfigJson {
    #[serde(default)]
    children: HashMap<String, ClusterChildConfigJson>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClusterChildConfigJson {
    #[serde(rename = "childPolicy")]
    child_policy: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Default)]
pub(crate) struct ClusterManagerLbBuilder;

impl LbPolicyBuilder for ClusterManagerLbBuilder {
    type LbPolicy = ClusterManagerPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        ClusterManagerPolicy::new(options)
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    // TODO De-duplicate the selection mechanism here with that used in ServiceConfig.
    fn parse_config(&self, config: &ParsedJsonLbConfig) -> Result<ClusterManagerConfig, String> {
        let json_val: ClusterManagerConfigJson = config
            .convert_to()
            .map_err(|e| format!("failed to deserialize xds_cluster_manager config: {e}"))?;

        if json_val.children.is_empty() {
            return Err(
                "failed to parse xds_cluster_manager config: 'children' must be non-empty"
                    .to_string(),
            );
        }

        let mut parsed_children = HashMap::with_capacity(json_val.children.len());
        for (cluster_name, child_cfg) in json_val.children {
            if child_cfg.child_policy.is_empty() {
                return Err(format!(
                    "cluster '{cluster_name}': childPolicy list must be non-empty"
                ));
            }
            let mut resolved = None;
            for candidate_map in child_cfg.child_policy {
                if candidate_map.len() != 1 {
                    return Err(format!(
                        "cluster '{cluster_name}': childPolicy entry must contain exactly 1 policy/config pair"
                    ));
                }
                let (policy_name, policy_json) = candidate_map.into_iter().next().unwrap();
                if let Some(builder) = GLOBAL_LB_REGISTRY.get_policy(&policy_name) {
                    let parsed_lb_config = builder
                        .parse_config(&ParsedJsonLbConfig::from_value(policy_json))
                        .map_err(|e| {
                            format!(
                                "cluster '{cluster_name}': failed to parse child policy '{policy_name}': {e}"
                            )
                        })?;
                    resolved = Some(ChildConfig {
                        builder,
                        config: parsed_lb_config,
                    });
                    break;
                }
            }
            let child_config = resolved.ok_or_else(|| {
                format!(
                    "cluster '{cluster_name}': no supported child policy found in childPolicy list"
                )
            })?;
            parsed_children.insert(cluster_name, child_config);
        }

        Ok(ClusterManagerConfig {
            children: parsed_children,
        })
    }
}

// An instance of the Cluster Manager Load Balancing policy.
#[derive(Debug)]
pub(crate) struct ClusterManagerPolicy {
    child_manager: ChildManager<String>,
}

impl ClusterManagerPolicy {
    fn new(options: LbPolicyOptions) -> Self {
        Self {
            child_manager: ChildManager::new(options.runtime, options.work_scheduler),
        }
    }

    // Publishes a picker covering every configured cluster, together with the
    // aggregate connectivity state of the children.
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController) {
        let connectivity_state = self.child_manager.aggregate_states();
        let pickers = self
            .child_manager
            .children()
            .map(|c| (c.identifier.clone(), c.state.picker.clone()))
            .collect::<HashMap<_, _>>();

        channel_controller.update_picker(LbState {
            connectivity_state,
            picker: Arc::new(ClusterPicker::new(pickers)),
        });
    }
}

impl LbPolicy for ClusterManagerPolicy {
    type LbConfig = ClusterManagerConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        // `ChildManager::update` removes any existing child that is absent from
        // this list, so clusters dropped from the config are shut down here.
        //
        // TODO: this is only safe once the Config Selector implements the
        // two-step removal in gRFC A31 -- the Resolver must first publish a
        // Config Selector that cannot select the cluster while the Service
        // Config still lists it, and only drop it from the Service Config once
        // no in-flight RPC references it. Until then a cluster can disappear
        // from under RPCs that have already selected it.
        let child_updates = config
            .children
            .iter()
            .map(|(cluster_name, child_cfg)| ChildUpdate {
                child_identifier: cluster_name.clone(),
                child_policy_builder: child_cfg.builder.clone(),
                child_update: Some((update.clone(), &child_cfg.config)),
            });

        self.child_manager
            .update(child_updates, channel_controller)?;
        self.update_picker(channel_controller);
        Ok(())
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        // Every work item originates from a child, so `data` is always `Some`.
        // Forward it unconditionally: `ChildManager::work` debug_asserts on a
        // `None` payload, and suppressing that here would hide the violation.
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

// Picker that delegates to the active child picker corresponding to the cluster attribute.
#[derive(Debug)]
struct ClusterPicker {
    children: HashMap<String, Arc<dyn Picker>>,
}

impl ClusterPicker {
    fn new(children: HashMap<String, Arc<dyn Picker>>) -> Self {
        Self { children }
    }

    // Resolves the target cluster name from the per-call attributes.
    fn resolve_cluster<'a>(&self, attributes: &'a CallAttributes) -> Result<&'a str, StatusError> {
        attributes
            .get::<XdsCluster>()
            .map(|cluster| cluster.0.as_str())
            .ok_or_else(|| {
                // Todo: should this be INTERNAL?
                StatusError::new(
                    StatusCodeError::Unavailable,
                    "cluster manager: cluster attribute not present",
                )
            })
    }

    // Routes the request to the child picker for `cluster_name`.
    //
    // A cluster missing from the map means either that it was removed from the
    // config, or that the Config Selector named a cluster this picker has never
    // seen. Per gRFC A31 both must terminate the RPC rather than queue it: a
    // wait-for-ready RPC left queued would hold a reference to the cluster
    // indefinitely and block its removal. `PickResult::Fail` does not terminate
    // wait-for-ready RPCs, so `PickResult::Drop` is required here.
    //
    // TODO: A31 distinguishes the two cases -- UNAVAILABLE for a deleted
    // cluster, INTERNAL for a name the picker does not recognise, which "should
    // not be possible" given the Config Selector's two-step removal. This
    // picker cannot tell them apart, and grpc-java and grpc-core disagree on
    // the code (UNAVAILABLE vs INTERNAL). Determine what is correct here.
    fn route_to_cluster(&self, cluster_name: &str, options: PickOptions<'_>) -> PickResult {
        match self.children.get(cluster_name) {
            Some(picker) => picker.pick(options),
            None => PickResult::Drop(StatusError::new(
                StatusCodeError::Unavailable,
                format!("cluster manager: unknown cluster '{cluster_name}'"),
            )),
        }
    }
}

impl Picker for ClusterPicker {
    fn pick(&self, options: PickOptions<'_>) -> PickResult {
        let cluster_name = match self.resolve_cluster(options.call_attributes) {
            Ok(name) => name.to_owned(),
            Err(err) => return PickResult::Drop(err),
        };
        self.route_to_cluster(&cluster_name, options)
    }
}

#[cfg(test)]
mod tests {
    use grpc::__unstable::client::load_balancing::WorkScheduler;
    use grpc::__unstable::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use grpc::__unstable::client::load_balancing::subchannel::Subchannel;
    use grpc::__unstable::client::load_balancing::subchannel::SubchannelState;
    use grpc::__unstable::rt::default_runtime;
    use grpc::client::ConnectivityState;
    use grpc::client::RequestHeaders;
    use grpc::core::Address;

    use super::*;

    // TODO: outstanding test coverage gaps (probably - needs review)
    //
    // Policy plumbing:
    // - `work()` delegating data to the child manager.
    // - `work(None)`.
    // - `child_manager.update()` returning Err.
    // - picker refresh driven by a child connectivity change.
    //
    // Also check whether the registry can be better injected for tests.

    #[test]
    fn policy_builder_name() {
        let builder = ClusterManagerLbBuilder;
        assert_eq!(builder.name(), "xds_cluster_manager_experimental");
    }

    #[test]
    fn parse_valid_config() {
        let json_str = serde_json::json!({
            "children": {
                "cluster_a": {
                    "childPolicy": [
                        { RR_POLICY_NAME: {} }
                    ]
                },
                "cluster_b": {
                    "childPolicy": [
                        { "unknown_policy": {} },
                        { RR_POLICY_NAME: {} }
                    ]
                }
            }
        })
        .to_string();

        let parsed_json = ParsedJsonLbConfig::new(&json_str).expect("parse json");
        let builder = ClusterManagerLbBuilder;
        let config = builder.parse_config(&parsed_json).expect("parse config");

        assert_eq!(config.children.len(), 2);
        assert!(config.children.contains_key("cluster_a"));
        assert!(config.children.contains_key("cluster_b"));
    }

    #[test]
    fn parse_no_supported_policy_fails() {
        let json_str = serde_json::json!({
            "children": {
                "cluster_a": {
                    "childPolicy": [
                        { "unsupported_lb_policy": {} }
                    ]
                }
            }
        })
        .to_string();

        let parsed_json = ParsedJsonLbConfig::new(&json_str).expect("parse json");
        let builder = ClusterManagerLbBuilder;
        let err = builder.parse_config(&parsed_json).unwrap_err();
        assert!(err.contains("no supported child policy"));
    }

    #[derive(Debug)]
    struct DummyPicker;

    impl Picker for DummyPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            PickResult::Queue
        }
    }

    #[test]
    fn cluster_picker_pick_fails_without_attributes() {
        let mut children: HashMap<String, Arc<dyn Picker>> = HashMap::new();
        children.insert("cluster_one".to_string(), Arc::new(DummyPicker));
        let cluster_picker = ClusterPicker { children };

        let req = RequestHeaders::new();
        let mut attrs = CallAttributes::new();
        match cluster_picker.pick(PickOptions::new(&req, &mut attrs)) {
            PickResult::Drop(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("not present"));
            }
            other => panic!("expected Drop, got {other:?}"),
        }
    }

    #[test]
    fn cluster_picker_with_attributes_routing() {
        let mut children: HashMap<String, Arc<dyn Picker>> = HashMap::new();
        children.insert("cluster_one".to_string(), Arc::new(DummyPicker));
        let cluster_picker = ClusterPicker { children };

        let req = RequestHeaders::new();
        let attrs_for = |cluster: &str| {
            let mut attrs = CallAttributes::new();
            attrs.add(XdsCluster(cluster.into()));
            attrs
        };

        // Known cluster in attributes -> routes to child picker
        let mut attrs_known = attrs_for("cluster_one");
        match cluster_picker.pick(PickOptions::new(&req, &mut attrs_known)) {
            PickResult::Queue => {}
            other => panic!("expected Queue from DummyPicker, got {other:?}"),
        }

        // Unknown cluster in attributes -> UNAVAILABLE
        let mut attrs_unknown = attrs_for("cluster_unknown");
        match cluster_picker.pick(PickOptions::new(&req, &mut attrs_unknown)) {
            PickResult::Drop(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("unknown cluster"));
            }
            other => panic!("expected Drop, got {other:?}"),
        }
    }

    #[test]
    fn parse_empty_children_fails() {
        let json_str = serde_json::json!({
            "children": {}
        })
        .to_string();

        let parsed_json = ParsedJsonLbConfig::new(&json_str).expect("parse json");
        let builder = ClusterManagerLbBuilder;
        let err = builder.parse_config(&parsed_json).unwrap_err();
        assert!(err.contains("children"));
    }

    #[test]
    fn parse_empty_child_policy_fails() {
        let json_str = serde_json::json!({
            "children": {
                "cluster_a": {
                    "childPolicy": []
                }
            }
        })
        .to_string();

        let parsed_json = ParsedJsonLbConfig::new(&json_str).expect("parse json");
        let builder = ClusterManagerLbBuilder;
        let err = builder.parse_config(&parsed_json).unwrap_err();
        assert!(err.contains("childPolicy"));
    }

    #[derive(Debug)]
    struct MockScheduler;

    impl WorkScheduler for MockScheduler {
        fn schedule_work(&self, _data: Option<WorkData>) {}
    }

    struct MockChannelController {
        latest_state: Option<LbState>,
    }

    impl ChannelController for MockChannelController {
        fn new_subchannel(
            &mut self,
            _address: &Address,
            _work_scheduler: Arc<dyn WorkScheduler>,
        ) -> (Arc<dyn Subchannel>, SubchannelState) {
            unimplemented!()
        }

        fn update_picker(&mut self, update: LbState) {
            self.latest_state = Some(update);
        }

        fn request_resolution(&mut self) {}
    }

    #[derive(Debug)]
    struct TestDummyLbPolicy;

    impl LbPolicy for TestDummyLbPolicy {
        type LbConfig = ();

        fn resolver_update(
            &mut self,
            _update: ResolverUpdate,
            _config: &Self::LbConfig,
            channel_controller: &mut dyn ChannelController,
        ) -> Result<(), String> {
            channel_controller.update_picker(LbState {
                connectivity_state: ConnectivityState::Ready,
                picker: Arc::new(DummyPicker),
            });
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

    #[derive(Debug)]
    struct TestDummyLbBuilder;

    impl LbPolicyBuilder for TestDummyLbBuilder {
        type LbPolicy = TestDummyLbPolicy;

        fn build(&self, _options: LbPolicyOptions) -> Self::LbPolicy {
            TestDummyLbPolicy
        }

        fn name(&self) -> &'static str {
            "test_dummy_lb"
        }

        fn parse_config(&self, _config: &ParsedJsonLbConfig) -> Result<(), String> {
            Ok(())
        }
    }

    // TODO: cover exit_idle. `TestDummyLbPolicy::exit_idle` is an empty body,
    // so nothing can currently observe which children are woken. Making it
    // record the call would allow asserting that exit_idle reaches every
    // configured child.
    #[tokio::test]
    async fn removed_cluster_is_shut_down() {
        GLOBAL_LB_REGISTRY.add_builder(TestDummyLbBuilder);

        let builder = ClusterManagerLbBuilder;
        let mut policy = builder.build(LbPolicyOptions {
            work_scheduler: Arc::new(MockScheduler),
            // TODO: replace with a no-op test runtime once `rt::Runtime` can be
            // implemented outside the `grpc` crate. That is blocked on
            // `EndpointListener` being `pub(crate)`; see the TODO on
            // `default_runtime`. Nothing here spawns or sleeps, so a stub would
            // do, but `default_runtime` is the only `GrpcRuntime` this crate
            // can construct.
            runtime: default_runtime(),
        });

        let mut controller = MockChannelController { latest_state: None };

        // Initial config with cluster_a and cluster_b.
        let json_2_clusters = serde_json::json!({
            "children": {
                "cluster_a": {
                    "childPolicy": [{ "test_dummy_lb": {} }]
                },
                "cluster_b": {
                    "childPolicy": [{ "test_dummy_lb": {} }]
                }
            }
        });
        let parsed_cfg_2 = builder
            .parse_config(&ParsedJsonLbConfig::from_value(json_2_clusters))
            .unwrap();

        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg_2, &mut controller)
            .unwrap();

        let state = controller.latest_state.take().expect("state update");
        assert_eq!(policy.child_manager.children().count(), 2);
        assert_eq!(state.connectivity_state, ConnectivityState::Ready);

        // Drop cluster_b from the config.
        let json_1_cluster = serde_json::json!({
            "children": {
                "cluster_a": {
                    "childPolicy": [{ "test_dummy_lb": {} }]
                }
            }
        });
        let parsed_cfg_1 = builder
            .parse_config(&ParsedJsonLbConfig::from_value(json_1_cluster))
            .unwrap();

        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg_1, &mut controller)
            .unwrap();

        // cluster_b is gone from child_manager immediately
        let identifiers: Vec<&str> = policy
            .child_manager
            .children()
            .map(|c| c.identifier.as_str())
            .collect();
        assert_eq!(identifiers, vec!["cluster_a"]);

        // ...and so is absent from the picker.
        let state = controller.latest_state.take().expect("state update");
        let req = RequestHeaders::new();
        let mut attrs_a = CallAttributes::new();
        attrs_a.add(XdsCluster("cluster_a".into()));
        assert!(matches!(
            state.picker.pick(PickOptions::new(&req, &mut attrs_a)),
            PickResult::Queue
        ));

        let mut attrs_b = CallAttributes::new();
        attrs_b.add(XdsCluster("cluster_b".into()));
        match state.picker.pick(PickOptions::new(&req, &mut attrs_b)) {
            PickResult::Drop(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("unknown cluster"));
            }
            other => panic!("expected Drop for removed cluster, got {other:?}"),
        }
    }
}
