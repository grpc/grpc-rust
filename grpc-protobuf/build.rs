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

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let proto_path = Path::new("third_party/googleapis/google/rpc/status.proto");
    let generated_dir = Path::new("generated");
    let gen_marker = Path::new("generated/google/rpc/generated.rs");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", proto_path.display());
    println!("cargo:rerun-if-changed={}", gen_marker.display());
    println!("cargo:rerun-if-env-changed=GRPC_RUST_REGENERATE_PROTO");

    let force_regenerate = env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some();
    let generated_missing = !generated_dir.exists() || !gen_marker.exists();
    let proto_newer = match (fs::metadata(proto_path), fs::metadata(gen_marker)) {
        (Ok(p_meta), Ok(g_meta)) => match (p_meta.modified(), g_meta.modified()) {
            (Ok(p_time), Ok(g_time)) => p_time > g_time,
            _ => true,
        },
        _ => true,
    };

    if force_regenerate || generated_missing || proto_newer {
        if generated_dir.exists() {
            let _ = fs::remove_dir_all(generated_dir);
        }

        let dependencies = protobuf_well_known_types::get_dependency("protobuf_well_known_types")
            .into_iter()
            .map(std::convert::Into::into)
            .collect();

        grpc_protobuf_build::CodeGen::new()
            .output_dir(generated_dir)
            .include("third_party/googleapis")
            .inputs(["google/rpc/status.proto"])
            .dependencies(dependencies)
            .client_only()
            .compile()
            .unwrap();
    }
}
