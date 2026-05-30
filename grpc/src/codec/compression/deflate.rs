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
use flate2::Compression as FlateCompression;
use flate2::write::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;

/// A deflate compression implementation.
#[derive(Debug, Clone, Copy)]
pub struct Deflate {
    level: FlateCompression,
}

impl Deflate {
    /// Creates a new deflate compression implementation.
    pub fn new() -> Self {
        Self {
            level: FlateCompression::new(6),
        }
    }
}

impl Default for Deflate {
    fn default() -> Self {
        Self::new()
    }
}

impl Compressor for Deflate {
    fn name(&self) -> &'static str {
        "deflate"
    }

    fn compress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut encoder = ZlibEncoder::new(destination.writer(), self.level);
        while source.has_remaining() {
            let chunk = source.chunk();
            encoder.write_all(chunk).map_err(|e| e.to_string())?;
            source.advance(chunk.len());
        }
        encoder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Decompressor for Deflate {
    fn name(&self) -> &'static str {
        "deflate"
    }

    fn decompress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut decoder = ZlibDecoder::new(destination.writer());
        while source.has_remaining() {
            let chunk = source.chunk();
            decoder.write_all(chunk).map_err(|e| e.to_string())?;
            source.advance(chunk.len());
        }
        decoder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn deflate_compress_decompress() {
        let compressor = Deflate::new();
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
