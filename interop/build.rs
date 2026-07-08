fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Use protoc-gen-rust-grpc for tonic_prost_build when it is available
    #[cfg(feature = "protoc-gen-rust-grpc")]
    if protoc_gen_rust_grpc::protoc().exists() {
        unsafe {
            std::env::set_var("PROTOC", protoc_gen_rust_grpc::protoc());
        }
    }

    let proto = "proto/grpc/testing/test.proto";

    tonic_prost_build::compile_protos(proto).unwrap();
    grpc_protobuf_build::CodeGen::new()
        .include("proto/grpc/testing")
        .inputs(["test.proto", "empty.proto", "messages.proto"])
        .compile()
        .unwrap();
}
