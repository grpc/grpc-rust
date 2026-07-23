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

//! Generic client implementation.
//!
//! This module contains the low level components to build a gRPC client. It
//! provides a codec agnostic gRPC client dispatcher and a decorated tower
//! service trait.
//!
//! This client is generally used by some code generation tool to provide stubs
//! for the gRPC service. Thusly, they are a bit cumbersome to use by hand.
//!
//! ## Concurrent usage
//!
//! Upon using the your generated client, you will discover all the functions
//! corresponding to your rpc methods take `&mut self`, making concurrent
//! usage of the client difficult. The answer is simply to clone the client,
//! which is cheap as all client instances will share the same channel for
//! communication. For more details, see
//! [transport::Channel](../transport/struct.Channel.html#multiplexing-requests).

mod grpc;
mod service;

pub use self::grpc::Grpc;
pub use self::service::GrpcService;
