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
use grpc::server::Trailers;
use protobuf::AsView;
use protobuf::Message;
use protobuf::Proxied;

use crate::ProtoRecvMessage;
use crate::SendFuture;
use crate::ServerStatus;
use crate::ServerStatusError;
use crate::StatusCodeError;
use crate::server::GrpcStreamingResponse;
use crate::trailers_conv::trailers_from_status;

/// A server-streaming RPC method handler on the server.
///
/// Implementations receive a single request message and send a stream of
/// response messages to the client.
#[trait_variant::make(Send)]
pub trait ServerStreamingMethod: Sync + 'static {
    /// The protobuf request message type.
    type Request: Message;
    /// The protobuf response message type.
    type Response: Message;

    /// Handles a server-streaming RPC call.
    ///
    /// Receives a view of the incoming `request` message and uses `responses`
    /// to stream response messages back to the client, returning a
    /// [`ServerStatus`] when the handler has completed.
    async fn call(
        &self,
        request: <Self::Request as Proxied>::View<'_>,
        responses: GrpcStreamingResponse<'_, Self::Response>,
    ) -> ServerStatus;
}

/// An adapter that wraps a [`ServerStreamingMethod`] to handle incoming
/// server-streaming RPCs.
pub struct ServerStreamingAdapter<M: ServerStreamingMethod> {
    method: M,
}

impl<M: ServerStreamingMethod> ServerStreamingAdapter<M> {
    /// Creates a new [`ServerStreamingAdapter`] wrapping the given `method`.
    pub fn new(method: M) -> Self {
        Self { method }
    }
}

#[async_trait]
impl<M> DynHandle for ServerStreamingAdapter<M>
where
    M: ServerStreamingMethod,
{
    async fn dyn_handle(
        &self,
        _headers: RequestHeaders,
        _options: CallOptions,
        tx: &mut dyn DynSendStream,
        mut rx: Box<dyn DynRecvStream>,
    ) -> Trailers {
        let mut req = <M::Request as Default>::default();

        if rx
            .dyn_next(&mut ProtoRecvMessage::from_mut(&mut req))
            .await
            .is_none_or(|res| res.is_err())
        {
            return trailers_from_status(Err(ServerStatusError::new(
                StatusCodeError::Internal,
                "client did not send a request message",
            )));
        }

        let responses = GrpcStreamingResponse::new(tx);
        let status = self.method.call(req.as_view(), responses).make_send().await;
        trailers_from_status(status)
    }
}
