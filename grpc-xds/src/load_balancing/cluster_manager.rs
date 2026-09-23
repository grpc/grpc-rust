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
//! The Cluster Manager LB policy ([gRFC A31](https://github.com/grpc/proposal/blob/master/A31-xds-timeout-support-and-config-selector.md1))
//! manages a list of child LB policies, each corresponding to an xDS cluster.
//! This policy selects a child policy (and therefore cluster) to route each RPC
//! to.
//!
//! Clusters removed from the configuration enter a 15-minute deactivation grace
//! period before being deleted. If a deactivated cluster reappears in
//! configuration before the timer expires, it is reactivated without recreating
//! connections.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use grpc::__unstable::client::load_balancing::ChannelController;
use grpc::__unstable::client::load_balancing::DynLbConfig;
use grpc::__unstable::client::load_balancing::DynLbPolicy;
use grpc::__unstable::client::load_balancing::DynLbPolicyBuilder;
use grpc::__unstable::client::load_balancing::GLOBAL_LB_REGISTRY;
use grpc::__unstable::client::load_balancing::LbPolicy;
use grpc::__unstable::client::load_balancing::LbPolicyBuilder;
use grpc::__unstable::client::load_balancing::LbPolicyOptions;
use grpc::__unstable::client::load_balancing::LbState;
use grpc::__unstable::client::load_balancing::ParsedJsonLbConfig;
use grpc::__unstable::client::load_balancing::PickResult;
use grpc::__unstable::client::load_balancing::Picker;
use grpc::__unstable::client::load_balancing::WorkData;
use grpc::__unstable::client::load_balancing::WorkScheduler;
use grpc::__unstable::client::load_balancing::child_manager::Child;
use grpc::__unstable::client::load_balancing::child_manager::ChildManager;
use grpc::__unstable::client::load_balancing::child_manager::ChildUpdate;
use grpc::__unstable::client::name_resolution::ResolverUpdate;
use grpc::__unstable::rt::BoxedTaskHandle;
use grpc::__unstable::rt::GrpcRuntime;
use grpc::StatusCodeError;
use grpc::StatusError;
use grpc::client::ConnectivityState;
use grpc::client::RequestHeaders;
use http::Extensions;
use serde::Deserialize;
use serde::Serialize;

pub(crate) static POLICY_NAME: &str = "xds_cluster_manager_experimental";

// Deactivation timeout before a removed child policy is terminated (15 minutes).
pub(crate) const DEFAULT_DEACTIVATION_TIMEOUT: Duration = Duration::from_secs(15 * 60);

// Target cluster attribute for an RPC, keyed by its `TypeId` in the
// per-call attribute map.
//
// TODO: swap [`Extensions`] for `grpc::call_attributes::CallAttributes` once
// it lands (grpc/grpc-rust#2878). Both are TypeId-keyed and `get` is identical;
// `CallAttributes` drops the `Sync` bound, so the swap only loosens
// requirements. Note `&CallAttributes` will not be `Send`, unlike
// `&Extensions`, so the reference must not outlive a synchronous pick.
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

// Deactivation event payload sent to the policy's work scheduler upon timeout expiry.
#[derive(Debug)]
struct ClusterDeactivationTimeout {
    cluster_name: String,
}

// TODO: This probably needs to live in grpc/src/rt/mod.rs
// Guard wrapping a [`BoxedTaskHandle`] that aborts the underlying task when dropped.
struct AbortOnDropHandle {
    handle: BoxedTaskHandle,
}

impl AbortOnDropHandle {
    fn new(handle: BoxedTaskHandle) -> Self {
        Self { handle }
    }
}

impl Drop for AbortOnDropHandle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl Debug for AbortOnDropHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbortOnDropHandle").finish_non_exhaustive()
    }
}

// Lifecycle status of a child managed by the cluster manager policy.
#[derive(Clone, Debug)]
enum ChildStatus {
    Active,
    // The cluster is deactivated (removed from config) and in a grace period.
    // Holding this variant keeps the deactivation timer task alive. When all
    // handles to this state are dropped, the timer is aborted.
    //
    // The handle is never read: its only purpose is the `Drop` side effect.
    Deactivated(Arc<AbortOnDropHandle>),
}

// Builder for child cluster LB policies wrapping an inner `DynLbPolicyBuilder`.
// Tracks `ChildStatus` to indicate whether this child is active or in
// deactivation grace period.
#[derive(Clone, Debug)]
struct AnnotatedChildBuilder {
    inner: Arc<DynLbPolicyBuilder>,
    status: ChildStatus,
}

// This is a wrapper so that each child policy can be marked as either active
// or deactivated. There are two big reasons for this:
// - Any path that removes a child is guaranteed to abort timers. This ensures
//   that a delayed timeout does not race with a re-inserted active child.
// - Single source of truth for child and activation status.
impl AnnotatedChildBuilder {
    fn new_active(inner: Arc<DynLbPolicyBuilder>) -> Self {
        Self {
            inner,
            status: ChildStatus::Active,
        }
    }

    fn new_deactivated(inner: Arc<DynLbPolicyBuilder>, handle: Arc<AbortOnDropHandle>) -> Self {
        Self {
            inner,
            status: ChildStatus::Deactivated(handle),
        }
    }

    fn is_active(&self) -> bool {
        matches!(self.status, ChildStatus::Active)
    }

    fn is_deactivated(&self) -> bool {
        matches!(self.status, ChildStatus::Deactivated(_))
    }
}

impl LbPolicyBuilder for AnnotatedChildBuilder {
    type LbPolicy = Box<DynLbPolicy>;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        self.inner.build(options)
    }

    fn name(&self) -> &'static str {
        // This *must* return exactly the inner name of the child so that the
        // child manager can correctly index it.
        self.inner.name()
    }

    fn parse_config(&self, config: &ParsedJsonLbConfig) -> Result<DynLbConfig, String> {
        self.inner.parse_config(config)
    }
}

// An instance of the Cluster Manager Load Balancing policy.
#[derive(Debug)]
pub(crate) struct ClusterManagerPolicy {
    work_scheduler: Arc<dyn WorkScheduler>,
    runtime: GrpcRuntime,
    child_manager: ChildManager<String, AnnotatedChildBuilder>,
}

impl ClusterManagerPolicy {
    fn new(options: LbPolicyOptions) -> Self {
        let child_manager =
            ChildManager::new(options.runtime.clone(), options.work_scheduler.clone());
        Self {
            child_manager,
            work_scheduler: options.work_scheduler,
            runtime: options.runtime,
        }
    }

    // Aggregates connectivity state across active children only.
    //
    // Precedence: READY > CONNECTING > IDLE > TRANSIENT_FAILURE.
    // If no active children exist, reports TRANSIENT_FAILURE.
    // Ignores deactivated children. If there are no active children, reports
    // TRANSIENT_FAILURE.
    fn aggregate_active_states(&self) -> ConnectivityState {
        self.child_manager
            .aggregate_states_filtered(|c| c.builder.is_active())
    }

    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController) {
        let aggregate_state = self.aggregate_active_states();
        let pickers = self
            .child_manager
            .children()
            .filter(|c| c.builder.is_active())
            .map(|c| (c.identifier.clone(), c.state.picker.clone()))
            .collect::<HashMap<_, _>>();

        let picker_update = LbState {
            connectivity_state: aggregate_state,
            picker: Arc::new(ClusterPicker::new(pickers)),
        };

        channel_controller.update_picker(picker_update);
    }

    // Deactivates child if active, or returns child if already deactivated.
    fn deactivate_child(
        &self,
        child: &Child<String, AnnotatedChildBuilder>,
    ) -> AnnotatedChildBuilder {
        match child.builder.status {
            // New deactivation.
            ChildStatus::Active => {
                let cluster = child.identifier.clone();
                let scheduler = self.work_scheduler.clone();
                let runtime = self.runtime.clone();

                let task_handle = self.runtime.spawn(Box::pin(async move {
                    runtime.sleep(DEFAULT_DEACTIVATION_TIMEOUT).await;
                    scheduler.schedule_work(Some(Box::new(ClusterDeactivationTimeout {
                        cluster_name: cluster,
                    })));
                }));
                let abort_handle = Arc::new(AbortOnDropHandle::new(task_handle));
                AnnotatedChildBuilder::new_deactivated(child.builder.inner.clone(), abort_handle)
            }
            ChildStatus::Deactivated(_) => child.builder.clone(),
        }
    }

    // Prunes the expired cluster from `child_manager` using `retain_children()`.
    fn handle_deactivation_timeout(&mut self, cluster_name: &str) {
        let mut pruned = false; // Did we find any clusters that are expired?
        let remaining: Vec<(String, AnnotatedChildBuilder)> = self
            .child_manager
            .children()
            .filter(|c| {
                let expired = (c.identifier == cluster_name) && c.builder.is_deactivated();
                pruned |= expired;
                !expired
            })
            .map(|c| (c.identifier.clone(), c.builder.clone()))
            .collect();

        // If any clusters were pruned, update the child manager.
        if pruned {
            self.child_manager.retain_children(remaining);
        }
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
        let mut child_updates = Vec::new();

        // Handle existing children omitted in the new config.
        for child in self.child_manager.children() {
            if !config.children.contains_key(&child.identifier) {
                child_updates.push(ChildUpdate {
                    child_identifier: child.identifier.clone(),
                    child_policy_builder: self.deactivate_child(child),
                    child_update: None,
                });
            }
        }

        // Make all new clusters active and forward the resolver update to them.
        // If a cluster was previously Deactivated, passing a new active builder
        // causes the old builder (and its Arc<AbortOnDropHandle>) to be dropped
        // during reset_children, which automatically aborts the background timer.
        for (cluster_name, child_cfg) in &config.children {
            child_updates.push(ChildUpdate {
                child_identifier: cluster_name.clone(),
                child_policy_builder: AnnotatedChildBuilder::new_active(child_cfg.builder.clone()),
                child_update: Some((update.clone(), &child_cfg.config)),
            });
        }

        self.child_manager
            .update(child_updates, channel_controller)?;
        self.update_picker(channel_controller);
        Ok(())
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        let Some(data) = data else {
            if self.child_manager.child_updated() {
                self.update_picker(channel_controller);
            }
            return;
        };

        match data.downcast::<ClusterDeactivationTimeout>() {
            Ok(timeout_event) => {
                self.handle_deactivation_timeout(&timeout_event.cluster_name);
            }
            Err(original_data) => {
                self.child_manager
                    .work(Some(original_data), channel_controller);
                if self.child_manager.child_updated() {
                    self.update_picker(channel_controller);
                }
            }
        }
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager
            .exit_idle_filtered(channel_controller, |c| c.builder.is_active());
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
    fn resolve_cluster<'a>(
        &self,
        attributes: Option<&'a Extensions>,
    ) -> Result<&'a str, StatusError> {
        attributes
            .and_then(|attrs| attrs.get::<XdsCluster>())
            .map(|cluster| cluster.0.as_str())
            .ok_or_else(|| {
                // Todo: should this be INTERNAL?
                StatusError::new(
                    StatusCodeError::Unavailable,
                    "cluster manager: cluster attribute not present",
                )
            })
    }

    // Routes the request to the active child picker for `cluster_name`.
    // If the cluster is unknown or deactivated, fails with UNAVAILABLE status error.
    fn route_to_cluster(&self, cluster_name: &str, request: &RequestHeaders) -> PickResult {
        match self.children.get(cluster_name) {
            Some(picker) => picker.pick(request),
            None => PickResult::Fail(StatusError::new(
                StatusCodeError::Unavailable,
                format!("cluster manager: unknown cluster '{cluster_name}'"),
            )),
        }
    }

    // Picks a child policy using the per-call attributes.
    fn pick_with_attributes(
        &self,
        request: &RequestHeaders,
        attributes: Option<&Extensions>,
    ) -> PickResult {
        match self.resolve_cluster(attributes) {
            Ok(cluster_name) => self.route_to_cluster(cluster_name, request),
            Err(err) => PickResult::Fail(err),
        }
    }
}

impl Picker for ClusterPicker {
    fn pick(&self, request: &RequestHeaders) -> PickResult {
        // TODO: pass the call's attributes here. Need to know the expected API.
        self.pick_with_attributes(request, None)
    }
}

#[cfg(test)]
mod tests {
    use grpc::__unstable::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use grpc::__unstable::client::load_balancing::subchannel::Subchannel;
    use grpc::__unstable::client::load_balancing::subchannel::SubchannelState;
    use grpc::__unstable::rt::default_runtime;
    use grpc::core::Address;

    use super::*;

    // TODO: outstanding test coverage gaps (probably - needs review)
    //
    // Connection preservation across reactivation:
    // - assert the child policy *instance* survives deactivate -> reactivate.
    //   Today the tests only count is_active()/is_deactivated();
    //   `MockChannelController::new_subchannel` is `unimplemented!()` and
    //   `TestDummyLbPolicy` never creates subchannels.
    //
    // Deactivation timer races:
    // - timer fires, then the cluster is re-added before `work()` runs: the
    //   `is_deactivated()` guard in `handle_deactivation_timeout` must suppress
    //   the prune.
    // - the same cluster deactivated, reactivated, and deactivated again.
    // - a timeout naming a cluster that was already pruned.
    //
    // Policy plumbing:
    // - `work()` delegating non-timeout data to the child manager.
    // - `work(None)`.
    // - `child_manager.update()` returning Err.
    // - picker refresh driven by a child connectivity change.
    // - aggregate state when every child is deactivated.
    //
    // Also check whether the registry can be better injected for tests.

    #[test]
    fn policy_builder_name() {
        let builder = ClusterManagerLbBuilder::default();
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
        let builder = ClusterManagerLbBuilder::default();
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
        let builder = ClusterManagerLbBuilder::default();
        let err = builder.parse_config(&parsed_json).unwrap_err();
        assert!(err.contains("no supported child policy"));
    }

    #[derive(Debug)]
    struct DummyPicker;

    impl Picker for DummyPicker {
        fn pick(&self, _request: &RequestHeaders) -> PickResult {
            PickResult::Queue
        }
    }

    // TODO: restore end-to-end picker coverage once per-call attributes reach
    // `Picker::pick`. That needs `CallAttributes` (grpc/grpc-rust#2878) and a
    // way to pass them to `pick()`.
    //
    // Cases to to cover once we have above:
    //
    // - pick with a known cluster attribute delegates to that child's picker
    // - pick with an unknown cluster attribute fails UNAVAILABLE
    // - pick for a deactivated cluster fails UNAVAILABLE (cluster is absent
    //   from the picker map even though the child is still in child_manager)
    // - pick for a reactivated cluster succeeds again
    // - pick after a cluster is pruned on timeout expiry fails UNAVAILABLE
    //
    // The last three were previously asserted inside
    // `test_deactivation_grace_period_retention` and
    // `test_raii_timer_abort_on_reactivation`; those tests now assert only
    // child_manager membership.
    #[test]
    fn cluster_picker_pick_fails_without_attributes() {
        let mut children: HashMap<String, Arc<dyn Picker>> = HashMap::new();
        children.insert("cluster_one".to_string(), Arc::new(DummyPicker));
        let cluster_picker = ClusterPicker { children };

        // `pick` cannot supply attributes, so every request fails regardless of
        // what the channel has configured.
        match cluster_picker.pick(&RequestHeaders::new()) {
            PickResult::Fail(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("not present"));
            }
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    #[test]
    fn cluster_picker_with_attributes_routing() {
        let mut children: HashMap<String, Arc<dyn Picker>> = HashMap::new();
        children.insert("cluster_one".to_string(), Arc::new(DummyPicker));
        let cluster_picker = ClusterPicker { children };

        // Known cluster in attributes -> routes to child picker
        let req = RequestHeaders::new();
        let attrs_for = |cluster: &str| {
            let mut attrs = Extensions::new();
            attrs.insert(XdsCluster(cluster.into()));
            attrs
        };
        let attrs_known = attrs_for("cluster_one");
        match cluster_picker.pick_with_attributes(&req, Some(&attrs_known)) {
            PickResult::Queue => {}
            other => panic!("expected Queue from DummyPicker, got {other:?}"),
        }

        // Unknown cluster in attributes -> UNAVAILABLE
        let attrs_unknown = attrs_for("cluster_unknown");
        match cluster_picker.pick_with_attributes(&req, Some(&attrs_unknown)) {
            PickResult::Fail(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("unknown cluster"));
            }
            other => panic!("expected Fail, got {other:?}"),
        }

        // Empty attributes -> UNAVAILABLE
        let attrs_empty = Extensions::new();
        match cluster_picker.pick_with_attributes(&req, Some(&attrs_empty)) {
            PickResult::Fail(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("not present"));
            }
            other => panic!("expected Fail, got {other:?}"),
        }

        // No attributes at all -> UNAVAILABLE. Nothing else about the request
        // can select a cluster.
        match cluster_picker.pick_with_attributes(&req, None) {
            PickResult::Fail(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(err.message().contains("not present"));
            }
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    #[test]
    fn parse_empty_children_fails() {
        let json_str = serde_json::json!({
            "children": {}
        })
        .to_string();

        let parsed_json = ParsedJsonLbConfig::new(&json_str).expect("parse json");
        let builder = ClusterManagerLbBuilder::default();
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
        let builder = ClusterManagerLbBuilder::default();
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

    #[derive(Debug, Default)]
    struct RecordingScheduler {
        events: std::sync::Mutex<Vec<Option<WorkData>>>,
    }

    impl WorkScheduler for RecordingScheduler {
        fn schedule_work(&self, data: Option<WorkData>) {
            self.events.lock().unwrap().push(data);
        }
    }

    impl RecordingScheduler {
        fn pop_event(&self) -> Option<WorkData> {
            let mut events = self.events.lock().unwrap();
            if events.is_empty() {
                None
            } else {
                events.remove(0)
            }
        }

        fn is_empty(&self) -> bool {
            self.events.lock().unwrap().is_empty()
        }
    }

    /// Advances virtual time by `duration` on the paused Tokio runtime.
    ///
    /// Yields before advancing so that any newly spawned deactivation timer
    /// task runs up to its first `.await`, polling its `sleep` future and
    /// registering on Tokio's timer wheel; a timer that has not registered is
    /// not affected by `advance`. Yields again afterwards so woken timer tasks
    /// can run their continuation (scheduling the timeout work item).
    ///
    /// Note that under `start_paused` the runtime auto-advances the clock
    /// whenever it goes idle, so awaiting anything that is not instantly ready
    /// can jump time forward by the full deactivation timeout.
    async fn advance_time(duration: Duration) {
        tokio::task::yield_now().await;
        tokio::time::advance(duration).await;
        tokio::task::yield_now().await;
    }

    // TODO: cover exit_idle. `TestDummyLbPolicy::exit_idle` is an empty body,
    // so nothing can currently observe which children are woken. Making it
    // record the call would allow asserting:
    //
    // - exit_idle reaches active children
    // - exit_idle skips deactivated children (they are absent from the picker,
    //   so waking them dials backends no RPC can reach, and repeated failures
    //   propagate request_resolution to the channel)
    // - exit_idle reaches a reactivated child again
    //
    // Unlike the picker cases above, this is not blocked on call attributes.
    #[tokio::test]
    async fn test_deactivation_grace_period_retention() {
        GLOBAL_LB_REGISTRY.add_builder(TestDummyLbBuilder);

        let builder = ClusterManagerLbBuilder::default();
        let mut policy = builder.build(LbPolicyOptions {
            work_scheduler: Arc::new(MockScheduler),
            runtime: default_runtime(),
        });

        let mut controller = MockChannelController { latest_state: None };

        // Initial config with cluster_a and cluster_b
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
        let parsed_cfg = builder
            .parse_config(&ParsedJsonLbConfig::from_value(json_2_clusters))
            .unwrap();

        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg, &mut controller)
            .unwrap();

        let state = controller.latest_state.take().expect("state update");
        // Verify ZERO external collections: state is tracked in child_manager
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_active())
                .count(),
            2
        );
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_deactivated())
                .count(),
            0
        );
        assert_eq!(state.connectivity_state, ConnectivityState::Ready);

        // Remove cluster_b -> cluster_b enters deactivation grace period
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

        let state2 = controller.latest_state.take().expect("state update");
        // child_manager still holds BOTH children: cluster_a active, cluster_b deactivated
        assert_eq!(policy.child_manager.children().count(), 2);
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_active())
                .count(),
            1
        );
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_deactivated())
                .count(),
            1
        );
        assert!(
            policy
                .child_manager
                .children()
                .any(|c| c.identifier == "cluster_b" && c.builder.is_deactivated())
        );

        // cluster_b is deactivated, so it is omitted from the picker even
        // though the child is still held by child_manager.
        //
        // TODO: assert this via `state2.picker.pick(...)` instead, once a pick
        // can name a cluster. See the picker TODO above.
        let picker = format!("{:?}", state2.picker);
        assert!(
            picker.contains("cluster_a"),
            "active cluster missing from picker: {picker}"
        );
        assert!(
            !picker.contains("cluster_b"),
            "deactivated cluster present in picker: {picker}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_raii_timer_abort_on_reactivation() {
        GLOBAL_LB_REGISTRY.add_builder(TestDummyLbBuilder);

        let scheduler = Arc::new(RecordingScheduler::default());
        let builder = ClusterManagerLbBuilder::default();
        let mut policy = builder.build(LbPolicyOptions {
            work_scheduler: scheduler.clone(),
            runtime: default_runtime(),
        });

        let mut controller = MockChannelController { latest_state: None };

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

        // Initial config with cluster_a and cluster_b
        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg_2, &mut controller)
            .unwrap();

        // Omit cluster_b -> enters deactivation, spawning timer
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

        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_deactivated())
                .count(),
            1
        );

        // Reactivate cluster_b partway through the grace period.
        advance_time(Duration::from_secs(60)).await;
        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg_2, &mut controller)
            .unwrap();

        // Old deactivated builder dropped in reset_children -> timer aborted via RAII Drop!
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_active())
                .count(),
            2
        );
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_deactivated())
                .count(),
            0
        );

        // Advance past the original timer's deadline.
        advance_time(DEFAULT_DEACTIVATION_TIMEOUT).await;

        // Verify NO deactivation timeout event was scheduled to work_scheduler
        assert!(scheduler.is_empty());

        // Both clusters are active, so both are back in the picker.
        //
        // TODO: assert this via `state.picker.pick(...)` instead, once a pick
        // can name a cluster. See the picker TODO above.
        let state = controller.latest_state.take().expect("state update");
        let picker = format!("{:?}", state.picker); // TODO: fix this hack when we have call attributes.  Shuoldn't use format like this.
        assert!(
            picker.contains("cluster_a") && picker.contains("cluster_b"),
            "reactivated cluster missing from picker: {picker}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout_expiration_prunes_child() {
        GLOBAL_LB_REGISTRY.add_builder(TestDummyLbBuilder);

        let scheduler = Arc::new(RecordingScheduler::default());
        let builder = ClusterManagerLbBuilder::default();
        let mut policy = builder.build(LbPolicyOptions {
            work_scheduler: scheduler.clone(),
            runtime: default_runtime(),
        });

        let mut controller = MockChannelController { latest_state: None };

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

        // Initial config with cluster_a and cluster_b
        policy
            .resolver_update(ResolverUpdate::default(), &parsed_cfg_2, &mut controller)
            .unwrap();

        // Omit cluster_b -> enters deactivation
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

        // Advance past the deactivation deadline so the timer fires.
        advance_time(DEFAULT_DEACTIVATION_TIMEOUT + Duration::from_secs(1)).await;

        // Pop scheduled timeout work
        let event = scheduler
            .pop_event()
            .expect("expected scheduled timeout work");
        let timeout_data = event
            .downcast::<ClusterDeactivationTimeout>()
            .expect("expected ClusterDeactivationTimeout payload");
        assert_eq!(timeout_data.cluster_name, "cluster_b");

        // Dispatch work event to policy -> prunes cluster_b from child_manager via retain_children
        policy.work(Some(timeout_data), &mut controller);

        // Verify cluster_b has been pruned completely from child_manager
        assert_eq!(policy.child_manager.children().count(), 1);
        assert_eq!(
            policy.child_manager.children().next().unwrap().identifier,
            "cluster_a"
        );
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_active())
                .count(),
            1
        );
        assert_eq!(
            policy
                .child_manager
                .children()
                .filter(|c| c.builder.is_deactivated())
                .count(),
            0
        );
    }
}
