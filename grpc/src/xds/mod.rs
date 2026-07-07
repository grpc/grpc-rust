//! xDS support.
//!
//! Message-only (decode-focused) protobuf types generated from the vendored
//! xDS protos under `proto/xds/third_party`. The generated code lives in the
//! [`generated`] submodule, whose module tree mirrors the proto package
//! directory structure (e.g. `generated::envoy::config::cluster::v3::cluster`).
//!
//! Regenerate on demand with `cargo run -p codegen --features xds`.

pub mod generated;
