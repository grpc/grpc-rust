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

#[allow(unused)]
pub mod generated {
    pub mod grpc {
        pub mod testing {
            include!(concat!(env!("OUT_DIR"), "/grpc/testing/generated.rs"));
            include!(concat!(
                env!("OUT_DIR"),
                "/grpc/testing/benchmark_service_grpc.pb.rs"
            ));
            include!(concat!(
                env!("OUT_DIR"),
                "/grpc/testing/worker_service_grpc.pb.rs"
            ));
        }
    }

    pub mod services {
        pub mod grpc {
            pub mod core {
                include!(concat!(env!("OUT_DIR"), "/tonic/grpc.core.rs"));
            }
            pub mod testing {
                include!(concat!(env!("OUT_DIR"), "/tonic/grpc.testing.rs"));
            }
        }
    }
}

mod client;
mod rusage;
mod server;
pub mod worker;
