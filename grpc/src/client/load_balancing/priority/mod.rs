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
//! While originally developed to support xDS priority failover across
//! localities or aggregate clusters (see [gRFC A27] and [gRFC A37]), this
//! policy contains no xDS-specific logic and can be used generically in any
//! hierarchical load balancing context.
//!
//! # Hierarchical Addresses
//!
//! Endpoints received in a [`ResolverUpdate`] are delivered as a flat list.
//! When nested load balancers are arranged hierarchically, each endpoint
//! address is annotated with a hierarchical path attribute (see [gRFC A56:
//! Hierarchical Addresses]).
//!
//! [`PriorityPolicy`] groups endpoint addresses by matching the first element
//! in each address's path against the child names, strips that element, and
//! routes the sub-list of addresses down to the corresponding child policy.
//!
//! # Child Lifetime Management & Lazy Creation
//!
//! To avoid unnecessary resource consumption and connection churn:
//! - Lazy creation: Child policies are not instantiated upfront for every
//!   configured priority. Instead, each child is created on-demand only when
//!   the priority selection algorithm needs to attempt using that priority tier
//!   (see [gRFC A56 §Child Lifetime Management]).
//! - Failback & Deactivation: When a higher-priority child becomes `READY` or
//!   `IDLE`, previously active lower-priority children are not immediately
//!   destroyed. Doing so would cause costly connection re-establishment if
//!   priorities flap. Instead, lower-priority children enter
//!   [`ChildState::Deactivated`] and run a 15-minute timer. If the
//!   higher-priority child fails within 15 minutes, the lower-priority child is
//!   reactivated instantly. If the timer expires without reactivation, the
//!   child is cleaned up.
//! - Unconfigured Child Removal: Per [gRFC A115], children omitted from a
//!   configuration update are immediately destroyed and removed from the child
//!   manager, completely bypassing the retention cache.
//!
//! # Connectivity State Tracking & Failover Timer
//!
//! Each child has an associated 10-second failover timer started when the child
//! begins attempting to connect (see [gRFC A56 §Child Connectivity State
//! Tracking]):
//! - While this timer is running, the priority selection algorithm waits on
//!   this child before falling over to lower priorities.
//! - The timer is cancelled if the child reports `READY`, `IDLE`, or
//!   `TRANSIENT_FAILURE`.
//! - If the timer fires and the child is still `CONNECTING`, the algorithm
//!   proceeds to evaluate lower-priority children while the higher-priority
//!   child continues attempting connection in the background.
//!
//! [gRFC A27]:
//!   https://github.com/grpc/proposal/blob/master/A27-xds-global-load-balancing.md
//! [gRFC A37]:
//!   https://github.com/grpc/proposal/blob/master/A37-xds-aggregate-and-logical-dns-clusters.md
//! [gRFC A56: `priority_experimental` LB policy]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md
//! [gRFC A56 §Child Lifetime Management]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-lifetime-management
//! [gRFC A56 §Child Connectivity State Tracking]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-connectivity-state-tracking
//! [gRFC A56 §Algorithm for Choosing a Priority]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#algorithm-for-choosing-a-priority
//! [gRFC A56: Hierarchical Addresses]:
//!   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#hierarchical-addresses
//! [gRFC A115: disable Priority LB policy child policy retention cache]:
//!   https://github.com/grpc/proposal/blob/master/A115-remove-priority-lb-child-policy-cache.md
//! [gRFC A115]:
//!   https://github.com/grpc/proposal/blob/master/A115-remove-priority-lb-child-policy-cache.md

use std::collections::HashMap;
use std::collections::hash_map::Entry;
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
use crate::client::load_balancing::priority::child::ChildConfig;
use crate::client::load_balancing::subchannel::Subchannel;
use crate::client::load_balancing::subchannel::SubchannelState;
use crate::client::name_resolution::ResolverUpdate;
use crate::rt::BoxedTaskHandle;
use crate::rt::GrpcRuntime;

mod child;

/// The name under which the builder is registered in the global LB registry.
pub static POLICY_NAME: &str = "priority_experimental";

/// Failover timeout for a child attempting to connect (10 seconds).
///
/// Per [gRFC A56 §Child Connectivity State Tracking], each child has a
/// 10-second failover timer that starts when it begins attempting to connect.
/// While this timer is active, the priority selection algorithm waits for
/// this child to connect before failing over to lower priorities.
///
/// [gRFC A56 §Child Connectivity State Tracking]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-connectivity-state-tracking
const CONNECTING_TIMEOUT: Duration = Duration::from_secs(10);

/// Retention timeout for deactivated lower-priority children (15 minutes).
///
/// Per [gRFC A56 §Child Lifetime Management], when switching to a
/// higher-priority child, active lower-priority children are deactivated and
/// retained for up to 15 minutes to prevent connection churn if the higher
/// priority flaps.
///
/// Note: Under [gRFC A115], this timeout applies only to failover
/// deactivations (Case 1); children removed from the configuration (Case 2)
/// are dropped immediately.
///
/// [gRFC A56 §Child Lifetime Management]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-lifetime-management
/// [gRFC A115]:
///   https://github.com/grpc/proposal/blob/master/A115-remove-priority-lb-child-policy-cache.md
const DEACTIVATION_TIMEOUT: Duration = Duration::from_secs(15 * 60);

/// Registers the `priority_experimental` LB policy builder in the global LB
/// registry.
pub fn reg() {
    GLOBAL_LB_REGISTRY.add_builder(Builder {});
}

/// Parsed configuration for the `priority_experimental` load balancing policy.
///
/// Corresponds to `PriorityLoadBalancingPolicyConfig` defined in
/// [gRFC A56 §LB Policy Configuration]:
///
/// ```proto
/// message PriorityLoadBalancingPolicyConfig {
///   map<string, Child> children = 1;
///   repeated string priorities = 2;
/// }
/// ```
///
/// [gRFC A56 §LB Policy Configuration]:
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
    children: HashMap<String, ChildConfig>,
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
}

/// Internal tracking data and state for a configured priority child.
#[derive(Debug)]
struct ChildData {
    /// The current lifecycle and connectivity state of the child policy.
    state: ChildState,
    /// The active dynamic LB configuration for this child.
    child_config: ChildConfig,
    /// The latest name resolver update received for this child.
    ///
    /// Preserved so that if this child is deactivated and later reactivated,
    /// or if it was uninitialized, it can be instantiated with the most
    /// recent endpoints and service configuration.
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
/// [gRFC A56 §Child Lifetime Management]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-lifetime-management
/// [gRFC A56 §Child Connectivity State Tracking]:
///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#child-connectivity-state-tracking
#[derive(Debug)]
enum ChildState {
    /// The child is configured but has not yet been instantiated in
    /// [`ChildManager`].
    ///
    /// It will be lazily created and initialized with
    /// [`ChildData::latest_update`] when evaluated during priority selection
    /// (see [gRFC A56 §Child Lifetime Management]).
    Uninitialized,

    /// The child is actively attempting to connect, with a 10-second failover
    /// timer running.
    ///
    /// While this timer is active, priority selection will wait for this child
    /// before evaluating lower priorities (see
    /// [gRFC A56 §Child Connectivity State Tracking]).
    Connecting(Timer, LbState),

    /// The child is in `CONNECTING` state, but its 10-second failover timer has
    /// expired.
    ///
    /// The child continues attempting connection in the background, but
    /// priority selection may now proceed to check lower priorities (see
    /// [gRFC A56 §Child Connectivity State Tracking]).
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
    /// the child is destroyed (see [gRFC A56 §Child Lifetime Management]).
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
            child_data: HashMap::default(),
            priorities: Vec::default(),
            published_lb_state: None,
            rt,
            work_scheduler: options.work_scheduler,
        }
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &ParsedJsonLbConfig) -> Result<Option<PriorityConfig>, String> {
        let cfg: PriorityConfig = config.convert_to().map_err(|e| e.to_string())?;
        cfg.validate()?;
        Ok(Some(cfg))
    }
}

/// The `priority_experimental` load balancing policy instance.
///
/// Manages a prioritized collection of child policies.
#[derive(Debug)]
struct PriorityPolicy {
    child_mgr: ChildManager<String, ChildBuilder>,
    child_data: HashMap<String, ChildData>,
    /// Current priority hierarchy: list of child names sorted from highest
    /// priority (0) to lowest.
    priorities: Vec<String>,
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
        config: Option<&Self::LbConfig>,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let Some(config) = config else {
            return Err("priority balancer received update with missing LB config".to_owned());
        };
        self.priorities = config.priorities.clone();
        let mut sharded_endpoints = update.endpoints.map(endpoint_filtering::group_by_path);

        // Remove children no longer present in any priority, see gRFC A115.
        self.child_data
            .retain(|k, _| config.children.contains_key(k));

        let mut updates_to_emit = Vec::new();

        for (k, child_cfg) in &config.children {
            let endpoints = match &mut sharded_endpoints {
                Ok(grouped) => Ok(grouped.remove(k).unwrap_or_default()),
                Err(status) => Err(status.clone()),
            };

            let resolver_update = ResolverUpdate {
                attributes: update.attributes.clone(),
                endpoints,
                service_config: update.service_config.clone(),
                resolution_note: update.resolution_note.clone(),
            };

            match self.child_data.entry(k.clone()) {
                Entry::Occupied(mut entry) => {
                    let data = entry.get_mut();
                    data.child_config = child_cfg.clone();
                    data.latest_update = resolver_update.clone();

                    if !matches!(data.state, ChildState::Uninitialized) {
                        // Stash key and resolver_update to assemble
                        // ChildUpdates in a second pass.
                        updates_to_emit.push((k.clone(), resolver_update));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(ChildData {
                        state: ChildState::Uninitialized,
                        child_config: child_cfg.clone(),
                        latest_update: resolver_update,
                    });
                }
            }
        }

        // Build child_updates with immutable references from self.child_data.
        let child_updates = updates_to_emit
            .into_iter()
            .map(|(child_id, resolver_update)| {
                let data = self.child_data.get(&child_id).expect(
                    "expected child_data entry to exist for child queued during resolver_update",
                );
                ChildUpdate {
                    child_identifier: child_id,
                    child_policy_builder: ChildBuilder {},
                    child_update: Some((resolver_update, Some(&data.child_config))),
                }
            });

        // Update children in ChildManager. As specified in gRFC A56
        // §Configuration Updates, priority re-evaluation is deferred until
        // all child updates have been applied.
        let res = self.child_mgr.update(child_updates, channel_controller);
        self.reconcile(channel_controller);
        res
    }

    fn subchannel_update(
        &mut self,
        subchannel: std::sync::Arc<dyn Subchannel>,
        state: &SubchannelState,
        channel_controller: &mut dyn ChannelController,
    ) {
        self.child_mgr
            .subchannel_update(subchannel, state, channel_controller);
        self.reconcile(channel_controller);
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
    fn update_child_data(&mut self) {
        for child in self.child_mgr.children() {
            let child_data = self.child_data.get_mut(&child.identifier).expect(
                "expected child_data entry to exist for active child reported by ChildManager",
            );
            // Take ownership of the current state and replace it with a
            // temporary Uninitialized value.
            let old_state = mem::replace(&mut child_data.state, ChildState::Uninitialized);
            let lb_state = child.state.clone();
            child_data.state = match old_state {
                ChildState::Deactivated(timer, _) => {
                    // While deactivated, retain the 15-minute deactivation
                    // timer and record the updated LbState (see gRFC A56 §Child
                    // Lifetime Management).
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
    /// [gRFC A56 §Algorithm for Choosing a Priority]:
    ///   https://github.com/grpc/proposal/blob/master/A56-priority-lb-policy.md#algorithm-for-choosing-a-priority
    fn choose_priority(&mut self, channel_controller: &mut dyn ChannelController) {
        // If priority list is empty, report TRANSIENT_FAILURE with
        // FailingPicker.
        if self.priorities.is_empty() {
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
        for idx in 0..self.priorities.len() {
            let child_id = self.priorities[idx].clone();
            let child_data = self.child_data.get_mut(&child_id).expect(
                "expected child_data entry to exist for priority during choose_priority pass 1",
            );

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
                let latest_update = {
                    child_data.state = ChildState::Connecting(
                        Timer::new(
                            CONNECTING_TIMEOUT,
                            self.work_scheduler.clone(),
                            self.rt.clone(),
                        ),
                        LbState::initial(),
                    );
                    child_data.latest_update.clone()
                };
                if self
                    .update_child(child_id.clone(), latest_update, channel_controller)
                    .is_err()
                {
                    channel_controller.request_resolution();
                }
                self.update_child_data();
            }

            let child_data = self.child_data.get(&child_id).expect(
                "expected child_data entry to exist for priority during choose_priority pass 1",
            );
            match &child_data.state {
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
        for idx in 0..self.priorities.len() {
            let child_id = &self.priorities[idx];
            let child_data = self.child_data.get(child_id).expect(
                "expected child_data entry to exist for priority during choose_priority pass 2",
            );
            if matches!(child_data.state, ChildState::ConnectingExpired(_)) {
                self.set_current_priority(channel_controller, idx, false);
                return;
            }
        }

        // We didn't find a child in CONNECTING, so delegate to the last child
        // (reporting its TRANSIENT_FAILURE state and failing picker).
        self.set_current_priority(channel_controller, self.priorities.len() - 1, false);
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
            for child_id in self.priorities.iter().skip(index + 1) {
                let child_data = self
                    .child_data
                    .get_mut(child_id)
                    .unwrap_or_else(|| panic!("missing child data for {child_id}"));
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
        let child_name = &self.priorities[index];
        let child_data = self.child_data.get(child_name).expect(
            "expected child_data entry to exist for selected priority in set_current_priority",
        );
        let lb_state = child_data
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
            .child_data
            .iter()
            .filter(|(_, cd)| !matches!(cd.state, ChildState::Uninitialized))
            .map(|(id, data)| {
                let update = if &child_id == id {
                    // .take() moves the owned value out without cloning.
                    // Since id is the key of a HashMap, there's at most one
                    // element that matches.
                    resolver_update
                        .take()
                        .map(|ru| (ru, Some(&data.child_config)))
                } else {
                    None
                };
                ChildUpdate {
                    child_identifier: id.clone(),
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
        for child_data in self.child_data.values_mut() {
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
    /// removed from `self.child_data` so they can be lazily re-created if
    /// higher priorities fail later (see [gRFC A56 §Child Lifetime
    /// Management]).
    fn handle_deactivation_timer(&mut self) {
        let mut any_expired = false;
        for child_data in self.child_data.values_mut() {
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
            .child_data
            .iter()
            .filter(|(_, cd)| !matches!(cd.state, ChildState::Uninitialized))
            .map(|(id, data)| (id.clone(), ChildBuilder {}));

        self.child_mgr.retain_children(iter);
    }
}

#[cfg(test)]
mod test;
