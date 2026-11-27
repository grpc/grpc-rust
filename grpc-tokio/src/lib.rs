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
//! Use [`runtime()`] to get a gRPC runtime backed by the current Tokio runtime,
//! and pass it to the `grpc` APIs that take one:
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
//! To use a specific Tokio runtime instead of the current one, use
//! [`runtime_with_handle()`].
//!
//! A channel or server is bound to the Tokio runtime it was created with: all
//! of its background tasks, timers, and I/O run on that runtime, regardless of
//! which thread or executor calls into it. That runtime must have the IO and
//! time drivers enabled, and must outlive the channel or server. Using a
//! channel or server after its Tokio runtime has shut down is a programming
//! error: it may panic, cause RPCs to fail, or cause them to hang.
//!
//! # Feature Flags
//!
//! * `dns` (enabled by default): Uses [hickory-resolver] for DNS resolution,
//!   which supports custom DNS authorities (e.g.
//!   `dns://8.8.8.8/foo.googleapis.com`). Without it, only host name lookups
//!   via the system resolver are supported.
//!
//! [hickory-resolver]: https://docs.rs/hickory-resolver

use grpc::__unstable::UnstableToken;
use grpc::rt::GrpcRuntime;
use tokio::runtime::Handle;

#[cfg(feature = "dns")]
mod hickory_resolver;
mod runtime;
#[cfg(unix)]
mod unix;

use runtime::TokioRuntime;

/// Returns a [`GrpcRuntime`] bound to the current Tokio runtime.
///
/// This does not create a Tokio runtime: it captures the one that is current
/// when this function is called (see [`Handle::current`]), and runs all tasks,
/// timers, and I/O there.
///
/// # Panics
///
/// Panics if called outside the context of a Tokio runtime. Use
/// [`runtime_with_handle`] to choose the runtime explicitly.
///
/// Using the returned runtime (or any channel or server created with it) after
/// the Tokio runtime has shut down may also panic, cause RPCs to fail, or cause
/// them to hang.
pub fn runtime() -> GrpcRuntime {
    runtime_with_handle(Handle::current())
}

/// Returns a [`GrpcRuntime`] bound to the Tokio runtime identified by
/// `handle`.
///
/// All tasks, timers, and I/O run on that runtime, which must have the IO and
/// time drivers enabled.
///
/// # Panics
///
/// Using the returned runtime (or any channel or server created with it) after
/// that Tokio runtime has shut down may panic, cause RPCs to fail, or cause
/// them to hang.
pub fn runtime_with_handle(handle: Handle) -> GrpcRuntime {
    GrpcRuntime::new(TokioRuntime::new(handle), UnstableToken)
}
