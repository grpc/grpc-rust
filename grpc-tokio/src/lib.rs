/*
 *
 * Copyright 2026 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

//! [Tokio](https://tokio.rs) runtime support for [gRPC-Rust](grpc).
//!
//! The `grpc` crate is runtime-agnostic: channels and servers perform all I/O,
//! timers, and task spawning through a pluggable runtime. This crate provides
//! that runtime for Tokio.
//!
//! Use [`runtime()`] to get a Tokio-backed gRPC runtime, and pass it to the
//! `grpc` APIs that take one:
//!
//! ```no_run
//! use std::sync::Arc;
//!
//! use grpc::client::Channel;
//! use grpc::credentials::LocalChannelCredentials;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let channel = Channel::builder(
//!     "dns:///localhost:50051",
//!     Arc::new(LocalChannelCredentials::new()),
//!     grpc_tokio::runtime(),
//! )
//! .build();
//! # }
//! ```
//!
//! All operations must be performed from within the context of a Tokio runtime
//! with the IO and time drivers enabled.
//!
//! # Feature Flags
//!
//! * `dns` (enabled by default): Uses [hickory-resolver] for DNS resolution,
//!   which supports custom DNS authorities (e.g.
//!   `dns://8.8.8.8/foo.googleapis.com`). Without it, only host name lookups
//!   via the system resolver are supported.
//!
//! [hickory-resolver]: https://docs.rs/hickory-resolver

use std::sync::LazyLock;

use grpc::__unstable::UnstableToken;
use grpc::rt::GrpcRuntime;

#[cfg(feature = "dns")]
mod hickory_resolver;
mod runtime;
#[cfg(unix)]
mod unix;

use runtime::TokioRuntime;

/// The Tokio runtime is stateless, so all callers share one instance.
static RUNTIME: LazyLock<GrpcRuntime> =
    LazyLock::new(|| GrpcRuntime::new(TokioRuntime::default(), UnstableToken));

/// Returns a [`GrpcRuntime`] backed by Tokio.
///
/// This does not create a Tokio runtime: each operation uses the Tokio
/// runtime that is current when it is called (e.g. via `tokio::spawn`), which
/// must have the IO and time drivers enabled.
pub fn runtime() -> GrpcRuntime {
    RUNTIME.clone()
}
