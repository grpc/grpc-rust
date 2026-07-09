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

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::attributes::Attributes;
use crate::client::ConnectivityState;
use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::DynLbPolicyBuilder;
use crate::client::load_balancing::FailingPicker;
use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::ParsedJsonLbConfig;
use crate::client::load_balancing::Pick;
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::QueuingPicker;
use crate::client::load_balancing::Subchannel;
use crate::client::load_balancing::SubchannelState;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::child_manager::ChildManager;
use crate::client::load_balancing::child_manager::ChildUpdate;
use crate::client::load_balancing::pick_first;
use crate::client::load_balancing::round_robin::RoundRobinPicker;
use crate::client::load_balancing::subchannel::WeakSubchannel;
use crate::client::name_resolution::Endpoint;
use crate::client::name_resolution::ResolverUpdate;
use crate::core::RequestHeaders;
use crate::metadata::MetadataMap;

pub(crate) static POLICY_NAME: &str = "least_request_experimental";
static START: Once = Once::new();

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LeastRequestLoadBalancingConfig {
    #[serde(default = "default_choice_count")]
    pub choice_count: u32,
}

fn default_choice_count() -> u32 {
    2
}

#[derive(Debug)]
pub(crate) struct LeastRequestBuilder {}

impl LbPolicyBuilder for LeastRequestBuilder {
    type LbPolicy = LeastRequestPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        let child_manager = ChildManager::new(options.runtime, options.work_scheduler);
        LeastRequestPolicy::new(
            child_manager,
            GLOBAL_LB_REGISTRY
                .get_policy(pick_first::POLICY_NAME)
                .unwrap(),
        )
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(
        &self,
        config: &ParsedJsonLbConfig,
    ) -> Result<Option<LeastRequestLoadBalancingConfig>, String> {
        let mut parsed: LeastRequestLoadBalancingConfig = config
            .convert_to()
            .map_err(|e| format!("failed to parse least_request config: {e}"))?;

        if parsed.choice_count < 2 {
            return Err("choice_count must be at least 2".to_string());
        }

        parsed.choice_count = (parsed.choice_count).min(10);
        Ok(Some(parsed))
    }
}

#[derive(Debug)]
pub(crate) struct LeastRequestPolicy {
    child_manager: ChildManager<Endpoint>,
    pick_first_builder: Arc<DynLbPolicyBuilder>,
    config: Option<LeastRequestLoadBalancingConfig>,
    subchannel_counters: HashMap<WeakSubchannel, Arc<AtomicUsize>>,
}

impl LeastRequestPolicy {
    fn new(
        child_manager: ChildManager<Endpoint>,
        pick_first_builder: Arc<DynLbPolicyBuilder>,
    ) -> Self {
        Self {
            child_manager,
            pick_first_builder,
            config: None,
            subchannel_counters: HashMap::new(),
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
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController, force: bool) {
        if !force && !self.child_manager.child_updated() {
            return;
        }
        let aggregate_state = self.child_manager.aggregate_states();

        let picker: Arc<dyn Picker> = match aggregate_state {
            ConnectivityState::Ready => {
                let mut ready_subchannels = Vec::new();
                let mut child_weaks = HashSet::new();

                for child in self.child_manager.children() {
                    for subchannel in child.subchannels() {
                        let weak = WeakSubchannel::new(&subchannel);
                        child_weaks.insert(weak.clone());

                        if child.state.connectivity_state == ConnectivityState::Ready {
                            let counter = self
                                .subchannel_counters
                                .entry(weak.clone())
                                .or_insert_with(|| Arc::new(AtomicUsize::new(0)))
                                .clone();
                            ready_subchannels.push(SubchannelWithCounter {
                                subchannel,
                                active_requests: counter,
                            });
                        }
                    }
                }

                // Clean up stale counters - retain all subchannels owned by the child manager
                self.subchannel_counters
                    .retain(|weak, _| child_weaks.contains(weak));

                let choice_count = self
                    .config
                    .as_ref()
                    .map(|cfg| cfg.choice_count as usize)
                    .unwrap_or_else(|| default_choice_count() as usize);

                Arc::new(LeastRequestPicker {
                    subchannels: ready_subchannels,
                    choice_count,
                })
            }
            ConnectivityState::Connecting
            | ConnectivityState::Idle
            | ConnectivityState::TransientFailure => {
                let pickers: Vec<Arc<dyn Picker>> = self
                    .child_manager
                    .children()
                    .filter(|cs| cs.state.connectivity_state == aggregate_state)
                    .map(|cs| cs.state.picker.clone())
                    .collect();

                if pickers.is_empty() {
                    match aggregate_state {
                        ConnectivityState::Connecting => Arc::new(QueuingPicker {}),
                        ConnectivityState::Idle => Arc::new(QueuingPicker {}),
                        ConnectivityState::TransientFailure => Arc::new(FailingPicker {
                            error: "No children in TransientFailure state".to_string(),
                        }),
                        _ => unreachable!(),
                    }
                } else if pickers.len() == 1 {
                    pickers[0].clone()
                } else {
                    Arc::new(RoundRobinPicker::new(pickers))
                }
            }
        };

        channel_controller.update_picker(LbState {
            connectivity_state: aggregate_state,
            picker,
        });
    }

    // Responds to an incoming ResolverUpdate containing an Err in endpoints by
    // forwarding it to all children unconditionally.  Updates the picker as
    // needed.
    fn handle_resolver_error(
        &mut self,
        resolver_update: ResolverUpdate,
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
        let _ = self
            .child_manager
            .resolver_update(resolver_update, None, channel_controller);
        self.update_picker(channel_controller, false);
        Err(err)
    }
}

impl LbPolicy for LeastRequestPolicy {
    type LbConfig = LeastRequestLoadBalancingConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: Option<&Self::LbConfig>,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let mut config_changed = false;
        if let Some(cfg) = config.filter(|&cfg| self.config.as_ref() != Some(cfg)) {
            self.config = Some(cfg.clone());
            config_changed = true;
        }

        if update.endpoints.is_err() {
            return self.handle_resolver_error(update, channel_controller);
        }

        // De-duplicate endpoints using a HashSet to maintain uniqueness
        let mut unique_endpoints = Vec::new();
        let mut seen = HashSet::new();
        for e in update.endpoints.unwrap() {
            if !seen.contains(&e) {
                seen.insert(e.clone());
                unique_endpoints.push(e);
            }
        }

        // Shard the update by endpoint.
        let service_config = update.service_config;
        let updates = unique_endpoints.into_iter().map(|e| {
            let update = ResolverUpdate {
                attributes: Attributes::default(),
                endpoints: Ok(vec![e.clone()]),
                service_config: service_config.clone(),
                resolution_note: None,
            };
            ChildUpdate {
                child_identifier: e,
                child_policy_builder: self.pick_first_builder.clone(),
                child_update: Some((update, None)),
            }
        });
        self.child_manager
            .update(updates, channel_controller)
            .unwrap();

        if self.child_manager.children().next().is_none() {
            // There are no children remaining, so report this error and produce
            // an erroring picker.
            let err = "Received empty address list from the name resolver";
            self.move_to_transient_failure(err.into(), channel_controller);
            return Err(err.into());
        }

        self.update_picker(channel_controller, config_changed);
        Ok(())
    }

    fn subchannel_update(
        &mut self,
        subchannel: Arc<dyn Subchannel>,
        state: &SubchannelState,
        channel_controller: &mut dyn ChannelController,
    ) {
        self.child_manager
            .subchannel_update(subchannel, state, channel_controller);
        self.update_picker(channel_controller, false);
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        self.child_manager.work(data, channel_controller);
        self.update_picker(channel_controller, false);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager.exit_idle(channel_controller);
        self.update_picker(channel_controller, false);
    }
}

/// Register least request as a LbPolicy.
pub(crate) fn reg() {
    START.call_once(|| {
        GLOBAL_LB_REGISTRY.add_builder(LeastRequestBuilder {});
    });
}

#[derive(Clone, Debug)]
struct SubchannelWithCounter {
    subchannel: Arc<dyn Subchannel>,
    active_requests: Arc<AtomicUsize>,
}

#[derive(Debug)]
struct LeastRequestPicker {
    subchannels: Vec<SubchannelWithCounter>,
    choice_count: usize,
}

impl Picker for LeastRequestPicker {
    fn pick(&self, _request_headers: &RequestHeaders) -> PickResult {
        let len = self.subchannels.len();
        if len == 0 {
            return PickResult::Queue;
        }

        let mut best_idx: Option<usize> = None;
        let mut best_active_requests = usize::MAX;

        for _ in 0..self.choice_count {
            let idx = if len == 1 {
                0
            } else {
                rand::random_range(0..len)
            };
            let active_reqs = self.subchannels[idx]
                .active_requests
                .load(Ordering::Relaxed);
            if best_idx.is_none() || active_reqs < best_active_requests {
                best_idx = Some(idx);
                best_active_requests = active_reqs;
            }
        }

        let selected_idx = best_idx.unwrap();
        let selected = &self.subchannels[selected_idx];

        selected.active_requests.fetch_add(1, Ordering::Relaxed);

        struct CompletionState {
            counter: Arc<AtomicUsize>,
            active: AtomicBool,
        }

        impl Drop for CompletionState {
            fn drop(&mut self) {
                if self.active.load(Ordering::Relaxed) {
                    self.counter.fetch_sub(1, Ordering::Relaxed);
                }
            }
        }

        let state = CompletionState {
            counter: selected.active_requests.clone(),
            active: AtomicBool::new(true),
        };

        let on_complete = Box::new(move || {
            if state.active.swap(false, Ordering::Relaxed) {
                state.counter.fetch_sub(1, Ordering::Relaxed);
            }
        });

        PickResult::Pick(Pick {
            subchannel: selected.subchannel.clone(),
            metadata: MetadataMap::new(),
            on_complete: Some(on_complete),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::load_balancing::test_utils::{TestChannelController, TestWorkScheduler};
    use crate::client::name_resolution::Address;
    use crate::rt::default_runtime;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc;

    use crate::client::load_balancing::FailingPicker;
    use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
    use crate::client::load_balancing::LbPolicy;
    use crate::client::load_balancing::LbState;
    use crate::client::load_balancing::Pick;
    use crate::client::load_balancing::PickResult;
    use crate::client::load_balancing::Picker;
    use crate::client::load_balancing::QueuingPicker;
    use crate::client::load_balancing::pick_first;
    use crate::client::load_balancing::test_utils;
    use crate::client::load_balancing::test_utils::{StubPolicyData, StubPolicyFuncs, TestEvent};
    use crate::metadata::MetadataMap;

    fn setup(
        test_name: &'static str,
    ) -> (
        mpsc::Receiver<TestEvent>,
        LeastRequestPolicy,
        Box<dyn ChannelController>,
    ) {
        pick_first::reg();
        super::reg();
        test_utils::reg_stub_policy(test_name, create_funcs_for_leastrequest_tests());

        let (tx_events, rx_events) = mpsc::channel();
        let work_scheduler = Arc::new(TestWorkScheduler {
            tx_events: tx_events.clone(),
        });
        let child_manager = ChildManager::new(default_runtime(), work_scheduler);
        let tcc = Box::new(TestChannelController { tx_events });
        let child_policy_builder = GLOBAL_LB_REGISTRY.get_policy(test_name).unwrap();
        let lb_policy = LeastRequestPolicy::new(child_manager, child_policy_builder);
        (rx_events, lb_policy, tcc)
    }

    struct TestSubchannelList {
        subchannels: Vec<Arc<dyn Subchannel>>,
    }

    impl TestSubchannelList {
        fn new(addresses: &[Address], channel_controller: &mut dyn ChannelController) -> Self {
            TestSubchannelList {
                subchannels: addresses
                    .iter()
                    .map(|a| channel_controller.new_subchannel(a).0)
                    .collect(),
            }
        }

        fn contains(&self, sc: &Arc<dyn Subchannel>) -> bool {
            self.subchannels.contains(sc)
        }
    }

    fn create_endpoints(num_endpoints: usize, num_addresses: usize) -> Vec<Endpoint> {
        let mut endpoints = Vec::with_capacity(num_endpoints);
        for i in 0..num_endpoints {
            let mut addresses: Vec<Address> = Vec::with_capacity(num_addresses);
            for j in 0..num_addresses {
                addresses.push(Address {
                    address: format!("{}.{}.{}.{}:{}", i + 1, i + 1, i + 1, i + 1, j).into(),
                    ..Default::default()
                });
            }
            endpoints.push(Endpoint {
                addresses,
                ..Default::default()
            })
        }
        endpoints
    }

    fn verify_subchannel_creation(
        rx_events: &mut mpsc::Receiver<TestEvent>,
        number_of_subchannels: usize,
    ) -> Vec<Arc<dyn Subchannel>> {
        let mut subchannels = Vec::new();
        for _ in 0..number_of_subchannels {
            match rx_events.recv().unwrap() {
                TestEvent::NewSubchannel(sc) => {
                    subchannels.push(sc);
                }
                other => panic!("unexpected event {:?}", other),
            };
        }
        subchannels
    }

    fn verify_connecting_picker(rx_events: &mut mpsc::Receiver<TestEvent>) -> Arc<dyn Picker> {
        match rx_events.recv().unwrap() {
            TestEvent::UpdatePicker(update) => {
                assert_eq!(update.connectivity_state, ConnectivityState::Connecting);
                let req = test_utils::new_request_headers();
                assert_eq!(update.picker.pick(&req), PickResult::Queue);
                update.picker
            }
            other => panic!("unexpected event {:?}", other),
        }
    }

    fn verify_ready_picker(rx_events: &mut mpsc::Receiver<TestEvent>) -> Arc<dyn Picker> {
        match rx_events.recv().unwrap() {
            TestEvent::UpdatePicker(update) => {
                assert_eq!(update.connectivity_state, ConnectivityState::Ready);
                update.picker.clone()
            }
            other => panic!("unexpected event {:?}", other),
        }
    }

    fn verify_transient_failure_picker(
        rx_events: &mut mpsc::Receiver<TestEvent>,
    ) -> Arc<dyn Picker> {
        match rx_events.recv().unwrap() {
            TestEvent::UpdatePicker(update) => {
                assert_eq!(
                    update.connectivity_state,
                    ConnectivityState::TransientFailure
                );
                update.picker.clone()
            }
            other => panic!("unexpected event {:?}", other),
        }
    }

    struct PickFirstState {
        subchannel_list: Option<TestSubchannelList>,
        addresses: Vec<Address>,
        connectivity_state: ConnectivityState,
    }

    fn create_funcs_for_leastrequest_tests() -> StubPolicyFuncs {
        StubPolicyFuncs {
            resolver_update: Some(Arc::new(
                |data: &mut StubPolicyData, update: ResolverUpdate, _, channel_controller| {
                    let state = data
                        .test_data
                        .get_or_insert_with(|| {
                            Box::new(PickFirstState {
                                subchannel_list: None,
                                addresses: vec![],
                                connectivity_state: ConnectivityState::Connecting,
                            })
                        })
                        .downcast_mut::<PickFirstState>()
                        .unwrap();
                    if let Err(error) = update.endpoints {
                        channel_controller.update_picker(LbState {
                            connectivity_state: ConnectivityState::TransientFailure,
                            picker: Arc::new(FailingPicker {
                                error: error.to_string(),
                            }),
                        });
                        state.connectivity_state = ConnectivityState::TransientFailure;
                        channel_controller.request_resolution();
                        return Ok(());
                    };
                    let endpoints = update.endpoints.unwrap();
                    let mut addresses = Vec::new();
                    for ep in endpoints {
                        addresses.extend(ep.addresses.clone());
                    }
                    if addresses.is_empty() {
                        channel_controller.update_picker(LbState {
                            connectivity_state: ConnectivityState::TransientFailure,
                            picker: Arc::new(FailingPicker {
                                error: "Received empty address list from the name resolver"
                                    .to_string(),
                            }),
                        });
                        state.connectivity_state = ConnectivityState::TransientFailure;
                        channel_controller.request_resolution();
                        return Err("Received empty address list from the name resolver".into());
                    }

                    if state.connectivity_state != ConnectivityState::Idle {
                        state.subchannel_list =
                            Some(TestSubchannelList::new(&addresses, channel_controller));
                    }
                    state.addresses = addresses;
                    Ok(())
                },
            )),
            subchannel_update: Some(Arc::new(
                |data: &mut StubPolicyData, subchannel, state, channel_controller| {
                    let test_data = data.test_data.as_mut().unwrap();
                    let test_state = test_data.downcast_mut::<PickFirstState>().unwrap();
                    let scl = &mut test_state.subchannel_list.as_ref().unwrap();
                    assert!(
                        scl.contains(&subchannel),
                        "subchannel_update received an update for a subchannel it does not own."
                    );
                    test_state.connectivity_state = state.connectivity_state;
                    match state.connectivity_state {
                        ConnectivityState::Ready => {
                            channel_controller.update_picker(LbState {
                                connectivity_state: state.connectivity_state,
                                picker: Arc::new(OneSubchannelPicker { sc: subchannel }),
                            });
                        }
                        ConnectivityState::Idle => {}
                        ConnectivityState::Connecting => {
                            channel_controller.update_picker(LbState {
                                connectivity_state: state.connectivity_state,
                                picker: Arc::new(QueuingPicker {}),
                            });
                        }
                        ConnectivityState::TransientFailure => {
                            channel_controller.update_picker(LbState {
                                connectivity_state: state.connectivity_state,
                                picker: Arc::new(FailingPicker {
                                    error: state
                                        .last_connection_error
                                        .as_ref()
                                        .unwrap()
                                        .to_string(),
                                }),
                            });
                        }
                    }
                },
            )),
            ..Default::default()
        }
    }

    #[derive(Debug)]
    struct OneSubchannelPicker {
        sc: Arc<dyn Subchannel>,
    }

    impl Picker for OneSubchannelPicker {
        fn pick(&self, _: &RequestHeaders) -> PickResult {
            PickResult::Pick(Pick {
                subchannel: self.sc.clone(),
                on_complete: None,
                metadata: MetadataMap::new(),
            })
        }
    }

    #[derive(Debug, Clone)]
    struct MockSubchannel {
        address: Address,
    }

    impl crate::client::load_balancing::subchannel::private::Sealed for MockSubchannel {}
    impl crate::client::load_balancing::subchannel::DynHash for MockSubchannel {
        fn dyn_hash(&self, state: &mut Box<&mut dyn std::hash::Hasher>) {
            use std::hash::Hash;
            self.address.hash(state);
        }
    }
    impl crate::client::load_balancing::subchannel::DynPartialEq for MockSubchannel {
        fn dyn_eq(&self, other: &&dyn std::any::Any) -> bool {
            if let Some(other) = other.downcast_ref::<Self>() {
                self.address == other.address
            } else {
                false
            }
        }
    }
    impl Subchannel for MockSubchannel {
        fn address(&self) -> Address {
            self.address.clone()
        }
        fn get_attribute_dyn(&self, _id: std::any::TypeId) -> Option<&dyn std::any::Any> {
            None
        }
        fn connect(&self) {}
    }

    #[test]
    fn test_config_parsing() {
        let builder = LeastRequestBuilder {};

        // Default choice count
        let default_config = ParsedJsonLbConfig::new("{}").unwrap();
        let parsed = builder.parse_config(&default_config).unwrap().unwrap();
        assert_eq!(parsed.choice_count, 2);

        // Explicit valid choice count
        let valid_config = ParsedJsonLbConfig::new("{\"choiceCount\": 5}").unwrap();
        let parsed = builder.parse_config(&valid_config).unwrap().unwrap();
        assert_eq!(parsed.choice_count, 5);

        // Clamped choice count
        let high_config = ParsedJsonLbConfig::new("{\"choiceCount\": 15}").unwrap();
        let parsed = builder.parse_config(&high_config).unwrap().unwrap();
        assert_eq!(parsed.choice_count, 10);

        // Rejected choice count
        let low_config = ParsedJsonLbConfig::new("{\"choiceCount\": 1}").unwrap();
        assert!(builder.parse_config(&low_config).is_err());
    }

    #[test]
    fn test_picker_least_request_selection() {
        let sc1 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:80".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let sc2 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:81".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let count1 = Arc::new(AtomicUsize::new(5));
        let count2 = Arc::new(AtomicUsize::new(2));

        let picker = LeastRequestPicker {
            subchannels: vec![
                SubchannelWithCounter {
                    subchannel: sc1.clone(),
                    active_requests: count1.clone(),
                },
                SubchannelWithCounter {
                    subchannel: sc2.clone(),
                    active_requests: count2.clone(),
                },
            ],
            choice_count: 2,
        };

        let mut picked_sc2 = false;
        for _ in 0..20 {
            let res = picker.pick(&RequestHeaders::default());
            let pick = res.unwrap_pick();
            if pick.subchannel.address().address == "127.0.0.1:81".to_string().into() {
                picked_sc2 = true;
                assert_eq!(count2.load(Ordering::Relaxed), 3);
                let on_complete = pick.on_complete.unwrap();
                on_complete();
                assert_eq!(count2.load(Ordering::Relaxed), 2);
                break;
            }
        }
        assert!(
            picked_sc2,
            "sc2 (with fewer requests) was never picked in 20 attempts"
        );
    }

    #[test]
    fn test_picker_tie_breaking() {
        let sc1 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:80".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let sc2 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:81".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let count1 = Arc::new(AtomicUsize::new(2));
        let count2 = Arc::new(AtomicUsize::new(2));

        let picker = LeastRequestPicker {
            subchannels: vec![
                SubchannelWithCounter {
                    subchannel: sc1.clone(),
                    active_requests: count1.clone(),
                },
                SubchannelWithCounter {
                    subchannel: sc2.clone(),
                    active_requests: count2.clone(),
                },
            ],
            choice_count: 2,
        };

        let res = picker.pick(&RequestHeaders::default());
        let pick = res.unwrap_pick();
        let chosen_addr = pick.subchannel.address().address.to_string();
        assert!(chosen_addr == "127.0.0.1:80" || chosen_addr == "127.0.0.1:81");
    }

    #[test]
    fn test_picker_fewer_subchannels_than_choice_count() {
        let sc1 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:80".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let sc2 = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:81".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let count1 = Arc::new(AtomicUsize::new(5));
        let count2 = Arc::new(AtomicUsize::new(2));

        let picker = LeastRequestPicker {
            subchannels: vec![
                SubchannelWithCounter {
                    subchannel: sc1.clone(),
                    active_requests: count1.clone(),
                },
                SubchannelWithCounter {
                    subchannel: sc2.clone(),
                    active_requests: count2.clone(),
                },
            ],
            choice_count: 3,
        };

        let mut picked_sc2 = false;
        for _ in 0..20 {
            let res = picker.pick(&RequestHeaders::default());
            let pick = res.unwrap_pick();
            if pick.subchannel.address().address == "127.0.0.1:81".to_string().into() {
                picked_sc2 = true;
                assert_eq!(count2.load(Ordering::Relaxed), 3);
                let on_complete = pick.on_complete.unwrap();
                on_complete();
                assert_eq!(count2.load(Ordering::Relaxed), 2);
                break;
            }
        }
        assert!(
            picked_sc2,
            "sc2 (with fewer requests) was never picked in 20 attempts"
        );
    }

    #[test]
    fn test_picker_cancellation_drop_guard() {
        let sc = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:80".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let count = Arc::new(AtomicUsize::new(5));

        let picker = LeastRequestPicker {
            subchannels: vec![SubchannelWithCounter {
                subchannel: sc,
                active_requests: count.clone(),
            }],
            choice_count: 1,
        };

        let res = picker.pick(&RequestHeaders::default());
        assert_eq!(count.load(Ordering::Relaxed), 6);
        drop(res);
        assert_eq!(count.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_policy_empty_resolver_update() {
        let (tx_events, _rx_events) = mpsc::channel();
        let work_scheduler = Arc::new(TestWorkScheduler {
            tx_events: tx_events.clone(),
        });
        let child_manager = ChildManager::new(default_runtime(), work_scheduler);
        pick_first::reg();
        let pick_first_builder = GLOBAL_LB_REGISTRY
            .get_policy(pick_first::POLICY_NAME)
            .unwrap();

        let mut policy = LeastRequestPolicy::new(child_manager, pick_first_builder);
        let mut tcc = TestChannelController { tx_events };

        let update = ResolverUpdate {
            endpoints: Ok(vec![]),
            ..Default::default()
        };

        let res = policy.resolver_update(update, None, &mut tcc);
        assert!(res.is_err());
    }

    #[test]
    fn test_endpoint_deduplication() {
        let (mut rx_events, mut lb_policy, mut tcc) = setup("stub-test_endpoint_deduplication");
        let tcc = tcc.as_mut();

        let endpoint = create_endpoints(1, 1)[0].clone();
        let endpoints = vec![endpoint.clone(), endpoint];

        let update = ResolverUpdate {
            endpoints: Ok(endpoints),
            ..Default::default()
        };
        let _ = lb_policy.resolver_update(update, None, tcc);

        let subchannels = verify_subchannel_creation(&mut rx_events, 1);
        assert_eq!(subchannels.len(), 1);
    }

    #[test]
    fn test_connectivity_state_aggregation_all_cases() {
        let (mut rx_events, mut lb_policy, mut tcc) =
            setup("stub-test_connectivity_state_aggregation");
        let tcc = tcc.as_mut();

        let endpoints = create_endpoints(2, 1);
        let update = ResolverUpdate {
            endpoints: Ok(endpoints),
            ..Default::default()
        };
        let _ = lb_policy.resolver_update(update, None, tcc);

        let subchannels = verify_subchannel_creation(&mut rx_events, 2);

        lb_policy.subchannel_update(subchannels[0].clone(), &SubchannelState::connecting(), tcc);
        lb_policy.subchannel_update(subchannels[1].clone(), &SubchannelState::connecting(), tcc);

        for _ in 0..2 {
            verify_connecting_picker(&mut rx_events);
        }

        lb_policy.subchannel_update(subchannels[0].clone(), &SubchannelState::ready(), tcc);
        verify_ready_picker(&mut rx_events);
    }

    #[test]
    fn test_picker_roundrobin_non_ready_states() {
        let (mut rx_events, mut lb_policy, mut tcc) =
            setup("stub-test_picker_roundrobin_non_ready");
        let tcc = tcc.as_mut();

        let endpoints = create_endpoints(2, 1);
        let update = ResolverUpdate {
            endpoints: Ok(endpoints),
            ..Default::default()
        };
        let _ = lb_policy.resolver_update(update, None, tcc);

        let subchannels = verify_subchannel_creation(&mut rx_events, 2);

        lb_policy.subchannel_update(
            subchannels[0].clone(),
            &SubchannelState {
                connectivity_state: ConnectivityState::TransientFailure,
                last_connection_error: Some("connection error 0".into()),
            },
            tcc,
        );
        verify_connecting_picker(&mut rx_events);

        lb_policy.subchannel_update(
            subchannels[1].clone(),
            &SubchannelState {
                connectivity_state: ConnectivityState::TransientFailure,
                last_connection_error: Some("connection error 1".into()),
            },
            tcc,
        );

        let picker = verify_transient_failure_picker(&mut rx_events);
        let headers = RequestHeaders::default();

        let res1 = picker.pick(&headers);
        let res2 = picker.pick(&headers);
        let res3 = picker.pick(&headers);

        let msg1 = match res1 {
            PickResult::Fail(status) => status.message().to_string(),
            other => panic!("expected Fail, got {:?}", other),
        };
        let msg2 = match res2 {
            PickResult::Fail(status) => status.message().to_string(),
            other => panic!("expected Fail, got {:?}", other),
        };
        let msg3 = match res3 {
            PickResult::Fail(status) => status.message().to_string(),
            other => panic!("expected Fail, got {:?}", other),
        };

        assert_ne!(msg1, msg2);
        assert_eq!(msg1, msg3);
        assert!(msg1.contains("connection error 0") || msg1.contains("connection error 1"));
        assert!(msg2.contains("connection error 0") || msg2.contains("connection error 1"));
    }

    #[test]
    fn stress_test_p2c_selection_uniformity() {
        // Create 5 subchannels
        let mut subchannels = Vec::new();
        let mut counters = Vec::new();
        for i in 0..5 {
            let sc = Arc::new(MockSubchannel {
                address: Address {
                    address: format!("127.0.0.1:{}", 80 + i).into(),
                    ..Default::default()
                },
            }) as Arc<dyn Subchannel>;
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            counters.push(count.clone());
            subchannels.push(SubchannelWithCounter {
                subchannel: sc,
                active_requests: count,
            });
        }

        let picker = LeastRequestPicker {
            subchannels,
            choice_count: 2,
        };

        let mut selection_counts = vec![0; 5];
        for _ in 0..10000 {
            // Reset counters to ensure they are perfectly equal
            for count in &counters {
                count.store(0, Ordering::Relaxed);
            }

            let res = picker.pick(&RequestHeaders::default());
            let pick = res.unwrap_pick();
            let port = pick
                .subchannel
                .address()
                .address
                .split(':')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let idx = port - 80;
            selection_counts[idx] += 1;
        }

        println!(
            "Selection counts for 5 subchannels over 10000 runs: {:?}",
            selection_counts
        );

        // Expected count = 2000. Standard deviation = 40.
        // Assert that all counts are within 4 std dev of expected (1840 to 2160).
        for (i, count) in selection_counts.iter().enumerate() {
            assert!(
                *count >= 1840 && *count <= 2160,
                "Subchannel {} selection count {} was outside the highly-probable range [1840, 2160]",
                i,
                count
            );
        }
    }

    #[test]
    fn stress_test_completion_state_thread_safety() {
        use std::sync::Barrier;
        use std::thread;

        let sc = Arc::new(MockSubchannel {
            address: Address {
                address: "127.0.0.1:80".to_string().into(),
                ..Default::default()
            },
        }) as Arc<dyn Subchannel>;

        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let picker = Arc::new(LeastRequestPicker {
            subchannels: vec![SubchannelWithCounter {
                subchannel: sc,
                active_requests: counter.clone(),
            }],
            choice_count: 1,
        });

        const NUM_THREADS: usize = 20;
        const ITERATIONS: usize = 500;
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        let mut handles = Vec::new();

        for _ in 0..NUM_THREADS {
            let picker = picker.clone();
            let barrier = barrier.clone();
            handles.push(thread::spawn(move || {
                barrier.wait();
                for i in 0..ITERATIONS {
                    let res = picker.pick(&RequestHeaders::default());
                    let pick = res.unwrap_pick();

                    if (i + rand::random_range(0..100)) % 2 == 0 {
                        if let Some(on_complete) = pick.on_complete {
                            on_complete();
                        }
                    } else {
                        // Just let it drop naturally
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // After all threads completed, the counter MUST be exactly 0!
        assert_eq!(
            counter.load(Ordering::SeqCst),
            0,
            "Counter leaked or underflowed!"
        );
    }

    #[test]
    fn stress_test_dynamic_config_invalidation() {
        let (mut rx_events, mut lb_policy, mut tcc) =
            setup("stub-test_dynamic_config_invalidation");
        let tcc = tcc.as_mut();

        let endpoints = create_endpoints(3, 1);
        let update = ResolverUpdate {
            endpoints: Ok(endpoints),
            ..Default::default()
        };

        let config2 = LeastRequestLoadBalancingConfig { choice_count: 2 };
        let _ = lb_policy.resolver_update(update.clone(), Some(&config2), tcc);

        let subchannels = verify_subchannel_creation(&mut rx_events, 3);
        verify_connecting_picker(&mut rx_events); // Consume the initial Connecting picker from resolver_update

        lb_policy.subchannel_update(subchannels[0].clone(), &SubchannelState::ready(), tcc);
        let _ = verify_ready_picker(&mut rx_events); // Consume the first Ready picker

        lb_policy.subchannel_update(subchannels[1].clone(), &SubchannelState::ready(), tcc);
        let _ = verify_ready_picker(&mut rx_events); // Consume the second Ready picker

        lb_policy.subchannel_update(subchannels[2].clone(), &SubchannelState::ready(), tcc);
        let picker = verify_ready_picker(&mut rx_events); // Consume the third Ready picker

        let dbg_format = format!("{:?}", picker);
        assert!(
            dbg_format.contains("choice_count: 2"),
            "Expected choice_count to be 2, got {:?}",
            dbg_format
        );

        let config3 = LeastRequestLoadBalancingConfig { choice_count: 3 };
        let _ = lb_policy.resolver_update(update.clone(), Some(&config3), tcc);

        let _ = verify_subchannel_creation(&mut rx_events, 3); // Consume the new subchannel events from stub policy
        let picker = verify_ready_picker(&mut rx_events);
        let dbg_format = format!("{:?}", picker);
        assert!(
            dbg_format.contains("choice_count: 3"),
            "Expected choice_count to be 3, got {:?}",
            dbg_format
        );

        let _ = lb_policy.resolver_update(update, Some(&config2), tcc);
        let _ = verify_subchannel_creation(&mut rx_events, 3); // Consume the new subchannel events from stub policy
        let picker = verify_ready_picker(&mut rx_events);
        let dbg_format = format!("{:?}", picker);
        assert!(
            dbg_format.contains("choice_count: 2"),
            "Expected choice_count to be 2, got {:?}",
            dbg_format
        );
    }
}
