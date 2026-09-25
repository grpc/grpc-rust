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
//! TODO(https://github.com/grpc/grpc-rust/issues/2890): gRFC A31 does not
//! specify retention behaviour for removed clusters, and existing
//! implementations in other languages disagree. Revisit once there is
//! cross-language agreement on the intended behaviour or we implement
//! subchannel caching, which serves the same purpose.

use std::collections::HashMap;
use std::sync::Arc;

use grpc::__unstable::client::load_balancing::ChannelController;
use grpc::__unstable::client::load_balancing::LbConfigJson;
use grpc::__unstable::client::load_balancing::LbPolicy;
use grpc::__unstable::client::load_balancing::LbPolicyBuilder;
use grpc::__unstable::client::load_balancing::LbPolicyOptions;
use grpc::__unstable::client::load_balancing::LbState;
use grpc::__unstable::client::load_balancing::ParsedLbConfig;
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

pub(crate) static POLICY_NAME: &str = "xds_cluster_manager_experimental";

// Target cluster attribute for an RPC, keyed by its `TypeId` in the
// per-call attribute map.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub(crate) struct XdsCluster(pub(crate) String);

impl std::fmt::Display for XdsCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// Validated configuration for `xds_cluster_manager_experimental`.
// Maps a cluster name to a load balancing configuration for that cluster.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ClusterManagerConfig {
    #[serde(default)]
    children: HashMap<XdsCluster, ClusterChildConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct ClusterChildConfig {
    #[serde(rename = "childPolicy")]
    child_policy: ParsedLbConfig,
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

    fn parse_config(&self, config: &LbConfigJson) -> Result<ClusterManagerConfig, String> {
        let parsed: ClusterManagerConfig = config
            .convert_to()
            .map_err(|e| format!("failed to deserialize xds_cluster_manager config: {e}"))?;

        if parsed.children.is_empty() {
            return Err(
                "failed to parse xds_cluster_manager config: 'children' must be non-empty"
                    .to_string(),
            );
        }

        Ok(parsed)
    }
}

// An instance of the Cluster Manager Load Balancing policy.
#[derive(Debug)]
pub(crate) struct ClusterManagerPolicy {
    child_manager: ChildManager<XdsCluster>,
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
            .map(|(cluster, child_cfg)| ChildUpdate {
                child_identifier: cluster.clone(),
                child_policy_builder: child_cfg.child_policy.builder.clone(),
                child_update: Some((update.clone(), &child_cfg.child_policy.config)),
            });

        self.child_manager
            .update(child_updates, channel_controller)?;
        self.update_picker(channel_controller);
        Ok(())
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        // Forward data unconditionally: `ChildManager::work` debug_asserts on a
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
    children: HashMap<XdsCluster, Arc<dyn Picker>>,
}

impl ClusterPicker {
    fn new(children: HashMap<XdsCluster, Arc<dyn Picker>>) -> Self {
        Self { children }
    }

    // Resolves the target cluster from the per-call attributes.
    fn resolve_cluster<'a>(
        &self,
        attributes: &'a CallAttributes,
    ) -> Result<&'a XdsCluster, StatusError> {
        attributes.get::<XdsCluster>().ok_or_else(|| {
            // Todo: should this be INTERNAL?
            StatusError::new(
                StatusCodeError::Unavailable,
                "cluster manager: cluster attribute not present",
            )
        })
    }

    // Looks up the child picker for `cluster`.
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
    fn child_picker(&self, cluster: &XdsCluster) -> Result<&Arc<dyn Picker>, StatusError> {
        self.children.get(cluster).ok_or_else(|| {
            StatusError::new(
                StatusCodeError::Unavailable,
                format!("cluster manager: unknown cluster '{cluster}'"),
            )
        })
    }
}

impl Picker for ClusterPicker {
    fn pick(&self, options: PickOptions<'_>) -> PickResult {
        let picker = match self
            .resolve_cluster(options.call_attributes)
            .and_then(|cluster| self.child_picker(cluster))
        {
            Ok(picker) => picker,
            Err(err) => return PickResult::Drop(err),
        };
        picker.pick(options)
    }
}

#[cfg(test)]
mod tests {
    use grpc::__unstable::client::load_balancing::GLOBAL_LB_REGISTRY;
    use grpc::__unstable::client::load_balancing::WorkScheduler;
    use grpc::__unstable::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use grpc::__unstable::client::load_balancing::subchannel::Subchannel;
    use grpc::__unstable::client::load_balancing::subchannel::SubchannelState;
    use grpc::__unstable::rt::default_runtime;
    use grpc::client::ConnectivityState;
    use grpc::client::RequestHeaders;
    use grpc::core::Address;

    use super::*;

    #[test]
    fn parse_config() {
        let builder = ClusterManagerLbBuilder;
        assert_eq!(builder.name(), "xds_cluster_manager_experimental");

        let valid_json = serde_json::json!({
            "children": {
                "cluster_a": { "childPolicy": [{ RR_POLICY_NAME: {} }] },
                "cluster_b": { "childPolicy": [{ RR_POLICY_NAME: {} }] }
            }
        });
        let config = builder
            .parse_config(&LbConfigJson::new(&valid_json.to_string()).unwrap())
            .expect("parse valid config");
        assert_eq!(config.children.len(), 2);
        assert!(
            config
                .children
                .contains_key(&XdsCluster("cluster_a".into()))
        );
        assert!(
            config
                .children
                .contains_key(&XdsCluster("cluster_b".into()))
        );

        let empty_children = serde_json::json!({ "children": {} });
        let err = builder
            .parse_config(&LbConfigJson::new(&empty_children.to_string()).unwrap())
            .unwrap_err();
        assert!(err.contains("children"));
    }

    #[test]
    fn cluster_picker_routing() {
        let mut children: HashMap<XdsCluster, Arc<dyn Picker>> = HashMap::new();
        children.insert(
            XdsCluster("cluster_one".to_string()),
            Arc::new(TaggedPicker("child_one".to_string())),
        );
        let cluster_picker = ClusterPicker { children };
        let req = RequestHeaders::new();

        // Known cluster -> delegates to child picker and forwards CallAttributes.
        let mut attrs = CallAttributes::new();
        attrs.add(XdsCluster("cluster_one".into()));
        assert!(matches!(
            cluster_picker.pick(PickOptions::new(&req, &mut attrs)),
            PickResult::Queue
        ));
        assert_eq!(
            attrs.get::<PickedChild>(),
            Some(&PickedChild("child_one".into()))
        );

        // Unknown cluster -> Drop(Unavailable).
        let mut attrs = CallAttributes::new();
        attrs.add(XdsCluster("cluster_unknown".into()));
        match cluster_picker.pick(PickOptions::new(&req, &mut attrs)) {
            PickResult::Drop(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("unknown cluster"));
            }
            other => panic!("expected Drop, got {other:?}"),
        }

        // Missing XdsCluster attribute -> Drop(Unavailable).
        let mut empty_attrs = CallAttributes::new();
        match cluster_picker.pick(PickOptions::new(&req, &mut empty_attrs)) {
            PickResult::Drop(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("not present"));
            }
            other => panic!("expected Drop, got {other:?}"),
        }
    }

    #[test]
    fn removed_cluster_is_shut_down() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();

        let state = apply_json_config(
            &mut policy,
            &mut controller,
            serde_json::json!({
                "children": {
                    "cluster_a": { "childPolicy": [{ "test_dummy_lb": { "tag": "child_a" } }] },
                    "cluster_b": { "childPolicy": [{ "test_dummy_lb": { "tag": "child_b" } }] }
                }
            }),
        );
        assert_eq!(policy.child_manager.children().count(), 2);
        assert_eq!(state.connectivity_state, ConnectivityState::Ready);
        assert_picks_child(&state.picker, "cluster_a", "child_a");
        assert_picks_child(&state.picker, "cluster_b", "child_b");

        // Drop cluster_b from the config.
        let state = apply_json_config(
            &mut policy,
            &mut controller,
            serde_json::json!({
                "children": {
                    "cluster_a": { "childPolicy": [{ "test_dummy_lb": { "tag": "child_a" } }] }
                }
            }),
        );

        let identifiers: Vec<&XdsCluster> = policy
            .child_manager
            .children()
            .map(|c| &c.identifier)
            .collect();
        assert_eq!(identifiers, vec![&XdsCluster("cluster_a".into())]);
        assert_picks_child(&state.picker, "cluster_a", "child_a");

        let req = RequestHeaders::new();
        let mut attrs_b = CallAttributes::new();
        attrs_b.add(XdsCluster("cluster_b".into()));
        match state.picker.pick(PickOptions::new(&req, &mut attrs_b)) {
            PickResult::Drop(err) => assert_eq!(err.code(), StatusCodeError::Unavailable),
            other => panic!("expected Drop for removed cluster, got {other:?}"),
        }
    }

    #[test]
    fn work_delegates_to_child_and_refreshes_picker() {
        let (mut policy, scheduler, mut controller) = setup_test_policy();

        let initial = apply_json_config(
            &mut policy,
            &mut controller,
            serde_json::json!({
                "children": {
                    "cluster_a": {
                        "childPolicy": [{ "test_dummy_lb": { "tag": "work_ready", "start_connecting": true } }]
                    }
                }
            }),
        );
        assert_eq!(initial.connectivity_state, ConnectivityState::Connecting);

        let work_item = scheduler
            .queued_work
            .lock()
            .unwrap()
            .pop()
            .expect("scheduled work");
        policy.work(work_item, &mut controller);

        let updated = controller.latest_state.take().expect("updated state");
        assert_eq!(updated.connectivity_state, ConnectivityState::Ready);
        assert_picks_child(&updated.picker, "cluster_a", "work_ready");
    }

    #[test]
    fn exit_idle_wakes_children_and_refreshes_picker() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();

        let initial = apply_json_config(
            &mut policy,
            &mut controller,
            serde_json::json!({
                "children": {
                    "cluster_a": { "childPolicy": [{ "test_dummy_lb": { "tag": "woken_a", "start_idle": true } }] },
                    "cluster_b": { "childPolicy": [{ "test_dummy_lb": { "tag": "woken_b", "start_idle": true } }] }
                }
            }),
        );
        assert_eq!(initial.connectivity_state, ConnectivityState::Idle);

        policy.exit_idle(&mut controller);

        let woken = controller.latest_state.take().expect("woken state");
        assert_eq!(woken.connectivity_state, ConnectivityState::Ready);
        assert_picks_child(&woken.picker, "cluster_a", "woken_a");
        assert_picks_child(&woken.picker, "cluster_b", "woken_b");
    }

    fn setup_test_policy() -> (
        ClusterManagerPolicy,
        Arc<MockScheduler>,
        MockChannelController,
    ) {
        GLOBAL_LB_REGISTRY.add_builder(TestDummyLbBuilder);
        let scheduler = Arc::new(MockScheduler::default());
        let policy = ClusterManagerLbBuilder.build(LbPolicyOptions {
            work_scheduler: scheduler.clone(),
            // TODO: replace with a no-op test runtime once `rt::Runtime` can be
            // implemented outside the `grpc` crate. That is blocked on
            // `EndpointListener` being `pub(crate)`; see the TODO on
            // `default_runtime`.
            runtime: default_runtime(),
        });
        let controller = MockChannelController { latest_state: None };
        (policy, scheduler, controller)
    }

    fn apply_json_config(
        policy: &mut ClusterManagerPolicy,
        controller: &mut MockChannelController,
        json: serde_json::Value,
    ) -> LbState {
        let cfg = ClusterManagerLbBuilder
            .parse_config(&LbConfigJson::new(&json.to_string()).unwrap())
            .unwrap();
        policy
            .resolver_update(ResolverUpdate::default(), &cfg, controller)
            .unwrap();
        controller.latest_state.take().expect("state update")
    }

    fn assert_picks_child(picker: &Arc<dyn Picker>, cluster: &str, expected_tag: &str) {
        let req = RequestHeaders::new();
        let mut attrs = CallAttributes::new();
        attrs.add(XdsCluster(cluster.into()));
        assert!(matches!(
            picker.pick(PickOptions::new(&req, &mut attrs)),
            PickResult::Queue
        ));
        assert_eq!(
            attrs.get::<PickedChild>(),
            Some(&PickedChild(expected_tag.into()))
        );
    }

    #[derive(Debug)]
    struct DummyPicker;

    impl Picker for DummyPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            PickResult::Queue
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct PickedChild(String);

    #[derive(Debug)]
    struct TaggedPicker(String);

    impl Picker for TaggedPicker {
        fn pick(&self, options: PickOptions<'_>) -> PickResult {
            options.call_attributes.add(PickedChild(self.0.clone()));
            PickResult::Queue
        }
    }

    #[derive(Debug, Default)]
    struct MockScheduler {
        queued_work: std::sync::Mutex<Vec<Option<WorkData>>>,
    }

    impl WorkScheduler for MockScheduler {
        fn schedule_work(&self, data: Option<WorkData>) {
            self.queued_work.lock().unwrap().push(data);
        }
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

    #[derive(Debug, Default, Deserialize)]
    struct TestDummyConfig {
        #[serde(default)]
        tag: String,
        #[serde(default)]
        start_connecting: bool,
        #[serde(default)]
        start_idle: bool,
    }

    #[derive(Debug)]
    struct TestDummyLbPolicy {
        work_scheduler: Arc<dyn WorkScheduler>,
        tag: String,
    }

    impl LbPolicy for TestDummyLbPolicy {
        type LbConfig = TestDummyConfig;

        fn resolver_update(
            &mut self,
            _update: ResolverUpdate,
            config: &Self::LbConfig,
            channel_controller: &mut dyn ChannelController,
        ) -> Result<(), String> {
            self.tag = config.tag.clone();
            if config.start_connecting {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Connecting,
                    picker: Arc::new(DummyPicker),
                });
                self.work_scheduler.schedule_work(None);
            } else if config.start_idle {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Idle,
                    picker: Arc::new(DummyPicker),
                });
            } else {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Ready,
                    picker: Arc::new(TaggedPicker(config.tag.clone())),
                });
            }
            Ok(())
        }

        fn work(
            &mut self,
            _data: Option<WorkData>,
            channel_controller: &mut dyn ChannelController,
        ) {
            channel_controller.update_picker(LbState {
                connectivity_state: ConnectivityState::Ready,
                picker: Arc::new(TaggedPicker(self.tag.clone())),
            });
        }

        fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
            channel_controller.update_picker(LbState {
                connectivity_state: ConnectivityState::Ready,
                picker: Arc::new(TaggedPicker(self.tag.clone())),
            });
        }
    }

    #[derive(Debug)]
    struct TestDummyLbBuilder;

    impl LbPolicyBuilder for TestDummyLbBuilder {
        type LbPolicy = TestDummyLbPolicy;

        fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
            TestDummyLbPolicy {
                work_scheduler: options.work_scheduler,
                tag: String::new(),
            }
        }

        fn name(&self) -> &'static str {
            "test_dummy_lb"
        }

        fn parse_config(&self, config: &LbConfigJson) -> Result<TestDummyConfig, String> {
            config.convert_to().map_err(|e| e.to_string())
        }
    }
}
