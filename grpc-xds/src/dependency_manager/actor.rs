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

//! The dependency manager's task: it owns every watch, applies their events,
//! and publishes the results.

use std::collections::HashSet;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use xds_client::ProcessingDone;
use xds_client::Resource;
use xds_client::ResourceEvent;
use xds_client::ResourceWatcher;
use xds_client::XdsClient;

use super::DependencyManagerOptions;
use super::XdsConfigError;
use super::XdsConfigErrorKind;
use super::XdsConfigWatcher;
use super::cluster_graph::walk;
use super::state::WatchState;
use super::watches::ClusterGraphEvent;
use super::watches::Watches;
use crate::resource::ListenerResource;
use crate::resource::RouteConfigResource;
use crate::resource::RouteSource;
use crate::routing::find_virtual_host_index;
use crate::xds_config::XdsConfig;

/// Runs a dependency manager until the XdsClient stops.
pub(super) async fn run(options: DependencyManagerOptions) {
    Actor::new(options).await.run_loop().await;
}

/// A watch on a Listener or RouteConfiguration, and its cached state.
#[derive(Debug)]
struct Watched<T: Resource> {
    name: String,
    watcher: ResourceWatcher<T>,
    state: WatchState<Arc<T>>,
}

impl<T: Resource> Watched<T> {
    async fn start(client: &XdsClient, name: String) -> Self {
        Self {
            watcher: client.watch(name.clone()).await,
            name,
            state: WatchState::Pending,
        }
    }

    fn apply(&mut self, event: ResourceEvent<T>) -> ProcessingDone {
        self.state.apply(event, |e| e.to_string().into())
    }

    /// The watch's value, or why there isn't one. `make_error` makes the error
    /// from the watch's name and the reason that it failed.
    fn ready(
        &self,
        make_error: impl FnOnce(String, Arc<str>) -> XdsConfigErrorKind,
    ) -> Result<&Arc<T>, Blocked> {
        match &self.state {
            WatchState::Pending => Err(Blocked::Pending),
            WatchState::Failed(reason) => Err(Blocked::Failed(make_error(
                self.name.clone(),
                reason.clone(),
            ))),
            WatchState::Ready { value, .. } => Ok(value),
        }
    }

    /// The watch's ambient error, if any, labelled for a resolution note.
    fn note(&self, label: &str) -> Option<Arc<str>> {
        match &self.state {
            WatchState::Ready {
                note: Some(note), ..
            } => Some(format!("{label} resource {}: {note}", self.name).into()),
            _ => None,
        }
    }
}

struct Actor {
    client: XdsClient,
    authority: String,
    node_id: Option<Arc<str>>,
    publisher: Publisher,
    listener: Watched<ListenerResource>,
    /// The watch on the RouteConfiguration that the Listener names, if it
    /// names one.
    ///
    /// While the Listener is pending or failed, this keeps whatever it was, so
    /// that a Listener that recovers unchanged needs no new fetch.
    route_config: Option<Watched<RouteConfigResource>>,
    watches: Watches,
}

/// An event from one of the actor's watches.
#[derive(Debug)]
enum WatchEvent {
    Listener(ResourceEvent<ListenerResource>),
    RouteConfig(ResourceEvent<RouteConfigResource>),
    ClusterGraph(ClusterGraphEvent),
}

/// The virtual host selected for the channel's data-plane authority.
#[derive(Debug)]
struct Selection {
    listener: Arc<ListenerResource>,
    route_config: Arc<RouteConfigResource>,
    virtual_host_index: usize,
    /// The clusters that the virtual host's routes use: the roots of the
    /// cluster graph.
    clusters: HashSet<String>,
    /// The Listener's and RouteConfiguration's ambient errors, which are
    /// added to every leaf cluster's resolution note.
    notes: Vec<Arc<str>>,
}

/// Why there is no snapshot to publish.
#[derive(Debug)]
enum Blocked {
    /// Something that the snapshot needs hasn't arrived yet.
    Pending,
    /// A top-level error.
    Failed(XdsConfigErrorKind),
}

/// Publishes updates to the [`XdsConfigWatcher`].
struct Publisher {
    watcher: Box<dyn XdsConfigWatcher>,
    node_id: Option<Arc<str>>,
    /// The last update published, if it was an error. A lasting error is
    /// published only once.
    last_error: Option<XdsConfigError>,
}

impl Publisher {
    fn config(&mut self, config: XdsConfig) {
        self.last_error = None;
        self.watcher.on_update(Ok(Arc::new(config)));
    }

    /// Publishes a top-level error, unless it was the last update published.
    fn error(&mut self, kind: XdsConfigErrorKind) {
        let error = XdsConfigError {
            kind,
            node_id: self.node_id.clone(),
        };
        if self.last_error.as_ref() != Some(&error) {
            self.last_error = Some(error.clone());
            self.watcher.on_update(Err(error));
        }
    }
}

impl Actor {
    async fn new(options: DependencyManagerOptions) -> Self {
        let DependencyManagerOptions {
            xds_client,
            listener_name,
            data_plane_authority,
            node_id,
            runtime: _,
            watcher,
        } = options;
        let listener = Watched::start(&xds_client, listener_name).await;
        Self {
            client: xds_client,
            authority: data_plane_authority,
            publisher: Publisher {
                watcher,
                node_id: node_id.clone(),
                last_error: None,
            },
            node_id,
            listener,
            route_config: None,
            watches: Watches::default(),
        }
    }

    /// Handles events in batches until the XdsClient stops.
    async fn run_loop(mut self) {
        while let Some(events) = poll_fn(|cx| self.poll_batch(cx)).await {
            self.handle_batch(events).await;
        }
        self.publisher.error(XdsConfigErrorKind::XdsClientClosed);
    }

    /// Takes every event that is ready without waiting. Pending until there
    /// is at least one, and `None` once the XdsClient has stopped.
    ///
    /// A batch is handled as one, so a response that changes many resources
    /// causes one update rather than one per resource. Batches are bounded,
    /// because the XdsClient doesn't read the next response until the batch
    /// releases its flow-control tokens.
    fn poll_batch(&mut self, cx: &mut Context<'_>) -> Poll<Option<Vec<WatchEvent>>> {
        let mut batch = Vec::new();
        loop {
            match self.poll_watch_event(cx) {
                Poll::Ready(Some(event)) => batch.push(event),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending if batch.is_empty() => return Poll::Pending,
                Poll::Pending => return Poll::Ready(Some(batch)),
            }
        }
    }

    /// Polls the watches, from the top of the resource tree down. Returns
    /// `None` once a watch has closed, which means that the XdsClient has
    /// stopped.
    fn poll_watch_event(&mut self, cx: &mut Context<'_>) -> Poll<Option<WatchEvent>> {
        if let Poll::Ready(event) = self.listener.watcher.poll_next(cx) {
            return Poll::Ready(event.map(WatchEvent::Listener));
        }
        if let Some(route_config) = &mut self.route_config
            && let Poll::Ready(event) = route_config.watcher.poll_next(cx)
        {
            return Poll::Ready(event.map(WatchEvent::RouteConfig));
        }
        self.watches
            .poll_next(cx)
            .map(|event| event.map(WatchEvent::ClusterGraph))
    }

    /// Applies `event` to the cached states, and returns its flow-control
    /// token, if it has one.
    fn handle_watch_event(&mut self, event: WatchEvent) -> Option<ProcessingDone> {
        match event {
            WatchEvent::Listener(event) => Some(self.listener.apply(event)),
            WatchEvent::RouteConfig(event) => {
                let route_config = self
                    .route_config
                    .as_mut()
                    .expect("RouteConfiguration events come from the RouteConfiguration watch");
                Some(route_config.apply(event))
            }
            WatchEvent::ClusterGraph(event) => self.watches.apply(event),
        }
    }

    /// Applies `events` to the cached states, then updates the watches to
    /// match and publishes the result.
    ///
    /// Everything is derived again from the cached states, whatever the batch
    /// changed. The events' flow-control tokens are held until the result has
    /// been published: the XdsClient reads the next response once they are
    /// dropped (gRFC A88).
    async fn handle_batch(&mut self, events: impl IntoIterator<Item = WatchEvent>) {
        let done: Vec<_> = events
            .into_iter()
            .filter_map(|event| self.handle_watch_event(event))
            .collect();
        match self.resolve().await {
            Ok(config) => self.publisher.config(config),
            Err(Blocked::Pending) => {}
            Err(Blocked::Failed(kind)) => self.publisher.error(kind),
        }
        drop(done);
    }

    /// Resolves the Listener, its RouteConfiguration, the virtual host and
    /// its clusters, in that order, and updates the watches to match.
    async fn resolve(&mut self) -> Result<XdsConfig, Blocked> {
        self.update_route_watch().await;
        // While there is no selection, the cluster watches are kept as they
        // are: they will likely be needed again once the Listener or
        // RouteConfiguration recovers.
        let selection = self.select_route()?;
        self.update_clusters(selection).await
    }

    /// Watches the RouteConfiguration that the Listener names.
    ///
    /// While the Listener is pending or failed, the watch is kept as it is.
    async fn update_route_watch(&mut self) {
        let WatchState::Ready { value, .. } = &self.listener.state else {
            return;
        };
        match &value.route_source {
            RouteSource::Rds(name) => {
                if self
                    .route_config
                    .as_ref()
                    .is_none_or(|route_config| route_config.name != *name)
                {
                    // The new watch starts before the old one is dropped, so
                    // that the RDS subscription is never empty.
                    self.route_config = Some(Watched::start(&self.client, name.clone()).await);
                }
            }
            RouteSource::Inline(_) => self.route_config = None,
        }
    }

    /// Selects the virtual host for the data-plane authority.
    fn select_route(&self) -> Result<Selection, Blocked> {
        let listener = self
            .listener
            .ready(|name, reason| XdsConfigErrorKind::Listener { name, reason })?;
        let route_config = match &listener.route_source {
            RouteSource::Inline(route_config) => route_config,
            RouteSource::Rds(name) => self
                .route_config
                .as_ref()
                .filter(|route_config| route_config.name == *name)
                .expect("update_route_watch watches the RouteConfiguration that the Listener names")
                .ready(|name, reason| XdsConfigErrorKind::RouteConfig { name, reason })?,
        };
        let virtual_host_index =
            find_virtual_host_index(&self.authority, &route_config.virtual_hosts).ok_or_else(
                || {
                    Blocked::Failed(XdsConfigErrorKind::NoMatchingVirtualHost {
                        route_config: route_config.name.clone(),
                        authority: self.authority.clone(),
                    })
                },
            )?;
        let notes = self
            .listener
            .note("LDS")
            .into_iter()
            .chain(self.route_config.as_ref().and_then(|rds| rds.note("RDS")))
            .collect();
        Ok(Selection {
            listener: listener.clone(),
            route_config: route_config.clone(),
            virtual_host_index,
            clusters: route_config.virtual_hosts[virtual_host_index].cluster_names(),
            notes,
        })
    }

    /// Walks the cluster graph from the selected virtual host, updates the
    /// watches to match it, and returns the snapshot once every cluster has
    /// resolved.
    async fn update_clusters(&mut self, selection: Selection) -> Result<XdsConfig, Blocked> {
        let graph = walk(
            &self.watches,
            selection.clusters.iter().map(String::as_str),
            &selection.notes,
            self.node_id.as_deref(),
        );
        self.watches
            .reconcile(&self.client, &graph.required_watches)
            .await;

        // A cluster is missing while it or anything below it is pending.
        if !selection
            .clusters
            .iter()
            .all(|name| graph.cluster_configs.contains_key(name))
        {
            return Err(Blocked::Pending);
        }
        Ok(XdsConfig::try_new(
            selection.listener,
            selection.route_config,
            selection.virtual_host_index,
            graph.cluster_configs,
        )
        .expect("the selection's route configuration and virtual host are the Listener's"))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use grpc::__unstable::rt::GrpcRuntime;

    use super::*;
    use crate::dependency_manager::XdsConfigUpdate;
    use crate::dependency_manager::cluster_graph::DnsTarget;
    use crate::resource::ClusterDiscovery;
    use crate::resource::ClusterResource;
    use crate::resource::DomainMatcher;
    use crate::resource::EndpointsResource;
    use crate::resource::PathSpecifier;
    use crate::resource::Route;
    use crate::resource::RouteAction;
    use crate::resource::RouteMatch;
    use crate::resource::VirtualHost;
    use crate::xds_config::ClusterChildren;

    const LISTENER: &str = "listener";
    const AUTHORITY: &str = "example.com";

    struct Sender(mpsc::Sender<XdsConfigUpdate>);

    impl XdsConfigWatcher for Sender {
        fn on_update(&mut self, update: XdsConfigUpdate) {
            let _ = self.0.send(update);
        }
    }

    /// Drives an actor whose watches never deliver anything, by applying
    /// events to it directly.
    struct Harness {
        actor: Actor,
        updates: mpsc::Receiver<XdsConfigUpdate>,
    }

    impl Harness {
        async fn new() -> Self {
            let (options, updates) = options();
            let actor = Actor::new(options).await;
            Self { actor, updates }
        }

        /// Handles `events` as one batch, and returns what it published.
        async fn batch(
            &mut self,
            events: impl IntoIterator<Item = WatchEvent>,
        ) -> Vec<XdsConfigUpdate> {
            self.actor.handle_batch(events).await;
            self.updates.try_iter().collect()
        }

        /// Handles `events` as one batch, which must publish a snapshot.
        async fn config(&mut self, events: impl IntoIterator<Item = WatchEvent>) -> Arc<XdsConfig> {
            match <[_; 1]>::try_from(self.batch(events).await) {
                Ok([Ok(config)]) => config,
                updates => panic!("expected one snapshot, got {updates:?}"),
            }
        }

        /// Handles `events` as one batch, which must publish an error.
        async fn error(&mut self, events: impl IntoIterator<Item = WatchEvent>) -> XdsConfigError {
            match <[_; 1]>::try_from(self.batch(events).await) {
                Ok([Err(error)]) => error,
                updates => panic!("expected one error, got {updates:?}"),
            }
        }

        /// Sets up a Listener with RouteConfiguration `routes`, whose EDS
        /// clusters have all resolved.
        async fn resolved(clusters: &[&str]) -> Self {
            let mut harness = Self::new().await;
            harness.batch([lds_rds("routes")]).await;
            harness.batch([rds("routes", clusters)]).await;
            harness.batch(clusters.iter().map(|c| cds_eds(c))).await;
            harness
                .config(clusters.iter().map(|c| eds(&format!("{c}-eds"))))
                .await;
            harness
        }

        fn watched_clusters(&self) -> Vec<&str> {
            sorted(self.actor.watches.cluster_names())
        }

        fn watched_endpoints(&self) -> Vec<&str> {
            sorted(self.actor.watches.endpoint_names())
        }

        fn route_config_watch(&self) -> Option<&str> {
            self.actor
                .route_config
                .as_ref()
                .map(|rds| rds.name.as_str())
        }
    }

    /// Options for an actor with a disconnected XdsClient, and the receiver
    /// of what the actor publishes.
    fn options() -> (DependencyManagerOptions, mpsc::Receiver<XdsConfigUpdate>) {
        let (tx, rx) = mpsc::channel();
        let options = DependencyManagerOptions {
            xds_client: XdsClient::disconnected(),
            listener_name: LISTENER.to_owned(),
            data_plane_authority: AUTHORITY.to_owned(),
            node_id: Some("node-1".into()),
            runtime: GrpcRuntime::default(),
            watcher: Box::new(Sender(tx)),
        };
        (options, rx)
    }

    fn sorted<'a>(names: impl Iterator<Item = &'a String>) -> Vec<&'a str> {
        let mut names: Vec<&str> = names.map(String::as_str).collect();
        names.sort_unstable();
        names
    }

    fn changed<T>(value: T) -> ResourceEvent<T> {
        ResourceEvent::ResourceChanged {
            result: Ok(Arc::new(value)),
            done: ProcessingDone::detached(),
        }
    }

    fn does_not_exist<T>() -> ResourceEvent<T> {
        ResourceEvent::ResourceChanged {
            result: Err(xds_client::Error::ResourceDoesNotExist),
            done: ProcessingDone::detached(),
        }
    }

    fn ambient<T>(reason: &str) -> ResourceEvent<T> {
        ResourceEvent::AmbientError {
            error: xds_client::Error::Connection(reason.to_owned()),
            done: ProcessingDone::detached(),
        }
    }

    fn virtual_host(domain: &str, clusters: &[&str]) -> VirtualHost {
        VirtualHost {
            name: domain.to_owned(),
            domains: vec![DomainMatcher::try_from(domain).unwrap()],
            routes: clusters
                .iter()
                .map(|cluster| Route {
                    route_match: RouteMatch {
                        path_specifier: PathSpecifier::Prefix(String::new()),
                        headers: Vec::new(),
                        case_sensitive: true,
                        match_fraction: None,
                    },
                    action: RouteAction::Cluster((*cluster).to_owned()),
                })
                .collect(),
        }
    }

    fn route_config(name: &str, clusters: &[&str]) -> RouteConfigResource {
        RouteConfigResource {
            name: name.to_owned(),
            virtual_hosts: vec![virtual_host("*", clusters)],
        }
    }

    fn listener(route_source: RouteSource) -> WatchEvent {
        WatchEvent::Listener(changed(ListenerResource {
            name: LISTENER.to_owned(),
            route_source,
        }))
    }

    fn lds_rds(route_config: &str) -> WatchEvent {
        listener(RouteSource::Rds(route_config.to_owned()))
    }

    fn lds_inline(route_config: RouteConfigResource) -> WatchEvent {
        listener(RouteSource::Inline(Arc::new(route_config)))
    }

    fn rds(name: &str, clusters: &[&str]) -> WatchEvent {
        WatchEvent::RouteConfig(changed(route_config(name, clusters)))
    }

    fn cds(name: &str, discovery: ClusterDiscovery) -> WatchEvent {
        let cluster = ClusterResource {
            name: name.to_owned(),
            discovery,
        };
        WatchEvent::ClusterGraph(ClusterGraphEvent::Cluster(
            name.to_owned(),
            changed(cluster),
        ))
    }

    /// An EDS cluster, whose EDS service is the cluster's name plus "-eds".
    fn cds_eds(name: &str) -> WatchEvent {
        let eds_service_name = format!("{name}-eds");
        cds(name, ClusterDiscovery::Eds { eds_service_name })
    }

    fn eds(service: &str) -> WatchEvent {
        let endpoints = EndpointsResource {
            cluster_name: service.to_owned(),
            localities: Vec::new(),
        };
        WatchEvent::ClusterGraph(ClusterGraphEvent::Endpoints(
            service.to_owned(),
            changed(endpoints),
        ))
    }

    fn cluster_names(config: &XdsConfig) -> Vec<&str> {
        sorted(config.clusters.keys())
    }

    fn leaf_note<'a>(config: &'a XdsConfig, cluster: &str) -> Option<&'a str> {
        let cluster = config.clusters[cluster].as_ref().expect("resolved cluster");
        let ClusterChildren::Leaf(leaf) = &cluster.children else {
            panic!("expected a leaf cluster");
        };
        leaf.resolution_note.as_deref()
    }

    #[tokio::test]
    async fn publishes_once_every_cluster_resolves() {
        let mut harness = Harness::new().await;
        assert!(harness.batch([lds_rds("routes")]).await.is_empty());
        assert_eq!(harness.route_config_watch(), Some("routes"));

        assert!(harness.batch([rds("routes", &["a", "b"])]).await.is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "b"]);

        assert!(harness.batch([cds_eds("a"), cds_eds("b")]).await.is_empty());
        assert_eq!(harness.watched_endpoints(), ["a-eds", "b-eds"]);

        assert!(harness.batch([eds("a-eds")]).await.is_empty());
        let config = harness.config([eds("b-eds")]).await;
        assert_eq!(config.listener.name, LISTENER);
        assert_eq!(config.route_config.name, "routes");
        assert_eq!(cluster_names(&config), ["a", "b"]);
        assert!(config.clusters.values().all(Result::is_ok));
    }

    #[tokio::test]
    async fn route_without_clusters_publishes_straight_away() {
        let mut harness = Harness::new().await;
        let config = harness
            .config([lds_inline(route_config("inline", &[]))])
            .await;
        assert!(config.clusters.is_empty());
    }

    #[tokio::test]
    async fn top_level_errors() {
        let no_virtual_host = RouteConfigResource {
            name: "inline".to_owned(),
            virtual_hosts: vec![virtual_host("other.com", &["a"])],
        };
        // Each case is a batch that must publish nothing, then one that must
        // publish the error.
        let cases = [
            (
                vec![],
                vec![WatchEvent::Listener(does_not_exist())],
                "[xDS node id: node-1]: LDS resource listener: resource does not exist",
            ),
            (
                vec![lds_rds("routes")],
                vec![WatchEvent::RouteConfig(does_not_exist())],
                "[xDS node id: node-1]: RDS resource routes: resource does not exist",
            ),
            (
                vec![],
                vec![lds_inline(no_virtual_host)],
                "[xDS node id: node-1]: could not find VirtualHost for example.com in \
                 RouteConfiguration inline",
            ),
        ];
        for (setup, events, expected) in cases {
            let mut harness = Harness::new().await;
            assert!(harness.batch(setup).await.is_empty());
            let error = harness.error(events).await;
            assert_eq!(error.to_string(), expected);
            assert!(harness.watched_clusters().is_empty());
        }
    }

    #[tokio::test]
    async fn lasting_error_is_published_once() {
        let mut harness = Harness::new().await;
        harness
            .error([WatchEvent::Listener(does_not_exist())])
            .await;
        assert!(
            harness
                .batch([WatchEvent::Listener(does_not_exist())])
                .await
                .is_empty()
        );

        harness
            .config([lds_inline(route_config("inline", &[]))])
            .await;
        harness
            .error([WatchEvent::Listener(does_not_exist())])
            .await;
    }

    #[tokio::test]
    async fn route_config_rename_waits_for_the_new_route_config() {
        let mut harness = Harness::resolved(&["a"]).await;

        assert!(harness.batch([lds_rds("routes-2")]).await.is_empty());
        assert_eq!(harness.route_config_watch(), Some("routes-2"));
        assert_eq!(harness.watched_clusters(), ["a"]);

        // Cluster "a" is still cached, so the snapshot needs no new fetch.
        let config = harness.config([rds("routes-2", &["a"])]).await;
        assert_eq!(config.route_config.name, "routes-2");
        assert!(config.clusters["a"].is_ok());
    }

    #[tokio::test]
    async fn listener_error_keeps_the_watches() {
        let mut harness = Harness::resolved(&["a"]).await;

        let error = harness
            .error([WatchEvent::Listener(does_not_exist())])
            .await;
        assert!(matches!(error.kind, XdsConfigErrorKind::Listener { .. }));
        assert_eq!(harness.route_config_watch(), Some("routes"));
        assert_eq!(harness.watched_clusters(), ["a"]);
        assert_eq!(harness.watched_endpoints(), ["a-eds"]);
        // A cluster event can't publish anything until the Listener recovers.
        assert!(harness.batch([eds("a-eds")]).await.is_empty());

        let config = harness.config([lds_rds("routes")]).await;
        assert!(config.clusters["a"].is_ok());
    }

    #[tokio::test]
    async fn inline_route_config_drops_the_rds_watch() {
        let mut harness = Harness::resolved(&["a"]).await;
        let config = harness
            .config([lds_inline(route_config("inline", &["a"]))])
            .await;
        assert_eq!(config.route_config.name, "inline");
        assert_eq!(harness.route_config_watch(), None);
    }

    #[tokio::test]
    async fn route_change_drops_unused_watches() {
        let mut harness = Harness::resolved(&["a", "b"]).await;
        let config = harness.config([rds("routes", &["b"])]).await;
        assert_eq!(cluster_names(&config), ["b"]);
        assert_eq!(harness.watched_clusters(), ["b"]);
        assert_eq!(harness.watched_endpoints(), ["b-eds"]);
    }

    #[tokio::test]
    async fn logical_dns_cluster_is_unsupported() {
        let mut harness = Harness::new().await;
        harness
            .batch([lds_inline(route_config("inline", &["dns"]))])
            .await;
        let discovery = ClusterDiscovery::LogicalDns {
            hostname: "dns.example.com".to_owned(),
            port: 443,
        };
        assert!(harness.batch([cds("dns", discovery)]).await.is_empty());

        let target = DnsTarget {
            host: "dns.example.com".to_owned(),
            port: 443,
        };
        let config = harness
            .config([WatchEvent::ClusterGraph(ClusterGraphEvent::Dns(target))])
            .await;
        assert_eq!(
            leaf_note(&config, "dns"),
            Some("LogicalDNS clusters are not supported yet")
        );
    }

    #[tokio::test]
    async fn ambient_errors_become_resolution_notes() {
        let mut harness = Harness::new().await;
        harness.batch([lds_rds("routes")]).await;
        harness.batch([rds("routes", &["a"])]).await;
        harness.batch([cds_eds("a")]).await;
        let config = harness.config([eds("a-eds")]).await;
        // With no other note, a leaf's note is the node ID.
        assert_eq!(leaf_note(&config, "a"), Some("xDS node ID:node-1"));

        let config = harness
            .config([
                WatchEvent::Listener(ambient("lds down")),
                WatchEvent::RouteConfig(ambient("rds down")),
                WatchEvent::ClusterGraph(ClusterGraphEvent::Cluster(
                    "a".to_owned(),
                    ambient("cds down"),
                )),
            ])
            .await;
        assert_eq!(
            leaf_note(&config, "a"),
            Some(
                "LDS resource listener: failed to connect: lds down; \
                 RDS resource routes: failed to connect: rds down; \
                 CDS resource a: failed to connect: cds down"
            )
        );
    }

    #[tokio::test]
    async fn cluster_error_is_reported_in_the_snapshot() {
        let mut harness = Harness::new().await;
        harness
            .batch([lds_inline(route_config("inline", &["a"]))])
            .await;
        let failure = ClusterGraphEvent::Cluster("a".to_owned(), does_not_exist());
        let config = harness.config([WatchEvent::ClusterGraph(failure)]).await;
        let error = config.clusters["a"].as_ref().unwrap_err();
        assert_eq!(error.message(), "CDS resource a: resource does not exist");
    }

    #[tokio::test]
    async fn stops_once_the_xds_client_stops() {
        // A disconnected XdsClient's watches close straight away.
        let (options, updates) = options();
        run(options).await;
        let updates: Vec<_> = updates.try_iter().collect();
        let [Err(error)] = updates.as_slice() else {
            panic!("expected one error, got {updates:?}");
        };
        assert_eq!(error.kind, XdsConfigErrorKind::XdsClientClosed);
        assert_eq!(
            error.to_string(),
            "[xDS node id: node-1]: xDS client closed"
        );
    }
}
