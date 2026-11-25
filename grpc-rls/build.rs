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

//! Build-time RLS codegen.
//!
//! Generates the protobuf messages and the `RouteLookupService` client stub for
//! the vendored `grpc.lookup.v1` protos under `third_party/grpc-proto/` into
//! `OUT_DIR`. See `src/lib.rs` for how the generated code is pulled in.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dependencies = protobuf_well_known_types::get_dependency("protobuf_well_known_types")
        .into_iter()
        .map(Into::into)
        .collect();

    // Protos vendored from grpc/grpc-proto and kept in sync by
    // `tools/sync-grpc-proto.sh`. RLS only acts as a client of the lookup
    // service, so no server code is generated.
    grpc_protobuf_build::CodeGen::new()
        .include("third_party/grpc-proto")
        .inputs([
            "grpc/lookup/v1/rls.proto",
            "grpc/lookup/v1/rls_config.proto",
        ])
        .dependencies(dependencies)
        .client_only()
        .compile()
        .unwrap();
}
