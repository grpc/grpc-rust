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

use std::marker::PhantomData;

use grpc::server::BoxedRecvStream;
use grpc::server::DynRecvStream;
use grpc::server::DynSendStream;
use grpc::server::ResponseStreamItem;
use grpc::server::SendOptions;
use protobuf::AsMut;
use protobuf::AsView;
use protobuf::Message;
use protobuf::MessageMut;
use protobuf::MessageView;

use crate::ProtoRecvMessage;
use crate::ProtoSendMessage;

mod bidi;
mod client_streaming;
mod server_streaming;
mod unary;

pub use bidi::*;
pub use client_streaming::*;
pub use server_streaming::*;
pub use unary::*;

/// Allows receiving streaming RPC protobuf request messages on the server.
pub struct GrpcStreamingRequest<M> {
    rx: BoxedRecvStream,
    _phantom: PhantomData<M>,
}

impl<M> GrpcStreamingRequest<M>
where
    M: Message,
    for<'b> M::Mut<'b>: MessageMut<'b>,
{
    /// Creates a new [`GrpcStreamingRequest`].
    pub(crate) fn new(rx: BoxedRecvStream) -> Self {
        Self {
            rx,
            _phantom: PhantomData,
        }
    }

    /// Receives the next request message from the stream into `req`.
    ///
    /// Returns `Some(Ok(()))` on success, `Some(Err(()))` if the stream
    /// encountered an error, or `None` if the client has closed the stream.
    pub async fn recv_into(
        &mut self,
        req: &mut impl AsMut<MutProxied = M>,
    ) -> Option<Result<(), ()>> {
        let mut res_view = ProtoRecvMessage::from_mut(req);
        self.rx.dyn_next(&mut res_view).await
    }

    /// Receives the next request message from the stream.
    ///
    /// Returns `Some(Ok(msg))` on success, `Some(Err(()))` if the stream
    /// encountered an error, or `None` if the client has closed the stream.
    pub async fn recv(&mut self) -> Option<Result<M, ()>> {
        let mut req = M::default();
        match self.recv_into(&mut req).await {
            Some(Ok(())) => Some(Ok(req)),
            Some(Err(())) => Some(Err(())),
            None => None,
        }
    }
}

/// Allows sending streaming RPC protobuf response messages from the server.
pub struct GrpcStreamingResponse<'a, M> {
    tx: &'a mut dyn DynSendStream,
    _phantom: PhantomData<M>,
}

impl<'a, M> GrpcStreamingResponse<'a, M>
where
    M: Message,
    for<'b> M::View<'b>: MessageView<'b>,
{
    pub(crate) fn new(tx: &'a mut dyn DynSendStream) -> Self {
        Self {
            tx,
            _phantom: PhantomData,
        }
    }

    /// Sends a response message on the stream.
    ///
    /// Will block if flow control does not allow sending the message. Returns
    /// an error if the stream has ended or been cancelled.
    ///
    /// Note: success does *not* indicate successful receipt of the response by
    /// the client; it only indicates that the stream has not yet terminated.
    pub async fn send(&mut self, resp: &impl AsView<Proxied = M>) -> Result<(), ()> {
        self.tx
            .dyn_send(
                ResponseStreamItem::Message(&ProtoSendMessage::from_view(resp)),
                SendOptions::default(),
            )
            .await
    }
}
