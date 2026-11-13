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

//! Client-side enforcement of RPC deadlines.
//!
//! Note that the `grpc-timeout` header is not set here; it is the
//! responsibility of the transport, which can compute the timeout immediately
//! before the request headers are written.

use std::future::poll_fn;
use std::pin::Pin;
use std::pin::pin;
use std::task::Poll;
use std::time::Instant;

use tokio::sync::oneshot;

use crate::StatusCodeError;
use crate::StatusError;
use crate::client::CallOptions;
use crate::client::Invoke;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream;
use crate::client::Trailers;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::core::StreamEnded;
use crate::rt::GrpcRuntime;
use crate::rt::Sleep;

fn deadline_exceeded() -> StatusError {
    StatusError::new(StatusCodeError::DeadlineExceeded, "deadline exceeded")
}

/// Starts calls on another layer, enforcing the deadline in each call's
/// [`CallOptions`].
///
/// To avoid the cost of spawning a timer task per call, the deadline is only
/// enforced while the call is starting or a stream is actively being used:
///
/// * a pending or future call to [`SendStream::send`] fails with
///   [`StreamEnded`],
/// * a pending or future call to [`RecvStream::recv`] cancels the RPC by
///   dropping the underlying [`RecvStream`] and returns trailers with a
///   `DeadlineExceeded` status (unless trailers were already returned).
///
/// A call that is idle when its deadline expires is not cancelled until one of
/// the above happens or the [`RecvStream`] is dropped.  Also, any data received
/// in this window may not be returned from [`RecvStream::recv`].
///
/// If the [`SendStream`] is dropped after the deadline expires, the underlying
/// [`SendStream`] is not dropped until the underlying [`RecvStream`] is, which
/// avoids sending a half-close instead.
///
/// If a call has no deadline, its streams are simply pass-through wrappers of
/// the underlying streams.
pub(crate) struct DeadlineInvoker<I> {
    runtime: GrpcRuntime,
    next: I,
}

impl<I> DeadlineInvoker<I> {
    /// Returns a `DeadlineInvoker` that starts calls on `next`.
    pub(crate) fn new(runtime: GrpcRuntime, next: I) -> Self {
        Self { runtime, next }
    }

    /// Returns the layer that calls are started on.
    pub(crate) fn next(&self) -> &I {
        &self.next
    }
}

impl<I: Invoke> Invoke for DeadlineInvoker<I> {
    type SendStream = DeadlineSendStream<I::SendStream>;
    type RecvStream = DeadlineRecvStream<I::SendStream, I::RecvStream>;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        let call_deadline = options.deadline();
        let mut deadline = Deadline::new(&self.runtime, call_deadline);
        let Ok((tx, rx)) = deadline.run(self.next.invoke(headers, options)).await else {
            return (
                DeadlineSendStream {
                    tx: None,
                    deadline,
                    park_tx: None,
                },
                DeadlineRecvStream(RecvState::InvokeTimeout),
            );
        };
        let (park_tx, parked_tx) = oneshot::channel();
        (
            DeadlineSendStream {
                tx: Some(tx),
                deadline: Deadline::new(&self.runtime, call_deadline),
                park_tx: Some(park_tx),
            },
            // Reuse the exisitng `deadline` here as `send` is less likely to need
            // to create a `sleep` internally (it writes to a buffer/channel
            // internally) than `recv` (it reads from the network).
            DeadlineRecvStream(RecvState::Active {
                rx,
                parked_tx,
                deadline,
            }),
        )
    }
}

/// Optionally stores and tracks a deadline, and runs futures bounded by it.
enum Deadline {
    /// The call has no deadline.
    None,
    /// The deadline has not yet expired.
    Pending {
        runtime: GrpcRuntime,
        deadline: Instant,
        /// Completes when the deadline expires.  Created lazily, the first
        /// time an operation has to wait, since most `invoke` and `send`
        /// operations complete immediately.  Always discarded upon expiration
        /// to avoid polling after completion.
        sleep: Option<Pin<Box<dyn Sleep>>>,
    },
    /// The deadline has expired.
    Expired,
}

impl Deadline {
    /// Creates a new instance bounded by `deadline`, if set.
    fn new(runtime: &GrpcRuntime, deadline: Option<Instant>) -> Self {
        match deadline {
            None => Self::None,
            Some(deadline) => Self::Pending {
                runtime: runtime.clone(),
                deadline,
                sleep: None,
            },
        }
    }

    /// Returns whether the deadline is already expired.
    fn is_expired(&self) -> bool {
        match self {
            Deadline::None => false,
            // Check the clock explicitly, as `sleep` may not have been polled to
            // completion already, and it would be improper to poll it now.
            Deadline::Pending {
                runtime, deadline, ..
            } => runtime.now() >= *deadline,
            Deadline::Expired => true,
        }
    }

    /// Runs `fut` until it completes, or fails with DeadlineExceeded if the
    /// deadline expires first.  If the deadline has already expired, `fut` is
    /// not polled.
    async fn run<F: Future>(&mut self, fut: F) -> Result<F::Output, StatusError> {
        match self {
            Self::None => Ok(fut.await),
            Self::Pending {
                runtime,
                deadline,
                sleep,
            } => {
                let deadline = *deadline;
                // Check the clock explicitly rather than relying on `sleep`
                // alone: a sleep is not ready on its first poll (when tokio
                // registers its timer), however far past due it is.
                if runtime.now() >= deadline {
                    *self = Self::Expired;
                    return Err(deadline_exceeded());
                }
                // Poll `fut` once before creating `sleep`, so that operations
                // that complete immediately don't need a timer.
                let mut fut = pin!(fut);
                if let Poll::Ready(out) = poll_fn(|cx| Poll::Ready(fut.as_mut().poll(cx))).await {
                    return Ok(out);
                }
                let sleep = sleep.get_or_insert_with(|| {
                    runtime.sleep(deadline.saturating_duration_since(runtime.now()))
                });
                tokio::select! {
                    biased;
                    _ = sleep.as_mut() => {
                        *self = Self::Expired;
                        Err(deadline_exceeded())
                    },
                    out = fut => Ok(out),
                }
            }
            Self::Expired => Err(deadline_exceeded()),
        }
    }
}

pub(crate) struct DeadlineSendStream<SS> {
    /// The underlying `SendStream` in an `Option` so it can be moved to the
    /// paired `DeadlineRecvStream`, or to represent a stream that timed out
    /// during invoke.
    tx: Option<SS>,
    deadline: Deadline,
    /// Used to send `tx` to the paired `DeadlineRecvStream` if we are dropped
    /// after the deadline expired, to prevent an improper half-close signal.
    park_tx: Option<oneshot::Sender<SS>>,
}

impl<SS: SendStream + Send> SendStream for DeadlineSendStream<SS> {
    async fn send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        let Some(tx) = &mut self.tx else {
            return Err(StreamEnded);
        };
        // Even if the deadline expires during this call, keep `tx` to avoid a
        // half-close.
        self.deadline
            .run(tx.send(msg, options))
            .await
            .unwrap_or(Err(StreamEnded))
    }
}

impl<SS> Drop for DeadlineSendStream<SS> {
    fn drop(&mut self) {
        // Dropping `tx` half-closes the stream unless the deadline has expired.
        // In that case, send it to the paired `DeadlineRecvStream` to be
        // dropped after the `RecvStream`.  If the `DeadlineRecvStream` is
        // already dropped, `send` fails and `tx` is dropped here.
        if let Some(tx) = self.tx.take()
            && self.deadline.is_expired()
            && let Some(park_tx) = self.park_tx.take()
        {
            let _ = park_tx.send(tx);
        }
    }
}

pub(crate) struct DeadlineRecvStream<SS, RS>(RecvState<SS, RS>);

enum RecvState<SS, RS> {
    InvokeTimeout,
    Active {
        /// The underlying `RecvStream`.  Declared before `parked_tx` so that
        /// it is dropped first.
        rx: RS,
        /// Holds the underlying `SendStream` if the `DeadlineSendStream` was
        /// dropped after the deadline was reached.
        parked_tx: oneshot::Receiver<SS>,
        deadline: Deadline,
    },
    StreamClosed,
}

impl<SS, RS> RecvStream for DeadlineRecvStream<SS, RS>
where
    SS: Send,
    RS: RecvStream + Send,
{
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        match &mut self.0 {
            RecvState::InvokeTimeout => {
                self.0 = RecvState::StreamClosed;
                ResponseStreamItem::Trailers(Trailers::new(Err(deadline_exceeded())))
            }
            RecvState::Active { rx, deadline, .. } => {
                let item = deadline.run(rx.recv(msg)).await.unwrap_or_else(|status| {
                    ResponseStreamItem::Trailers(Trailers::new(Err(status)))
                });
                if matches!(item, ResponseStreamItem::Trailers(_)) {
                    // Drops `rx`, cancelling the call if it is still running.
                    self.0 = RecvState::StreamClosed;
                }
                item
            }
            RecvState::StreamClosed => ResponseStreamItem::StreamClosed,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::time::Duration;

    use bytes::Bytes;

    use super::*;
    use crate::client::test_util::ByteSendMsg;
    use crate::client::test_util::NopRecvMessage;
    use crate::rt::tokio::TokioRuntime;

    type Events = Arc<Mutex<Vec<&'static str>>>;

    /// An invoker whose streams record when they are dropped.
    #[derive(Default)]
    struct FakeInvoker {
        events: Events,
        /// If set, `invoke` never completes.
        block_invoke: bool,
        /// If set, `send` never completes.
        block_send: bool,
        /// Items returned by `recv`, in order.  Once exhausted, `recv`
        /// never completes.
        recv_items: Mutex<VecDeque<ResponseStreamItem>>,
    }

    impl FakeInvoker {
        fn events(&self) -> Vec<&'static str> {
            self.events.lock().unwrap().clone()
        }
    }

    // Implemented on a reference so that tests can inspect the invoker after
    // passing it to a `DeadlineInvoker`.
    impl Invoke for &FakeInvoker {
        type SendStream = FakeSendStream;
        type RecvStream = FakeRecvStream;

        async fn invoke(
            &self,
            _headers: RequestHeaders,
            _options: CallOptions,
        ) -> (FakeSendStream, FakeRecvStream) {
            self.events.lock().unwrap().push("invoke");
            if self.block_invoke {
                std::future::pending::<()>().await;
            }
            (
                FakeSendStream {
                    events: self.events.clone(),
                    block: self.block_send,
                },
                FakeRecvStream {
                    events: self.events.clone(),
                    items: std::mem::take(&mut self.recv_items.lock().unwrap()),
                },
            )
        }
    }

    struct FakeSendStream {
        events: Events,
        block: bool,
    }

    impl SendStream for FakeSendStream {
        async fn send(&mut self, _: &dyn SendMessage, _: SendOptions) -> Result<(), StreamEnded> {
            self.events.lock().unwrap().push("send");
            if self.block {
                std::future::pending::<()>().await;
            }
            Ok(())
        }
    }

    impl Drop for FakeSendStream {
        fn drop(&mut self) {
            self.events.lock().unwrap().push("tx dropped");
        }
    }

    struct FakeRecvStream {
        events: Events,
        items: VecDeque<ResponseStreamItem>,
    }

    impl RecvStream for FakeRecvStream {
        async fn recv(&mut self, _: &mut dyn RecvMessage) -> ResponseStreamItem {
            match self.items.pop_front() {
                Some(item) => item,
                None => std::future::pending().await,
            }
        }
    }

    impl Drop for FakeRecvStream {
        fn drop(&mut self) {
            self.events.lock().unwrap().push("rx dropped");
        }
    }

    const TIMEOUT: Duration = Duration::from_secs(1);

    fn runtime() -> GrpcRuntime {
        GrpcRuntime::new(TokioRuntime::default())
    }

    async fn invoke(
        invoker: &FakeInvoker,
        deadline: impl Into<Option<Instant>>,
    ) -> (
        DeadlineSendStream<FakeSendStream>,
        DeadlineRecvStream<FakeSendStream, FakeRecvStream>,
    ) {
        let mut options = CallOptions::new();
        if let Some(deadline) = deadline.into() {
            options.set_deadline(deadline);
        }
        DeadlineInvoker::new(runtime(), invoker)
            .invoke(RequestHeaders::new(), options)
            .await
    }

    async fn send(tx: &mut impl SendStream) -> Result<(), StreamEnded> {
        tx.send(&ByteSendMsg::new(&Bytes::new()), SendOptions::default())
            .await
    }

    fn assert_deadline_exceeded(item: ResponseStreamItem) {
        let ResponseStreamItem::Trailers(trailers) = item else {
            panic!("expected trailers, got {item:?}");
        };
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            StatusCodeError::DeadlineExceeded
        );
    }

    fn assert_closed(item: ResponseStreamItem) {
        assert!(
            matches!(item, ResponseStreamItem::StreamClosed),
            "expected StreamClosed, got {item:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn forwards_before_or_without_deadline() {
        for deadline in [Some(runtime().now() + TIMEOUT), None] {
            let invoker = FakeInvoker {
                recv_items: Mutex::new(VecDeque::from([
                    ResponseStreamItem::Message,
                    ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
                ])),
                ..Default::default()
            };
            let (mut tx, mut rx) = invoke(&invoker, deadline).await;
            if deadline.is_none() {
                // Without a deadline, the call never expires.
                runtime().sleep(TIMEOUT * 2).await;
            }
            assert!(send(&mut tx).await.is_ok());
            assert!(matches!(
                rx.recv(&mut NopRecvMessage).await,
                ResponseStreamItem::Message
            ));
            assert!(matches!(
                rx.recv(&mut NopRecvMessage).await,
                ResponseStreamItem::Trailers(_)
            ));
            // The underlying `RecvStream` is dropped once trailers are
            // returned.
            assert_eq!(invoker.events(), ["invoke", "send", "rx dropped"]);
            assert_closed(rx.recv(&mut NopRecvMessage).await);

            // Dropping the send stream half-closes as usual.
            drop(tx);
            assert_eq!(
                invoker.events(),
                ["invoke", "send", "rx dropped", "tx dropped"]
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_expired_before_or_during_invoke() {
        // An already-expired deadline does not start the call; one that
        // expires while the call is being started abandons it.
        for (deadline, expected_events) in [
            (runtime().now(), &[][..]),
            (runtime().now() + TIMEOUT, &["invoke"][..]),
        ] {
            let invoker = FakeInvoker {
                block_invoke: true,
                ..Default::default()
            };
            let (mut tx, mut rx) = invoke(&invoker, deadline).await;
            assert!(send(&mut tx).await.is_err());
            assert_deadline_exceeded(rx.recv(&mut NopRecvMessage).await);
            assert_closed(rx.recv(&mut NopRecvMessage).await);
            assert_eq!(invoker.events(), expected_events);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn idle_call_cancels_before_half_close() {
        let invoker = FakeInvoker::default();
        let (tx, mut rx) = invoke(&invoker, runtime().now() + TIMEOUT).await;

        // The deadline is not enforced while the call is idle.
        runtime().sleep(TIMEOUT * 2).await;
        assert_eq!(invoker.events(), ["invoke"]);

        // Dropping the send stream after the deadline does not half-close.
        drop(tx);
        assert_eq!(invoker.events(), ["invoke"]);

        assert_deadline_exceeded(rx.recv(&mut NopRecvMessage).await);
        assert_eq!(invoker.events(), ["invoke", "rx dropped", "tx dropped"]);
        assert_closed(rx.recv(&mut NopRecvMessage).await);
    }

    // Verifies that a response that is already available is not returned if
    // the deadline expires before the application reads it.
    #[tokio::test(start_paused = true)]
    async fn response_read_after_deadline_is_not_returned() {
        let invoker = FakeInvoker {
            recv_items: Mutex::new(VecDeque::from([
                ResponseStreamItem::Message,
                ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
            ])),
            ..Default::default()
        };
        let (_tx, mut rx) = invoke(&invoker, runtime().now() + TIMEOUT).await;

        runtime().sleep(TIMEOUT * 2).await;
        assert_deadline_exceeded(rx.recv(&mut NopRecvMessage).await);
        assert_closed(rx.recv(&mut NopRecvMessage).await);
    }

    #[tokio::test(start_paused = true)]
    async fn send_timeout_cancels_before_half_close() {
        let invoker = FakeInvoker {
            block_send: true,
            ..Default::default()
        };
        let (mut tx, rx) = invoke(&invoker, runtime().now() + TIMEOUT).await;

        assert!(send(&mut tx).await.is_err());
        // Sends continue to fail without reaching the transport.
        assert!(send(&mut tx).await.is_err());
        drop(tx);
        assert_eq!(invoker.events(), ["invoke", "send"]);
        drop(rx);
        assert_eq!(
            invoker.events(),
            ["invoke", "send", "rx dropped", "tx dropped"]
        );
    }

    // Verifies that, if the send stream is dropped after its send expires while
    // a concurrent recv still holds the underlying RecvStream, the underlying
    // SendStream is not dropped (half-closing the stream) until the
    // RecvStream is dropped (cancelling it).
    #[tokio::test(start_paused = true)]
    async fn send_expires_during_recv_cancels_before_half_closing() {
        let invoker = FakeInvoker {
            block_send: true,
            ..Default::default()
        };
        let (mut tx, mut rx) = invoke(&invoker, runtime().now() + TIMEOUT).await;

        // Start a recv operation.
        let mut msg = NopRecvMessage;
        let recv = rx.recv(&mut msg);

        // Then perform a send, expecting timeout.
        assert!(send(&mut tx).await.is_err());
        assert_eq!(invoker.events(), ["invoke", "send"]);
        // Drop the send side, which should have no effect.
        drop(tx);
        assert_eq!(invoker.events(), ["invoke", "send"]);

        // Now poll recv, which should trigger cancellation.
        assert_deadline_exceeded(recv.await);
        assert_eq!(
            invoker.events(),
            ["invoke", "send", "rx dropped", "tx dropped"]
        );
    }
}
