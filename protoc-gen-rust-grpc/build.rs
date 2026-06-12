use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR env var is defined"));

    let install_dir = out_dir.join("install");
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir)
            .expect("All files in install/ directory should be deletable");
    }

    populate_install_dir(&install_dir);

    generate_code(&out_dir.join("embedded_assets.rs"), &install_dir);
}

/// Compiles if appropriate, leaving the install directory untouched otherwise.
fn populate_install_dir(install_dir: &Path) {
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

    // Avoid rebuilding if the C++ source files (and this file) didn't change.
    println!("cargo:rerun-if-changed=src/cpp_source");

    let mut cmake_config = cmake::Config::new("src/cpp_source");
    cmake_config.define("BUILD_PROTOC", "ON");
    cmake_config.define("BUILD_PLUGIN", "ON");
    cmake_config.define("CMAKE_INSTALL_PREFIX", &install_dir);
    // There may be many copies of the files, so we don't want large debug information. Debug
    // information increases the binary size by 20x.
    cmake_config.profile("MinSizeRel");
    cmake_config.build();
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> Result<(), String> {
    if dir.is_dir() {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
        for entry in entries {
            let entry =
                entry.map_err(|e| format!("Failed reading entry in {}: {}", dir.display(), e))?;
            let ft = entry.file_type().map_err(|e| {
                format!(
                    "Failed getting file type for entry {}: {}",
                    entry.path().display(),
                    e
                )
            })?;
            let path = entry.path();
            if ft.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}

fn to_literal(s: &str) -> String {
    assert!(
        !s.contains("\"##"),
        "Strings containing \"## are not supported"
    );
    format!("r##\"{}\"##", s)
}

fn generate_code(dest_file: &Path, install_dir: &Path) {
    let mut f = fs::File::create(&dest_file).expect("Create generated code file");

    let mut files_to_embed = Vec::new();
    visit_dirs(&install_dir, &mut |path| {
        files_to_embed.push(path.to_owned());
    })
    .expect("Install directory is fully traversable");
    files_to_embed.sort();

    writeln!(f, "static EMBEDDED_FILES: &[(&str, &[u8])] = &[")
        .expect("Generated code write should succeed");
    for path in files_to_embed {
        let rel_path = path
            .strip_prefix(&install_dir)
            .expect("File path starts with install dir");
        // lib.rs expects forward slashes.
        let rel_path_str = rel_path
            .to_str()
            .expect("File name can be in a str")
            .replace("\\", "/");
        let path_str = path.to_str().expect("File name can be in a str");
        writeln!(
            f,
            "    ({}, include_bytes!({})),",
            to_literal(&rel_path_str),
            to_literal(&path_str)
        )
        .expect("Generated code write should succeed");
    }
    writeln!(f, "];").expect("Generated code write should succeed");
}
