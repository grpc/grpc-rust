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

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;

use super::*;
use crate::client::ConnectivityState;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::ParsedJsonLbConfig;
use crate::client::load_balancing::QueuingPicker;
use crate::client::load_balancing::Subchannel;
use crate::client::load_balancing::SubchannelState;
use crate::client::load_balancing::endpoint_filtering;
use crate::client::load_balancing::pick_first::PickFirstConfig;
use crate::client::load_balancing::test_utils;
use crate::client::load_balancing::test_utils::StubPolicyFuncs;
use crate::client::load_balancing::test_utils::TestChannelController;
use crate::client::load_balancing::test_utils::TestEvent;
use crate::client::load_balancing::test_utils::TestWorkScheduler;
use crate::client::name_resolution::Endpoint;
use crate::client::name_resolution::ResolverUpdate;
use crate::core::Address;
use crate::rt::default_runtime;

/// Verifies that configuration parsing rejects configs where a priority
/// name in `priorities` has no corresponding entry in `children`.
#[test]
fn parse_config_child_not_found() {
    let js = r#"{
  "priorities": ["child-1", "child-2", "child-3"],
  "children": {
    "child-1": {"config": [{"round_robin":{}}]},
    "child-3": {"config": [{"round_robin":{}}]}
  }
}"#;
    let builder = Builder {};
    let got = ParsedJsonLbConfig::new(js).and_then(|cfg| builder.parse_config(&cfg));
    assert!(got.is_err());
}

/// Verifies that configuration parsing rejects configs where a child in
/// `children` is not listed in `priorities`.
#[test]
fn parse_config_child_not_used() {
    let js = r#"{
  "priorities": ["child-1", "child-2"],
  "children": {
    "child-1": {"config": [{"round_robin":{}}]},
    "child-2": {"config": [{"round_robin":{}}]},
    "child-3": {"config": [{"round_robin":{}}]}
  }
}"#;
    let builder = Builder {};
    let got = ParsedJsonLbConfig::new(js).and_then(|cfg| builder.parse_config(&cfg));
    assert!(got.is_err());
}

/// Verifies successful parsing of a valid multi-priority configuration
/// containing multiple child policy types and `ignoreReresolutionRequests`.
#[test]
fn parse_config_success() {
    let js = r#"{
  "priorities": ["child-1", "child-2", "child-3"],
  "children": {
    "child-1": {"config": [{"round_robin":{}}], "ignoreReresolutionRequests": true},
    "child-2": {"config": [{"pick_first": {"shuffleAddressList": true}}]},
    "child-3": {"config": [{"round_robin":{}}]}
  }
}"#;
    let builder = Builder {};
    let got = ParsedJsonLbConfig::new(js)
        .and_then(|cfg| builder.parse_config(&cfg))
        .unwrap()
        .unwrap();

    assert_eq!(got.priorities, vec!["child-1", "child-2", "child-3"]);
    assert_eq!(got.children.len(), 3);

    let child1 = got.children.get("child-1").unwrap();
    assert!(child1.ignore_reresolution_requests);
    assert_eq!(child1.config.builder.name(), "round_robin");
    assert!(child1.config.config.is_none());

    let child2 = got.children.get("child-2").unwrap();
    assert!(!child2.ignore_reresolution_requests);
    assert_eq!(child2.config.builder.name(), "pick_first");
    let pf_cfg = child2
        .config
        .config
        .as_ref()
        .unwrap()
        .downcast_ref::<PickFirstConfig>()
        .unwrap();
    assert!(pf_cfg.shuffle_address_list);

    let child3 = got.children.get("child-3").unwrap();
    assert!(!child3.ignore_reresolution_requests);
    assert_eq!(child3.config.builder.name(), "round_robin");
    assert!(child3.config.config.is_none());
}

/// Test environment container holding a PriorityPolicy, channel controller,
/// receiver for test events, and the builder.
struct TestEnv {
    rx_events: mpsc::Receiver<TestEvent>,
    policy: PriorityPolicy,
    tcc: TestChannelController,
    builder: Builder,
}

fn setup_test_env() -> TestEnv {
    let (tx_events, rx_events) = mpsc::channel::<TestEvent>();
    let tcc = TestChannelController {
        tx_events: tx_events.clone(),
    };
    let work_scheduler = Arc::new(TestWorkScheduler { tx_events });
    let rt = default_runtime();
    let builder = Builder {};
    let policy = builder.build(LbPolicyOptions {
        runtime: rt,
        work_scheduler,
    });
    TestEnv {
        rx_events,
        policy,
        tcc,
        builder,
    }
}

/// Helper to receive the next `ScheduleWork` event from the test channel.
fn recv_schedule_work(rx: &mpsc::Receiver<TestEvent>) -> Option<WorkData> {
    loop {
        match rx.recv().unwrap() {
            TestEvent::ScheduleWork(data) => return data,
            _ => continue,
        }
    }
}

/// Advances virtual time by `duration` on the paused Tokio runtime.
///
/// Yields execution before advancing so that any newly spawned background timer
/// tasks (such as those spawned by `Timer::new`) have a chance to execute up to
/// their first `.await` point, polling their `sleep` future and registering on
/// Tokio's timer wheel. After advancing the clock, yields again to allow woken
/// timer tasks to execute their completion continuations (such as scheduling
/// work).
async fn advance_time(duration: Duration) {
    tokio::task::yield_now().await;
    tokio::time::advance(duration).await;
    tokio::task::yield_now().await;
}

/// Helper creating an Endpoint configured with a hierarchical path.
fn new_test_endpoint(child_name: &str, addr: &str) -> Endpoint {
    let endpoint = Endpoint {
        addresses: vec![Address {
            address: addr.to_string().into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    endpoint_filtering::set_path_in_endpoint(endpoint, vec![child_name.to_string()])
}

#[derive(Clone, Default)]
struct ControllableStubHandle {
    subchannel: Arc<Mutex<Option<Arc<dyn Subchannel>>>>,
    resolution_requested: Arc<Mutex<bool>>,
}

impl ControllableStubHandle {
    fn new() -> Self {
        Self::default()
    }

    fn subchannel(&self) -> Option<Arc<dyn Subchannel>> {
        self.subchannel.lock().unwrap().clone()
    }

    fn get_subchannel(&self) -> Option<Arc<dyn Subchannel>> {
        self.subchannel()
    }

    fn set_subchannel(&self, subchannel: Option<Arc<dyn Subchannel>>) {
        *self.subchannel.lock().unwrap() = subchannel;
    }

    fn resolution_requested(&self) -> bool {
        *self.resolution_requested.lock().unwrap()
    }

    fn get_resolution_requested(&self) -> bool {
        self.resolution_requested()
    }

    fn set_resolution_requested(&self, requested: bool) {
        *self.resolution_requested.lock().unwrap() = requested;
    }
}

/// Helper registering a stub child LB policy that creates a subchannel and
/// reports connectivity updates.
fn new_stub(policy_name: &'static str) -> ControllableStubHandle {
    let handle = ControllableStubHandle::new();
    let handle_clone = handle.clone();

    let funcs = StubPolicyFuncs {
        resolver_update: Some(Arc::new({
            let handle = handle_clone.clone();
            move |_data, update, _cfg, controller| {
                let addr = update
                    .endpoints
                    .as_ref()
                    .ok()
                    .and_then(|e| e.first())
                    .and_then(|e| e.addresses.first())
                    .cloned()
                    .unwrap_or_default();
                let (subchannel, _) = controller.new_subchannel(&addr);
                handle.set_subchannel(Some(subchannel));
                Ok(())
            }
        })),
        subchannel_update: Some(Arc::new({
            let handle = handle_clone;
            move |_data, _subchannel, state, controller| {
                if state.connectivity_state == ConnectivityState::TransientFailure {
                    handle.set_resolution_requested(true);
                    controller.request_resolution();
                }
                controller.update_picker(LbState {
                    connectivity_state: state.connectivity_state,
                    picker: Arc::new(QueuingPicker {}),
                });
            }
        })),
        ..Default::default()
    };
    test_utils::reg_stub_policy(policy_name, funcs);
    handle
}

/// Verifies that an empty priority list immediately reports
/// TRANSIENT_FAILURE with an explanatory failing picker.
#[tokio::test]
async fn empty_priorities_reports_transient_failure() {
    let mut env = setup_test_env();
    let js = r#"{
  "priorities": [],
  "children": {}
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let picker_event = env
        .rx_events
        .try_iter()
        .find_map(|e| match e {
            TestEvent::UpdatePicker(state) => Some(state),
            _ => None,
        })
        .expect("expected UpdatePicker event");
    assert_eq!(
        picker_event.connectivity_state,
        ConnectivityState::TransientFailure
    );
}

/// Verifies that when a high-priority child is READY, traffic routes to it
/// and adding or removing lower priorities does not cause connection
/// churn or initialize unused children.
#[tokio::test]
async fn high_priority_ready_and_add_remove_lower() {
    let stub_handle0 = new_stub("stub_hr_0");
    let stub_handle1 = new_stub("stub_hr_1");
    let stub_handle2 = new_stub("stub_hr_2");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_hr_0": {}}]},
    "child-1": {"config": [{"stub_hr_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    // child-0 should be lazily instantiated and connecting.
    let sc0 = stub_handle0
        .subchannel()
        .expect("child-0 subchannel should be created");
    assert!(
        stub_handle1.subchannel().is_none(),
        "child-1 should NOT be created while child-0 is connecting"
    );

    // Make child-0 Ready.
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::ready(), &mut env.tcc);

    let last_picker = env
        .rx_events
        .try_iter()
        .filter_map(|e| match e {
            TestEvent::UpdatePicker(state) => Some(state),
            _ => None,
        })
        .last()
        .expect("expected picker update");
    assert_eq!(last_picker.connectivity_state, ConnectivityState::Ready);

    // Add child-2 to priorities.
    let js2 = r#"{
  "priorities": ["child-0", "child-1", "child-2"],
  "children": {
    "child-0": {"config": [{"stub_hr_0": {}}]},
    "child-1": {"config": [{"stub_hr_1": {}}]},
    "child-2": {"config": [{"stub_hr_2": {}}]}
  }
}"#;
    let cfg2 = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js2).unwrap())
        .unwrap()
        .unwrap();
    let update2 = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
            new_test_endpoint("child-2", "127.0.0.1:8002"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update2, Some(&cfg2), &mut env.tcc)
        .unwrap();

    // child-0 is still Ready; child-1 and child-2 should still not be
    // created.
    assert!(stub_handle1.subchannel().is_none());
    assert!(stub_handle2.subchannel().is_none());
}

/// Verifies priority failover when the primary child enters
/// TRANSIENT_FAILURE, and failback with deactivation when it recovers.
#[tokio::test]
async fn switch_priority_failover_and_failback() {
    let stub_handle0 = new_stub("stub_sp_0");
    let stub_handle1 = new_stub("stub_sp_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_sp_0": {}}]},
    "child-1": {"config": [{"stub_sp_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let sc0 = stub_handle0
        .subchannel()
        .expect("child-0 subchannel created");
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::ready(), &mut env.tcc);

    // Turn down child-0 with TransientFailure.
    env.policy.subchannel_update(
        sc0.clone(),
        &SubchannelState::transient_failure("connection refused"),
        &mut env.tcc,
    );

    // Failover: child-1 is lazily created and starts connecting.
    let sc1 = stub_handle1
        .subchannel()
        .expect("child-1 should be created on failover");

    // Make child-1 Ready.
    env.policy
        .subchannel_update(sc1.clone(), &SubchannelState::ready(), &mut env.tcc);

    let last_picker = env
        .rx_events
        .try_iter()
        .filter_map(|e| match e {
            TestEvent::UpdatePicker(s) => Some(s),
            _ => None,
        })
        .last()
        .unwrap();
    assert_eq!(last_picker.connectivity_state, ConnectivityState::Ready);

    // Failback: child-0 recovers to Ready.
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::ready(), &mut env.tcc);

    // child-0 is selected again.
    // child-1 is deactivated with a 15-minute timer.
    assert!(matches!(
        env.policy.child_data.get("child-1").unwrap().state,
        ChildState::Deactivated(_, _)
    ));
}

/// Verifies that when a primary child remains in CONNECTING, the 10-second
/// failover timer expires and triggers failover to the next priority.
#[tokio::test(start_paused = true)]
async fn init_timeout_failover() {
    let stub_handle0 = new_stub("stub_to_0");
    let stub_handle1 = new_stub("stub_to_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_to_0": {}}]},
    "child-1": {"config": [{"stub_to_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    // child-0 is connecting.
    assert!(stub_handle0.subchannel().is_some());
    assert!(
        stub_handle1.subchannel().is_none(),
        "child-1 should not be initialized before timeout"
    );

    // Advance time by 5 seconds (less than 10s timeout).
    advance_time(Duration::from_secs(5)).await;

    assert!(
        matches!(
            env.policy.child_data.get("child-0").unwrap().state,
            ChildState::Connecting(_, _)
        ),
        "child-0 should still be in Connecting state after 5 seconds"
    );
    assert!(stub_handle1.subchannel().is_none());

    // Advance time by another 6 seconds (total 11s > 10s timeout).
    advance_time(Duration::from_secs(6)).await;

    let data = recv_schedule_work(&env.rx_events);
    env.policy.work(data, &mut env.tcc);

    // child-0 should now be ConnectingExpired.
    assert!(matches!(
        env.policy.child_data.get("child-0").unwrap().state,
        ChildState::ConnectingExpired(_)
    ));

    // child-1 should have been lazily initialized.
    assert!(
        stub_handle1.subchannel().is_some(),
        "child-1 should be created after timeout"
    );
}

/// Verifies that receiving multiple CONNECTING updates does not reset the
/// 10-second failover timer.
#[tokio::test(start_paused = true)]
async fn connecting_to_connecting_does_not_restart_timer() {
    let stub_handle0 = new_stub("stub_c2c_0");
    let _stub_handle1 = new_stub("stub_c2c_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_c2c_0": {}}]},
    "child-1": {"config": [{"stub_c2c_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();
    let sc0 = stub_handle0.subchannel().unwrap();

    // Advance time by 5 seconds.
    advance_time(Duration::from_secs(5)).await;

    // Send another Connecting update for child-0.
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::connecting(), &mut env.tcc);

    // Advance time by 6 seconds (total 11s from start, but only 6s from
    // 2nd update).
    advance_time(Duration::from_secs(6)).await;

    let data = recv_schedule_work(&env.rx_events);
    env.policy.work(data, &mut env.tcc);

    // The timer should have expired based on original start time
    // (11s > 10s).
    assert!(matches!(
        env.policy.child_data.get("child-0").unwrap().state,
        ChildState::ConnectingExpired(_)
    ));
}

/// Verifies that a child transitioning from TRANSIENT_FAILURE to
/// CONNECTING enters ConnectingExpired without a new 10-second failover
/// timer.
#[tokio::test]
async fn transient_failure_to_connecting_enters_connecting_expired() {
    let stub_handle0 = new_stub("stub_tf2c_0");
    let stub_handle1 = new_stub("stub_tf2c_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_tf2c_0": {}}]},
    "child-1": {"config": [{"stub_tf2c_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let sc0 = stub_handle0.subchannel().unwrap();

    // child-0 goes to TransientFailure.
    env.policy.subchannel_update(
        sc0.clone(),
        &SubchannelState::transient_failure("fail"),
        &mut env.tcc,
    );

    // child-1 is initialized and goes Ready.
    let sc1 = stub_handle1.subchannel().unwrap();
    env.policy
        .subchannel_update(sc1.clone(), &SubchannelState::ready(), &mut env.tcc);

    // Now child-0 attempts to connect again (TransientFailure -> Connecting).
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::connecting(), &mut env.tcc);

    // Per gRFC A56, child-0 enters ConnectingExpired (no new 10s timer).
    assert!(matches!(
        env.policy.child_data.get("child-0").unwrap().state,
        ChildState::ConnectingExpired(_)
    ));

    // And child-1 (which is Ready) remains the chosen active child!
    let last_picker = env
        .rx_events
        .try_iter()
        .filter_map(|e| match e {
            TestEvent::UpdatePicker(s) => Some(s),
            _ => None,
        })
        .last()
        .unwrap();
    assert_eq!(last_picker.connectivity_state, ConnectivityState::Ready);
}

/// Verifies the 15-minute deactivation retention timer, background update
/// retention, child pruning upon timer expiration, and subsequent clean
/// reactivation without panic.
#[tokio::test(start_paused = true)]
async fn deactivation_and_reactivation() {
    let stub_handle0 = new_stub("stub_dr_0");
    let stub_handle1 = new_stub("stub_dr_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_dr_0": {}}]},
    "child-1": {"config": [{"stub_dr_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let sc0 = stub_handle0.subchannel().unwrap();

    // child-0 fails -> failover to child-1.
    env.policy.subchannel_update(
        sc0.clone(),
        &SubchannelState::transient_failure("fail"),
        &mut env.tcc,
    );
    let sc1 = stub_handle1.subchannel().unwrap();
    env.policy
        .subchannel_update(sc1.clone(), &SubchannelState::ready(), &mut env.tcc);

    // child-0 recovers -> failback to child-0.
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::ready(), &mut env.tcc);

    // child-1 is deactivated with a 15-minute timer.
    assert!(matches!(
        env.policy.child_data.get("child-1").unwrap().state,
        ChildState::Deactivated(_, _)
    ));

    // While deactivated, background updates to child-1 do NOT cancel
    // deactivation.
    env.policy
        .subchannel_update(sc1.clone(), &SubchannelState::connecting(), &mut env.tcc);
    assert!(matches!(
        env.policy.child_data.get("child-1").unwrap().state,
        ChildState::Deactivated(_, _)
    ));
    // Advance time past 15 minutes (901 seconds).
    advance_time(Duration::from_secs(15 * 60 + 1)).await;

    let data = recv_schedule_work(&env.rx_events);
    env.policy.work(data, &mut env.tcc);

    // child-1 should now be Uninitialized in child_data, and pruned from
    // child_mgr.
    assert!(matches!(
        env.policy.child_data.get("child-1").unwrap().state,
        ChildState::Uninitialized
    ));
    assert_eq!(env.policy.child_mgr.children().count(), 1);

    // Now child-0 fails again. child-1 should be reactivated.
    stub_handle1.set_subchannel(None);
    env.policy.subchannel_update(
        sc0.clone(),
        &SubchannelState::transient_failure("fail again"),
        &mut env.tcc,
    );

    assert!(
        stub_handle1.subchannel().is_some(),
        "child-1 should be re-created from Uninitialized"
    );
}

/// Verifies that ignoreReresolutionRequests config correctly filters
/// re-resolution requests from child policies.
#[tokio::test]
async fn ignore_reresolution_requests_configuration() {
    let stub_handle0 = new_stub("stub_irr_0");
    let stub_handle1 = new_stub("stub_irr_1");

    let mut env = setup_test_env();

    // child-0 has ignoreReresolutionRequests: true
    // child-1 has ignoreReresolutionRequests: false
    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_irr_0": {}}], "ignoreReresolutionRequests": true},
    "child-1": {"config": [{"stub_irr_1": {}}], "ignoreReresolutionRequests": false}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let sc0 = stub_handle0.subchannel().unwrap();

    // child-0 enters TransientFailure (our stub calls
    // request_resolution()).
    env.policy.subchannel_update(
        sc0.clone(),
        &SubchannelState::transient_failure("fail"),
        &mut env.tcc,
    );
    assert!(stub_handle0.resolution_requested());

    // Since child-0 has ignoreReresolutionRequests = true, tcc did NOT
    // receive RequestResolution.
    let had_resolution_req = env
        .rx_events
        .try_iter()
        .any(|e| matches!(e, TestEvent::RequestResolution));
    assert!(
        !had_resolution_req,
        "re-resolution request from child-0 should be ignored"
    );

    // Failover occurred to child-1.
    let sc1 = stub_handle1.subchannel().unwrap();

    // child-1 enters TransientFailure (our stub calls
    // request_resolution()).
    env.policy.subchannel_update(
        sc1.clone(),
        &SubchannelState::transient_failure("fail"),
        &mut env.tcc,
    );
    assert!(stub_handle1.resolution_requested());

    // Since child-1 has ignoreReresolutionRequests = false, tcc DOES
    // receive RequestResolution!
    let had_resolution_req2 = env
        .rx_events
        .try_iter()
        .any(|e| matches!(e, TestEvent::RequestResolution));
    assert!(
        had_resolution_req2,
        "re-resolution request from child-1 should be forwarded"
    );
}

/// Verifies that removing a child from the configuration immediately deletes
/// it from child_data and child_mgr.
#[tokio::test]
async fn remove_child_from_config_deletes_immediately() {
    let _stub_handle0 = new_stub("stub_rc_0");
    let _stub_handle1 = new_stub("stub_rc_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_rc_0": {}}]},
    "child-1": {"config": [{"stub_rc_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    assert!(env.policy.child_data.contains_key("child-0"));
    assert!(env.policy.child_data.contains_key("child-1"));

    // Remove child-1 from configuration.
    let js2 = r#"{
  "priorities": ["child-0"],
  "children": {
    "child-0": {"config": [{"stub_rc_0": {}}]}
  }
}"#;
    let cfg2 = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js2).unwrap())
        .unwrap()
        .unwrap();
    let update2 = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![new_test_endpoint("child-0", "127.0.0.1:8000")]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update2, Some(&cfg2), &mut env.tcc)
        .unwrap();

    // child-1 must be removed immediately (gRFC A115).
    assert!(env.policy.child_data.contains_key("child-0"));
    assert!(!env.policy.child_data.contains_key("child-1"));
    assert_eq!(env.policy.priorities, vec!["child-0"]);
}

/// Verifies that PriorityPolicy::work drops its own PriorityTimerWork (without
/// forwarding to ChildManager) and correctly forwards child balancer work items
/// to ChildManager.
#[tokio::test]
async fn work_item_filtering_drops_timer_work_and_forwards_child_work() {
    let child_work_called = Arc::new(Mutex::new(false));
    let cwc_clone = child_work_called.clone();

    let funcs = StubPolicyFuncs {
        resolver_update: Some(Arc::new(move |data, update, _cfg, controller| {
            let addr = update
                .endpoints
                .as_ref()
                .ok()
                .and_then(|e| e.first())
                .and_then(|e| e.addresses.first())
                .cloned()
                .unwrap_or_default();
            controller.new_subchannel(&addr);
            // Schedule work from the child policy!
            data.lb_policy_options.work_scheduler.schedule_work(None);
            Ok(())
        })),
        work: Some(Arc::new(move |_data, _work_data, _controller| {
            *cwc_clone.lock().unwrap() = true;
        })),
        ..Default::default()
    };
    test_utils::reg_stub_policy("stub_filter_work", funcs);

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0"],
  "children": {
    "child-0": {"config": [{"stub_filter_work": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![new_test_endpoint("child-0", "127.0.0.1:8000")]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    // Deliver a PriorityTimerWork item: it should be consumed by PriorityPolicy
    // and NOT forwarded to child_mgr.
    let timer_work: WorkData = Box::new(PriorityTimerWork);
    env.policy.work(Some(timer_work), &mut env.tcc);
    assert!(
        !*child_work_called.lock().unwrap(),
        "PriorityTimerWork should be dropped and not forwarded to child policy"
    );

    // Deliver the work item that the child scheduled in its resolver_update:
    // It should be forwarded to ChildManager and invoke the child's work fn.
    let child_work = recv_schedule_work(&env.rx_events);
    env.policy.work(child_work, &mut env.tcc);
    assert!(
        *child_work_called.lock().unwrap(),
        "Child policy work item should be forwarded to child policy"
    );
}

/// Verifies that updates to inactive/background children do not cause redundant
/// `UpdatePicker` events to be published to the channel controller if the
/// active child's picker remains unchanged.
#[tokio::test]
async fn picker_updates_are_debounced_for_inactive_child_events() {
    let stub_handle0 = new_stub("stub_debounce_0");
    let _stub_handle1 = new_stub("stub_debounce_1");

    let mut env = setup_test_env();

    let js = r#"{
  "priorities": ["child-0", "child-1"],
  "children": {
    "child-0": {"config": [{"stub_debounce_0": {}}]},
    "child-1": {"config": [{"stub_debounce_1": {}}]}
  }
}"#;
    let cfg = env
        .builder
        .parse_config(&ParsedJsonLbConfig::new(js).unwrap())
        .unwrap()
        .unwrap();
    let update = ResolverUpdate {
        attributes: Default::default(),
        endpoints: Ok(vec![
            new_test_endpoint("child-0", "127.0.0.1:8000"),
            new_test_endpoint("child-1", "127.0.0.1:8001"),
        ]),
        service_config: Ok(None),
        resolution_note: None,
    };
    env.policy
        .resolver_update(update, Some(&cfg), &mut env.tcc)
        .unwrap();

    let sc0 = stub_handle0.subchannel().unwrap();

    // Transition child-0 to Ready.
    env.policy
        .subchannel_update(sc0.clone(), &SubchannelState::ready(), &mut env.tcc);

    // Drain events; verify child-0 published Ready.
    let mut saw_ready = false;
    while let Ok(event) = env.rx_events.try_recv() {
        if let TestEvent::UpdatePicker(state) = event
            && state.connectivity_state == ConnectivityState::Ready
        {
            saw_ready = true;
        }
    }
    assert!(saw_ready);

    // An event that reconciles without altering the active child's picker
    // (such as exit_idle) does not re-emit an UpdatePicker event.
    env.policy.exit_idle(&mut env.tcc);

    // Verify NO new UpdatePicker was emitted because it was debounced.
    let mut unexpected_picker_update = false;
    while let Ok(event) = env.rx_events.try_recv() {
        if let TestEvent::UpdatePicker(_) = event {
            unexpected_picker_update = true;
        }
    }
    assert!(
        !unexpected_picker_update,
        "identical picker update should be debounced"
    );
}
