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

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use rand::seq::SliceRandom;

use crate::client::ConnectivityState;
use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::FailingPicker;
use crate::client::load_balancing::LbConfigJson;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::Pick;
use crate::client::load_balancing::PickOptions;
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::QueuingPicker;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::WorkScheduler;
use crate::client::load_balancing::subchannel::Subchannel;
use crate::client::load_balancing::subchannel::SubchannelState;
use crate::client::load_balancing::subchannel::SubchannelUpdate;
use crate::client::name_resolution::Endpoint;
use crate::client::name_resolution::ResolverUpdate;
use crate::core::Address;
use crate::metadata::MetadataMap;
use crate::rt::BoxedTaskHandle;
use crate::rt::GrpcRuntime;

pub static POLICY_NAME: &str = "pick_first";

// The Connection Attempt Delay (gRFC A61): how long to wait for a connection
// attempt before also starting one on the next address.
const CONNECTION_ATTEMPT_DELAY: Duration = Duration::from_millis(250);

type ShufflerFn = dyn Fn(&mut [Endpoint]) + Send + Sync + 'static;

#[derive(Debug, serde::Deserialize, Clone, Default)]
pub(crate) struct PickFirstConfig {
    #[serde(default, rename = "shuffleAddressList")]
    shuffle_address_list: bool,
}

#[derive(Debug)]
pub(crate) struct PickFirstBuilder {}

impl LbPolicyBuilder for PickFirstBuilder {
    type LbPolicy = PickFirstPolicy;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        PickFirstPolicy {
            work_scheduler: options.work_scheduler,
            runtime: options.runtime,
            connectivity_state: ConnectivityState::Idle,
            subchannels: Vec::default(),
            subchannel_states: HashMap::default(),
            selected: None,
            frontier_index: 0,
            last_connection_error: None,
            resolution_note: None,
            shuffler: build_shuffler(),
            timer: None,
            steady_state: None,
        }
    }

    fn name(&self) -> &'static str {
        POLICY_NAME
    }

    fn parse_config(&self, config: &LbConfigJson) -> Result<PickFirstConfig, String> {
        let config: PickFirstConfig = config.convert_to().map_err(|e| e.to_string())?;
        Ok(config)
    }
}

pub(crate) fn reg() {
    super::GLOBAL_LB_REGISTRY.add_builder(PickFirstBuilder {});
}

pub(crate) struct PickFirstPolicy {
    work_scheduler: Arc<dyn WorkScheduler>,
    runtime: GrpcRuntime,
    connectivity_state: ConnectivityState,

    // Subchannel information.
    subchannels: Vec<Arc<dyn Subchannel>>,
    subchannel_states: HashMap<Address, SubchannelState>, // Cached states for all subchannels by address.
    selected: Option<Arc<dyn Subchannel>>,
    frontier_index: usize,

    // Detailed error tracking.
    last_connection_error: Option<String>,

    // The resolution note from the latest resolver update. It is added to the
    // status of every TRANSIENT_FAILURE this policy reports.
    resolution_note: Option<String>,

    // Injectable shuffler for deterministic testing.
    shuffler: Arc<ShufflerFn>,

    // Timer state tracks when the last connect attempt was started.
    timer: Option<Timer>,

    // Steady state tracking for continuous retries after pass exhaustion.
    // TODO: should steady_state be a "mode" selector enum, e.g.:
    // - FirstPass - holds the timer above
    // - SteadyState - existing info
    // - Ready - holds selected subchannel
    // - Error - for zero addresses; holds no data
    steady_state: Option<SteadyState>,
}

impl Debug for PickFirstPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PickFirstPolicy")
            .field("subchannels", &self.subchannels)
            .field("selected", &self.selected)
            .field("frontier_index", &self.frontier_index)
            .field("connectivity_state", &self.connectivity_state)
            .field("last_connection_error", &self.last_connection_error)
            .field("resolution_note", &self.resolution_note)
            .finish()
    }
}

impl PickFirstPolicy {
    fn rebuild_subchannels(
        &mut self,
        new_addresses: Vec<Address>,
        channel_controller: &mut dyn ChannelController,
    ) -> Option<Arc<dyn Subchannel>> {
        // Map existing subchannels by address.
        let mut existing_subchannels: HashMap<Address, Arc<dyn Subchannel>> = self
            .subchannels
            .drain(..)
            .map(|sc| (sc.address(), sc))
            .collect();

        let mut new_subchannels = Vec::with_capacity(new_addresses.len());
        let mut new_states = HashMap::with_capacity(new_addresses.len());
        let mut ready_subchannel = None;

        for addr in new_addresses {
            let (sc, state) = if let Some(sc) = existing_subchannels.remove(&addr) {
                let state = self.subchannel_states.get(&addr).unwrap().clone();
                (sc, state)
            } else {
                // Get a new subchannel handle from the controller if we don't
                // have an existing one.  Updates for it are delivered to our
                // work method via our work scheduler.
                channel_controller.new_subchannel(&addr, self.work_scheduler.clone())
            };

            // Track the best candidate for immediate activation:
            // 1. Priority: The currently selected subchannel if still READY.
            // 2. Fallback: The first generic READY subchannel encountered.
            if state.connectivity_state == ConnectivityState::Ready {
                if self.subchannel_is_selected(&sc) {
                    // Sticky channel wins immediately and overrides any
                    // fallback candidates.
                    ready_subchannel = Some(sc.clone());
                } else if ready_subchannel.is_none() {
                    // Capture fallback candidate, but does not overwrite if a
                    //sticky channel was already found.
                    ready_subchannel = Some(sc.clone());
                }
            }

            new_subchannels.push(sc);
            new_states.insert(addr, state);
        }

        self.subchannels = new_subchannels;
        self.subchannel_states = new_states;
        ready_subchannel
    }

    /// Call when the selected subchannel loses connection.
    // This causes the LB to go IDLE.
    fn subchannel_drop(&mut self, channel_controller: &mut dyn ChannelController) {
        self.selected = None;
        self.update_picker(
            ConnectivityState::Idle,
            Arc::new(IdlePicker::new(self.work_scheduler.clone())),
            channel_controller,
        );
    }

    fn update_picker(
        &mut self,
        connectivity_state: ConnectivityState,
        picker: Arc<dyn Picker>,
        channel_controller: &mut dyn ChannelController,
    ) {
        if self.connectivity_state == connectivity_state
            && connectivity_state == ConnectivityState::Connecting
        {
            // Prevent redundant connecting updates.
            // TODO: prevent redundant IDLE updates?
            return;
        }
        self.connectivity_state = connectivity_state;
        channel_controller.update_picker(LbState {
            connectivity_state,
            picker,
        });
    }

    fn subchannel_activate(
        &mut self,
        subchannel: Arc<dyn Subchannel>,
        channel_controller: &mut dyn ChannelController,
    ) {
        if self.subchannel_is_selected(&subchannel) {
            // Already selected; skip activation.
            return;
        }
        self.selected = Some(subchannel.clone());
        self.subchannels = vec![subchannel.clone()]; // Keep only the winner.
        self.steady_state = None; // Reset mode to First Pass.
        self.timer = None; // Stop the happy eyeballs timer.

        self.update_picker(
            ConnectivityState::Ready,
            Arc::new(OneSubchannelPicker { sc: subchannel }),
            channel_controller,
        );
    }

    /// Starts a connection pass through the address list.
    // This clears the selected subchannel.
    fn start_connection_pass(&mut self, channel_controller: &mut dyn ChannelController) {
        self.selected = None;

        // If there is a viable subchannel at the frontier, connect to it and
        // update picker to CONNECTING.
        if let Some(sc) = self.advance_frontier(true) {
            self.trigger_subchannel_connection(sc, channel_controller);

            // TODO: avoid this update if we are in TF (i.e. sticky TF)?
            self.update_picker(
                ConnectivityState::Connecting,
                Arc::new(QueuingPicker {}),
                channel_controller,
            );
        } else {
            // Otherwise all addresses are in transient failure: update picker.
            // TODO: set the last connection error?  Is it correct to do so,
            // e.g. if the address it came from was removed from the address
            // list and a name resolver update triggered this connection pass?
            _ = self.set_transient_failure(channel_controller, None);
        }
    }

    // Book-keeping for tracking progress on the first pass through the address
    // list. Assumes the subchannel is in a non-READY state.
    // If the failure is from the subchannel at the frontier, advances the
    // frontier and triggers a connection on the next subchannel.
    fn update_first_pass(
        &mut self,
        subchannel: Arc<dyn Subchannel>,
        state: &SubchannelState,
        channel_controller: &mut dyn ChannelController,
    ) {
        // Advance frontier if this failure is from the active frontier subchannel.
        if let Some(attempting) = self.subchannels.get(self.frontier_index)
            && attempting.address() == subchannel.address()
            && state.connectivity_state == ConnectivityState::TransientFailure
            && let Some(next_sc) = self.advance_frontier(false)
        {
            self.trigger_subchannel_connection(next_sc, channel_controller);
        }

        // Check if First Pass is fully exhausted (frontier exhausted AND zero connecting).
        if self.frontier_index >= self.subchannels.len() {
            let any_connecting = self.subchannels.iter().any(|sc| {
                self.subchannel_states
                    .get(&sc.address())
                    .is_some_and(|s| s.connectivity_state == ConnectivityState::Connecting)
            });

            if !any_connecting {
                // Nothing currently connecting; first pass complete.  Enter
                // steady state.
                self.timer = None;
                let error = self.last_connection_error.clone();
                _ = self.set_transient_failure(channel_controller, error);
                self.steady_state = Some(SteadyState::new(self.subchannels.len()));

                // Trigger connection attempts on any subchannels that
                // transitioned to IDLE during the first pass, ensuring they
                // don't get stuck.
                for sc in &self.subchannels {
                    let is_idle = self
                        .subchannel_states
                        .get(&sc.address())
                        .is_some_and(|s| s.connectivity_state == ConnectivityState::Idle);
                    if is_idle {
                        sc.connect();
                    }
                }
            }
        }
    }

    /// Advances the frontier to the next non-TransientFailure subchannel and
    /// returns it. If `reset` is true, starts the scan from index 0.
    // The frontier is the latest index in which connectivity has been attempted.
    // Returns a cloned Subchannel to avoid borrowing from self.
    fn advance_frontier(&mut self, reset: bool) -> Option<Arc<dyn Subchannel>> {
        if reset {
            self.frontier_index = 0;
        } else {
            self.frontier_index += 1;
        }

        while self.frontier_index < self.subchannels.len() {
            let sc = &self.subchannels[self.frontier_index];
            let addr = sc.address();
            let state = self
                .subchannel_states
                .get(&addr)
                .map(|s| s.connectivity_state)
                .expect("Expected non-None subchannel state");

            match state {
                // Push the frontier if sc is in TransientFailure
                ConnectivityState::TransientFailure => self.frontier_index += 1,
                // Otherwise return the subchannel.
                _ => return Some(sc.clone()),
            }
        }
        None
    }

    /// Returns true if the given subchannel matches the currently selected
    /// active subchannel.
    fn subchannel_is_selected(&self, subchannel: &Arc<dyn Subchannel>) -> bool {
        self.selected
            .as_ref()
            .is_some_and(|sel| sel.address() == subchannel.address())
    }

    /// Returns true if the subchannel's address is present in the most recently
    /// received address list.
    // This compares against the current list of subchannels the LB is
    // attempting to connect to. To see if the LB already connected to the
    // channel, use 'subchannel_is_selected'.
    fn subchannel_is_current(&self, subchannel: &Arc<dyn Subchannel>) -> bool {
        self.subchannels
            .iter()
            .any(|sc| sc.address() == subchannel.address())
    }

    /// Triggers a connection on the subchannel, and starts the 250ms timer.
    /// If no connection succeeds before the timer expires, the frontier will
    /// advance to the next subchannel.
    fn trigger_subchannel_connection(
        &mut self,
        sc: Arc<dyn Subchannel>,
        channel_controller: &mut dyn ChannelController,
    ) {
        self.subchannel_states.insert(
            sc.address(),
            SubchannelState {
                connectivity_state: ConnectivityState::Connecting,
                last_connection_error: None,
            },
        );
        sc.connect();

        // Start happy eyeballs timer; replacing any pre-existing timer.
        self.timer = Some(Timer::start(
            self.runtime.clone(),
            self.work_scheduler.clone(),
        ));
    }

    // Converts the update endpoints to an address list.
    // Shuffles endpoints (if enabled) before flattening and de-duplication.
    fn compile_address(
        &mut self,
        mut endpoints: Vec<Endpoint>,
        config: &PickFirstConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Vec<Address> {
        // Shuffle endpoints if enabled.
        if config.shuffle_address_list {
            (self.shuffler)(&mut endpoints);
        }

        // Flatten and de-duplicate unique addresses in order.
        let mut seen = HashSet::new();
        let unique_addresses: Vec<Address> = endpoints
            .into_iter()
            .flat_map(|ep| ep.addresses)
            .filter(|addr| seen.insert(addr.clone()))
            .collect();

        // Partition out all 'unknown' non-TCP addresses.
        // This is to remain consistent with similar behavior in C++ and Java.
        let (tcp_addresses, unknown): (Vec<Address>, Vec<Address>) =
            unique_addresses.into_iter().partition(|addr| {
                addr.network_type == crate::client::name_resolution::TCP_IP_NETWORK_TYPE
            });

        // Partition by family (Basic IPv6 detection via colon).
        let (ipv6, ipv4): (Vec<Address>, Vec<Address>) = tcp_addresses
            .into_iter()
            .partition(|addr| addr.address.contains(':'));

        // Interleave the two lists so ipv6 and ipv4 addresses are alternated.
        let mut interleaved = Vec::with_capacity(ipv6.len() + ipv4.len() + unknown.len());
        let mut v6_iter = ipv6.into_iter();
        let mut v4_iter = ipv4.into_iter();
        let mut unknown_iter = unknown.into_iter();

        loop {
            let mut more = false;

            if let Some(v6) = v6_iter.next() {
                interleaved.push(v6);
                more = true;
            }
            if let Some(v4) = v4_iter.next() {
                interleaved.push(v4);
                more = true;
            }
            if let Some(unknown) = unknown_iter.next() {
                interleaved.push(unknown);
                more = true;
            }

            if !more {
                break;
            }
        }
        interleaved
    }

    // Sets LB state to TRANSIENT_FAILURE and updates picker with error.
    // Triggers a re-resolution request.
    // TODO: make error mandatory.
    fn set_transient_failure(
        &mut self,
        channel_controller: &mut dyn ChannelController,
        error: Option<String>,
    ) -> Result<(), String> {
        // Replace the last connection error if we have a new one.
        if let Some(e) = error {
            self.last_connection_error = Some(e);
        }
        let err = self
            .last_connection_error
            .clone()
            .expect("no last connection error set");
        // Include the resolver's note, which may explain the failure (e.g. why
        // the address list is empty).
        let err = match self.resolution_note.as_deref() {
            Some(note) if !note.is_empty() => format!("{err} ({note})"),
            _ => err,
        };
        self.update_picker(
            ConnectivityState::TransientFailure,
            Arc::new(FailingPicker { error: err.clone() }),
            channel_controller,
        );
        channel_controller.request_resolution();
        Err(err.clone())
    }

    fn subchannel_update(
        &mut self,
        subchannel: Arc<dyn Subchannel>,
        state: &SubchannelState,
        channel_controller: &mut dyn ChannelController,
    ) {
        if !self.subchannel_is_current(&subchannel) {
            // This update is from an outdated subchannel that is no longer in
            // the address list. Ignore it.
            return;
        }

        // Update the cache for all updates.
        self.subchannel_states
            .insert(subchannel.address(), state.clone());

        match (
            // Does the load balancer have an active subchannel already?
            self.subchannel_is_selected(&subchannel),
            // What is the updating subchannel's state?
            state.connectivity_state,
        ) {
            (true, ConnectivityState::Ready) => {
                // The selected subchannel is still ready; do nothing w/update.
            }
            (true, _) => {
                // The selected subchannel has failed (is no longer READY);
                // drop the connection.
                self.subchannel_drop(channel_controller);
            }
            (false, ConnectivityState::Ready) => {
                // The updating subchannel is READY; activate it.
                self.subchannel_activate(subchannel, channel_controller);
            }
            (false, _) => {
                // Always capture freshest unselected error.
                if state.connectivity_state == ConnectivityState::TransientFailure
                    && let Some(err) = &state.last_connection_error
                {
                    self.last_connection_error = Some(err.clone());
                }

                // Track progress based on whether we are in a connection pass.
                if let Some(steady) = self.steady_state.as_mut() {
                    steady.subchannel_nonready(channel_controller, subchannel, state);
                } else {
                    self.update_first_pass(subchannel, state, channel_controller);
                }
            }
        }
    }
}

impl LbPolicy for PickFirstPolicy {
    type LbConfig = PickFirstConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: &Self::LbConfig,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        self.timer = None;

        // Reset steady state on new update
        self.steady_state = None;

        self.resolution_note = update.resolution_note;

        match update.endpoints {
            Ok(endpoints) => {
                let new_addresses = self.compile_address(endpoints, config, channel_controller);
                // If we have no addresses, clear subchannels and set TRANSIENT_FAILURE.
                if new_addresses.is_empty() {
                    self.subchannels.clear();
                    self.selected = None;
                    self.set_transient_failure(
                        channel_controller,
                        Some("empty address list".to_string()),
                    )?;
                }

                if let Some(ready_subchannel) =
                    self.rebuild_subchannels(new_addresses, channel_controller)
                {
                    self.subchannel_activate(ready_subchannel, channel_controller);
                } else {
                    self.start_connection_pass(channel_controller);
                }
            }
            Err(e) => {
                let error = e.to_string();
                if self.subchannels.is_empty()
                    || self.connectivity_state == ConnectivityState::TransientFailure
                {
                    self.set_transient_failure(channel_controller, Some(error))?;
                }
            }
        }

        Ok(())
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        if let Some(data) = data {
            // If data is set, it should be a subchannel update.
            match data.downcast::<SubchannelUpdate>() {
                Ok(update) => {
                    self.subchannel_update(update.subchannel, &update.state, channel_controller);
                }
                Err(data) => debug_assert!(
                    false,
                    "work called with {data:?}; expected a SubchannelUpdate"
                ),
            }
            return;
        }
        if self.connectivity_state == ConnectivityState::Idle {
            // TODO: is it safe to assume any call to work() while idle means we
            // should connect?
            self.exit_idle(channel_controller);
        } else if self.timer.as_ref().is_some_and(|t| t.expired()) {
            // Advance frontier and trigger next connection.
            if let Some(next_sc) = self.advance_frontier(false) {
                self.trigger_subchannel_connection(next_sc, channel_controller);
            }
        }
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        // Only start a connection pass when in Idle and have already
        // received a non-empty resolver update (`subchannels` is non-empty).
        // Before the first resolver update, `connectivity_state` is Idle while
        // `subchannels` is empty.
        if self.connectivity_state == ConnectivityState::Idle && !self.subchannels.is_empty() {
            self.start_connection_pass(channel_controller);
        }
    }
}

/// Implements the happy eyeballs timer task.  `expired` becomes set when it
/// fires.  When dropped, the timer is cancelled.
struct Timer {
    expired: Arc<AtomicBool>,
    handle: BoxedTaskHandle,
}

impl Timer {
    /// Starts a new timer, returning it.
    fn start(runtime: GrpcRuntime, work_scheduler: Arc<dyn WorkScheduler>) -> Self {
        let expired = Arc::new(AtomicBool::new(false));
        let expired_clone = expired.clone();
        let handle = runtime.clone().spawn(Box::pin(async move {
            runtime.sleep(CONNECTION_ATTEMPT_DELAY).await;
            expired_clone.store(true, Ordering::SeqCst);
            work_scheduler.schedule_work(None);
        }));
        Self { expired, handle }
    }

    /// Returns whether the timer has expired yet.
    fn expired(&self) -> bool {
        self.expired.load(Ordering::SeqCst)
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[derive(Debug)]
struct OneSubchannelPicker {
    sc: Arc<dyn Subchannel>,
}

impl Picker for OneSubchannelPicker {
    fn pick(&self, _: PickOptions<'_>) -> PickResult {
        PickResult::Pick(Pick {
            subchannel: self.sc.clone(),
            metadata: MetadataMap::new(),
            on_complete: None,
        })
    }
}

#[derive(Debug)]
struct IdlePicker {
    triggered_work: AtomicBool,
    work_scheduler: Arc<dyn WorkScheduler>,
}

impl IdlePicker {
    fn new(work_scheduler: Arc<dyn WorkScheduler>) -> Self {
        Self {
            triggered_work: AtomicBool::new(false),
            work_scheduler,
        }
    }
}

impl Picker for IdlePicker {
    fn pick(&self, _: PickOptions<'_>) -> PickResult {
        if !self.triggered_work.swap(true, Ordering::Relaxed) {
            self.work_scheduler.schedule_work(None);
        }
        PickResult::Queue
    }
}

fn build_shuffler() -> Arc<ShufflerFn> {
    Arc::new(|endpoints| {
        let mut rng = rand::rng();
        endpoints.shuffle(&mut rng);
    })
}

/// Tracks a the 'steady state' pass of subchannels when looking for a ready
/// connection. If the number of reported subchannel failures reaches the
/// failure threshold, this will ask the Name Resolver to re-resolve.
#[derive(Debug)]
struct SteadyState {
    /// The number of failures before triggering a re-resolution of addresses.
    /// This is a rough heuristic to approximate if all subchannels have failed
    /// since we entered steady state, and can be tuned as needed.
    failure_threshold: usize,
    /// The number of failures connecting, used to roughly approximate if a
    /// re-resolution needs to happen.
    failure_count: usize,
}

impl SteadyState {
    fn new(threshold: usize) -> Self {
        Self {
            failure_threshold: threshold,
            failure_count: 0,
        }
    }

    /// Handles non-ready subchannel updates when the LB is in 'steady state'
    /// connection mode.
    fn subchannel_nonready(
        &mut self,
        channel_controller: &mut dyn ChannelController,
        subchannel: Arc<dyn Subchannel>,
        state: &SubchannelState,
    ) {
        match state.connectivity_state {
            ConnectivityState::Idle => {
                // Subchannel backoff expired: trigger reconnection attempt.
                subchannel.connect();
            }
            ConnectivityState::TransientFailure => {
                // Track failures. If all known subchannels have failed,
                // request new addresses.
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.failure_count = 0;
                    channel_controller.request_resolution();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::time::Duration;

    use super::*;
    use crate::client::CallOptions;
    use crate::client::load_balancing::test_utils;
    use crate::client::load_balancing::test_utils::TestChannelController;
    use crate::client::load_balancing::test_utils::TestEnv;
    use crate::rt::default_runtime;

    // A ChannelController wrapper that delegates to TestChannelController,
    // overriding the initial SubchannelState returned by new_subchannel for
    // configured addresses.
    struct InitialStateController<'a> {
        tcc: &'a mut TestChannelController,
        initial_states: HashMap<&'a str, SubchannelState>,
    }

    impl ChannelController for InitialStateController<'_> {
        fn new_subchannel(
            &mut self,
            address: &Address,
            work_scheduler: Arc<dyn WorkScheduler>,
        ) -> (Arc<dyn Subchannel>, SubchannelState) {
            let (sc, state) = self.tcc.new_subchannel(address, work_scheduler);
            let state = self
                .initial_states
                .get(&*address.address)
                .cloned()
                .unwrap_or(state);
            (sc, state)
        }

        fn update_picker(&mut self, update: LbState) {
            self.tcc.update_picker(update);
        }

        fn request_resolution(&mut self) {
            self.tcc.request_resolution();
        }
    }

    // Creates an endpoint containing `addrs`, in order.
    fn endpoint(addrs: &[&str]) -> Endpoint {
        Endpoint {
            addresses: addrs
                .iter()
                .map(|addr| Address {
                    address: crate::byte_str::ByteStr::from(addr.to_string()),
                    network_type: crate::client::name_resolution::TCP_IP_NETWORK_TYPE,
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }

    // Creates one single-address endpoint per entry in `addrs`, as the DNS
    // resolver does.
    fn create_endpoints(addrs: &[&str]) -> Vec<Endpoint> {
        addrs.iter().map(|addr| endpoint(&[addr])).collect()
    }

    // Constructs the test environment for PickFirstPolicy tests.
    fn new_env() -> TestEnv<PickFirstPolicy> {
        let mut env = TestEnv::new(|work_scheduler| {
            PickFirstBuilder {}.build(LbPolicyOptions {
                work_scheduler,
                runtime: default_runtime(),
            })
        });

        // Deterministic shuffling for tests: reverse the endpoints
        env.policy.shuffler = Arc::new(|endpoints| {
            endpoints.reverse();
        });

        env
    }

    impl TestEnv<PickFirstPolicy> {
        // Sends a resolver update for `addrs`, and verifies that the policy
        // creates their subchannels, asks the first one to connect, and
        // reports CONNECTING. Returns the subchannels, in address order.
        fn simulate_connection(&mut self, addrs: &[&str]) -> Vec<Arc<dyn Subchannel>> {
            self.send_resolver_update(create_endpoints(addrs)).unwrap();
            let subchannels = self.expect_new_subchannels(addrs);
            self.expect_connect_to(addrs[0]);
            self.expect_queuing_picker();
            subchannels
        }

        // Calls simulate_connection, makes the first address READY, and
        // verifies that the policy selects it.
        fn simulate_successful_connection(&mut self, addrs: &[&str]) -> Vec<Arc<dyn Subchannel>> {
            let subchannels = self.simulate_connection(addrs);
            self.send_subchannel_update(&subchannels[0], &SubchannelState::ready());
            self.expect_ready_picker(&subchannels[0]);
            subchannels
        }

        // Calls simulate_connection and fails the first address with
        // "connection refused". Events resulting from the failure are left for
        // the caller to verify.
        fn simulate_failed_connection(&mut self, addrs: &[&str]) -> Vec<Arc<dyn Subchannel>> {
            let subchannels = self.simulate_connection(addrs);
            self.send_subchannel_update(
                &subchannels[0],
                &SubchannelState::transient_failure("connection refused"),
            );
            subchannels
        }

        /// Sends a resolver update with `endpoints` and resolution note
        /// `note`, using the default config.
        fn send_resolver_update_with_note(
            &mut self,
            endpoints: Vec<Endpoint>,
            note: &str,
        ) -> Result<(), String> {
            let update = ResolverUpdate {
                endpoints: Ok(endpoints),
                resolution_note: Some(note.to_string()),
                ..Default::default()
            };
            self.policy
                .resolver_update(update, &PickFirstConfig::default(), &mut self.tcc)
        }

        // Sends a resolver update for `addrs` while overriding the initial
        // SubchannelState returned by new_subchannel for entries in
        // `initial_states`.
        fn send_resolver_update_with_initial_states(
            &mut self,
            addrs: &[&str],
            initial_states: &[(&str, SubchannelState)],
        ) -> Result<(), String> {
            let update = ResolverUpdate {
                endpoints: Ok(create_endpoints(addrs)),
                ..Default::default()
            };
            let mut controller = InitialStateController {
                tcc: &mut self.tcc,
                initial_states: initial_states.iter().cloned().collect(),
            };
            self.policy
                .resolver_update(update, &PickFirstConfig::default(), &mut controller)
        }

        // Advances virtual time by `duration` on the paused Tokio runtime,
        // yielding before and after so spawned timer tasks can register on the
        // timer wheel and schedule their completion work.
        async fn advance_time(&mut self, duration: Duration) {
            tokio::task::yield_now().await;
            tokio::time::advance(duration).await;
            tokio::task::yield_now().await;
        }

        // Advances the paused clock past the Connection Attempt Delay, letting
        // any pending timer fire.
        async fn advance_past_timer(&mut self) {
            self.advance_time(CONNECTION_ATTEMPT_DELAY + Duration::from_millis(1))
                .await;
        }

        // Lets the Connection Attempt Delay timer fire, and delivers the
        // resulting work to the policy. Requires a paused clock.
        async fn fire_timer(&mut self) {
            self.advance_past_timer().await;
            self.run_work();
        }
    }

    // If the first address fails, the LB should failover to the second address.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_failover() {
        let mut env = new_env();
        let scs = env.simulate_failed_connection(&["addr1", "addr2"]);

        // Should connect to addr2.
        env.expect_connect_to("addr2");

        // Simulate addr2 succeeding.
        env.send_subchannel_update(&scs[1], &SubchannelState::ready());
        env.expect_ready_picker(&scs[1]);
        env.expect_no_events();
    }

    // Ensures that if a subchannel is already selected, and is still present in
    // the new resolver update, the LB will keep using it and not switch to a
    // different subchannel.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_stickiness() {
        let mut env = new_env();
        let scs = env.simulate_successful_connection(&["addr1", "addr2"]);

        // New resolver update including addr1.
        let endpoints_new = create_endpoints(&["addr2", "addr1", "addr3"]);
        env.send_resolver_update(endpoints_new).unwrap();

        // Should create new subchannels for addr2 (was cleared by cleanup) and
        // addr3 (was not in previous list).
        env.expect_new_subchannels(&["addr2", "addr3"]);

        // Should NOT have any more events (no Connect, no UpdatePicker),
        // because it stuck to the original selected subchannel.
        env.expect_no_events();

        // addr1 is still selected: only the selected subchannel going IDLE
        // moves the policy to IDLE.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());
        env.expect_picker_state(ConnectivityState::Idle);
        env.expect_no_events();
    }

    // When an update removes the selected address, the policy starts a new
    // first pass from the start of the new list and reports CONNECTING.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_resolver_update_without_selected_address_starts_first_pass() {
        let mut env = new_env();

        // Select addr2, so the old pass ended past the start of the list.
        let scs = env.simulate_failed_connection(&["addr1", "addr2"]);
        env.expect_connect_to("addr2");
        env.send_subchannel_update(&scs[1], &SubchannelState::ready());
        env.expect_ready_picker(&scs[1]);

        // The update drops addr2. addr1 gets a new subchannel: its old one was
        // discarded when addr2 was selected.
        env.send_resolver_update(create_endpoints(&["addr1", "addr3"]))
            .unwrap();
        env.expect_new_subchannels(&["addr1", "addr3"]);
        // The new pass starts at the head of the new list, not where the old
        // pass left off.
        env.expect_connect_to("addr1");
        // The policy reports CONNECTING rather than going IDLE.
        env.expect_queuing_picker();
        env.expect_no_events();
    }

    // Shuffling and interleaving of addresses is deterministic and correct
    // based on the provided shuffler and config.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_shuffling_and_interleaving_deterministic() {
        let mut env = new_env();

        // Enable shuffling in config.
        let config = PickFirstConfig {
            shuffle_address_list: true,
        };

        // Provide three endpoints:
        // EP1: [V6_1, V4_1]
        // EP2: [V6_2]
        // EP3: [V4_2]
        let endpoints = vec![
            endpoint(&["::1", "127.0.0.1"]),
            endpoint(&["::2"]),
            endpoint(&["127.0.0.2"]),
        ];

        env.send_resolver_update_with_config(endpoints, &config)
            .unwrap();

        // Mock shuffler reverses endpoints: [EP3, EP2, EP1]
        // EP3: [127.0.0.2] (V4)
        // EP2: [::2] (V6)
        // EP1: [::1] (V6), [127.0.0.1] (V4)
        //
        // Categorized:
        // IPv6: [::2, ::1]
        // IPv4: [127.0.0.2, 127.0.0.1]
        //
        // Interleaved: [::2, 127.0.0.2, ::1, 127.0.0.1]
        env.expect_new_subchannels(&["::2", "127.0.0.2", "::1", "127.0.0.1"]);
        env.expect_connect_to("::2");
        env.expect_queuing_picker();
        env.expect_no_events();
    }

    // De-duplicate addresses that appear multiple times within the same
    // endpoint, and across different endpoints. One subchannel each.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_duplicate_de_duplication() {
        let mut env = new_env();

        // Create endpoints with duplicates. The addresses are not TCP, so this
        // also checks that every non-TCP address survives the interleaving.
        let mut endpoints = vec![endpoint(&["addr1", "addr1"]), endpoint(&["addr2", "addr1"])];
        for addr in endpoints.iter_mut().flat_map(|ep| ep.addresses.iter_mut()) {
            addr.network_type = crate::client::name_resolution::UNIX_NETWORK_TYPE;
        }

        env.send_resolver_update(endpoints).unwrap();

        // Should only create subchannels for addr1 and addr2 (2 unique addrs).
        env.expect_new_subchannels(&["addr1", "addr2"]);

        // Verify no 3rd subchannel was created.
        env.expect_connect_to("addr1");
        env.expect_queuing_picker();
        env.expect_no_events();
    }

    // If the resolver update contains no addresses, the LB should clear
    // subchannels, update to TransientFailure, and request re-resolution.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_empty_update_clears_state() {
        let mut env = new_env();
        let scs = env.simulate_successful_connection(&["addr1", "addr2"]);

        // Send empty update.
        let res = env.send_resolver_update(vec![]);

        assert_eq!(res, Err("empty address list".to_string()));

        // Check picker is in TransientFailure.
        env.expect_failing_picker("empty address list");

        // Check that re-resolution was requested.
        env.expect_request_resolution();
        env.expect_no_events();

        // The subchannels were cleared, so updates from them are ignored. The
        // old selected subchannel going IDLE does not move the policy to IDLE.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());
        env.expect_no_events();
        // Nor is it selected when it becomes READY again.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());
        env.expect_no_events();

        // The selection was cleared too: addr1 returning READY in a new update
        // is selected again.
        env.send_resolver_update_with_initial_states(
            &["addr1"],
            &[("addr1", SubchannelState::ready())],
        )
        .unwrap();
        let sc = env.expect_new_subchannel_for("addr1");
        env.expect_ready_picker(&sc);
        env.expect_no_events();
    }

    // The resolution note is added to connection failures.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_connection_failure_includes_resolution_note() {
        let mut env = new_env();
        env.send_resolver_update_with_note(create_endpoints(&["addr1"]), "note")
            .unwrap();
        let sc1 = env.expect_new_subchannel_for("addr1");
        env.expect_connect_to("addr1");
        env.expect_queuing_picker();

        env.send_subchannel_update(
            &sc1,
            &SubchannelState::transient_failure("connection refused"),
        );

        env.expect_failing_picker("connection refused (note)");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // The resolver's resolution note is added to the error of the
    // TransientFailure picker, e.g. to explain why the address list is empty.
    // Each resolver update replaces the resolution note, so a note isn't used
    // after an update without one.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_resolution_note_replaced_by_next_update() {
        let mut env = new_env();
        let res = env.send_resolver_update_with_note(vec![], "old note");
        assert_eq!(res, Err("empty address list (old note)".to_string()));
        env.expect_failing_picker("empty address list (old note)");
        env.expect_request_resolution();

        let res = env.send_resolver_update(vec![]);

        assert_eq!(res, Err("empty address list".to_string()));
        env.expect_failing_picker("empty address list");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // If all addresses fail during a connection pass, the LB should enter
    // steady state and monitor for backoff expiry to retry connections.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_steady_state_retries() {
        let mut env = new_env();
        let scs = env.simulate_failed_connection(&["addr1"]);

        // Expect UpdatePicker(TransientFailure) and RequestResolution.
        env.expect_failing_picker("connection refused");
        env.expect_request_resolution();

        // Simulate addr1 transitioning to IDLE (backoff over).
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());

        // Should automatically call connect() again.
        env.expect_connect_to("addr1");
        env.expect_no_events();
    }

    // A subchannel that returns to IDLE during the first pass is not retried
    // until the first pass completes. Entering steady state connects it
    // immediately, steady state retries it on IDLE, and the policy switches to
    // a subchannel as soon as it becomes READY (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_steady_state_multi_backend() {
        let mut env = new_env();
        let scs = env.simulate_failed_connection(&["addr1", "addr2"]);

        // Should failover to addr2: expect Connect(addr2).
        env.expect_connect_to("addr2");

        // While addr2 is connecting, simulate addr1 going IDLE (backoff over).
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());

        // We should NOT reconnect to addr1 during the first pass.
        env.expect_no_events();

        // Now fail addr2 to complete first pass.
        env.send_subchannel_update(
            &scs[1],
            &SubchannelState::transient_failure("connection refused"),
        );

        // Expect UpdatePicker(TransientFailure), RequestResolution, and
        // Connect(addr1) from first pass exhaustion.
        env.expect_failing_picker("connection refused");
        env.expect_request_resolution();
        env.expect_connect_to("addr1");

        // Simulate addr1 going IDLE again.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());

        // Now it should automatically call connect() again.
        env.expect_connect_to("addr1");

        // Simulate addr1 successfully connecting and becoming READY.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());

        // The policy should switch to it immediately (enter READY state).
        env.expect_ready_picker(&scs[0]);
        env.expect_no_events();
    }

    // In steady state the policy requests re-resolution (gRFC A61) after N
    // (subchannel count) failures.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_steady_state_reresolves_every_n_failures() {
        let mut env = new_env();
        let scs = env.simulate_failed_connection(&["addr1", "addr2"]);
        env.expect_connect_to("addr2");
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 failed"));
        env.expect_failing_picker("addr2 failed");
        env.expect_request_resolution();

        // The first failure in steady state is counted, without re-resolving.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());
        env.expect_connect_to("addr1");
        env.send_subchannel_update(&scs[0], &SubchannelState::transient_failure("addr1 again"));
        env.expect_no_events();

        // The second failure reaches N.
        env.send_subchannel_update(&scs[1], &SubchannelState::idle());
        env.expect_connect_to("addr2");
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 again"));
        env.expect_request_resolution();
        env.expect_no_events();

        // The count starts over.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());
        env.expect_connect_to("addr1");
        env.send_subchannel_update(&scs[0], &SubchannelState::transient_failure("addr1 third"));
        env.expect_no_events();

        // Another N failures request re-resolution again.
        env.send_subchannel_update(&scs[1], &SubchannelState::idle());
        env.expect_connect_to("addr2");
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 third"));
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // This test is meant to validate that if a new address with different
    // attributes is sent as part of a resolver update, the policy treats it as
    // a different address and creates a new subchannel for it, rather than
    // ignoring it as a duplicate.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_address_update_with_attributes() {
        let addr = "addr1";
        let mut env = new_env();
        let scs = env.simulate_connection(&[addr]);

        // Push same address but with attributes.
        let mut endpoint_updated = endpoint(&[addr]);
        endpoint_updated.addresses[0].attributes =
            crate::attributes::Attributes::new().add("metadata_value".to_string());

        env.send_resolver_update(vec![endpoint_updated]).unwrap();

        // This should be a different subchannel due to different attributes.
        let sc = env.expect_new_subchannel_for(addr);
        assert_ne!(sc.address(), scs[0].address());
        env.expect_connect_to(addr);
        env.expect_no_events();
    }

    // When a resolver update arrives during the first pass, subchannels that
    // already failed keep their TRANSIENT_FAILURE state, so the new pass skips
    // them instead of attempting them again (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_resolver_update_preserves_subchannel_failure_state() {
        let mut env = new_env();

        // addr1 fails. The pass moves on to addr2.
        env.simulate_failed_connection(&["addr1", "addr2"]);
        env.expect_connect_to("addr2");

        // The new list keeps addr1, which already failed, and adds addr3.
        env.send_resolver_update(create_endpoints(&["addr1", "addr3"]))
            .unwrap();
        // addr1's subchannel is reused; only addr3 gets a new one.
        let sc3 = env.expect_new_subchannel_for("addr3");
        // The new pass skips addr1, though it is first in the list.
        env.expect_connect_to("addr3");
        env.expect_no_events();

        // When addr3 fails too, the pass is over.
        env.send_subchannel_update(&sc3, &SubchannelState::transient_failure("addr3 failed"));
        env.expect_failing_picker("addr3 failed");
        env.expect_request_resolution();
        // addr1 still counts as failed, so it is not retried. Had it been reset
        // to IDLE, the end of the pass would connect it here.
        env.expect_no_events();
    }

    // Updates from subchannels whose addresses were removed by a resolver
    // update are ignored, both during the first pass and in steady state.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_ignores_removed_subchannel_updates() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1", "addr2"]);

        // Replace the address list with [addr3].
        env.send_resolver_update(create_endpoints(&["addr3"]))
            .unwrap();
        let sc3 = env.expect_new_subchannel_for("addr3");
        env.expect_connect_to("addr3");

        // In the first pass, READY from a removed subchannel does not select
        // it.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());
        env.expect_no_events();

        // addr3 fails, ending the first pass.
        env.send_subchannel_update(&sc3, &SubchannelState::transient_failure("addr3 failed"));
        env.expect_failing_picker("addr3 failed");
        env.expect_request_resolution();

        // In steady state, a subchannel going IDLE is asked to connect, but
        // not a removed one.
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());
        env.expect_no_events();
        // With one address left, every failure in steady state requests
        // re-resolution, but not a removed subchannel's failure.
        env.send_subchannel_update(
            &scs[1],
            &SubchannelState::transient_failure("connection refused"),
        );
        env.expect_no_events();

        // addr3 is still the only address in use.
        env.send_subchannel_update(&sc3, &SubchannelState::idle());
        env.expect_connect_to("addr3");
        env.send_subchannel_update(&sc3, &SubchannelState::ready());
        env.expect_ready_picker(&sc3);
        env.expect_no_events();
    }

    // If a resolver error is received while the LB is in the process of
    // connecting to addresses, it should not abort the connection attempt or
    // clear the existing addresses, as long as there are still valid addresses
    // in the LB. This ensures that transient resolver errors do not cause
    // unnecessary disruption to active connection attempts.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_resolver_error_during_connecting() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1"]);

        // Simulate resolver error arriving.
        let resolver_error = "dns resolution failed".to_string();
        env.send_resolver_error(resolver_error).unwrap();

        // Resolver errors received during active connection attempts should NOT
        // abort the attempt or force TransientFailure immediately if the load
        // balancer still has valid addresses.
        // Expect NO events to be emitted (no UpdatePicker/RequestResolution).
        env.expect_no_events();

        // The in-progress connection attempt is still honored.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());
        env.expect_ready_picker(&scs[0]);
        env.expect_no_events();
    }

    // Out-of-Order Failure Detection
    // Ensures the policy waits for all parallel connection attempts to drop
    // before failing the channel.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_happy_eyeballs_out_of_order_failure() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1", "addr2"]);

        // Simulate Happy Eyeballs timer firing to launch parallel connection
        // to addr2.
        env.fire_timer().await;
        env.expect_connect_to("addr2");

        // Simulate addr2 failing first while addr1 is still in flight.
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 failed"));

        // Verify policy does NOT enter TransientFailure yet.
        env.expect_no_events();

        // Simulate addr1 failing. Pass is now fully exhausted.
        env.send_subchannel_update(&scs[0], &SubchannelState::transient_failure("addr1 failed"));

        env.expect_failing_picker("addr1 failed");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // An earlier connection attempt that succeeds after the timer moved the
    // pass on is selected, and the running timer is cancelled (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_earlier_attempt_ready_cancels_timer() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1", "addr2", "addr3", "addr4"]);

        // Each timer moves the pass on and starts the next attempt's timer: the
        // second timer fires only because the first one started it.
        env.fire_timer().await;
        env.expect_connect_to("addr2");
        env.fire_timer().await;
        env.expect_connect_to("addr3");

        // addr1's attempt, from two timers back, succeeds and is selected.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());
        env.expect_ready_picker(&scs[0]);

        // No timer fires: the timer for addr3's attempt was cancelled.
        env.advance_past_timer().await;
        env.expect_no_events();
    }

    // When the current connection attempt fails, the next attempt starts at
    // once with its own Connection Attempt Delay timer; the failed attempt's
    // timer is cancelled (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_attempt_failure_restarts_timer() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1", "addr2", "addr3"]);

        // addr1 fails partway through its delay; addr2 is attempted at once.
        env.advance_time(CONNECTION_ATTEMPT_DELAY / 2).await;
        env.send_subchannel_update(&scs[0], &SubchannelState::transient_failure("addr1 failed"));
        env.expect_connect_to("addr2");
        env.expect_no_events();

        // Past addr1's original deadline, but before addr2's: nothing fires.
        env.advance_time(CONNECTION_ATTEMPT_DELAY * 3 / 4).await;
        env.expect_no_events();

        // addr2's own timer fires 250ms after its attempt started, moving the
        // pass on to addr3.
        env.advance_time(CONNECTION_ATTEMPT_DELAY / 4 + Duration::from_millis(1))
            .await;
        env.run_work();
        env.expect_connect_to("addr3");
        env.expect_no_events();
    }

    // A failure from a subchannel the pass has not reached yet does not count
    // toward ending the pass: the subchannel may be IDLE again by the time the
    // pass reaches it, and the pass then attempts it and waits for the result.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_failure_not_yet_reached_not_counted() {
        let mut env = new_env();
        let scs = env.simulate_connection(&["addr1", "addr2", "addr3"]);

        // addr2 fails and finishes backoff before the pass reaches it.
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 failed"));
        env.send_subchannel_update(&scs[1], &SubchannelState::idle());

        // addr1 fails; the pass moves on and attempts addr2.
        env.send_subchannel_update(&scs[0], &SubchannelState::transient_failure("addr1 failed"));
        env.expect_connect_to("addr2");
        env.expect_no_events();

        // addr2's timer moves the pass on to addr3, which fails. Every address
        // has now failed once, but the pass is not over: addr2's earlier
        // failure does not count, and its attempt in this pass is in flight.
        env.fire_timer().await;
        env.expect_connect_to("addr3");
        env.send_subchannel_update(&scs[2], &SubchannelState::transient_failure("addr3 failed"));
        env.expect_no_events();

        // addr2 fails during this pass, completing it.
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 again"));
        env.expect_failing_picker("addr2 again");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // Timer work from an earlier first pass, delivered after a resolver update
    // started a new pass, does not move the new pass on.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_stale_timer_work_ignored_in_new_pass() {
        let mut env = new_env();
        env.simulate_connection(&["addr1", "addr2"]);

        // Hold the timer's work, as if the resolver update was handled before
        // it. The new pass reuses the subchannels and starts over at addr1.
        env.advance_past_timer().await;
        let timer_work = env.expect_schedule_work();
        env.send_resolver_update(create_endpoints(&["addr1", "addr2"]))
            .unwrap();
        env.expect_connect_to("addr1");
        env.expect_no_events();

        // The stale work does not move the new pass on to addr2.
        env.policy.work(timer_work, &mut env.tcc);
        env.expect_no_events();

        // The new pass's own timer moves it on to addr2.
        env.fire_timer().await;
        env.expect_connect_to("addr2");
        env.expect_no_events();
    }

    // If new_subchannel returns a subchannel that is already READY, the policy
    // selects it immediately, without asking any other subchannel to connect
    // (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_new_subchannel_initially_ready() {
        let mut env = new_env();
        env.send_resolver_update_with_initial_states(
            &["addr1", "addr2", "addr3"],
            &[("addr2", SubchannelState::ready())],
        )
        .unwrap();

        let scs = env.expect_new_subchannels(&["addr1", "addr2", "addr3"]);
        env.expect_ready_picker(&scs[1]);
        env.expect_no_events();
    }

    // If new_subchannel returns a subchannel already in TRANSIENT_FAILURE, the
    // first pass counts it as failed and skips to the next address (gRFC A61).
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_new_subchannel_initially_transient_failure() {
        let mut env = new_env();
        env.send_resolver_update_with_initial_states(
            &["addr1", "addr2"],
            &[(
                "addr1",
                SubchannelState::transient_failure("initial failure"),
            )],
        )
        .unwrap();

        let scs = env.expect_new_subchannels(&["addr1", "addr2"]);
        env.expect_connect_to("addr2");
        env.expect_queuing_picker();
        env.expect_no_events();

        // Once addr2 also fails, the pass finishes without waiting for addr1.
        env.send_subchannel_update(&scs[1], &SubchannelState::transient_failure("addr2 failed"));
        env.expect_failing_picker("addr2 failed");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // Freshest Error Caching (Steady State)
    // Ensures background failures during Steady State continuously overwrite
    // stale connection errors.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_steady_state_freshest_error() {
        let mut env = new_env();
        let scs = env.simulate_failed_connection(&["addr1"]);

        // Consume exhaustion events.
        env.expect_failing_picker("connection refused");
        env.expect_request_resolution();

        // Simulate background failure during Steady State with net-new error
        // telemetry. With one subchannel, every failure in steady state
        // reaches the failure count that triggers re-resolution.
        env.send_subchannel_update(
            &scs[0],
            &SubchannelState::transient_failure("steady state network drop"),
        );
        env.expect_request_resolution();

        // Verify policy caches the freshest error: a new pass over the same
        // address finds its subchannel in TransientFailure, and reports that
        // error.
        env.send_resolver_update(create_endpoints(&["addr1"]))
            .unwrap();
        env.expect_failing_picker("steady state network drop");
        env.expect_request_resolution();
        env.expect_no_events();
    }

    // Tests that when a selected subchannel disconnects (transitions to Idle),
    // the policy reports an Idle state and uses an IdlePicker.
    // When an RPC occurs, the IdlePicker schedules work, and the policy
    // reconnects when the work scheduler runs.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_disconnect_to_idle_and_reconnect() {
        let mut env = new_env();
        let scs = env.simulate_successful_connection(&["addr1", "addr2"]);

        // Simulate the subchannel disconnecting (transitioning to Idle).
        env.send_subchannel_update(&scs[0], &SubchannelState::idle());

        // Verify the policy updates the picker to Idle state.
        let idle_picker = env.expect_picker_state(ConnectivityState::Idle);

        // At this point, there should be no more events, as we are waiting for
        // an RPC.
        env.expect_no_events();

        // Simulate an RPC (pick) happening.
        let pick_result = idle_picker.pick(PickOptions::new(
            &test_utils::new_request_headers(),
            &CallOptions::new(),
        ));
        assert!(matches!(pick_result, PickResult::Queue));

        // The picker should schedule work. Run it to execute the scheduled
        // connection attempt.
        env.run_work();

        // Verify that the policy initiates a reconnection to addr1, and the
        // picker goes to Connecting.
        env.expect_connect_to("addr1");
        env.expect_queuing_picker();
        env.expect_no_events();
    }

    // Tests that exit_idle is a no-op before the first resolver update or
    // while Connecting or Ready.
    #[tokio::test(start_paused = true)]
    async fn test_pick_first_exit_idle() {
        let mut env = new_env();

        // Before any resolver update (Idle, but subchannels is empty),
        // exit_idle should be a no-op.
        env.policy.exit_idle(&mut env.tcc);
        env.expect_no_events();

        // While Connecting, exit_idle should be a no-op.
        let scs = env.simulate_connection(&["addr1"]);

        env.policy.exit_idle(&mut env.tcc);
        env.expect_no_events();

        // While Ready, exit_idle should be a no-op and keep the selected
        // subchannel active.
        env.send_subchannel_update(&scs[0], &SubchannelState::ready());
        env.expect_ready_picker(&scs[0]);

        env.policy.exit_idle(&mut env.tcc);
        env.expect_no_events();
    }
}
