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

//! Library for compiling and using the [`gRPC-Rust`] plugin for [`protoc`].
//!
//! [`protoc`]: https://protobuf.dev/installation/
//! [`gRPC-Rust`]: https://crates.io/crates/grpc

include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

use std::fs;
use std::path::PathBuf;

const PROTOC_NAME: &str = if cfg!(target_os = "windows") {
    "protoc.exe"
} else {
    "protoc"
};
const PLUGIN_NAME: &str = if cfg!(target_os = "windows") {
    "protoc-gen-rust-grpc.exe"
} else {
    "protoc-gen-rust-grpc"
};

/// The full path to the `protoc` executable.
pub fn protoc() -> Result<PathBuf, String> {
    let path = bin()?.join(PROTOC_NAME);
    Ok(path)
}

/// The full path to the gRPC `protoc` plugin, `protoc-gen-rust-grpc`.
pub fn protoc_gen_rust_grpc() -> Result<PathBuf, String> {
    let path = bin()?.join(PLUGIN_NAME);
    Ok(path)
}

/// The path to the `bin` directory containing the C++ binaries this package
/// builds. It extracts protobuf into a subdirectory of the calling crate's `OUT_DIR`.
pub fn bin() -> Result<PathBuf, String> {
    let out_dir =
        extract_assets_impl().map_err(|e| format!("Failed to write protobuf assets: {}", e))?;
    Ok(out_dir.join("bin"))
}

fn extract_assets_impl() -> Result<PathBuf, String> {
    let out = std::env::var("OUT_DIR").map_err(|e| {
        format!("Could not get OUT_DIR environment variable. protoc-gen-rust-grpc may only be called from within a build script: {}", e)
    })?;
    let target_dir =
        PathBuf::from(out).join(format!("grpc-rust-bin-{}", env!("CARGO_PKG_VERSION")));

    if EMBEDDED_FILES.is_empty() {
        return Err("No embedded assets are available (build was skipped).".to_string());
    }

    for (rel_path, bytes) in EMBEDDED_FILES {
        let target_path = target_dir.join(rel_path);

        if target_path.exists() {
            let metadata = target_path.metadata().map_err(|e| {
                format!(
                    "Failed to read metadata for existing file {}: {}",
                    target_path.display(),
                    e
                )
            })?;
            if metadata.len() == bytes.len() as u64 {
                // The file is still present from a previous execution.
                continue;
            }
        }

        let parent = target_path
            .parent()
            .expect("The file has a parent directory");
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directory {}: {}",
                parent.display(),
                e
            )
        })?;

        let temp_path = parent.join(format!(
            ".tmp-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::write(&temp_path, bytes).map_err(|e| {
            format!(
                "Failed to write embedded asset {} to temporary file: {}",
                target_path.display(),
                e
            )
        })?;

        // Assume all files in bin/ should be executable
        #[cfg(unix)]
        if rel_path.starts_with("bin/") {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o755)).map_err(|e| {
                format!(
                    "Failed to set executable permissions on {}: {}",
                    temp_path.display(),
                    e
                )
            })?;
        }

        if let Err(e) = fs::rename(&temp_path, &target_path) {
            let _ = fs::remove_file(&temp_path);
            if !target_path.exists() {
                return Err(format!(
                    "Failed to rename temporary file {} to target path {}: {}",
                    temp_path.display(),
                    target_path.display(),
                    e
                ));
            }
        }
    }

    Ok(target_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_embedded_files() {
        if EMBEDDED_FILES.is_empty() {
            // Build was skipped via PROTOC_GEN_RUST_GRPC_NO_BUILD=1 or DOCS_RS
            return;
        }

        assert!(
            EMBEDDED_FILES
                .iter()
                .any(|(p, _)| *p == format!("bin/{}", PROTOC_NAME)),
            "protoc binary missing from embedded files"
        );
        assert!(
            EMBEDDED_FILES
                .iter()
                .any(|(p, _)| *p == format!("bin/{}", PLUGIN_NAME)),
            "protoc-gen-rust-grpc binary missing from embedded files"
        );
        assert!(
            EMBEDDED_FILES
                .iter()
                .any(|(p, _)| *p == "include/google/protobuf/empty.proto"),
            "empty.proto missing from embedded files"
        );
    }
}
