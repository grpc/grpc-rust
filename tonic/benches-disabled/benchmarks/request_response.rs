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

use criterion::*;

use crate::benchmarks::compiled_protos::helloworld::{HelloReply, HelloRequest};
use crate::benchmarks::utils;

fn build_request(_name: String) {
    let _request = tonic::Request::new(HelloRequest { name: _name });
}

fn build_response(_message: String) {
    let _response = tonic::Request::new(HelloReply { message: _message });
}

pub fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("Request_Response");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let tiny_string = utils::generate_rnd_string(100).unwrap();
    let short_string = utils::generate_rnd_string(1_000).unwrap();
    let medium_string = utils::generate_rnd_string(10_000).unwrap();
    let big_string = utils::generate_rnd_string(100_000).unwrap();
    let huge_string = utils::generate_rnd_string(1_000_000).unwrap();
    let massive_string = utils::generate_rnd_string(10_000_000).unwrap();

    for size in [
        tiny_string,
        short_string,
        medium_string,
        big_string,
        huge_string,
        massive_string,
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(size.len() as u64));

        group.bench_with_input(BenchmarkId::new("request", size.len()), size, |b, i| {
            b.iter(|| build_request(i.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("response", size.len()), size, |b, i| {
            b.iter(|| build_response(i.to_string()))
        });
    }
    group.finish();
}
