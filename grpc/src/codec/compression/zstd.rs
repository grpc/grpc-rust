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

use std::io::Write;

use bytes::Buf;
use bytes::BufMut;
use zstd::stream::write::Decoder;
use zstd::stream::write::Encoder;

use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;

/// A zstd compression implementation.
#[derive(Debug, Clone, Copy)]
pub struct Zstd {
    level: i32,
}

impl Zstd {
    /// Creates a new zstd compression implementation with default compression level.
    pub fn new() -> Self {
        Self::with_level(zstd::DEFAULT_COMPRESSION_LEVEL)
    }

    /// Creates a new zstd compression implementation with a specific compression level.
    pub fn with_level(level: i32) -> Self {
        Self { level }
    }
}

impl Default for Zstd {
    fn default() -> Self {
        Self::new()
    }
}

impl Compressor for Zstd {
    fn name(&self) -> &'static str {
        "zstd"
    }

    fn compress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut encoder =
            Encoder::new(destination.writer(), self.level).map_err(|e| e.to_string())?;
        while source.has_remaining() {
            let chunk = source.chunk();
            encoder.write_all(chunk).map_err(|e| e.to_string())?;
            source.advance(chunk.len());
        }
        encoder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Decompressor for Zstd {
    fn name(&self) -> &'static str {
        "zstd"
    }

    fn decompress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut decoder = Decoder::new(destination.writer()).map_err(|e| e.to_string())?;
        while source.has_remaining() {
            let chunk = source.chunk();
            decoder.write_all(chunk).map_err(|e| e.to_string())?;
            source.advance(chunk.len());
        }
        decoder.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn zstd_compress_decompress() {
        let compressor = Zstd::new();
        let data = Bytes::from_static(b"hello world");
        let mut compressed = Vec::new();
        compressor
            .compress(&mut data.clone(), &mut compressed)
            .unwrap();

        assert_ne!(compressed.as_slice(), data);
        let mut decompressed = Vec::new();
        compressor
            .decompress(&mut compressed.as_slice(), &mut decompressed)
            .unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}
