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

//! Type-erased versions of the client's traits which are not dyn compatible.

use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;

use crate::async_trait;
use crate::client::CallOptions;
use crate::client::Invoke;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream;
use crate::client::interceptor::Intercept;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::core::StreamEnded;

/// A dyn-compatible version of [`Invoke`].
///
/// To erase an [`Invoke`], use [`BoxInvoker::new`] or [`ArcInvoker::new`].
#[async_trait]
trait DynInvoke: Send + Sync {
    /// A dynamic version of [`Invoke::invoke`].
    async fn dyn_invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (BoxSendStream, BoxRecvStream);
}

/// Every [`Invoke`] is a [`DynInvoke`], so a `Box` or `Arc` of one can be
/// converted into a [`BoxInvoker`] or [`ArcInvoker`] without allocating.
///
/// The returned streams are boxed, except for streams that are already a
/// [`BoxSendStream`] or [`BoxRecvStream`], which are passed through unchanged.
#[async_trait]
impl<I: Invoke + Send> DynInvoke for I {
    async fn dyn_invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (BoxSendStream, BoxRecvStream) {
        let (tx, rx) = self.invoke(headers, options).await;
        (BoxSendStream::new(tx), BoxRecvStream::new(rx))
    }
}

/// A type-erased [`Invoke`] implementation returning boxed streams.
pub struct BoxInvoker(Box<dyn DynInvoke>);

impl BoxInvoker {
    /// Wraps `invoke` in a `BoxInvoker`.  The streams it returns are boxed.
    /// If `invoke` is already a `BoxInvoker`, it is returned as-is rather than
    /// boxed again.
    ///
    /// To convert an existing `Box` without allocating, use
    /// [`BoxInvoker::from_box`].
    pub fn new<I: Invoke + Send + 'static>(invoke: I) -> Self {
        // Both type IDs are known at compile time, so this check is free in
        // optimized builds.
        if TypeId::of::<I>() == TypeId::of::<Self>() {
            return take_as::<I, Self>(invoke);
        }
        Self(Box::new(invoke))
    }

    /// Converts a boxed `invoke` into a `BoxInvoker` without allocating.  The
    /// streams it returns are boxed.
    pub fn from_box<I: Invoke + Send + 'static>(invoke: Box<I>) -> Self {
        Self(invoke)
    }
}

impl Debug for BoxInvoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxInvoker").finish_non_exhaustive()
    }
}

impl Invoke for BoxInvoker {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        self.0.dyn_invoke(headers, options).await
    }
}

/// A type-erased, cloneable [`Invoke`] implementation returning boxed streams.
#[derive(Clone)]
pub struct ArcInvoker(Arc<dyn DynInvoke>);

impl ArcInvoker {
    /// Wraps `invoke` in an `ArcInvoker`.  The streams it returns are boxed.
    /// If `invoke` is already an `ArcInvoker`, it is returned as-is rather than
    /// wrapped again.
    ///
    /// To convert an existing `Arc` without allocating, use
    /// [`ArcInvoker::from_arc`].
    pub fn new<I: Invoke + Send + 'static>(invoke: I) -> Self {
        // Both type IDs are known at compile time, so this check is free in
        // optimized builds.
        if TypeId::of::<I>() == TypeId::of::<Self>() {
            return take_as::<I, Self>(invoke);
        }
        Self(Arc::new(invoke))
    }

    /// Converts a shared `invoke` into an `ArcInvoker` without allocating.  The
    /// streams it returns are boxed.
    pub fn from_arc<I: Invoke + Send + 'static>(invoke: Arc<I>) -> Self {
        Self(invoke)
    }
}

impl Debug for ArcInvoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcInvoker").finish_non_exhaustive()
    }
}

impl Invoke for ArcInvoker {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        self.0.dyn_invoke(headers, options).await
    }
}

/// A dyn-compatible version of [`Intercept<&ArcInvoker>`].
///
/// To erase an [`Intercept`], use [`ArcInterceptor::new`].
#[async_trait]
trait DynIntercept: Send + Sync {
    /// A dynamic version of [`Intercept::intercept`].
    async fn dyn_intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        next: &ArcInvoker,
    ) -> (BoxSendStream, BoxRecvStream);
}

/// Every [`Intercept<&ArcInvoker>`] is a [`DynIntercept`], so an `Arc` of one
/// can be converted into an [`ArcInterceptor`] without allocating.
///
/// The returned streams are boxed, except for streams that are already a
/// [`BoxSendStream`] or [`BoxRecvStream`], which are passed through unchanged.
#[async_trait]
impl<I> DynIntercept for I
where
    I: for<'a> Intercept<&'a ArcInvoker> + Send,
{
    async fn dyn_intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        next: &ArcInvoker,
    ) -> (BoxSendStream, BoxRecvStream) {
        let (tx, rx) = self.intercept(headers, options, next).await;
        (BoxSendStream::new(tx), BoxRecvStream::new(rx))
    }
}

/// A type-erased, cloneable [`Intercept`] implementation that calls an
/// [`ArcInvoker`] and returns boxed streams.
///
/// Like [`Intercepted`](crate::client::interceptor::Intercepted), it passes
/// `next` by reference, so it accepts both interceptors that call `next` once
/// (`impl<I: InvokeOnce> Intercept<I>`) and interceptors that call it multiple
/// times (`impl<I: Invoke + Clone + 'static> Intercept<&I>`).  Interceptors
/// that need to keep `next` beyond the call may clone it.
#[derive(Clone)]
pub struct ArcInterceptor(Arc<dyn DynIntercept>);

impl ArcInterceptor {
    /// Wraps `intercept` in an `ArcInterceptor`.  The streams it returns are
    /// boxed.  If `intercept` is already an `ArcInterceptor`, it is returned
    /// as-is rather than wrapped again.
    ///
    /// To convert an existing `Arc` without allocating, use
    /// [`ArcInterceptor::from_arc`].
    pub fn new<I>(intercept: I) -> Self
    where
        I: for<'a> Intercept<&'a ArcInvoker> + Send + 'static,
    {
        // Both type IDs are known at compile time, so this check is free in
        // optimized builds.
        if TypeId::of::<I>() == TypeId::of::<Self>() {
            return take_as::<I, Self>(intercept);
        }
        Self(Arc::new(intercept))
    }

    /// Converts a shared `intercept` into an `ArcInterceptor` without
    /// allocating.  The streams it returns are boxed.
    pub fn from_arc<I>(intercept: Arc<I>) -> Self
    where
        I: for<'a> Intercept<&'a ArcInvoker> + Send + 'static,
    {
        Self(intercept)
    }
}

impl Debug for ArcInterceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcInterceptor").finish_non_exhaustive()
    }
}

impl Intercept<&ArcInvoker> for ArcInterceptor {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    async fn intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        next: &ArcInvoker,
    ) -> (Self::SendStream, Self::RecvStream) {
        self.0.dyn_intercept(headers, options, next).await
    }
}

/// A dyn-compatible version of [`SendStream`].
#[async_trait]
trait DynSendStream: Send {
    async fn dyn_send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded>;
}

#[async_trait]
impl<S: SendStream> DynSendStream for S {
    async fn dyn_send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        self.send(msg, options).await
    }
}

/// A type-erased [`SendStream`].
pub struct BoxSendStream(Box<dyn DynSendStream>);

impl BoxSendStream {
    /// Wraps `stream` in a `BoxSendStream`, boxing it.  If `stream` is already
    /// a `BoxSendStream`, it is returned as-is rather than boxed again.
    pub fn new<S: SendStream + 'static>(stream: S) -> Self {
        // Both type IDs are known at compile time, so this check is free in
        // optimized builds.
        if TypeId::of::<S>() == TypeId::of::<Self>() {
            return take_as::<S, Self>(stream);
        }
        Self(Box::new(stream))
    }
}

impl Debug for BoxSendStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxSendStream").finish_non_exhaustive()
    }
}

impl SendStream for BoxSendStream {
    async fn send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        self.0.dyn_send(msg, options).await
    }
}

/// A dyn-compatible version of [`RecvStream`].
#[async_trait]
trait DynRecvStream: Send {
    async fn dyn_recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem;
}

#[async_trait]
impl<R: RecvStream> DynRecvStream for R {
    async fn dyn_recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        self.recv(msg).await
    }
}

/// A type-erased [`RecvStream`].
pub struct BoxRecvStream(Box<dyn DynRecvStream>);

impl BoxRecvStream {
    /// Wraps `stream` in a `BoxRecvStream`, boxing it.  If `stream` is already
    /// a `BoxRecvStream`, it is returned as-is rather than boxed again.
    pub fn new<R: RecvStream + 'static>(stream: R) -> Self {
        // Both type IDs are known at compile time, so this check is free in
        // optimized builds.
        if TypeId::of::<R>() == TypeId::of::<Self>() {
            return take_as::<R, Self>(stream);
        }
        Self(Box::new(stream))
    }
}

impl Debug for BoxRecvStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxRecvStream").finish_non_exhaustive()
    }
}

impl RecvStream for BoxRecvStream {
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        self.0.dyn_recv(msg).await
    }
}

/// Moves `value` out as a `U`.  Panics unless `T` and `U` are the same type.
fn take_as<T: 'static, U: 'static>(value: T) -> U {
    let mut value = Some(value);
    (&mut value as &mut dyn Any)
        .downcast_mut::<Option<U>>()
        .and_then(Option::take)
        .expect("take_as called with different types")
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use super::*;
    use crate::StatusCodeError;
    use crate::StatusError;
    use crate::client::InvokeOnce;
    use crate::client::stream_util::FailingRecvStream;
    use crate::client::test_util::NopInvoker;

    // Wrapping an already-boxed stream returns the same box rather than
    // boxing it again.
    #[test]
    fn box_stream_new_does_not_rebox() {
        let (tx, rx) = FailingRecvStream::new_stream_pair(
            StatusError::new(StatusCodeError::Unavailable, "test"),
            None,
        );
        let tx_ptr = &*tx.0 as *const dyn DynSendStream as *const ();
        let rx_ptr = &*rx.0 as *const dyn DynRecvStream as *const ();
        let tx = BoxSendStream::new(tx);
        let rx = BoxRecvStream::new(rx);
        assert_eq!(&*tx.0 as *const dyn DynSendStream as *const (), tx_ptr);
        assert_eq!(&*rx.0 as *const dyn DynRecvStream as *const (), rx_ptr);
    }

    // Wrapping an already-boxed invoker returns the same box rather than
    // boxing it again.
    #[test]
    fn box_invoker_new_does_not_rebox() {
        let invoker = BoxInvoker::new(NopInvoker);
        let ptr = &*invoker.0 as *const dyn DynInvoke as *const ();
        let invoker = BoxInvoker::new(invoker);
        assert_eq!(&*invoker.0 as *const dyn DynInvoke as *const (), ptr);
    }

    // Wrapping an existing `ArcInvoker` returns it as-is rather than wrapping
    // it in another `Arc`.
    #[test]
    fn arc_invoker_new_does_not_rewrap() {
        let invoker = ArcInvoker::new(NopInvoker);
        let rewrapped = ArcInvoker::new(invoker.clone());
        assert!(Arc::ptr_eq(&invoker.0, &rewrapped.0));
    }

    // Wrapping an existing `ArcInterceptor` returns it as-is rather than
    // wrapping it in another `Arc`.
    #[test]
    fn arc_intercept_new_does_not_rewrap() {
        let intercept = ArcInterceptor::new(OnceInterceptor);
        let rewrapped = ArcInterceptor::new(intercept.clone());
        assert!(Arc::ptr_eq(&intercept.0, &rewrapped.0));
    }

    /// An invoker that counts the RPCs started on it.
    #[derive(Clone, Default)]
    struct CountingInvoker(Arc<AtomicUsize>);

    impl Invoke for CountingInvoker {
        type SendStream = <NopInvoker as Invoke>::SendStream;
        type RecvStream = <NopInvoker as Invoke>::RecvStream;

        async fn invoke(
            &self,
            headers: RequestHeaders,
            options: CallOptions,
        ) -> (Self::SendStream, Self::RecvStream) {
            self.0.fetch_add(1, Ordering::Relaxed);
            NopInvoker.invoke(headers, options).await
        }
    }

    /// An interceptor written against `InvokeOnce`, which calls `next` once.
    struct OnceInterceptor;

    impl<I: InvokeOnce> Intercept<I> for OnceInterceptor {
        type SendStream = I::SendStream;
        type RecvStream = I::RecvStream;

        async fn intercept(
            &self,
            headers: RequestHeaders,
            options: CallOptions,
            next: I,
        ) -> (Self::SendStream, Self::RecvStream) {
            next.invoke_once(headers, options).await
        }
    }

    /// An interceptor written against `&I`, which calls `next` twice.
    struct FanOutInterceptor;

    impl<I: Invoke + Clone + 'static> Intercept<&I> for FanOutInterceptor {
        type SendStream = I::SendStream;
        type RecvStream = I::RecvStream;

        async fn intercept(
            &self,
            headers: RequestHeaders,
            options: CallOptions,
            next: &I,
        ) -> (Self::SendStream, Self::RecvStream) {
            let _ = next.invoke(headers.clone(), options.clone()).await;
            next.invoke(headers, options).await
        }
    }

    // Both interceptor shapes documented on `Intercept` can be erased into an
    // `ArcInterceptor` and call through to `next`.
    #[tokio::test]
    async fn arc_intercept_accepts_documented_interceptor_shapes() {
        for (intercept, want_calls) in [
            (ArcInterceptor::new(OnceInterceptor), 1),
            (ArcInterceptor::new(FanOutInterceptor), 2),
        ] {
            let counter = CountingInvoker::default();
            let next = ArcInvoker::new(counter.clone());
            let _ = intercept
                .intercept(RequestHeaders::new(), CallOptions::default(), &next)
                .await;
            assert_eq!(counter.0.load(Ordering::Relaxed), want_calls);
        }
    }
}
