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

//! ADS resource actor and physical server task.
//!
//! The worker coordinates resource subscriptions, cache state, watcher
//! delivery, and ACK/NACK construction. A separate server task owns the ADS
//! stream lifecycle, reconnect backoff, and wire I/O.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use crate::client::config::{ClientConfig, ServerConfig};
use crate::client::retry::Backoff;
use crate::client::watch::{ProcessingDone, ResourceEvent};
use crate::codec::XdsCodec;
use crate::error::{Error, Result};
use crate::message::{DiscoveryRequest, DiscoveryResponse, ErrorDetail, Node};
use crate::metrics::{self, KeyValue, MetricsRecorder};
use crate::resource::{DecodedResource, DecoderFn};
use crate::runtime::Runtime;
use crate::transport::{Transport, TransportBuilder, TransportStream};

const SERVER_EVENT_BUFFER_SIZE: usize = 64;
const SERVER_COMMAND_BUFFER_SIZE: usize = 64;
const DIRTY_RESEND_INTERVAL: Duration = Duration::from_millis(10);

/// Per-client A78 metric attributes (`grpc.target` + `grpc.xds.server`).
///
/// Both values are stored as `Arc<str>` so each emission clones them as a
/// cheap atomic op (via the `StringValue::RefCounted` variant) instead of
/// allocating a new `String` per attribute slot.
struct ClientAttrs {
    target: Arc<str>,
    server: Arc<str>,
}

impl ClientAttrs {
    /// Sentinel `grpc.xds.authority` value used for the unnamed top-level
    /// (non-federated) authority.
    ///
    /// Matches grpc-go's top-level placeholder.
    ///
    /// TODO: once federated bootstrap support lands, derive the authority from
    /// the resource name (`xdstp://<authority>/...`) on a per-resource basis.
    const TOP_LEVEL_AUTHORITY: &'static str = "#old";

    fn connection_attrs(&self) -> [KeyValue; 2] {
        [
            KeyValue::str(metrics::attrs::GRPC_TARGET, Arc::clone(&self.target)),
            KeyValue::str(metrics::attrs::GRPC_XDS_SERVER, Arc::clone(&self.server)),
        ]
    }

    fn type_attrs(&self, type_url: &Arc<str>) -> [KeyValue; 3] {
        [
            KeyValue::str(metrics::attrs::GRPC_TARGET, Arc::clone(&self.target)),
            KeyValue::str(metrics::attrs::GRPC_XDS_SERVER, Arc::clone(&self.server)),
            KeyValue::str(metrics::attrs::GRPC_XDS_RESOURCE_TYPE, Arc::clone(type_url)),
        ]
    }

    fn cache_state_attrs(&self, type_url: &Arc<str>, cache_state: &'static str) -> [KeyValue; 4] {
        [
            KeyValue::str(metrics::attrs::GRPC_TARGET, Arc::clone(&self.target)),
            KeyValue::str(
                metrics::attrs::GRPC_XDS_AUTHORITY,
                Self::TOP_LEVEL_AUTHORITY,
            ),
            KeyValue::str(metrics::attrs::GRPC_XDS_RESOURCE_TYPE, Arc::clone(type_url)),
            KeyValue::str(metrics::attrs::GRPC_XDS_CACHE_STATE, cache_state),
        ]
    }
}

/// Worker-side wrapper around an optional [`MetricsRecorder`] backend.
pub(crate) struct RecorderHandle {
    recorder: Option<Arc<dyn MetricsRecorder>>,
    attrs: ClientAttrs,
    /// Last-emitted `grpc.xds_client.resources` gauge value per
    /// `resource_type -> cache_state`. Used to diff against the live
    /// cache snapshot so we only push buckets whose count changed; the cache in
    /// the worker remains the single source of truth.
    resource_counts: HashMap<Arc<str>, HashMap<&'static str, i64>>,
}

impl RecorderHandle {
    pub(crate) fn new(recorder: Option<Arc<dyn MetricsRecorder>>, target: Arc<str>) -> Self {
        Self {
            recorder,
            attrs: ClientAttrs {
                target,
                server: Arc::from(""),
            },
            resource_counts: HashMap::new(),
        }
    }

    /// Update the `grpc.xds.server` attribute for subsequent emissions.
    pub(crate) fn set_server(&mut self, server: Arc<str>) {
        self.attrs.server = server;
    }

    /// `grpc.xds_client.connected` — 1 for connected, 0 for disconnected.
    fn record_connected(&self, connected: bool) {
        let Some(recorder) = &self.recorder else {
            return;
        };
        recorder.record_gauge_i64(
            &metrics::instruments::XDS_CLIENT_CONNECTED,
            if connected { 1 } else { 0 },
            &self.attrs.connection_attrs(),
        );
    }

    /// `grpc.xds_client.server_failure` — incremented once per failed connection cycle.
    fn record_server_failure(&self) {
        let Some(recorder) = &self.recorder else {
            return;
        };
        recorder.add_counter_u64(
            &metrics::instruments::XDS_CLIENT_SERVER_FAILURE,
            1,
            &self.attrs.connection_attrs(),
        );
    }

    /// `grpc.xds_client.resource_updates_valid` + `_invalid`, with aggregated
    /// counts from a single response.
    fn record_resource_updates(&self, type_url: &Arc<str>, valid: u64, invalid: u64) {
        let Some(recorder) = &self.recorder else {
            return;
        };
        if valid == 0 && invalid == 0 {
            return;
        }
        let type_attrs = self.attrs.type_attrs(type_url);
        if valid > 0 {
            recorder.add_counter_u64(
                &metrics::instruments::XDS_CLIENT_RESOURCE_UPDATES_VALID,
                valid,
                &type_attrs,
            );
        }
        if invalid > 0 {
            recorder.add_counter_u64(
                &metrics::instruments::XDS_CLIENT_RESOURCE_UPDATES_INVALID,
                invalid,
                &type_attrs,
            );
        }
    }

    /// Reconcile the `grpc.xds_client.resources` gauge for `type_url` against an
    /// authoritative cache snapshot (`cache_state` label -> current count).
    ///
    /// The worker's resource cache is the single source of truth; this only
    /// diffs the snapshot against the values last emitted for `type_url` and
    /// pushes the buckets that changed. Buckets that dropped out of the snapshot
    /// are pushed as `0`, because a push gauge would otherwise retain a stale
    /// non-zero reading for a bucket that has emptied. Idempotent: calling it
    /// with an unchanged snapshot emits nothing.
    fn sync_resource_counts(&mut self, type_url: &Arc<str>, counts: &HashMap<&'static str, i64>) {
        let Some(recorder) = &self.recorder else {
            return;
        };
        let last = self
            .resource_counts
            .entry(Arc::clone(type_url))
            .or_default();

        // New or changed buckets.
        for (&state, &count) in counts {
            if last.get(&state) != Some(&count) {
                recorder.record_gauge_i64(
                    &metrics::instruments::XDS_CLIENT_RESOURCES,
                    count,
                    &self.attrs.cache_state_attrs(type_url, state),
                );
            }
        }
        // Buckets that emptied since the last snapshot — reset to 0.
        for &state in last.keys() {
            if !counts.contains_key(&state) {
                recorder.record_gauge_i64(
                    &metrics::instruments::XDS_CLIENT_RESOURCES,
                    0,
                    &self.attrs.cache_state_attrs(type_url, state),
                );
            }
        }

        *last = counts.clone();
    }
}

/// Global counter for generating unique watcher IDs.
static NEXT_WATCHER_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identifier for a watcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WatcherId(u64);

impl WatcherId {
    /// Create a new unique watcher ID.
    pub fn new() -> Self {
        Self(NEXT_WATCHER_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for WatcherId {
    fn default() -> Self {
        Self::new()
    }
}

/// Commands sent from `XdsClient` to the worker.
pub(crate) enum WorkerCommand {
    /// Subscribe to a resource.
    Watch {
        /// The type URL of the resource.
        type_url: &'static str,
        /// The resource name (empty string for wildcard subscription).
        name: String,
        /// Unique identifier for this watcher.
        watcher_id: WatcherId,
        /// Channel to send resource events to the watcher.
        event_tx: mpsc::Sender<ResourceEvent<DecodedResource>>,
        /// Decoder function for this resource type.
        decoder: DecoderFn,
        /// Whether all resources must be present in SotW responses (per A53).
        all_resources_required_in_sotw: bool,
    },
    /// Unsubscribe a watcher.
    Unwatch {
        /// The watcher to remove.
        watcher_id: WatcherId,
    },
    /// Timer expired for a resource that was never received (gRFC A57).
    ResourceTimerExpired {
        /// The type URL of the resource.
        type_url: String,
        /// The resource name.
        name: String,
    },
}

/// Represents the subscription mode for a resource type.
///
/// This enum captures the mutually exclusive subscription states:
/// - Wildcard: receive all resources of this type
/// - Named: receive only specific resources by name
#[derive(Debug, Clone, PartialEq, Eq)]
enum SubscriptionMode {
    /// Wildcard subscription - receive all resources of this type.
    /// In xDS protocol, this is represented by an empty resource_names list.
    Wildcard,
    /// Named subscription - receive only specific resources.
    /// Contains the set of resource names to subscribe to.
    Named(HashSet<String>),
}

impl SubscriptionMode {
    /// Get resource names for DiscoveryRequest.
    /// Returns empty vec for wildcard (xDS spec: empty = all resources).
    fn resource_names_for_request(&self) -> Vec<String> {
        match self {
            Self::Wildcard => Vec::new(),
            Self::Named(names) => names.iter().cloned().collect(),
        }
    }
}

/// State of a cached resource per gRFC A88.
#[derive(Debug, Clone)]
enum ResourceState {
    /// Resource has been requested but not yet received.
    Requested,
    /// Resource has been successfully received and validated.
    Received,
    /// Resource validation failed. Contains the error message.
    NACKed(String),
    /// Resource does not exist (server indicated deletion or absence).
    DoesNotExist,
}

impl ResourceState {
    /// Canonical A78 `grpc.xds.cache_state` attribute value for this state.
    ///
    /// When gRFC A88 (data error caching) is implemented, a `NACKedButCached`
    /// variant will map to `"nacked_but_cached"` here.
    fn cache_state_label(&self) -> &'static str {
        match self {
            ResourceState::Requested => "requested",
            ResourceState::Received => "acked",
            ResourceState::NACKed(_) => "nacked",
            ResourceState::DoesNotExist => "does_not_exist",
        }
    }
}

/// A cached resource entry.
#[derive(Debug, Clone)]
struct CachedResource {
    /// Current state of the resource.
    state: ResourceState,
    /// The decoded resource, if successfully received.
    /// None if state is Requested, NACKed, or DoesNotExist.
    resource: Option<Arc<DecodedResource>>,
}

impl CachedResource {
    /// Create a new cached resource in Requested state.
    fn requested() -> Self {
        Self {
            state: ResourceState::Requested,
            resource: None,
        }
    }

    /// Create a cached resource in Received state.
    fn received(resource: Arc<DecodedResource>) -> Self {
        Self {
            state: ResourceState::Received,
            resource: Some(resource),
        }
    }

    /// Create a cached resource in DoesNotExist state.
    fn does_not_exist() -> Self {
        Self {
            state: ResourceState::DoesNotExist,
            resource: None,
        }
    }

    /// Create a cached resource in NACKed state.
    fn nacked(error: String) -> Self {
        Self {
            state: ResourceState::NACKed(error),
            resource: None,
        }
    }

    /// Returns true if the resource is in Requested state (waiting for server response).
    fn is_requested(&self) -> bool {
        matches!(self.state, ResourceState::Requested)
    }

    /// Convert cached state to a ResourceEvent for notifying watchers.
    /// Returns None if state is Requested (nothing to notify yet).
    fn to_event(&self) -> Option<ResourceEvent<DecodedResource>> {
        // Cache-dump events for new watchers do not gate flow control.
        let done = ProcessingDone::detached();
        match &self.state {
            ResourceState::Received => {
                self.resource
                    .as_ref()
                    .map(|r| ResourceEvent::ResourceChanged {
                        result: Ok(Arc::clone(r)),
                        done,
                    })
            }
            ResourceState::DoesNotExist => Some(ResourceEvent::ResourceChanged {
                result: Err(Error::ResourceDoesNotExist),
                done,
            }),
            ResourceState::NACKed(error) => Some(ResourceEvent::ResourceChanged {
                result: Err(Error::Validation(error.clone())),
                done,
            }),
            ResourceState::Requested => None,
        }
    }
}

/// Per-type_url state tracking.
struct TypeState {
    /// Reference-counted type URL, shared with metric attribute slots so
    /// per-emission attribute construction is a cheap.
    type_url: Arc<str>,
    /// Decoder function for this resource type.
    decoder: DecoderFn,
    /// Active watchers for this type.
    watchers: HashMap<WatcherId, WatcherEntry>,
    /// Current subscription mode (wildcard or named resources).
    subscription: SubscriptionMode,
    /// Resource cache: name -> cached resource.
    cache: HashMap<String, CachedResource>,
    /// Whether missing resources in SotW should be treated as deleted (per A53).
    all_resources_required_in_sotw: bool,
}

impl std::fmt::Debug for TypeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeState")
            .field("type_url", &self.type_url)
            .field("decoder", &"<decoder fn>")
            .field("watchers", &self.watchers)
            .field("subscription", &self.subscription)
            .field("cache", &format!("<{} entries>", self.cache.len()))
            .field(
                "all_resources_required_in_sotw",
                &self.all_resources_required_in_sotw,
            )
            .finish()
    }
}

impl TypeState {
    fn new(type_url: Arc<str>, decoder: DecoderFn, all_resources_required_in_sotw: bool) -> Self {
        Self {
            type_url,
            decoder,
            watchers: HashMap::new(),
            subscription: SubscriptionMode::Named(HashSet::new()),
            cache: HashMap::new(),
            all_resources_required_in_sotw,
        }
    }

    /// Recalculate subscription mode from watchers.
    fn recalculate_subscriptions(&mut self) {
        let has_wildcard = self
            .watchers
            .values()
            .any(|entry| entry.subscription.is_wildcard());

        if has_wildcard {
            self.subscription = SubscriptionMode::Wildcard;
        } else {
            let names: HashSet<String> = self
                .watchers
                .values()
                .filter_map(|entry| match &entry.subscription {
                    WatcherSubscription::Named(name) => Some(name.clone()),
                    WatcherSubscription::Wildcard => None,
                })
                .collect();
            self.subscription = SubscriptionMode::Named(names);
        }
    }

    /// Get resource names to send in DiscoveryRequest.
    fn resource_names_for_request(&self) -> Vec<String> {
        self.subscription.resource_names_for_request()
    }

    /// Get senders for all watchers interested in a specific resource.
    fn matching_watchers(&self, name: &str) -> Vec<mpsc::Sender<ResourceEvent<DecodedResource>>> {
        self.watchers
            .values()
            .filter(|e| e.subscription.matches(name))
            .map(|e| e.event_tx.clone())
            .collect()
    }

    /// Current number of cached resources in each `grpc.xds.cache_state`, keyed
    /// by the canonical state label. States with no resources are omitted; this
    /// is the authoritative snapshot for the `grpc.xds_client.resources` gauge.
    fn resource_state_counts(&self) -> HashMap<&'static str, i64> {
        let mut counts: HashMap<&'static str, i64> = HashMap::new();
        for cached in self.cache.values() {
            *counts.entry(cached.state.cache_state_label()).or_insert(0) += 1;
        }
        counts
    }
}

/// Protocol state scoped to the active physical ADS stream and resource type.
#[derive(Default)]
struct StreamState {
    /// Version from the last accepted response.
    version_info: String,
    /// Nonce from the last response on the current stream generation.
    nonce: String,
}

/// Specifies which resources a watcher is interested in.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WatcherSubscription {
    /// Wildcard subscription - receive all resources of this type.
    Wildcard,
    /// Named subscription - receive only the specified resource.
    Named(String),
}

impl WatcherSubscription {
    /// Create a subscription from a resource name.
    /// Empty string is treated as wildcard.
    fn from_name(name: String) -> Self {
        if name.is_empty() {
            Self::Wildcard
        } else {
            Self::Named(name)
        }
    }

    /// Check if this subscription matches a resource name.
    fn matches(&self, resource_name: &str) -> bool {
        match self {
            Self::Wildcard => true,
            Self::Named(name) => name == resource_name,
        }
    }

    /// Returns true if this is a wildcard subscription.
    fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }
}

/// Per-watcher state.
#[derive(Debug)]
struct WatcherEntry {
    /// Channel to send events to this watcher.
    event_tx: mpsc::Sender<ResourceEvent<DecodedResource>>,
    /// What resources this watcher is subscribed to.
    subscription: WatcherSubscription,
}

/// The ADS worker manages the xDS stream and dispatches resources to watchers.
pub(crate) struct AdsWorker<TB, C, R> {
    /// Transport builder for creating transports to xDS servers.
    transport_builder: Option<TB>,
    /// Codec for encoding/decoding messages.
    codec: C,
    /// Runtime for spawning tasks and sleeping.
    runtime: R,
    /// Node identification.
    node: Node,
    /// Retry policy copied into the single physical server task.
    retry_policy: crate::client::retry::RetryPolicy,
    /// Priority-ordered list of xDS servers.
    /// Index 0 has the highest priority.
    servers: Vec<Arc<ServerConfig>>,
    /// Timeout for initial resource response (gRFC A57). None = disabled.
    resource_initial_timeout: Option<Duration>,
    /// Sender for timer callback commands.
    command_tx: mpsc::Sender<WorkerCommand>,
    /// Receiver for commands from XdsClient.
    command_rx: mpsc::Receiver<WorkerCommand>,
    /// Per-type_url state.
    type_states: HashMap<String, TypeState>,
    /// Per-type protocol state for the single active physical ADS stream.
    stream_state: HashMap<String, StreamState>,
    /// Cancellation handles for resource timers (gRFC A57).
    /// Key is (type_url, resource_name). Dropping the sender cancels the timer.
    resource_timers: HashMap<(String, String), oneshot::Sender<()>>,
    /// Optional backend + per-client A78 metric attributes
    /// (`grpc.target` + `grpc.xds.server`).
    recorder: RecorderHandle,
    /// Subscription types whose latest snapshot could not be queued.
    dirty_types: HashSet<String>,
    /// Removed subscription types whose snapshot deletion could not be queued.
    removed_types: HashSet<String>,
}

/// A watcher notification staged during response handling: the channel to
/// deliver on and the event, carrying its `ProcessingDone` token. Events
/// that participate in ADS flow control share the response's single
/// `ProcessingDone` signal; the others carry a detached token.
type Delivery = (
    mpsc::Sender<ResourceEvent<DecodedResource>>,
    ResourceEvent<DecodedResource>,
);

/// In-flight watcher deliveries for one response: sends every staged event
/// (with backpressure) and resolves once all watchers signal `ProcessingDone`.
/// While unresolved it gates reading the *next* response, but nothing else —
/// the worker continues draining commands while it is pending.
type PendingDispatch = Pin<Box<dyn Future<Output = ()> + Send>>;

enum ServerEvent {
    Connected { generation: u64 },
    Response { generation: u64, bytes: Bytes },
    Closed { generation: u64, saw_response: bool },
    Stopped,
}

enum ServerCommand {
    Send {
        type_url: String,
        bytes: Bytes,
    },
    Remove {
        type_url: String,
    },
    SendAck {
        generation: u64,
        bytes: Bytes,
        reconnect_request: (String, Bytes),
    },
    Resume {
        generation: u64,
    },
    Close {
        generation: u64,
    },
}

struct ServerHandle {
    command_tx: mpsc::Sender<ServerCommand>,
}

impl<TB, C, R> AdsWorker<TB, C, R>
where
    TB: TransportBuilder,
    C: XdsCodec,
    R: Runtime,
{
    /// Create a new worker.
    pub(crate) fn new(
        transport_builder: TB,
        codec: C,
        runtime: R,
        config: ClientConfig,
        command_tx: mpsc::Sender<WorkerCommand>,
        command_rx: mpsc::Receiver<WorkerCommand>,
        recorder: Option<Arc<dyn MetricsRecorder>>,
    ) -> Self {
        let target: Arc<str> = Arc::from(config.target.unwrap_or_default());
        Self {
            transport_builder: Some(transport_builder),
            codec,
            runtime,
            node: config.node,
            retry_policy: config.retry_policy,
            servers: config.servers.into_iter().map(Arc::new).collect(),
            resource_initial_timeout: config.resource_initial_timeout,
            command_tx,
            command_rx,
            type_states: HashMap::new(),
            stream_state: HashMap::new(),
            resource_timers: HashMap::new(),
            recorder: RecorderHandle::new(recorder, target),
            dirty_types: HashSet::new(),
            removed_types: HashSet::new(),
        }
    }

    /// Run the worker event loop.
    ///
    /// This method runs until all `XdsClient` handles are dropped
    /// (which closes the command channel).
    pub(crate) async fn run(mut self) {
        // gRFC A78 defines `grpc.xds_client.server_failure` as a count of xDS
        // servers *going from healthy to unhealthy*. `healthy` mirrors the
        // `connected` gauge so the counter (and gauge) are recorded only on that
        // transition.
        while self.type_states.is_empty() {
            match self.command_rx.recv().await {
                Some(cmd) => {
                    let _ = self.handle_command(None, cmd).await;
                }
                None => return,
            }
        }

        let server = match self.servers.first() {
            Some(server) => Arc::clone(server),
            None => return,
        };
        self.recorder.set_server(Arc::from(server.uri()));
        let (event_tx, mut event_rx) = mpsc::channel(SERVER_EVENT_BUFFER_SIZE);
        let (command_tx, command_rx) = mpsc::channel(SERVER_COMMAND_BUFFER_SIZE);
        let handle = ServerHandle { command_tx };
        let request_snapshots = self.build_initial_requests().into_iter().collect();
        let task = ServerTask {
            transport_builder: self
                .transport_builder
                .take()
                .expect("server task already started"),
            runtime: self.runtime.clone(),
            server,
            command_rx,
            event_tx,
            backoff: Backoff::new(self.retry_policy.clone()),
            request_snapshots,
        };
        self.runtime.spawn(task.run());

        let mut healthy = false;
        let mut pending: Option<(u64, PendingDispatch)> = None;
        loop {
            self.flush_dirty(&handle);
            tokio::select! {
                event = event_rx.recv() => match event {
                    Some(ServerEvent::Connected { generation }) => {
                        if !healthy {
                            self.recorder.record_connected(true);
                            healthy = true;
                        }
                        // A reconnect supersedes any response completion from an
                        // older stream generation.
                        if pending.as_ref().is_some_and(|(g, _)| *g != generation) {
                            pending = None;
                        }
                    }
                    Some(ServerEvent::Response { generation, bytes }) => {
                        match self.handle_response(&handle, generation, bytes).await {
                            Ok(dispatch) => {
                                if let Some(dispatch) = dispatch {
                                    pending = Some((generation, dispatch));
                                } else {
                                    let _ = handle.command_tx.send(ServerCommand::Resume { generation }).await;
                                }
                            }
                            Err(_) => {
                                let _ = handle.command_tx.send(ServerCommand::Close { generation }).await;
                            }
                        }
                    }
                    Some(ServerEvent::Closed { generation, saw_response }) => {
                        if pending.as_ref().is_some_and(|(g, _)| *g == generation) {
                            pending = None;
                        }
                        for state in self.stream_state.values_mut() {
                            state.nonce.clear();
                        }
                        if !saw_response {
                            self.record_unhealthy(&mut healthy);
                        }
                    }
                    Some(ServerEvent::Stopped) | None => return,
                },
                _ = async { pending.as_mut().unwrap().1.as_mut().await }, if pending.is_some() => {
                    let (generation, _) = pending.take().expect("pending dispatch disappeared");
                    let _ = handle.command_tx.send(ServerCommand::Resume { generation }).await;
                }
                cmd = self.command_rx.recv() => match cmd {
                    Some(cmd) => { let _ = self.handle_command(Some(&handle), cmd).await; }
                    None => return,
                },
                _ = self.runtime.sleep(DIRTY_RESEND_INTERVAL),
                    if !self.dirty_types.is_empty() || !self.removed_types.is_empty() => {}
            }
        }
    }

    /// Record an xDS server transition to unhealthy (gRFC A78
    /// `grpc.xds_client.server_failure`). Increments the `server_failure`
    /// counter and drops the `connected` gauge to 0, but only on the
    /// healthy -> unhealthy edge, so repeated reconnect attempts during a single
    /// outage are not counted.
    fn record_unhealthy(&self, healthy: &mut bool) {
        if *healthy {
            self.recorder.record_server_failure();
            self.recorder.record_connected(false);
            *healthy = false;
        }
    }

    /// Build initial DiscoveryRequests for all active subscriptions.
    ///
    /// These are sent when establishing the stream to prevent deadlock with
    /// servers that don't send response headers until they receive a request.
    fn build_initial_requests(&self) -> Vec<(String, Bytes)> {
        let mut requests = Vec::new();

        for (type_url, type_state) in &self.type_states {
            if type_state.watchers.is_empty() {
                continue;
            }

            let resource_names = type_state.resource_names_for_request();
            let version_info = self
                .stream_state
                .get(type_url)
                .map(|state| state.version_info.as_str())
                .unwrap_or_default();

            let request = DiscoveryRequest {
                node: &self.node,
                type_url,
                resource_names: &resource_names,
                version_info,
                response_nonce: "", // Initial request has empty nonce
                error_detail: None,
            };

            if let Ok(bytes) = self.codec.encode_request(&request) {
                requests.push((type_url.clone(), bytes));
            }
        }

        requests
    }

    /// Build the in-flight delivery future for one response's staged watcher
    /// notifications, or `None` when there is nothing to deliver.
    fn dispatch_pending(
        deliveries: Vec<Delivery>,
        done_rx: oneshot::Receiver<()>,
    ) -> Option<PendingDispatch> {
        if deliveries.is_empty() {
            return None;
        }
        Some(Box::pin(async move {
            for (event_tx, event) in deliveries {
                // Backpressure: await if the watcher's channel is full. Send
                // errors (watcher dropped) are ignored; the rejected event's
                // `ProcessingDone` token drops with it.
                let _ = event_tx.send(event).await;
            }
            // Resolves once every token sharing this response's signal drops.
            let _ = done_rx.await;
        }))
    }

    /// Handle a command, optionally sending network requests if connected.
    ///
    /// Before the server task starts, only state updates are performed. Once a
    /// handle is present, subscription changes queue the latest request.
    async fn handle_command(
        &mut self,
        server: Option<&ServerHandle>,
        cmd: WorkerCommand,
    ) -> Result<()> {
        match cmd {
            WorkerCommand::Watch {
                type_url,
                name,
                watcher_id,
                event_tx,
                decoder,
                all_resources_required_in_sotw,
            } => {
                if self.add_watcher(
                    type_url,
                    name,
                    watcher_id,
                    event_tx,
                    decoder,
                    all_resources_required_in_sotw,
                ) && let Some(server) = server
                {
                    self.send_request(server, type_url)?;
                }
            }
            WorkerCommand::Unwatch { watcher_id } => {
                if let Some((type_url, true)) = self.remove_watcher(watcher_id)
                    && let Some(server) = server
                {
                    if self.type_states.contains_key(&type_url) {
                        self.send_request(server, &type_url)?;
                    } else {
                        self.queue_remove(server, type_url);
                    }
                }
            }
            WorkerCommand::ResourceTimerExpired { type_url, name } => {
                self.handle_resource_timeout(&type_url, &name).await;
            }
        }
        Ok(())
    }

    /// Add a watcher to the state.
    ///
    /// If the resource is already cached, the watcher receives the cached state immediately.
    /// Returns true if subscriptions changed (need to send new request to server).
    fn add_watcher(
        &mut self,
        type_url: &'static str,
        name: String,
        watcher_id: WatcherId,
        event_tx: mpsc::Sender<ResourceEvent<DecodedResource>>,
        decoder: DecoderFn,
        all_resources_required_in_sotw: bool,
    ) -> bool {
        let type_url_string = type_url.to_string();
        let type_state = self
            .type_states
            .entry(type_url_string.clone())
            .or_insert_with(|| {
                TypeState::new(Arc::from(type_url), decoder, all_resources_required_in_sotw)
            });

        let old_subscription = type_state.subscription.clone();
        let watcher_subscription = WatcherSubscription::from_name(name.clone());

        // Track if we need to start a timer (resource in Requested state)
        let mut start_timer_for: Option<String> = None;
        // Track newly-inserted cache entry for the resources gauge (None -> Requested).
        let mut was_new = false;

        // For named subscriptions, check cache and send cached state to new watcher.
        // For wildcard subscriptions, watchers receive updates as they come in.
        if let WatcherSubscription::Named(ref resource_name) = watcher_subscription {
            let cached = match type_state.cache.entry(resource_name.clone()) {
                Entry::Vacant(v) => {
                    was_new = true;
                    v.insert(CachedResource::requested())
                }
                Entry::Occupied(o) => o.into_mut(),
            };

            if let Some(event) = cached.to_event() {
                // Send cached state to watcher (non-blocking, ignore if full)
                let _ = event_tx.try_send(event);
            }

            if cached.is_requested() {
                // Resource pending - start a timer (gRFC A57)
                start_timer_for = Some(resource_name.clone());
            }
        }

        type_state.watchers.insert(
            watcher_id,
            WatcherEntry {
                event_tx,
                subscription: watcher_subscription,
            },
        );
        type_state.recalculate_subscriptions();

        let subscriptions_changed = type_state.subscription != old_subscription;

        // Reconcile the resources gauge from the updated cache.
        if was_new {
            let counts = type_state.resource_state_counts();
            self.recorder
                .sync_resource_counts(&type_state.type_url, &counts);
        }

        // Start timer if resource is in Requested state
        if let (Some(resource_name), Some(timeout)) =
            (start_timer_for, self.resource_initial_timeout)
        {
            self.start_resource_timer(&type_url_string, resource_name, timeout);
        }

        subscriptions_changed
    }

    /// Remove a watcher from the state.
    /// Returns the type_url and whether subscriptions changed.
    fn remove_watcher(&mut self, watcher_id: WatcherId) -> Option<(String, bool)> {
        let type_url = self
            .type_states
            .iter()
            .find(|(_, state)| state.watchers.contains_key(&watcher_id))
            .map(|(url, _)| url.clone())?;

        let type_state = self.type_states.get_mut(&type_url)?;

        let old_subscription = type_state.subscription.clone();

        type_state.watchers.remove(&watcher_id);
        type_state.recalculate_subscriptions();

        let subscriptions_changed = type_state.subscription != old_subscription;

        if type_state.watchers.is_empty() {
            let type_url_arc = Arc::clone(&type_state.type_url);
            self.type_states.remove(&type_url);
            self.stream_state.remove(&type_url);
            // The type is gone — reset all of its resource buckets to zero.
            self.recorder
                .sync_resource_counts(&type_url_arc, &HashMap::new());
            // Cancel all pending resource timers for this type.
            self.resource_timers.retain(|key, _| key.0 != type_url);
        }

        Some((type_url, subscriptions_changed))
    }

    fn flush_dirty(&mut self, server: &ServerHandle) {
        for type_url in self.removed_types.clone() {
            self.queue_remove(server, type_url);
        }
        for type_url in self.dirty_types.clone() {
            let _ = self.send_request(server, &type_url);
        }
    }

    fn queue_remove(&mut self, server: &ServerHandle, type_url: String) {
        self.dirty_types.remove(&type_url);
        match server.command_tx.try_send(ServerCommand::Remove {
            type_url: type_url.clone(),
        }) {
            Ok(()) => {
                self.removed_types.remove(&type_url);
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                self.removed_types.insert(type_url);
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {}
        }
    }

    /// Encode and queue the latest DiscoveryRequest snapshot for a type.
    fn send_request(&mut self, server: &ServerHandle, type_url: &str) -> Result<()> {
        self.removed_types.remove(type_url);
        let type_state = match self.type_states.get(type_url) {
            Some(s) => s,
            None => {
                self.dirty_types.remove(type_url);
                return Ok(());
            }
        };

        let resource_names = type_state.resource_names_for_request();
        let (version_info, nonce) = self
            .stream_state
            .get(type_url)
            .map(|state| (state.version_info.as_str(), state.nonce.as_str()))
            .unwrap_or_default();
        let request = DiscoveryRequest {
            node: &self.node,
            type_url,
            resource_names: &resource_names,
            version_info,
            response_nonce: nonce,
            error_detail: None,
        };

        let bytes = self.codec.encode_request(&request)?;
        match server.command_tx.try_send(ServerCommand::Send {
            type_url: type_url.to_string(),
            bytes,
        }) {
            Ok(()) => {
                self.dirty_types.remove(type_url);
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                self.dirty_types.insert(type_url.to_string());
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {}
        }
        Ok(())
    }

    /// Handle a response from the server.
    ///
    /// Implements partial success per gRFC A46: valid resources are accepted even
    /// if some resources in the response fail validation. Each resource is processed
    /// independently:
    /// - Valid resources are cached and dispatched to watchers
    /// - Invalid resources are cached as NACKed and errors sent to specific watchers
    /// - Missing resources (for types with ALL_RESOURCES_REQUIRED_IN_SOTW) are marked deleted
    ///
    /// Cache/state updates and the ACK/NACK happen here; the watcher
    /// notifications are only *staged* and returned as a [`PendingDispatch`]
    /// future (`None` when there is nothing to deliver) for `run_connected`
    /// to drive, so a slow (or stuck) watcher delays reading the next
    /// response — ADS flow control — without freezing command processing.
    async fn handle_response(
        &mut self,
        server: &ServerHandle,
        generation: u64,
        bytes: Bytes,
    ) -> Result<Option<PendingDispatch>> {
        let response = self.codec.decode_response(bytes)?;
        let type_url = response.type_url.clone();

        let (type_url_arc, decoder) = match self.type_states.get(&type_url) {
            Some(s) => (Arc::clone(&s.type_url), &s.decoder),
            None => {
                return Ok(None);
            }
        };

        // One shared `ProcessingDone` signal for the whole response: every
        // flow-control event carries a share of it, and the receiver resolves
        // when the last share (including the original, dropped on return
        // from this function) is gone.
        let (done, done_rx) = ProcessingDone::channel();

        // Decode all resources, tracking valid and invalid separately.
        // Per A46, we accept valid resources even if some fail validation.
        // Per A88, we categorize errors:
        // - top_level_errors: deserialization failures where name cannot be extracted
        // - per_resource_errors: validation failures where name is known
        let mut valid_resources: Vec<DecodedResource> = Vec::new();
        let mut top_level_errors: Vec<String> = Vec::new();
        let mut per_resource_errors: Vec<(String, String)> = Vec::new(); // (name, error)

        for resource_any in &response.resources {
            match decoder(resource_any.value.clone()) {
                crate::resource::DecodeResult::Success { resource, .. } => {
                    valid_resources.push(resource);
                }
                crate::resource::DecodeResult::ResourceError { name, error } => {
                    per_resource_errors.push((name, error.to_string()));
                }
                crate::resource::DecodeResult::TopLevelError(error) => {
                    top_level_errors.push(error.to_string());
                }
            }
        }

        // Emit A78 resource_updates_valid/invalid counters once per response with
        // aggregated counts (equivalent to per-resource increments in any backend).
        let valid_count = valid_resources.len() as u64;
        let invalid_count = (top_level_errors.len() + per_resource_errors.len()) as u64;
        self.recorder
            .record_resource_updates(&type_url_arc, valid_count, invalid_count);

        self.stream_state.entry(type_url.clone()).or_default().nonce = response.nonce.clone();

        let received_names: HashSet<String> = valid_resources
            .iter()
            .map(|r| r.name().to_string())
            .collect();

        // Stage watcher notifications instead of sending them here: the sends
        // (and the ProcessingDone waits) happen in the returned deliveries,
        // driven by `run_connected` concurrently with command processing.
        // State/cache updates still happen synchronously below, so the ACK
        // reflects the accepted config regardless of watcher progress.
        let mut deliveries = Vec::new();

        self.dispatch_resources(&mut deliveries, &type_url, valid_resources, &done);

        // Only notify watchers for per-resource errors (where we know the name).
        // Top-level errors have no associated name, so no watcher to notify.
        for (resource_name, error) in &per_resource_errors {
            self.notify_resource_error(&mut deliveries, &type_url, resource_name, error);
        }

        // Detect deleted resources (per A53):
        // For resource types with ALL_RESOURCES_REQUIRED_IN_SOTW = true,
        // any previously-received resource not in this response is deleted.
        self.detect_deleted_resources(&mut deliveries, &type_url, &received_names, &done);

        let has_errors = !top_level_errors.is_empty() || !per_resource_errors.is_empty();
        if !has_errors {
            // Only update version on ACK; NACK must keep the old version so the
            // server knows which version the client is still running.
            self.stream_state
                .entry(type_url.clone())
                .or_default()
                .version_info = response.version_info.clone();
            self.send_ack(server, generation, &response).await?;
        } else {
            // Build NACK message combining both error categories
            let mut error_parts = Vec::new();

            if !top_level_errors.is_empty() {
                error_parts.push(format!("top level errors: {}", top_level_errors.join("; ")));
            }

            if !per_resource_errors.is_empty() {
                let per_resource_msg = per_resource_errors
                    .iter()
                    .map(|(name, err)| format!("{name}: {err}"))
                    .collect::<Vec<_>>()
                    .join("; ");
                error_parts.push(per_resource_msg);
            }

            self.send_nack(server, generation, &response, error_parts.join("; "))
                .await?;
        }

        Ok(Self::dispatch_pending(deliveries, done_rx))
    }

    /// Update the cache from decoded resources and stage watcher deliveries.
    ///
    /// The staged events share the response's `ProcessingDone` signal, which
    /// gates reading the next response (ADS flow control); the sends
    /// themselves happen in the [`PendingDispatch`] future, with backpressure
    /// on full channels.
    fn dispatch_resources(
        &mut self,
        deliveries: &mut Vec<Delivery>,
        type_url: &str,
        resources: Vec<DecodedResource>,
        done: &ProcessingDone,
    ) {
        let watcher_info: Vec<_> = match self.type_states.get_mut(type_url) {
            Some(s) => {
                for resource in &resources {
                    let resource_name = resource.name().to_string();
                    s.cache.insert(
                        resource_name,
                        CachedResource::received(Arc::new(resource.clone())),
                    );
                }
                let counts = s.resource_state_counts();
                self.recorder.sync_resource_counts(&s.type_url, &counts);
                s.watchers
                    .iter()
                    .map(|(id, entry)| (*id, entry.event_tx.clone(), entry.subscription.clone()))
                    .collect()
            }
            None => return,
        };

        // Cancel resource timers for received resources (gRFC A57).
        for resource in &resources {
            self.resource_timers
                .remove(&(type_url.to_string(), resource.name().to_string()));
        }

        for resource in resources {
            let resource_name = resource.name().to_string();
            let resource = Arc::new(resource);

            for (_watcher_id, event_tx, subscription) in &watcher_info {
                if subscription.matches(&resource_name) {
                    let event = ResourceEvent::ResourceChanged {
                        result: Ok(Arc::clone(&resource)),
                        done: done.share(),
                    };
                    deliveries.push((event_tx.clone(), event));
                }
            }
        }
    }

    /// Stage validation-error notifications for a specific resource.
    ///
    /// Per gRFC A46/A88, errors are routed only to watchers interested in
    /// that specific resource (plus wildcard watchers). Error events do not
    /// gate flow control (they carry a detached `ProcessingDone` token).
    fn notify_resource_error(
        &mut self,
        deliveries: &mut Vec<Delivery>,
        type_url: &str,
        resource_name: &str,
        error: &str,
    ) {
        let type_state = match self.type_states.get_mut(type_url) {
            Some(s) => s,
            None => return,
        };

        type_state.cache.insert(
            resource_name.to_string(),
            CachedResource::nacked(error.to_string()),
        );
        let counts = type_state.resource_state_counts();
        self.recorder
            .sync_resource_counts(&type_state.type_url, &counts);

        // Cancel the resource timer (gRFC A57).
        self.resource_timers
            .remove(&(type_url.to_string(), resource_name.to_string()));

        for event_tx in type_state.matching_watchers(resource_name) {
            let event = ResourceEvent::ResourceChanged {
                result: Err(Error::Validation(error.to_string())),
                done: ProcessingDone::detached(),
            };
            deliveries.push((event_tx, event));
        }
    }

    /// Detect resources that were deleted (present in cache but not in response)
    /// and stage the deletion notifications.
    ///
    /// Per gRFC A53, for resource types with ALL_RESOURCES_REQUIRED_IN_SOTW = true,
    /// if a previously-received resource is absent from a new SotW response,
    /// it is treated as deleted.
    fn detect_deleted_resources(
        &mut self,
        deliveries: &mut Vec<Delivery>,
        type_url: &str,
        received_names: &HashSet<String>,
        done: &ProcessingDone,
    ) {
        let type_state = match self.type_states.get_mut(type_url) {
            Some(s) => s,
            None => return,
        };

        if !type_state.all_resources_required_in_sotw {
            return;
        }

        let deleted_names: Vec<String> = type_state
            .cache
            .iter()
            .filter(|(name, cached)| {
                matches!(cached.state, ResourceState::Received) && !received_names.contains(*name)
            })
            .map(|(name, _)| name.clone())
            .collect();

        for name in deleted_names {
            type_state
                .cache
                .insert(name.clone(), CachedResource::does_not_exist());

            for event_tx in type_state.matching_watchers(&name) {
                let event = ResourceEvent::ResourceChanged {
                    result: Err(Error::ResourceDoesNotExist),
                    done: done.share(),
                };
                deliveries.push((event_tx, event));
            }
        }

        // Reconcile the resources gauge once from the updated cache.
        let counts = type_state.resource_state_counts();
        self.recorder
            .sync_resource_counts(&type_state.type_url, &counts);
    }

    /// Send an ACK for a response.
    async fn send_ack(
        &self,
        server: &ServerHandle,
        generation: u64,
        response: &DiscoveryResponse,
    ) -> Result<()> {
        let type_state = match self.type_states.get(&response.type_url) {
            Some(s) => s,
            None => return Ok(()),
        };

        let resource_names = type_state.resource_names_for_request();
        let request = DiscoveryRequest {
            node: &self.node,
            type_url: &response.type_url,
            resource_names: &resource_names,
            version_info: &response.version_info,
            response_nonce: &response.nonce,
            error_detail: None,
        };

        let bytes = self.codec.encode_request(&request)?;
        let reconnect_request = DiscoveryRequest {
            node: &self.node,
            type_url: &response.type_url,
            resource_names: &resource_names,
            version_info: &response.version_info,
            response_nonce: "",
            error_detail: None,
        };
        let reconnect_bytes = self.codec.encode_request(&reconnect_request)?;
        server
            .command_tx
            .send(ServerCommand::SendAck {
                generation,
                bytes,
                reconnect_request: (response.type_url.clone(), reconnect_bytes),
            })
            .await
            .map_err(|_| Error::Connection("server task closed".into()))
    }

    /// Send a NACK for a response.
    async fn send_nack(
        &self,
        server: &ServerHandle,
        generation: u64,
        response: &DiscoveryResponse,
        error_message: String,
    ) -> Result<()> {
        let type_state = match self.type_states.get(&response.type_url) {
            Some(s) => s,
            None => return Ok(()),
        };

        let resource_names = type_state.resource_names_for_request();
        let version_info = self
            .stream_state
            .get(&response.type_url)
            .map(|state| state.version_info.as_str())
            .unwrap_or_default();
        let request = DiscoveryRequest {
            node: &self.node,
            type_url: &response.type_url,
            resource_names: &resource_names,
            version_info, // Keep old version for NACK
            response_nonce: &response.nonce,
            error_detail: Some(ErrorDetail {
                code: 3, // INVALID_ARGUMENT
                message: error_message,
            }),
        };

        let bytes = self.codec.encode_request(&request)?;
        let reconnect_request = DiscoveryRequest {
            node: &self.node,
            type_url: &response.type_url,
            resource_names: &resource_names,
            version_info,
            response_nonce: "",
            error_detail: None,
        };
        let reconnect_bytes = self.codec.encode_request(&reconnect_request)?;
        server
            .command_tx
            .send(ServerCommand::SendAck {
                generation,
                bytes,
                reconnect_request: (response.type_url.clone(), reconnect_bytes),
            })
            .await
            .map_err(|_| Error::Connection("server task closed".into()))
    }

    /// Start a timer for a resource in Requested state (gRFC A57).
    ///
    /// If a timer is already running for this resource, this is a no-op to
    /// preserve the original timeout deadline per A57.
    ///
    /// When the timer fires, it sends a `ResourceTimerExpired` command.
    /// The handler checks if the resource is still in Requested state before acting.
    fn start_resource_timer(&mut self, type_url: &str, name: String, timeout: Duration) {
        let key = (type_url.to_string(), name.clone());

        // Don't reset an existing timer — A57 says timeout starts on first request.
        if self.resource_timers.contains_key(&key) {
            return;
        }

        let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
        let type_url_owned = type_url.to_string();
        let command_tx = self.command_tx.clone();
        let runtime = self.runtime.clone();

        self.runtime.spawn(async move {
            tokio::select! {
                _ = runtime.sleep(timeout) => {
                    let _ = command_tx.send(WorkerCommand::ResourceTimerExpired {
                        type_url: type_url_owned,
                        name,
                    }).await;
                }
                _ = cancel_rx => {}
            }
        });

        self.resource_timers.insert(key, cancel_tx);
    }

    /// Handle a resource timer expiration (gRFC A57).
    ///
    /// If the resource is still in Requested state, marks it as DoesNotExist
    /// and notifies all watchers interested in this resource.
    async fn handle_resource_timeout(&mut self, type_url: &str, name: &str) {
        self.resource_timers
            .remove(&(type_url.to_string(), name.to_string()));

        let type_state = match self.type_states.get_mut(type_url) {
            Some(s) => s,
            None => return,
        };

        let is_pending = type_state
            .cache
            .get(name)
            .map(|c| c.is_requested())
            .unwrap_or(true);

        if !is_pending {
            return;
        }

        type_state
            .cache
            .insert(name.to_string(), CachedResource::does_not_exist());
        let counts = type_state.resource_state_counts();
        self.recorder
            .sync_resource_counts(&type_state.type_url, &counts);

        for event_tx in type_state.matching_watchers(name) {
            let event = ResourceEvent::ResourceChanged {
                result: Err(Error::ResourceDoesNotExist),
                done: ProcessingDone::detached(),
            };
            let _ = event_tx.send(event).await;
        }
    }
}

/// Owns the lifecycle and wire I/O for the one active physical ADS server.
///
/// The resource actor above owns watches and cache transitions; this task owns
/// transport construction, reconnect backoff, stream generations, and the
/// current request snapshots needed to open a replacement stream.
struct ServerTask<TB, R> {
    transport_builder: TB,
    runtime: R,
    server: Arc<ServerConfig>,
    command_rx: mpsc::Receiver<ServerCommand>,
    event_tx: mpsc::Sender<ServerEvent>,
    backoff: Backoff,
    request_snapshots: HashMap<String, Bytes>,
}

enum BackoffOutcome {
    Retry,
    Exhausted,
    Shutdown,
}

impl<TB, R> ServerTask<TB, R>
where
    TB: TransportBuilder,
    R: Runtime,
{
    async fn run(mut self) {
        let mut generation = 0_u64;
        loop {
            let transport = match self.transport_builder.build(&self.server).await {
                Ok(transport) => transport,
                Err(_) => {
                    if !self.closed(generation, false).await {
                        return;
                    }
                    match self.backoff().await {
                        BackoffOutcome::Retry => continue,
                        BackoffOutcome::Exhausted => {
                            let _ = self.event_tx.send(ServerEvent::Stopped).await;
                            return;
                        }
                        BackoffOutcome::Shutdown => return,
                    }
                }
            };

            let initial_requests = self.request_snapshots.values().cloned().collect();
            let mut stream = match transport.new_stream(initial_requests).await {
                Ok(stream) => stream,
                Err(_) => {
                    if !self.closed(generation, false).await {
                        return;
                    }
                    match self.backoff().await {
                        BackoffOutcome::Retry => continue,
                        BackoffOutcome::Exhausted => {
                            let _ = self.event_tx.send(ServerEvent::Stopped).await;
                            return;
                        }
                        BackoffOutcome::Shutdown => return,
                    }
                }
            };

            generation = generation.wrapping_add(1);
            self.backoff.reset();
            if self
                .event_tx
                .send(ServerEvent::Connected { generation })
                .await
                .is_err()
            {
                return;
            }

            let mut saw_response = false;
            let mut response_in_flight = false;
            'connected: loop {
                tokio::select! {
                    command = self.command_rx.recv() => match command {
                        Some(ServerCommand::Send { type_url, bytes }) => {
                            self.request_snapshots.insert(type_url, bytes.clone());
                            if stream.send(bytes).await.is_err() {
                                break 'connected;
                            }
                        }
                        Some(ServerCommand::Remove { type_url }) => {
                            self.request_snapshots.remove(&type_url);
                        }
                        Some(ServerCommand::SendAck {
                            generation: ack_generation,
                            bytes,
                            reconnect_request,
                        }) => {
                            if ack_generation == generation {
                                self.request_snapshots.insert(
                                    reconnect_request.0,
                                    reconnect_request.1,
                                );
                                if stream.send(bytes).await.is_err() {
                                    break 'connected;
                                }
                            }
                        }
                        Some(ServerCommand::Resume { generation: resume_generation }) => {
                            if resume_generation == generation {
                                response_in_flight = false;
                            }
                        }
                        Some(ServerCommand::Close { generation: close_generation }) => {
                            if close_generation == generation {
                                break 'connected;
                            }
                        }
                        None => return,
                    },
                    result = stream.recv(), if !response_in_flight => match result {
                        Ok(Some(bytes)) => {
                            saw_response = true;
                            response_in_flight = true;
                            if self.event_tx.send(ServerEvent::Response {
                                generation,
                                bytes,
                            }).await.is_err() {
                                return;
                            }
                        }
                        Ok(None) | Err(_) => break 'connected,
                    }
                }
            }

            if !self.closed(generation, saw_response).await {
                return;
            }
            match self.backoff().await {
                BackoffOutcome::Retry => {}
                BackoffOutcome::Exhausted => {
                    let _ = self.event_tx.send(ServerEvent::Stopped).await;
                    return;
                }
                BackoffOutcome::Shutdown => return,
            }
        }
    }

    async fn closed(&self, generation: u64, saw_response: bool) -> bool {
        self.event_tx
            .send(ServerEvent::Closed {
                generation,
                saw_response,
            })
            .await
            .is_ok()
    }

    async fn backoff(&mut self) -> BackoffOutcome {
        let Some(duration) = self.backoff.next_backoff() else {
            return BackoffOutcome::Exhausted;
        };
        let sleep = self.runtime.sleep(duration);
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                _ = &mut sleep => return BackoffOutcome::Retry,
                command = self.command_rx.recv() => match command {
                    Some(ServerCommand::Send { type_url, bytes }) => {
                        self.request_snapshots.insert(type_url, bytes);
                    }
                    Some(ServerCommand::Remove { type_url }) => {
                        self.request_snapshots.remove(&type_url);
                    }
                    Some(ServerCommand::SendAck { reconnect_request, .. }) => {
                        self.request_snapshots.insert(reconnect_request.0, reconnect_request.1);
                    }
                    Some(ServerCommand::Resume { .. })
                    | Some(ServerCommand::Close { .. }) => {}
                    None => return BackoffOutcome::Shutdown,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// Captures every measurement so tests can assert on the call sequence.
    #[derive(Default)]
    struct CapturingRecorder {
        events: Mutex<Vec<Recorded>>,
    }

    #[derive(Debug, PartialEq)]
    struct Recorded {
        instrument: &'static str,
        kind: Measurement,
        attrs: Vec<(&'static str, String)>,
    }

    #[derive(Debug, PartialEq)]
    enum Measurement {
        CounterU64(u64),
        UpDownI64(i64),
        Gauge(i64),
    }

    impl CapturingRecorder {
        fn take(&self) -> Vec<Recorded> {
            std::mem::take(&mut *self.events.lock().unwrap())
        }
    }

    fn stringify(attrs: &[KeyValue]) -> Vec<(&'static str, String)> {
        attrs
            .iter()
            .map(|kv| {
                let v = match &kv.value {
                    metrics::Value::Bool(b) => b.to_string(),
                    metrics::Value::Int(i) => i.to_string(),
                    metrics::Value::F64(f) => f.to_string(),
                    metrics::Value::Str(s) => s.to_string(),
                };
                (kv.key, v)
            })
            .collect()
    }

    impl MetricsRecorder for CapturingRecorder {
        fn add_counter_u64(
            &self,
            instrument: &'static metrics::Instrument,
            value: u64,
            attrs: &[KeyValue],
        ) {
            self.events.lock().unwrap().push(Recorded {
                instrument: instrument.name,
                kind: Measurement::CounterU64(value),
                attrs: stringify(attrs),
            });
        }

        fn add_up_down_counter_i64(
            &self,
            instrument: &'static metrics::Instrument,
            value: i64,
            attrs: &[KeyValue],
        ) {
            self.events.lock().unwrap().push(Recorded {
                instrument: instrument.name,
                kind: Measurement::UpDownI64(value),
                attrs: stringify(attrs),
            });
        }

        fn record_histogram_f64(&self, _: &'static metrics::Instrument, _: f64, _: &[KeyValue]) {
            unreachable!("worker emits no histograms");
        }

        fn record_gauge_i64(
            &self,
            instrument: &'static metrics::Instrument,
            value: i64,
            attrs: &[KeyValue],
        ) {
            self.events.lock().unwrap().push(Recorded {
                instrument: instrument.name,
                kind: Measurement::Gauge(value),
                attrs: stringify(attrs),
            });
        }
    }

    fn attr<'a>(rec: &'a Recorded, key: &str) -> Option<&'a str> {
        rec.attrs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Build a [`RecorderHandle`] backed by a [`CapturingRecorder`], wired
    /// with the canonical test attributes used by the transition tests.
    fn test_handle() -> (Arc<CapturingRecorder>, RecorderHandle) {
        let recorder = Arc::new(CapturingRecorder::default());
        let dyn_recorder: Arc<dyn MetricsRecorder> = recorder.clone();
        let mut handle = RecorderHandle::new(Some(dyn_recorder), Arc::from("xds:///my-service"));
        handle.set_server(Arc::from("xds.example.com:443"));
        (recorder, handle)
    }

    fn test_type_url() -> Arc<str> {
        Arc::from("envoy.config.listener.v3.Listener")
    }

    /// Value of the `resources` gauge emitted for a given `cache_state`, if any.
    fn gauge_for(events: &[Recorded], cache_state: &str) -> Option<i64> {
        events.iter().find_map(|e| {
            if attr(e, "grpc.xds.cache_state") == Some(cache_state) {
                match e.kind {
                    Measurement::Gauge(v) => Some(v),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    #[test]
    fn first_sync_emits_each_bucket_with_attrs() {
        let (recorder, mut handle) = test_handle();
        let type_url = test_type_url();
        let counts: HashMap<&'static str, i64> = HashMap::from([("acked", 2), ("requested", 1)]);
        handle.sync_resource_counts(&type_url, &counts);

        let events = recorder.take();
        assert_eq!(events.len(), 2);
        assert_eq!(gauge_for(&events, "acked"), Some(2));
        assert_eq!(gauge_for(&events, "requested"), Some(1));

        let acked = events
            .iter()
            .find(|e| attr(e, "grpc.xds.cache_state") == Some("acked"))
            .expect("acked bucket emitted");
        assert_eq!(acked.instrument, "grpc.xds_client.resources");
        assert_eq!(
            attr(acked, "grpc.xds.resource_type"),
            Some("envoy.config.listener.v3.Listener")
        );
        assert_eq!(attr(acked, "grpc.target"), Some("xds:///my-service"));
        assert_eq!(attr(acked, "grpc.xds.authority"), Some("#old"));
        assert_eq!(attr(acked, "grpc.xds.server"), None);
    }

    #[test]
    fn unchanged_sync_is_idempotent() {
        let (recorder, mut handle) = test_handle();
        let type_url = test_type_url();
        let counts: HashMap<&'static str, i64> = HashMap::from([("acked", 2)]);
        handle.sync_resource_counts(&type_url, &counts);
        let _ = recorder.take();

        handle.sync_resource_counts(&type_url, &counts);
        assert!(recorder.take().is_empty());
    }

    #[test]
    fn sync_emits_only_changed_buckets() {
        let (recorder, mut handle) = test_handle();
        let type_url = test_type_url();
        handle.sync_resource_counts(&type_url, &HashMap::from([("acked", 2)]));
        let _ = recorder.take();

        // `acked` drops to 1 and a new `nacked` bucket appears.
        handle.sync_resource_counts(&type_url, &HashMap::from([("acked", 1), ("nacked", 1)]));

        let events = recorder.take();
        assert_eq!(events.len(), 2);
        assert_eq!(gauge_for(&events, "acked"), Some(1));
        assert_eq!(gauge_for(&events, "nacked"), Some(1));
    }

    #[test]
    fn emptied_bucket_is_reset_to_zero() {
        let (recorder, mut handle) = test_handle();
        let type_url = test_type_url();
        handle.sync_resource_counts(&type_url, &HashMap::from([("acked", 1)]));
        let _ = recorder.take();

        // The whole type empties (e.g. all resources removed).
        handle.sync_resource_counts(&type_url, &HashMap::new());

        let events = recorder.take();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].instrument, "grpc.xds_client.resources");
        assert_eq!(gauge_for(&events, "acked"), Some(0));
    }
}

/// Regression tests for the worker's ADS flow control: watcher deliveries and
/// `ProcessingDone` waits must not freeze the event loop (deadlock with
/// watchers that issue commands while holding the token) and must gate
/// reading the next response.
#[cfg(test)]
mod flow_control_tests {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use bytes::Bytes;
    use tokio::sync::mpsc;

    use crate::client::config::{ClientConfig, ServerConfig};
    use crate::client::retry::RetryPolicy;
    use crate::client::watch::{ResourceEvent, ResourceWatcher};
    use crate::codec::XdsCodec;
    use crate::error::Result;
    use crate::message::{DiscoveryRequest, DiscoveryResponse, Node, ResourceAny};
    use crate::metrics::{self, KeyValue, MetricsRecorder};
    use crate::resource::{Resource, TypeUrl};
    use crate::runtime::tokio::TokioRuntime;
    use crate::transport::TransportBuilder;
    use crate::transport::mock::{MockServer, MockTransport, MockTransportBuilder, mock_transport};
    use crate::{XdsClient, error::Error};

    const TEST_TYPE_URL: &str = "type.googleapis.com/test.Resource";
    const SOTW_TYPE_URL: &str = "type.googleapis.com/test.SotwResource";

    /// Minimal resource: the message is the resource name itself. Names
    /// starting with `bad` fail validation (a per-resource error, per A46).
    #[derive(Debug, Clone)]
    struct TestResource;

    impl Resource for TestResource {
        type Message = String;
        const TYPE_URL: TypeUrl = TypeUrl::new(TEST_TYPE_URL);
        const ALL_RESOURCES_REQUIRED_IN_SOTW: bool = false;

        fn deserialize(bytes: Bytes) -> Result<Self::Message> {
            String::from_utf8(bytes.to_vec()).map_err(|e| Error::Validation(e.to_string()))
        }

        fn name(message: &Self::Message) -> &str {
            message
        }

        fn validate(message: Self::Message) -> Result<Self> {
            if message.starts_with("bad") {
                return Err(Error::Validation("bad resource".to_string()));
            }
            Ok(Self)
        }
    }

    /// Like [`TestResource`] but with `ALL_RESOURCES_REQUIRED_IN_SOTW`, so
    /// resources missing from a response are treated as deleted (gRFC A53).
    #[derive(Debug, Clone)]
    struct SotwResource;

    impl Resource for SotwResource {
        type Message = String;
        const TYPE_URL: TypeUrl = TypeUrl::new(SOTW_TYPE_URL);
        const ALL_RESOURCES_REQUIRED_IN_SOTW: bool = true;

        fn deserialize(bytes: Bytes) -> Result<Self::Message> {
            String::from_utf8(bytes.to_vec()).map_err(|e| Error::Validation(e.to_string()))
        }

        fn name(message: &Self::Message) -> &str {
            message
        }

        fn validate(_message: Self::Message) -> Result<Self> {
            Ok(Self)
        }
    }

    /// Line-based codec: `type_url \n version \n nonce \n name,name,...`.
    struct FakeCodec;

    impl XdsCodec for FakeCodec {
        fn encode_request(&self, request: &DiscoveryRequest<'_>) -> Result<Bytes> {
            Ok(Bytes::from(format!(
                "{}\n{}\n{}\n{}",
                request.type_url,
                request.version_info,
                request.response_nonce,
                request.resource_names.join(",")
            )))
        }

        fn decode_response(&self, bytes: Bytes) -> Result<DiscoveryResponse> {
            let text =
                String::from_utf8(bytes.to_vec()).map_err(|e| Error::Validation(e.to_string()))?;
            let mut lines = text.split('\n');
            let type_url = lines.next().unwrap_or_default().to_string();
            let version_info = lines.next().unwrap_or_default().to_string();
            let nonce = lines.next().unwrap_or_default().to_string();
            let resources = lines
                .next()
                .unwrap_or_default()
                .split(',')
                .filter(|n| !n.is_empty())
                .map(|name| ResourceAny {
                    type_url: type_url.clone(),
                    value: Bytes::from(name.to_string()),
                })
                .collect();
            Ok(DiscoveryResponse {
                version_info,
                resources,
                type_url,
                nonce,
            })
        }
    }

    fn response_for(type_url: &str, version: &str, nonce: &str, names: &[&str]) -> Bytes {
        Bytes::from(format!(
            "{type_url}\n{version}\n{nonce}\n{}",
            names.join(",")
        ))
    }

    fn response(version: &str, nonce: &str, names: &[&str]) -> Bytes {
        response_for(TEST_TYPE_URL, version, nonce, names)
    }

    fn sotw_response(version: &str, nonce: &str, names: &[&str]) -> Bytes {
        response_for(SOTW_TYPE_URL, version, nonce, names)
    }

    /// (version_info, response_nonce) of an encoded request.
    fn parse_request(bytes: &Bytes) -> (String, String) {
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        let mut lines = text.split('\n');
        let _type_url = lines.next().unwrap_or_default();
        let version = lines.next().unwrap_or_default().to_string();
        let nonce = lines.next().unwrap_or_default().to_string();
        (version, nonce)
    }

    fn fast_retry_policy() -> RetryPolicy {
        RetryPolicy::new(Duration::from_millis(1), Duration::from_millis(1), 1.0)
            .unwrap()
            .with_jitter(0.0)
            .unwrap()
    }

    struct RecordingBuilder {
        inner: MockTransportBuilder,
        built_uris: Arc<Mutex<Vec<String>>>,
    }

    #[derive(Default)]
    struct ConnectionRecorder {
        events: Mutex<Vec<(&'static str, i64)>>,
    }

    impl MetricsRecorder for ConnectionRecorder {
        fn add_counter_u64(
            &self,
            instrument: &'static metrics::Instrument,
            value: u64,
            _attrs: &[KeyValue],
        ) {
            if instrument.name == metrics::instruments::XDS_CLIENT_SERVER_FAILURE.name {
                self.events
                    .lock()
                    .unwrap()
                    .push((instrument.name, value as i64));
            }
        }

        fn add_up_down_counter_i64(
            &self,
            _instrument: &'static metrics::Instrument,
            _value: i64,
            _attrs: &[KeyValue],
        ) {
        }

        fn record_histogram_f64(
            &self,
            _instrument: &'static metrics::Instrument,
            _value: f64,
            _attrs: &[KeyValue],
        ) {
        }

        fn record_gauge_i64(
            &self,
            instrument: &'static metrics::Instrument,
            value: i64,
            _attrs: &[KeyValue],
        ) {
            if instrument.name == metrics::instruments::XDS_CLIENT_CONNECTED.name {
                self.events.lock().unwrap().push((instrument.name, value));
            }
        }
    }

    impl TransportBuilder for RecordingBuilder {
        type Transport = MockTransport;

        async fn build(&self, server: &ServerConfig) -> Result<Self::Transport> {
            self.built_uris
                .lock()
                .unwrap()
                .push(server.uri().to_string());
            self.inner.build(server).await
        }
    }

    /// Client watching `res-0` (of resource type `T`) with an established
    /// mock stream, its initial request already drained.
    async fn connected_client_for<T: Resource>() -> (XdsClient, ResourceWatcher<T>, MockServer) {
        let (builder, mut servers) = mock_transport();
        let config = ClientConfig::new(Node::new("test", "0"), "mock:///xds");
        let client = XdsClient::builder(config, builder, FakeCodec, TokioRuntime).build();

        let watcher = client.watch::<T>("res-0").await;
        let mut server = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .expect("timed out waiting for stream")
            .expect("transport dropped");
        let _initial = tokio::time::timeout(Duration::from_secs(5), server.requests.recv())
            .await
            .expect("timed out waiting for initial request")
            .expect("stream closed");
        (client, watcher, server)
    }

    async fn connected_client() -> (XdsClient, ResourceWatcher<TestResource>, MockServer) {
        connected_client_for::<TestResource>().await
    }

    /// Watch `name` and wait for the resulting subscription request, so the
    /// watcher is registered before the test sends a response.
    async fn watch_synced(
        client: &XdsClient,
        server: &mut MockServer,
        name: &str,
    ) -> ResourceWatcher<TestResource> {
        let watcher = client.watch::<TestResource>(name).await;
        let _request = tokio::time::timeout(Duration::from_secs(5), server.requests.recv())
            .await
            .expect("timed out waiting for subscription request")
            .expect("stream closed");
        watcher
    }

    /// Next event, unwrapped to its result and `ProcessingDone` token.
    async fn next_changed<T: Resource>(
        watcher: &mut ResourceWatcher<T>,
    ) -> (
        Result<std::sync::Arc<T>>,
        crate::client::watch::ProcessingDone,
    ) {
        let event = tokio::time::timeout(Duration::from_secs(5), watcher.next())
            .await
            .expect("timed out waiting for event")
            .expect("watcher closed");
        match event {
            ResourceEvent::ResourceChanged { result, done } => (result, done),
            ResourceEvent::AmbientError { .. } => panic!("unexpected ambient error"),
        }
    }

    /// Asserts no event is delivered to `watcher` within a short window.
    async fn assert_no_event<T: Resource>(watcher: &mut ResourceWatcher<T>, message: &str) {
        assert!(
            tokio::time::timeout(Duration::from_millis(200), watcher.next())
                .await
                .is_err(),
            "{message}"
        );
    }

    /// A watcher that issues more commands than the command channel buffers
    /// while holding its `ProcessingDone` token must not deadlock the worker.
    ///
    /// Before the flow-control fix the worker awaited the token inside
    /// `handle_response`, so it stopped draining commands; once the channel
    /// filled, watcher and worker waited on each other forever.
    #[tokio::test]
    async fn commands_drain_while_processing_done_is_held() {
        let (client, mut watcher, server) = connected_client().await;

        server
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0"]))))
            .unwrap();
        let event = tokio::time::timeout(Duration::from_secs(5), watcher.next())
            .await
            .expect("timed out waiting for event")
            .expect("watcher closed");
        let ResourceEvent::ResourceChanged {
            result: Ok(_),
            done,
        } = event
        else {
            panic!("expected ResourceChanged(Ok)");
        };

        // Well past COMMAND_CHANNEL_BUFFER_SIZE (64).
        let mut extra = Vec::new();
        tokio::time::timeout(Duration::from_secs(5), async {
            for i in 1..=100 {
                extra.push(client.watch::<TestResource>(format!("res-{i}")).await);
            }
        })
        .await
        .expect("deadlock: commands not drained while ProcessingDone was held");

        // The worker is still healthy end-to-end: it reads the next response
        // and delivers it.
        drop(done);
        server
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0"]))))
            .unwrap();
        let event = tokio::time::timeout(Duration::from_secs(5), watcher.next())
            .await
            .expect("timed out waiting for second event")
            .expect("watcher closed");
        assert!(matches!(
            event,
            ResourceEvent::ResourceChanged { result: Ok(_), .. }
        ));
    }

    /// The ACK goes out as soon as the response is validated and cached, and
    /// the *next* response is not delivered until the previous one's
    /// `ProcessingDone` tokens drop (ADS flow control).
    #[tokio::test]
    async fn next_response_gated_until_processing_done() {
        let (_client, mut watcher, mut server) = connected_client().await;

        server
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0"]))))
            .unwrap();
        let event = tokio::time::timeout(Duration::from_secs(5), watcher.next())
            .await
            .expect("timed out waiting for event")
            .expect("watcher closed");
        let ResourceEvent::ResourceChanged {
            result: Ok(_),
            done,
        } = event
        else {
            panic!("expected ResourceChanged(Ok)");
        };

        // ACK is not gated on ProcessingDone.
        let ack = tokio::time::timeout(Duration::from_secs(5), server.requests.recv())
            .await
            .expect("ACK not sent while ProcessingDone was held")
            .expect("stream closed");
        assert_eq!(parse_request(&ack), ("1".to_string(), "n1".to_string()));

        // The next response is gated on ProcessingDone.
        server
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0"]))))
            .unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(200), watcher.next())
                .await
                .is_err(),
            "second response delivered while the first was still being processed"
        );

        drop(done);
        let event = tokio::time::timeout(Duration::from_secs(5), watcher.next())
            .await
            .expect("timed out waiting for second event")
            .expect("watcher closed");
        assert!(matches!(
            event,
            ResourceEvent::ResourceChanged { result: Ok(_), .. }
        ));
        let ack = tokio::time::timeout(Duration::from_secs(5), server.requests.recv())
            .await
            .expect("second ACK not sent")
            .expect("stream closed");
        assert_eq!(parse_request(&ack), ("2".to_string(), "n2".to_string()));
    }

    /// The next response is gated until *every* watcher drops its
    /// `ProcessingDone` token, not just the first one.
    #[tokio::test]
    async fn next_response_gated_until_all_watchers_signal() {
        let (client, mut w1, mut server) = connected_client().await;
        let mut w2 = watch_synced(&client, &mut server, "res-1").await;

        server
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0", "res-1"]))))
            .unwrap();
        let (r1, done1) = next_changed(&mut w1).await;
        let (r2, done2) = next_changed(&mut w2).await;
        assert!(r1.is_ok() && r2.is_ok());

        drop(done1);
        server
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0", "res-1"]))))
            .unwrap();
        assert_no_event(&mut w1, "second response delivered while a token was held").await;

        drop(done2);
        assert!(next_changed(&mut w1).await.0.is_ok());
        assert!(next_changed(&mut w2).await.0.is_ok());
    }

    /// Validation-error events do not gate flow control (gRFC A46/A88):
    /// the response is NACKed, valid resources are still delivered, and a
    /// held error token must not delay the next response.
    #[tokio::test]
    async fn error_events_do_not_gate_next_response() {
        let (client, mut w_ok, mut server) = connected_client().await;
        let mut w_bad = watch_synced(&client, &mut server, "bad-0").await;

        server
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0", "bad-0"]))))
            .unwrap();
        let (result, done_ok) = next_changed(&mut w_ok).await;
        assert!(result.is_ok());
        let (result, _err_done) = next_changed(&mut w_bad).await;
        assert!(matches!(result, Err(Error::Validation(_))));

        // NACK keeps the old (empty) version.
        let nack = tokio::time::timeout(Duration::from_secs(5), server.requests.recv())
            .await
            .expect("NACK not sent")
            .expect("stream closed");
        assert_eq!(parse_request(&nack), ("".to_string(), "n1".to_string()));

        drop(done_ok);
        server
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0"]))))
            .unwrap();
        // `_err_done` is still held; it must not gate this delivery.
        assert!(next_changed(&mut w_ok).await.0.is_ok());
    }

    /// Deletion events (SotW resource missing from a response, gRFC A53)
    /// gate the next response like regular updates.
    #[tokio::test]
    async fn deletion_events_gate_next_response() {
        let (_client, mut watcher, server) = connected_client_for::<SotwResource>().await;

        server
            .responses
            .send(Ok(Some(sotw_response("1", "n1", &["res-0"]))))
            .unwrap();
        let (result, done) = next_changed(&mut watcher).await;
        assert!(result.is_ok());
        drop(done);

        // res-0 missing from the SotW response: deleted.
        server
            .responses
            .send(Ok(Some(sotw_response("2", "n2", &[]))))
            .unwrap();
        let (result, deletion_done) = next_changed(&mut watcher).await;
        assert!(matches!(result, Err(Error::ResourceDoesNotExist)));

        server
            .responses
            .send(Ok(Some(sotw_response("3", "n3", &["res-0"]))))
            .unwrap();
        assert_no_event(
            &mut watcher,
            "response delivered while the deletion token was held",
        )
        .await;

        drop(deletion_done);
        assert!(next_changed(&mut watcher).await.0.is_ok());
    }

    /// Events staged for a watcher that was dropped must not wedge flow
    /// control: their failed sends release the shared signal.
    #[tokio::test]
    async fn dropped_watcher_does_not_stall_flow_control() {
        let (client, mut w1, mut server) = connected_client().await;
        drop(watch_synced(&client, &mut server, "res-1").await);

        server
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0", "res-1"]))))
            .unwrap();
        let (result, done) = next_changed(&mut w1).await;
        assert!(result.is_ok());
        drop(done);

        server
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0", "res-1"]))))
            .unwrap();
        assert!(next_changed(&mut w1).await.0.is_ok());
    }

    /// Reconnect uses the last accepted version, clears the stream-scoped
    /// nonce, and continues to deliver updates through the same watcher.
    #[tokio::test]
    async fn reconnect_preserves_request_and_watcher_behavior() {
        let (builder, mut servers) = mock_transport();
        let config = ClientConfig::new(Node::new("test", "0"), "mock:///xds")
            .with_retry_policy(fast_retry_policy());
        let client = XdsClient::builder(config, builder, FakeCodec, TokioRuntime).build();
        let mut watcher = client.watch::<TestResource>("res-0").await;

        let mut first = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .unwrap()
            .unwrap();
        let initial = first.requests.recv().await.unwrap();
        assert_eq!(parse_request(&initial), (String::new(), String::new()));

        first
            .responses
            .send(Ok(Some(response("1", "n1", &["res-0"]))))
            .unwrap();
        let (result, done) = next_changed(&mut watcher).await;
        assert!(result.is_ok());
        drop(done);
        assert_eq!(
            parse_request(&first.requests.recv().await.unwrap()),
            ("1".to_string(), "n1".to_string())
        );

        first.responses.send(Ok(None)).unwrap();
        let mut second = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .expect("worker did not reconnect")
            .expect("transport dropped");
        assert_eq!(
            parse_request(&second.requests.recv().await.unwrap()),
            ("1".to_string(), String::new())
        );

        second
            .responses
            .send(Ok(Some(response("2", "n2", &["res-0"]))))
            .unwrap();
        assert!(next_changed(&mut watcher).await.0.is_ok());
    }

    /// Connected and server-failure metrics retain their pre-refactor edge
    /// semantics across a failure before the first response and reconnect.
    #[tokio::test]
    async fn reconnect_preserves_connection_metric_transitions() {
        let (builder, mut servers) = mock_transport();
        let recorder = Arc::new(ConnectionRecorder::default());
        let dyn_recorder: Arc<dyn MetricsRecorder> = recorder.clone();
        let config = ClientConfig::new(Node::new("test", "0"), "mock:///xds")
            .with_retry_policy(fast_retry_policy());
        let client = XdsClient::builder(config, builder, FakeCodec, TokioRuntime)
            .with_metrics_recorder(dyn_recorder)
            .build();
        let _watcher = client.watch::<TestResource>("res-0").await;

        let first = servers.recv().await.unwrap();
        first.responses.send(Ok(None)).unwrap();
        let _second = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .expect("worker did not reconnect")
            .expect("transport dropped");

        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if recorder.events.lock().unwrap().len() >= 4 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("metric transitions were not recorded");
        assert_eq!(
            recorder.events.lock().unwrap().as_slice(),
            [
                (metrics::instruments::XDS_CLIENT_CONNECTED.name, 1),
                (metrics::instruments::XDS_CLIENT_SERVER_FAILURE.name, 1),
                (metrics::instruments::XDS_CLIENT_CONNECTED.name, 0),
                (metrics::instruments::XDS_CLIENT_CONNECTED.name, 1),
            ]
        );
    }

    /// The compatibility refactor still selects only the first configured
    /// server; ordered fallback belongs to the following PR.
    #[tokio::test]
    async fn only_the_first_configured_server_is_active() {
        let (inner, mut servers) = mock_transport();
        let built_uris = Arc::new(Mutex::new(Vec::new()));
        let builder = RecordingBuilder {
            inner,
            built_uris: Arc::clone(&built_uris),
        };
        let config = ClientConfig::with_servers(
            Node::new("test", "0"),
            vec![
                ServerConfig::new("mock:///primary"),
                ServerConfig::new("mock:///backup"),
            ],
        );
        let client = XdsClient::builder(config, builder, FakeCodec, TokioRuntime).build();
        let _watcher = client.watch::<TestResource>("res-0").await;
        let _server = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .unwrap()
            .unwrap();

        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(built_uris.lock().unwrap().as_slice(), ["mock:///primary"]);
    }

    /// A coalesced subscription remains dirty until the bounded server-task
    /// command channel actually has room for its latest snapshot.
    #[tokio::test]
    async fn dirty_subscription_survives_a_full_command_channel() {
        let (builder, _servers) = mock_transport();
        let (worker_command_tx, worker_command_rx) = mpsc::channel(1);
        let config = ClientConfig::new(Node::new("test", "0"), "mock:///xds");
        let mut worker = super::AdsWorker::new(
            builder,
            FakeCodec,
            TokioRuntime,
            config,
            worker_command_tx,
            worker_command_rx,
            None,
        );
        let (event_tx, _event_rx) = mpsc::channel(1);
        assert!(worker.add_watcher(
            TEST_TYPE_URL,
            "res-0".to_string(),
            super::WatcherId::new(),
            event_tx,
            Box::new(|_| unreachable!()),
            false,
        ));

        let (command_tx, mut command_rx) = mpsc::channel(1);
        let server = super::ServerHandle { command_tx };
        server
            .command_tx
            .try_send(super::ServerCommand::Resume { generation: 0 })
            .unwrap();

        worker.send_request(&server, TEST_TYPE_URL).unwrap();
        assert!(worker.dirty_types.contains(TEST_TYPE_URL));
        worker.flush_dirty(&server);
        assert!(worker.dirty_types.contains(TEST_TYPE_URL));

        let _ = command_rx.recv().await;
        worker.flush_dirty(&server);
        assert!(!worker.dirty_types.contains(TEST_TYPE_URL));
        assert!(matches!(
            command_rx.recv().await,
            Some(super::ServerCommand::Send { .. })
        ));
    }

    /// A failed stream write is surfaced as stream closure and follows the
    /// same reconnect path as a receive-side failure.
    #[tokio::test]
    async fn write_failure_reconnects_and_resubscribes() {
        let (builder, mut servers) = mock_transport();
        let config = ClientConfig::new(Node::new("test", "0"), "mock:///xds")
            .with_retry_policy(fast_retry_policy());
        let client = XdsClient::builder(config, builder, FakeCodec, TokioRuntime).build();
        let _first_watcher = client.watch::<TestResource>("res-0").await;
        let mut first = servers.recv().await.unwrap();
        let _ = first.requests.recv().await.unwrap();
        drop(first.requests);

        let _second_watcher = client.watch::<TestResource>("res-1").await;
        let mut second = tokio::time::timeout(Duration::from_secs(5), servers.recv())
            .await
            .expect("write failure did not trigger reconnect")
            .expect("transport dropped");
        let request = String::from_utf8(second.requests.recv().await.unwrap().to_vec()).unwrap();
        assert!(request.ends_with("res-0,res-1") || request.ends_with("res-1,res-0"));
    }
}
