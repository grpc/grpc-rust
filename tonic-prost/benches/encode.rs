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

//! Encode-path microbenchmarks for `reserve(encoded_len)` + `encode_raw`
//! versus `Message::encode`.
//!
//! Dimensions:
//! - shape: empty, bytes, `bytes::Bytes`, string, packed/unpacked varints,
//!   nested chunk write, mixed unary RPC, repeated nested
//! - size: empty through 1 MiB, including tonic's 8 KiB codec buffer and
//!   32 KiB yield threshold
//! - start buffer: empty, tonic unary (8 KiB + 5-byte header), reused
//!   streaming buffer, almost-full 8 KiB buffer
//! - streaming: many messages into one buffer, split at the yield threshold

#![allow(missing_docs)]

use std::hint::black_box;
use std::time::Duration;

use bytes::{BufMut, Bytes, BytesMut};
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use prost::Message;

const CODEC_BUF: usize = 8 * 1024;
const HEADER: usize = 5;
const YIELD: usize = 32 * 1024;

const BLOB_SIZES: &[usize] = &[0, 64, 1024, CODEC_BUF, YIELD, 256 * 1024, 1024 * 1024];
const VARINT_COUNTS: &[usize] = &[0, 16, 256, 2 * 1024, 16 * 1024];
const ITEM_COUNTS: &[usize] = &[1, 16, 256, 2 * 1024];

#[derive(Clone, PartialEq, Message)]
struct EmptyMsg {}

#[derive(Clone, PartialEq, Message)]
struct BytesMsg {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct BytesBufMsg {
    #[prost(bytes = "bytes", tag = "1")]
    data: Bytes,
}

#[derive(Clone, PartialEq, Message)]
struct StringMsg {
    #[prost(string, tag = "1")]
    data: String,
}

#[derive(Clone, PartialEq, Message)]
struct PackedVarints {
    #[prost(int64, repeated, packed = "true", tag = "1")]
    values: Vec<i64>,
}

#[derive(Clone, PartialEq, Message)]
struct UnpackedVarints {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    values: Vec<i64>,
}

#[derive(Clone, PartialEq, Message)]
struct Chunk {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct WriteRequest {
    #[prost(string, tag = "1")]
    stream: String,
    #[prost(uint64, tag = "2")]
    offset: u64,
    #[prost(message, tag = "3")]
    chunk: Option<Chunk>,
}

#[derive(Clone, PartialEq, Message)]
struct MixedRpc {
    #[prost(string, tag = "1")]
    method: String,
    #[prost(uint64, tag = "2")]
    request_id: u64,
    #[prost(string, tag = "3")]
    resource: String,
    #[prost(bytes = "vec", tag = "4")]
    payload: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct Kv {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(int64, tag = "2")]
    value: i64,
}

#[derive(Clone, PartialEq, Message)]
struct Batch {
    #[prost(message, repeated, tag = "1")]
    entries: Vec<Kv>,
}

#[derive(Clone, Copy)]
enum Start {
    Empty,
    TonicHeader,
    Reused,
    AlmostFull,
}

impl Start {
    fn label(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TonicHeader => "tonic_header",
            Self::Reused => "reused_16kib",
            Self::AlmostFull => "almost_full_8kib",
        }
    }

    fn buf(self) -> BytesMut {
        match self {
            Self::Empty => BytesMut::new(),
            Self::TonicHeader => {
                let mut buf = BytesMut::with_capacity(CODEC_BUF);
                buf.resize(HEADER, 0);
                buf
            }
            Self::Reused => {
                let mut buf = BytesMut::with_capacity(CODEC_BUF);
                buf.resize(16 * 1024, 0xCD);
                buf
            }
            Self::AlmostFull => {
                let mut buf = BytesMut::with_capacity(CODEC_BUF);
                buf.resize(CODEC_BUF - 16, 0xCD);
                buf
            }
        }
    }
}

fn encode(msg: &impl Message, buf: &mut BytesMut, reserve: bool) {
    if reserve {
        buf.reserve(msg.encoded_len());
        msg.encode_raw(buf);
    } else {
        msg.encode(buf).unwrap();
    }
}

fn pair<M: Message>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    param: &str,
    msg: &M,
    start: Start,
) {
    for (name, reserve) in [("no_reserve", false), ("reserve", true)] {
        group.bench_function(BenchmarkId::new(name, param), |b| {
            b.iter(|| {
                let mut buf = start.buf();
                encode(msg, &mut buf, reserve);
                black_box(buf);
            });
        });
    }
}

fn size_label(n: usize) -> String {
    if n == 0 {
        "0".to_string()
    } else if n < 1024 {
        format!("{n}B")
    } else if n < 1024 * 1024 {
        format!("{}KiB", n / 1024)
    } else {
        format!("{}MiB", n / (1024 * 1024))
    }
}

fn bench_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("empty");
    let msg = EmptyMsg {};
    pair(&mut group, Start::Empty.label(), &msg, Start::Empty);
    pair(
        &mut group,
        Start::TonicHeader.label(),
        &msg,
        Start::TonicHeader,
    );
    group.finish();
}

fn bench_bytes_tonic(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/tonic_header");
    for &size in BLOB_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        let msg = BytesMsg {
            data: vec![0xAB; size],
        };
        pair(&mut group, &size_label(size), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_bytes_empty_start(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/empty_start");
    for &size in &[64usize, CODEC_BUF, 1024 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));
        let msg = BytesMsg {
            data: vec![0xAB; size],
        };
        pair(&mut group, &size_label(size), &msg, Start::Empty);
    }
    group.finish();
}

fn bench_buffer_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_state/8KiB_bytes");
    let msg = BytesMsg {
        data: vec![0xAB; CODEC_BUF],
    };
    for start in [
        Start::Empty,
        Start::TonicHeader,
        Start::Reused,
        Start::AlmostFull,
    ] {
        pair(&mut group, start.label(), &msg, start);
    }
    group.finish();
}

fn bench_bytes_buf(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_buf/tonic_header");
    let size = 1024 * 1024;
    group.throughput(Throughput::Bytes(size as u64));
    let msg = BytesBufMsg {
        data: Bytes::from(vec![0xAB; size]),
    };
    pair(&mut group, "1MiB", &msg, Start::TonicHeader);
    group.finish();
}

fn bench_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("string/tonic_header");
    for &size in &[64usize, 1024, CODEC_BUF, 256 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));
        let msg = StringMsg {
            data: "a".repeat(size),
        };
        pair(&mut group, &size_label(size), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_packed(c: &mut Criterion) {
    let mut group = c.benchmark_group("packed_varints/tonic_header");
    for &n in VARINT_COUNTS {
        let msg = PackedVarints {
            values: (0..n as i64).collect(),
        };
        pair(&mut group, &n.to_string(), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_unpacked(c: &mut Criterion) {
    let mut group = c.benchmark_group("unpacked_varints/tonic_header");
    for &n in VARINT_COUNTS {
        let msg = UnpackedVarints {
            values: (0..n as i64).collect(),
        };
        pair(&mut group, &n.to_string(), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_nested_chunk(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_chunk_write/tonic_header");
    for &size in &[64usize, CODEC_BUF, 256 * 1024, 1024 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));
        let msg = WriteRequest {
            stream: "segment/chunk-1".into(),
            offset: 1 << 20,
            chunk: Some(Chunk {
                data: vec![0xAB; size],
            }),
        };
        pair(&mut group, &size_label(size), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_mixed_rpc(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_rpc/tonic_header");
    for &size in &[0usize, 64, CODEC_BUF, 256 * 1024] {
        let msg = MixedRpc {
            method: "foo.Bar/DoThing".into(),
            request_id: 42,
            resource: "resources/foo/12345".into(),
            payload: vec![0xAB; size],
        };
        pair(&mut group, &size_label(size), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn bench_repeated_nested(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeated_nested/tonic_header");
    for &n in ITEM_COUNTS {
        let msg = Batch {
            entries: (0..n)
                .map(|i| Kv {
                    key: format!("k{i:04}"),
                    value: i as i64,
                })
                .collect(),
        };
        pair(&mut group, &n.to_string(), &msg, Start::TonicHeader);
    }
    group.finish();
}

fn encode_stream<M: Message>(msgs: &[M], reserve: bool) {
    let mut buf = BytesMut::with_capacity(CODEC_BUF);
    for msg in msgs {
        buf.reserve(HEADER);
        unsafe {
            buf.advance_mut(HEADER);
        }
        encode(msg, &mut buf, reserve);
        if buf.len() >= YIELD {
            black_box(buf.split_to(buf.len()));
        }
    }
    if !buf.is_empty() {
        black_box(buf);
    }
}

fn bench_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_batch");
    for &(count, size) in &[(128usize, 256usize), (64, 1024), (16, CODEC_BUF)] {
        let msgs: Vec<BytesMsg> = (0..count)
            .map(|_| BytesMsg {
                data: vec![0xAB; size],
            })
            .collect();
        let param = format!("{}x{}", count, size_label(size));
        group.throughput(Throughput::Bytes((count * size) as u64));
        for (name, reserve) in [("no_reserve", false), ("reserve", true)] {
            group.bench_with_input(BenchmarkId::new(name, &param), &msgs, |b, msgs| {
                b.iter(|| encode_stream(msgs, reserve));
            });
        }
    }
    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_secs(1))
        .sample_size(50)
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets =
        bench_empty,
        bench_bytes_tonic,
        bench_bytes_empty_start,
        bench_buffer_state,
        bench_bytes_buf,
        bench_string,
        bench_packed,
        bench_unpacked,
        bench_nested_chunk,
        bench_mixed_rpc,
        bench_repeated_nested,
        bench_streaming
}
criterion_main!(benches);
