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

//! Utilities for async operations.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// A pinned, boxed, `Send` future.
///
/// This is the boxed future type surfaced by the public transport (connector)
/// and load-balancing service interfaces (e.g. [`Connector::connect`]).
///
/// [`Connector::connect`]: crate::Connector::connect
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// A [`tokio::task::JoinHandle`] wrapper that aborts the task when dropped.
pub(crate) struct AbortOnDrop(pub(crate) tokio::task::JoinHandle<()>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// A stream fed by a spawned producer task that the stream owns.
///
/// Dropping the stream aborts the producer.
pub(crate) struct TaskStream<T> {
    rx: ReceiverStream<T>,
    _task: AbortOnDrop,
}

impl<T> TaskStream<T> {
    /// Spawns `producer` with the sending half of a channel of `capacity`
    /// and returns the stream of what it sends.
    pub(crate) fn spawn<P, F>(capacity: usize, producer: P) -> Self
    where
        P: FnOnce(mpsc::Sender<T>) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel(capacity);
        let task = tokio::spawn(producer(tx));
        Self {
            rx: ReceiverStream::new(rx),
            _task: AbortOnDrop(task),
        }
    }
}

impl<T> Stream for TaskStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        Pin::new(&mut self.rx).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}
