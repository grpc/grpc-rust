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
#![expect(missing_docs, reason = "benchmark crate has no public API")]

use std::alloc::{GlobalAlloc, Layout, System};
use std::convert::Infallible;
use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use bytes::{Buf, BufMut, Bytes};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use http::uri::PathAndQuery;
use tonic::body::Body;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tower_service::Service;

const SIZES: [usize; 3] = [16, 1024, 65536];

static COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

struct Counting;

#[global_allocator]
static ALLOCATOR: Counting = Counting;

// Keep allocation counts separate from deallocation counts so each RPC delta is
// independent of when its response is dropped.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(new_size, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[derive(Clone, Copy, Debug)]
struct BytesCodec;

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
struct Echo;

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

struct Loopback;

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

async fn one_rpc(client: &mut tonic::client::Grpc<Loopback>, payload: &Bytes) -> Bytes {
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

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn report_allocations() {
    for size in SIZES {
        let payload = Bytes::from(vec![0xAB; size]);
        let mut client = tonic::client::Grpc::new(Loopback);
        let runtime = runtime();

        for _ in 0..200 {
            let _ = runtime.block_on(one_rpc(&mut client, &payload));
        }

        let count = COUNT.load(Ordering::Relaxed);
        let bytes = BYTES.load(Ordering::Relaxed);

        for _ in 0..1000 {
            let _ = runtime.block_on(one_rpc(&mut client, &payload));
        }

        let d_count = COUNT.load(Ordering::Relaxed) - count;
        let d_bytes = BYTES.load(Ordering::Relaxed) - bytes;
        eprintln!(
            "unary/{size}: {} allocs/rpc, {} bytes/rpc",
            d_count / 1000,
            d_bytes / 1000
        );
    }
}

fn bench_unary(c: &mut Criterion) {
    report_allocations();

    let mut group = c.benchmark_group("unary");
    for size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        let payload = Bytes::from(vec![0xAB; size]);
        let mut client = tonic::client::Grpc::new(Loopback);
        let runtime = runtime();
        let response = runtime.block_on(one_rpc(&mut client, &payload));
        assert_eq!(response, payload);

        group.bench_with_input(size.to_string(), &payload, |b, payload| {
            b.iter(|| runtime.block_on(one_rpc(&mut client, payload)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_unary);
criterion_main!(benches);
