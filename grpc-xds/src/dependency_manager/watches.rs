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
use crate::resource::EndpointsResource;

/// The resolution note of every LogicalDNS cluster, for now.
// TODO: resolve LogicalDNS targets with a DNS resolver.
const DNS_UNSUPPORTED: &str = "LogicalDNS clusters are not supported yet";

/// The CDS, EDS and DNS watches, and the cached state of each.
///
/// Each watcher has a cached state under the same name, and vice versa. Only
/// [`Self::reconcile`] adds or removes watches, and it changes both together.
#[derive(Debug, Default)]
pub(crate) struct Watches {
    cache: CachedResources,
    clusters: HashMap<String, ResourceWatcher<ClusterResource>>,
    endpoints: HashMap<String, ResourceWatcher<EndpointsResource>>,
    /// DNS targets whose resolution hasn't been reported yet.
    unresolved_dns: Vec<DnsTarget>,
    /// Updates that have been polled but not yet returned by
    /// [`Self::poll_next`]. `None` means that a watcher has closed.
    ready: VecDeque<Option<Update>>,
}

/// An event for one of the [`Watches`].
#[derive(Debug)]
pub(crate) enum Update {
    Cluster(String, ResourceEvent<ClusterResource>),
    Endpoints(String, ResourceEvent<EndpointsResource>),
    /// A LogicalDNS target has been resolved.
    Dns(DnsTarget),
}

impl Watches {
    pub(crate) fn cache(&self) -> &CachedResources {
        &self.cache
    }

    /// Starts the watches in `required` that don't exist yet, and drops the
    /// ones that aren't in it. Dropping a watcher unsubscribes from its
    /// resource.
    ///
    /// A new watch starts as [`WatchState::Pending`].
    ///
    /// The caller must have taken every update from [`Self::poll_next`]
    /// first, so that no update is left for a watch that this drops.
    pub(crate) async fn reconcile(&mut self, client: &XdsClient, required: &RequiredWatches) {
        reconcile(
            client,
            &mut self.clusters,
            &mut self.cache.clusters,
            &required.clusters,
        )
        .await;
        reconcile(
            client,
            &mut self.endpoints,
            &mut self.cache.endpoints,
            &required.eds_services,
        )
        .await;

        let dns = &mut self.cache.dns;
        dns.retain(|target, _| required.dns_targets.contains(target));
        for target in &required.dns_targets {
            if !dns.contains_key(target) {
                dns.insert(target.clone(), WatchState::Pending);
                self.unresolved_dns.push(target.clone());
            }
        }
    }

    /// Returns the next update, or `None` once the XdsClient has stopped.
    ///
    /// When no polled updates are left, this polls every watcher and keeps
    /// all the updates that are ready, so that a response for many resources
    /// costs one pass over the watchers rather than one pass per resource.
    pub(crate) fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Update>> {
        if self.ready.is_empty() {
            self.poll_watchers(cx);
        }
        self.ready.pop_front().map_or(Poll::Pending, Poll::Ready)
    }

    fn poll_watchers(&mut self, cx: &mut Context<'_>) {
        let ready = &mut self.ready;
        for (name, watcher) in &mut self.clusters {
            poll_ready(watcher, cx, |event| {
                ready.push_back(event.map(|event| Update::Cluster(name.clone(), event)));
            });
        }
        for (name, watcher) in &mut self.endpoints {
            poll_ready(watcher, cx, |event| {
                ready.push_back(event.map(|event| Update::Endpoints(name.clone(), event)));
            });
        }
        // Resolution always fails for now, so it is reported straight away.
        ready.extend(
            self.unresolved_dns
                .drain(..)
                .map(|target| Some(Update::Dns(target))),
        );
    }

    /// Applies `update` to its cached state, and returns its flow-control
    /// token, if it has one.
    pub(crate) fn apply(&mut self, update: Update) -> Option<ProcessingDone> {
        const WATCHED: &str = "an update's watch is dropped only once its updates are taken";
        match update {
            Update::Cluster(name, event) => {
                let state = self.cache.clusters.get_mut(&name).expect(WATCHED);
                Some(state.apply(event, |e| format!("CDS resource {name}: {e}").into()))
            }
            Update::Endpoints(name, event) => {
                let state = self.cache.endpoints.get_mut(&name).expect(WATCHED);
                Some(state.apply(event, |e| format!("EDS resource {name}: {e}").into()))
            }
            Update::Dns(target) => {
                let state = self.cache.dns.get_mut(&target).expect(WATCHED);
                *state = WatchState::Failed(DNS_UNSUPPORTED.into());
                None
            }
        }
    }
}

/// Makes `watchers` and `states` hold exactly the names in `required`.
async fn reconcile<T: Resource>(
    client: &XdsClient,
    watchers: &mut HashMap<String, ResourceWatcher<T>>,
    states: &mut HashMap<String, WatchState<Arc<T>>>,
    required: &HashSet<String>,
) {
    watchers.retain(|name, _| required.contains(name));
    states.retain(|name, _| required.contains(name));
    for name in required {
        if !watchers.contains_key(name) {
            watchers.insert(name.clone(), client.watch(name.clone()).await);
            states.insert(name.clone(), WatchState::Pending);
        }
    }
}

/// Passes each event that is ready on `watcher` to `push`, followed by `None`
/// if the watcher has closed.
fn poll_ready<T: Resource>(
    watcher: &mut ResourceWatcher<T>,
    cx: &mut Context<'_>,
    mut push: impl FnMut(Option<ResourceEvent<T>>),
) {
    while let Poll::Ready(event) = watcher.poll_next(cx) {
        let closed = event.is_none();
        push(event);
        if closed {
            return;
        }
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
    async fn reconcile_keeps_watchers_and_states_in_step() {
        let client = XdsClient::disconnected();
        let mut watches = Watches::default();
        watches
            .reconcile(&client, &required_clusters(&["a", "b"]))
            .await;
        watches
            .cache
            .clusters
            .insert("a".to_owned(), WatchState::Failed("kept".into()));

        watches
            .reconcile(&client, &required_clusters(&["a", "c"]))
            .await;

        assert_eq!(names(&watches.clusters), ["a", "c"]);
        assert_eq!(names(&watches.cache.clusters), ["a", "c"]);
        // An existing watch keeps its state; a new one starts pending.
        assert!(
            matches!(&watches.cache.clusters["a"], WatchState::Failed(reason) if &**reason == "kept")
        );
        assert!(matches!(watches.cache.clusters["c"], WatchState::Pending));
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
        assert_eq!(watches.cache.dns[&target], WatchState::Pending);

        // The failure arrives as an update, which starts the next batch.
        let mut cx = Context::from_waker(Waker::noop());
        let Poll::Ready(Some(update)) = watches.poll_next(&mut cx) else {
            panic!("expected the DNS update");
        };
        assert!(watches.apply(update).is_none());
        assert_eq!(
            watches.cache.dns[&target],
            WatchState::Failed(DNS_UNSUPPORTED.into())
        );
        assert!(watches.poll_next(&mut cx).is_pending());

        watches
            .reconcile(&XdsClient::disconnected(), &RequiredWatches::default())
            .await;
        assert!(watches.cache.dns.is_empty());
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
