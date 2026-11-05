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

//! The watches below the route configuration (CDS, EDS and DNS), and their
//! cached states.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use xds_client::ProcessingDone;
use xds_client::Resource;
use xds_client::ResourceEvent;
use xds_client::ResourceWatcher;
use xds_client::XdsClient;

use super::cluster_graph::CachedResources;
use super::cluster_graph::DnsTarget;
use super::cluster_graph::RequiredWatches;
use super::state::WatchState;
use crate::resource::ClusterResource;
use crate::resource::EndpointAddress;
use crate::resource::EndpointsResource;

/// The resolution note of every LogicalDNS cluster, for now.
// TODO: resolve LogicalDNS targets with a DNS resolver.
const DNS_UNSUPPORTED: &str = "LogicalDNS clusters are not supported yet";

/// The CDS, EDS and DNS watches, and the cached state of each.
///
/// Only [`Self::reconcile`] adds or removes watches. The cluster graph walk
/// reads the cached states through [`CachedResources`].
#[derive(Debug, Default)]
pub(super) struct Watches {
    /// The CDS watches, by cluster name.
    clusters: HashMap<String, Watch<ClusterResource>>,
    /// The EDS watches, by EDS service name.
    endpoints: HashMap<String, Watch<EndpointsResource>>,
    /// The state of each DNS resolution, by LogicalDNS cluster target.
    dns: HashMap<DnsTarget, WatchState<Arc<[EndpointAddress]>>>,
    /// DNS targets whose resolution hasn't been reported yet.
    unresolved_dns: Vec<DnsTarget>,
    /// Events that have been polled but not yet returned by
    /// [`Self::poll_next`]. `None` means that a watcher has closed.
    ready: VecDeque<Option<ClusterGraphEvent>>,
}

/// An xDS resource watch, and the cached state of its resource.
#[derive(Debug)]
struct Watch<T: Resource> {
    watcher: ResourceWatcher<T>,
    state: WatchState<Arc<T>>,
}

/// An event for one of the [`Watches`].
#[derive(Debug)]
pub(super) enum ClusterGraphEvent {
    Cluster(String, ResourceEvent<ClusterResource>),
    Endpoints(String, ResourceEvent<EndpointsResource>),
    /// A LogicalDNS target has been resolved.
    Dns(DnsTarget),
}

impl CachedResources for Watches {
    fn cluster(&self, name: &str) -> Option<&WatchState<Arc<ClusterResource>>> {
        self.clusters.get(name).map(|watch| &watch.state)
    }

    fn endpoints(&self, eds_service_name: &str) -> Option<&WatchState<Arc<EndpointsResource>>> {
        self.endpoints
            .get(eds_service_name)
            .map(|watch| &watch.state)
    }

    fn dns(&self, target: &DnsTarget) -> Option<&WatchState<Arc<[EndpointAddress]>>> {
        self.dns.get(target)
    }
}

impl Watches {
    /// Starts the watches in `required` that don't exist yet, and drops the
    /// ones that aren't in it. Dropping a watcher unsubscribes from its
    /// resource, and drops its events that [`Self::poll_next`] hasn't
    /// returned yet.
    ///
    /// A new watch starts as [`WatchState::Pending`].
    pub(super) async fn reconcile(&mut self, client: &XdsClient, required: &RequiredWatches) {
        reconcile(client, &mut self.clusters, &required.clusters).await;
        reconcile(client, &mut self.endpoints, &required.eds_services).await;

        let dns = &mut self.dns;
        dns.retain(|target, _| required.dns_targets.contains(target));
        for target in &required.dns_targets {
            if !dns.contains_key(target) {
                dns.insert(target.clone(), WatchState::Pending);
                self.unresolved_dns.push(target.clone());
            }
        }

        self.unresolved_dns
            .retain(|target| self.dns.contains_key(target));
        self.ready.retain(|event| match event {
            // A closed watcher means that the XdsClient has stopped.
            None => true,
            Some(ClusterGraphEvent::Cluster(name, _)) => self.clusters.contains_key(name),
            Some(ClusterGraphEvent::Endpoints(name, _)) => self.endpoints.contains_key(name),
            Some(ClusterGraphEvent::Dns(target)) => self.dns.contains_key(target),
        });
    }

    /// Returns the next event, or `None` once the XdsClient has stopped.
    ///
    /// When no polled events are left, this polls every watcher and keeps
    /// all the events that are ready, so that a response for many resources
    /// costs one pass over the watchers rather than one pass per resource.
    pub(super) fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<ClusterGraphEvent>> {
        if self.ready.is_empty() {
            self.poll_watchers(cx);
        }
        self.ready.pop_front().map_or(Poll::Pending, Poll::Ready)
    }

    fn poll_watchers(&mut self, cx: &mut Context<'_>) {
        poll_ready(
            &mut self.clusters,
            cx,
            ClusterGraphEvent::Cluster,
            &mut self.ready,
        );
        poll_ready(
            &mut self.endpoints,
            cx,
            ClusterGraphEvent::Endpoints,
            &mut self.ready,
        );
        // Resolution always fails for now, so it is reported straight away.
        self.ready.extend(
            self.unresolved_dns
                .drain(..)
                .map(|target| Some(ClusterGraphEvent::Dns(target))),
        );
    }

    /// Applies `event` to its cached state, and returns its flow-control
    /// token, if it has one.
    pub(super) fn apply(&mut self, event: ClusterGraphEvent) -> Option<ProcessingDone> {
        const WATCHED: &str = "reconcile drops the events of the watches that it drops";
        match event {
            ClusterGraphEvent::Cluster(name, event) => {
                let watch = self.clusters.get_mut(&name).expect(WATCHED);
                Some(
                    watch
                        .state
                        .apply(event, |e| format!("CDS resource {name}: {e}").into()),
                )
            }
            ClusterGraphEvent::Endpoints(name, event) => {
                let watch = self.endpoints.get_mut(&name).expect(WATCHED);
                Some(
                    watch
                        .state
                        .apply(event, |e| format!("EDS resource {name}: {e}").into()),
                )
            }
            ClusterGraphEvent::Dns(target) => {
                let state = self.dns.get_mut(&target).expect(WATCHED);
                *state = WatchState::Failed(DNS_UNSUPPORTED.into());
                None
            }
        }
    }
}

/// Makes `watches` hold exactly the names in `required`.
async fn reconcile<T: Resource>(
    client: &XdsClient,
    watches: &mut HashMap<String, Watch<T>>,
    required: &HashSet<String>,
) {
    watches.retain(|name, _| required.contains(name));
    for name in required {
        if !watches.contains_key(name) {
            let watch = Watch {
                watcher: client.watch(name.clone()).await,
                state: WatchState::Pending,
            };
            watches.insert(name.clone(), watch);
        }
    }
}

/// Adds each event that is ready on `watches` to `ready`, as a
/// [`ClusterGraphEvent`] made by `make_event`, followed by `None` for each
/// watcher that has closed.
fn poll_ready<T: Resource>(
    watches: &mut HashMap<String, Watch<T>>,
    cx: &mut Context<'_>,
    make_event: fn(String, ResourceEvent<T>) -> ClusterGraphEvent,
    ready: &mut VecDeque<Option<ClusterGraphEvent>>,
) {
    for (name, watch) in watches {
        while let Poll::Ready(event) = watch.watcher.poll_next(cx) {
            let closed = event.is_none();
            ready.push_back(event.map(|event| make_event(name.clone(), event)));
            if closed {
                break;
            }
        }
    }
}

#[cfg(test)]
impl Watches {
    /// The names of the CDS watches.
    pub(super) fn cluster_names(&self) -> impl Iterator<Item = &String> {
        self.clusters.keys()
    }

    /// The names of the EDS watches.
    pub(super) fn endpoint_names(&self) -> impl Iterator<Item = &String> {
        self.endpoints.keys()
    }
}

#[cfg(test)]
mod tests {
    use std::task::Waker;

    use super::*;

    fn names<V>(map: &HashMap<String, V>) -> Vec<&str> {
        let mut names: Vec<&str> = map.keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }

    fn required_clusters(names: &[&str]) -> RequiredWatches {
        RequiredWatches {
            clusters: names.iter().map(|name| (*name).to_owned()).collect(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn reconcile_adds_and_drops_watches() {
        let client = XdsClient::disconnected();
        let mut watches = Watches::default();
        watches
            .reconcile(&client, &required_clusters(&["a", "b"]))
            .await;
        watches.clusters.get_mut("a").unwrap().state = WatchState::Failed("kept".into());

        watches
            .reconcile(&client, &required_clusters(&["a", "c"]))
            .await;

        assert_eq!(names(&watches.clusters), ["a", "c"]);
        // An existing watch keeps its state; a new one starts pending.
        assert!(
            matches!(watches.cluster("a"), Some(WatchState::Failed(reason)) if &**reason == "kept")
        );
        assert!(matches!(watches.cluster("c"), Some(WatchState::Pending)));
        assert!(watches.cluster("b").is_none());
    }

    #[tokio::test]
    async fn dns_targets_resolve_as_unsupported() {
        let target = DnsTarget {
            host: "example.com".to_owned(),
            port: 443,
        };
        let required = RequiredWatches {
            dns_targets: HashSet::from([target.clone()]),
            ..Default::default()
        };
        let mut watches = Watches::default();
        watches
            .reconcile(&XdsClient::disconnected(), &required)
            .await;
        assert_eq!(watches.dns[&target], WatchState::Pending);

        // The failure arrives as an event, which starts the next batch.
        let mut cx = Context::from_waker(Waker::noop());
        let Poll::Ready(Some(event)) = watches.poll_next(&mut cx) else {
            panic!("expected the DNS event");
        };
        assert!(watches.apply(event).is_none());
        assert_eq!(
            watches.dns[&target],
            WatchState::Failed(DNS_UNSUPPORTED.into())
        );
        assert!(watches.poll_next(&mut cx).is_pending());

        watches
            .reconcile(&XdsClient::disconnected(), &RequiredWatches::default())
            .await;
        assert!(watches.dns.is_empty());
    }

    #[tokio::test]
    async fn reconcile_drops_the_events_of_dropped_watches() {
        let client = XdsClient::disconnected();
        let two_targets = RequiredWatches {
            dns_targets: ["a.example.com", "b.example.com"]
                .map(|host| DnsTarget {
                    host: host.to_owned(),
                    port: 443,
                })
                .into(),
            ..Default::default()
        };
        let none = RequiredWatches::default();
        let mut watches = Watches::default();
        let mut cx = Context::from_waker(Waker::noop());

        // Results that haven't been polled yet.
        watches.reconcile(&client, &two_targets).await;
        watches.reconcile(&client, &none).await;
        assert!(watches.poll_next(&mut cx).is_pending());

        // Events that have been polled but not yet returned.
        watches.reconcile(&client, &two_targets).await;
        assert!(watches.poll_next(&mut cx).is_ready());
        watches.reconcile(&client, &none).await;
        assert!(watches.poll_next(&mut cx).is_pending());
    }

    #[tokio::test]
    async fn closed_watcher_ends_updates() {
        // A disconnected client's watchers are closed from the start.
        let mut watches = Watches::default();
        watches
            .reconcile(&XdsClient::disconnected(), &required_clusters(&["a"]))
            .await;
        let mut cx = Context::from_waker(Waker::noop());
        assert!(matches!(watches.poll_next(&mut cx), Poll::Ready(None)));
    }
}
