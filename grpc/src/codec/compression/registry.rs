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
use std::sync::RwLock;

use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;

/// A read-only interface to query supported compression algorithms.
pub trait CompressionResolver: Send + Sync + 'static {
    /// Get a shared reference to a compressor by its canonical encoding name.
    fn get_compressor(&self, name: &str) -> Result<Option<Arc<dyn Compressor>>, String>;

    /// Get a shared reference to a decompressor by its canonical encoding name.
    fn get_decompressor(&self, name: &str) -> Result<Option<Arc<dyn Decompressor>>, String>;

    /// Retrieve the list of supported compression encodings.
    fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String>;
}

/// A full interface to query and modify supported compression algorithms.
/// This allows components to dynamically register new algorithms at runtime.
pub trait CompressionRegistry: CompressionResolver {
    /// Register a compressor. Existing registrations for the same encoding name will be overwritten.
    fn register_compressor(&self, compressor: Arc<dyn Compressor>) -> Result<(), String>;

    /// Register a decompressor. Existing registrations for the same encoding name will be overwritten.
    fn register_decompressor(&self, decompressor: Arc<dyn Decompressor>) -> Result<(), String>;
}

/// The inner, immutable data layer holding the active compression algorithms.
///
/// Performance Characteristic: This structure is never mutated directly; it is always deeply cloned,
/// updated, and completely swapped via Arc pointers. This means readers only need to
/// hold the global lock long enough to clone an Arc, reducing lock contention.
#[derive(Clone)]
struct RegistryInner {
    compressors: HashMap<&'static str, Arc<dyn Compressor>>,
    decompressors: HashMap<&'static str, Arc<dyn Decompressor>>,
    accept_encodings: Arc<[&'static str]>,
}

impl Default for RegistryInner {
    fn default() -> Self {
        let mut inner = Self {
            compressors: HashMap::new(),
            decompressors: HashMap::new(),
            accept_encodings: Arc::new([]),
        };
        inner.register_defaults();
        inner
    }
}

impl RegistryInner {
    fn update_headers(&mut self) {
        let mut encodings: Vec<_> = self
            .decompressors
            .keys()
            .copied()
            .filter(|&k| k != "identity")
            .collect();
        encodings.sort_unstable();
        encodings.push("identity");
        self.accept_encodings = encodings.into();
    }

    fn register_defaults(&mut self) {
        // Register built-in compressors
        #[cfg(feature = "gzip")]
        self.compressors.insert(
            "gzip",
            Arc::new(crate::codec::compression::gzip::Gzip::default()),
        );
        #[cfg(feature = "deflate")]
        self.compressors.insert(
            "deflate",
            Arc::new(crate::codec::compression::deflate::Deflate::default()),
        );
        #[cfg(feature = "zstd")]
        self.compressors.insert(
            "zstd",
            Arc::new(crate::codec::compression::zstd::Zstd::default()),
        );

        // Register built-in decompressors
        #[cfg(feature = "gzip")]
        self.decompressors.insert(
            "gzip",
            Arc::new(crate::codec::compression::gzip::Gzip::default()),
        );
        #[cfg(feature = "deflate")]
        self.decompressors.insert(
            "deflate",
            Arc::new(crate::codec::compression::deflate::Deflate::default()),
        );
        #[cfg(feature = "zstd")]
        self.decompressors.insert(
            "zstd",
            Arc::new(crate::codec::compression::zstd::Zstd::default()),
        );

        self.update_headers();
    }
}

/// The global static synchronization layer holding the RCU pointer.
/// We use an RwLock to safely swap the Arc pointer when mutations occur.
static GLOBAL_COMPRESSION_REGISTRY: LazyLock<RwLock<Arc<RegistryInner>>> =
    LazyLock::new(|| RwLock::new(Arc::new(RegistryInner::default())));

/// A stateless unit struct proxying to the global static synchronization layer.
/// This struct serves as an implementor of CompressionRegistry/CompressionResolver, providing
/// a clean interface for consumers to interact with the global state.
#[derive(Debug, Default, Clone, Copy)]
pub struct GlobalCompressionRegistry;

impl CompressionResolver for GlobalCompressionRegistry {
    fn get_compressor(&self, name: &str) -> Result<Option<Arc<dyn Compressor>>, String> {
        // RCU Read: Acquire lock, clone Arc, drop lock.
        // TODO: Verify the safety of extracting the inner Arc from a poisoned lock here
        // and look for alternatives if necessary.
        let snapshot = match GLOBAL_COMPRESSION_REGISTRY.read() {
            Ok(g) => g.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };
        // Lock-free read on the immutable snapshot!
        Ok(snapshot.compressors.get(name).cloned())
    }

    fn get_decompressor(&self, name: &str) -> Result<Option<Arc<dyn Decompressor>>, String> {
        let snapshot = match GLOBAL_COMPRESSION_REGISTRY.read() {
            Ok(g) => g.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };
        Ok(snapshot.decompressors.get(name).cloned())
    }

    fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String> {
        let snapshot = match GLOBAL_COMPRESSION_REGISTRY.read() {
            Ok(g) => g.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };
        Ok(snapshot.accept_encodings.clone())
    }
}

impl CompressionRegistry for GlobalCompressionRegistry {
    fn register_compressor(&self, compressor: Arc<dyn Compressor>) -> Result<(), String> {
        // RCU Read-Copy-Update implementation
        let snapshot = match GLOBAL_COMPRESSION_REGISTRY.read() {
            Ok(g) => g.clone(),
            Err(_) => {
                return Err("Compression registry lock is poisoned; writes are disabled.".into());
            }
        };

        let mut new_inner = (*snapshot).clone(); // Deep clone HashMap data
        new_inner.compressors.insert(compressor.name(), compressor);

        // Swap pointer globally
        let mut write_guard = match GLOBAL_COMPRESSION_REGISTRY.write() {
            Ok(g) => g,
            Err(_) => {
                return Err("Compression registry lock is poisoned; writes are disabled.".into());
            }
        };
        *write_guard = Arc::new(new_inner);
        Ok(())
    }

    fn register_decompressor(&self, decompressor: Arc<dyn Decompressor>) -> Result<(), String> {
        let snapshot = match GLOBAL_COMPRESSION_REGISTRY.read() {
            Ok(g) => g.clone(),
            Err(_) => {
                return Err("Compression registry lock is poisoned; writes are disabled.".into());
            }
        };

        let mut new_inner = (*snapshot).clone();
        new_inner
            .decompressors
            .insert(decompressor.name(), decompressor);
        new_inner.update_headers();

        let mut write_guard = match GLOBAL_COMPRESSION_REGISTRY.write() {
            Ok(g) => g,
            Err(_) => {
                return Err("Compression registry lock is poisoned; writes are disabled.".into());
            }
        };
        *write_guard = Arc::new(new_inner);
        Ok(())
    }
}

/// Returns a handle to the global compression registry.
/// Ideal for default dependency injection in client channels, server endpoints, and interceptors.
pub fn global_compression_registry() -> impl CompressionRegistry + Clone {
    GlobalCompressionRegistry
}

#[cfg(test)]
mod tests {
    use bytes::Buf;
    use bytes::BufMut;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct MockCompression;

    impl Compressor for MockCompression {
        fn name(&self) -> &'static str {
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
        fn name(&self) -> &'static str {
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
        let registry = GlobalCompressionRegistry;

        // Verify gzip is present by default
        #[cfg(feature = "gzip")]
        {
            assert!(registry.get_compressor("gzip").unwrap().is_some());
            assert!(registry.get_decompressor("gzip").unwrap().is_some());
        }

        // Verify deflate is present by default
        #[cfg(feature = "deflate")]
        {
            assert!(registry.get_compressor("deflate").unwrap().is_some());
            assert!(registry.get_decompressor("deflate").unwrap().is_some());
        }

        // Verify zstd is present by default
        #[cfg(feature = "zstd")]
        {
            assert!(registry.get_compressor("zstd").unwrap().is_some());
            assert!(registry.get_decompressor("zstd").unwrap().is_some());
        }
    }

    #[test]
    fn accept_encoding() {
        let registry = GlobalCompressionRegistry;
        let encodings = registry.accept_encodings().unwrap();
        assert!(encodings.contains(&"identity"));
    }

    #[test]
    fn test_dynamic_registration_and_overwrite() {
        // We test registration on an isolated inner instance to avoid parallel test racing
        let mut inner = RegistryInner::default();

        // 1. Verify mock is not present
        assert!(!inner.compressors.contains_key("mock"));

        // 2. Register mock
        let mock = Arc::new(MockCompression);
        inner.compressors.insert("mock", mock.clone());
        assert!(inner.compressors.contains_key("mock"));

        // 3. Registering again with the same name overwrites correctly
        let mock2 = Arc::new(MockCompression);
        inner.compressors.insert("mock", mock2);
        assert!(inner.compressors.contains_key("mock"));
    }

    #[test]
    fn test_accept_encodings_header_update() {
        // We test header updates on an isolated inner instance to avoid parallel test racing
        let mut inner = RegistryInner::default();

        // 1. Ensure mock is not in accept_encodings
        assert!(!inner.accept_encodings.contains(&"mock"));

        // 2. Register mock decompressor
        let mock = Arc::new(MockCompression);
        inner.decompressors.insert("mock", mock);

        // 3. Trigger header update
        inner.update_headers();

        // 4. Verify mock is now successfully broadcasted in accept_encodings
        assert!(inner.accept_encodings.contains(&"mock"));
        assert!(inner.accept_encodings.contains(&"identity")); // Identity must always be present
    }

    // TODO: Figure out how to add unit test coverage for GLOBAL_COMPRESSION_REGISTRY
    // modifications (e.g. `register_compressor` on the proxy itself) without causing
    // flakiness due to parallel tests.
    #[test]
    fn safe_reads_on_poison() {
        use std::panic;
        let registry = GlobalCompressionRegistry;

        let _ = panic::catch_unwind(|| {
            let _guard = GLOBAL_COMPRESSION_REGISTRY.write().unwrap();
            panic!("intentionally poisoning lock");
        });

        // Reads should STILL work fine since the Arc wasn't corrupted!
        assert!(registry.get_compressor("mock").is_ok());

        // Writes should fail cleanly
        assert!(
            registry
                .register_compressor(Arc::new(MockCompression))
                .is_err()
        );
    }
}
