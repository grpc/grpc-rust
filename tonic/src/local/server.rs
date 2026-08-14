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

//! gRPC server support for `!Send` local mode.
//!
//! This mirrors [`crate::server`], with every `Send` bound dropped from
//! futures, bodies, services, streams, and messages. The header-setting,
//! compression-settings, and compression-override logic is genuinely shared
//! with `crate::server::grpc` (not duplicated); what stays deliberately
//! parallel below is exactly the bounds on [`Grpc`]'s methods and the two
//! service traits, since the `Send`/`!Send` split can't be expressed as one
//! shared generic in stable Rust (a boxed trait object's `Send`-ness is part
//! of its concrete type).

use std::{fmt, future::Future, pin::pin};

use http_body::Body as HttpBody;
use tokio_stream::{Stream, StreamExt};
use tower_service::Service;

use crate::codec::compression::{
    CompressionEncoding, EnabledCompressionEncodings, SingleMessageCompressionOverride,
};
use crate::codec::{Codec, EncodeBody};
use crate::local::body::Body;
use crate::local::codec::Streaming;
use crate::server::{
    ServerGrpcConfig, ServerStreamingService, UnaryService, compression_override_from_response,
    set_grpc_response_headers, t,
};
use crate::{Request, Response, Status};

/// A specialization of [`tower_service::Service`], without a `Send` bound.
///
/// Mirror of [`crate::server::ClientStreamingService`] over a
/// [`crate::local::codec::Streaming`] request.
pub trait ClientStreamingService<R> {
    /// Protobuf response message type
    type Response;

    /// Response future
    type Future: Future<Output = Result<Response<Self::Response>, Status>>;

    /// Call the service
    fn call(&mut self, request: Request<Streaming<R>>) -> Self::Future;
}

impl<T, M1, M2> ClientStreamingService<M1> for T
where
    T: Service<Request<Streaming<M1>>, Response = Response<M2>, Error = Status>,
{
    type Response = M2;
    type Future = T::Future;

    fn call(&mut self, request: Request<Streaming<M1>>) -> Self::Future {
        Service::call(self, request)
    }
}

/// A specialization of [`tower_service::Service`], without a `Send` bound.
///
/// Mirror of [`crate::server::StreamingService`] over a
/// [`crate::local::codec::Streaming`] request.
pub trait StreamingService<R> {
    /// Protobuf response message type
    type Response;

    /// Stream of outbound response messages
    type ResponseStream: Stream<Item = Result<Self::Response, Status>>;

    /// Response future
    type Future: Future<Output = Result<Response<Self::ResponseStream>, Status>>;

    /// Call the service
    fn call(&mut self, request: Request<Streaming<R>>) -> Self::Future;
}

impl<T, S, M1, M2> StreamingService<M1> for T
where
    T: Service<Request<Streaming<M1>>, Response = Response<S>, Error = Status>,
    S: Stream<Item = Result<M2, Status>>,
{
    type Response = M2;
    type ResponseStream = S;
    type Future = T::Future;

    fn call(&mut self, request: Request<Streaming<M1>>) -> Self::Future {
        Service::call(self, request)
    }
}

/// A gRPC server handler for `!Send` local mode.
///
/// See [`crate::server::Grpc`], which this mirrors without `Send` bounds on
/// futures, bodies, services, streams, or messages.
pub struct Grpc<T> {
    codec: T,
    config: ServerGrpcConfig,
}

impl<T> Grpc<T>
where
    T: Codec,
{
    /// Creates a new gRPC server with the provided [`Codec`].
    pub fn new(codec: T) -> Self {
        Self {
            codec,
            config: ServerGrpcConfig::default(),
        }
    }

    /// Enable accepting compressed requests.
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.config.accept_compression_encodings.enable(encoding);
        self
    }

    /// Enable sending compressed responses.
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.config.send_compression_encodings.enable(encoding);
        self
    }

    /// Limits the maximum size of a decoded message.
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.config.max_decoding_message_size = Some(limit);
        self
    }

    /// Limits the maximum size of a encoded message.
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.config.max_encoding_message_size = Some(limit);
        self
    }

    #[doc(hidden)]
    pub fn apply_compression_config(
        mut self,
        accept_encodings: EnabledCompressionEncodings,
        send_encodings: EnabledCompressionEncodings,
    ) -> Self {
        self.config
            .apply_compression_config(accept_encodings, send_encodings);
        self
    }

    #[doc(hidden)]
    pub fn apply_max_message_size_config(
        mut self,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    ) -> Self {
        self.config
            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
        self
    }

    /// Handle a single unary gRPC request.
    ///
    /// Bounds deliberately parallel [`crate::server::Grpc::unary`] minus `Send`
    /// (uses the shared `tonic::server::UnaryService`, which never had a `Send`
    /// bound of its own).
    pub async fn unary<S, B>(
        &mut self,
        mut service: S,
        req: http::Request<B>,
    ) -> http::Response<Body>
    where
        S: UnaryService<T::Decode, Response = T::Encode>,
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
    {
        let accept_encoding = CompressionEncoding::from_accept_encoding_header(
            req.headers(),
            self.config.send_compression_encodings,
        );

        let request = match self.map_request_unary(req).await {
            Ok(r) => r,
            Err(status) => {
                return self.map_response::<tokio_stream::Once<Result<T::Encode, Status>>>(
                    Err(status),
                    accept_encoding,
                    SingleMessageCompressionOverride::default(),
                    self.config.max_encoding_message_size,
                );
            }
        };

        let response = service
            .call(request)
            .await
            .map(|r| r.map(|m| tokio_stream::once(Ok(m))));

        let compression_override = compression_override_from_response(&response);

        self.map_response(
            response,
            accept_encoding,
            compression_override,
            self.config.max_encoding_message_size,
        )
    }

    /// Handle a server side streaming request.
    ///
    /// Bounds deliberately parallel [`crate::server::Grpc::server_streaming`]
    /// minus `Send` (uses the shared `tonic::server::ServerStreamingService`).
    pub async fn server_streaming<S, B>(
        &mut self,
        mut service: S,
        req: http::Request<B>,
    ) -> http::Response<Body>
    where
        S: ServerStreamingService<T::Decode, Response = T::Encode>,
        S::ResponseStream: 'static,
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
    {
        let accept_encoding = CompressionEncoding::from_accept_encoding_header(
            req.headers(),
            self.config.send_compression_encodings,
        );

        let request = match self.map_request_unary(req).await {
            Ok(r) => r,
            Err(status) => {
                return self.map_response::<S::ResponseStream>(
                    Err(status),
                    accept_encoding,
                    SingleMessageCompressionOverride::default(),
                    self.config.max_encoding_message_size,
                );
            }
        };

        let response = service.call(request).await;

        self.map_response(
            response,
            accept_encoding,
            // disabling compression of individual stream items must be done on
            // the items themselves
            SingleMessageCompressionOverride::default(),
            self.config.max_encoding_message_size,
        )
    }

    /// Handle a client side streaming gRPC request.
    ///
    /// Takes this module's own [`ClientStreamingService`] — the `!Send` mirror
    /// of [`crate::server::ClientStreamingService`] — since that trait's `call`
    /// is defined over the (Send) core `Streaming`, not this module's.
    pub async fn client_streaming<S, B>(
        &mut self,
        mut service: S,
        req: http::Request<B>,
    ) -> http::Response<Body>
    where
        S: ClientStreamingService<T::Decode, Response = T::Encode>,
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError> + 'static,
    {
        let accept_encoding = CompressionEncoding::from_accept_encoding_header(
            req.headers(),
            self.config.send_compression_encodings,
        );

        let request = t!(self.map_request_streaming(req));

        let response = service
            .call(request)
            .await
            .map(|r| r.map(|m| tokio_stream::once(Ok(m))));

        let compression_override = compression_override_from_response(&response);

        self.map_response(
            response,
            accept_encoding,
            compression_override,
            self.config.max_encoding_message_size,
        )
    }

    /// Handle a bi-directional streaming gRPC request.
    ///
    /// Takes this module's own [`StreamingService`], for the same reason as
    /// [`Grpc::client_streaming`] above.
    pub async fn streaming<S, B>(
        &mut self,
        mut service: S,
        req: http::Request<B>,
    ) -> http::Response<Body>
    where
        S: StreamingService<T::Decode, Response = T::Encode>,
        S::ResponseStream: 'static,
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
    {
        let accept_encoding = CompressionEncoding::from_accept_encoding_header(
            req.headers(),
            self.config.send_compression_encodings,
        );

        let request = t!(self.map_request_streaming(req));

        let response = service.call(request).await;

        self.map_response(
            response,
            accept_encoding,
            SingleMessageCompressionOverride::default(),
            self.config.max_encoding_message_size,
        )
    }

    async fn map_request_unary<B>(
        &mut self,
        request: http::Request<B>,
    ) -> Result<Request<T::Decode>, Status>
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
    {
        let request_compression_encoding = self.config.request_encoding_if_supported(&request)?;

        let (parts, body) = request.into_parts();

        let mut stream = pin!(Streaming::new_request(
            self.codec.decoder(),
            body,
            request_compression_encoding,
            self.config.max_decoding_message_size,
        ));

        let message = stream
            .try_next()
            .await?
            .ok_or_else(|| Status::internal("Missing request message."))?;

        let mut req = Request::from_http_parts(parts, message);

        if let Some(trailers) = stream.trailers().await? {
            req.metadata_mut().merge(trailers);
        }

        Ok(req)
    }

    fn map_request_streaming<B>(
        &mut self,
        request: http::Request<B>,
    ) -> Result<Request<Streaming<T::Decode>>, Status>
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
    {
        let encoding = self.config.request_encoding_if_supported(&request)?;

        let request = request.map(|body| {
            Streaming::new_request(
                self.codec.decoder(),
                body,
                encoding,
                self.config.max_decoding_message_size,
            )
        });

        Ok(Request::from_http(request))
    }

    fn map_response<B>(
        &mut self,
        response: Result<Response<B>, Status>,
        accept_encoding: Option<CompressionEncoding>,
        compression_override: SingleMessageCompressionOverride,
        max_message_size: Option<usize>,
    ) -> http::Response<Body>
    where
        B: Stream<Item = Result<T::Encode, Status>> + 'static,
    {
        let response = t!(response);

        let (mut parts, body) = response.into_http().into_parts();

        set_grpc_response_headers(&mut parts, accept_encoding);

        let body = EncodeBody::new_server(
            self.codec.encoder(),
            body,
            accept_encoding,
            compression_override,
            max_message_size,
        );

        http::Response::from_parts(parts, Body::new(body))
    }
}

impl<T: fmt::Debug> fmt::Debug for Grpc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grpc")
            .field("codec", &self.codec)
            .field(
                "accept_compression_encodings",
                &self.config.accept_compression_encodings,
            )
            .field(
                "send_compression_encodings",
                &self.config.send_compression_encodings,
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use bytes::{Buf, BufMut, Bytes, BytesMut};

    use super::*;
    use crate::codec::{DecodeBuf, Decoder, EncodeBuf, Encoder};

    /// A trivial `Codec` over `Vec<u8>`, used only to drive `Grpc<T>` in tests.
    #[derive(Clone, Default)]
    struct VecCodec;

    impl Codec for VecCodec {
        type Encode = Vec<u8>;
        type Decode = Vec<u8>;
        type Encoder = VecCodec;
        type Decoder = VecCodec;

        fn encoder(&mut self) -> Self::Encoder {
            VecCodec
        }

        fn decoder(&mut self) -> Self::Decoder {
            VecCodec
        }
    }

    impl Encoder for VecCodec {
        type Item = Vec<u8>;
        type Error = Status;

        fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            dst.put_slice(&item);
            Ok(())
        }
    }

    impl Decoder for VecCodec {
        type Item = Vec<u8>;
        type Error = Status;

        fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
            let len = buf.remaining();
            Ok(Some(buf.copy_to_bytes(len).to_vec()))
        }
    }

    /// A single gRPC-framed message: 1-byte compression flag + 4-byte big-endian
    /// length + payload.
    fn framed(payload: &[u8]) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u8(0);
        buf.put_u32(payload.len() as u32);
        buf.put_slice(payload);
        buf.freeze()
    }

    fn framed_request(payload: &[u8]) -> http::Request<Body> {
        http::Request::builder()
            .header(http::header::CONTENT_TYPE, "application/grpc")
            .body(Body::new(http_body_util::Full::new(framed(payload))))
            .unwrap()
    }

    /// A non-`Send` `UnaryService` counting calls via `Rc<RefCell<u64>>`.
    struct CountingUnary {
        calls: Rc<RefCell<u64>>,
    }

    impl UnaryService<Vec<u8>> for CountingUnary {
        type Response = Vec<u8>;
        type Future = std::future::Ready<Result<Response<Vec<u8>>, Status>>;

        fn call(&mut self, request: Request<Vec<u8>>) -> Self::Future {
            *self.calls.borrow_mut() += 1;
            std::future::ready(Ok(Response::new(request.into_inner())))
        }
    }

    #[tokio::test]
    async fn unary_non_send_handler() {
        let calls = Rc::new(RefCell::new(0u64));
        let mut grpc = Grpc::new(VecCodec);

        let response = grpc
            .unary(
                CountingUnary {
                    calls: calls.clone(),
                },
                framed_request(b"hello"),
            )
            .await;

        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/grpc"
        );

        let mut stream = Streaming::<Vec<u8>>::new_response(
            VecCodec,
            response.into_body(),
            http::StatusCode::OK,
            None,
            None,
        );
        assert_eq!(stream.message().await.unwrap(), Some(b"hello".to_vec()));

        let response = grpc
            .unary(
                CountingUnary {
                    calls: calls.clone(),
                },
                framed_request(b"world"),
            )
            .await;
        let mut stream = Streaming::<Vec<u8>>::new_response(
            VecCodec,
            response.into_body(),
            http::StatusCode::OK,
            None,
            None,
        );
        assert_eq!(stream.message().await.unwrap(), Some(b"world".to_vec()));

        assert_eq!(*calls.borrow(), 2);
    }

    /// A non-`Send` `StreamingService` whose response stream captures an `Rc`
    /// inside its `map` closure (so the stream itself is `!Send`).
    struct EchoStreaming {
        tag: Rc<RefCell<u32>>,
    }

    impl StreamingService<Vec<u8>> for EchoStreaming {
        type Response = Vec<u8>;
        type ResponseStream = std::pin::Pin<Box<dyn Stream<Item = Result<Vec<u8>, Status>>>>;
        type Future = std::future::Ready<Result<Response<Self::ResponseStream>, Status>>;

        fn call(&mut self, _request: Request<Streaming<Vec<u8>>>) -> Self::Future {
            let tag = self.tag.clone();
            let stream =
                tokio_stream::iter(vec![b"one".to_vec(), b"two".to_vec()]).map(move |item| {
                    *tag.borrow_mut() += 1;
                    Ok(item)
                });
            std::future::ready(Ok(Response::new(Box::pin(stream))))
        }
    }

    #[tokio::test]
    async fn streaming_non_send_stream() {
        let tag = Rc::new(RefCell::new(0u32));
        let mut grpc = Grpc::new(VecCodec);

        let response = grpc
            .streaming(
                EchoStreaming { tag: tag.clone() },
                http::Request::new(Body::empty()),
            )
            .await;

        assert_eq!(response.status(), http::StatusCode::OK);

        let mut stream = Streaming::<Vec<u8>>::new_response(
            VecCodec,
            response.into_body(),
            http::StatusCode::OK,
            None,
            None,
        );
        assert_eq!(stream.message().await.unwrap(), Some(b"one".to_vec()));
        assert_eq!(stream.message().await.unwrap(), Some(b"two".to_vec()));
        assert_eq!(stream.message().await.unwrap(), None);
        assert_eq!(*tag.borrow(), 2);
    }

    #[tokio::test]
    async fn bad_request_maps_to_status() {
        let mut grpc = Grpc::new(VecCodec);
        let calls = Rc::new(RefCell::new(0u64));

        // Header claims an invalid compression flag (only 0/1 are valid), which is
        // rejected as soon as the 5-byte frame header is read.
        let garbage =
            http::Request::new(Body::new(http_body_util::Full::new(Bytes::from_static(&[
                2, 0, 0, 0, 0,
            ]))));

        let response = grpc.unary(CountingUnary { calls }, garbage).await;

        let status = response
            .headers()
            .get("grpc-status")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        assert_ne!(status, "0");
    }
}
