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

use grpc::async_trait;
use grpc::server::CallOptions;
use grpc::server::DynHandle;
use grpc::server::DynRecvStream;
use grpc::server::DynSendStream;
use grpc::server::RequestHeaders;
use grpc::server::ResponseStreamItem;
use grpc::server::SendOptions;
use grpc::server::Trailers;
use protobuf::AsMut;
use protobuf::ClearAndParse;
use protobuf::Message;
use protobuf::MutProxied;
use protobuf::Proxied;
use protobuf::Serialize;

use crate::ProtoSendMessage;
use crate::SendFuture;
use crate::ServerStatus;
use crate::server::GrpcStreamingRequest;
use crate::trailers_conv::trailers_from_status;

/// A client-streaming RPC method handler on the server.
///
/// Implementations receive a stream of request messages from the client and
/// populate a single response message.
#[trait_variant::make(Send)]
pub trait ClientStreamingMethod: Sync + 'static {
    /// The protobuf request message type.
    type Request: Message + Default;
    /// The protobuf response message type.
    type Response: Message + Default;

    /// Handles a client-streaming RPC call.
    ///
    /// Receives a stream of incoming `requests` from the client and populates
    /// the `response` message, returning a [`ServerStatus`] to indicate success
    /// or failure.
    async fn call(
        &self,
        requests: GrpcStreamingRequest<Self::Request>,
        response: <Self::Response as MutProxied>::Mut<'_>,
    ) -> ServerStatus;
}

/// An adapter that wraps a [`ClientStreamingMethod`] to handle incoming
/// client-streaming RPCs.
pub struct ClientStreamingAdapter<M: ClientStreamingMethod> {
    method: M,
}

impl<M: ClientStreamingMethod> ClientStreamingAdapter<M> {
    /// Creates a new [`ClientStreamingAdapter`] wrapping the given `method`.
    pub fn new(method: M) -> Self {
        Self { method }
    }
}

#[async_trait]
impl<M> DynHandle for ClientStreamingAdapter<M>
where
    M: ClientStreamingMethod,
    for<'a> <M::Request as MutProxied>::Mut<'a>: ClearAndParse + Send + Sync,
    for<'a> <M::Response as Proxied>::View<'a>: Serialize + Send + Sync,
{
    async fn dyn_handle(
        &self,
        _headers: RequestHeaders,
        _options: CallOptions,
        tx: &mut dyn DynSendStream,
        rx: Box<dyn DynRecvStream>,
    ) -> Trailers {
        let requests = GrpcStreamingRequest::new(rx);
        let mut resp = <M::Response as Default>::default();
        let status = self.method.call(requests, resp.as_mut()).make_send().await;

        if status.is_ok() {
            let send = ProtoSendMessage::from_view(&resp);
            let mut options = SendOptions::default();
            options.final_msg = true;
            let _ = tx
                .dyn_send(ResponseStreamItem::Message(&send), options)
                .await;
        }

        trailers_from_status(status)
    }
}
