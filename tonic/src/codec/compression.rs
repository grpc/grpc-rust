use crate::{metadata::MetadataValue, Status};
use bytes::{Buf, BufMut, BytesMut};
#[cfg(feature = "gzip")]
use flate2::read::{GzDecoder, GzEncoder};
#[cfg(feature = "deflate")]
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::{borrow::Cow, fmt};
#[cfg(feature = "zstd")]
use zstd::stream::read::{Decoder, Encoder};

pub(crate) const ENCODING_HEADER: &str = "grpc-encoding";
pub(crate) const ACCEPT_ENCODING_HEADER: &str = "grpc-accept-encoding";

/// Struct used to configure which encodings are enabled on a server or channel.
///
/// Represents an ordered list of compression encodings that are enabled.
#[derive(Debug, Default, Clone, Copy)]
pub struct EnabledCompressionEncodings {
    // One slot per encoding. Increase this when adding a new codec; otherwise
    // enable() silently ignores it when the array is full.
    inner: [Option<CompressionConfig>; 3],
}

impl EnabledCompressionEncodings {
    /// Enable a [`CompressionEncoding`].
    ///
    /// Adds the new encoding to the end of the encoding list, or updates the
    /// settings of an already enabled encoding without changing its position.
    // CompressionEncoding is uninhabited when no compression features are enabled.
    #[allow(unreachable_code)]
    pub fn enable(&mut self, encoding: CompressionEncoding) {
        self.enable_with_config(encoding.into());
    }

    /// Enable an encoding with compression settings.
    ///
    /// Replaces settings for an enabled encoding without changing its position.
    pub fn enable_with_config(&mut self, config: CompressionConfig) {
        for e in self.inner.iter_mut() {
            match e {
                Some(e) if e.encoding == config.encoding => {
                    *e = config;
                    return;
                }
                None => {
                    *e = Some(config);
                    return;
                }
                _ => continue,
            }
        }
    }

    /// Remove the last [`CompressionEncoding`].
    pub fn pop(&mut self) -> Option<CompressionEncoding> {
        self.inner
            .iter_mut()
            .rev()
            .find(|entry| entry.is_some())?
            .take()
            .map(CompressionConfig::encoding)
    }

    pub(crate) fn into_accept_encoding_header_value(self) -> Option<http::HeaderValue> {
        let mut value = BytesMut::new();
        for encoding in self.inner.into_iter().flatten() {
            value.put_slice(encoding.encoding.as_str().as_bytes());
            value.put_u8(b',');
        }

        if value.is_empty() {
            return None;
        }

        value.put_slice(b"identity");
        Some(http::HeaderValue::from_maybe_shared(value).unwrap())
    }

    /// Check if a [`CompressionEncoding`] is enabled.
    ///
    /// The compression level is ignored.
    pub fn is_enabled(&self, encoding: CompressionEncoding) -> bool {
        self.get(encoding.as_str()).is_some()
    }

    pub(crate) fn get(&self, name: &str) -> Option<CompressionConfig> {
        self.inner
            .iter()
            .flatten()
            .copied()
            .find(|config| config.encoding.as_str() == name)
    }

    /// Check if any [`CompressionEncoding`]s are enabled.
    pub fn is_empty(&self) -> bool {
        self.inner.iter().all(|e| e.is_none())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CompressionSettings {
    pub(crate) config: CompressionConfig,
    /// buffer_growth_interval controls memory growth for internal buffers to balance resizing cost against memory waste.
    /// The default buffer growth interval is 8 kilobytes.
    pub(crate) buffer_growth_interval: usize,
}

/// A gzip compression level between 0 (no compression) and 9 (best compression).
///
/// Level 1 favors speed. Values outside the supported range are rejected.
///
/// ```
/// # #[cfg(feature = "gzip")]
/// # {
/// use tonic::codec::{CompressionConfig, GzipLevel};
///
/// let config: CompressionConfig = GzipLevel::FAST.into();
/// # }
/// ```
#[cfg(feature = "gzip")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GzipLevel(u32);

#[cfg(feature = "gzip")]
impl GzipLevel {
    /// The minimum supported compression level (no compression).
    pub const MIN: u32 = 0;
    /// The maximum supported compression level (best compression).
    pub const MAX: u32 = 9;

    /// No compression (level 0). Messages still use the gzip format.
    pub const NONE: Self = Self(0);
    /// Fast compression (level 1).
    pub const FAST: Self = Self(1);
    /// Best compression (level 9).
    pub const BEST: Self = Self(9);

    /// Returns the numeric compression level.
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[cfg(feature = "gzip")]
impl TryFrom<u32> for GzipLevel {
    type Error = InvalidGzipLevel;

    fn try_from(level: u32) -> Result<Self, Self::Error> {
        if level <= Self::MAX {
            Ok(Self(level))
        } else {
            Err(InvalidGzipLevel(level))
        }
    }
}

/// The requested gzip compression level is outside the supported range of 0 to 9.
///
/// Returned by [`GzipLevel::try_from`].
#[cfg(feature = "gzip")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidGzipLevel(u32);

#[cfg(feature = "gzip")]
impl fmt::Display for InvalidGzipLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = self.0;
        write!(
            f,
            "gzip compression level {level} is out of range {}..={}",
            GzipLevel::MIN,
            GzipLevel::MAX,
        )
    }
}

#[cfg(feature = "gzip")]
impl std::error::Error for InvalidGzipLevel {}

/// The compression encodings Tonic supports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompressionEncoding {
    /// Gzip compression with the default compression level
    /// ([`flate2::Compression::default()`]).
    #[cfg(feature = "gzip")]
    Gzip,
    #[allow(missing_docs)]
    #[cfg(feature = "deflate")]
    Deflate,
    #[allow(missing_docs)]
    #[cfg(feature = "zstd")]
    Zstd,
}

impl CompressionEncoding {
    pub(crate) const ENCODINGS: &'static [CompressionEncoding] = &[
        #[cfg(feature = "gzip")]
        CompressionEncoding::Gzip,
        #[cfg(feature = "deflate")]
        CompressionEncoding::Deflate,
        #[cfg(feature = "zstd")]
        CompressionEncoding::Zstd,
    ];

    /// Based on the `grpc-accept-encoding` header, pick an encoding to use.
    /// Retains the locally configured level for compressing outgoing messages.
    pub(crate) fn from_accept_encoding_header(
        map: &http::HeaderMap,
        enabled_encodings: EnabledCompressionEncodings,
    ) -> Option<CompressionConfig> {
        if enabled_encodings.is_empty() {
            return None;
        }

        let header_value = map.get(ACCEPT_ENCODING_HEADER)?;
        let header_value_str = header_value.to_str().ok()?;

        split_by_comma(header_value_str).find_map(|value| enabled_encodings.get(value))
    }

    /// Get the value of `grpc-encoding` header. Returns an error if the encoding isn't supported.
    /// Decoding only needs the encoding; compression levels are not negotiated.
    pub(crate) fn from_encoding_header(
        map: &http::HeaderMap,
        enabled_encodings: EnabledCompressionEncodings,
    ) -> Result<Option<Self>, Status> {
        let Some(header_value) = map.get(ENCODING_HEADER) else {
            return Ok(None);
        };

        if let Some(encoding) = header_value
            .to_str()
            .ok()
            .and_then(|name| enabled_encodings.get(name))
            .map(CompressionConfig::encoding)
        {
            return Ok(Some(encoding));
        }

        match header_value.as_bytes() {
            b"identity" => Ok(None),
            other => {
                let other = match std::str::from_utf8(other) {
                    Ok(s) => Cow::Borrowed(s),
                    Err(_) => Cow::Owned(format!("{other:?}")),
                };

                let mut status = Status::unimplemented(format!(
                    "Content is compressed with `{other}` which isn't supported"
                ));

                let header_value = enabled_encodings
                    .into_accept_encoding_header_value()
                    .map(MetadataValue::unchecked_from_header_value)
                    .unwrap_or_else(|| MetadataValue::from_static("identity"));
                status
                    .metadata_mut()
                    .insert(ACCEPT_ENCODING_HEADER, header_value);

                Err(status)
            }
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            #[cfg(feature = "gzip")]
            CompressionEncoding::Gzip => "gzip",
            #[cfg(feature = "deflate")]
            CompressionEncoding::Deflate => "deflate",
            #[cfg(feature = "zstd")]
            CompressionEncoding::Zstd => "zstd",
        }
    }

    #[cfg(any(feature = "gzip", feature = "deflate", feature = "zstd"))]
    pub(crate) fn into_header_value(self) -> http::HeaderValue {
        http::HeaderValue::from_static(self.as_str())
    }
}

impl fmt::Display for CompressionEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An encoding and its settings for compressing outgoing messages.
///
/// Converting a [`CompressionEncoding`] uses its default settings. With the
/// `gzip` feature, converting a `GzipLevel` selects gzip at that level.
/// Pass this configuration to `send_compressed_with_config` on a client or server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompressionConfig {
    encoding: CompressionEncoding,
    #[cfg(feature = "gzip")]
    gzip_level: Option<GzipLevel>,
}

impl CompressionConfig {
    /// Returns the encoding without its local compression settings.
    pub const fn encoding(self) -> CompressionEncoding {
        self.encoding
    }

    #[cfg(feature = "gzip")]
    fn gzip_level(self) -> flate2::Compression {
        self.gzip_level
            .map(|level| flate2::Compression::new(level.get()))
            .unwrap_or_default()
    }
}

impl From<CompressionEncoding> for CompressionConfig {
    fn from(encoding: CompressionEncoding) -> Self {
        Self {
            encoding,
            #[cfg(feature = "gzip")]
            gzip_level: None,
        }
    }
}

#[cfg(feature = "gzip")]
impl From<GzipLevel> for CompressionConfig {
    fn from(level: GzipLevel) -> Self {
        Self {
            encoding: CompressionEncoding::Gzip,
            gzip_level: Some(level),
        }
    }
}

fn split_by_comma(s: &str) -> impl Iterator<Item = &str> {
    s.split(',').map(|s| s.trim())
}

/// Compress `len` bytes from `decompressed_buf` into `out_buf`.
/// buffer_size_increment is a hint to control the growth of out_buf versus the cost of resizing it.
#[allow(unused_variables, unreachable_code)]
pub(crate) fn compress(
    settings: CompressionSettings,
    decompressed_buf: &mut BytesMut,
    out_buf: &mut BytesMut,
    len: usize,
) -> Result<(), std::io::Error> {
    let buffer_growth_interval = settings.buffer_growth_interval;
    let capacity = ((len / buffer_growth_interval) + 1) * buffer_growth_interval;
    out_buf.reserve(capacity);

    #[cfg(any(feature = "gzip", feature = "deflate", feature = "zstd"))]
    let mut out_writer = out_buf.writer();

    match settings.config.encoding {
        #[cfg(feature = "gzip")]
        CompressionEncoding::Gzip => {
            let mut gzip_encoder =
                GzEncoder::new(&decompressed_buf[0..len], settings.config.gzip_level());
            std::io::copy(&mut gzip_encoder, &mut out_writer)?;
        }
        #[cfg(feature = "deflate")]
        CompressionEncoding::Deflate => {
            let mut deflate_encoder = ZlibEncoder::new(
                &decompressed_buf[0..len],
                // FIXME: Support custom levels with a validated level type and per-codec
                // error, following GzipLevel and CompressionConfig (grpc/grpc-rust#1237).
                flate2::Compression::new(6),
            );
            std::io::copy(&mut deflate_encoder, &mut out_writer)?;
        }
        #[cfg(feature = "zstd")]
        CompressionEncoding::Zstd => {
            let mut zstd_encoder = Encoder::new(
                &decompressed_buf[0..len],
                // FIXME: Support custom levels with a validated level type and per-codec
                // error, following GzipLevel and CompressionConfig (grpc/grpc-rust#1237).
                zstd::DEFAULT_COMPRESSION_LEVEL,
            )?;
            std::io::copy(&mut zstd_encoder, &mut out_writer)?;
        }
    }

    decompressed_buf.advance(len);

    Ok(())
}

/// Decompress `len` bytes from `compressed_buf` into `out_buf`.
#[allow(unused_variables, unreachable_code)]
pub(crate) fn decompress(
    settings: CompressionSettings,
    compressed_buf: &mut BytesMut,
    mut out_buf: bytes::buf::Limit<&mut BytesMut>,
    len: usize,
) -> Result<(), std::io::Error> {
    let buffer_growth_interval = settings.buffer_growth_interval;
    let estimate_decompressed_len = len * 2;
    let capacity = std::cmp::min(
        bytes::buf::Limit::limit(&out_buf),
        ((estimate_decompressed_len / buffer_growth_interval) + 1) * buffer_growth_interval,
    );

    out_buf.get_mut().reserve(capacity);

    #[cfg(any(feature = "gzip", feature = "deflate", feature = "zstd"))]
    let mut out_writer = out_buf.writer();

    match settings.config.encoding {
        #[cfg(feature = "gzip")]
        CompressionEncoding::Gzip => {
            let mut gzip_decoder = GzDecoder::new(&compressed_buf[0..len]);
            std::io::copy(&mut gzip_decoder, &mut out_writer)?;
        }
        #[cfg(feature = "deflate")]
        CompressionEncoding::Deflate => {
            let mut deflate_decoder = ZlibDecoder::new(&compressed_buf[0..len]);
            std::io::copy(&mut deflate_decoder, &mut out_writer)?;
        }
        #[cfg(feature = "zstd")]
        CompressionEncoding::Zstd => {
            let mut zstd_decoder = Decoder::new(&compressed_buf[0..len])?;
            std::io::copy(&mut zstd_decoder, &mut out_writer)?;
        }
    }

    compressed_buf.advance(len);

    Ok(())
}

/// Controls compression behavior for individual messages within a stream.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SingleMessageCompressionOverride {
    /// Inherit whatever compression is already configured. If the stream is compressed this
    /// message will also be configured.
    ///
    /// This is the default.
    #[default]
    Inherit,
    /// Don't compress this message, even if compression is enabled on the stream.
    Disable,
}

#[cfg(test)]
mod tests {
    #[cfg(any(feature = "gzip", feature = "deflate", feature = "zstd"))]
    use http::HeaderValue;

    use super::*;

    #[test]
    fn convert_none_into_header_value() {
        let encodings = EnabledCompressionEncodings::default();

        assert!(encodings.into_accept_encoding_header_value().is_none());
    }

    #[test]
    fn from_encoding_header_without_enabled_encodings() {
        let enabled = EnabledCompressionEncodings::default();
        let mut headers = http::HeaderMap::new();
        assert_eq!(
            CompressionEncoding::from_encoding_header(&headers, enabled).unwrap(),
            None
        );
        headers.insert(ENCODING_HEADER, http::HeaderValue::from_static("identity"));
        assert_eq!(
            CompressionEncoding::from_encoding_header(&headers, enabled).unwrap(),
            None
        );

        for value in [b"gzip".as_slice(), b"unknown", b"\xff"] {
            headers.insert(
                ENCODING_HEADER,
                http::HeaderValue::from_bytes(value).unwrap(),
            );
            let status = CompressionEncoding::from_encoding_header(&headers, enabled).unwrap_err();
            assert_eq!(status.code(), crate::Code::Unimplemented);
            assert_eq!(
                status.metadata().get(ACCEPT_ENCODING_HEADER).unwrap(),
                "identity"
            );
        }
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn gzip_level_negotiation() {
        let mut enabled = EnabledCompressionEncodings::default();
        enabled.enable(CompressionEncoding::Gzip);
        enabled.enable_with_config(GzipLevel::FAST.into());
        enabled.enable_with_config(GzipLevel::BEST.into());

        assert!(enabled.is_enabled(CompressionEncoding::Gzip));
        assert_eq!(
            enabled.into_accept_encoding_header_value().unwrap(),
            "gzip,identity"
        );
        assert_eq!(
            CompressionConfig::from(GzipLevel::BEST)
                .encoding()
                .into_header_value(),
            "gzip"
        );

        let mut headers = http::HeaderMap::new();
        headers.insert(ACCEPT_ENCODING_HEADER, HeaderValue::from_static("gzip"));
        headers.insert(ENCODING_HEADER, HeaderValue::from_static("gzip"));
        assert_eq!(
            CompressionEncoding::from_accept_encoding_header(&headers, enabled),
            Some(GzipLevel::BEST.into())
        );
        assert_eq!(
            CompressionEncoding::from_encoding_header(&headers, enabled).unwrap(),
            Some(CompressionEncoding::Gzip)
        );
        assert_eq!(enabled.pop(), Some(CompressionEncoding::Gzip));
        assert!(enabled.is_empty());
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn gzip_level_validation() {
        for level in GzipLevel::MIN..=GzipLevel::MAX {
            assert_eq!(GzipLevel::try_from(level).unwrap().get(), level);
        }
        for level in [GzipLevel::MAX + 1, u32::MAX] {
            let error = GzipLevel::try_from(level).unwrap_err();
            assert_eq!(error, InvalidGzipLevel(level));
            assert_eq!(
                error.to_string(),
                format!("gzip compression level {level} is out of range 0..=9")
            );
        }
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn enable_gzip_resets_level() {
        let mut enabled = EnabledCompressionEncodings::default();
        enabled.enable_with_config(GzipLevel::BEST.into());
        #[cfg(feature = "zstd")]
        enabled.enable(CompressionEncoding::Zstd);

        enabled.enable(CompressionEncoding::Gzip);
        assert_eq!(enabled.get("gzip"), Some(CompressionEncoding::Gzip.into()));
        #[cfg(feature = "zstd")]
        assert_eq!(enabled.pop(), Some(CompressionEncoding::Zstd));
        assert_eq!(enabled.pop(), Some(CompressionEncoding::Gzip));
        assert!(enabled.is_empty());
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn gzip_compression_levels() {
        let data = b"some data to compress".repeat(1024);
        for level in GzipLevel::MIN..=GzipLevel::MAX {
            let gzip_level = match level {
                0 => GzipLevel::NONE,
                1 => GzipLevel::FAST,
                9 => GzipLevel::BEST,
                _ => GzipLevel::try_from(level).unwrap(),
            };
            let settings = CompressionSettings {
                config: gzip_level.into(),
                buffer_growth_interval: 8192,
            };
            let mut input = BytesMut::from(data.as_slice());
            let mut compressed = BytesMut::new();
            compress(settings, &mut input, &mut compressed, data.len()).unwrap();
            assert!(input.is_empty());

            let mut expected = Vec::new();
            std::io::copy(
                &mut GzEncoder::new(data.as_slice(), flate2::Compression::new(level)),
                &mut expected,
            )
            .unwrap();
            assert_eq!(compressed.as_ref(), expected);

            if level == 6 {
                let mut default_compressed = BytesMut::new();
                compress(
                    CompressionSettings {
                        config: CompressionEncoding::Gzip.into(),
                        ..settings
                    },
                    &mut BytesMut::from(data.as_slice()),
                    &mut default_compressed,
                    data.len(),
                )
                .unwrap();
                assert_eq!(compressed, default_compressed);
            }

            for config in [CompressionEncoding::Gzip.into(), settings.config] {
                let mut compressed = compressed.clone();
                let len = compressed.len();
                let mut decompressed = BytesMut::new();
                decompress(
                    CompressionSettings { config, ..settings },
                    &mut compressed,
                    (&mut decompressed).limit(data.len()),
                    len,
                )
                .unwrap();
                assert!(compressed.is_empty());
                assert_eq!(decompressed.as_ref(), data);
            }
        }
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn convert_gzip_into_header_value() {
        const GZIP: HeaderValue = HeaderValue::from_static("gzip,identity");

        let encodings = EnabledCompressionEncodings {
            inner: [Some(CompressionEncoding::Gzip.into()), None, None],
        };

        assert_eq!(encodings.into_accept_encoding_header_value().unwrap(), GZIP);

        let encodings = EnabledCompressionEncodings {
            inner: [None, None, Some(CompressionEncoding::Gzip.into())],
        };

        assert_eq!(encodings.into_accept_encoding_header_value().unwrap(), GZIP);
    }

    #[test]
    #[cfg(feature = "zstd")]
    fn convert_zstd_into_header_value() {
        const ZSTD: HeaderValue = HeaderValue::from_static("zstd,identity");

        let encodings = EnabledCompressionEncodings {
            inner: [Some(CompressionEncoding::Zstd.into()), None, None],
        };

        assert_eq!(encodings.into_accept_encoding_header_value().unwrap(), ZSTD);

        let encodings = EnabledCompressionEncodings {
            inner: [None, None, Some(CompressionEncoding::Zstd.into())],
        };

        assert_eq!(encodings.into_accept_encoding_header_value().unwrap(), ZSTD);
    }

    #[test]
    #[cfg(all(feature = "gzip", feature = "deflate", feature = "zstd"))]
    fn convert_compression_encodings_into_header_value() {
        let encodings = EnabledCompressionEncodings {
            inner: [
                Some(CompressionEncoding::Gzip.into()),
                Some(CompressionEncoding::Deflate.into()),
                Some(CompressionEncoding::Zstd.into()),
            ],
        };

        assert_eq!(
            encodings.into_accept_encoding_header_value().unwrap(),
            HeaderValue::from_static("gzip,deflate,zstd,identity"),
        );

        let encodings = EnabledCompressionEncodings {
            inner: [
                Some(CompressionEncoding::Zstd.into()),
                Some(CompressionEncoding::Deflate.into()),
                Some(CompressionEncoding::Gzip.into()),
            ],
        };

        assert_eq!(
            encodings.into_accept_encoding_header_value().unwrap(),
            HeaderValue::from_static("zstd,deflate,gzip,identity"),
        );
    }
}
