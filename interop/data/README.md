# Interop Testing Certificates

These files are copies of gRPC's shared interop test credentials, from
[`src/core/tsi/test_creds`](https://github.com/grpc/grpc/tree/master/src/core/tsi/test_creds)
in the grpc/grpc repository. Every gRPC implementation ships the same set, which
lets grpc-rust run TLS interop tests against any other implementation.

| File | Used by |
|---|---|
| `ca.pem` | The interop client trusts this CA when run with `--use_test_ca` (default `--ca_file`). |
| `server1.pem`, `server1.key` | Interop servers present this certificate when run with `--use_tls`. It is valid for `*.test.google.fr`, `waterzooi.test.google.be`, `*.test.youtube.com` and `192.168.1.3`, so clients pass e.g. `--server_host_override=foo.test.google.fr`. |

Do not regenerate these files: a locally generated CA is not trusted by the
other implementations' interop clients and servers. Keep them byte-identical to
the grpc/grpc copies. The current set expires in March 2030; when gRPC rotates
it, copy the new files here.
