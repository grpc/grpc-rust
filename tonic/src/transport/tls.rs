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

/// Represents a X509 certificate.
#[derive(Debug, Clone)]
pub struct Certificate {
    pub(crate) pem: Vec<u8>,
}

/// Represents a private key and X509 certificate.
#[derive(Debug, Clone)]
pub struct Identity {
    pub(crate) cert: Certificate,
    pub(crate) key: Vec<u8>,
}

impl Certificate {
    /// Parse a PEM encoded X509 Certificate.
    ///
    /// The provided PEM should include at least one PEM encoded certificate.
    pub fn from_pem(pem: impl AsRef<[u8]>) -> Self {
        let pem = pem.as_ref().into();
        Self { pem }
    }

    /// Get a immutable reference to underlying certificate
    pub fn get_ref(&self) -> &[u8] {
        self.pem.as_slice()
    }

    /// Get a mutable reference to underlying certificate
    pub fn get_mut(&mut self) -> &mut [u8] {
        self.pem.as_mut()
    }

    /// Consumes `self`, returning the underlying certificate
    pub fn into_inner(self) -> Vec<u8> {
        self.pem
    }
}

impl From<Vec<u8>> for Certificate {
    fn from(pem: Vec<u8>) -> Self {
        Self { pem }
    }
}

impl AsRef<[u8]> for Certificate {
    fn as_ref(&self) -> &[u8] {
        self.pem.as_ref()
    }
}

impl AsMut<[u8]> for Certificate {
    fn as_mut(&mut self) -> &mut [u8] {
        self.pem.as_mut()
    }
}

impl Identity {
    /// Parse a PEM encoded certificate and private key.
    ///
    /// The provided cert must contain at least one PEM encoded certificate.
    pub fn from_pem(cert: impl AsRef<[u8]>, key: impl AsRef<[u8]>) -> Self {
        let cert = Certificate::from_pem(cert);
        let key = key.as_ref().into();
        Self { cert, key }
    }

    /// Wrap a PEM-encoded certificate and private key without copying.
    ///
    /// The provided `cert` must contain at least one PEM-encoded certificate.
    pub fn from_pem_vec(cert: Vec<u8>, key: Vec<u8>) -> Self {
        Self {
            cert: Certificate::from(cert),
            key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certificate_from_vec_takes_ownership_without_copying() {
        let pem = b"cert-pem".to_vec();
        let ptr = pem.as_ptr();

        let cert = Certificate::from(pem);
        assert_eq!(cert.get_ref(), b"cert-pem");
        assert_eq!(cert.get_ref().as_ptr(), ptr);
        let inner = cert.into_inner();
        assert_eq!(inner.as_ptr(), ptr);
    }

    #[test]
    fn identity_from_pem_vec_takes_ownership_without_copying() {
        let cert = b"cert-pem".to_vec();
        let key = b"key-pem".to_vec();
        let cert_ptr = cert.as_ptr();
        let key_ptr = key.as_ptr();

        let identity = Identity::from_pem_vec(cert, key);
        assert_eq!(identity.cert.get_ref(), b"cert-pem");
        assert_eq!(identity.key, b"key-pem");
        assert_eq!(identity.cert.get_ref().as_ptr(), cert_ptr);
        assert_eq!(identity.key.as_ptr(), key_ptr);
    }
}
