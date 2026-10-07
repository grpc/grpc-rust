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

//! Wall-clock cost of an in-process unary round trip.
//!
//! This target runs on the system allocator so nothing instruments the timed
//! region. Allocations per RPC are measured by the separate `unary_allocs`
//! target, which installs a counting allocator in its own process.

mod unary_support;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use unary_support::{SIZES, client, one_rpc, payload, runtime};

fn bench_unary(c: &mut Criterion) {
    let mut group = c.benchmark_group("unary");
    for size in SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        let payload = payload(size);
        let mut client = client();
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
