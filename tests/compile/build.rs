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

fn main() {
    tonic_prost_build::compile_protos("proto/result.proto").unwrap();
    tonic_prost_build::compile_protos("proto/service.proto").unwrap();
    tonic_prost_build::compile_protos("proto/stream.proto").unwrap();
    tonic_prost_build::compile_protos("proto/same_name.proto").unwrap();
    tonic_prost_build::compile_protos("proto/ambiguous_methods.proto").unwrap();
    tonic_prost_build::compile_protos("proto/includer.proto").unwrap();
    tonic_prost_build::configure()
        .extern_path(".root_crate_path.Animal", "crate::Animal")
        .compile_protos(&["proto/root_crate_path.proto"], &["."])
        .unwrap();
    tonic_prost_build::configure()
        .skip_debug(["skip_debug.Test"])
        .skip_debug(["skip_debug.Output"])
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/skip_debug.proto"], &["proto"])
        .unwrap();
    tonic_prost_build::configure()
        .use_arc_self(true)
        .compile_protos(&["proto/use_arc_self.proto"], &["proto"])
        .unwrap();
}
