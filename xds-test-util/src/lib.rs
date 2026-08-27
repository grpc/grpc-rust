/*
 *
 * Copyright 2025 gRPC authors.
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
//! Test utilities for xDS implementations.
//!
//! This crate provides helpers for exercising xDS clients against an in-process
//! management server. It is intended for tests only and not for production use.
//!
//! The main entry point is [`XdsTestControlPlaneService`], a fake Aggregated
//! Discovery Service (ADS) control plane. It is a Rust port of grpc-java's
//! `XdsTestControlPlaneService`.
//!
//! ```ignore
//! let running = XdsTestControlPlaneService::new().start().await?;
//! running.set_xds_config(&AdsTypeUrl::Lds, listeners);
//! let addr = running.addr(); // point your xDS bootstrap here
//! // `running` shuts the server down when dropped.
//! ```

pub mod config;
mod control_plane;

pub use control_plane::RunningControlPlane;
pub use control_plane::XdsTestControlPlaneService;
