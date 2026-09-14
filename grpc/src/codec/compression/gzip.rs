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
use flate2::Compress;
use flate2::Compression as FlateCompression;
use flate2::Decompress;
use flate2::FlushCompress;
use flate2::FlushDecompress;
use flate2::Status;

use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;

/// The base-2 logarithm of the gzip sliding window size. 15 is the maximum
/// (and standard) value; `flate2` adds the gzip framing on top of it.
const GZIP_WINDOW_BITS: u8 = 15;

/// A gzip compression implementation.
#[derive(Debug, Clone, Copy)]
pub struct Gzip {
    level: FlateCompression,
}

impl Gzip {
    /// Creates a new gzip compression implementation.
    pub fn new() -> Self {
        Self {
            level: FlateCompression::new(6),
        }
    }
}

impl Default for Gzip {
    fn default() -> Self {
        Self::new()
    }
}

impl Compressor for Gzip {
    fn name(&self) -> &str {
        "gzip"
    }

    /// Compresses `source` into `destination`, writing the gzip output directly
    /// into the destination's spare capacity (no intermediate buffer).
    ///
    /// # Errors
    ///
    /// Returns an error if compression fails or if `destination` is a
    /// fixed-capacity sink too small to hold the output. On error, the contents
    /// of `destination` are unspecified and should be discarded by the caller.
    fn compress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut compressor = Compress::new_gzip(self.level, GZIP_WINDOW_BITS);
        loop {
            let input = source.chunk();
            // Once every source byte lives in the current chunk, ask the codec
            // to emit the final gzip trailer via `Finish`.
            let flush = if source.remaining() == input.len() {
                FlushCompress::Finish
            } else {
                FlushCompress::None
            };

            // Writable spare capacity of the destination. `chunk_mut()` grows
            // growable sinks (`Vec`/`BytesMut`); a fixed sink may yield an empty
            // region once full.
            // SAFETY: `as_uninit_slice_mut` exposes the destination's
            // uninitialized spare capacity. We only ever write into it below (via
            // the codec) and never read it before writing.
            let output = unsafe { destination.chunk_mut().as_uninit_slice_mut() };
            if output.is_empty() {
                return Err("gzip: compression destination buffer is full".to_string());
            }

            let before_in = compressor.total_in();
            let before_out = compressor.total_out();
            let status = compressor
                .compress_uninit(input, output, flush)
                .map_err(|e| e.to_string())?;
            let consumed = (compressor.total_in() - before_in) as usize;
            let produced = (compressor.total_out() - before_out) as usize;

            source.advance(consumed);
            // SAFETY: the codec initialized exactly `produced` bytes at the start
            // of the region obtained above, and `produced <= output.len()` per
            // flate2's contract.
            unsafe { destination.advance_mut(produced) };

            match status {
                Status::StreamEnd => return Ok(()),
                Status::Ok | Status::BufError => {
                    if consumed == 0 && produced == 0 {
                        return Err("gzip: compression stalled with no progress".to_string());
                    }
                }
            }
        }
    }
}

impl Decompressor for Gzip {
    fn name(&self) -> &str {
        "gzip"
    }

    /// Decompresses `source` into `destination`, writing the inflated output
    /// directly into the destination's spare capacity (no intermediate buffer).
    ///
    /// `source` must contain exactly one gzip member. A gRPC message frame
    /// carries a single compressed member, so any bytes remaining after the end
    /// of the gzip stream — trailing garbage or a concatenated additional
    /// member — are treated as malformed and rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the stream is corrupt or truncated, if there is
    /// trailing data after the gzip member, or if `destination` is a
    /// fixed-capacity sink too small to hold the output. On error, the contents
    /// of `destination` are unspecified and should be discarded by the caller.
    fn decompress(&self, source: &mut dyn Buf, destination: &mut dyn BufMut) -> Result<(), String> {
        let mut decompressor = Decompress::new_gzip(GZIP_WINDOW_BITS);
        loop {
            let input = source.chunk();

            // Writable spare capacity of the destination. `chunk_mut()` grows
            // growable sinks (`Vec`/`BytesMut`); a fixed sink may yield an empty
            // region once full.
            // SAFETY: `as_uninit_slice_mut` exposes the destination's
            // uninitialized spare capacity. We only ever write into it below (via
            // the codec) and never read it before writing.
            let output = unsafe { destination.chunk_mut().as_uninit_slice_mut() };
            if output.is_empty() {
                return Err("gzip: decompression destination buffer is full".to_string());
            }

            let before_in = decompressor.total_in();
            let before_out = decompressor.total_out();
            let status = decompressor
                .decompress_uninit(input, output, FlushDecompress::None)
                .map_err(|e| e.to_string())?;
            let consumed = (decompressor.total_in() - before_in) as usize;
            let produced = (decompressor.total_out() - before_out) as usize;

            source.advance(consumed);
            // SAFETY: the codec initialized exactly `produced` bytes at the start
            // of the region obtained above, and `produced <= output.len()` per
            // flate2's contract.
            unsafe { destination.advance_mut(produced) };

            match status {
                Status::StreamEnd => {
                    // Reject any bytes after the end of the gzip stream (see the
                    // single-member contract in the method docs).
                    if source.has_remaining() {
                        return Err("gzip: trailing data after end of gzip stream".to_string());
                    }
                    return Ok(());
                }
                Status::Ok | Status::BufError => {
                    // No forward progress means the input is truncated (needs
                    // more bytes) with nothing left to feed it.
                    if consumed == 0 && produced == 0 {
                        return Err("gzip: truncated or corrupt gzip stream".to_string());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn gzip_compress_decompress() {
        let compressor = Gzip::new();
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

    /// Compresses then decompresses `data` through growable sinks that start
    /// empty, forcing repeated `chunk_mut()` growth, and asserts the round trip
    /// reproduces the original bytes.
    fn assert_round_trip(data: &[u8]) {
        let gzip = Gzip::new();

        let mut compressed = Vec::new();
        gzip.compress(&mut Bytes::copy_from_slice(data), &mut compressed)
            .unwrap();

        let mut decompressed = Vec::new();
        gzip.decompress(&mut compressed.as_slice(), &mut decompressed)
            .unwrap();

        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn round_trip_empty() {
        assert_round_trip(b"");
    }

    #[test]
    fn round_trip_small() {
        assert_round_trip(b"hello world");
    }

    #[test]
    fn round_trip_large_compressible() {
        // > 32KB of highly compressible data to force multiple grow cycles.
        let data = vec![b'a'; 100 * 1024];
        assert_round_trip(&data);
    }

    #[test]
    fn compress_into_too_small_buffer_errors() {
        let gzip = Gzip::new();
        let data = vec![b'x'; 8 * 1024];

        // A fixed, non-growable sink far too small for the gzip output.
        let mut tiny = [0u8; 4];
        let mut sink: &mut [u8] = &mut tiny;
        let result = gzip.compress(&mut Bytes::copy_from_slice(&data), &mut sink);

        // Must return Err (not panic, not overrun).
        assert!(result.is_err());
    }

    #[test]
    fn decompress_into_too_small_buffer_errors() {
        let gzip = Gzip::new();
        let data = vec![b'y'; 8 * 1024];
        let mut compressed = Vec::new();
        gzip.compress(&mut Bytes::copy_from_slice(&data), &mut compressed)
            .unwrap();

        // Decompress into a fixed sink too small to hold the 8KB output.
        let mut tiny = [0u8; 16];
        let mut sink: &mut [u8] = &mut tiny;
        let result = gzip.decompress(&mut compressed.as_slice(), &mut sink);

        assert!(result.is_err());
    }

    #[test]
    fn compress_does_not_write_past_fixed_region() {
        // Guard-byte canary: the destination is a sub-slice of a larger backing
        // array; the trailing sentinel bytes must remain untouched even when the
        // destination fills up and compression fails.
        let gzip = Gzip::new();
        let data = vec![b'z'; 8 * 1024];

        const DEST_LEN: usize = 8;
        const SENTINEL: u8 = 0xAB;
        let mut backing = [SENTINEL; DEST_LEN + 16];
        {
            let (dest, _guard) = backing.split_at_mut(DEST_LEN);
            let mut sink: &mut [u8] = dest;
            // Expected to error since the destination is far too small.
            let _ = gzip.compress(&mut Bytes::copy_from_slice(&data), &mut sink);
        }
        // The guard region after the destination must be pristine.
        assert!(backing[DEST_LEN..].iter().all(|&b| b == SENTINEL));
    }

    #[test]
    fn decompress_split_across_header() {
        // The gzip header is 10 bytes; split the source inside it (at byte 4) so
        // the decoder must resume header parsing across a chunk boundary.
        let data = b"streaming across the gzip header boundary";
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(&mut Bytes::copy_from_slice(data), &mut compressed)
            .unwrap();
        assert!(compressed.len() > 4);

        let mut source = (&compressed[..4]).chain(&compressed[4..]);
        let mut decompressed = Vec::new();
        Gzip::new()
            .decompress(&mut source, &mut decompressed)
            .unwrap();
        assert_eq!(decompressed.as_slice(), data);
    }

    #[test]
    fn decompress_split_across_trailer() {
        // The 8-byte CRC32/ISIZE trailer sits at the end; split within it so the
        // decoder must consume the trailer across a chunk boundary.
        let data = b"streaming across the gzip trailer boundary";
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(&mut Bytes::copy_from_slice(data), &mut compressed)
            .unwrap();
        let split = compressed.len() - 4;

        let mut source = (&compressed[..split]).chain(&compressed[split..]);
        let mut decompressed = Vec::new();
        Gzip::new()
            .decompress(&mut source, &mut decompressed)
            .unwrap();
        assert_eq!(decompressed.as_slice(), data);
    }

    #[test]
    fn decompress_truncated_stream_errors() {
        // Dropping the trailing bytes of a valid stream leaves the decoder
        // wanting more input it never receives -> truncation error.
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(
                &mut Bytes::from_static(b"a truncated gzip stream must be rejected"),
                &mut compressed,
            )
            .unwrap();
        let truncated = &compressed[..compressed.len() - 4];

        let mut decompressed = Vec::new();
        let result = Gzip::new().decompress(&mut &truncated[..], &mut decompressed);
        assert!(result.is_err());
    }

    #[test]
    fn decompress_corrupt_trailer_errors() {
        // Flipping a byte in the CRC32/ISIZE trailer must fail integrity checks.
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(
                &mut Bytes::from_static(b"corrupt trailer must be detected"),
                &mut compressed,
            )
            .unwrap();
        let last = compressed.len() - 1;
        compressed[last] ^= 0xFF;

        let mut decompressed = Vec::new();
        let result = Gzip::new().decompress(&mut compressed.as_slice(), &mut decompressed);
        assert!(result.is_err());
    }

    #[test]
    fn decompress_trailing_data_errors() {
        // Matches gRPC C-core: bytes after a complete gzip member are trailing
        // data and must be rejected (a gRPC frame carries exactly one member).
        let data = b"member followed by trailing garbage";
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(&mut Bytes::copy_from_slice(data), &mut compressed)
            .unwrap();
        compressed.extend_from_slice(b"trailing garbage not part of the stream");

        let mut decompressed = Vec::new();
        let result = Gzip::new().decompress(&mut compressed.as_slice(), &mut decompressed);
        assert!(result.is_err());
    }

    #[test]
    fn decompress_multi_member_errors() {
        // Matches gRPC C-core: a concatenated second gzip member is trailing
        // data after the first member's end, so decoding must error rather than
        // silently decode only the first member.
        let mut compressed = Vec::new();
        Gzip::new()
            .compress(&mut Bytes::from_static(b"first member"), &mut compressed)
            .unwrap();
        let mut second_member = Vec::new();
        Gzip::new()
            .compress(
                &mut Bytes::from_static(b"second member"),
                &mut second_member,
            )
            .unwrap();
        compressed.extend_from_slice(&second_member);

        let mut decompressed = Vec::new();
        let result = Gzip::new().decompress(&mut compressed.as_slice(), &mut decompressed);
        assert!(result.is_err());
    }
}
