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

//! Connection jitter for newly discovered endpoints.
//!
//! Every client watching a cluster learns about a new endpoint at roughly the
//! same moment. If all of them connect at once, the new host takes a burst of
//! handshakes and first requests. [`ConnectionJitter`] spreads that burst out
//! by holding each new endpoint back from the load balancer for a random
//! delay.
//!
//! - A new endpoint waits a uniformly random delay in `[0, max_jitter)`.
//! - At most `max_delayed_ratio` of the cluster's endpoints may wait at once.
//!   An endpoint that would exceed the ratio is released immediately. When
//!   removals shrink the cluster, randomly chosen waiting endpoints are
//!   released until the ratio holds again.
//!
//! Only endpoints that are new to the cluster wait. Re-inserting an endpoint
//! the load balancer already has passes straight through.

use std::collections::{HashSet, VecDeque};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_core::Stream;
use indexmap::IndexMap;
use tower::BoxError;
use tower::discover::Change;

use crate::client::endpoint::EndpointAddress;
use crate::client::lb::BoxDiscover;
use crate::client::loadbalance::keyed_futures::KeyedFutures;

/// Connection-jitter settings that can delay at least one endpoint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ConnectionJitterConfig {
    max_jitter: Duration,
    max_delayed_ratio: f64,
}

impl ConnectionJitterConfig {
    /// Validates the settings. Returns `Ok(None)` when they cannot delay any
    /// endpoint: a zero `max_jitter` or a zero `max_delayed_ratio`.
    ///
    /// # Errors
    ///
    /// Returns an error if `max_delayed_ratio` is not in the range `[0.0, 1.0]`.
    pub(crate) fn new(
        max_jitter: Duration,
        max_delayed_ratio: f64,
    ) -> Result<Option<Self>, String> {
        if !(0.0..=1.0).contains(&max_delayed_ratio) {
            return Err(format!(
                "max_delayed_ratio must be between 0.0 and 1.0, got {max_delayed_ratio}"
            ));
        }
        if max_jitter.is_zero() || max_delayed_ratio == 0.0 {
            return Ok(None);
        }
        Ok(Some(Self {
            max_jitter,
            max_delayed_ratio,
        }))
    }
}

/// A [`Discover`](tower::discover::Discover) stream adapter that holds newly
/// discovered endpoints back for a random delay. See the [module
/// docs](self).
pub(crate) struct ConnectionJitter<S> {
    inner: BoxDiscover<EndpointAddress, S>,
    inner_done: bool,
    config: ConnectionJitterConfig,
    /// Endpoints the inner stream has inserted and not removed, whether or
    /// not they have been released.
    members: HashSet<EndpointAddress>,
    /// Endpoints still waiting, with the service to release when they do.
    delayed: IndexMap<EndpointAddress, S>,
    timers: KeyedFutures<EndpointAddress, ()>,
    /// Changes ready to yield, in order.
    ready: VecDeque<Change<EndpointAddress, S>>,
}

// No field is ever pinned in place: `inner` is already a `Pin<Box<_>>`, and
// the services are only moved in and out of collections.
impl<S> Unpin for ConnectionJitter<S> {}

impl<S: Send + 'static> ConnectionJitter<S> {
    pub(crate) fn new(
        inner: BoxDiscover<EndpointAddress, S>,
        config: ConnectionJitterConfig,
    ) -> Self {
        Self {
            inner,
            inner_done: false,
            config,
            members: HashSet::new(),
            delayed: IndexMap::new(),
            timers: KeyedFutures::new(),
            ready: VecDeque::new(),
        }
    }

    fn on_change(&mut self, change: Change<EndpointAddress, S>) {
        match change {
            Change::Insert(addr, svc) => {
                if let Some(waiting) = self.delayed.get_mut(&addr) {
                    // Keep the original release time, but release the newest
                    // service.
                    *waiting = svc;
                } else if !self.members.insert(addr.clone()) {
                    self.ready.push_back(Change::Insert(addr, svc));
                } else if self.exceeds_limit(self.delayed.len() + 1) {
                    tracing::trace!("connection jitter: {addr} not delayed, limit reached");
                    self.ready.push_back(Change::Insert(addr, svc));
                } else {
                    let delay = self.random_delay();
                    tracing::trace!("connection jitter: delaying {addr} by {delay:?}");
                    let _ = self.timers.add(addr.clone(), tokio::time::sleep(delay));
                    self.delayed.insert(addr, svc);
                }
            }
            Change::Remove(addr) => {
                self.members.remove(&addr);
                if self.delayed.swap_remove(&addr).is_some() {
                    // The load balancer never saw it, so there is nothing to
                    // remove downstream.
                    tracing::trace!("connection jitter: {addr} removed while delayed");
                    let _ = self.timers.cancel(&addr);
                } else {
                    self.ready.push_back(Change::Remove(addr));
                }
                while self.exceeds_limit(self.delayed.len()) {
                    let index = fastrand::usize(..self.delayed.len());
                    if let Some((addr, svc)) = self.delayed.swap_remove_index(index) {
                        tracing::trace!("connection jitter: releasing {addr} early, limit reached");
                        let _ = self.timers.cancel(&addr);
                        self.ready.push_back(Change::Insert(addr, svc));
                    }
                }
            }
        }
    }

    fn exceeds_limit(&self, delayed: usize) -> bool {
        delayed as f64 > self.config.max_delayed_ratio * self.members.len() as f64
    }

    fn random_delay(&self) -> Duration {
        let max_nanos = u64::try_from(self.config.max_jitter.as_nanos()).unwrap_or(u64::MAX);
        Duration::from_nanos(fastrand::u64(..max_nanos))
    }
}

impl<S: Send + 'static> Stream for ConnectionJitter<S> {
    type Item = Result<Change<EndpointAddress, S>, BoxError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if let Some(change) = this.ready.pop_front() {
                return Poll::Ready(Some(Ok(change)));
            }
            // Drain discovery before timers so that a removal cancels a
            // waiting endpoint before its timer can release it.
            if !this.inner_done {
                match this.inner.as_mut().poll_next(cx) {
                    Poll::Ready(Some(Ok(change))) => {
                        this.on_change(change);
                        continue;
                    }
                    Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
                    Poll::Ready(None) => this.inner_done = true,
                    Poll::Pending => {}
                }
            }
            match this.timers.poll_next(cx) {
                Poll::Ready(Some((addr, ()))) => {
                    if let Some(svc) = this.delayed.swap_remove(&addr) {
                        tracing::trace!("connection jitter: releasing {addr}");
                        this.ready.push_back(Change::Insert(addr, svc));
                    }
                }
                // Endpoints still waiting when discovery ends are released on
                // schedule, so the load balancer ends up with the same set it
                // would have had without jitter.
                Poll::Ready(None) if this.inner_done => return Poll::Ready(None),
                Poll::Ready(None) | Poll::Pending => return Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::FutureExt;
    use tokio::sync::mpsc;
    use tokio::time::Instant;
    use tokio_stream::StreamExt;
    use tokio_stream::wrappers::ReceiverStream;

    const MAX_JITTER: Duration = Duration::from_secs(10);

    type Sender = mpsc::Sender<Result<Change<EndpointAddress, u32>, BoxError>>;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Event {
        Insert(EndpointAddress, u32),
        Remove(EndpointAddress),
    }

    fn addr(port: u16) -> EndpointAddress {
        EndpointAddress::new("10.0.0.1", port)
    }

    fn jitter(ratio: f64) -> (Sender, ConnectionJitter<u32>) {
        // The stream draws from this thread's RNG; seed it so delays repeat.
        fastrand::seed(7);
        let (tx, rx) = mpsc::channel(64);
        let config = ConnectionJitterConfig::new(MAX_JITTER, ratio)
            .unwrap()
            .unwrap();
        let stream = ConnectionJitter::new(Box::pin(ReceiverStream::new(rx)), config);
        (tx, stream)
    }

    async fn insert(tx: &Sender, port: u16, svc: u32) {
        tx.send(Ok(Change::Insert(addr(port), svc))).await.unwrap();
    }

    async fn remove(tx: &Sender, port: u16) {
        tx.send(Ok(Change::Remove(addr(port)))).await.unwrap();
    }

    fn event(change: Change<EndpointAddress, u32>) -> Event {
        match change {
            Change::Insert(addr, svc) => Event::Insert(addr, svc),
            Change::Remove(addr) => Event::Remove(addr),
        }
    }

    /// Changes available without letting any time pass.
    fn drain_now(stream: &mut ConnectionJitter<u32>) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some(Some(item)) = stream.next().now_or_never() {
            events.push(event(item.unwrap()));
        }
        events
    }

    /// Changes released by the timers, each checked against `MAX_JITTER`.
    async fn drain_timers(stream: &mut ConnectionJitter<u32>, start: Instant) -> Vec<Event> {
        let mut events = Vec::new();
        while let Ok(Some(item)) = tokio::time::timeout(MAX_JITTER, stream.next()).await {
            assert!(start.elapsed() <= MAX_JITTER, "released after max_jitter");
            events.push(event(item.unwrap()));
        }
        events
    }

    #[test]
    fn config_disables_settings_that_cannot_delay() {
        assert_eq!(ConnectionJitterConfig::new(Duration::ZERO, 0.5), Ok(None));
        assert_eq!(ConnectionJitterConfig::new(MAX_JITTER, 0.0), Ok(None));
        assert!(matches!(
            ConnectionJitterConfig::new(MAX_JITTER, 1.0),
            Ok(Some(_))
        ));
    }

    #[test]
    fn config_rejects_ratio_outside_unit_range() {
        for ratio in [-0.5, 1.5, f64::NAN] {
            let err = ConnectionJitterConfig::new(MAX_JITTER, ratio).unwrap_err();
            assert!(err.contains("max_delayed_ratio"), "{err}");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn ratio_caps_how_many_new_endpoints_wait() {
        let (tx, mut stream) = jitter(0.5);
        let start = Instant::now();
        for port in 1..=4 {
            insert(&tx, port, u32::from(port)).await;
        }

        // Endpoint 1 would make 1 of 1 wait and endpoint 3 would make 2 of 3
        // wait, both above the ratio, so they are released immediately.
        assert_eq!(
            drain_now(&mut stream),
            vec![Event::Insert(addr(1), 1), Event::Insert(addr(3), 3)]
        );

        let mut released = drain_timers(&mut stream, start).await;
        released.sort();
        assert_eq!(
            released,
            vec![Event::Insert(addr(2), 2), Event::Insert(addr(4), 4)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn removing_a_waiting_endpoint_cancels_it() {
        let (tx, mut stream) = jitter(1.0);
        insert(&tx, 1, 1).await;
        assert!(drain_now(&mut stream).is_empty());

        remove(&tx, 1).await;
        assert!(drain_now(&mut stream).is_empty());

        tokio::time::advance(MAX_JITTER).await;
        assert!(drain_now(&mut stream).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn readding_a_cancelled_endpoint_releases_it_once() {
        let (tx, mut stream) = jitter(1.0);
        let start = Instant::now();
        // The re-insert lands before the cancelled timer has been polled away,
        // so the stale timer must not release the new entry.
        insert(&tx, 1, 1).await;
        remove(&tx, 1).await;
        insert(&tx, 1, 10).await;
        assert!(drain_now(&mut stream).is_empty());

        assert_eq!(
            drain_timers(&mut stream, start).await,
            vec![Event::Insert(addr(1), 10)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn removals_release_waiting_endpoints_to_restore_the_ratio() {
        let (tx, mut stream) = jitter(0.5);
        for port in 1..=4 {
            insert(&tx, port, u32::from(port)).await;
        }
        assert_eq!(
            drain_now(&mut stream),
            vec![Event::Insert(addr(1), 1), Event::Insert(addr(3), 3)]
        );

        // 2 of 3 waiting exceeds the ratio, so one waiting endpoint is
        // released right after the removal.
        remove(&tx, 1).await;
        let events = drain_now(&mut stream);
        assert_eq!(events.len(), 2, "{events:?}");
        assert_eq!(events[0], Event::Remove(addr(1)));
        let still_waiting = match &events[1] {
            Event::Insert(promoted, _) if *promoted == addr(2) => 4,
            Event::Insert(promoted, _) if *promoted == addr(4) => 2,
            other => panic!("expected a waiting endpoint, got {other:?}"),
        };

        let start = Instant::now();
        assert_eq!(
            drain_timers(&mut stream, start).await,
            vec![Event::Insert(addr(still_waiting), u32::from(still_waiting))]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn reinserting_a_released_endpoint_passes_through() {
        let (tx, mut stream) = jitter(0.5);
        insert(&tx, 1, 1).await;
        assert_eq!(drain_now(&mut stream), vec![Event::Insert(addr(1), 1)]);

        insert(&tx, 1, 10).await;
        assert_eq!(drain_now(&mut stream), vec![Event::Insert(addr(1), 10)]);
    }

    #[tokio::test(start_paused = true)]
    async fn reinserting_a_waiting_endpoint_releases_the_newest_service_once() {
        let (tx, mut stream) = jitter(1.0);
        let start = Instant::now();
        insert(&tx, 1, 1).await;
        insert(&tx, 1, 10).await;
        assert!(drain_now(&mut stream).is_empty());

        assert_eq!(
            drain_timers(&mut stream, start).await,
            vec![Event::Insert(addr(1), 10)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn waiting_endpoints_are_released_after_discovery_ends() {
        let (tx, mut stream) = jitter(1.0);
        insert(&tx, 1, 1).await;
        drop(tx);
        assert!(stream.next().now_or_never().is_none(), "ended too early");

        let start = Instant::now();
        assert_eq!(
            stream.next().await.map(|item| event(item.unwrap())),
            Some(Event::Insert(addr(1), 1))
        );
        assert!(start.elapsed() <= MAX_JITTER);
        assert!(stream.next().await.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn discovery_errors_pass_through() {
        let (tx, mut stream) = jitter(1.0);
        tx.send(Err("boom".into())).await.unwrap();
        let err = stream.next().now_or_never().unwrap().unwrap().unwrap_err();
        assert_eq!(err.to_string(), "boom");
    }
}
