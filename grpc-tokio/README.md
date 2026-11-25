# grpc-tokio

[Tokio](https://tokio.rs) runtime support for [`grpc`](https://docs.rs/grpc).

The `grpc` crate is runtime-agnostic. This crate provides a Tokio-backed
runtime to pass to the `grpc` APIs that take one:

```rust,ignore
let rt = grpc_tokio::runtime();
let channel = Channel::builder(target, credentials, rt.clone()).build();
let server = Server::builder(rt.clone()).add_service(service).build();
let listener = Server::tcp_listener(addr, creds, config, rt).await?;
```

> NOTE: This version is a preview and not recommended for any production
> use.  All APIs are unstable.  Proceed at your own risk.

## Feature Flags

- `dns` (default): Use [`hickory-resolver`](https://docs.rs/hickory-resolver)
  for DNS resolution, enabling custom DNS authorities.

## License

This project is licensed under the [MIT license] and/or the [`Apache license
2.0`].

[MIT license]: https://github.com/grpc/grpc-rust/blob/master/LICENSE
[`Apache license 2.0`]: https://www.apache.org/licenses/LICENSE-2.0.txt
