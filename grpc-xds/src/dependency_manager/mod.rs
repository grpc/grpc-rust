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

//! The xDS dependency manager (gRFC A74).
//!
//! The dependency manager watches a channel's Listener, RouteConfiguration,
//! Cluster and Endpoints resources (and DNS for LogicalDNS clusters), and
//! publishes an [`XdsConfig`](crate::xds_config::XdsConfig) snapshot whenever
//! a consistent set of them has arrived.
//!
//! [`XdsDependencyManager::start`] spawns a task, the actor, which owns all of
//! the state and sends each update to an [`XdsConfigWatcher`]. Inside:
//!
//! * `state` folds each watch's events into a cached state.
//! * `cluster_graph` walks from the root clusters through aggregate clusters,
//!   and decides what to publish and what to watch.
//! * `watches` owns the watches below the route configuration, and starts and
//!   drops them as the walk requires.
//! * `actor` runs the loop: it applies events in batches, selects the route
//!   configuration's virtual host, and publishes.
//!
//! [`XdsDependencyManager::start`]: crate::dependency_manager::XdsDependencyManager::start
//! [`XdsConfigWatcher`]: crate::dependency_manager::XdsConfigWatcher

mod actor;
mod cluster_graph;
mod state;
mod watches;

use std::fmt;
use std::sync::Arc;

use grpc::__unstable::rt::BoxedTaskHandle;
use grpc::__unstable::rt::GrpcRuntime;
use xds_client::XdsClient;

use crate::xds_config::XdsConfig;

/// What the dependency manager publishes: a complete snapshot, or the reason
/// that there can't be one.
pub(crate) type XdsConfigUpdate = Result<Arc<XdsConfig>, XdsConfigError>;

/// Receives the dependency manager's updates.
pub(crate) trait XdsConfigWatcher: Send + 'static {
    /// Called with each update, in order.
    ///
    /// This runs on the dependency manager's task, so it must not block. It
    /// would typically store the update and schedule work to handle it.
    fn on_update(&mut self, update: XdsConfigUpdate);
}

/// What [`XdsDependencyManager::start`] needs.
// TODO: remove once the xds resolver uses the dependency manager.
#[cfg_attr(not(test), expect(dead_code))]
pub(crate) struct DependencyManagerOptions {
    pub(crate) xds_client: XdsClient,
    /// The name of the channel's Listener resource.
    pub(crate) listener_name: String,
    /// The channel's data-plane authority, which selects the virtual host.
    pub(crate) data_plane_authority: String,
    /// The xDS node ID. It is added to every [`XdsConfigError`], and is the
    /// resolution note of a leaf cluster that has no other note.
    pub(crate) node_id: Option<Arc<str>>,
    /// Where the dependency manager's task runs.
    pub(crate) runtime: GrpcRuntime,
    pub(crate) watcher: Box<dyn XdsConfigWatcher>,
}

impl fmt::Debug for DependencyManagerOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependencyManagerOptions")
            .field("listener_name", &self.listener_name)
            .field("data_plane_authority", &self.data_plane_authority)
            .field("node_id", &self.node_id)
            .finish_non_exhaustive()
    }
}

/// A running dependency manager. Dropping it stops the dependency manager,
/// and with it every watch.
pub(crate) struct XdsDependencyManager {
    task: BoxedTaskHandle,
}

impl XdsDependencyManager {
    /// Starts watching the Listener, on a new task on `options.runtime`.
    // TODO: remove once the xds resolver uses the dependency manager.
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn start(options: DependencyManagerOptions) -> Self {
        let runtime = options.runtime.clone();
        let task = runtime.spawn(Box::pin(actor::run(options)));
        Self { task }
    }
}

impl Drop for XdsDependencyManager {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl fmt::Debug for XdsDependencyManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdsDependencyManager")
            .finish_non_exhaustive()
    }
}

/// Why the dependency manager can't publish a snapshot.
///
/// Problems below the virtual host, such as a missing cluster, don't cause
/// this error: they are reported inside the snapshot (gRFC A74).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct XdsConfigError {
    pub(crate) kind: XdsConfigErrorKind,
    /// Shown in the message, so that the client can be found in control plane
    /// logs.
    node_id: Option<Arc<str>>,
}

/// The kinds of [`XdsConfigError`]: the top-level errors in gRFC A74.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum XdsConfigErrorKind {
    /// The Listener does not exist, or is invalid and no valid version of it
    /// has been received.
    Listener { name: String, reason: Arc<str> },
    /// The same, for the Listener's RouteConfiguration from RDS.
    RouteConfig { name: String, reason: Arc<str> },
    /// No virtual host matches the data-plane authority.
    NoMatchingVirtualHost {
        route_config: String,
        authority: String,
    },
    /// The XdsClient has stopped, so no more updates will arrive.
    XdsClientClosed,
}

impl fmt::Display for XdsConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(node_id) = &self.node_id {
            write!(f, "[xDS node id: {node_id}]: ")?;
        }
        match &self.kind {
            XdsConfigErrorKind::Listener { name, reason } => {
                write!(f, "LDS resource {name}: {reason}")
            }
            XdsConfigErrorKind::RouteConfig { name, reason } => {
                write!(f, "RDS resource {name}: {reason}")
            }
            XdsConfigErrorKind::NoMatchingVirtualHost {
                route_config,
                authority,
            } => write!(
                f,
                "could not find VirtualHost for {authority} in RouteConfiguration {route_config}"
            ),
            XdsConfigErrorKind::XdsClientClosed => f.write_str("xDS client closed"),
        }
    }
}

impl std::error::Error for XdsConfigError {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::Future;
    use std::time::Duration;

    use tokio::sync::mpsc;
    use xds_client::ClientConfig;
    use xds_client::Node;
    use xds_client::ProstCodec;
    use xds_client::TokioRuntime;
    use xds_test_util::XdsTestControlPlaneService;
    use xds_test_util::config::AdsTypeUrl;
    use xds_test_util::config::build_cla;
    use xds_test_util::config::build_cluster;
    use xds_test_util::config::build_rds_listener;
    use xds_test_util::config::build_route_config_with_clusters;

    use super::*;
    use crate::resource::ClusterResource;
    use crate::resource::EndpointsResource;
    use crate::resource::ListenerResource;
    use crate::resource::RouteConfigResource;
    use crate::transport::GrpcTransportBuilder;

    const LISTENER: &str = "listener";
    const ROUTES: &str = "routes";
    const TIMEOUT: Duration = Duration::from_secs(10);

    struct Sender(mpsc::UnboundedSender<XdsConfigUpdate>);

    impl XdsConfigWatcher for Sender {
        fn on_update(&mut self, update: XdsConfigUpdate) {
            let _ = self.0.send(update);
        }
    }

    fn by_name<M>(names: &[&str], build: impl Fn(&str) -> M) -> HashMap<String, M> {
        names
            .iter()
            .map(|name| ((*name).to_owned(), build(name)))
            .collect()
    }

    fn set_route(control_plane: &XdsTestControlPlaneService, clusters: &[&str]) {
        let clusters: Vec<String> = clusters.iter().map(|c| (*c).to_owned()).collect();
        let route_config = build_route_config_with_clusters(ROUTES, &clusters);
        control_plane.set_xds_config(
            &AdsTypeUrl::Rds,
            HashMap::from([(ROUTES.to_owned(), route_config)]),
        );
    }

    /// A control plane serving a Listener whose route uses `clusters`, and
    /// every cluster that might be needed.
    async fn control_plane(clusters: &[&str]) -> xds_test_util::RunningControlPlane {
        let control_plane = XdsTestControlPlaneService::new()
            .start()
            .await
            .expect("start control plane");
        let service = control_plane.get_service();
        service.set_xds_config(
            &AdsTypeUrl::Lds,
            by_name(&[LISTENER], |name| build_rds_listener(name, ROUTES)),
        );
        set_route(service, clusters);
        let all = ["a", "b"];
        service.set_xds_config(&AdsTypeUrl::Cds, by_name(&all, build_cluster));
        service.set_xds_config(
            &AdsTypeUrl::Eds,
            by_name(&all, |name| {
                build_cla(name, &[("127.0.0.1".to_owned(), 8080)])
            }),
        );
        control_plane
    }

    fn xds_client(control_plane: &xds_test_util::RunningControlPlane) -> XdsClient {
        let config = ClientConfig::new(
            Node::new("test", "0"),
            format!("dns:///{}", control_plane.addr()),
        );
        let transport = GrpcTransportBuilder::new(GrpcRuntime::default());
        XdsClient::builder(config, transport, ProstCodec, TokioRuntime).build()
    }

    fn start(
        xds_client: XdsClient,
    ) -> (
        XdsDependencyManager,
        mpsc::UnboundedReceiver<XdsConfigUpdate>,
    ) {
        let (tx, rx) = mpsc::unbounded_channel();
        let dependency_manager = XdsDependencyManager::start(DependencyManagerOptions {
            xds_client,
            listener_name: LISTENER.to_owned(),
            data_plane_authority: "example.com".to_owned(),
            node_id: None,
            runtime: GrpcRuntime::default(),
            watcher: Box::new(Sender(tx)),
        });
        (dependency_manager, rx)
    }

    async fn with_timeout<T>(future: impl Future<Output = T>) -> T {
        tokio::time::timeout(TIMEOUT, future)
            .await
            .expect("timed out")
    }

    /// Waits for the next snapshot, skipping any that don't have exactly
    /// `clusters`.
    async fn snapshot(
        rx: &mut mpsc::UnboundedReceiver<XdsConfigUpdate>,
        clusters: &[&str],
    ) -> Arc<XdsConfig> {
        with_timeout(async {
            loop {
                let config = rx
                    .recv()
                    .await
                    .expect("dependency manager stopped")
                    .expect("snapshot");
                let mut names: Vec<&str> = config.clusters.keys().map(String::as_str).collect();
                names.sort_unstable();
                if names == clusters {
                    return config;
                }
            }
        })
        .await
    }

    async fn eventually(condition: impl Fn() -> bool) {
        with_timeout(async {
            while !condition() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await;
    }

    #[tokio::test]
    async fn first_update_is_a_complete_snapshot() {
        let control_plane = control_plane(&["a", "b"]).await;
        let (_dependency_manager, mut rx) = start(xds_client(&control_plane));

        let update = with_timeout(rx.recv()).await.expect("an update");
        let config = update.expect("a snapshot");
        assert_eq!(config.route_config.name, ROUTES);
        let mut clusters: Vec<&str> = config.clusters.keys().map(String::as_str).collect();
        clusters.sort_unstable();
        assert_eq!(clusters, ["a", "b"]);
        assert!(config.clusters.values().all(Result::is_ok));
    }

    #[tokio::test]
    async fn route_change_unsubscribes_unused_clusters() {
        let control_plane = control_plane(&["a", "b"]).await;
        let (_dependency_manager, mut rx) = start(xds_client(&control_plane));
        snapshot(&mut rx, &["a", "b"]).await;

        let service = control_plane.get_service();
        set_route(service, &["b"]);
        snapshot(&mut rx, &["b"]).await;
        eventually(|| {
            !service.has_subscriber_for(&AdsTypeUrl::Cds, "a")
                && !service.has_subscriber_for(&AdsTypeUrl::Eds, "a")
        })
        .await;
        assert!(service.has_subscriber_for(&AdsTypeUrl::Cds, "b"));
        assert!(service.has_subscriber_for(&AdsTypeUrl::Eds, "b"));
    }

    #[tokio::test]
    async fn dropping_the_dependency_manager_unsubscribes() {
        let control_plane = control_plane(&["a"]).await;
        let xds_client = xds_client(&control_plane);
        // The XdsClient sends no request for a type once it has no watches, so
        // keep one of each type to see the dependency manager's watches end.
        let _other_watches = (
            xds_client.watch::<ListenerResource>("other").await,
            xds_client.watch::<RouteConfigResource>("other").await,
            xds_client.watch::<ClusterResource>("other").await,
            xds_client.watch::<EndpointsResource>("other").await,
        );
        let (dependency_manager, mut rx) = start(xds_client.clone());
        snapshot(&mut rx, &["a"]).await;

        drop(dependency_manager);
        // The task has stopped once it has dropped the watcher.
        with_timeout(async { while rx.recv().await.is_some() {} }).await;
        eventually(|| {
            [
                (AdsTypeUrl::Lds, LISTENER),
                (AdsTypeUrl::Rds, ROUTES),
                (AdsTypeUrl::Cds, "a"),
                (AdsTypeUrl::Eds, "a"),
            ]
            .iter()
            .all(|(type_url, name)| {
                !control_plane
                    .get_service()
                    .has_subscriber_for(type_url, name)
            })
        })
        .await;
    }
}
