//! xDS support for gRPC.
//!
//! Message-only (decode-focused) protobuf types generated from the vendored
//! xDS protos under `proto/xds/third_party`. The generated code lives in the
//! crate-internal `generated` submodule, whose module tree mirrors the proto
//! package directory structure (e.g.
//! `generated::envoy::config::cluster::v3::cluster`).
//!
//! The generated messages are an implementation detail: they implement the
//! `protobuf` runtime traits, so the tree is kept `pub(crate)` rather than
//! leaking those types (and protobuf internals) into grpc's public API.
//!
//! Regenerate on demand with `cargo run -p codegen --features xds`.

// The entire generated tree is machine-written; keep `cargo fmt` out of it.
#[rustfmt::skip]
pub(crate) mod generated;
