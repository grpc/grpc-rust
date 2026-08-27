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

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;

const IDENTITY_ENCODING: &str = "identity";

/// The immutable data backing a [`CompressionRegistry`].
struct RegistryInner {
    compressors: HashMap<String, Arc<dyn Compressor>>,
    decompressors: HashMap<String, Arc<dyn Decompressor>>,
    accept_encodings: Arc<[String]>,
}

/// Computes the sorted `grpc-accept-encoding` list from the registered
/// decompressors, always placing `identity` last.
fn compute_accept_encodings(
    decompressors: &HashMap<String, Arc<dyn Decompressor>>,
) -> Arc<[String]> {
    let mut encodings: Vec<String> = decompressors
        .keys()
        .filter(|k| k.as_str() != IDENTITY_ENCODING)
        .cloned()
        .collect();
    encodings.sort_unstable();
    encodings.push(IDENTITY_ENCODING.to_owned());
    encodings.into()
}

/// A builder for assembling a [`CompressionRegistry`].
#[derive(Default)]
pub struct CompressionRegistryBuilder {
    compressors: HashMap<String, Arc<dyn Compressor>>,
    decompressors: HashMap<String, Arc<dyn Decompressor>>,
}

impl CompressionRegistryBuilder {
    /// Creates a builder with no codecs registered.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder pre-populated with the built-in codecs (`gzip`).
    pub fn with_defaults() -> Self {
        let mut builder = Self::default();
        builder.register_builtin_defaults();
        builder
    }

    /// Registers a compressor, keyed by its [`Compressor::name`].
    ///
    /// An existing registration for the same encoding name is overwritten.
    pub fn register_compressor(mut self, compressor: Arc<dyn Compressor>) -> Self {
        self.compressors
            .insert(compressor.name().to_owned(), compressor);
        self
    }

    /// Registers a decompressor, keyed by its [`Decompressor::name`].
    ///
    /// An existing registration for the same encoding name is overwritten.
    pub fn register_decompressor(mut self, decompressor: Arc<dyn Decompressor>) -> Self {
        self.decompressors
            .insert(decompressor.name().to_owned(), decompressor);
        self
    }

    /// Builds a [`CompressionRegistry`] from the registered codecs.
    pub fn build(self) -> CompressionRegistry {
        let accept_encodings = compute_accept_encodings(&self.decompressors);
        CompressionRegistry {
            inner: Arc::new(RegistryInner {
                compressors: self.compressors,
                decompressors: self.decompressors,
                accept_encodings,
            }),
        }
    }

    /// Registers the built-in codecs that ship out of the box.
    fn register_builtin_defaults(&mut self) {
        // Register built-in compressors.
        #[cfg(feature = "gzip")]
        self.compressors.insert(
            "gzip".to_owned(),
            Arc::new(crate::codec::compression::gzip::Gzip::default()),
        );

        // Register built-in decompressors.
        #[cfg(feature = "gzip")]
        self.decompressors.insert(
            "gzip".to_owned(),
            Arc::new(crate::codec::compression::gzip::Gzip::default()),
        );
    }
}

/// A set of compression codecs, looked up by their encoding name.
///
/// Build one with [`CompressionRegistryBuilder`], or use the default set of
/// built-in codecs via [`CompressionRegistry::global`]. Cheap to clone.
#[derive(Clone)]
pub struct CompressionRegistry {
    inner: Arc<RegistryInner>,
}

/// The default registry containing the built-in codecs.
static GLOBAL_COMPRESSION_REGISTRY: LazyLock<CompressionRegistry> =
    LazyLock::new(|| CompressionRegistryBuilder::with_defaults().build());

impl CompressionRegistry {
    /// Returns the default registry containing the built-in codecs.
    pub fn global() -> Self {
        GLOBAL_COMPRESSION_REGISTRY.clone()
    }

    /// Returns the compressor registered for the given encoding name, or `None`
    /// if none is registered.
    pub fn get_compressor(&self, name: &str) -> Option<Arc<dyn Compressor>> {
        self.inner.compressors.get(name).cloned()
    }

    /// Returns the decompressor registered for the given encoding name, or
    /// `None` if none is registered.
    pub fn get_decompressor(&self, name: &str) -> Option<Arc<dyn Decompressor>> {
        self.inner.decompressors.get(name).cloned()
    }

    /// Returns the encoding names to advertise in the `grpc-accept-encoding`
    /// header.
    pub fn accept_encodings(&self) -> &[String] {
        &self.inner.accept_encodings
    }
}

#[cfg(test)]
mod tests {
    use bytes::Buf;
    use bytes::BufMut;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct MockCompression;

    impl Compressor for MockCompression {
        fn name(&self) -> &str {
            "mock"
        }

        fn compress(
            &self,
            _source: &mut dyn Buf,
            _destination: &mut dyn BufMut,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    impl Decompressor for MockCompression {
        fn name(&self) -> &str {
            "mock"
        }

        fn decompress(
            &self,
            _source: &mut dyn Buf,
            _destination: &mut dyn BufMut,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_default_compressors_populated() {
        let registry = CompressionRegistry::global();

        // Verify gzip is present by default.
        #[cfg(feature = "gzip")]
        {
            assert!(registry.get_compressor("gzip").is_some());
            assert!(registry.get_decompressor("gzip").is_some());
        }
    }

    #[test]
    fn accept_encoding() {
        let registry = CompressionRegistry::global();
        let encodings = registry.accept_encodings();
        assert!(encodings.iter().any(|e| e == "identity"));
    }

    #[test]
    fn test_builder_registration_and_overwrite() {
        // A freshly built registry does not contain the mock codec.
        let registry = CompressionRegistryBuilder::new().build();
        assert!(registry.get_compressor("mock").is_none());

        // Registering the mock makes it available.
        let registry = CompressionRegistryBuilder::new()
            .register_compressor(Arc::new(MockCompression))
            .build();
        assert!(registry.get_compressor("mock").is_some());

        // Registering again with the same name overwrites correctly.
        let registry = CompressionRegistryBuilder::new()
            .register_compressor(Arc::new(MockCompression))
            .register_compressor(Arc::new(MockCompression))
            .build();
        assert!(registry.get_compressor("mock").is_some());
    }

    #[test]
    fn test_accept_encodings_header_update() {
        // A registry without the mock decompressor must not advertise it.
        let registry = CompressionRegistryBuilder::new().build();
        assert!(!registry.accept_encodings().iter().any(|e| e == "mock"));

        // Registering a mock decompressor advertises it, and `identity` is
        // always present.
        let registry = CompressionRegistryBuilder::new()
            .register_decompressor(Arc::new(MockCompression))
            .build();
        assert!(registry.accept_encodings().iter().any(|e| e == "mock"));
        assert!(registry.accept_encodings().iter().any(|e| e == "identity"));
    }
}
