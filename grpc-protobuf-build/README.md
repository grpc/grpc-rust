# grpc-protobuf-build

Compiles proto files via protobuf rust and generates service stubs and proto
definitions for use with tonic.

> NOTE: This version is a preview and not recommended for any production
> use.  All APIs are unstable.  Proceed at your own risk.

## Features

Required dependencies

```toml
[dependencies]
protobuf = "<protobuf-version>"

[build-dependencies]
grpc-protobuf-build = "<grpc-version>"
```

## Getting Started

Please see [our website] for everything you should need to get started using
gRPC!

[our website]: https://grpc.io/docs/languages/rust

## Detailed Crate Usage

`grpc-protobuf-build` works by being included as a [`build.rs`
file](https://doc.rust-lang.org/cargo/reference/build-scripts.html) at the root
of the binary/library.

You can rely on the defaults via

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc_protobuf_build::CodeGen::new()
        .include("proto")
        .inputs(["service.proto"])
        .compile()?;
    Ok(())
}
```

Or configure the generated code deeper via

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dependency = grpc_protobuf_build::Dependency::builder()
        .crate_name("external_protos".to_string())
        .proto_import_paths(vec![PathBuf::from("external/message.proto")])
        .proto_files(vec!["message.proto".to_string()])
        .build()?;

    grpc_protobuf_build::CodeGen::new()
        .generate_message_code(false)
        .inputs(["proto/helloworld/helloworld.proto"])
        .include("external")
        .message_module_path("super::proto")
        .dependencies(vec![dependency])
        //.output_dir("src/generated")  // you can change the generated code's location
        .compile()?;
   Ok(())
}
```

Then you can reference the generated Rust like this in your code:
```rust,ignore
mod protos {
    // Include message code.
    include!(concat!(env!("OUT_DIR"), "proto/helloworld/generated.rs"));
}

mod grpc {
    // Include service code.
    include!(concat!(env!("OUT_DIR"), "proto/helloworld/helloworld_grpc.pb.rs"));
}
```

If you don't modify the `message_module_path`, you can use the `include_proto`
macro to simplify the import code.
```rust,ignore
pub mod grpc_pb {
    grpc::include_proto!("proto/helloworld", "helloworld");
}
```

Or if you want to save the generated code in your own code base,
you can uncomment the line `.output_dir(...)` above, and in your lib file
config a mod like this:
```rust,ignore
pub mod generated {
    pub mod helloworld {
        pub mod proto {
            include!("helloworld/generated.rs");
        }

        pub mod grpc {
            include!("helloworld/test_grpc.pb.rs");
        }
    }
}
```
