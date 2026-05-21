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
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

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
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::Subchannel;
use crate::client::load_balancing::SubchannelState;
use crate::client::load_balancing::child_manager::ChildManager;
use crate::client::load_balancing::child_manager::ChildUpdate;
use crate::client::load_balancing::pick_first;
use crate::client::load_balancing::subchannel::WeakSubchannel;
use crate::client::name_resolution::Endpoint;
use crate::client::name_resolution::ResolverUpdate;
use crate::core::RequestHeaders;

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
    ) -> Result<Option<<Self::LbPolicy as LbPolicy>::LbConfig>, String> {
        let parsed: LeastRequestLoadBalancingConfig = config
            .convert_to()
            .map_err(|e| format!("failed to parse least_request config: {e}"))?;

        if parsed.choice_count < 2 {
            return Err("choice_count must be at least 2".to_string());
        }

        let choice_count = parsed.choice_count.min(10);
        Ok(Some(LeastRequestLoadBalancingConfig { choice_count }))
    }
}

#[derive(Debug)]
pub(crate) struct LeastRequestPolicy {
    child_manager: ChildManager<Endpoint>,
    pick_first_builder: Arc<DynLbPolicyBuilder>,
    choice_count: u32,
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
            choice_count: 2,
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
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController) {
        if !self.child_manager.child_updated() {
            return;
        }
        let aggregate_state = self.child_manager.aggregate_states();

        if aggregate_state == ConnectivityState::Ready {
            let mut ready_subchannels = Vec::new();
            for child in self.child_manager.children() {
                if child.state.connectivity_state == ConnectivityState::Ready {
                    if let PickResult::Pick(pick) = child
                        .state
                        .picker
                        .pick(&crate::core::RequestHeaders::default())
                    {
                        let weak = WeakSubchannel::new(&pick.subchannel);
                        let counter = self
                            .subchannel_counters
                            .entry(weak)
                            .or_insert_with(|| Arc::new(AtomicUsize::new(0)))
                            .clone();
                        ready_subchannels.push(SubchannelWithCounter {
                            subchannel: pick.subchannel.clone(),
                            active_requests: counter,
                        });
                    }
                }
            }

            // Clean up stale counters
            self.subchannel_counters
                .retain(|weak, _| weak.upgrade().is_some());

            let picker_update = LbState {
                connectivity_state: aggregate_state,
                picker: Arc::new(LeastRequestPicker {
                    subchannels: ready_subchannels,
                    choice_count: self.choice_count as usize,
                }),
            };
            channel_controller.update_picker(picker_update);
        } else {
            // Forward the child picker for non-ready aggregate state
            let picker = self
                .child_manager
                .children()
                .find(|cs| cs.state.connectivity_state == aggregate_state)
                .map(|cs| cs.state.picker.clone())
                .unwrap_or_else(|| {
                    Arc::new(crate::client::load_balancing::QueuingPicker) as Arc<dyn Picker>
                });

            channel_controller.update_picker(LbState {
                connectivity_state: aggregate_state,
                picker,
            });
        }
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
        self.update_picker(channel_controller);
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
        if let Some(cfg) = config {
            self.choice_count = cfg.choice_count;
        }

        if update.endpoints.is_err() {
            return self.handle_resolver_error(update, channel_controller);
        }

        // Shard the update by endpoint.
        let updates = update.endpoints.as_ref().unwrap().iter().map(|e| {
            let update = ResolverUpdate {
                attributes: crate::attributes::Attributes::default(),
                endpoints: Ok(vec![e.clone()]),
                service_config: update.service_config.clone(),
                resolution_note: None,
            };
            ChildUpdate {
                child_identifier: e.clone(),
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

        self.update_picker(channel_controller);
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
        self.update_picker(channel_controller);
    }

    fn work(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager.work(channel_controller);
        self.update_picker(channel_controller);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.child_manager.exit_idle(channel_controller);
        self.update_picker(channel_controller);
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

        let sample_limit = self.choice_count.min(len);
        let mut best_idx: Option<usize> = None;
        let mut best_active_requests = usize::MAX;

        for _ in 0..sample_limit {
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

        let active = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let counter = selected.active_requests.clone();

        struct ActiveRequestGuard {
            counter: Arc<AtomicUsize>,
            active: Arc<std::sync::atomic::AtomicBool>,
        }

        impl Drop for ActiveRequestGuard {
            fn drop(&mut self) {
                if self.active.swap(false, Ordering::Relaxed) {
                    self.counter.fetch_sub(1, Ordering::Relaxed);
                }
            }
        }

        let guard = ActiveRequestGuard {
            counter: counter.clone(),
            active: active.clone(),
        };

        let counter_clone = counter.clone();
        let on_complete = Box::new(move || {
            if active.swap(false, Ordering::Relaxed) {
                counter_clone.fetch_sub(1, Ordering::Relaxed);
            }
            let _ = &guard;
        });

        PickResult::Pick(crate::client::load_balancing::Pick {
            subchannel: selected.subchannel.clone(),
            metadata: crate::metadata::MetadataMap::new(),
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

        // Run the pick in a loop up to 20 times since random sampling with replacement
        // might occasionally select sc1 twice (with 25% probability).
        let mut picked_sc2 = false;
        for _ in 0..20 {
            let res = picker.pick(&RequestHeaders::default());
            let pick = res.unwrap_pick();
            if pick.subchannel.address().address == "127.0.0.1:81".to_string().into() {
                picked_sc2 = true;
                // Active request count of the selected subchannel should have incremented
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

        // With identical active request counts, either sc1 or sc2 should be chosen
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
            // choice_count is 3, but only 2 subchannels are available
            choice_count: 3,
        };

        // Picker should handle this gracefully by sampling both subchannels,
        // and picking the one with fewer active requests (sc2).
        let mut picked_sc2 = false;
        for _ in 0..20 {
            let res = picker.pick(&RequestHeaders::default());
            let pick = res.unwrap_pick();
            if pick.subchannel.address().address == "127.0.0.1:81".to_string().into() {
                picked_sc2 = true;
                // Active request count of the selected subchannel should have incremented
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

        // Pick once
        let res = picker.pick(&RequestHeaders::default());
        assert_eq!(count.load(Ordering::Relaxed), 6);

        // Simulate cancellation/drop of Pick without calling on_complete callback
        drop(res);

        // Count must have been decremented back to 5 by the Drop guard
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
}
