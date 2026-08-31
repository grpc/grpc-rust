# grpc-bssl

BoringSSL TLS transport credentials provider for [`grpc`](../grpc).

This crate provides `BsslChannelCredentials` and `BsslServerCredentials`, which integrate BoringSSL (`bssl-tls` and `bssl-tls-tokio`) transport security with the `grpc` crate.
