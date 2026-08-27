# gRPC Benchmarking Framework Code

This directory contains the worker, server, and client implementations for the
[gRPC benchmarking framework](https://grpc.io/docs/guides/benchmarking/). The
driver code, along with instructions to run the benchmarks, resides in the
[grpc/grpc repository](https://github.com/grpc/grpc/blob/master/tools/run_tests/performance/README.md).
These benchmarks continuously monitor gRPC performance, providing metrics
through the
[performance dashboard](https://grafana-dot-grpc-testing.appspot.com/).
