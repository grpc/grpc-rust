# tonic-prost-build

Prost build integration for [tonic] gRPC framework.

## Overview

This crate provides code generation for gRPC services using protobuf definitions via the [prost] ecosystem. It bridges [prost-build] with [tonic]'s generic code generation infrastructure.

## Usage

Add to your `build.rs`:

```rust
fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/service.proto"], &["proto"])
        .unwrap();
}
```

## Well-known types

By default, generated code refers to Protobuf well-known types, such as `google.protobuf.Timestamp`, through the [prost-types] crate, so the crate that includes the generated code must depend on it:

```toml
[dependencies]
prost-types = "0.14"
```

To generate these types from their `.proto` definitions instead, call `.compile_well_known_types(true)` on the builder.

## Features

- `transport`: Enables transport layer code generation
- `cleanup-markdown`: Enables markdown cleanup in generated documentation

[tonic]: https://github.com/grpc/grpc-rust
[prost]: https://github.com/tokio-rs/prost
[prost-build]: https://github.com/tokio-rs/prost
[prost-types]: https://docs.rs/prost-types
