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
//! Generates the protobuf message modules for the vendored `grpc.lookup.v1`
//! protos under `third_party/grpc-proto/` into `OUT_DIR`. See `src/lib.rs` for
//! how the generated root module is pulled in.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gen_dir = out_dir.join("generated");

    // Protos vendored from grpc/grpc-proto and kept in sync by
    // `tools/sync-grpc-proto.sh`. The root doubles as the protoc include path.
    let include_dir = manifest.join("third_party/grpc-proto");
    println!("cargo:rerun-if-changed={}", include_dir.display());

    // Resolve protoc. Locally, `protoc-gen-grpc-rust` builds it and we use that;
    // in CI its C++ build is skipped (`PROTOC_GEN_GRPC_RUST_NO_BUILD=1`) and a
    // prebuilt protoc is provided on `$PATH`, so `find_protoc` falls back to it.
    let protoc = find_protoc();

    // Include-relative import paths of the protos to compile. Both live in the
    // same package directory, so protoc places `generated.rs` at a predictable
    // location.
    let protos = [
        "grpc/lookup/v1/rls.proto",
        "grpc/lookup/v1/rls_config.proto",
    ];

    // Fresh output tree.
    if gen_dir.exists() {
        std::fs::remove_dir_all(&gen_dir).unwrap();
    }
    std::fs::create_dir_all(&gen_dir).unwrap();

    protobuf_codegen::CodeGen::new()
        .protoc_path(&protoc)
        .inputs(protos.iter())
        .include(&include_dir)
        .output_dir(&gen_dir)
        .dependency(protobuf_well_known_types::get_dependency(
            "protobuf_well_known_types",
        ))
        .generate_and_compile()
        .unwrap_or_else(|e| panic!("RLS codegen failed: {e}"));
}

/// Resolves the `protoc` binary. Prefers the one built by `protoc-gen-grpc-rust`
/// (local dev); when that build is skipped (`PROTOC_GEN_GRPC_RUST_NO_BUILD`, as
/// in CI), falls back to `$PROTOC` or the first `protoc` on `$PATH`. Returns the
/// full path so the sibling `../include` dir can be located.
fn find_protoc() -> PathBuf {
    const PROTOC_NAME: &str = "protoc";
    // Prefer to use the protoc built by `protoc-gen-grpc-rust`.
    let built = protoc_gen_grpc_rust::protoc();
    if built.is_file() {
        return built;
    }
    // Fall back to `$PROTOC` if set.
    if let Some(p) = std::env::var_os("PROTOC").map(PathBuf::from)
        && p.is_file()
    {
        return p;
    }
    // Fall back to $PATH resolution.
    PROTOC_NAME.into()
}
