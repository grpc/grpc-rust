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
use super::subscriptions::SubscriptionWatch;
use super::watches::ClusterGraphEvent;
use super::watches::Watches;
use crate::resource::ListenerResource;
use crate::resource::RouteConfigResource;
use crate::resource::RouteSource;
use crate::routing::find_virtual_host_index;
use crate::xds_config::XdsConfig;

/// Runs a dependency manager until the XdsClient stops.
pub(super) async fn run(options: DependencyManagerOptions, subscriptions: SubscriptionWatch) {
    Actor::new(options, subscriptions).run_loop().await;
}

/// A watch on a Listener or RouteConfiguration, and its cached state.
#[derive(Debug)]
struct Watched<T: Resource> {
    name: String,
    watcher: ResourceWatcher<T>,
    state: WatchState<T>,
}

impl<T: Resource> Watched<T> {
    fn start(client: &XdsClient, name: String) -> Self {
        Self {
            watcher: client.watch(name.clone()),
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
    /// The clusters that the selected virtual host's routes use, as of the
    /// last [`Actor::resolve`], or `None` while there is no selection.
    route_clusters: Option<HashSet<String>>,
    subscriptions: SubscriptionWatch,
    /// Clusters currently held by live [`ClusterSubscription`] handles.
    ///
    /// [`ClusterSubscription`]: super::ClusterSubscription
    subscribed_clusters: HashSet<Arc<str>>,
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
    fn new(options: DependencyManagerOptions, subscriptions: SubscriptionWatch) -> Self {
        let DependencyManagerOptions {
            xds_client,
            listener_name,
            data_plane_authority,
            node_id,
            runtime: _,
            watcher,
        } = options;
        let listener = Watched::start(&xds_client, listener_name);
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
            route_clusters: None,
            subscriptions,
            subscribed_clusters: HashSet::new(),
        }
    }

    /// Handles events in batches until the XdsClient stops.
    async fn run_loop(mut self) {
        while let Some(events) = poll_fn(|cx| self.poll_batch(cx)).await {
            self.handle_batch(events);
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
    ///
    /// A dynamic subscription change is applied here, and yields a batch, even
    /// an empty one, if it changed the roots of the cluster graph.
    fn poll_batch(&mut self, cx: &mut Context<'_>) -> Poll<Option<Vec<WatchEvent>>> {
        let mut batch = Vec::new();
        loop {
            match self.poll_watch_event(cx) {
                Poll::Ready(Some(event)) => batch.push(event),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => break,
            }
        }
        let roots_changed = self.poll_subscriptions(cx);
        if batch.is_empty() && !roots_changed {
            Poll::Pending
        } else {
            Poll::Ready(Some(batch))
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

    /// Applies any change to the dynamically subscribed clusters, and reports
    /// whether it changed the roots of the cluster graph: the union of the
    /// selected virtual host's clusters and the subscribed clusters.
    ///
    /// While there is no selection, the cluster watches are kept as they are
    /// (see [`Actor::resolve`]), so a subscription change needs no update.
    fn poll_subscriptions(&mut self, cx: &mut Context<'_>) -> bool {
        let Poll::Ready(clusters) = self.subscriptions.poll_changed(cx) else {
            return false;
        };
        let prev = std::mem::replace(&mut self.subscribed_clusters, clusters);
        self.route_clusters.as_ref().is_some_and(|route_clusters| {
            prev.symmetric_difference(&self.subscribed_clusters)
                .any(|cluster| !route_clusters.contains(cluster.as_ref()))
        })
    }

    /// Applies `events` to the cached states, then updates the watches to
    /// match and publishes the result.
    ///
    /// Everything is derived again from the cached states, whatever the batch
    /// changed. The events' flow-control tokens are held until the result has
    /// been published: the XdsClient reads the next response once they are
    /// dropped (gRFC A88).
    fn handle_batch(&mut self, events: impl IntoIterator<Item = WatchEvent>) {
        let done: Vec<_> = events
            .into_iter()
            .filter_map(|event| self.handle_watch_event(event))
            .collect();
        match self.resolve() {
            Ok(config) => self.publisher.config(config),
            Err(Blocked::Pending) => {}
            Err(Blocked::Failed(kind)) => self.publisher.error(kind),
        }
        drop(done);
    }

    /// Resolves the Listener, its RouteConfiguration, the virtual host and
    /// its clusters, in that order, and updates the watches to match.
    fn resolve(&mut self) -> Result<XdsConfig, Blocked> {
        self.update_route_watch();
        // While there is no selection, the cluster watches are kept as they
        // are: they will likely be needed again once the Listener or
        // RouteConfiguration recovers.
        let selection = match self.select_route() {
            Ok(selection) => selection,
            Err(blocked) => {
                self.route_clusters = None;
                return Err(blocked);
            }
        };
        self.update_clusters(selection)
    }

    /// Watches the RouteConfiguration that the Listener names.
    ///
    /// While the Listener is pending or failed, the watch is kept as it is.
    fn update_route_watch(&mut self) {
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
                    self.route_config = Some(Watched::start(&self.client, name.clone()));
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

    /// Walks the cluster graph from the selected virtual host and any active
    /// dynamic cluster subscriptions, updates the watches to match it, and
    /// returns the snapshot once every root cluster has resolved.
    fn update_clusters(&mut self, selection: Selection) -> Result<XdsConfig, Blocked> {
        let route_clusters = self.route_clusters.insert(selection.clusters);
        let subscribed_clusters = &self.subscribed_clusters;
        let roots = || {
            route_clusters
                .iter()
                .map(String::as_str)
                .chain(subscribed_clusters.iter().map(Arc::as_ref))
        };
        let graph = walk(
            &self.watches,
            roots(),
            &selection.notes,
            self.node_id.as_deref(),
        );
        self.watches
            .reconcile(&self.client, &graph.required_watches);

        // A cluster is missing while it or anything below it is pending.
        if !roots().all(|name| graph.cluster_configs.contains_key(name)) {
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
    use std::task::Waker;

    use super::*;
    use crate::dependency_manager::ClusterSubscriber;
    use crate::dependency_manager::XdsConfigUpdate;
    use crate::dependency_manager::cluster_graph::DnsTarget;
    use crate::dependency_manager::subscriptions;
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
        subscriber: ClusterSubscriber,
        updates: mpsc::Receiver<XdsConfigUpdate>,
    }

    impl Harness {
        fn new() -> Self {
            let (options, updates) = options();
            let (subscriber, subscriptions) = subscriptions::new();
            let actor = Actor::new(options, subscriptions);
            Self {
                actor,
                subscriber,
                updates,
            }
        }

        /// Flushes any pending subscription change into the actor, and returns
        /// what it published.
        fn sync_subscriptions(&mut self) -> Vec<XdsConfigUpdate> {
            self.batch_with_subscriptions([])
        }

        /// Flushes a subscription change that must publish a snapshot.
        fn sync_config(&mut self) -> Arc<XdsConfig> {
            match <[_; 1]>::try_from(self.sync_subscriptions()) {
                Ok([Ok(config)]) => config,
                updates => panic!("expected one snapshot, got {updates:?}"),
            }
        }

        /// Handles `events` alongside any pending subscription change in one
        /// batch, and returns what it published.
        fn batch_with_subscriptions(
            &mut self,
            events: impl IntoIterator<Item = WatchEvent>,
        ) -> Vec<XdsConfigUpdate> {
            let events: Vec<_> = events.into_iter().collect();
            let roots_changed = self
                .actor
                .poll_subscriptions(&mut Context::from_waker(Waker::noop()));
            if !events.is_empty() || roots_changed {
                self.actor.handle_batch(events);
            }
            self.updates.try_iter().collect()
        }

        /// Handles `events` as one batch, and returns what it published.
        fn batch(&mut self, events: impl IntoIterator<Item = WatchEvent>) -> Vec<XdsConfigUpdate> {
            self.actor.handle_batch(events);
            self.updates.try_iter().collect()
        }

        /// Handles `events` as one batch, which must publish a snapshot.
        fn config(&mut self, events: impl IntoIterator<Item = WatchEvent>) -> Arc<XdsConfig> {
            match <[_; 1]>::try_from(self.batch(events)) {
                Ok([Ok(config)]) => config,
                updates => panic!("expected one snapshot, got {updates:?}"),
            }
        }

        /// Handles `events` as one batch, which must publish an error.
        fn error(&mut self, events: impl IntoIterator<Item = WatchEvent>) -> XdsConfigError {
            match <[_; 1]>::try_from(self.batch(events)) {
                Ok([Err(error)]) => error,
                updates => panic!("expected one error, got {updates:?}"),
            }
        }

        /// Sets up a Listener with RouteConfiguration `routes`, whose EDS
        /// clusters have all resolved.
        fn resolved(clusters: &[&str]) -> Self {
            let mut harness = Self::new();
            harness.batch([lds_rds("routes")]);
            harness.batch([rds("routes", clusters)]);
            harness.batch(clusters.iter().map(|c| cds_eds(c)));
            harness.config(clusters.iter().map(|c| eds(&format!("{c}-eds"))));
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
            runtime: grpc_tokio::runtime(),
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

    #[test]
    fn publishes_once_every_cluster_resolves() {
        let mut harness = Harness::new();
        assert!(harness.batch([lds_rds("routes")]).is_empty());
        assert_eq!(harness.route_config_watch(), Some("routes"));

        assert!(harness.batch([rds("routes", &["a", "b"])]).is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "b"]);

        assert!(harness.batch([cds_eds("a"), cds_eds("b")]).is_empty());
        assert_eq!(harness.watched_endpoints(), ["a-eds", "b-eds"]);

        assert!(harness.batch([eds("a-eds")]).is_empty());
        let config = harness.config([eds("b-eds")]);
        assert_eq!(config.listener.name, LISTENER);
        assert_eq!(config.route_config.name, "routes");
        assert_eq!(cluster_names(&config), ["a", "b"]);
        assert!(config.clusters.values().all(Result::is_ok));
    }

    #[test]
    fn route_without_clusters_publishes_straight_away() {
        let mut harness = Harness::new();
        let config = harness.config([lds_inline(route_config("inline", &[]))]);
        assert!(config.clusters.is_empty());
    }

    #[test]
    fn top_level_errors() {
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
            let mut harness = Harness::new();
            assert!(harness.batch(setup).is_empty());
            let error = harness.error(events);
            assert_eq!(error.to_string(), expected);
            assert!(harness.watched_clusters().is_empty());
        }
    }

    #[test]
    fn lasting_error_is_published_once() {
        let mut harness = Harness::new();
        harness.error([WatchEvent::Listener(does_not_exist())]);
        assert!(
            harness
                .batch([WatchEvent::Listener(does_not_exist())])
                .is_empty()
        );

        harness.config([lds_inline(route_config("inline", &[]))]);
        harness.error([WatchEvent::Listener(does_not_exist())]);
    }

    #[test]
    fn route_config_rename_waits_for_the_new_route_config() {
        let mut harness = Harness::resolved(&["a"]);

        assert!(harness.batch([lds_rds("routes-2")]).is_empty());
        assert_eq!(harness.route_config_watch(), Some("routes-2"));
        assert_eq!(harness.watched_clusters(), ["a"]);

        // Cluster "a" is still cached, so the snapshot needs no new fetch.
        let config = harness.config([rds("routes-2", &["a"])]);
        assert_eq!(config.route_config.name, "routes-2");
        assert!(config.clusters["a"].is_ok());
    }

    #[test]
    fn listener_error_keeps_the_watches() {
        let mut harness = Harness::resolved(&["a"]);

        let error = harness.error([WatchEvent::Listener(does_not_exist())]);
        assert!(matches!(error.kind, XdsConfigErrorKind::Listener { .. }));
        assert_eq!(harness.route_config_watch(), Some("routes"));
        assert_eq!(harness.watched_clusters(), ["a"]);
        assert_eq!(harness.watched_endpoints(), ["a-eds"]);
        // A cluster event can't publish anything until the Listener recovers.
        assert!(harness.batch([eds("a-eds")]).is_empty());

        let config = harness.config([lds_rds("routes")]);
        assert!(config.clusters["a"].is_ok());
    }

    #[test]
    fn inline_route_config_drops_the_rds_watch() {
        let mut harness = Harness::resolved(&["a"]);
        let config = harness.config([lds_inline(route_config("inline", &["a"]))]);
        assert_eq!(config.route_config.name, "inline");
        assert_eq!(harness.route_config_watch(), None);
    }

    #[test]
    fn route_change_drops_unused_watches() {
        let mut harness = Harness::resolved(&["a", "b"]);
        let config = harness.config([rds("routes", &["b"])]);
        assert_eq!(cluster_names(&config), ["b"]);
        assert_eq!(harness.watched_clusters(), ["b"]);
        assert_eq!(harness.watched_endpoints(), ["b-eds"]);
    }

    #[test]
    fn logical_dns_cluster_is_unsupported() {
        let mut harness = Harness::new();
        harness.batch([lds_inline(route_config("inline", &["dns"]))]);
        let discovery = ClusterDiscovery::LogicalDns {
            hostname: "dns.example.com".to_owned(),
            port: 443,
        };
        assert!(harness.batch([cds("dns", discovery)]).is_empty());

        let target = DnsTarget {
            host: "dns.example.com".to_owned(),
            port: 443,
        };
        let config = harness.config([WatchEvent::ClusterGraph(ClusterGraphEvent::Dns(target))]);
        assert_eq!(
            leaf_note(&config, "dns"),
            Some("LogicalDNS clusters are not supported yet")
        );
    }

    #[test]
    fn ambient_errors_become_resolution_notes() {
        let mut harness = Harness::new();
        harness.batch([lds_rds("routes")]);
        harness.batch([rds("routes", &["a"])]);
        harness.batch([cds_eds("a")]);
        let config = harness.config([eds("a-eds")]);
        // With no other note, a leaf's note is the node ID.
        assert_eq!(leaf_note(&config, "a"), Some("xDS node ID:node-1"));

        let config = harness.config([
            WatchEvent::Listener(ambient("lds down")),
            WatchEvent::RouteConfig(ambient("rds down")),
            WatchEvent::ClusterGraph(ClusterGraphEvent::Cluster(
                "a".to_owned(),
                ambient("cds down"),
            )),
        ]);
        assert_eq!(
            leaf_note(&config, "a"),
            Some(
                "LDS resource listener: failed to connect: lds down; \
                 RDS resource routes: failed to connect: rds down; \
                 CDS resource a: failed to connect: cds down"
            )
        );
    }

    #[test]
    fn cluster_error_is_reported_in_the_snapshot() {
        let mut harness = Harness::new();
        harness.batch([lds_inline(route_config("inline", &["a"]))]);
        let failure = ClusterGraphEvent::Cluster("a".to_owned(), does_not_exist());
        let config = harness.config([WatchEvent::ClusterGraph(failure)]);
        let error = config.clusters["a"].as_ref().unwrap_err();
        assert_eq!(error.message(), "CDS resource a: resource does not exist");
    }

    #[test]
    fn subscribe_adds_dynamic_cluster_and_waits_for_it_to_resolve() {
        let mut harness = Harness::resolved(&["a"]);

        let sub = harness.subscriber.subscribe_to_cluster("dyn");
        assert!(harness.sync_subscriptions().is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "dyn"]);

        assert!(harness.batch([cds_eds("dyn")]).is_empty());
        assert_eq!(harness.watched_endpoints(), ["a-eds", "dyn-eds"]);

        let config = harness.config([eds("dyn-eds")]);
        assert_eq!(cluster_names(&config), ["a", "dyn"]);
        assert!(config.clusters.values().all(Result::is_ok));

        drop(sub);
        let config = harness.sync_config();
        assert_eq!(cluster_names(&config), ["a"]);
        assert_eq!(harness.watched_clusters(), ["a"]);
        assert_eq!(harness.watched_endpoints(), ["a-eds"]);
    }

    #[test]
    fn subscribing_to_route_cluster_is_noop_until_route_drops_it() {
        let mut harness = Harness::resolved(&["a", "b"]);

        // Subscribing and unsubscribing while the route still references "b"
        // does not change the root cluster set and must not re-publish.
        let sub_b = harness.subscriber.subscribe_to_cluster("b");
        assert!(harness.sync_subscriptions().is_empty());
        drop(sub_b);
        assert!(harness.sync_subscriptions().is_empty());

        // Holding a subscription to "a" is initially a no-op, then keeps "a"
        // alive when the route drops it (gRFC A31 cluster draining).
        let sub_a = harness.subscriber.subscribe_to_cluster("a");
        assert!(harness.sync_subscriptions().is_empty());

        let config = harness.config([rds("routes", &["b"])]);
        assert_eq!(cluster_names(&config), ["a", "b"]);
        assert_eq!(harness.watched_clusters(), ["a", "b"]);

        drop(sub_a);
        let config = harness.sync_config();
        assert_eq!(cluster_names(&config), ["b"]);
        assert_eq!(harness.watched_clusters(), ["b"]);
        assert_eq!(harness.watched_endpoints(), ["b-eds"]);
    }

    #[test]
    fn multiple_subscriptions_refcount_until_last_drop() {
        let mut harness = Harness::new();
        harness.config([lds_inline(route_config("inline", &[]))]);

        let sub1 = harness.subscriber.subscribe_to_cluster("dyn");
        let sub2 = harness.subscriber.subscribe_to_cluster("dyn");
        let sub3 = sub1.clone();
        assert!(harness.sync_subscriptions().is_empty());
        assert!(harness.batch([cds_eds("dyn")]).is_empty());
        let config = harness.config([eds("dyn-eds")]);
        assert_eq!(cluster_names(&config), ["dyn"]);

        drop(sub1);
        drop(sub2);
        assert!(harness.sync_subscriptions().is_empty());
        assert_eq!(harness.watched_clusters(), ["dyn"]);

        drop(sub3);
        let config = harness.sync_config();
        assert!(config.clusters.is_empty());
        assert!(harness.watched_clusters().is_empty());
        assert!(harness.watched_endpoints().is_empty());
    }

    #[test]
    fn subscribed_aggregate_cluster_walks_descendants() {
        let mut harness = Harness::new();
        harness.config([lds_inline(route_config("inline", &[]))]);

        let sub = harness.subscriber.subscribe_to_cluster("agg");
        assert!(harness.sync_subscriptions().is_empty());
        assert_eq!(harness.watched_clusters(), ["agg"]);

        let aggregate = ClusterDiscovery::Aggregate {
            children: vec!["leaf-1".to_owned(), "leaf-2".to_owned()],
        };
        assert!(harness.batch([cds("agg", aggregate)]).is_empty());
        assert_eq!(harness.watched_clusters(), ["agg", "leaf-1", "leaf-2"]);

        assert!(
            harness
                .batch([cds_eds("leaf-1"), cds_eds("leaf-2")])
                .is_empty()
        );
        assert!(harness.batch([eds("leaf-1-eds")]).is_empty());
        let config = harness.config([eds("leaf-2-eds")]);
        assert_eq!(cluster_names(&config), ["agg", "leaf-1", "leaf-2"]);

        drop(sub);
        let config = harness.sync_config();
        assert!(config.clusters.is_empty());
        assert!(harness.watched_clusters().is_empty());
        assert!(harness.watched_endpoints().is_empty());
    }

    #[test]
    fn subscribed_cluster_error_is_reported_in_the_snapshot() {
        let mut harness = Harness::resolved(&["a"]);

        let _sub = harness.subscriber.subscribe_to_cluster("missing");
        assert!(harness.sync_subscriptions().is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "missing"]);

        let failure = ClusterGraphEvent::Cluster("missing".to_owned(), does_not_exist());
        let config = harness.config([WatchEvent::ClusterGraph(failure)]);
        assert!(config.clusters["a"].is_ok());
        let error = config.clusters["missing"].as_ref().unwrap_err();
        assert_eq!(
            error.message(),
            "CDS resource missing: resource does not exist"
        );
    }

    #[test]
    fn subscription_before_listener_resolves_is_watched_once_ready() {
        let mut harness = Harness::new();
        let _sub = harness.subscriber.subscribe_to_cluster("dyn");
        assert!(harness.sync_subscriptions().is_empty());
        assert!(harness.watched_clusters().is_empty());

        // Once LDS provides an inline route with no route clusters, the queued
        // subscription becomes a root and blocks the first snapshot until ready.
        assert!(
            harness
                .batch([lds_inline(route_config("inline", &[]))])
                .is_empty()
        );
        assert_eq!(harness.watched_clusters(), ["dyn"]);

        assert!(harness.batch([cds_eds("dyn")]).is_empty());
        let config = harness.config([eds("dyn-eds")]);
        assert_eq!(cluster_names(&config), ["dyn"]);
    }

    #[test]
    fn dropping_pending_subscription_unblocks_route_update() {
        let mut harness = Harness::resolved(&["a"]);

        let slow_sub = harness.subscriber.subscribe_to_cluster("slow");
        assert!(harness.sync_subscriptions().is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "slow"]);

        // Route switches to "b" and "b" resolves, but "slow" is still pending.
        assert!(harness.batch([rds("routes", &["b"])]).is_empty());
        assert!(harness.batch([cds_eds("b")]).is_empty());
        assert!(harness.batch([eds("b-eds")]).is_empty());

        // Dropping the pending subscription drops its watch and unblocks "b".
        drop(slow_sub);
        let config = harness.sync_config();
        assert_eq!(cluster_names(&config), ["b"]);
        assert_eq!(harness.watched_clusters(), ["b"]);
    }

    #[test]
    fn batched_lds_rds_and_subscription_change_does_not_panic() {
        let mut harness = Harness::new();

        // Initial LDS naming RDS "routes" arrives in the same batch as a
        // dynamic subscription before `route_config` watch exists.
        let sub_1 = harness.subscriber.subscribe_to_cluster("dyn-1");
        assert!(
            harness
                .batch_with_subscriptions([lds_rds("routes")])
                .is_empty()
        );
        assert_eq!(harness.route_config_watch(), Some("routes"));

        assert!(harness.batch([rds("routes", &["a"])]).is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "dyn-1"]);
        assert!(harness.batch([cds_eds("a"), cds_eds("dyn-1")]).is_empty());
        let config = harness.config([eds("a-eds"), eds("dyn-1-eds")]);
        assert_eq!(cluster_names(&config), ["a", "dyn-1"]);

        // An LDS update switching to a new RDS name ("routes-2") batched
        // together with a second subscription change also reconciles cleanly.
        let _sub_2 = harness.subscriber.subscribe_to_cluster("dyn-2");
        drop(sub_1);
        assert!(
            harness
                .batch_with_subscriptions([lds_rds("routes-2")])
                .is_empty()
        );
        assert_eq!(harness.route_config_watch(), Some("routes-2"));

        assert!(harness.batch([rds("routes-2", &["a"])]).is_empty());
        assert_eq!(harness.watched_clusters(), ["a", "dyn-2"]);
        assert!(harness.batch([cds_eds("dyn-2")]).is_empty());
        let config = harness.config([eds("dyn-2-eds")]);
        assert_eq!(config.route_config.name, "routes-2");
        assert_eq!(cluster_names(&config), ["a", "dyn-2"]);
    }

    #[test]
    fn subscribed_leaf_stays_when_its_aggregate_drops_it() {
        let mut harness = Harness::new();
        harness.batch([lds_inline(route_config("inline", &["agg"]))]);
        let aggregate = ClusterDiscovery::Aggregate {
            children: vec!["leaf-1".to_owned(), "leaf-2".to_owned()],
        };
        harness.batch([cds("agg", aggregate)]);
        harness.batch([cds_eds("leaf-1"), cds_eds("leaf-2")]);
        let initial = harness.config([eds("leaf-1-eds"), eds("leaf-2-eds")]);
        assert_eq!(cluster_names(&initial), ["agg", "leaf-1", "leaf-2"]);

        // "leaf-1" is already reachable through "agg", but subscribing to it
        // changes the roots, so the same clusters are published again.
        let _sub_leaf = harness.subscriber.subscribe_to_cluster("leaf-1");
        let config = harness.sync_config();
        assert_eq!(cluster_names(&config), ["agg", "leaf-1", "leaf-2"]);

        let shrunk = ClusterDiscovery::Aggregate {
            children: vec!["leaf-2".to_owned()],
        };
        let config = harness.config([cds("agg", shrunk)]);
        assert_eq!(cluster_names(&config), ["agg", "leaf-1", "leaf-2"]);
        assert_eq!(harness.watched_clusters(), ["agg", "leaf-1", "leaf-2"]);
    }

    #[tokio::test]
    async fn stops_once_the_xds_client_stops() {
        // A disconnected XdsClient's watches close straight away.
        let (options, updates) = options();
        let (_subscriber, subscriptions) = subscriptions::new();
        run(options, subscriptions).await;
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
