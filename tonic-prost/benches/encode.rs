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

//! ProstEncoder change: `buf.reserve(encoded_len)` before `item.encode`.
//!
//! EncodeBuf forwards to BytesMut, so encoding into BytesMut is the same
//! growth path. Each size is run with and without the reserve.

#![allow(missing_docs)]

use bencher::{Bencher, benchmark_group, benchmark_main};
use bytes::BytesMut;
use prost::Message;
use std::hint::black_box;

#[derive(Clone, PartialEq, Message)]
struct BytesMsg {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct UnpackedVarints {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    values: Vec<i64>,
}

fn encode_bytes(b: &mut Bencher, size: usize, reserve: bool) {
    let msg = BytesMsg {
        data: vec![0xAB; size],
    };
    b.bytes = size as u64;
    b.iter(|| {
        let mut buf = BytesMut::new();
        if reserve {
            buf.reserve(msg.encoded_len());
            msg.encode_raw(&mut buf);
        } else {
            msg.encode(&mut buf).unwrap();
        }
        black_box(buf);
    });
}

fn encode_bytes_after_grpc_header(b: &mut Bencher, size: usize, reserve: bool) {
    let msg = BytesMsg {
        data: vec![0xAB; size],
    };
    b.bytes = size as u64;
    b.iter(|| {
        // tonic's encode path: 8KiB codec buffer, then 5-byte gRPC frame header
        let mut buf = BytesMut::with_capacity(8 * 1024);
        buf.resize(5, 0);
        if reserve {
            buf.reserve(msg.encoded_len());
            msg.encode_raw(&mut buf);
        } else {
            msg.encode(&mut buf).unwrap();
        }
        black_box(buf);
    });
}

fn encode_varints(b: &mut Bencher, n: usize, reserve: bool) {
    let msg = UnpackedVarints {
        values: (0..n as i64).collect(),
    };
    b.iter(|| {
        let mut buf = BytesMut::new();
        if reserve {
            buf.reserve(msg.encoded_len());
            msg.encode_raw(&mut buf);
        } else {
            msg.encode(&mut buf).unwrap();
        }
        black_box(buf);
    });
}

fn bytes_64_no_reserve(b: &mut Bencher) {
    encode_bytes(b, 64, false);
}
fn bytes_64_reserve(b: &mut Bencher) {
    encode_bytes(b, 64, true);
}
fn bytes_8k_no_reserve(b: &mut Bencher) {
    encode_bytes(b, 8 * 1024, false);
}
fn bytes_8k_reserve(b: &mut Bencher) {
    encode_bytes(b, 8 * 1024, true);
}
fn bytes_1m_no_reserve(b: &mut Bencher) {
    encode_bytes(b, 1024 * 1024, false);
}
fn bytes_1m_reserve(b: &mut Bencher) {
    encode_bytes(b, 1024 * 1024, true);
}
fn bytes_1m_header_no_reserve(b: &mut Bencher) {
    encode_bytes_after_grpc_header(b, 1024 * 1024, false);
}
fn bytes_1m_header_reserve(b: &mut Bencher) {
    encode_bytes_after_grpc_header(b, 1024 * 1024, true);
}
fn varints_16k_no_reserve(b: &mut Bencher) {
    encode_varints(b, 16 * 1024, false);
}
fn varints_16k_reserve(b: &mut Bencher) {
    encode_varints(b, 16 * 1024, true);
}

benchmark_group!(
    encode,
    bytes_64_no_reserve,
    bytes_64_reserve,
    bytes_8k_no_reserve,
    bytes_8k_reserve,
    bytes_1m_no_reserve,
    bytes_1m_reserve,
    bytes_1m_header_no_reserve,
    bytes_1m_header_reserve,
    varints_16k_no_reserve,
    varints_16k_reserve,
);
benchmark_main!(encode);
