use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-env-changed=GRPC_RUST_REGENERATE_PROTO");
    if env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some() {
        let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let dependencies = protobuf_well_known_types::get_dependency("protobuf_well_known_types")
            .into_iter()
            .map(|d| d.into())
            .collect();

        grpc_protobuf_build::CodeGen::new()
            .output_dir(manifest_dir.join("generated"))
            .include(manifest_dir.join("third_party/googleapis"))
            .inputs(["google/rpc/status.proto"])
            .dependencies(dependencies)
            .client_only()
            .compile()
            .unwrap();
    }
}
