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

use super::{
    super::{Connection, Endpoint},
    Executor,
};

use std::{
    collections::HashMap,
    future::{Future, poll_fn},
    hash::Hash,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
use tokio::sync::{mpsc::Receiver, oneshot};
use tokio_stream::Stream;
use tower::discover::Change as TowerChange;

/// A change in the service set.
#[derive(Debug, Clone)]
pub enum Change<K, V> {
    /// A new service identified by key `K` was identified.
    Insert(K, V),
    /// The service identified by key `K` disappeared.
    Remove(K),
}

// Only the desired endpoints are retained, so idle discovery does not accumulate
// an event queue. Generations distinguish replacements under the same key.
struct Snapshot<K> {
    endpoints: HashMap<K, DesiredEndpoint>,
    generation: u64,
    waker: Option<Waker>,
}

struct DesiredEndpoint {
    generation: u64,
    // Preserved across replacements, reset after an explicit removal.
    first_generation: u64,
    endpoint: Endpoint,
}

pub(crate) struct DynamicServiceStream<K: Hash + Eq + Clone> {
    snapshot: Arc<Mutex<Snapshot<K>>>,
    emitted: HashMap<K, u64>,
    // Dropping the adapter cancels the receiver even if senders remain alive.
    _cancel: oneshot::Sender<()>,
}

impl<K: Hash + Eq + Clone + Send + 'static> DynamicServiceStream<K> {
    pub(crate) fn new<E>(mut changes: Receiver<Change<K, Endpoint>>, executor: &E) -> Self
    where
        E: Executor<Pin<Box<dyn Future<Output = ()> + Send>>>,
    {
        let snapshot = Arc::new(Mutex::new(Snapshot {
            endpoints: HashMap::new(),
            generation: 0,
            waker: None,
        }));
        let shared = snapshot.clone();
        let (cancel, mut cancelled) = oneshot::channel();
        executor.execute(Box::pin(async move {
            // Poll cancellation alongside recv without requiring Tokio's macros
            // feature in channel-only builds.
            while let Some(change) = poll_fn(|cx| {
                if Pin::new(&mut cancelled).poll(cx).is_ready() {
                    return Poll::Ready(None);
                }
                changes.poll_recv(cx)
            })
            .await
            {
                let waker = {
                    let mut snapshot = shared.lock().unwrap();
                    match change {
                        Change::Insert(key, endpoint) => {
                            snapshot.generation += 1;
                            let generation = snapshot.generation;
                            let first_generation = snapshot
                                .endpoints
                                .get(&key)
                                .map_or(generation, |desired| desired.first_generation);
                            snapshot.endpoints.insert(
                                key,
                                DesiredEndpoint {
                                    generation,
                                    first_generation,
                                    endpoint,
                                },
                            );
                        }
                        Change::Remove(key) => {
                            if snapshot.endpoints.remove(&key).is_none() {
                                continue;
                            }
                        }
                    }
                    snapshot.waker.take()
                };
                if let Some(waker) = waker {
                    waker.wake();
                }
            }
            // Closing discovery preserves the final snapshot for future RPCs.
        }));
        Self {
            snapshot,
            emitted: HashMap::new(),
            _cancel: cancel,
        }
    }
}

impl<K: Hash + Eq + Clone> Stream for DynamicServiceStream<K> {
    type Item = Result<TowerChange<K, Connection>, crate::BoxError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut snapshot = this.snapshot.lock().unwrap();
        if let Some(key) = this
            .emitted
            .iter()
            .find(|(key, generation)| {
                snapshot
                    .endpoints
                    .get(*key)
                    .is_none_or(|desired| **generation < desired.first_generation)
            })
            .map(|(key, _)| key)
            .cloned()
        {
            this.emitted.remove(&key);
            return Poll::Ready(Some(Ok(TowerChange::Remove(key))));
        }
        if let Some((key, desired)) = snapshot
            .endpoints
            .iter()
            .find(|(key, desired)| this.emitted.get(*key) != Some(&desired.generation))
        {
            let key = key.clone();
            let endpoint = desired.endpoint.clone();
            this.emitted.insert(key.clone(), desired.generation);
            drop(snapshot);
            let connection = Connection::lazy(endpoint.http_connector(), endpoint);
            return Poll::Ready(Some(Ok(TowerChange::Insert(key, connection))));
        }
        // Registration shares the publication lock, avoiding a missed update
        // between checking the snapshot and registering for the next change.
        snapshot.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl<K: Hash + Eq + Clone> Unpin for DynamicServiceStream<K> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::{Channel, channel::BoxFuture};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;
    use tokio::sync::mpsc::channel;

    #[derive(Clone, Default)]
    struct RecordingExecutor(Arc<Mutex<Vec<BoxFuture<'static, ()>>>>);

    impl Executor<BoxFuture<'static, ()>> for RecordingExecutor {
        fn execute(&self, future: BoxFuture<'static, ()>) {
            self.0.lock().unwrap().push(future);
        }
    }

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn reconciles_idle_snapshot() {
        let executor = RecordingExecutor::default();
        let (tx, rx) = channel(1);
        let mut stream = DynamicServiceStream::new(rx, &executor);
        // Manual polling keeps this test in one Tokio task; disable its
        // cooperative budget so each poll drains the queued change.
        let mut receiver = Box::pin(tokio::task::unconstrained(
            executor.0.lock().unwrap().pop().unwrap(),
        ));
        let wakes = Arc::new(WakeCount::default());
        let waker = Waker::from(wakes.clone());
        let cx = &mut Context::from_waker(&waker);
        let receiver_cx = &mut Context::from_waker(Waker::noop());
        let a = Endpoint::from_static("http://127.0.0.1:1");
        let b = Endpoint::from_static("http://127.0.0.1:2");

        assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());
        assert_eq!(wakes.0.load(Ordering::SeqCst), 0);
        tx.try_send(Change::Insert(0, a.clone())).unwrap();
        assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Insert(0, _))))
        ));
        let first_generation = stream.emitted[&0];

        // Do not poll discovery while replacements and transient keys arrive.
        for change in [
            Change::Insert(0, a.clone()),
            Change::Insert(0, b.clone()),
            Change::Insert(1, a.clone()),
            Change::Remove(1),
        ] {
            tx.try_send(change).unwrap();
            assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        }
        assert_eq!(
            stream.snapshot.lock().unwrap().endpoints[&0].endpoint.uri(),
            b.uri()
        );
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Insert(0, _))))
        ));
        assert_ne!(stream.emitted[&0], first_generation);
        assert_eq!(
            stream.emitted[&0],
            stream.snapshot.lock().unwrap().endpoints[&0].generation
        );
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());

        // An idle remove/reinsert must evict the old ready service before
        // inserting its replacement, even across further same-key inserts.
        for change in [
            Change::Remove(0),
            Change::Insert(0, a.clone()),
            Change::Insert(0, b.clone()),
        ] {
            tx.try_send(change).unwrap();
            assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        }
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Remove(0))))
        ));
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Insert(0, _))))
        ));
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());

        tx.try_send(Change::Remove(0)).unwrap();
        assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Remove(0))))
        ));
        for change in [
            Change::Insert(0, a.clone()),
            Change::Remove(0),
            Change::Insert(0, b.clone()),
        ] {
            tx.try_send(change).unwrap();
            assert!(receiver.as_mut().poll(receiver_cx).is_pending());
        }
        assert!(matches!(
            Pin::new(&mut stream).poll_next(cx),
            Poll::Ready(Some(Ok(TowerChange::Insert(0, _))))
        ));
        for key in 1..1000 {
            for change in [Change::Insert(key, a.clone()), Change::Remove(key)] {
                tx.try_send(change).unwrap();
                assert!(receiver.as_mut().poll(receiver_cx).is_pending());
            }
        }
        assert_eq!(stream.snapshot.lock().unwrap().endpoints.len(), 1);
        assert_eq!(stream.emitted.len(), 1);
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());
        let idle_wakes = wakes.0.load(Ordering::SeqCst);
        assert!(receiver.as_mut().poll(cx).is_pending());
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());
        assert_eq!(wakes.0.load(Ordering::SeqCst), idle_wakes);

        drop(tx);
        assert!(receiver.as_mut().poll(receiver_cx).is_ready());
        assert_eq!(stream.snapshot.lock().unwrap().endpoints.len(), 1);
        assert!(Pin::new(&mut stream).poll_next(cx).is_pending());
    }

    #[tokio::test]
    async fn channel_uses_supplied_executor() {
        let executor = RecordingExecutor::default();
        let (channel, tx) = Channel::balance_channel_with_executor(1, executor.clone());
        let mut tasks = std::mem::take(&mut *executor.0.lock().unwrap());
        assert_eq!(tasks.len(), 2);
        tx.try_send(Change::<usize, _>::Remove(0)).unwrap();
        assert!(matches!(
            tx.try_send(Change::Remove(1)),
            Err(tokio::sync::mpsc::error::TrySendError::Full(_))
        ));
        let mut receiver = tasks.remove(0);
        let mut worker = tasks.pop().unwrap();
        let cx = &mut Context::from_waker(Waker::noop());
        assert!(receiver.as_mut().poll(cx).is_pending());
        assert_eq!(tx.capacity(), 1);
        drop(channel);
        assert!(worker.as_mut().poll(cx).is_ready());
        drop(worker);
        assert!(receiver.as_mut().poll(cx).is_ready());
        assert!(tx.is_closed());
    }

    #[tokio::test]
    async fn last_channel_clone_cancels_receiver() {
        let (channel, tx) = Channel::balance_channel::<usize>(1);
        let clone = channel.clone();
        drop(channel);
        tx.send(Change::Remove(0)).await.unwrap();
        drop(
            tokio::time::timeout(std::time::Duration::from_secs(1), tx.reserve())
                .await
                .unwrap()
                .unwrap(),
        );
        assert!(!tx.is_closed());
        drop(clone);
        tokio::time::timeout(std::time::Duration::from_secs(1), tx.closed())
            .await
            .expect("dropping the last channel must close discovery");
    }
}
