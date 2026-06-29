# gRPC Benchmarking Framework Code

This directory contains the worker, server, and client implementations for the
[gRPC benchmarking framework](https://grpc.io/docs/guides/benchmarking/). The
driver code, along with instructions to run the benchmarks, resides in the
[grpc/grpc repository](https://github.com/grpc/grpc/blob/master/tools/run_tests/performance/README.md).
These benchmarks continuously monitor gRPC performance, providing metrics
through the
[performance dashboard](https://grafana-dot-grpc-testing.appspot.com/).

## Sources

*   [`protos`](./proto/): Copied from the
    [grpc-protos](https://github.com/grpc/grpc-proto/tree/master/grpc/testing)
    repository.
*   [`data/tls`](./data/tls/): Copied from the
    [grpc core](https://github.com/grpc/grpc/tree/3e1d845d437633917b7451bae9d226836658c0bb/src/core/tsi/test_creds)
    repository. See its
    [README](https://github.com/grpc/grpc/blob/3e1d845d437633917b7451bae9d226836658c0bb/src/core/tsi/test_creds/README)
    for more details.
