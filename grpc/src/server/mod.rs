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

//! Server-side gRPC implementation and utilities.
//!
//! This module provides the core types and traits for building gRPC servers.
//! While most applications will use generated code (e.g. using
//! [`protoc-gen-rust-grpc`](https://crates.io/protoc-gen-rust-grpc)) to
//! interact with gRPC services, this module provides the underlying primitives.
//!
//! # Key Concepts
//!
//! - **[`Server`]:** The main entry point for server-side gRPC operations.
//! - **[`Listener`]:** A trait for accepting incoming RPCs.
//! - **[`Handle`]:** A trait implemented by services to handle incoming RPCs.
//!
//! # Additional Types
//!
//! - **[`Call`]:** Represents an incoming RPC accepted by a [`Listener`].
//! - **[`SendStream`] / [`RecvStream`]:** Represent the sending and receiving
//!   sides of a server-side RPC.
//! - **[`RequestHeaders`]:** Represents gRPC headers sent by the client to
//!   initiate a request.
//! - **[`ResponseHeaders`] / [`Trailers`]:** Represent gRPC headers and
//!   trailers sent to the client during the server's response.

use std::sync::Arc;

use tokio::sync::oneshot;
use tonic::async_trait;

use crate::client::CallOptions;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::metadata::MetadataMap;

pub(crate) mod interceptor;

pub struct Server {
    handler: Option<Arc<dyn DynHandle>>,
}

pub struct Call<SS, RS> {
    pub headers: RequestHeaders,
    pub send: SS,
    pub recv: RS,
    pub trailers_tx: oneshot::Sender<Trailers>,
}

#[trait_variant::make(Send)]
pub trait Listener {
    type SendStream: SendStream + 'static;
    type RecvStream: RecvStream + 'static;
    async fn accept(&self) -> Option<Call<Self::SendStream, Self::RecvStream>>;
}

impl Server {
    pub fn new() -> Self {
        Self { handler: None }
    }

    pub fn set_handler<H>(&mut self, h: H)
    where
        H: Handle + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(h))
    }

    pub async fn serve(&self, l: &impl Listener) {
        while let Some(call) = l.accept().await {
            let mut send: Box<dyn DynSendStream> = Box::new(call.send);
            let recv = BoxedRecvStream(Box::new(call.recv));
            let options = CallOptions::default();
            let trailers_tx = call.trailers_tx;
            let trailers = self
                .handler
                .as_ref()
                .unwrap()
                .dyn_handle(call.headers, options, &mut *send, recv)
                .await;
            let _ = trailers_tx.send(trailers);
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait which may be implemented by types to handle server-side logic of
/// RPCs (Remote Procedure Calls, often shortened to "call").
#[trait_variant::make(Send)]
pub trait Handle: Send + Sync {
    /// Handles an RPC, accepting the send and receive streams that are used to
    /// interact with the call.  Note that `tx` is not static, so it cannot be
    /// sent to another task, meaning the RPC must end before handle returns.
    async fn handle(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        tx: &mut impl SendStream,
        rx: impl RecvStream + 'static,
    ) -> Trailers;
}

#[async_trait]
trait DynHandle: Send + Sync {
    async fn dyn_handle(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        tx: &mut dyn DynSendStream,
        rx: BoxedRecvStream,
    ) -> Trailers;
}

#[async_trait]
impl<T: Handle> DynHandle for T {
    async fn dyn_handle(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        mut tx: &mut dyn DynSendStream,
        rx: BoxedRecvStream,
    ) -> Trailers {
        self.handle(headers, options, &mut tx, rx).await
    }
}

// TODO: delete this type which is only needed pre-rust v1.92 due to a bug
// handling lifetimes:
//
// error: implementation of `server::RecvStream` is not general enough
//    --> grpc/src/server/mod.rs:108:5
//     |
// 108 |     async fn dyn_handle(
//     |     ^^^^^ implementation of `server::RecvStream` is not general enough
//     |
//     = note: `Box<(dyn server::DynRecvStream + '0)>` must implement `server::RecvStream`, for any lifetime `'0`...
//     = note: ...but `server::RecvStream` is actually implemented for the type `Box<(dyn server::DynRecvStream + 'static)>`
struct BoxedRecvStream(Box<dyn DynRecvStream + 'static>);

// Implement RecvStream for the wrapper instead of the Box directly
impl RecvStream for BoxedRecvStream {
    async fn next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>> {
        self.0.dyn_next(msg).await
    }
}

/// An item in a response stream from the server's view.
///
/// These items are sent to the client via a [`SendStream`], using references to
/// avoid allocations.
pub enum ResponseStreamItem<'a> {
    /// Indicates the headers for the stream.
    Headers(ResponseHeaders),
    /// Indicates a message on the stream.
    Message(&'a dyn SendMessage),
}

/// Represents the sending side of a server stream.  See `ResponseStream`
/// documentation for information about the different types of items and the
/// order in which they must be sent.
#[trait_variant::make(Send)]
pub trait SendStream {
    /// Sends the next item on the stream. Returns `Ok(())` on success, or
    /// `Err(())` on failure. `Err(())` is a terminal state.
    /// Calling this method after an error should be avoided and is unspecified.
    ///
    /// # Cancel safety
    ///
    /// This method is not intended to be cancellation safe.  If the returned
    /// future is not polled to completion, the behavior of any subsequent calls
    /// to the SendStream are undefined and data may be lost.
    async fn send<'a>(
        &mut self,
        item: ResponseStreamItem<'a>,
        options: SendOptions,
    ) -> Result<(), ()>;
}

#[async_trait]
trait DynSendStream: Send {
    async fn dyn_send<'a>(
        &mut self,
        item: ResponseStreamItem<'a>,
        options: SendOptions,
    ) -> Result<(), ()>;
}

#[async_trait]
impl<T: SendStream> DynSendStream for T {
    async fn dyn_send<'a>(
        &mut self,
        item: ResponseStreamItem<'a>,
        options: SendOptions,
    ) -> Result<(), ()> {
        self.send(item, options).await
    }
}

impl<'b> SendStream for &mut (dyn DynSendStream + 'b) {
    async fn send<'a>(
        &mut self,
        item: ResponseStreamItem<'a>,
        options: SendOptions,
    ) -> Result<(), ()> {
        (**self).dyn_send(item, options).await
    }
}

impl<'b> SendStream for Box<dyn DynSendStream + 'b> {
    async fn send<'a>(
        &mut self,
        item: ResponseStreamItem<'a>,
        options: SendOptions,
    ) -> Result<(), ()> {
        (**self).dyn_send(item, options).await
    }
}

/// Contains settings to configure a send operation on a SendStream.
#[derive(Default)]
#[non_exhaustive]
pub struct SendOptions {
    /// Delays sending the message until the trailers are provided on the stream
    /// and batches the two items together if possible.
    pub final_msg: bool,
    /// If set, compression will be disabled for this message.
    pub disable_compression: bool,
}

/// Represents the receiving side of a server stream.
#[trait_variant::make(Send)]
pub trait RecvStream {
    /// Returns the next message on the stream. Returns `Some(Ok(()))` on
    /// success, `None` on normal stream end, or `Some(Err(()))` if the stream
    /// encountered an error before the client's final request message. Both
    /// `None` and `Some(Err(()))` are terminal states.
    /// Calling this method again after reaching a terminal state is unspecified
    /// and should be avoided.
    ///
    /// # Cancel safety
    ///
    /// This method is not intended to be cancellation safe.  If the returned
    /// future is not polled to completion, the behavior of any subsequent calls
    /// to the RecvStream are undefined and data may be lost.
    async fn next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>>;
}

#[async_trait]
trait DynRecvStream: Send {
    async fn dyn_next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>>;
}

#[async_trait]
impl<T: RecvStream> DynRecvStream for T {
    async fn dyn_next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>> {
        self.next(msg).await
    }
}

impl<'a> RecvStream for Box<dyn DynRecvStream + 'a> {
    async fn next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>> {
        (**self).dyn_next(msg).await
    }
}

/// Contains all information transmitted in the response headers of an RPC.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    metadata: MetadataMap,
}

impl ResponseHeaders {
    /// Returns a default ResponseHeaders instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the metadata of self with `metadata`.
    pub fn with_metadata(mut self, metadata: MetadataMap) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns a reference to the metadata in these headers.
    pub fn metadata(&self) -> &MetadataMap {
        &self.metadata
    }

    /// Returns a mutable reference to the metadata in these headers.
    pub fn metadata_mut(&mut self) -> &mut MetadataMap {
        &mut self.metadata
    }

    pub(crate) fn into_metadata(self) -> MetadataMap {
        self.metadata
    }
}

/// Contains all information transmitted in the request headers of an RPC.
#[derive(Debug, Clone, Default)]
pub struct RequestHeaders {
    /// The full (e.g. "/Service/Method") method name specified for the call.
    method_name: String,
    /// The application-specified metadata for the call.
    metadata: MetadataMap,
}

impl RequestHeaders {
    /// Returns a default RequestHeaders instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the method name of self with `method_name`.
    pub fn with_method_name(mut self, method_name: impl Into<String>) -> Self {
        self.method_name = method_name.into();
        self
    }

    /// Replaces the metadata of self with `metadata`.
    pub fn with_metadata(mut self, metadata: MetadataMap) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns the full (e.g. "/Service/Method") method name for these headers.
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Returns a reference to the metadata in these headers.
    pub fn metadata(&self) -> &MetadataMap {
        &self.metadata
    }

    /// Returns a mutable reference to the metadata in these headers.
    pub fn metadata_mut(&mut self) -> &mut MetadataMap {
        &mut self.metadata
    }

    /// Returns the owned fields in the RequestHeaders.
    // TODO: make public once fields are fixed.
    pub(crate) fn into_parts(self) -> (String, MetadataMap) {
        (self.method_name, self.metadata)
    }
}

/// Contains all information transmitted in the response trailers of an RPC.
/// gRPC does not support request trailers.
#[derive(Debug, Clone)]
pub struct Trailers {
    status: crate::Result<()>,
    metadata: MetadataMap,
}

impl Trailers {
    /// Returns a default [`Trailers`] instance.
    pub fn new(status: crate::Result<()>) -> Self {
        Self {
            status,
            metadata: MetadataMap::default(),
        }
    }

    /// Replaces the status of self with `status`.
    pub fn with_status(mut self, status: crate::Result<()>) -> Self {
        self.status = status;
        self
    }

    /// Returns a reference to the status contained in these trailers.
    pub fn status(&self) -> &crate::Result<()> {
        &self.status
    }

    /// Replaces the metadata of self with `metadata`.
    pub fn with_metadata(mut self, metadata: MetadataMap) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns a mutable reference to the metadata in these trailers.
    pub fn metadata_mut(&mut self) -> &mut MetadataMap {
        &mut self.metadata
    }

    /// Returns a reference to the metadata in these trailers.
    pub fn metadata(&self) -> &MetadataMap {
        &self.metadata
    }

    /// Returns the status in the [`Trailers`], consuming the entire status.
    pub fn into_status(self) -> crate::Result<()> {
        self.status
    }

    pub(crate) fn into_parts(self) -> (crate::Result<()>, MetadataMap) {
        (self.status, self.metadata)
    }
}
