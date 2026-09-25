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

//! Priority Load Balancing Policy (`priority_experimental`).
//!
//! This module implements the `priority_experimental` load balancing policy as
//! specified in [gRFC A56: `priority_experimental` LB policy] and updated by
//! [gRFC A115: disable Priority LB policy child policy retention cache].
//!
//! # Overview
//!
//! The priority LB policy manages an ordered list of child policies. It routes
//! RPCs to the highest-priority child that is reachable (in `READY` or `IDLE`
//! state). If higher-priority children are unavailable, fail, or take too long
//! to connect, the policy fails over to lower-priority children.
//!
//! Each endpoint in a [`ResolverUpdate`] delivered to the priority LB must be
//! annotated with a hierarchical path attribute
//! (see [gRFC A56: Hierarchical Addresses]), otherwise, it will be ignored.
//!
//! [gRFC A56: `priority_experimental` LB policy]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md
//! [gRFC A56: Hierarchical Addresses]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#hierarchical-addresses
//! [gRFC A115: disable Priority LB policy child policy retention cache]:
//!   https://github.com/grpc/proposal/blob/master/A115-remove-priority-lb-child-policy-cache.md

use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::time::Duration;

use tokio::time::Instant;

use crate::client::ConnectivityState;
use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::FailingPicker;
use crate::client::load_balancing::GLOBAL_LB_REGISTRY;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::ParsedJsonLbConfig;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::WorkScheduler;
use crate::client::load_balancing::child_manager::ChildManager;
use crate::client::load_balancing::child_manager::ChildUpdate;
use crate::client::load_balancing::endpoint_filtering;
use crate::client::load_balancing::priority::child::ChildBuilder;
use crate::client::load_balancing::priority::child::PriorityChildConfig;
use crate::client::name_resolution::ResolverUpdate;
use crate::rt::BoxedTaskHandle;
use crate::rt::GrpcRuntime;

mod child;

/// The name under which the builder is registered in the global LB registry.
pub static POLICY_NAME: &str = "priority_experimental";

/// Failover timeout for a child attempting to connect (10 seconds).
const CONNECTING_TIMEOUT: Duration = Duration::from_secs(10);

/// Retention timeout for deactivated lower-priority children (15 minutes).
const DEACTIVATION_TIMEOUT: Duration = Duration::from_secs(15 * 60);

/// Registers the `priority_experimental` LB policy builder in the global LB
/// registry.
pub fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(Builder {});
}

/// Parsed configuration for the `priority_experimental` load balancing policy.
///
/// Corresponds to `PriorityLoadBalancingPolicyConfig` defined in
/// [gRFC A56 (Section LB Policy Configuration)]:
///
/// ```proto
/// message PriorityLoadBalancingPolicyConfig {
///   map<string, Child> children = 1;
///   repeated string priorities = 2;
/// }
/// ```
///
/// [gRFC A56 (Section LB Policy Configuration)]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#lb-policy-configuration
#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PriorityConfig {
    /// Ordered list of child balancer names in decreasing priority order
    /// (index 0 is highest priority).
    priorities: Vec<String>,

    /// Map from child balancer names to their configurations.
    ///
    /// Names correspond to entries in [`priorities`]. Decoupling names from
    /// priority positions allows existing children to be moved between
    /// priorities without recreating the child policy and its subchannels.
    children: HashMap<String, PriorityChildConfig>,
}

impl PriorityConfig {
    fn validate(&self) -> Result<(), String> {
        for name in &self.priorities {
            if !self.children.contains_key(name) {
                return Err(format!(
                    "LB policy name \"{name}\" found in Priorities field ({:?}) is not found in Children field ({:?})",
                    self.priorities, self.children
                ));
            }
        }
        for name in self.children.keys() {
            if !self.priorities.contains(name) {
                return Err(format!(
                    "LB policy name \"{name}\" found in Children field ({:?}) is not found in Priorities field ({:?})",
                    self.children, self.priorities
                ));
            }
        }
        Ok(())
    }

    /// Returns the configured children ordered from the highest priority to
    /// the lowest.
    fn ordered_children(&self) -> impl Iterator<Item = (&String, &PriorityChildConfig)> {
        self.priorities
            .iter()
            .filter_map(|name| self.children.get(name).map(|config| (name, config)))
    }
}

/// Internal tracking data and state for a configured priority child.
#[derive(Debug)]
struct ChildData {
    /// The name identifying this child in the LB config and in
    /// [`ChildManager`].
    name: String,
    /// The current lifecycle and connectivity state of the child policy.
    state: ChildState,
    /// The current LB configuration for this child.
    child_config: PriorityChildConfig,
    /// The latest name resolver update received for this child.
    latest_update: ResolverUpdate,
}

/// Internal work item scheduled by [`PriorityPolicy`] timers upon expiration.
#[derive(Debug)]
struct PriorityTimerWork;

/// An RAII timer that triggers a work notification upon expiration.
///
/// When instantiated via [`Timer::new`], a background task is spawned on the
/// runtime that sleeps for the specified duration and then invokes
/// [`WorkScheduler::schedule_work`] with [`PriorityTimerWork`].
///
/// If dropped before expiration (e.g., when a child transitions out of
/// `Connecting` or is reactivated from `Deactivated`), the spawned task is
/// aborted via its task handle, preventing stale timer wakeups.
struct Timer {
    /// Absolute instant when the timer expires.
    deadline: Instant,
    /// Task handle for the background sleep task, aborted upon drop.
    task_handle: BoxedTaskHandle,
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("deadline", &self.deadline)
            .finish()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.task_handle.abort();
    }
}

impl Timer {
    /// Spawns a new timer for the given duration that schedules work on
    /// completion.
    fn new(duration: Duration, work_scheduler: Arc<dyn WorkScheduler>, rt: GrpcRuntime) -> Timer {
        let rt_clone = rt.clone();
        let task_handle = rt.spawn(Box::pin(async move {
            rt_clone.sleep(duration).await;
            work_scheduler.schedule_work(Some(Box::new(PriorityTimerWork)));
        }));
        Timer {
            deadline: Instant::now() + duration,
            task_handle,
        }
    }
}

/// Lifecycle and connectivity states of a child policy under
/// `priority_experimental`.
///
/// [gRFC A56 (Section Child Lifetime Management)]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-lifetime-management
/// [gRFC A56 (Section Child Connectivity State Tracking)]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-connectivity-state-tracking
#[derive(Debug)]
enum ChildState {
    /// The child is configured but has not yet been instantiated in
    /// [`ChildManager`].
    ///
    /// It will be lazily created and initialized with
    /// [`ChildData::latest_update`] when evaluated during priority selection
    /// (see [gRFC A56 (Section Child Lifetime Management)]).
    Uninitialized,

    /// The child is actively attempting to connect, with a 10-second failover
    /// timer running.
    ///
    /// While this timer is active, priority selection will wait for this child
    /// before evaluating lower priorities (see
    /// [gRFC A56 (Section Child Connectivity State Tracking)]).
    Connecting(Timer, LbState),

    /// The child is in `CONNECTING` state, but its 10-second failover timer has
    /// expired.
    ///
    /// The child continues attempting connection in the background, but
    /// priority selection may now proceed to check lower priorities (see
    /// [gRFC A56 (Section Child Connectivity State Tracking)]).
    ConnectingExpired(LbState),

    /// The child reported `TRANSIENT_FAILURE`.
    ///
    /// The failover timer is cancelled, and priority selection may proceed to
    /// check lower priorities.
    TransientFailure(LbState),

    /// The child is in `READY` or `IDLE` state.
    ///
    /// When a child reaches this state, it is selected as the active priority
    /// and lower-priority children are deactivated (with a 15-minute timer).
    ReadyOrIdle(LbState),

    /// The child was previously active, but has been superseded by a
    /// higher-priority child reaching `ReadyOrIdle` state.
    ///
    /// A 15-minute deactivation timer is running ([`DEACTIVATION_TIMEOUT`]).
    /// If higher priorities fail before this timer expires, the child will be
    /// reactivated immediately without connection churn. If the timer expires,
    /// the child is destroyed (see
    /// [gRFC A56 (Section Child Lifetime Management)]).
    ///
    /// Per [gRFC A115], only children still configured in `PriorityConfig`
    /// enter this state; unconfigured children are removed immediately.
    ///
    /// [gRFC A115]:
    ///   https://github.com/grpc/proposal/blob/master/A115-remove-priority-lb-child-policy-cache.md
    Deactivated(Timer, LbState),
}

impl ChildState {
    /// Returns a reference to the child's current [`LbState`], or `None` if
    /// the child is [`ChildState::Uninitialized`].
    fn lb_state(&self) -> Option<&LbState> {
        match self {
            ChildState::Uninitialized => None,
            ChildState::Connecting(_, lb_state)
            | ChildState::ConnectingExpired(lb_state)
            | ChildState::TransientFailure(lb_state)
            | ChildState::ReadyOrIdle(lb_state)
            | ChildState::Deactivated(_, lb_state) => Some(lb_state),
        }
    }
}

/// LB policy builder for the `priority_experimental` load balancing policy.
#[derive(Debug)]
struct Builder {}

impl LbPolicyBuilder for Builder {
    type LbPolicy = PriorityPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        let rt = options.runtime;
        PriorityPolicy {
            child_mgr: ChildManager::new(rt.clone(), options.work_scheduler.clone()),
            children: Vec::default(),
            published_lb_state: None,
            rt,
            work_scheduler: options.work_scheduler,
        }
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &ParsedJsonLbConfig) -> Result<PriorityConfig, String> {
        let cfg: PriorityConfig = config.convert_to().map_err(|e| e.to_string())?;
        cfg.validate()?;
        Ok(cfg)
    }
}

/// The `priority_experimental` load balancing policy instance.
///
/// Manages a prioritized collection of child policies.
#[derive(Debug)]
struct PriorityPolicy {
    child_mgr: ChildManager<String, ChildBuilder>,
    /// Current priority hierarchy: the configured children, ordered from the
    /// highest priority (index 0) to the lowest.
    children: Vec<ChildData>,
    /// The most recent LB state published to the channel controller.
    ///
    /// Used to debounce redundant picker updates when internal priority or
    /// child events do not change the active picker.
    published_lb_state: Option<LbState>,
    rt: GrpcRuntime,
    work_scheduler: Arc<dyn WorkScheduler>,
}

impl LbPolicy for PriorityPolicy {
    type LbConfig = PriorityConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let mut sharded_endpoints = update.endpoints.map(endpoint_filtering::group_by_path);

        // Index the existing children by name so they can be moved into the
        // new priority order.  Children that are no longer configured are left
        // behind in this map and dropped, see gRFC A115.
        let mut old_children: HashMap<String, ChildData> = mem::take(&mut self.children)
            .into_iter()
            .map(|child_data| (child_data.name.clone(), child_data))
            .collect();

        for (name, child_cfg) in config.ordered_children() {
            let endpoints = match &mut sharded_endpoints {
                Ok(grouped) => Ok(grouped.remove(name).unwrap_or_default()),
                Err(status) => Err(status.clone()),
            };

            let resolver_update = ResolverUpdate {
                attributes: update.attributes.clone(),
                endpoints,
                service_config: update.service_config.clone(),
                resolution_note: update.resolution_note.clone(),
            };

            let child_data = match old_children.remove(name) {
                Some(mut child_data) => {
                    child_data.child_config = child_cfg.clone();
                    child_data.latest_update = resolver_update;
                    child_data
                }
                None => ChildData {
                    name: name.clone(),
                    state: ChildState::Uninitialized,
                    child_config: child_cfg.clone(),
                    latest_update: resolver_update,
                },
            };
            self.children.push(child_data);
        }

        // Only children that have already been created are sent to the
        // ChildManager; the remaining ones are created lazily by
        // choose_priority.
        let child_updates = self
            .children
            .iter()
            .filter(|child_data| !matches!(child_data.state, ChildState::Uninitialized))
            .map(|child_data| ChildUpdate {
                child_identifier: child_data.name.clone(),
                child_policy_builder: ChildBuilder {},
                child_update: Some((child_data.latest_update.clone(), &child_data.child_config)),
            });

        // Update children in ChildManager. As specified in gRFC A56
        // (Section Configuration Updates), priority re-evaluation is deferred
        // until all child updates have been applied.
        let res = self.child_mgr.update(child_updates, channel_controller);
        self.reconcile(channel_controller);
        res
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        // Drop work items scheduled by PriorityPolicy's own timers so they are
        // not forwarded to ChildManager; forward all other work to children.
        let is_priority_work = data
            .as_ref()
            .is_some_and(|d| d.downcast_ref::<PriorityTimerWork>().is_some());
        if !is_priority_work {
            self.child_mgr.work(data, channel_controller);
        }
        self.reconcile(channel_controller);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        // TODO: Similar to C++, only call exit_idle on the currently selected
        // child once the child_manager supports it.
        self.child_mgr.exit_idle(channel_controller);
        self.reconcile(channel_controller);
    }
}

impl PriorityPolicy {
    /// Reconciles timers, synchronizes child connectivity states, and
    /// re-evaluates priority selection.
    ///
    /// This is the central event handler invoked after resolver updates,
    /// subchannel state changes, or timer expirations. It executes four
    /// sequential steps:
    /// 1. [`handle_deactivation_timer`]: Prunes deactivated children whose
    ///    15-minute timer has expired.
    /// 2. [`handle_connectivity_timer`]: Transitions children whose 10-second
    ///    failover timer has expired to [`ChildState::ConnectingExpired`].
    /// 3. [`update_child_data`]: Synchronizes local [`ChildState`] tracking
    ///    with states reported by [`ChildManager`].
    /// 4. [`choose_priority`]: Executes the idempotent priority selection
    ///    algorithm ([gRFC A56]).
    ///
    /// [gRFC A56]:
    ///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md
    fn reconcile(&mut self, channel_controller: &mut dyn ChannelController) {
        self.handle_deactivation_timer();
        self.handle_connectivity_timer();
        self.update_child_data();
        self.choose_priority(channel_controller);
    }

    /// Synchronizes the local [`ChildState`] of each child with the latest
    /// state reported by [`ChildManager`].
    ///
    /// Children that have not been created in [`ChildManager`] yet, i.e. those
    /// in [`ChildState::Uninitialized`], are left untouched.
    fn update_child_data(&mut self) {
        let latest_states: HashMap<&str, &LbState> = self
            .child_mgr
            .children()
            .map(|child| (child.identifier.as_str(), &child.state))
            .collect();

        for child_data in &mut self.children {
            let Some(lb_state) = latest_states.get(child_data.name.as_str()) else {
                continue;
            };
            // Take ownership of the current state and replace it with a
            // temporary Uninitialized value.
            let old_state = mem::replace(&mut child_data.state, ChildState::Uninitialized);
            let lb_state = (*lb_state).clone();
            child_data.state = match old_state {
                ChildState::Deactivated(timer, _) => {
                    // While deactivated, retain the 15-minute deactivation
                    // timer and record the updated LbState (see gRFC A56
                    // (Section Child Lifetime Management)).
                    ChildState::Deactivated(timer, lb_state)
                }
                ChildState::Uninitialized => {
                    unreachable!("child tracked by ChildManager cannot be in Uninitialized state")
                }
                ChildState::Connecting(timer, _) => match lb_state.connectivity_state {
                    ConnectivityState::Idle | ConnectivityState::Ready => {
                        ChildState::ReadyOrIdle(lb_state)
                    }
                    ConnectivityState::Connecting => ChildState::Connecting(timer, lb_state),
                    ConnectivityState::TransientFailure => ChildState::TransientFailure(lb_state),
                },
                ChildState::ConnectingExpired(_) | ChildState::TransientFailure(_) => {
                    match lb_state.connectivity_state {
                        ConnectivityState::Idle | ConnectivityState::Ready => {
                            ChildState::ReadyOrIdle(lb_state)
                        }
                        ConnectivityState::Connecting => ChildState::ConnectingExpired(lb_state),
                        ConnectivityState::TransientFailure => {
                            ChildState::TransientFailure(lb_state)
                        }
                    }
                }
                ChildState::ReadyOrIdle(_) => match lb_state.connectivity_state {
                    ConnectivityState::Idle | ConnectivityState::Ready => {
                        ChildState::ReadyOrIdle(lb_state)
                    }
                    ConnectivityState::Connecting => ChildState::Connecting(
                        Timer::new(
                            CONNECTING_TIMEOUT,
                            self.work_scheduler.clone(),
                            self.rt.clone(),
                        ),
                        lb_state,
                    ),
                    ConnectivityState::TransientFailure => ChildState::TransientFailure(lb_state),
                },
            };
        }
    }

    /// Evaluates the priority hierarchy and selects the active child policy to
    /// route traffic.
    ///
    /// Implements the idempotent selection algorithm defined in
    /// [gRFC A56 (Section Algorithm for Choosing a Priority)]:
    ///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#algorithm-for-choosing-a-priority
    fn choose_priority(&mut self, channel_controller: &mut dyn ChannelController) {
        // If priority list is empty, report TRANSIENT_FAILURE with
        // FailingPicker.
        if self.children.is_empty() {
            self.update_picker(
                channel_controller,
                LbState {
                    connectivity_state: ConnectivityState::TransientFailure,
                    picker: Arc::new(FailingPicker {
                        error: "priority policy has empty priority list".to_owned(),
                    }),
                },
            );
            return;
        }

        // Iterate through priorities in decreasing priority order (0..N-1),
        // searching for a child in READY/IDLE or whose 10s failover timer is
        // still pending.
        for idx in 0..self.children.len() {
            let child_data = &mut self.children[idx];

            // Reactivate child if previously deactivated.
            if let ChildState::Deactivated(_, lb_state) = &child_data.state {
                let new_state = match lb_state.connectivity_state {
                    ConnectivityState::Idle | ConnectivityState::Ready => {
                        ChildState::ReadyOrIdle(lb_state.clone())
                    }
                    ConnectivityState::Connecting => ChildState::Connecting(
                        Timer::new(
                            CONNECTING_TIMEOUT,
                            self.work_scheduler.clone(),
                            self.rt.clone(),
                        ),
                        lb_state.clone(),
                    ),
                    ConnectivityState::TransientFailure => {
                        ChildState::TransientFailure(lb_state.clone())
                    }
                };
                child_data.state = new_state;
            }

            // Lazily create and initialize the child if uninitialized.
            if matches!(child_data.state, ChildState::Uninitialized) {
                let (child_id, latest_update) = {
                    child_data.state = ChildState::Connecting(
                        Timer::new(
                            CONNECTING_TIMEOUT,
                            self.work_scheduler.clone(),
                            self.rt.clone(),
                        ),
                        LbState::initial(),
                    );
                    (child_data.name.clone(), child_data.latest_update.clone())
                };
                if self
                    .update_child(child_id, latest_update, channel_controller)
                    .is_err()
                {
                    channel_controller.request_resolution();
                }
                self.update_child_data();
            }

            match &self.children[idx].state {
                ChildState::Uninitialized => {
                    unreachable!("uninitialized child was initialized prior to state evaluation")
                }
                // Child is Connecting and failover timer is pending: use this
                // child, without deactivating lower priorities.
                ChildState::Connecting(_, _) => {
                    self.set_current_priority(channel_controller, idx, false);
                    return;
                }
                // Child failover timer expired or in transient failure: skip to
                // lower priorities.
                ChildState::ConnectingExpired(_) | ChildState::TransientFailure(_) => {}
                // Child is Ready or Idle: use this child and deactivate lower
                // priorities.
                ChildState::ReadyOrIdle(_) => {
                    self.set_current_priority(channel_controller, idx, true);
                    return;
                }
                ChildState::Deactivated(_, _) => {
                    unreachable!("child was reactivated and cannot remain in Deactivated state")
                }
            }
        }

        // We did not find a priority in READY or IDLE or whose failover timer
        // was pending, so check for one in CONNECTING (whose failover timer has
        // expired).
        for idx in 0..self.children.len() {
            if matches!(self.children[idx].state, ChildState::ConnectingExpired(_)) {
                self.set_current_priority(channel_controller, idx, false);
                return;
            }
        }

        // We didn't find a child in CONNECTING, so delegate to the last child
        // (reporting its TRANSIENT_FAILURE state and failing picker).
        self.set_current_priority(channel_controller, self.children.len() - 1, false);
    }

    /// Activates the selected priority tier and updates the channel picker.
    fn set_current_priority(
        &mut self,
        channel_controller: &mut dyn ChannelController,
        index: usize,
        deactivate_lower_priorities: bool,
    ) {
        // Deactivate lower priorities if needed.
        if deactivate_lower_priorities {
            for child_data in self.children.iter_mut().skip(index + 1) {
                let old_state = mem::replace(&mut child_data.state, ChildState::Uninitialized);
                child_data.state = match old_state {
                    ChildState::Uninitialized => ChildState::Uninitialized,
                    ChildState::Connecting(_, lb_state)
                    | ChildState::ConnectingExpired(lb_state)
                    | ChildState::TransientFailure(lb_state)
                    | ChildState::ReadyOrIdle(lb_state) => ChildState::Deactivated(
                        Timer::new(
                            DEACTIVATION_TIMEOUT,
                            self.work_scheduler.clone(),
                            self.rt.clone(),
                        ),
                        lb_state,
                    ),
                    ChildState::Deactivated(timer, lb_state) => {
                        ChildState::Deactivated(timer, lb_state)
                    }
                };
            }
        }

        // Use this child's picker.
        let lb_state = self.children[index]
            .state
            .lb_state()
            .expect("cannot set priority to uninitialized child; child must be initialized first")
            .clone();
        self.update_picker(channel_controller, lb_state);
    }

    /// Updates the channel controller with the new [`LbState`] if it differs
    /// from the currently published state, recording it in
    /// `self.published_lb_state` for deduplication.
    fn update_picker(&mut self, channel_controller: &mut dyn ChannelController, lb_state: LbState) {
        if self.published_lb_state.as_ref() == Some(&lb_state) {
            return;
        }
        self.published_lb_state = Some(lb_state.clone());
        channel_controller.update_picker(lb_state);
    }

    /// Lazily activates a previously uninitialized child by submitting its
    /// initial [`ResolverUpdate`] and configuration to [`ChildManager`].
    fn update_child(
        &mut self,
        child_id: String,
        resolver_update: ResolverUpdate,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let mut resolver_update = Some(resolver_update);
        let child_updates = self
            .children
            .iter()
            .filter(|child_data| !matches!(child_data.state, ChildState::Uninitialized))
            .map(|child_data| {
                let update = if child_id == child_data.name {
                    // .take() moves the owned value out without cloning.
                    // Child names are unique, so there's at most one element
                    // that matches.
                    resolver_update
                        .take()
                        .map(|ru| (ru, &child_data.child_config))
                } else {
                    None
                };
                ChildUpdate {
                    child_identifier: child_data.name.clone(),
                    child_policy_builder: ChildBuilder {},
                    child_update: update,
                }
            });

        self.child_mgr.update(child_updates, channel_controller)
    }

    /// Checks for children in [`ChildState::Connecting`] whose 10-second
    /// failover timer has expired, transitioning them to
    /// [`ChildState::ConnectingExpired`].
    fn handle_connectivity_timer(&mut self) {
        for child_data in &mut self.children {
            if let ChildState::Connecting(connecting_state, lb_state) = &child_data.state
                && Instant::now() >= connecting_state.deadline
            {
                child_data.state = ChildState::ConnectingExpired(lb_state.clone());
            }
        }
    }

    /// Prunes deactivated children whose 15-minute retention timer has expired.
    ///
    /// Expired children revert to [`ChildState::Uninitialized`] and are removed
    /// from [`ChildManager`] to tear down their subchannels. They are NOT
    /// removed from `self.children` so they can be lazily re-created if
    /// higher priorities fail later (see [gRFC A56 (Section Child Lifetime
    /// Management)]).
    fn handle_deactivation_timer(&mut self) {
        let mut any_expired = false;
        for child_data in &mut self.children {
            if let ChildState::Deactivated(timer, _) = &child_data.state
                && Instant::now() >= timer.deadline
            {
                child_data.state = ChildState::Uninitialized;
                any_expired = true;
            }
        }

        if !any_expired {
            return;
        }
        let iter = self
            .children
            .iter()
            .filter(|child_data| !matches!(child_data.state, ChildState::Uninitialized))
            .map(|child_data| (child_data.name.clone(), ChildBuilder {}));

        self.child_mgr.retain_children(iter);
    }
}

#[cfg(test)]
impl PriorityPolicy {
    /// Returns the data tracked for the child with the given name, if it is
    /// currently configured.
    fn child(&self, name: &str) -> Option<&ChildData> {
        self.children
            .iter()
            .find(|child_data| child_data.name == name)
    }
}

#[cfg(test)]
mod test;
