fn main() {
    // docs.rs won't let us download sources, so skip the C++ compile.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // If CI/prebuilt environment tells us to skip the C++ build, do so immediately.
    if let Ok(val) = std::env::var("GRPC_RUST_SKIP_CPP_BUILD")
        && !val.is_empty()
        && val != "0"
    {
        println!(
            "cargo:warning=GRPC_RUST_SKIP_CPP_BUILD is set, skipping C++ protobuf plugin build."
        );
        return;
    }

    // Avoid rebuilding if the C++ source files (and this file) didn't change.
    println!("cargo:rerun-if-changed=src/cpp_source");
    println!("cargo:rerun-if-changed=build.rs");

    let mut cmake_config = cmake::Config::new("src/cpp_source");
    cmake_config.define("BUILD_PROTOC", "ON");
    cmake_config.define("BUILD_PLUGIN", "ON");
    cmake_config.build();
}
