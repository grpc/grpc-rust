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

//! Implementation of the `xds_cluster_impl_experimental` Load Balancing policy.
//!
//! The `xds_cluster_impl` LB policy ([gRFC A27], [gRFC A32], [gRFC A37],
//! [gRFC A74]) wraps a single child LB policy for a leaf xDS cluster (EDS or
//! `LOGICAL_DNS`) and enforces cluster-level data-plane policies on each RPC:
//! - EDS probabilistic drop overloads (`ClusterLoadAssignment.policy.drop_overloads`)
//! - Circuit breaking (`max_concurrent_requests`, keyed globally by
//!   `{cluster, eds_service_name}`)
//!
//! [gRFC A27]: https://github.com/grpc/proposal/blob/master/A27-xds-global-load-balancing.md
//! [gRFC A32]: https://github.com/grpc/proposal/blob/master/A32-xds-circuit-breaking.md
//! [gRFC A37]: https://github.com/grpc/proposal/blob/master/A37-xds-aggregate-and-logical-dns-clusters.md
//! [gRFC A74]: https://github.com/grpc/proposal/blob/master/A74-xds-config-tears.md
// TODO: implement LRS load reporting (`ClusterDropStats` and per-locality
// `ClusterLocalityStats`) once the LRS client exists in `xds-client`. LRS
// `cluster_service_name` must use `ClusterDiscovery::Eds::raw_eds_service_name`
// (the possibly-empty CDS value), not the resolved `eds_service_name`.
// TODO: plumb cluster transport security config (`transport_socket`) into
// subchannel attributes once `XdsCredentials` is implemented.
// TODO: implement `:authority` host rewriting (`auto_host_rewrite`) once
// `RouteAction`, `LbEndpoint`, and `Pick` support authority overrides.
// TODO: attach `grpc.lb.locality` and `grpc.lb.backend_service` OpenTelemetry
// labels once the OTel metrics plugin infrastructure exists.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::Weak;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

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
use grpc::__unstable::client::load_balancing::WorkScheduler;
use grpc::__unstable::client::load_balancing::graceful_switch::GracefulSwitchLbConfig;
use grpc::__unstable::client::load_balancing::graceful_switch::GracefulSwitchPolicy;
use grpc::__unstable::client::load_balancing::registry::GLOBAL_LB_REGISTRY;
use grpc::__unstable::client::load_balancing::subchannel::Subchannel;
use grpc::__unstable::client::load_balancing::subchannel::SubchannelState;
use grpc::__unstable::client::name_resolution::ResolverUpdate;
use grpc::StatusCodeError;
use grpc::StatusError;
use grpc::client::ConnectivityState;
use grpc::core::Address;
use serde::Deserialize;

use crate::resource::ClusterDiscovery;
use crate::resource::DropOverload;
use crate::xds_config::ClusterChildren;
use crate::xds_config::LeafEndpointSource;
use crate::xds_config::XdsConfig;

static POLICY_NAME: &str = "xds_cluster_impl_experimental";

const MILLION: u32 = 1_000_000;

/// Key identifying a cluster's shared circuit-breaker call counter in the
/// process-global counter map (gRFC A32).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CounterKey {
    cluster: String,
    eds_service_name: String,
}

/// Process-global map of active circuit-breaker counters keyed by
/// `{cluster, eds_service_name}` so concurrent request limits are enforced
/// across all `xds_cluster_impl` instances and channels targeting the same
/// cluster (gRFC A32).
static CIRCUIT_BREAKER_COUNTERS: LazyLock<Mutex<HashMap<CounterKey, Weak<CallCounter>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Shared atomic counter tracking in-flight RPCs for a `{cluster, eds_service_name}` pair.
#[derive(Debug)]
struct CallCounter {
    key: CounterKey,
    in_flight: AtomicU32,
}

impl CallCounter {
    fn get_or_create(cluster: &str, eds_service_name: &str) -> Arc<Self> {
        let key = CounterKey {
            cluster: cluster.to_string(),
            eds_service_name: eds_service_name.to_string(),
        };
        let mut map = CIRCUIT_BREAKER_COUNTERS
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(existing) = map.get(&key).and_then(Weak::upgrade) {
            return existing;
        }
        let counter = Arc::new(Self {
            key: key.clone(),
            in_flight: AtomicU32::new(0),
        });
        map.insert(key, Arc::downgrade(&counter));
        counter
    }

    /// Atomically increments the in-flight counter if it is strictly below
    /// `max_concurrent_requests`. Returns `true` if capacity was acquired.
    fn try_acquire(&self, max_concurrent_requests: u32) -> bool {
        let mut curr = self.in_flight.load(Ordering::Acquire);
        loop {
            if curr >= max_concurrent_requests {
                return false;
            }
            match self.in_flight.compare_exchange_weak(
                curr,
                curr + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(actual) => curr = actual,
            }
        }
    }

    /// Decrements the in-flight counter. Only called by [`InFlightGuard`],
    /// which guarantees it happens exactly once per acquired slot.
    fn release(&self) {
        self.in_flight.fetch_sub(1, Ordering::AcqRel);
    }

    #[cfg(test)]
    fn load(&self) -> u32 {
        self.in_flight.load(Ordering::Acquire)
    }
}

impl Drop for CallCounter {
    fn drop(&mut self) {
        let mut map = CIRCUIT_BREAKER_COUNTERS
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if map.get(&self.key).is_some_and(|w| w.strong_count() == 0) {
            map.remove(&self.key);
        }
    }
}

/// One unit of circuit-breaker capacity held by an in-flight RPC.
///
/// The slot is returned to the [`CallCounter`] when the guard is dropped. The
/// guard is owned by the pick's `FnOnce` completion callback, so this happens
/// exactly once: either when the callback runs (the RPC completed), or when
/// the callback is discarded without running (the RPC was abandoned before it
/// started).
#[derive(Debug)]
struct InFlightGuard {
    counter: Arc<CallCounter>,
}

impl InFlightGuard {
    /// Acquires a slot from `counter` if fewer than `max_concurrent_requests`
    /// RPCs are in flight.
    fn try_acquire(counter: &Arc<CallCounter>, max_concurrent_requests: u32) -> Option<Self> {
        counter.try_acquire(max_concurrent_requests).then(|| Self {
            counter: Arc::clone(counter),
        })
    }
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        self.counter.release();
    }
}

/// Validated configuration for `xds_cluster_impl_experimental` (post-gRFC A74).
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterImplConfig {
    cluster: String,
    child_policy: ParsedLbConfig,
}

#[derive(Debug, Default)]
struct ClusterImplBuilder;

impl LbPolicyBuilder for ClusterImplBuilder {
    type LbPolicy = ClusterImplPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        ClusterImplPolicy::new(options)
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &LbConfigJson) -> Result<ClusterImplConfig, String> {
        let parsed: ClusterImplConfig = config
            .convert_to()
            .map_err(|e| format!("failed to deserialize xds_cluster_impl config: {e}"))?;

        if parsed.cluster.is_empty() {
            return Err(
                "failed to parse xds_cluster_impl config: 'cluster' must be non-empty".to_string(),
            );
        }

        Ok(parsed)
    }
}

/// Resolved per-cluster data extracted from [`XdsConfig`].
#[derive(Debug, Clone)]
struct ClusterData {
    call_counter: Arc<CallCounter>,
    max_concurrent_requests: u32,
    drop_overloads: Arc<[DropOverload]>,
}

/// An instance of the `xds_cluster_impl_experimental` Load Balancing policy.
#[derive(Debug)]
struct ClusterImplPolicy {
    child: GracefulSwitchPolicy,
    child_state: LbState,
    cluster_data: Option<ClusterData>,
}

impl ClusterImplPolicy {
    fn new(options: LbPolicyOptions) -> Self {
        Self {
            child: GracefulSwitchPolicy::new(options.runtime, options.work_scheduler),
            child_state: LbState::initial(),
            cluster_data: None,
        }
    }

    fn extract_cluster_data(
        &self,
        update: &ResolverUpdate,
        cluster_name: &str,
    ) -> Result<ClusterData, String> {
        let xds_config = update
            .attributes
            .get::<XdsConfig>()
            .ok_or_else(|| "xds_cluster_impl: missing XdsConfig attribute".to_string())?;

        let cluster_result = xds_config.clusters.get(cluster_name).ok_or_else(|| {
            format!("xds_cluster_impl: cluster '{cluster_name}' not found in XdsConfig")
        })?;

        let cluster_config = cluster_result.as_ref().map_err(|err| {
            format!("xds_cluster_impl: cluster '{cluster_name}' failed to resolve: {err}")
        })?;

        let ClusterChildren::Leaf(leaf) = &cluster_config.children else {
            return Err(format!(
                "xds_cluster_impl: cluster '{cluster_name}' is an aggregate cluster, expected leaf cluster"
            ));
        };

        // The counter is keyed by the raw CDS `eds_service_name` (empty if
        // unset), matching other gRPC implementations (gRFC A32).
        let eds_service_name = match &cluster_config.cluster.discovery {
            ClusterDiscovery::Eds {
                raw_eds_service_name,
                ..
            } => raw_eds_service_name.as_str(),
            ClusterDiscovery::LogicalDns { .. } => "",
            ClusterDiscovery::Aggregate { .. } => {
                return Err(format!(
                    "xds_cluster_impl: cluster '{cluster_name}' has aggregate discovery type"
                ));
            }
        };

        let call_counter = match &self.cluster_data {
            Some(existing)
                if existing.call_counter.key.cluster == cluster_name
                    && existing.call_counter.key.eds_service_name == eds_service_name =>
            {
                Arc::clone(&existing.call_counter)
            }
            _ => CallCounter::get_or_create(cluster_name, eds_service_name),
        };

        let max_concurrent_requests = cluster_config.cluster.max_concurrent_requests;
        let drop_overloads: Arc<[DropOverload]> = match &leaf.source {
            Some(LeafEndpointSource::Eds(eds)) => Arc::from(eds.drop_overloads.as_slice()),
            Some(LeafEndpointSource::LogicalDns(_)) | None => Arc::from([]),
        };

        Ok(ClusterData {
            call_counter,
            max_concurrent_requests,
            drop_overloads,
        })
    }

    fn update_picker(&self, channel_controller: &mut dyn ChannelController) {
        let Some(cluster_data) = &self.cluster_data else {
            return;
        };
        let picker = Arc::new(ClusterImplPicker {
            child_picker: Arc::clone(&self.child_state.picker),
            drop_overloads: Arc::clone(&cluster_data.drop_overloads),
            max_concurrent_requests: cluster_data.max_concurrent_requests,
            call_counter: Arc::clone(&cluster_data.call_counter),
        });
        channel_controller.update_picker(LbState {
            connectivity_state: self.child_state.connectivity_state,
            picker,
        });
    }
}

impl LbPolicy for ClusterImplPolicy {
    type LbConfig = ClusterImplConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let cluster_data = match self.extract_cluster_data(&update, &config.cluster) {
            Ok(data) => data,
            Err(err) => {
                // The child policy is intentionally left running so it can
                // resume without reconnecting once a valid update arrives.
                // While `cluster_data` is `None`, `update_picker` ignores the
                // child's picker updates, so the failure picker stays in place.
                self.cluster_data = None;
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::TransientFailure,
                    picker: Arc::new(FailurePicker { error: err.clone() }),
                });
                return Err(err);
            }
        };
        self.cluster_data = Some(cluster_data);

        let gs_config = GracefulSwitchLbConfig::new(
            Arc::clone(&config.child_policy.builder),
            Arc::clone(&config.child_policy.config),
        );
        // Whether the child updated its picker doesn't matter here: a new
        // picker is always published below.
        let mut _child_updated = false;
        let result = {
            let mut intercepting = InterceptingController {
                inner: channel_controller,
                child_state: &mut self.child_state,
                updated: &mut _child_updated,
            };
            self.child
                .resolver_update(update, &gs_config, &mut intercepting)
        };

        // Always publish a picker on resolver_update so changes to
        // `drop_overloads`, `max_concurrent_requests`, or `call_counter` take
        // effect immediately even if the child policy did not replace its picker.
        self.update_picker(channel_controller);
        result
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        let mut child_updated = false;
        {
            let mut intercepting = InterceptingController {
                inner: channel_controller,
                child_state: &mut self.child_state,
                updated: &mut child_updated,
            };
            self.child.work(data, &mut intercepting);
        }
        if child_updated {
            self.update_picker(channel_controller);
        }
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        let mut child_updated = false;
        {
            let mut intercepting = InterceptingController {
                inner: channel_controller,
                child_state: &mut self.child_state,
                updated: &mut child_updated,
            };
            self.child.exit_idle(&mut intercepting);
        }
        if child_updated {
            self.update_picker(channel_controller);
        }
    }
}

/// [`ChannelController`] wrapper that intercepts `update_picker` calls from the
/// child policy so `ClusterImplPolicy` can wrap the child's picker in a
/// [`ClusterImplPicker`].
struct InterceptingController<'a> {
    inner: &'a mut dyn ChannelController,
    child_state: &'a mut LbState,
    updated: &'a mut bool,
}

impl ChannelController for InterceptingController<'_> {
    fn new_subchannel(
        &mut self,
        address: &Address,
        work_scheduler: Arc<dyn WorkScheduler>,
    ) -> (Arc<dyn Subchannel>, SubchannelState) {
        // TODO: attach cluster transport security configuration to subchannel
        // address attributes once XdsCredentials is implemented.
        self.inner.new_subchannel(address, work_scheduler)
    }

    fn update_picker(&mut self, update: LbState) {
        *self.child_state = update;
        *self.updated = true;
    }

    fn request_resolution(&mut self) {
        self.inner.request_resolution();
    }
}

/// Picker that enforces EDS drops and circuit breaking around a child picker.
#[derive(Debug)]
struct ClusterImplPicker {
    child_picker: Arc<dyn Picker>,
    drop_overloads: Arc<[DropOverload]>,
    max_concurrent_requests: u32,
    call_counter: Arc<CallCounter>,
}

impl Picker for ClusterImplPicker {
    fn pick(&self, options: PickOptions<'_>) -> PickResult {
        // 1. EDS drop check: evaluate categories sequentially.
        for drop_overload in self.drop_overloads.iter() {
            if should_drop(drop_overload.requests_per_million) {
                // TODO: record categorized drop via LRS `ClusterDropStats::add_call_dropped`
                // once LRS client support is implemented.
                return PickResult::Drop(StatusError::new(
                    StatusCodeError::Unavailable,
                    format!("EDS-configured drop: {}", drop_overload.category),
                ));
            }
        }

        // 2. Delegate to child picker. Non-Pick results (Queue, Fail, Drop) do
        //    not consume circuit-breaker capacity.
        let mut pick = match self.child_picker.pick(options) {
            PickResult::Pick(pick) => pick,
            other => return other,
        };

        // 3. Circuit breaking check.
        //
        // gRFC A32 specifies that the counter is incremented when the call
        // starts on the subchannel. The slot is acquired here, at pick time,
        // because the picker is the last point where the call can still be
        // rejected. If the pick is abandoned before the call starts (e.g. the
        // RPC is cancelled while the channel is starting the stream), the
        // returned `Pick` and its completion callback are dropped, and
        // `InFlightGuard`'s `Drop` returns the slot.
        let Some(in_flight) =
            InFlightGuard::try_acquire(&self.call_counter, self.max_concurrent_requests)
        else {
            // TODO: record uncategorized drop via LRS
            // `ClusterDropStats::add_uncategorized_drops` once LRS client
            // support is implemented.
            //
            // Per gRFC A32, circuit breaker rejections are not retried and
            // must fail even wait-for-ready RPCs (completing an in-flight RPC
            // does not publish a new picker, so a queued RPC could otherwise
            // wait indefinitely). The child's `on_complete`, if any, is
            // dropped without being invoked: the call never started.
            return PickResult::Drop(StatusError::new(
                StatusCodeError::Unavailable,
                "circuit breaker max_concurrent_requests reached",
            ));
        };

        // 4. Wrap the completion callback to release the in-flight slot when
        //    the RPC finishes, preserving any callback set by the child. If
        //    the callback is dropped without being invoked, dropping the
        //    captured guard still releases the slot.
        // TODO: record locality call completion in LRS `ClusterLocalityStats`
        // using `outcome.status()` and ORCA metrics from
        // `outcome.trailing_metadata()`, apply `:authority` host rewrite if
        // enabled, and attach OTel `grpc.lb.locality` /
        // `grpc.lb.backend_service` labels.
        let child_on_complete = pick.on_complete.take();
        pick.on_complete = Some(Box::new(move |outcome| {
            drop(in_flight);
            if let Some(cb) = child_on_complete {
                cb(outcome);
            }
        }));

        PickResult::Pick(pick)
    }
}

fn should_drop(requests_per_million: u32) -> bool {
    if requests_per_million == 0 {
        return false;
    }
    if requests_per_million >= MILLION {
        return true;
    }
    should_drop_with_sample(requests_per_million, rand::random_range(0..MILLION))
}

/// Returns whether a request should be dropped given a uniformly random
/// `sample` in `[0, 1_000_000)`.
fn should_drop_with_sample(requests_per_million: u32, sample: u32) -> bool {
    sample < requests_per_million
}

#[derive(Debug)]
struct FailurePicker {
    error: String,
}

impl Picker for FailurePicker {
    fn pick(&self, _options: PickOptions<'_>) -> PickResult {
        PickResult::Fail(StatusError::new(
            StatusCodeError::Unavailable,
            self.error.clone(),
        ))
    }
}

/// Registers `xds_cluster_impl_experimental` in the global LB policy registry.
pub(crate) fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(ClusterImplBuilder);
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::any::TypeId;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::AtomicUsize;

    use grpc::__unstable::client::load_balancing::CallOutcome;
    use grpc::__unstable::client::load_balancing::CompletionCallback;
    use grpc::__unstable::client::load_balancing::Pick;
    use grpc::__unstable::client::load_balancing::round_robin::POLICY_NAME as RR_POLICY_NAME;
    use grpc::__unstable::client::load_balancing::subchannel::ForwardingSubchannel;
    use grpc::__unstable::client::name_resolution::Endpoint;
    use grpc::__unstable::rt::default_runtime;
    use grpc::attributes::Attributes;
    use grpc::client::CallOptions;
    use grpc::client::RequestHeaders;
    use grpc::metadata::MetadataMap;

    use super::*;
    use crate::resource::ClusterResource;
    use crate::resource::DEFAULT_MAX_CONCURRENT_REQUESTS;
    use crate::resource::DomainMatcher;
    use crate::resource::EndpointsResource;
    use crate::resource::ListenerResource;
    use crate::resource::RouteConfigResource;
    use crate::resource::RouteSource;
    use crate::resource::VirtualHost;
    use crate::xds_config::AggregateClusters;
    use crate::xds_config::ClusterConfig;
    use crate::xds_config::ClusterResolutionError;
    use crate::xds_config::ClusterResult;
    use crate::xds_config::LeafEndpoints;

    #[test]
    fn parse_config_valid_and_invalid() {
        let builder = ClusterImplBuilder;
        assert_eq!(builder.name(), "xds_cluster_impl_experimental");

        let valid_json = serde_json::json!({
            "cluster": "cluster_a",
            "childPolicy": [{ RR_POLICY_NAME: {} }]
        });
        let config = builder
            .parse_config(&LbConfigJson::new(&valid_json.to_string()).unwrap())
            .expect("valid config");
        assert_eq!(config.cluster, "cluster_a");
        assert_eq!(config.child_policy.builder.name(), RR_POLICY_NAME);

        // Empty cluster name must be rejected.
        let empty_cluster = serde_json::json!({
            "cluster": "",
            "childPolicy": [{ RR_POLICY_NAME: {} }]
        });
        let err = builder
            .parse_config(&LbConfigJson::new(&empty_cluster.to_string()).unwrap())
            .unwrap_err();
        assert!(err.contains("'cluster' must be non-empty"), "got: {err}");

        // Missing cluster field must be rejected.
        let missing_cluster = serde_json::json!({
            "childPolicy": [{ RR_POLICY_NAME: {} }]
        });
        assert!(
            builder
                .parse_config(&LbConfigJson::new(&missing_cluster.to_string()).unwrap())
                .is_err()
        );

        // Empty childPolicy list must be rejected.
        let empty_child = serde_json::json!({
            "cluster": "cluster_a",
            "childPolicy": []
        });
        assert!(
            builder
                .parse_config(&LbConfigJson::new(&empty_child.to_string()).unwrap())
                .is_err()
        );
    }

    #[test]
    fn missing_or_invalid_xds_config_reports_transient_failure() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_a",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        // 1. Missing XdsConfig attribute.
        let err = policy
            .resolver_update(ResolverUpdate::default(), &cfg, &mut controller)
            .unwrap_err();
        assert!(err.contains("missing XdsConfig attribute"), "got: {err}");
        let state = controller.latest_state.take().expect("state");
        assert_eq!(
            state.connectivity_state,
            ConnectivityState::TransientFailure
        );
        assert_fails_unavailable(&state.picker, "missing XdsConfig attribute");

        // 2. Cluster absent from XdsConfig.
        let update = update_with_xds_config(HashMap::new());
        let err = policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap_err();
        assert!(err.contains("not found in XdsConfig"), "got: {err}");
        let state = controller.latest_state.take().expect("state");
        assert_eq!(
            state.connectivity_state,
            ConnectivityState::TransientFailure
        );
        assert_fails_unavailable(&state.picker, "not found in XdsConfig");

        // 3. Cluster entry is an Err in XdsConfig.
        let mut clusters = HashMap::new();
        clusters.insert(
            "cluster_a".to_string(),
            Err(ClusterResolutionError::new("CDS validation NACK")),
        );
        let update = update_with_xds_config(clusters);
        let err = policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap_err();
        assert!(err.contains("CDS validation NACK"), "got: {err}");
        let state = controller.latest_state.take().expect("state");
        assert_eq!(
            state.connectivity_state,
            ConnectivityState::TransientFailure
        );
        assert_fails_unavailable(&state.picker, "CDS validation NACK");

        // 4. Cluster entry is an Aggregate cluster instead of a Leaf cluster.
        let mut clusters = HashMap::new();
        clusters.insert(
            "cluster_a".to_string(),
            Ok(Arc::new(ClusterConfig {
                cluster: Arc::new(ClusterResource {
                    name: "cluster_a".to_string(),
                    discovery: ClusterDiscovery::Aggregate {
                        children: vec!["leaf_1".to_string()],
                    },
                    max_concurrent_requests: DEFAULT_MAX_CONCURRENT_REQUESTS,
                }),
                children: ClusterChildren::Aggregate(AggregateClusters {
                    leaf_clusters: vec!["leaf_1".to_string()],
                    resolution_note: None,
                }),
            })),
        );
        let update = update_with_xds_config(clusters);
        let err = policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap_err();
        assert!(err.contains("aggregate cluster"), "got: {err}");
        let state = controller.latest_state.take().expect("state");
        assert_eq!(
            state.connectivity_state,
            ConnectivityState::TransientFailure
        );
        assert_fails_unavailable(&state.picker, "aggregate cluster");
    }

    #[test]
    fn eds_drop_enforcement() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_eds_drop",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        // Configure two drop categories: first has 0 ppm (never drops), second
        // has 1,000,000 ppm (always drops).
        let update = update_with_leaf_eds_cluster(
            "cluster_eds_drop",
            "eds_svc",
            100,
            vec![
                DropOverload {
                    category: "never_drop".to_string(),
                    requests_per_million: 0,
                },
                DropOverload {
                    category: "throttle".to_string(),
                    requests_per_million: 1_000_000,
                },
                DropOverload {
                    category: "later_category".to_string(),
                    requests_per_million: 1_000_000,
                },
            ],
        );

        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");
        assert_eq!(state.connectivity_state, ConnectivityState::Ready);

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        match state.picker.pick(PickOptions::new(&req, &opts)) {
            PickResult::Drop(status) => {
                assert_eq!(status.code(), StatusCodeError::Unavailable);
                assert_eq!(status.message(), "EDS-configured drop: throttle");
            }
            other => panic!("expected PickResult::Drop, got {other:?}"),
        }

        // Verify circuit breaker counter was not incremented by the dropped RPC.
        let counter = CallCounter::get_or_create("cluster_eds_drop", "eds_svc");
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn circuit_breaking_enforces_max_concurrent_requests_and_chains_on_complete() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let child_completed = Arc::new(AtomicUsize::new(0));
        let cfg = parse_test_config(
            "cluster_cb",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        let update = update_with_leaf_eds_cluster("cluster_cb", "eds_cb", 2, Vec::new());
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");
        assert_eq!(state.connectivity_state, ConnectivityState::Ready);

        let req = RequestHeaders::new();
        let mut opts = CallOptions::new();
        opts.set_option(ChildCompletionTracker(Arc::clone(&child_completed)));

        // Pick 1 succeeds (in-flight = 1).
        let pick1 = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        // Pick 2 succeeds (in-flight = 2, reaching max_concurrent_requests = 2).
        let pick2 = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        // Pick 3 exceeds max_concurrent_requests = 2 -> PickResult::Drop(UNAVAILABLE).
        match state.picker.pick(PickOptions::new(&req, &opts)) {
            PickResult::Drop(status) => {
                assert_eq!(status.code(), StatusCodeError::Unavailable);
                assert!(
                    status.message().contains("max_concurrent_requests"),
                    "got: {}",
                    status.message()
                );
            }
            other => panic!("expected PickResult::Drop, got {other:?}"),
        }
        // The rejected pick's child callback is discarded, not invoked.
        assert_eq!(child_completed.load(Ordering::Acquire), 0);

        // Complete pick 1: in-flight drops to 1 and child's on_complete runs.
        complete_ok(pick1);
        assert_eq!(child_completed.load(Ordering::Acquire), 1);

        // Now another pick succeeds (in-flight = 2 again).
        let pick3 = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        // Complete remaining picks.
        complete_ok(pick2);
        complete_ok(pick3);
        assert_eq!(child_completed.load(Ordering::Acquire), 3);

        let counter = CallCounter::get_or_create("cluster_cb", "eds_cb");
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn circuit_breaker_zero_limit_rejects_all_picks() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_zero_cb",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        let update = update_with_leaf_eds_cluster("cluster_zero_cb", "eds_zero", 0, Vec::new());
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        match state.picker.pick(PickOptions::new(&req, &opts)) {
            PickResult::Drop(status) => {
                assert_eq!(status.code(), StatusCodeError::Unavailable);
            }
            other => panic!("expected PickResult::Drop, got {other:?}"),
        }
    }

    #[test]
    fn non_pick_child_results_do_not_increment_circuit_breaker() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_queue",
            "test_cluster_impl_child",
            serde_json::json!({ "start_connecting": true }),
        );

        let update = update_with_leaf_eds_cluster("cluster_queue", "eds_queue", 1, Vec::new());
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");
        assert_eq!(state.connectivity_state, ConnectivityState::Connecting);

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        // Even with max_concurrent_requests = 1, multiple queued picks must not
        // consume circuit breaker budget.
        for _ in 0..5 {
            assert!(matches!(
                state.picker.pick(PickOptions::new(&req, &opts)),
                PickResult::Queue
            ));
        }
        let counter = CallCounter::get_or_create("cluster_queue", "eds_queue");
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn global_counter_shared_across_policy_instances_and_cleaned_up_on_drop() {
        let (mut policy1, _sched1, mut ctrl1) = setup_test_policy();
        let (mut policy2, _sched2, mut ctrl2) = setup_test_policy();
        let (mut policy_other, _sched3, mut ctrl_other) = setup_test_policy();

        let cfg_shared = parse_test_config(
            "cluster_shared",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let cfg_other = parse_test_config(
            "cluster_other",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        // Both policy1 and policy2 target ("cluster_shared", "eds_shared") with limit 1.
        let update_shared =
            update_with_leaf_eds_cluster("cluster_shared", "eds_shared", 1, Vec::new());
        policy1
            .resolver_update(update_shared.clone(), &cfg_shared, &mut ctrl1)
            .unwrap();
        policy2
            .resolver_update(update_shared, &cfg_shared, &mut ctrl2)
            .unwrap();

        let update_other =
            update_with_leaf_eds_cluster("cluster_other", "eds_other", 1, Vec::new());
        policy_other
            .resolver_update(update_other, &cfg_other, &mut ctrl_other)
            .unwrap();

        let state1 = ctrl1.latest_state.take().expect("state1");
        let state2 = ctrl2.latest_state.take().expect("state2");
        let state_other = ctrl_other.latest_state.take().expect("state_other");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();

        // Pick on policy1 acquires the single slot for ("cluster_shared", "eds_shared").
        let pick1 = state1
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        // Pick on policy2 is rejected because it shares the counter with policy1!
        assert!(matches!(
            state2.picker.pick(PickOptions::new(&req, &opts)),
            PickResult::Drop(_)
        ));

        // Pick on policy_other succeeds because it uses a distinct counter key.
        let pick_other = state_other
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        complete_ok(pick1);
        complete_ok(pick_other);

        // After releasing pick1, policy2 can now pick.
        let pick2 = state2
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        complete_ok(pick2);

        // Drop all policies and states and verify the global map removes the keys.
        drop(state1);
        drop(state2);
        drop(state_other);
        drop(policy1);
        drop(policy2);
        drop(policy_other);

        let map = CIRCUIT_BREAKER_COUNTERS.lock().unwrap();
        assert!(
            !map.contains_key(&CounterKey {
                cluster: "cluster_shared".to_string(),
                eds_service_name: "eds_shared".to_string(),
            }),
            "counter entry should be removed when all references are dropped"
        );
        assert!(
            !map.contains_key(&CounterKey {
                cluster: "cluster_other".to_string(),
                eds_service_name: "eds_other".to_string(),
            }),
            "counter entry should be removed when all references are dropped"
        );
    }

    #[test]
    fn dynamic_xds_config_update_refreshes_picker_parameters() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_dyn",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );

        // Initial update: max_concurrent_requests = 1, no drops.
        let update1 = update_with_leaf_eds_cluster("cluster_dyn", "eds_dyn", 1, Vec::new());
        policy
            .resolver_update(update1, &cfg, &mut controller)
            .unwrap();
        let state1 = controller.latest_state.take().expect("state1");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick1 = state1
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        assert!(matches!(
            state1.picker.pick(PickOptions::new(&req, &opts)),
            PickResult::Drop(_)
        ));

        // Second update raises max_concurrent_requests to 2.
        let update2 = update_with_leaf_eds_cluster("cluster_dyn", "eds_dyn", 2, Vec::new());
        policy
            .resolver_update(update2, &cfg, &mut controller)
            .unwrap();
        let state2 = controller.latest_state.take().expect("state2");

        // While pick1 is still in flight, state2 allows a second concurrent pick!
        let pick2 = state2
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        assert!(matches!(
            state2.picker.pick(PickOptions::new(&req, &opts)),
            PickResult::Drop(_)
        ));

        complete_ok(pick1);
        complete_ok(pick2);
    }

    #[test]
    fn work_and_exit_idle_delegate_to_child_and_wrap_picker() {
        let (mut policy, scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_lifecycle",
            "test_cluster_impl_child",
            serde_json::json!({ "start_idle": true }),
        );

        let update =
            update_with_leaf_eds_cluster("cluster_lifecycle", "eds_lifecycle", 10, Vec::new());
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let initial = controller.latest_state.take().expect("initial state");
        assert_eq!(initial.connectivity_state, ConnectivityState::Idle);

        // exit_idle wakes the child -> transitions to Connecting and schedules work.
        policy.exit_idle(&mut controller);
        let connecting = controller.latest_state.take().expect("connecting state");
        assert_eq!(connecting.connectivity_state, ConnectivityState::Connecting);

        // Running scheduled work transitions child to Ready -> wrapped picker is Ready.
        let work_item = scheduler
            .queued_work
            .lock()
            .unwrap()
            .pop()
            .expect("scheduled work");
        policy.work(work_item, &mut controller);
        let ready = controller.latest_state.take().expect("ready state");
        assert_eq!(ready.connectivity_state, ConnectivityState::Ready);

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick = ready
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        complete_ok(pick);
    }

    #[test]
    fn completed_or_abandoned_pick_releases_slot() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_release_once",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let update =
            update_with_leaf_eds_cluster("cluster_release_once", "eds_release_once", 2, Vec::new());
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick1 = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        let pick2 = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        let counter = CallCounter::get_or_create("cluster_release_once", "eds_release_once");
        assert_eq!(counter.load(), 2);

        // Invoking the completion callback releases the slot.
        complete_ok(pick1);
        assert_eq!(counter.load(), 1);

        // Dropping a pick without invoking its completion callback (e.g. an
        // abandoned RPC) must still release its slot.
        drop(pick2);
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn child_callback_receives_call_outcome() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_child_outcome",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let update = update_with_leaf_eds_cluster(
            "cluster_child_outcome",
            "eds_child_outcome",
            10,
            Vec::new(),
        );
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let child_completed = Arc::new(AtomicUsize::new(0));
        let req = RequestHeaders::new();
        let mut opts = CallOptions::new();
        opts.set_option(ChildCompletionTracker(Arc::clone(&child_completed)));
        let recorder = ChildOutcomeRecorder::default();
        opts.set_option(recorder.clone());

        // A failed RPC with trailers.
        let pick = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        let status = Err(StatusError::new(StatusCodeError::ResourceExhausted, "busy"));
        let trailers = MetadataMap::new();
        pick.on_complete.expect("on_complete")(
            &CallOutcome::new(&status).with_trailing_metadata(&trailers),
        );

        // A successful RPC without trailing metadata.
        let pick = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        complete_ok(pick);

        assert_eq!(child_completed.load(Ordering::Acquire), 2);
        assert_eq!(
            *recorder.0.lock().unwrap(),
            vec![
                (Some(StatusCodeError::ResourceExhausted), true),
                (None, false)
            ]
        );
    }

    #[test]
    fn should_drop_with_sample_boundaries() {
        assert!(!should_drop_with_sample(0, 0));
        assert!(should_drop_with_sample(1, 0));
        assert!(!should_drop_with_sample(1, 1));
        assert!(should_drop_with_sample(500_000, 499_999));
        assert!(!should_drop_with_sample(500_000, 500_000));
        assert!(should_drop_with_sample(MILLION, MILLION - 1));
    }

    #[test]
    fn eds_drop_rate_is_probabilistic() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_drop_rate",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let update = update_with_leaf_eds_cluster(
            "cluster_drop_rate",
            "eds_drop_rate",
            DEFAULT_MAX_CONCURRENT_REQUESTS,
            vec![DropOverload {
                category: "half".to_string(),
                requests_per_million: 500_000,
            }],
        );
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        const PICKS: usize = 10_000;
        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let mut dropped = 0;
        for _ in 0..PICKS {
            match state.picker.pick(PickOptions::new(&req, &opts)) {
                PickResult::Drop(_) => dropped += 1,
                // Completing immediately keeps the circuit breaker from
                // interfering with the measurement.
                PickResult::Pick(pick) => complete_ok(pick),
                other => panic!("unexpected pick result: {other:?}"),
            }
        }
        // Expected 5000 with a standard deviation of 50; allow 10 sigma so the
        // test is not flaky.
        assert!(
            (4_500..=5_500).contains(&dropped),
            "dropped {dropped} of {PICKS} picks"
        );
    }

    #[test]
    fn logical_dns_cluster_uses_empty_eds_service_name_and_no_drops() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_logical_dns",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let update = update_with_leaf_cluster(
            "cluster_logical_dns",
            ClusterDiscovery::LogicalDns {
                hostname: "example.com".to_string(),
                port: 443,
            },
            Some(LeafEndpointSource::LogicalDns(Arc::from([]))),
            1,
        );
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        assert!(matches!(
            state.picker.pick(PickOptions::new(&req, &opts)),
            PickResult::Drop(_)
        ));
        let counter = CallCounter::get_or_create("cluster_logical_dns", "");
        assert_eq!(counter.load(), 1);
        complete_ok(pick);
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn unresolved_eds_endpoints_apply_no_drops() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_unresolved",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let update = update_with_leaf_cluster(
            "cluster_unresolved",
            ClusterDiscovery::Eds {
                eds_service_name: "eds_unresolved".to_string(),
                raw_eds_service_name: "eds_unresolved".to_string(),
            },
            None,
            10,
        );
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        complete_ok(pick);
    }

    #[test]
    fn unset_eds_service_name_keys_counter_by_empty_name() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_unset_eds",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        // CDS left `eds_service_name` unset, so it resolves to the cluster
        // name for the EDS watch, but the raw CDS value is empty.
        let update = update_with_leaf_cluster(
            "cluster_unset_eds",
            ClusterDiscovery::Eds {
                eds_service_name: "cluster_unset_eds".to_string(),
                raw_eds_service_name: String::new(),
            },
            None,
            10,
        );
        policy
            .resolver_update(update, &cfg, &mut controller)
            .unwrap();
        let state = controller.latest_state.take().expect("state");

        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        let pick = state
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();
        let raw_key_counter = CallCounter::get_or_create("cluster_unset_eds", "");
        let resolved_key_counter =
            CallCounter::get_or_create("cluster_unset_eds", "cluster_unset_eds");
        assert_eq!(raw_key_counter.load(), 1);
        assert_eq!(resolved_key_counter.load(), 0);
        complete_ok(pick);
        assert_eq!(raw_key_counter.load(), 0);
    }

    #[test]
    fn eds_service_name_change_switches_shared_counter() {
        let (mut policy, _scheduler, mut controller) = setup_test_policy();
        let cfg = parse_test_config(
            "cluster_switch",
            "test_cluster_impl_child",
            serde_json::json!({}),
        );
        let req = RequestHeaders::new();
        let opts = CallOptions::new();

        let update1 = update_with_leaf_eds_cluster("cluster_switch", "eds_old", 1, Vec::new());
        policy
            .resolver_update(update1, &cfg, &mut controller)
            .unwrap();
        let state1 = controller.latest_state.take().expect("state1");
        let pick1 = state1
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        // Switching the EDS service name moves the policy to a different
        // `{cluster, eds_service_name}` counter, so the in-flight RPC on the
        // old counter does not block new picks.
        let update2 = update_with_leaf_eds_cluster("cluster_switch", "eds_new", 1, Vec::new());
        policy
            .resolver_update(update2, &cfg, &mut controller)
            .unwrap();
        let state2 = controller.latest_state.take().expect("state2");
        let pick2 = state2
            .picker
            .pick(PickOptions::new(&req, &opts))
            .unwrap_pick();

        let old_counter = CallCounter::get_or_create("cluster_switch", "eds_old");
        let new_counter = CallCounter::get_or_create("cluster_switch", "eds_new");
        assert_eq!(old_counter.load(), 1);
        assert_eq!(new_counter.load(), 1);

        complete_ok(pick1);
        complete_ok(pick2);
        assert_eq!(old_counter.load(), 0);
        assert_eq!(new_counter.load(), 0);
    }

    fn setup_test_policy() -> (ClusterImplPolicy, Arc<MockScheduler>, MockChannelController) {
        GLOBAL_LB_REGISTRY.add_builder(TestChildLbBuilder);
        let scheduler = Arc::new(MockScheduler::default());
        let policy = ClusterImplBuilder.build(LbPolicyOptions {
            work_scheduler: Arc::clone(&scheduler) as Arc<dyn WorkScheduler>,
            runtime: default_runtime(),
        });
        let controller = MockChannelController { latest_state: None };
        (policy, scheduler, controller)
    }

    fn parse_test_config(
        cluster: &str,
        child_name: &str,
        child_cfg: serde_json::Value,
    ) -> ClusterImplConfig {
        let json = serde_json::json!({
            "cluster": cluster,
            "childPolicy": [{ child_name: child_cfg }]
        });
        ClusterImplBuilder
            .parse_config(&LbConfigJson::new(&json.to_string()).unwrap())
            .expect("parse test config")
    }

    fn update_with_xds_config(clusters: HashMap<String, ClusterResult>) -> ResolverUpdate {
        let route_config = Arc::new(RouteConfigResource {
            name: "routes".to_string(),
            virtual_hosts: vec![VirtualHost {
                name: "vh".to_string(),
                domains: vec![DomainMatcher::Universal],
                routes: Vec::new(),
            }],
        });
        let listener = Arc::new(ListenerResource {
            name: "listener".to_string(),
            route_source: RouteSource::Inline(Arc::clone(&route_config)),
        });
        let xds_config =
            XdsConfig::try_new(listener, route_config, 0, clusters).expect("valid test XdsConfig");

        let mut update = ResolverUpdate::default();
        update.endpoints = Ok(vec![Endpoint::default()]);
        update.attributes = Attributes::new().add(xds_config);
        update
    }

    fn update_with_leaf_eds_cluster(
        cluster_name: &str,
        eds_service_name: &str,
        max_concurrent_requests: u32,
        drop_overloads: Vec<DropOverload>,
    ) -> ResolverUpdate {
        update_with_leaf_cluster(
            cluster_name,
            ClusterDiscovery::Eds {
                eds_service_name: eds_service_name.to_string(),
                raw_eds_service_name: eds_service_name.to_string(),
            },
            Some(LeafEndpointSource::Eds(Arc::new(EndpointsResource {
                cluster_name: eds_service_name.to_string(),
                localities: Vec::new(),
                drop_overloads,
            }))),
            max_concurrent_requests,
        )
    }

    fn update_with_leaf_cluster(
        cluster_name: &str,
        discovery: ClusterDiscovery,
        source: Option<LeafEndpointSource>,
        max_concurrent_requests: u32,
    ) -> ResolverUpdate {
        let mut clusters = HashMap::new();
        clusters.insert(
            cluster_name.to_string(),
            Ok(Arc::new(ClusterConfig {
                cluster: Arc::new(ClusterResource {
                    name: cluster_name.to_string(),
                    discovery,
                    max_concurrent_requests,
                }),
                children: ClusterChildren::Leaf(LeafEndpoints {
                    source,
                    resolution_note: None,
                }),
            })),
        );
        update_with_xds_config(clusters)
    }

    fn assert_fails_unavailable(picker: &Arc<dyn Picker>, expected_substr: &str) {
        let req = RequestHeaders::new();
        let opts = CallOptions::new();
        match picker.pick(PickOptions::new(&req, &opts)) {
            PickResult::Fail(err) => {
                assert_eq!(err.code(), StatusCodeError::Unavailable);
                assert!(
                    err.message().contains(expected_substr),
                    "expected '{}' in '{}'",
                    expected_substr,
                    err.message()
                );
            }
            other => panic!("expected PickResult::Fail, got {other:?}"),
        }
    }

    #[derive(Clone, Debug)]
    struct ChildCompletionTracker(Arc<AtomicUsize>);

    /// Records, for each child completion, the error code (`None` for OK) and
    /// whether trailing metadata was provided.
    #[derive(Clone, Debug, Default)]
    struct ChildOutcomeRecorder(Arc<Mutex<Vec<RecordedOutcome>>>);

    type RecordedOutcome = (Option<StatusCodeError>, bool);

    /// Completes a pick's RPC successfully, without trailing metadata.
    fn complete_ok(pick: Pick) {
        pick.on_complete.expect("on_complete")(&CallOutcome::new(&Ok(())));
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct DummySubchannel {
        address: Address,
    }

    impl ForwardingSubchannel for DummySubchannel {
        fn delegate(&self) -> &Arc<dyn Subchannel> {
            unimplemented!()
        }

        fn address(&self) -> Address {
            self.address.clone()
        }

        fn get_attribute_dyn(&self, _id: TypeId) -> Option<&dyn Any> {
            None
        }

        fn connect(&self) {}
    }

    #[derive(Debug)]
    struct ReadyChildPicker;

    impl Picker for ReadyChildPicker {
        fn pick(&self, options: PickOptions<'_>) -> PickResult {
            let tracker = options
                .call_options
                .get_option::<ChildCompletionTracker>()
                .cloned();
            let recorder = options
                .call_options
                .get_option::<ChildOutcomeRecorder>()
                .cloned();
            let on_complete: Option<CompletionCallback> = tracker.map(|t| {
                Box::new(move |outcome: &CallOutcome<'_>| {
                    t.0.fetch_add(1, Ordering::AcqRel);
                    if let Some(r) = recorder {
                        let code = outcome.status().as_ref().err().map(|e| e.code());
                        r.0.lock()
                            .unwrap()
                            .push((code, outcome.trailing_metadata().is_some()));
                    }
                }) as CompletionCallback
            });
            PickResult::Pick(Pick {
                subchannel: Arc::new(DummySubchannel {
                    address: Address::default(),
                }),
                metadata: MetadataMap::new(),
                on_complete,
            })
        }
    }

    #[derive(Debug)]
    struct QueuingChildPicker;

    impl Picker for QueuingChildPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            PickResult::Queue
        }
    }

    #[derive(Debug, Default)]
    struct MockScheduler {
        queued_work: Mutex<Vec<Option<WorkData>>>,
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
            address: &Address,
            _work_scheduler: Arc<dyn WorkScheduler>,
        ) -> (Arc<dyn Subchannel>, SubchannelState) {
            (
                Arc::new(DummySubchannel {
                    address: address.clone(),
                }),
                SubchannelState::ready(),
            )
        }

        fn update_picker(&mut self, update: LbState) {
            self.latest_state = Some(update);
        }

        fn request_resolution(&mut self) {}
    }

    #[derive(Debug, Default, Deserialize)]
    struct TestChildConfig {
        #[serde(default)]
        start_connecting: bool,
        #[serde(default)]
        start_idle: bool,
    }

    #[derive(Debug)]
    struct TestChildLbPolicy {
        work_scheduler: Arc<dyn WorkScheduler>,
        is_idle: AtomicBool,
    }

    impl LbPolicy for TestChildLbPolicy {
        type LbConfig = TestChildConfig;

        fn resolver_update(
            &mut self,
            _update: ResolverUpdate,
            config: &Self::LbConfig,
            channel_controller: &mut dyn ChannelController,
        ) -> Result<(), String> {
            if config.start_idle {
                self.is_idle.store(true, Ordering::Release);
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Idle,
                    picker: Arc::new(QueuingChildPicker),
                });
            } else if config.start_connecting {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Connecting,
                    picker: Arc::new(QueuingChildPicker),
                });
            } else {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Ready,
                    picker: Arc::new(ReadyChildPicker),
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
                picker: Arc::new(ReadyChildPicker),
            });
        }

        fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
            if self.is_idle.swap(false, Ordering::AcqRel) {
                channel_controller.update_picker(LbState {
                    connectivity_state: ConnectivityState::Connecting,
                    picker: Arc::new(QueuingChildPicker),
                });
                self.work_scheduler.schedule_work(None);
            }
        }
    }

    #[derive(Debug)]
    struct TestChildLbBuilder;

    impl LbPolicyBuilder for TestChildLbBuilder {
        type LbPolicy = TestChildLbPolicy;

        fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
            TestChildLbPolicy {
                work_scheduler: options.work_scheduler,
                is_idle: AtomicBool::new(false),
            }
        }

        fn name(&self) -> &'static str {
            "test_cluster_impl_child"
        }

        fn parse_config(&self, config: &LbConfigJson) -> Result<TestChildConfig, String> {
            config.convert_to().map_err(|e| e.to_string())
        }
    }
}
