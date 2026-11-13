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

//! [Load Reporting Service](https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/load_stats/v3/lrs.proto)
//! client.

use bytes::Bytes;

use crate::error::Result;
use crate::lrs::message::LoadStatsRequest;
use crate::lrs::message::LoadStatsResponse;

pub mod message;

/// Trait for encoding/decoding LRS (Load Reporting Service) messages.
///
/// Implementations convert between the crate-owned message types
/// and their serialized wire format.
pub trait LrsCodec: Send + Sync + 'static {
    /// Encode a [`LoadStatsRequest`] to bytes.
    fn encode_load_stats_request(&self, request: &LoadStatsRequest<'_>) -> Result<Bytes>;

    /// Decode bytes into a [`LoadStatsResponse`].
    fn decode_load_stats_response(&self, bytes: Bytes) -> Result<LoadStatsResponse>;
}
