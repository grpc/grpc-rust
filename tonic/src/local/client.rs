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

//! Generic client implementation for `!Send` local mode.
//!
//! NB: keep in sync with src/client/grpc.rs
//!
//! `Grpc<T>`'s methods below deliberately parallel `crate::client::Grpc`'s: the
//! bodies are copies with the trait bounds (dropping `Send`), body type (local
//! [`Body`]) and streaming type (local [`Streaming`]) changed. That bound
//! difference is not incidental duplication - it IS the feature this module
//! exists to provide, so it cannot be shared with the core. What *is* shared:
//! `GrpcConfig`, `prepare_request`, and `classify_response` (all in
//! `crate::client::grpc`).

use std::{fmt, future, pin::pin};

use http::uri::{PathAndQuery, Uri};
use http_body::Body as HttpBody;
use tokio_stream::{Stream, StreamExt};

use crate::client::classify_response;
use crate::codec::{Codec, CompressionEncoding, Decoder, EncodeBody};
use crate::local::body::Body;
use crate::local::codec::Streaming;
use crate::{Request, Response, Status, client::GrpcService};

/// A gRPC client dispatcher for `!Send` local mode.
///
/// Mirrors [`crate::client::Grpc`]: it wraps some inner [`GrpcService`] and
/// encodes/decodes messages via the provided codec, but drops the `Send`
/// bounds on futures, streams, bodies, and messages.
pub struct Grpc<T> {
    inner: T,
    config: crate::client::GrpcConfig,
}

impl<T> Grpc<T> {
    /// Creates a new gRPC client with the provided [`GrpcService`].
    pub fn new(inner: T) -> Self {
        Self::with_origin(inner, Uri::default())
    }

    /// Creates a new gRPC client with the provided [`GrpcService`] and `Uri`.
    ///
    /// The provided Uri will use only the scheme and authority parts as the
    /// path_and_query portion will be set for each method.
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        Self {
            inner,
            config: crate::client::GrpcConfig::new(origin),
        }
    }

    /// Compress requests with the provided encoding.
    ///
    /// Requires the server to accept the specified encoding, otherwise it might return an error.
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.config.send_compression_encodings = Some(encoding);
        self
    }

    /// Enable accepting compressed responses.
    ///
    /// Requires the server to also support sending compressed responses.
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.config.accept_compression_encodings.enable(encoding);
        self
    }

    /// Limits the maximum size of a decoded message.
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.config.max_decoding_message_size = Some(limit);
        self
    }

    /// Limits the maximum size of an encoded message.
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.config.max_encoding_message_size = Some(limit);
        self
    }

    /// Check if the inner [`GrpcService`] is able to accept a new request.
    ///
    /// This will call [`GrpcService::poll_ready`] until it returns ready or
    /// an error. If this returns ready the inner [`GrpcService`] is ready to
    /// accept one more request.
    pub async fn ready(&mut self) -> Result<(), T::Error>
    where
        T: GrpcService<Body>,
    {
        future::poll_fn(|cx| self.inner.poll_ready(cx)).await
    }

    /// Send a single unary gRPC request.
    pub async fn unary<M1, M2, C>(
        &mut self,
        request: Request<M1>,
        path: PathAndQuery,
        codec: C,
    ) -> Result<Response<M2>, Status>
    where
        T: GrpcService<Body>,
        T::ResponseBody: HttpBody + 'static,
        <T::ResponseBody as HttpBody>::Error: Into<crate::BoxError>,
        C: Codec<Encode = M1, Decode = M2>,
        M1: 'static,
        M2: 'static,
    {
        let request = request.map(|m| tokio_stream::once(m));
        self.client_streaming(request, path, codec).await
    }

    /// Send a client side streaming gRPC request.
    pub async fn client_streaming<S, M1, M2, C>(
        &mut self,
        request: Request<S>,
        path: PathAndQuery,
        codec: C,
    ) -> Result<Response<M2>, Status>
    where
        T: GrpcService<Body>,
        T::ResponseBody: HttpBody + 'static,
        <T::ResponseBody as HttpBody>::Error: Into<crate::BoxError>,
        S: Stream<Item = M1> + 'static,
        C: Codec<Encode = M1, Decode = M2>,
        M1: 'static,
        M2: 'static,
    {
        let (mut parts, body, extensions) =
            self.streaming(request, path, codec).await?.into_parts();

        let mut body = pin!(body);

        let message = body
            .try_next()
            .await
            .map_err(|mut status| {
                status.metadata_mut().merge(parts.clone());
                status
            })?
            .ok_or_else(|| Status::internal("Missing response message."))?;

        if let Some(trailers) = body.trailers().await? {
            parts.merge(trailers);
        }

        Ok(Response::from_parts(parts, message, extensions))
    }

    /// Send a server side streaming gRPC request.
    pub async fn server_streaming<M1, M2, C>(
        &mut self,
        request: Request<M1>,
        path: PathAndQuery,
        codec: C,
    ) -> Result<Response<Streaming<M2>>, Status>
    where
        T: GrpcService<Body>,
        T::ResponseBody: HttpBody + 'static,
        <T::ResponseBody as HttpBody>::Error: Into<crate::BoxError>,
        C: Codec<Encode = M1, Decode = M2>,
        M1: 'static,
        M2: 'static,
    {
        let request = request.map(|m| tokio_stream::once(m));
        self.streaming(request, path, codec).await
    }

    /// Send a bi-directional streaming gRPC request.
    pub async fn streaming<S, M1, M2, C>(
        &mut self,
        request: Request<S>,
        path: PathAndQuery,
        mut codec: C,
    ) -> Result<Response<Streaming<M2>>, Status>
    where
        T: GrpcService<Body>,
        T::ResponseBody: HttpBody + 'static,
        <T::ResponseBody as HttpBody>::Error: Into<crate::BoxError>,
        S: Stream<Item = M1> + 'static,
        C: Codec<Encode = M1, Decode = M2>,
        M1: 'static,
        M2: 'static,
    {
        let request = request
            .map(|s| {
                EncodeBody::new_client(
                    codec.encoder(),
                    s.map(Ok),
                    self.config.send_compression_encodings,
                    self.config.max_encoding_message_size,
                )
            })
            .map(Body::new);

        let request = self.config.prepare_request(request, path);

        let response = self
            .inner
            .call(request)
            .await
            .map_err(Status::from_error_generic)?;

        let decoder = codec.decoder();

        self.create_response(decoder, response)
    }

    // Keeping this code in a separate function from Self::streaming lets functions that return the
    // same output share the generated binary code
    fn create_response<M2>(
        &self,
        decoder: impl Decoder<Item = M2, Error = Status> + 'static,
        response: http::Response<T::ResponseBody>,
    ) -> Result<Response<Streaming<M2>>, Status>
    where
        T: GrpcService<Body>,
        T::ResponseBody: HttpBody + 'static,
        <T::ResponseBody as HttpBody>::Error: Into<crate::BoxError>,
    {
        let (encoding, expect_additional_trailers) =
            classify_response(response.headers(), self.config.accept_compression_encodings)?;

        let status_code = response.status();

        let response = response.map(|body| {
            if expect_additional_trailers {
                Streaming::new_response(
                    decoder,
                    body,
                    status_code,
                    encoding,
                    self.config.max_decoding_message_size,
                )
            } else {
                Streaming::new_empty(decoder, body)
            }
        });

        Ok(Response::from_http(response))
    }
}

impl<T: Clone> Clone for Grpc<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            config: self.config.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Grpc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Grpc");
        d.field("inner", &self.inner);
        self.config.debug_fields(&mut d).finish()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::convert::Infallible;
    use std::rc::Rc;
    use std::task::{Context, Poll};

    use bytes::{Buf, BufMut};
    use http_body_util::BodyExt;

    use super::*;
    use crate::codec::{
        BufferSettings, DecodeBuf, EncodeBuf, Encoder, SingleMessageCompressionOverride,
    };

    /// A trivial [`Codec`] over `Vec<u8>` messages, mirroring the plain-bytes
    /// mock codecs used in `tonic::codec`'s own tests.
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

        fn buffer_settings(&self) -> BufferSettings {
            BufferSettings::default()
        }
    }

    impl Decoder for VecCodec {
        type Item = Vec<u8>;
        type Error = Status;

        fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
            let len = src.remaining();
            Ok(Some(src.copy_to_bytes(len).to_vec()))
        }

        fn buffer_settings(&self) -> BufferSettings {
            BufferSettings::default()
        }
    }

    /// A non-`Send` mock [`GrpcService`] holding an `Rc<RefCell<..>>`,
    /// capturing the outgoing request and replaying a canned response.
    struct MockService {
        captured: Rc<RefCell<Option<http::Request<Body>>>>,
        response: Rc<RefCell<Option<http::Response<Body>>>>,
    }

    impl tower_service::Service<http::Request<Body>> for MockService {
        type Response = http::Response<Body>;
        type Error = Infallible;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<Body>) -> Self::Future {
            *self.captured.borrow_mut() = Some(req);
            let response = self
                .response
                .borrow_mut()
                .take()
                .expect("mock response already consumed");
            future::ready(Ok(response))
        }
    }

    /// Build a plain 200 gRPC response body carrying a single message; the
    /// `grpc-status` trailer is emitted by `EncodeBody` itself.
    fn canned_response(message: Vec<u8>) -> http::Response<Body> {
        let body = EncodeBody::new_server(
            VecCodec,
            tokio_stream::once(Ok::<_, Status>(message)),
            None,
            SingleMessageCompressionOverride::default(),
            None,
        );

        http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Body::new(body))
            .unwrap()
    }

    #[tokio::test]
    async fn unary_roundtrip_non_send_service() {
        let captured = Rc::new(RefCell::new(None));
        let response_msg = b"pong".to_vec();
        let service = MockService {
            captured: captured.clone(),
            response: Rc::new(RefCell::new(Some(canned_response(response_msg.clone())))),
        };

        let mut client = Grpc::new(service);
        let path: PathAndQuery = "/test.Service/Method".parse().unwrap();

        let response = client
            .unary(Request::new(b"ping".to_vec()), path, VecCodec)
            .await
            .unwrap();
        assert_eq!(response.into_inner(), response_msg);

        let req = captured.borrow_mut().take().expect("request captured");
        assert_eq!(
            req.headers().get(http::header::CONTENT_TYPE),
            Some(&crate::metadata::GRPC_CONTENT_TYPE)
        );
        assert_eq!(req.uri().path(), "/test.Service/Method");
    }

    #[tokio::test]
    async fn streaming_request_body_is_framed() {
        let captured = Rc::new(RefCell::new(None));
        let service = MockService {
            captured: captured.clone(),
            response: Rc::new(RefCell::new(Some(canned_response(b"ack".to_vec())))),
        };

        let mut client = Grpc::new(service);
        let path: PathAndQuery = "/test.Service/Stream".parse().unwrap();
        let request = Request::new(tokio_stream::iter(vec![b"hi".to_vec()]));

        client
            .client_streaming(request, path, VecCodec)
            .await
            .unwrap();

        let req = captured.borrow_mut().take().expect("request captured");
        let bytes = req.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(bytes[0], 0u8);
        let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap());
        assert_eq!(len as usize, b"hi".len());
    }
}
