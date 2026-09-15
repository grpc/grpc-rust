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
//! Shared in-process harness for the two unary benches.
//!
//! `unary` measures wall-clock time and runs on the system allocator.
//! `unary_allocs` installs a counting global allocator and reports allocations
//! per RPC. The two must not share a process: a counting allocator adds two
//! atomic read-modify-writes to every allocation, which taxes a timed region
//! and makes the timing untrustworthy.

use std::convert::Infallible;
use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BufMut, Bytes};
use http::uri::PathAndQuery;
use tonic::body::Body;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tower_service::Service;

/// 16 bytes sits below the 8 KiB initial codec buffer, 65536 above it. 1024
/// stays below both that buffer and the 32 KiB yield threshold.
pub(crate) const SIZES: [usize; 3] = [16, 1024, 65536];

#[derive(Clone, Copy, Debug)]
pub(crate) struct BytesCodec;

impl Codec for BytesCodec {
    type Encode = Bytes;
    type Decode = Bytes;
    type Encoder = Self;
    type Decoder = Self;

    fn encoder(&mut self) -> Self::Encoder {
        Self
    }

    fn decoder(&mut self) -> Self::Decoder {
        Self
    }
}

impl Encoder for BytesCodec {
    type Item = Bytes;
    type Error = tonic::Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        dst.reserve(item.len());
        dst.put(item);
        Ok(())
    }
}

impl Decoder for BytesCodec {
    type Item = Bytes;
    type Error = tonic::Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(src.copy_to_bytes(src.remaining())))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Echo;

impl Service<tonic::Request<Bytes>> for Echo {
    type Response = tonic::Response<Bytes>;
    type Error = tonic::Status;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: tonic::Request<Bytes>) -> Self::Future {
        ready(Ok(tonic::Response::new(request.into_inner())))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Loopback;

impl Service<http::Request<Body>> for Loopback {
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        Box::pin(async move {
            Ok(tonic::server::Grpc::new(BytesCodec)
                .unary(Echo, request)
                .await)
        })
    }
}

pub(crate) fn payload(size: usize) -> Bytes {
    Bytes::from(vec![0xAB; size])
}

pub(crate) fn client() -> tonic::client::Grpc<Loopback> {
    tonic::client::Grpc::new(Loopback)
}

pub(crate) fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

pub(crate) async fn one_rpc(client: &mut tonic::client::Grpc<Loopback>, payload: &Bytes) -> Bytes {
    client
        .unary(
            tonic::Request::new(payload.clone()),
            PathAndQuery::from_static("/bench.Echo/Unary"),
            BytesCodec,
        )
        .await
        .unwrap()
        .into_inner()
}
