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
use std::path::{Path, PathBuf};

#[allow(clippy::too_many_lines)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Optionally use protoc-gen-rust-grpc's protoc for prost. protoc-gen-rust-grpc will skip its
    // build when PROTOC_GEN_RUST_GRPC_NO_BUILD=1 (used in gRPC's CI), so we check that the binary
    // exists.
    #[cfg(feature = "protoc-gen-rust-grpc")]
    if protoc_gen_rust_grpc::protoc().exists() {
        unsafe {
            env::set_var("PROTOC", protoc_gen_rust_grpc::protoc());
        }
    }

    tonic_prost_build::configure()
        .compile_protos(&["proto/routeguide/route_guide.proto"], &["proto"])
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    tonic_prost_build::compile_protos("proto/echo/echo.proto").unwrap();

    tonic_prost_build::compile_protos("proto/unaryecho/echo.proto").unwrap();

    tonic_prost_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .server_attribute("Echo", "#[derive(PartialEq)]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .client_attribute("Echo", "#[derive(PartialEq)]")
        .compile_protos(&["proto/attrs/attrs.proto"], &["proto"])
        .unwrap();

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
            &["proto/googleapis"],
        )
        .unwrap();

    build_json_codec_service();

    let smallbuff_copy = out_dir.join("smallbuf");
    let _ = std::fs::create_dir(smallbuff_copy.clone()); // This will panic below if the directory failed to create
    tonic_prost_build::configure()
        .out_dir(smallbuff_copy)
        .codec_path("crate::common::SmallBufferCodec")
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-env-changed=GRPC_RUST_REGENERATE_PROTO");
    println!("cargo:rerun-if-env-changed=SKIP_GRPC_RUST_PROTO_CODEGEN");
    let grpc_helloworld = env::var_os("CARGO_FEATURE_GRPC_HELLOWORLD").is_some();
    let grpc_routeguide = env::var_os("CARGO_FEATURE_GRPC_ROUTEGUIDE").is_some();

    if grpc_helloworld || grpc_routeguide {
        let hw_proto = Path::new("proto/helloworld/helloworld.proto");
        let rg_proto = Path::new("proto/routeguide/route_guide.proto");
        let hw_gen = Path::new("generated/helloworld/generated.rs");
        let rg_gen = Path::new("generated/routeguide/generated.rs");

        println!("cargo:rerun-if-changed={}", hw_proto.display());
        println!("cargo:rerun-if-changed={}", rg_proto.display());
        println!("cargo:rerun-if-changed={}", hw_gen.display());
        println!("cargo:rerun-if-changed={}", rg_gen.display());

        let skip_codegen = env::var_os("SKIP_GRPC_RUST_PROTO_CODEGEN").is_some();
        let force_regenerate = env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some();

        if skip_codegen {
            if force_regenerate {
                println!(
                    "cargo:warning=Both SKIP_GRPC_RUST_PROTO_CODEGEN and GRPC_RUST_REGENERATE_PROTO are set. Skipping code generation."
                );
            }
            assert!(
                hw_gen.exists() && rg_gen.exists(),
                "SKIP_GRPC_RUST_PROTO_CODEGEN is set, but generated files are missing in generated/"
            );
        } else {
            let generated_missing = !hw_gen.exists() || !rg_gen.exists();
            let proto_newer = match (
                fs::metadata(hw_proto).and_then(|m| m.modified()),
                fs::metadata(rg_proto).and_then(|m| m.modified()),
                fs::metadata(hw_gen).and_then(|m| m.modified()),
                fs::metadata(rg_gen).and_then(|m| m.modified()),
            ) {
                (Ok(hw_p), Ok(rg_p), Ok(hw_g), Ok(rg_g)) => hw_p > hw_g || rg_p > rg_g,
                _ => true,
            };

            if force_regenerate || generated_missing || proto_newer {
                if has_protoc() {
                    let generated_dir = Path::new("generated");
                    if generated_dir.exists() {
                        let _ = fs::remove_dir_all(generated_dir);
                    }

                    grpc_protobuf_build::CodeGen::new()
                        .output_dir(generated_dir.join("helloworld"))
                        .input("helloworld.proto")
                        .include("proto/helloworld")
                        .client_only()
                        .compile()
                        .unwrap();

                    grpc_protobuf_build::CodeGen::new()
                        .output_dir(generated_dir.join("routeguide"))
                        .input("route_guide.proto")
                        .include("proto/routeguide")
                        .client_only()
                        .compile()
                        .unwrap();
                } else if generated_missing {
                    panic!(
                        "Cannot generate protobuf code: protoc is not available and generated files are missing in generated/"
                    );
                } else {
                    println!(
                        "cargo:warning=protoc not found; skipping proto regeneration and using checked-in files."
                    );
                }
            }
        }
    }

    if env::var_os("CARGO_FEATURE_GRPC_GCP").is_some() {
        let dependencies = protobuf_well_known_types::get_dependency("protobuf_well_known_types")
            .into_iter()
            .map(std::convert::Into::into)
            .collect();

        grpc_protobuf_build::CodeGen::new()
            .include("proto/googleapis")
            .inputs([
                "google/pubsub/v1/pubsub.proto",
                "google/pubsub/v1/schema.proto",
                "google/api/annotations.proto",
                "google/api/resource.proto",
                "google/api/http.proto",
                "google/api/field_behavior.proto",
                "google/api/client.proto",
                "google/protobuf/descriptor.proto", // bundled with protoc.
            ])
            .dependencies(dependencies)
            .client_only()
            .compile()
            .unwrap();
    }
}

fn has_protoc() -> bool {
    #[cfg(feature = "protoc-gen-rust-grpc")]
    if protoc_gen_rust_grpc::protoc().is_file() {
        return true;
    }
    if env::var_os("GRPC_RUST_PROTOC_DIR").is_some_and(|dir| {
        !dir.is_empty()
            && (Path::new(&dir).join("protoc").is_file()
                || Path::new(&dir).join("protoc.exe").is_file())
    }) {
        return true;
    }
    if env::var_os("PROTOC").is_some_and(|p| !p.is_empty() && Path::new(&p).is_file()) {
        return true;
    }
    if let Some(path_var) = env::var_os("PATH") {
        for dir in env::split_paths(&path_var) {
            if dir.join("protoc").is_file() || dir.join("protoc.exe").is_file() {
                return true;
            }
        }
    }
    false
}

// Manually define the json.helloworld.Greeter service which used a custom JsonCodec to use json
// serialization instead of protobuf for sending messages on the wire.
// This will result in generated client and server code which relies on its request, response and
// codec types being defined in a module `crate::common`.
//
// See the client/server examples defined in `src/json-codec` for more information.
fn build_json_codec_service() {
    let greeter_service = tonic_prost_build::manual::Service::builder()
        .name("Greeter")
        .package("json.helloworld")
        .method(
            tonic_prost_build::manual::Method::builder()
                .name("say_hello")
                .route_name("SayHello")
                .input_type("crate::common::HelloRequest")
                .output_type("crate::common::HelloResponse")
                .codec_path("crate::common::JsonCodec")
                .build(),
        )
        .build();

    tonic_prost_build::manual::Builder::new().compile(&[greeter_service]);
}
