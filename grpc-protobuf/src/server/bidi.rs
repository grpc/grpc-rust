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
use protobuf::ClearAndParse;
use protobuf::Message;
use protobuf::MutProxied;
use protobuf::Proxied;
use protobuf::Serialize;

use crate::ServerStatus;
use crate::server::GrpcStreamingRequest;
use crate::server::GrpcStreamingResponse;
use crate::trailers_conv::trailers_from_status;

/// A bidirectional-streaming RPC method handler on the server.
///
/// Implementations receive a stream of request messages and send a stream of
/// response messages to the client.
#[trait_variant::make(Send)]
pub trait BidiStreamingMethod: Sync + 'static {
    /// The protobuf request message type.
    type Request: Message + Default;
    /// The protobuf response message type.
    type Response: Message + Default;

    /// Handles a bidirectional-streaming RPC call.
    ///
    /// Receives incoming `requests` from the client and uses `responses` to
    /// stream response messages back to the client, returning a [`ServerStatus`]
    /// when the handler has completed.
    async fn call(
        &self,
        requests: GrpcStreamingRequest<Self::Request>,
        responses: GrpcStreamingResponse<'_, Self::Response>,
    ) -> ServerStatus;
}

/// An adapter that wraps a [`BidiStreamingMethod`] to handle incoming
/// bidirectional-streaming RPCs.
pub struct BidiStreamingAdapter<M: BidiStreamingMethod> {
    method: M,
}

impl<M: BidiStreamingMethod> BidiStreamingAdapter<M> {
    /// Creates a new [`BidiStreamingAdapter`] wrapping the given `method`.
    pub fn new(method: M) -> Self {
        Self { method }
    }
}

#[async_trait]
impl<M> DynHandle for BidiStreamingAdapter<M>
where
    M: BidiStreamingMethod,
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
        // The request stream owns `rx`; the response sink borrows `tx`. They
        // are independent, so a handler can freely interleave receives and
        // sends.
        let requests = GrpcStreamingRequest::new(rx);
        let responses = GrpcStreamingResponse::new(tx);
        let status = self.method.call(requests, responses).await;
        trailers_from_status(status)
    }
}
