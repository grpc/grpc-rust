use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // docs.rs won't let us download sources, so skip the C++ compile.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // If CI/prebuilt environment tells us to skip the C++ build, do so immediately.
    if let Ok(val) = std::env::var("PROTOC_GEN_RUST_GRPC_NO_BUILD")
        && !val.is_empty()
        && val != "0"
    {
        // If the env var becomes unset, make sure we execute our build.rs again.
        println!("cargo:rerun-if-env-changed=PROTOC_GEN_RUST_GRPC_NO_BUILD");
        println!(
            "cargo:warning=PROTOC_GEN_RUST_GRPC_NO_BUILD is set, skipping C++ protobuf plugin build."
        );
        return;
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR env var is defined"));
    let cpp_source = Path::new("src").join("cpp_source");

    // Avoid rebuilding if the C++ source files (and this file) didn't change.
    println!("cargo:rerun-if-changed={}", cpp_source.to_str().unwrap());

    let install_dir = out_dir.join("install");
    if install_dir.exists() {
        std::fs::remove_dir_all(&install_dir)
            .expect("All files in install/ directory should be deletable");
    }

    let mut cmake_config = cmake::Config::new(&cpp_source);
    cmake_config.define("BUILD_PROTOC", "ON");
    cmake_config.define("BUILD_PLUGIN", "ON");
    cmake_config.define("CMAKE_INSTALL_PREFIX", &install_dir);
    cmake_config.build();
}
