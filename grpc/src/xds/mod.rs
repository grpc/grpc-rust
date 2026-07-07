//! xDS support for gRPC.
//!
//! Message-only (decode-focused) protobuf types generated from the vendored
//! xDS protos under `proto/xds/third_party`. The generated code lives in the
//! [`generated`] submodule, whose module tree mirrors the proto package
//! directory structure (e.g. `generated::envoy::config::cluster::v3::cluster`).
//!
//! Regenerate on demand with `cargo run -p codegen --features xds`.

// The entire generated tree is machine-written; keep `cargo fmt` out of it.
#[rustfmt::skip]
pub mod generated;
