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

//! The `least_request_experimental` LB policy, as described in [gRFC A48].
//!
//! Like `round_robin`, this policy creates a `pick_first` child for every
//! endpoint and attempts to keep all of them connected.  Instead of rotating
//! through the children, its picker samples `choice_count` children uniformly
//! at random (with replacement) and picks the one with the fewest outstanding
//! requests.  Only the "all weights equal" variant of Envoy's algorithm is
//! implemented.
//!
//! [gRFC A48]: https://github.com/grpc/proposal/blob/master/A48-xds-least-request-lb-policy.md

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;

use crate::client::ConnectivityState;
use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::FailingPicker;
use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::LbConfigJson;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::PickOptions;
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::child_manager::ChildManager;
use crate::client::load_balancing::child_manager::ChildUpdate;
use crate::client::load_balancing::pick_first::PickFirstBuilder;
use crate::client::load_balancing::pick_first::PickFirstConfig;
use crate::client::name_resolution::Endpoint;
use crate::client::name_resolution::ResolverUpdate;

pub static POLICY_NAME: &str = "least_request_experimental";

/// The default number of children sampled per pick (power of two choices).
const DEFAULT_CHOICE_COUNT: u32 = 2;
/// The minimum allowed `choiceCount`; smaller values are rejected.
const MIN_CHOICE_COUNT: u32 = 2;
/// The maximum effective `choiceCount`; larger values are capped to this.
const MAX_CHOICE_COUNT: u32 = 10;

/// Returns a uniformly-distributed random index in `0..n`.
type RandomIndexFn = dyn Fn(usize) -> usize + Send + Sync;

/// The JSON representation of the `least_request_experimental` config.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LeastRequestConfigJson {
    #[serde(default)]
    choice_count: Option<u32>,
}

/// Validated configuration for `least_request_experimental`.
#[derive(Debug, Clone)]
pub(crate) struct LeastRequestConfig {
    /// Number of children randomly sampled per pick, in
    /// `[MIN_CHOICE_COUNT, MAX_CHOICE_COUNT]`.
    choice_count: u32,
    pick_first: PickFirstConfig,
}

impl Default for LeastRequestConfig {
    fn default() -> Self {
        Self {
            choice_count: DEFAULT_CHOICE_COUNT,
            pick_first: PickFirstConfig::default(),
        }
    }
}

impl LeastRequestConfig {
    /// Validates `choice_count` per gRFC A48: values below 2 are rejected and
    /// values above 10 are capped to 10.
    fn new(choice_count: u32) -> Result<Self, String> {
        if choice_count < MIN_CHOICE_COUNT {
            return Err(format!(
                "{POLICY_NAME}: choiceCount must be at least {MIN_CHOICE_COUNT}, got {choice_count}"
            ));
        }
        Ok(Self {
            choice_count: choice_count.min(MAX_CHOICE_COUNT),
            ..Default::default()
        })
    }
}

#[derive(Debug)]
pub(crate) struct LeastRequestBuilder {}

impl LbPolicyBuilder for LeastRequestBuilder {
    type LbPolicy = LeastRequestPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        let child_manager = ChildManager::new(options.runtime, options.work_scheduler);
        LeastRequestPolicy::new(child_manager, Arc::new(|n| rand::random_range(..n)))
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &LbConfigJson) -> Result<LeastRequestConfig, String> {
        let json: LeastRequestConfigJson = config
            .convert_to()
            .map_err(|e| format!("{POLICY_NAME}: invalid config: {e}"))?;
        LeastRequestConfig::new(json.choice_count.unwrap_or(DEFAULT_CHOICE_COUNT))
    }
}

/// Register least request as an LbPolicy.
pub(crate) fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(LeastRequestBuilder {});
}

pub(crate) struct LeastRequestPolicy {
    child_manager: ChildManager<Endpoint, PickFirstBuilder>,
    /// Outstanding request counters, one per endpoint (child).  Counters are
    /// local to this policy instance and live as long as the endpoint remains
    /// in the resolver update, so they carry over across picker updates.
    /// For each value, the inner Arc is the actual counter, and the outer Arc
    /// is the reference to it.
    active_requests: HashMap<Endpoint, Arc<Arc<()>>>,
    /// The most recently received `choice_count`.
    choice_count: u32,
    random_index: Arc<RandomIndexFn>,
}

impl Debug for LeastRequestPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeastRequestPolicy")
            .field("child_manager", &self.child_manager)
            .field("active_requests", &self.active_requests)
            .field("choice_count", &self.choice_count)
            .finish_non_exhaustive()
    }
}

impl LeastRequestPolicy {
    fn new(
        child_manager: ChildManager<Endpoint, PickFirstBuilder>,
        random_index: Arc<RandomIndexFn>,
    ) -> Self {
        Self {
            child_manager,
            active_requests: HashMap::new(),
            choice_count: DEFAULT_CHOICE_COUNT,
            random_index,
        }
    }

    // Sets the policy's state to TRANSIENT_FAILURE with a picker returning the
    // error string provided, then requests re-resolution from the channel.
    fn move_to_transient_failure(
        &mut self,
        error: String,
        channel_controller: &mut dyn ChannelController,
    ) {
        channel_controller.update_picker(LbState {
            connectivity_state: ConnectivityState::TransientFailure,
            picker: Arc::new(FailingPicker { error }),
        });
        channel_controller.request_resolution();
    }

    // Sends an aggregate picker based on states of children.
    //
    // The state is determined according to normal state aggregation rules (the
    // same as round_robin), and the picker performs least-request selection
    // among all children in that state.  Outside of READY, the children's
    // pickers queue or fail, so the selection only matters for READY children.
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController) {
        self.update_picker_if(false, channel_controller);
    }

    // Like `update_picker`, but also sends a new picker when `force` is set
    // even if no child changed.
    fn update_picker_if(&mut self, force: bool, channel_controller: &mut dyn ChannelController) {
        // Always call child_updated() so the children's flags are cleared.
        if !self.child_manager.child_updated() && !force {
            return;
        }
        let aggregate_state = self.child_manager.aggregate_states();
        let children = self
            .child_manager
            .children()
            .filter(|(_, child)| child.state().connectivity_state == aggregate_state)
            .map(|(endpoint, child)| PickerChild {
                picker: child.state().picker.clone(),
                active_requests: self
                    .active_requests
                    .get(endpoint)
                    .expect("every child has a counter")
                    .clone(),
            })
            .collect();
        channel_controller.update_picker(LbState {
            connectivity_state: aggregate_state,
            picker: Arc::new(LeastRequestPicker {
                children,
                choice_count: self.choice_count,
                random_index: self.random_index.clone(),
            }),
        });
    }

    // Responds to an incoming ResolverUpdate containing an Err in endpoints by
    // forwarding it to all children unconditionally.  Updates the picker as
    // needed.
    fn handle_resolver_error(
        &mut self,
        resolver_update: ResolverUpdate,
        config: &LeastRequestConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let err = format!(
            "Received error from name resolver: {}",
            resolver_update.endpoints.as_ref().unwrap_err()
        );
        if self.child_manager.children().next().is_none() {
            // We had no children so we must produce an erroring picker.
            self.move_to_transient_failure(err.clone(), channel_controller);
            return Err(err);
        }
        // Forward the error to each child, ignoring their responses.
        let _ = self.child_manager.resolver_update(
            resolver_update,
            &config.pick_first,
            channel_controller,
        );
        self.update_picker(channel_controller);
        Err(err)
    }
}

impl LbPolicy for LeastRequestPolicy {
    type LbConfig = LeastRequestConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let choice_count_changed = self.choice_count != config.choice_count;
        self.choice_count = config.choice_count;

        if update.endpoints.is_err() {
            return self.handle_resolver_error(update, config, channel_controller);
        }

        // De-duplicate endpoints so that we don't create more than one child
        // for the same endpoint.  Per gRFC A48, the extra weight implied by a
        // duplicate is ignored since weighted least request is not supported.
        let mut seen = HashSet::new();
        let endpoints: Vec<Endpoint> = update
            .endpoints
            .as_ref()
            .unwrap()
            .iter()
            .filter(|e| seen.insert(*e))
            .cloned()
            .collect();

        // Retain counters for endpoints that remain, and create counters for
        // new ones.  Counters for removed endpoints are dropped here; any
        // in-flight RPCs keep theirs alive until they complete.
        let mut old_counters = std::mem::take(&mut self.active_requests);
        self.active_requests = endpoints
            .iter()
            .map(|e| (e.clone(), old_counters.remove(e).unwrap_or_default()))
            .collect();

        // Shard the update by endpoint.
        let updates = endpoints.iter().map(|e| {
            let update = update.with_endpoints(Ok(vec![e.clone()]));
            ChildUpdate {
                child_identifier: e.clone(),
                child_policy_builder: PickFirstBuilder {},
                child_update: Some((update, &config.pick_first)),
            }
        });
        // Errors from individual children are reflected in their pickers.
        let _ = self.child_manager.update(updates, channel_controller);

        if self.child_manager.children().next().is_none() {
            // There are no children remaining, so report this error and produce
            // an erroring picker.
            let err = "Received empty address list from the name resolver";
            self.move_to_transient_failure(err.into(), channel_controller);
            return Err(err.into());
        }

        // A new choice_count requires a new picker even if no child changed.
        self.update_picker_if(choice_count_changed, channel_controller);
        Ok(())
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        self.child_manager.work(data, channel_controller);
        self.update_picker(channel_controller);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager.exit_idle(channel_controller);
        self.update_picker(channel_controller);
    }
}

/// A child as seen by the picker: its current picker and its outstanding
/// request counter.
#[derive(Debug)]
struct PickerChild {
    picker: Arc<dyn Picker>,
    active_requests: Arc<Arc<()>>,
}

impl PickerChild {
    fn active_requests(&self) -> usize {
        let counter: &Arc<()> = &*self.active_requests;
        Arc::strong_count(counter)
    }

    fn add_call(&self) -> Arc<()> {
        Arc::clone(&*self.active_requests)
    }
}

struct LeastRequestPicker {
    children: Vec<PickerChild>,
    choice_count: u32,
    random_index: Arc<RandomIndexFn>,
}

impl Debug for LeastRequestPicker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeastRequestPicker")
            .field("children", &self.children)
            .field("choice_count", &self.choice_count)
            .finish_non_exhaustive()
    }
}

impl LeastRequestPicker {

    fn random_child(&self) -> &PickerChild {
        return &self.children[(self.random_index)(self.children.len())]
    }

    /// Samples `choice_count` children (with replacement) and returns the one
    /// with the fewest outstanding requests, mirroring Envoy's algorithm.
    fn choose(&self) -> &PickerChild {
        let mut candidate = self.random_child();
        for _ in 1..self.choice_count {
            let sampled = self.random_child();
            if sampled.active_requests() < candidate.active_requests() {
                candidate = sampled;
            }
        }
        candidate
    }
}

impl Picker for LeastRequestPicker {
    fn pick(&self, options: PickOptions<'_>) -> PickResult {
        let child = self.choose();
        let mut result = child.picker.pick(options);
        if let PickResult::Pick(pick) = &mut result {
            let counter = child.add_call();
            let child_on_complete = pick.on_complete.take();
            pick.on_complete = Some(Box::new(move |outcome| {
                drop(counter);
                if let Some(cb) = child_on_complete {
                    cb(outcome);
                }
            }));
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use super::*;
    use crate::client::CallOptions;
    use crate::client::load_balancing::CallOutcome;
    use crate::client::load_balancing::ParsedLbConfig;
    use crate::client::load_balancing::Pick;
    use crate::client::load_balancing::subchannel::Subchannel;
    use crate::client::load_balancing::subchannel::SubchannelState;
    use crate::client::load_balancing::test_utils;
    use crate::client::load_balancing::test_utils::TestEnv;
    use crate::core::Address;
    use crate::rt::default_runtime;

    /// A scripted source of random indices.  Each call pops the next value
    /// (modulo `n`); panics if the script runs out.
    #[derive(Clone, Default)]
    struct ScriptedRandom(Arc<Mutex<VecDeque<usize>>>);

    impl ScriptedRandom {
        fn push(&self, values: &[usize]) {
            self.0.lock().unwrap().extend(values);
        }

        fn as_fn(&self) -> Arc<RandomIndexFn> {
            let queue = self.0.clone();
            Arc::new(move |n| {
                queue
                    .lock()
                    .unwrap()
                    .pop_front()
                    .expect("ran out of scripted random values")
                    % n
            })
        }

        fn assert_drained(&self) {
            assert!(self.0.lock().unwrap().is_empty());
        }
    }

    fn new_env(random: &ScriptedRandom) -> TestEnv<LeastRequestPolicy> {
        let random = random.as_fn();
        TestEnv::new(|work_scheduler| {
            LeastRequestPolicy::new(ChildManager::new(default_runtime(), work_scheduler), random)
        })
    }

    fn create_endpoints(num_endpoints: usize) -> Vec<Endpoint> {
        (0..num_endpoints)
            .map(|i| Endpoint {
                addresses: vec![Address {
                    address: format!("{}.{}.{}.{}:1", i + 1, i + 1, i + 1, i + 1).into(),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .collect()
    }

    fn config(choice_count: u32) -> LeastRequestConfig {
        LeastRequestConfig::new(choice_count).unwrap()
    }

    fn pick(picker: &Arc<dyn Picker>) -> Pick {
        let req = test_utils::new_request_headers();
        match picker.pick(PickOptions::new(&req, &CallOptions::new())) {
            PickResult::Pick(pick) => pick,
            other => panic!("unexpected pick result {other}"),
        }
    }

    fn complete(pick: Pick) {
        (pick.on_complete.expect("on_complete must be set"))(&CallOutcome::new(&Ok(())));
    }

    impl TestEnv<LeastRequestPolicy> {
        // Expects `n` subchannels to be created and connected, in order.
        fn verify_subchannel_creation(&mut self, n: usize) -> Vec<Arc<dyn Subchannel>> {
            (0..n)
                .map(|_| {
                    let sc = self.expect_new_subchannel();
                    self.expect_connect();
                    sc
                })
                .collect()
        }

        // Sends `endpoints`, and moves all resulting subchannels to READY.
        // Returns the subchannels and the final READY picker.
        fn setup_ready(
            &mut self,
            endpoints: Vec<Endpoint>,
            config: &LeastRequestConfig,
        ) -> (Vec<Arc<dyn Subchannel>>, Arc<dyn Picker>) {
            let n = endpoints.len();
            self.send_resolver_update_with_config(endpoints, config)
                .unwrap();
            let subchannels = self.verify_subchannel_creation(n);
            self.expect_picker_state(ConnectivityState::Connecting);
            let mut picker = None;
            for sc in &subchannels {
                self.send_subchannel_update(sc, &SubchannelState::ready());
                picker = Some(self.expect_picker_state(ConnectivityState::Ready));
            }
            (subchannels, picker.unwrap())
        }
    }

    fn parse(json: &str) -> Result<LeastRequestConfig, String> {
        LeastRequestBuilder {}.parse_config(&LbConfigJson::new(json).unwrap())
    }

    #[test]
    fn config_defaults_choice_count_to_two() {
        assert_eq!(parse("{}").unwrap().choice_count, 2);
    }

    #[test]
    fn config_accepts_valid_choice_count() {
        assert_eq!(parse(r#"{"choiceCount": 2}"#).unwrap().choice_count, 2);
        assert_eq!(parse(r#"{"choiceCount": 7}"#).unwrap().choice_count, 7);
        assert_eq!(parse(r#"{"choiceCount": 10}"#).unwrap().choice_count, 10);
    }

    #[test]
    fn config_caps_choice_count_at_ten() {
        assert_eq!(parse(r#"{"choiceCount": 11}"#).unwrap().choice_count, 10);
        assert_eq!(
            parse(r#"{"choiceCount": 4294967295}"#)
                .unwrap()
                .choice_count,
            10
        );
    }

    #[test]
    fn config_rejects_choice_count_below_two() {
        assert!(parse(r#"{"choiceCount": 0}"#).is_err());
        assert!(parse(r#"{"choiceCount": 1}"#).is_err());
    }

    #[test]
    fn config_rejects_invalid_types() {
        assert!(parse(r#"{"choiceCount": "3"}"#).is_err());
        assert!(parse(r#"{"choiceCount": -1}"#).is_err());
    }

    #[test]
    fn policy_is_registered() {
        super::super::register();
        let parsed =
            ParsedLbConfig::parse(r#"[{"least_request_experimental": {"choiceCount": 3}}]"#)
                .unwrap();
        assert_eq!(parsed.builder.name(), POLICY_NAME);
    }

    // Tests the scenario where the resolver returns an error before a valid
    // update. The LB policy should move to TRANSIENT_FAILURE.
    #[tokio::test]
    async fn resolver_error_before_valid_update() {
        let mut env = new_env(&ScriptedRandom::default());
        assert!(env.send_resolver_error("resolver error".into()).is_err());
        env.expect_failing_picker("Received error from name resolver: resolver error");
        env.expect_request_resolution();
    }

    // An empty endpoint list moves the policy to TRANSIENT_FAILURE.
    #[tokio::test]
    async fn empty_endpoints_moves_to_transient_failure() {
        let mut env = new_env(&ScriptedRandom::default());
        assert!(env.send_resolver_update(vec![]).is_err());
        env.expect_failing_picker("Received empty address list from the name resolver");
        env.expect_request_resolution();
    }

    // With all subchannels in TRANSIENT_FAILURE, the policy reports
    // TRANSIENT_FAILURE, and returns to READY when one becomes READY.
    #[tokio::test]
    async fn transient_failure_until_ready() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        env.send_resolver_update(create_endpoints(2)).unwrap();
        let subchannels = env.verify_subchannel_creation(2);
        random.push(&[0, 1]);
        env.expect_queuing_picker();

        let fail = SubchannelState {
            connectivity_state: ConnectivityState::TransientFailure,
            last_connection_error: Some("connection error".into()),
        };
        env.send_subchannel_update(&subchannels[0], &fail);
        env.expect_request_resolution();
        random.push(&[0, 0]);
        env.expect_queuing_picker();
        env.send_subchannel_update(&subchannels[1], &fail);
        env.expect_request_resolution();
        random.push(&[0, 1]);
        env.expect_failing_picker("connection error");

        env.send_subchannel_update(&subchannels[1], &SubchannelState::ready());
        random.push(&[0, 0]);
        env.expect_ready_picker(&subchannels[1]);
        random.assert_drained();
    }

    // The picker selects the sampled child with the fewest outstanding
    // requests, and ties go to the first sample.
    #[tokio::test]
    async fn picks_least_loaded_of_sampled_children() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let (sc, picker) = env.setup_ready(create_endpoints(3), &config(2));

        // All idle: tie, so the first sample wins.
        random.push(&[1, 2]);
        let p1 = pick(&picker);
        assert_eq!(&p1.subchannel, &sc[1]);

        // sc[1] has one outstanding request; sc[2] has none.
        random.push(&[1, 2]);
        let p2 = pick(&picker);
        assert_eq!(&p2.subchannel, &sc[2]);

        // sc[1] and sc[2] each have one; sc[0] has none.
        random.push(&[2, 0]);
        let p3 = pick(&picker);
        assert_eq!(&p3.subchannel, &sc[0]);

        // All have one: tie, so the first sample wins.
        random.push(&[2, 1]);
        let p4 = pick(&picker);
        assert_eq!(&p4.subchannel, &sc[2]);

        // sc[2] now has two.  Completing p1 leaves sc[1] with zero.
        complete(p1);
        random.push(&[2, 0]);
        assert_eq!(&pick(&picker).subchannel, &sc[0]); // 2 vs 1
        random.push(&[0, 1]);
        assert_eq!(&pick(&picker).subchannel, &sc[1]); // 2 vs 0

        drop((p2, p3, p4));
        random.assert_drained();
    }

    // All `choice_count` samples are drawn, and the least loaded among them
    // is chosen.
    #[tokio::test]
    async fn samples_choice_count_children() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let (sc, picker) = env.setup_ready(create_endpoints(4), &config(4));

        // Load up sc[0], sc[1] and sc[2].
        let mut held = Vec::new();
        for (i, want) in sc.iter().enumerate().take(3) {
            random.push(&[i, i, i, i]);
            let p = pick(&picker);
            assert_eq!(&p.subchannel, want);
            held.push(p);
        }
        // Only the last of four samples is unloaded.
        random.push(&[0, 1, 2, 3]);
        assert_eq!(&pick(&picker).subchannel, &sc[3]);
        random.assert_drained();
    }

    // The outstanding request counter is decremented if the completion
    // callback is dropped without being invoked.
    #[tokio::test]
    async fn counter_released_when_callback_dropped() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let (sc, picker) = env.setup_ready(create_endpoints(2), &config(2));

        random.push(&[0, 1]);
        let p = pick(&picker);
        assert_eq!(&p.subchannel, &sc[0]);
        random.push(&[0, 1]);
        assert_eq!(&pick(&picker).subchannel, &sc[1]); // and dropped immediately

        drop(p);
        // Both are back at zero: tie, so the first sample wins.
        random.push(&[0, 1]);
        assert_eq!(&pick(&picker).subchannel, &sc[0]);
        random.push(&[1, 0]);
        assert_eq!(&pick(&picker).subchannel, &sc[1]);
        random.assert_drained();
    }

    // Counters are retained across picker updates and resolver updates for
    // endpoints that remain.
    #[tokio::test]
    async fn counters_persist_across_updates() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let endpoints = create_endpoints(3);
        let (sc, picker) = env.setup_ready(endpoints[..2].to_vec(), &config(2));

        random.push(&[0, 0]);
        let held = pick(&picker);
        assert_eq!(&held.subchannel, &sc[0]);

        // Add a third endpoint.  The existing children are retained.
        env.send_resolver_update_with_config(endpoints.clone(), &config(2))
            .unwrap();
        let new_sc = env.verify_subchannel_creation(1).remove(0);
        let picker = env.expect_picker_state(ConnectivityState::Ready);
        env.send_subchannel_update(&new_sc, &SubchannelState::ready());
        let picker2 = env.expect_picker_state(ConnectivityState::Ready);

        // sc[0] still has an outstanding request in both new pickers.
        random.push(&[0, 1]);
        assert_eq!(&pick(&picker).subchannel, &sc[1]);
        random.push(&[0, 2]);
        assert_eq!(&pick(&picker2).subchannel, &new_sc);

        complete(held);
        random.push(&[0, 2]);
        assert_eq!(&pick(&picker2).subchannel, &sc[0]);
        random.assert_drained();
    }

    // Changing only choice_count produces a new picker that uses it.
    #[tokio::test]
    async fn choice_count_change_updates_picker() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let endpoints = create_endpoints(2);
        let (_, _) = env.setup_ready(endpoints.clone(), &config(2));

        env.send_resolver_update_with_config(endpoints.clone(), &config(5))
            .unwrap();
        let picker = env.expect_picker_state(ConnectivityState::Ready);
        random.push(&[0, 0, 0, 0, 0]);
        pick(&picker);
        random.assert_drained();

        // The same config again does not produce a new picker.
        env.send_resolver_update_with_config(endpoints, &config(5))
            .unwrap();
        env.expect_no_events();
    }

    // Duplicate endpoints are de-duplicated so only one connection is made.
    #[tokio::test]
    async fn duplicate_endpoints_are_deduplicated() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        let endpoint = create_endpoints(1).remove(0);
        env.send_resolver_update(vec![endpoint.clone(), endpoint.clone()])
            .unwrap();
        let subchannels = env.verify_subchannel_creation(1);
        env.expect_picker_state(ConnectivityState::Connecting);
        env.expect_no_events();

        env.send_subchannel_update(&subchannels[0], &SubchannelState::ready());
        random.push(&[0, 0]);
        env.expect_ready_picker(&subchannels[0]);
        random.assert_drained();
    }

    // Only READY children are candidates once any child is READY.
    #[tokio::test]
    async fn only_ready_children_are_picked() {
        let random = ScriptedRandom::default();
        let mut env = new_env(&random);
        env.send_resolver_update(create_endpoints(3)).unwrap();
        let sc = env.verify_subchannel_creation(3);
        env.expect_picker_state(ConnectivityState::Connecting);
        env.send_subchannel_update(&sc[1], &SubchannelState::ready());
        let picker = env.expect_picker_state(ConnectivityState::Ready);
        for i in 0..5 {
            random.push(&[i, i + 1]);
            assert_eq!(&pick(&picker).subchannel, &sc[1]);
        }
        random.assert_drained();
    }
}
