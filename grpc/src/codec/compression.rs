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

use bytes::Buf;
use bytes::BufMut;

#[cfg(feature = "deflate")]
mod deflate;
#[cfg(feature = "gzip")]
mod gzip;
#[cfg(feature = "zstd")]
mod zstd;

pub(crate) mod registry;

/// A trait for compressing outgoing gRPC payloads.
pub trait Compressor: Send + Sync + 'static {
    /// The canonical gRPC content coding name (e.g., "gzip").
    fn name(&self) -> &'static str;

    /// Compress data from `source` into `destination`.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if compression fails. Implementations should gracefully
    /// handle constrained `destination` buffers by returning an error rather than panicking
    /// (e.g., by verifying `destination.remaining_mut()` is sufficient before writing).
    fn compress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String>;
}

/// A trait for decompressing incoming gRPC payloads.
pub trait Decompressor: Send + Sync + 'static {
    /// The canonical gRPC content coding name (e.g., "gzip").
    fn name(&self) -> &'static str;

    /// Decompress data from `source` into `destination`.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if decompression fails. Implementations should gracefully
    /// handle constrained `destination` buffers by returning an error rather than panicking
    /// (e.g., by verifying `destination.remaining_mut()` is sufficient before writing).
    fn decompress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String>;
}
