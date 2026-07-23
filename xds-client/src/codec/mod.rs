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

//! Codec for encoding/decoding xDS messages.
//!
//! The codec layer converts between crate-owned message types
//! ([`DiscoveryRequest`], [`DiscoveryResponse`]) and serialized bytes.
//! This abstraction allows different protobuf implementations
//! (prost, google-protobuf) to be used with the same xDS client logic.

use crate::error::Result;
use crate::message::{DiscoveryRequest, DiscoveryResponse};
use bytes::Bytes;

#[cfg(feature = "codegen-prost")]
pub mod prost;

/// Trait for encoding/decoding xDS discovery messages.
///
/// Implementations convert between the crate-owned message types
/// and their serialized wire format.
pub trait XdsCodec: Send + Sync + 'static {
    /// Encode a [`DiscoveryRequest`] to bytes.
    fn encode_request(&self, request: &DiscoveryRequest<'_>) -> Result<Bytes>;

    /// Decode bytes into a [`DiscoveryResponse`].
    fn decode_response(&self, bytes: Bytes) -> Result<DiscoveryResponse>;
}
