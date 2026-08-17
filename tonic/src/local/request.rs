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

//! `IntoStreamingRequest` for `!Send` local mode.
//!
//! NB: keep in sync with the `IntoStreamingRequest` trait and blanket impls
//! in src/request.rs (deliberately parallel: the only difference is the
//! dropped `Send` bound on the stream, which is the feature).

use tokio_stream::Stream;

use crate::Request;

/// Trait implemented by RPC streaming request types, without a `Send` bound
/// on the stream.
///
/// Mirrors [`crate::IntoStreamingRequest`] for `tonic::local`; tonic-build's
/// local codegen and blanket impls handle this for you.
pub trait IntoStreamingRequest {
    /// The RPC request stream type
    type Stream: Stream<Item = Self::Message> + 'static;

    /// The RPC request type
    type Message;

    /// Wrap the stream of messages in a `tonic::Request`
    fn into_streaming_request(self) -> Request<Self::Stream>;
}

impl<T> IntoStreamingRequest for T
where
    T: Stream + 'static,
{
    type Stream = T;
    type Message = T::Item;

    fn into_streaming_request(self) -> Request<Self> {
        Request::new(self)
    }
}

impl<T> IntoStreamingRequest for Request<T>
where
    T: Stream + 'static,
{
    type Stream = T;
    type Message = T::Item;

    fn into_streaming_request(self) -> Self {
        self
    }
}
