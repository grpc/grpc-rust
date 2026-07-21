//! ADS worker that manages the xDS stream.
//!
//! The worker runs as a background task, managing:
//! - The ADS stream lifecycle (connection, reconnection)
//! - Resource subscriptions and version/nonce tracking
//! - Dispatching resources to watchers
//! - ACK/NACK protocol

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
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
        let (done, _rx) = ProcessingDone::channel();
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
    /// Version from last successful response.
    version_info: String,
    /// Nonce from last response (for ACK/NACK).
    nonce: String,
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
            .field("version_info", &self.version_info)
            .field("nonce", &self.nonce)
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
            version_info: String::new(),
            nonce: String::new(),
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
    transport_builder: TB,
    /// Codec for encoding/decoding messages.
    codec: C,
    /// Runtime for spawning tasks and sleeping.
    runtime: R,
    /// Node identification.
    node: Node,
    /// Backoff calculator for reconnection attempts.
    backoff: Backoff,
    /// Priority-ordered list of xDS servers.
    /// Index 0 has the highest priority.
    servers: Vec<ServerConfig>,
    /// Timeout for initial resource response (gRFC A57). None = disabled.
    resource_initial_timeout: Option<Duration>,
    /// Sender for timer callback commands.
    command_tx: mpsc::Sender<WorkerCommand>,
    /// Receiver for commands from XdsClient.
    command_rx: mpsc::Receiver<WorkerCommand>,
    /// Per-type_url state.
    type_states: HashMap<String, TypeState>,
    /// Cancellation handles for resource timers (gRFC A57).
    /// Key is (type_url, resource_name). Dropping the sender cancels the timer.
    resource_timers: HashMap<(String, String), oneshot::Sender<()>>,
    /// Optional backend + per-client A78 metric attributes
    /// (`grpc.target` + `grpc.xds.server`).
    recorder: RecorderHandle,
}

/// Outcome of a connected ADS session (see [`AdsWorker::run_connected`]).
enum ConnectedOutcome {
    /// All `XdsClient` handles were dropped; the worker should shut down.
    Shutdown,
    /// The ADS stream failed; the worker should reconnect. `saw_response`
    /// indicates whether at least one response was received before the failure
    /// — per gRFC A78, a stream that fails after a response is not counted as a
    /// server failure.
    Failed { saw_response: bool },
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
            transport_builder,
            codec,
            runtime,
            node: config.node,
            backoff: Backoff::new(config.retry_policy),
            servers: config.servers,
            resource_initial_timeout: config.resource_initial_timeout,
            command_tx,
            command_rx,
            type_states: HashMap::new(),
            resource_timers: HashMap::new(),
            recorder: RecorderHandle::new(recorder, target),
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
        let mut healthy = false;
        loop {
            // Wait for at least one subscription before connecting.
            // This prevents deadlock with servers that require a message before
            // sending response headers - we need something to send.
            while self.type_states.is_empty() {
                match self.command_rx.recv().await {
                    Some(cmd) => {
                        let _ = self
                            .handle_command::<<TB::Transport as Transport>::Stream>(None, cmd)
                            .await;
                    }
                    None => return,
                }
            }

            // Nonces are tied to the stream
            for type_state in self.type_states.values_mut() {
                type_state.nonce.clear();
            }

            // Connect to server.
            // Future extension (gRFC A71): Try servers in priority order with fallback.
            let server = match self.servers.first() {
                Some(s) => s,
                None => return, // No servers configured
            };
            self.recorder.set_server(Arc::from(server.uri()));

            let transport = match self.transport_builder.build(server).await {
                Ok(t) => t,
                Err(_) => {
                    self.record_unhealthy(&mut healthy);
                    match self.backoff.next_backoff() {
                        Some(backoff) => self.runtime.sleep(backoff).await,
                        None => return, // Max attempts exceeded
                    }
                    continue;
                }
            };

            let stream = match transport.new_stream(self.build_initial_requests()).await {
                Ok(s) => {
                    self.backoff.reset();
                    s
                }
                Err(_) => {
                    self.record_unhealthy(&mut healthy);
                    match self.backoff.next_backoff() {
                        Some(backoff) => self.runtime.sleep(backoff).await,
                        None => return, // Max attempts exceeded
                    }
                    continue;
                }
            };

            if !healthy {
                self.recorder.record_connected(true);
                healthy = true;
            }

            match self.run_connected(stream).await {
                ConnectedOutcome::Shutdown => return,
                ConnectedOutcome::Failed { saw_response } => {
                    // gRFC A78: a server goes unhealthy (one `server_failure`) on
                    // a connectivity failure or when the ADS stream fails
                    // *without* seeing a response message. A stream that failed
                    // after receiving a response is not counted; just reconnect.
                    if !saw_response {
                        self.record_unhealthy(&mut healthy);
                    }
                    match self.backoff.next_backoff() {
                        Some(backoff) => self.runtime.sleep(backoff).await,
                        None => return, // Max attempts exceeded
                    }
                    continue;
                }
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
    fn build_initial_requests(&self) -> Vec<Bytes> {
        let mut requests = Vec::new();

        for (type_url, type_state) in &self.type_states {
            if type_state.watchers.is_empty() {
                continue;
            }

            let resource_names = type_state.resource_names_for_request();

            let request = DiscoveryRequest {
                node: &self.node,
                type_url,
                resource_names: &resource_names,
                version_info: &type_state.version_info,
                response_nonce: "", // Initial request has empty nonce
                error_detail: None,
            };

            if let Ok(bytes) = self.codec.encode_request(&request) {
                requests.push(bytes);
            }
        }

        requests
    }

    /// Run the main event loop while connected.
    ///
    /// Returns [`ConnectedOutcome::Shutdown`] if the worker should shut down
    /// (command channel closed), or [`ConnectedOutcome::Failed`] if the stream
    /// failed and the worker should reconnect (carrying whether a response was
    /// seen, per gRFC A78).
    async fn run_connected<S: TransportStream>(&mut self, mut stream: S) -> ConnectedOutcome {
        // Whether at least one response was received on this stream. Per gRFC
        // A78 a stream that fails *after* receiving a response is not counted as
        // a server failure.
        let mut saw_response = false;
        loop {
            tokio::select! {
                result = stream.recv() => {
                    match result {
                        Ok(Some(bytes)) => {
                            saw_response = true;
                            if self.handle_response(&mut stream, bytes).await.is_err() {
                                return ConnectedOutcome::Failed { saw_response };
                            }
                        }
                        // Stream closed by server or errored; reconnect.
                        Ok(None) | Err(_) => return ConnectedOutcome::Failed { saw_response },
                    }
                }

                cmd = self.command_rx.recv() => {
                    match cmd {
                        Some(cmd) => {
                            if self.handle_command(Some(&mut stream), cmd).await.is_err() {
                                return ConnectedOutcome::Failed { saw_response };
                            }
                        }
                        None => return ConnectedOutcome::Shutdown,
                    }
                }
            }
        }
    }

    /// Handle a command, optionally sending network requests if connected.
    ///
    /// When `stream` is `None`, only state updates are performed (disconnected mode).
    /// When `stream` is `Some`, subscription changes trigger network requests.
    async fn handle_command<S: TransportStream>(
        &mut self,
        stream: Option<&mut S>,
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
                ) && let Some(stream) = stream
                {
                    self.send_request(stream, type_url).await?;
                }
            }
            WorkerCommand::Unwatch { watcher_id } => {
                if let Some((type_url, true)) = self.remove_watcher(watcher_id)
                    && let Some(stream) = stream
                {
                    self.send_request(stream, &type_url).await?;
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
            // The type is gone — reset all of its resource buckets to zero.
            self.recorder
                .sync_resource_counts(&type_url_arc, &HashMap::new());
            // Cancel all pending resource timers for this type.
            self.resource_timers.retain(|key, _| key.0 != type_url);
        }

        Some((type_url, subscriptions_changed))
    }

    /// Send a DiscoveryRequest for a type.
    async fn send_request<S: TransportStream>(&self, stream: &mut S, type_url: &str) -> Result<()> {
        let type_state = match self.type_states.get(type_url) {
            Some(s) => s,
            None => return Ok(()),
        };

        let resource_names = type_state.resource_names_for_request();
        let request = DiscoveryRequest {
            node: &self.node,
            type_url,
            resource_names: &resource_names,
            version_info: &type_state.version_info,
            response_nonce: &type_state.nonce,
            error_detail: None,
        };

        let bytes = self.codec.encode_request(&request)?;
        stream.send(bytes).await
    }

    /// Handle a response from the server.
    ///
    /// Implements partial success per gRFC A46: valid resources are accepted even
    /// if some resources in the response fail validation. Each resource is processed
    /// independently:
    /// - Valid resources are cached and dispatched to watchers
    /// - Invalid resources are cached as NACKed and errors sent to specific watchers
    /// - Missing resources (for types with ALL_RESOURCES_REQUIRED_IN_SOTW) are marked deleted
    async fn handle_response<S: TransportStream>(
        &mut self,
        stream: &mut S,
        bytes: Bytes,
    ) -> Result<()> {
        let response = self.codec.decode_response(bytes)?;
        let type_url = response.type_url.clone();

        let (type_url_arc, decoder) = match self.type_states.get(&type_url) {
            Some(s) => (Arc::clone(&s.type_url), &s.decoder),
            None => {
                return Ok(());
            }
        };

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

        if let Some(type_state) = self.type_states.get_mut(&type_url) {
            type_state.nonce = response.nonce.clone();
        }

        let received_names: HashSet<String> = valid_resources
            .iter()
            .map(|r| r.name().to_string())
            .collect();

        let mut processing_done_futures = self.dispatch_resources(&type_url, valid_resources).await;

        // Only notify watchers for per-resource errors (where we know the name).
        // Top-level errors have no associated name, so no watcher to notify.
        for (resource_name, error) in &per_resource_errors {
            self.notify_resource_error(&type_url, resource_name, error)
                .await;
        }

        // Detect deleted resources (per A53):
        // For resource types with ALL_RESOURCES_REQUIRED_IN_SOTW = true,
        // any previously-received resource not in this response is deleted.
        let deleted_futures = self
            .detect_deleted_resources(&type_url, &received_names)
            .await;
        processing_done_futures.extend(deleted_futures);

        // Wait for all watchers to finish processing.
        for rx in processing_done_futures {
            let _ = rx.await;
        }

        let has_errors = !top_level_errors.is_empty() || !per_resource_errors.is_empty();
        if !has_errors {
            // Only update version on ACK; NACK must keep the old version so the
            // server knows which version the client is still running.
            if let Some(ts) = self.type_states.get_mut(&type_url) {
                ts.version_info = response.version_info.clone();
            }
            self.send_ack(stream, &response).await
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

            self.send_nack(stream, &response, error_parts.join("; "))
                .await
        }
    }

    /// Dispatch decoded resources to watchers and update cache.
    ///
    /// Returns futures that resolve when watchers signal ProcessingDone.
    /// Uses backpressure: waits if a watcher's channel is full.
    async fn dispatch_resources(
        &mut self,
        type_url: &str,
        resources: Vec<DecodedResource>,
    ) -> Vec<oneshot::Receiver<()>> {
        let mut processing_done_futures = Vec::new();

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
            None => return processing_done_futures,
        };

        // Cancel resource timers for received resources (gRFC A57).
        for resource in &resources {
            self.resource_timers
                .remove(&(type_url.to_string(), resource.name().to_string()));
        }

        for resource in resources {
            let resource_name = resource.name().to_string();
            let resource = Arc::new(resource);

            for (_watcher_id, event_tx, subscription) in watcher_info.clone() {
                if subscription.matches(&resource_name) {
                    let (done, rx) = ProcessingDone::channel();
                    let event = ResourceEvent::ResourceChanged {
                        result: Ok(Arc::clone(&resource)),
                        done,
                    };
                    // Use backpressure: await if channel is full.
                    // Ignore send errors (watcher dropped).
                    let _ = event_tx.send(event).await;
                    processing_done_futures.push(rx);
                }
            }
        }

        processing_done_futures
    }

    /// Notify watchers of a validation error for a specific resource.
    ///
    /// Per gRFC A46/A88, errors are routed only to watchers interested in
    /// that specific resource (plus wildcard watchers).
    async fn notify_resource_error(&mut self, type_url: &str, resource_name: &str, error: &str) {
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
            let (done, _rx) = ProcessingDone::channel();
            let event = ResourceEvent::ResourceChanged {
                result: Err(Error::Validation(error.to_string())),
                done,
            };
            let _ = event_tx.send(event).await;
        }
    }

    /// Detect resources that were deleted (present in cache but not in response).
    ///
    /// Per gRFC A53, for resource types with ALL_RESOURCES_REQUIRED_IN_SOTW = true,
    /// if a previously-received resource is absent from a new SotW response,
    /// it is treated as deleted.
    async fn detect_deleted_resources(
        &mut self,
        type_url: &str,
        received_names: &HashSet<String>,
    ) -> Vec<oneshot::Receiver<()>> {
        let mut processing_done_futures = Vec::new();

        let type_state = match self.type_states.get_mut(type_url) {
            Some(s) => s,
            None => return processing_done_futures,
        };

        if !type_state.all_resources_required_in_sotw {
            return processing_done_futures;
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
                let (done, rx) = ProcessingDone::channel();
                let event = ResourceEvent::ResourceChanged {
                    result: Err(Error::ResourceDoesNotExist),
                    done,
                };
                let _ = event_tx.send(event).await;
                processing_done_futures.push(rx);
            }
        }

        // Reconcile the resources gauge once from the updated cache.
        let counts = type_state.resource_state_counts();
        self.recorder
            .sync_resource_counts(&type_state.type_url, &counts);

        processing_done_futures
    }

    /// Send an ACK for a response.
    async fn send_ack<S: TransportStream>(
        &self,
        stream: &mut S,
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
        stream.send(bytes).await
    }

    /// Send a NACK for a response.
    async fn send_nack<S: TransportStream>(
        &self,
        stream: &mut S,
        response: &DiscoveryResponse,
        error_message: String,
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
            version_info: &type_state.version_info, // Keep old version for NACK
            response_nonce: &response.nonce,
            error_detail: Some(ErrorDetail {
                code: 3, // INVALID_ARGUMENT
                message: error_message,
            }),
        };

        let bytes = self.codec.encode_request(&request)?;
        stream.send(bytes).await
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
            let (done, _rx) = ProcessingDone::channel();
            let event = ResourceEvent::ResourceChanged {
                result: Err(Error::ResourceDoesNotExist),
                done,
            };
            let _ = event_tx.send(event).await;
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
