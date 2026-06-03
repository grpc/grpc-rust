fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let proto = "proto/grpc/testing/test.proto";

    tonic_prost_build::compile_protos(proto).unwrap();
    grpc_protobuf_build::CodeGen::new()
        .include("proto/grpc/testing")
        .inputs(["test.proto", "empty.proto", "messages.proto"])
        .compile()
        .unwrap();
}
