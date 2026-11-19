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

//! Reference-counted dynamic cluster subscriptions (gRFC A74).

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::sync::Weak;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

/// Creates a paired [`ClusterSubscriber`] and [`SubscriptionWatch`].
pub(super) fn new() -> (ClusterSubscriber, SubscriptionWatch) {
    let shared = Arc::new(Mutex::new(SharedSubscriptions::default()));
    (
        ClusterSubscriber {
            shared: Arc::clone(&shared),
        },
        SubscriptionWatch { shared },
    )
}

#[derive(Default)]
struct SharedSubscriptions {
    /// The [`Entry`] shared by every [`ClusterSubscription`] for each
    /// subscribed cluster name.
    ///
    /// Once the last handle for a cluster is dropped, its entry stays here
    /// with no strong references until [`Entry`]'s `Drop` removes it.
    entries: HashMap<Arc<str>, Weak<Entry>>,
    /// Set whenever an entry is added or removed.
    changed: bool,
    /// Waker registered by [`SubscriptionWatch::poll_changed`].
    waker: Option<Waker>,
}

impl SharedSubscriptions {
    /// Records that the subscribed cluster set changed, and returns the waker
    /// to wake once the lock has been released.
    fn mark_changed(&mut self) -> Option<Waker> {
        self.changed = true;
        self.waker.take()
    }

    fn snapshot(&self) -> HashSet<Arc<str>> {
        self.entries
            .iter()
            .filter(|(_, entry)| entry.strong_count() > 0)
            .map(|(cluster_name, _)| Arc::clone(cluster_name))
            .collect()
    }
}

fn lock(mutex: &Mutex<SharedSubscriptions>) -> MutexGuard<'_, SharedSubscriptions> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Clonable handle for creating dynamic [`ClusterSubscription`]s (gRFC A74).
///
/// Implements [`Eq`] by identity of the underlying [`XdsDependencyManager`] so
/// it can be passed to LB policies inside [`grpc::attributes::Attributes`].
///
/// [`XdsDependencyManager`]: super::XdsDependencyManager
#[derive(Clone)]
pub(crate) struct ClusterSubscriber {
    shared: Arc<Mutex<SharedSubscriptions>>,
}

impl ClusterSubscriber {
    /// Subscribes to `cluster_name` for as long as the returned
    /// [`ClusterSubscription`] (or any clone of it) remains alive.
    ///
    /// Until the cluster has resolved or failed, the dependency manager holds
    /// back its next snapshot, so that the snapshot includes the cluster.
    /// Other changes wait as well.
    ///
    /// If the owning [`XdsDependencyManager`] has already been dropped, this is
    /// a safe no-op: it still returns a [`ClusterSubscription`], without
    /// starting any watches or publishing updates.
    ///
    /// [`XdsDependencyManager`]: super::XdsDependencyManager
    // TODO: remove once the xds resolver / cds LB policy uses subscriptions.
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn subscribe_to_cluster(&self, cluster_name: &str) -> ClusterSubscription {
        let mut state = lock(&self.shared);
        if let Some(entry) = state.entries.get(cluster_name).and_then(Weak::upgrade) {
            return ClusterSubscription(entry);
        }
        let cluster_name: Arc<str> = cluster_name.into();
        let entry = Arc::new(Entry {
            cluster_name: Arc::clone(&cluster_name),
            shared: Arc::clone(&self.shared),
        });
        // This may replace an entry whose last handle is being dropped
        // concurrently. That entry's `Drop` then leaves this one in place.
        state.entries.insert(cluster_name, Arc::downgrade(&entry));
        let waker = state.mark_changed();
        drop(state);
        let subscription = ClusterSubscription(entry);
        if let Some(waker) = waker {
            waker.wake();
        }
        subscription
    }
}

impl PartialEq for ClusterSubscriber {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

impl Eq for ClusterSubscriber {}

impl fmt::Debug for ClusterSubscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterSubscriber").finish_non_exhaustive()
    }
}

/// RAII handle keeping a cluster subscribed in [`XdsDependencyManager`]
/// (gRFC A74).
///
/// Every handle for the same cluster shares one reference count, so cloning
/// and dropping a handle are cheap. Dropping the last handle ends the
/// subscription; the cluster stays watched only while the selected virtual
/// host's routes or an aggregate cluster still reference it.
///
/// Keep a handle for as long as the cluster is needed, rather than
/// subscribing again on each update: the dependency manager may publish
/// whenever a cluster gains its first handle or loses its last, so
/// resubscribing on every update could keep it publishing forever.
///
/// [`XdsDependencyManager`]: super::XdsDependencyManager
#[derive(Clone)]
#[must_use = "dropping a ClusterSubscription immediately unsubscribes from the cluster"]
pub(crate) struct ClusterSubscription(Arc<Entry>);

impl ClusterSubscription {
    /// The name of the subscribed cluster.
    // TODO: remove once the xds resolver / cds LB policy uses subscriptions.
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn cluster_name(&self) -> &str {
        &self.0.cluster_name
    }
}

impl fmt::Debug for ClusterSubscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterSubscription")
            .field("cluster_name", &self.0.cluster_name)
            .finish_non_exhaustive()
    }
}

/// The state shared by every [`ClusterSubscription`] for one cluster.
struct Entry {
    cluster_name: Arc<str>,
    shared: Arc<Mutex<SharedSubscriptions>>,
}

impl Drop for Entry {
    fn drop(&mut self) {
        let mut state = lock(&self.shared);
        let is_current = state
            .entries
            .get(&self.cluster_name)
            .is_some_and(|entry| ptr::eq(entry.as_ptr(), self));
        if !is_current {
            return;
        }
        state.entries.remove(&self.cluster_name);
        let waker = state.mark_changed();
        drop(state);
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// Actor-side receiver notified when the set of subscribed clusters changes.
pub(super) struct SubscriptionWatch {
    shared: Arc<Mutex<SharedSubscriptions>>,
}

impl SubscriptionWatch {
    /// Returns the active subscribed cluster set if it changed since the last
    /// ready poll, or [`Poll::Pending`] if it didn't.
    ///
    /// Either way, `cx`'s waker is woken on the next change, so the caller
    /// needn't poll again until then.
    pub(super) fn poll_changed(&mut self, cx: &mut Context<'_>) -> Poll<HashSet<Arc<str>>> {
        let mut state = lock(&self.shared);
        let old_waker = if state
            .waker
            .as_ref()
            .is_some_and(|waker| waker.will_wake(cx.waker()))
        {
            None
        } else {
            state.waker.replace(cx.waker().clone())
        };
        let poll = if std::mem::take(&mut state.changed) {
            Poll::Ready(state.snapshot())
        } else {
            Poll::Pending
        };
        drop(state);
        drop(old_waker);
        poll
    }
}

impl Drop for SubscriptionWatch {
    fn drop(&mut self) {
        let mut state = lock(&self.shared);
        let waker = state.waker.take();
        drop(state);
        drop(waker);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::task::Wake;

    use grpc::attributes::Attributes;

    use super::*;

    struct CountingWake(AtomicUsize);

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn wakes_only_when_active_cluster_set_changes() {
        let (subscriber, mut watch) = new();
        let counter = Arc::new(CountingWake(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);

        assert_eq!(watch.poll_changed(&mut cx), Poll::Pending);
        assert_eq!(counter.0.load(Ordering::SeqCst), 0);

        // 0 -> 1 wakes and marks changed.
        let sub1 = subscriber.subscribe_to_cluster("cluster-a");
        assert_eq!(sub1.cluster_name(), "cluster-a");
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        assert_eq!(
            watch.poll_changed(&mut cx),
            Poll::Ready(HashSet::from([Arc::from("cluster-a")]))
        );

        // 1 -> 2 (second subscribe) and 2 -> 3 (clone) do not change the set.
        let sub2 = subscriber.subscribe_to_cluster("cluster-a");
        let sub3 = sub1.clone();
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        assert_eq!(watch.poll_changed(&mut cx), Poll::Pending);

        // 3 -> 2 -> 1 do not change the set either.
        drop(sub2);
        drop(sub3);
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        assert_eq!(watch.poll_changed(&mut cx), Poll::Pending);

        // 1 -> 0 wakes and reports the empty set.
        drop(sub1);
        assert_eq!(counter.0.load(Ordering::SeqCst), 2);
        assert_eq!(watch.poll_changed(&mut cx), Poll::Ready(HashSet::new()));
    }

    #[test]
    fn ready_poll_leaves_the_waker_registered() {
        let (subscriber, mut watch) = new();
        let counter = Arc::new(CountingWake(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);

        let _sub_a = subscriber.subscribe_to_cluster("cluster-a");
        assert!(watch.poll_changed(&mut cx).is_ready());

        // The next change wakes the caller, without another poll in between.
        let _sub_b = subscriber.subscribe_to_cluster("cluster-b");
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dropping_watch_releases_registered_waker_and_allows_later_handles_to_drop() {
        let (subscriber, mut watch) = new();
        let counter = Arc::new(CountingWake(AtomicUsize::new(0)));
        {
            let waker = Waker::from(Arc::clone(&counter));
            let mut cx = Context::from_waker(&waker);
            assert_eq!(watch.poll_changed(&mut cx), Poll::Pending);
        }
        assert_eq!(Arc::strong_count(&counter), 2);

        let sub = subscriber.subscribe_to_cluster("cluster-a");
        // Consume the change, which leaves the waker registered, then drop
        // `watch` while both `subscriber` and `sub` stay alive.
        {
            let waker = Waker::from(Arc::clone(&counter));
            let mut cx = Context::from_waker(&waker);
            assert!(watch.poll_changed(&mut cx).is_ready());
        }
        assert_eq!(Arc::strong_count(&counter), 2);

        drop(watch);
        assert_eq!(Arc::strong_count(&counter), 1);

        // Subscribing and dropping after `SubscriptionWatch` dropped remains a
        // safe no-op.
        let sub_after_stop = subscriber.subscribe_to_cluster("cluster-b");
        assert_eq!(sub_after_stop.cluster_name(), "cluster-b");
        drop(sub_after_stop);
        drop(sub);
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn subscriber_equality_follows_manager_identity_in_attributes() {
        let (sub_a1, _watch_a) = new();
        let sub_a2 = sub_a1.clone();
        let (sub_b, _watch_b) = new();

        assert_eq!(sub_a1, sub_a2);
        assert_ne!(sub_a1, sub_b);

        let attrs_a1 = Attributes::new().add(sub_a1);
        let attrs_a2 = Attributes::new().add(sub_a2);
        let attrs_b = Attributes::new().add(sub_b);
        assert_eq!(attrs_a1, attrs_a2);
        assert_ne!(attrs_a1, attrs_b);
    }
}
