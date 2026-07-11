//! A fake xDS ADS control plane for tests.
//!
//! Rust port of grpc-java's `XdsTestControlPlaneService`. It is a bidi-stream
//! service that acts as a local xDS control plane. Config is injected through
//! [`set_xds_config`](XdsTestControlPlaneService::set_xds_config).
//!
//! The service maintains, per ADS resource type:
//! - a resources table (resource name to packed protobuf `Any`),
//! - a subscriber table (each active stream to its subscribed resource names),
//! - a version counter (bumped on every config set), and
//! - a per-stream nonce counter.
//!
//! All resource types are treated as state-of-the-world: whenever any resource
//! of a type changes, every subscriber to that type receives all of its
//! subscribed resources of that type.
//!
//! Incoming ADS requests share the same proto message but represent different
//! phases, which the service distinguishes:
//! 1. an initial request (new or changed subscription) — answered with a response,
//! 2. a NACK (carries `error_detail`) — logged and ignored,
//! 3. an ACK (same resource names already subscribed) — ignored.

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use envoy_types::pb::envoy::service::discovery::v3::aggregated_discovery_service_server::AggregatedDiscoveryServiceServer;
use envoy_types::pb::envoy::service::discovery::v3::{DiscoveryRequest, DiscoveryResponse};
use envoy_types::pb::google::protobuf::Any;
use prost::Message;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

/// ADS type URL for LDS (`Listener`) resources.
pub const ADS_TYPE_URL_LDS: &str = "type.googleapis.com/envoy.config.listener.v3.Listener";
/// ADS type URL for RDS (`RouteConfiguration`) resources.
pub const ADS_TYPE_URL_RDS: &str = "type.googleapis.com/envoy.config.route.v3.RouteConfiguration";
/// ADS type URL for CDS (`Cluster`) resources.
pub const ADS_TYPE_URL_CDS: &str = "type.googleapis.com/envoy.config.cluster.v3.Cluster";
/// ADS type URL for EDS (`ClusterLoadAssignment`) resources.
pub const ADS_TYPE_URL_EDS: &str =
    "type.googleapis.com/envoy.config.endpoint.v3.ClusterLoadAssignment";

/// The ADS resource types tracked by the control plane.
const ADS_TYPE_URLS: [&str; 4] = [
    ADS_TYPE_URL_LDS,
    ADS_TYPE_URL_RDS,
    ADS_TYPE_URL_CDS,
    ADS_TYPE_URL_EDS,
];

/// Sender half of a stream's outbound `DiscoveryResponse` channel.
///
/// The channel carries bare `DiscoveryResponse`s; the (framework-specific)
/// serving adapter wraps them in the transport's success type.
type ResponseSender = mpsc::UnboundedSender<DiscoveryResponse>;

/// Per-(type, stream) subscription bookkeeping.
#[derive(Debug)]
struct Subscription {
    /// Outbound channel used to push responses to this stream.
    sender: ResponseSender,
    /// Resource names currently subscribed for this type on this stream.
    resource_names: HashSet<String>,
    /// Last nonce sent to this stream for this type (starts at 0).
    nonce: u64,
}

/// Mutable control-plane state, guarded by a single mutex.
#[derive(Debug)]
struct State {
    /// `type_url` to (resource name to packed `Any`).
    resources: HashMap<String, HashMap<String, Any>>,
    /// `type_url` to latest version (starts at 1, bumped on each config set).
    versions: HashMap<String, u64>,
    /// `type_url` to (stream id to subscription).
    subscribers: HashMap<String, HashMap<u64, Subscription>>,
}

impl State {
    fn new() -> Self {
        let mut versions = HashMap::new();
        let mut subscribers = HashMap::new();
        for type_url in ADS_TYPE_URLS {
            versions.insert(type_url.to_string(), 1);
            subscribers.insert(type_url.to_string(), HashMap::new());
        }
        Self {
            resources: HashMap::new(),
            versions,
            subscribers,
        }
    }
}

/// Shared inner state of the control plane.
#[derive(Debug)]
struct Inner {
    state: Mutex<State>,
    next_stream_id: AtomicU64,
}

impl Inner {
    /// Handles a single inbound `DiscoveryRequest` for a stream.
    fn handle_request(&self, stream_id: u64, tx: &ResponseSender, req: DiscoveryRequest) {
        let mut state = self.state.lock().expect("control plane state poisoned");

        // NACK: a request carrying an error detail rejects the last response.
        if req.error_detail.is_some() {
            return;
        }

        let type_url = req.type_url;
        if type_url.is_empty() {
            return;
        }

        // Nonce check: if the request carries a response nonce, it must match
        // the last nonce we sent on this (type, stream); otherwise ignore it.
        if !req.response_nonce.is_empty() {
            let matches = state
                .subscribers
                .get(&type_url)
                .and_then(|streams| streams.get(&stream_id))
                .is_some_and(|sub| sub.nonce.to_string() == req.response_nonce);
            if !matches {
                return;
            }
        }

        let requested: HashSet<String> = req.resource_names.into_iter().collect();

        // ACK: identical subscription already recorded, nothing to send.
        if let Some(sub) = state
            .subscribers
            .get(&type_url)
            .and_then(|streams| streams.get(&stream_id))
            && sub.resource_names == requested
        {
            return;
        }

        let version = state.versions.get(&type_url).copied().unwrap_or(1);

        // Borrow the resource and subscriber tables as disjoint fields so the
        // response can be built from `resources` while mutating `subscribers`.
        let State {
            resources,
            subscribers,
            ..
        } = &mut *state;
        let type_resources = resources.get(&type_url);
        let sub = subscribers
            .entry(type_url.clone())
            .or_default()
            .entry(stream_id)
            .or_insert_with(|| Subscription {
                sender: tx.clone(),
                resource_names: HashSet::new(),
                nonce: 0,
            });
        sub.nonce += 1;
        let response = build_response(&type_url, version, sub.nonce, &requested, type_resources);
        let _ = sub.sender.send(response);
        sub.resource_names = requested;
    }

    /// Removes all subscriptions for a stream when it ends.
    fn remove_stream(&self, stream_id: u64) {
        let mut state = self.state.lock().expect("control plane state poisoned");
        for streams in state.subscribers.values_mut() {
            streams.remove(&stream_id);
        }
    }
}

/// A bidi-stream service that acts as a local xDS control plane for tests.
///
/// Clone freely: all clones share the same underlying state, so config injected
/// through one clone is served by another. Inject config with
/// [`set_xds_config`](Self::set_xds_config) and retain a clone to push updates
/// while the server runs.
///
/// The service implements the ADS server trait and can be registered with a
/// tonic server via [`AggregatedDiscoveryServiceServer`](crate::AggregatedDiscoveryServiceServer).
#[derive(Clone, Debug)]
pub struct XdsTestControlPlaneService {
    inner: Arc<Inner>,
}

impl Default for XdsTestControlPlaneService {
    fn default() -> Self {
        Self::new()
    }
}

impl XdsTestControlPlaneService {
    /// Creates a new control plane with no resources configured.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                state: Mutex::new(State::new()),
                next_stream_id: AtomicU64::new(0),
            }),
        }
    }

    /// Sets the full set of resources for `type_url`, replacing any previous
    /// resources of that type, and pushes an update to every current subscriber.
    ///
    /// Each resource is packed into a protobuf `Any` tagged with `type_url`.
    /// Resource types are treated as state-of-the-world.
    pub fn set_xds_config<M: Message>(&self, type_url: &str, resources: HashMap<String, M>) {
        let packed: HashMap<String, Any> = resources
            .into_iter()
            .map(|(name, msg)| {
                (
                    name,
                    Any {
                        type_url: type_url.to_string(),
                        value: msg.encode_to_vec(),
                    },
                )
            })
            .collect();

        let mut state = self.inner.state.lock().expect("control plane state poisoned");
        state.resources.insert(type_url.to_string(), packed);

        // getAndIncrement: the pushed response carries the pre-increment version.
        let version = {
            let counter = state.versions.entry(type_url.to_string()).or_insert(1);
            let current = *counter;
            *counter += 1;
            current
        };

        let State {
            resources,
            subscribers,
            ..
        } = &mut *state;
        let type_resources = resources.get(type_url);
        if let Some(streams) = subscribers.get_mut(type_url) {
            for sub in streams.values_mut() {
                sub.nonce += 1;
                let response =
                    build_response(type_url, version, sub.nonce, &sub.resource_names, type_resources);
                let _ = sub.sender.send(response);
            }
        }
    }

    /// Returns the currently configured resources for `type_url`, as packed
    /// protobuf `Any`s keyed by resource name.
    ///
    /// For a protobuf-free view suitable for assertions, use
    /// [`resource_names`](Self::resource_names).
    #[must_use]
    pub fn get_current_config(&self, type_url: &str) -> HashMap<String, Any> {
        let state = self.inner.state.lock().expect("control plane state poisoned");
        state.resources.get(type_url).cloned().unwrap_or_default()
    }

    /// Returns the names of the currently configured resources for `type_url`,
    /// sorted.
    ///
    /// This is a protocol-agnostic view of what the control plane is serving,
    /// suitable for assertions without depending on protobuf types.
    #[must_use]
    pub fn resource_names(&self, type_url: &str) -> Vec<String> {
        let state = self.inner.state.lock().expect("control plane state poisoned");
        state
            .resources
            .get(type_url)
            .map(|resources| {
                let mut names: Vec<String> = resources.keys().cloned().collect();
                names.sort();
                names
            })
            .unwrap_or_default()
    }

    /// Returns the number of active subscribers per ADS resource type.
    #[must_use]
    pub fn get_subscriber_counts(&self) -> HashMap<String, usize> {
        let state = self.inner.state.lock().expect("control plane state poisoned");
        ADS_TYPE_URLS
            .iter()
            .map(|type_url| {
                (
                    (*type_url).to_string(),
                    state.subscribers.get(*type_url).map_or(0, HashMap::len),
                )
            })
            .collect()
    }

    /// Serves the ADS control plane on an ephemeral `127.0.0.1` port in a
    /// background task.
    ///
    /// Returns a [`RunningControlPlane`] that exposes the bound address via
    /// [`addr`](RunningControlPlane::addr), derefs to this service so config
    /// setters can be called on it directly, and shuts the server down when
    /// dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if binding the ephemeral port fails.
    pub async fn start(&self) -> std::io::Result<RunningControlPlane> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let service = self.clone();
        let handle = tokio::spawn(
            Server::builder()
                .add_service(AggregatedDiscoveryServiceServer::new(service.clone()))
                .serve_with_incoming(TcpListenerStream::new(listener)),
        );
        Ok(RunningControlPlane {
            service,
            addr,
            handle,
        })
    }
}

/// A running [`XdsTestControlPlaneService`] returned by
/// [`XdsTestControlPlaneService::start`].
///
/// Derefs to the underlying control plane, so config setters (e.g.
/// [`set_xds_config`](XdsTestControlPlaneService::set_xds_config)) can be called
/// on it directly. The server task is aborted when this handle is dropped.
#[derive(Debug)]
pub struct RunningControlPlane {
    service: XdsTestControlPlaneService,
    addr: SocketAddr,
    handle: JoinHandle<Result<(), tonic::transport::Error>>,
}

impl RunningControlPlane {
    /// The address the ADS server is listening on.
    #[must_use]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// The underlying control plane service (shared state).
    #[must_use]
    pub fn service(&self) -> &XdsTestControlPlaneService {
        &self.service
    }
}

impl std::ops::Deref for RunningControlPlane {
    type Target = XdsTestControlPlaneService;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl Drop for RunningControlPlane {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

/// Tonic serving adapter for the ADS bidi stream.
mod tonic_service {
    use std::pin::Pin;
    use std::sync::atomic::Ordering;

    use envoy_types::pb::envoy::service::discovery::v3::{
        DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
        aggregated_discovery_service_server::AggregatedDiscoveryService,
    };
    use tokio_stream::wrappers::UnboundedReceiverStream;
    use tokio_stream::{Stream, StreamExt};
    use tonic::{Request, Response, Status};

    use super::{Arc, XdsTestControlPlaneService, mpsc};

    #[tonic::async_trait]
    impl AggregatedDiscoveryService for XdsTestControlPlaneService {
        type StreamAggregatedResourcesStream =
            Pin<Box<dyn Stream<Item = Result<DiscoveryResponse, Status>> + Send>>;

        async fn stream_aggregated_resources(
            &self,
            request: Request<tonic::Streaming<DiscoveryRequest>>,
        ) -> Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
            let mut inbound = request.into_inner();
            let (tx, rx) = mpsc::unbounded_channel::<DiscoveryResponse>();
            let inner = Arc::clone(&self.inner);
            let stream_id = inner.next_stream_id.fetch_add(1, Ordering::Relaxed);

            tokio::spawn(async move {
                while let Some(item) = inbound.next().await {
                    match item {
                        Ok(req) => inner.handle_request(stream_id, &tx, req),
                        Err(_) => break,
                    }
                }
                inner.remove_stream(stream_id);
            });

            let outbound = UnboundedReceiverStream::new(rx).map(Ok);
            Ok(Response::new(Box::pin(outbound)))
        }

        type DeltaAggregatedResourcesStream =
            Pin<Box<dyn Stream<Item = Result<DeltaDiscoveryResponse, Status>> + Send>>;

        async fn delta_aggregated_resources(
            &self,
            _request: Request<tonic::Streaming<DeltaDiscoveryRequest>>,
        ) -> Result<Response<Self::DeltaAggregatedResourcesStream>, Status> {
            Err(Status::unimplemented(
                "delta ADS is not supported by the test control plane",
            ))
        }
    }
}

/// Builds a `DiscoveryResponse` containing the requested resources that exist.
fn build_response(
    type_url: &str,
    version: u64,
    nonce: u64,
    resource_names: &HashSet<String>,
    type_resources: Option<&HashMap<String, Any>>,
) -> DiscoveryResponse {
    let resources = match type_resources {
        Some(map) => resource_names
            .iter()
            .filter_map(|name| map.get(name).cloned())
            .collect(),
        None => Vec::new(),
    };
    DiscoveryResponse {
        version_info: version.to_string(),
        resources,
        type_url: type_url.to_string(),
        nonce: nonce.to_string(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use envoy_types::pb::envoy::config::listener::v3::Listener;

    #[test]
    fn set_and_get_config() {
        let control_plane = XdsTestControlPlaneService::new();

        let mut listeners = HashMap::new();
        listeners.insert(
            "my-listener".to_string(),
            Listener {
                name: "my-listener".to_string(),
                ..Default::default()
            },
        );
        control_plane.set_xds_config(ADS_TYPE_URL_LDS, listeners);

        let config = control_plane.get_current_config(ADS_TYPE_URL_LDS);
        assert_eq!(config.len(), 1);
        let packed = config.get("my-listener").expect("listener present");
        assert_eq!(packed.type_url, ADS_TYPE_URL_LDS);

        // No streams have connected yet.
        let counts = control_plane.get_subscriber_counts();
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.get(ADS_TYPE_URL_LDS), Some(&0));

        // A type that was never configured has no resources.
        assert!(control_plane.get_current_config(ADS_TYPE_URL_CDS).is_empty());
    }
}
