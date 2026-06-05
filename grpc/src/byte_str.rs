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
use std::ops::Deref;

use bytes::Bytes;

/// A cheaply cloneable and sliceable chunk of contiguous memory.
///
/// The bytes held by `ByteStr` are arbitrary and may not be valid UTF-8.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ByteStr {
    bytes: Bytes,
}

impl ByteStr {
    /// Strips a prefix, returning a new zero-copy ByteStr.
    #[inline]
    pub(crate) fn strip_prefix(&self, prefix: &[u8]) -> Option<ByteStr> {
        if self.starts_with(prefix) {
            Some(ByteStr {
                bytes: self.bytes.slice(prefix.len()..),
            })
        } else {
            None
        }
    }
}

impl Deref for ByteStr {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<String> for ByteStr {
    #[inline]
    fn from(src: String) -> ByteStr {
        ByteStr {
            bytes: Bytes::from(src),
        }
    }
}

impl<'a> TryFrom<&'a ByteStr> for &'a str {
    type Error = std::str::Utf8Error;

    #[inline]
    fn try_from(value: &'a ByteStr) -> Result<Self, Self::Error> {
        std::str::from_utf8(value)
    }
}

impl TryFrom<ByteStr> for String {
    type Error = std::str::Utf8Error;

    #[inline]
    fn try_from(value: ByteStr) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(&value)?;
        Ok(s.to_owned())
    }
}

impl PartialEq<str> for ByteStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.bytes == other.as_bytes()
    }
}

impl<'a> PartialEq<&'a str> for ByteStr {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.bytes == other.as_bytes()
    }
}

impl PartialEq<ByteStr> for str {
    #[inline]
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_bytes() == other.bytes
    }
}

impl PartialEq<ByteStr> for &str {
    #[inline]
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_bytes() == other.bytes
    }
}

impl FromIterator<u8> for ByteStr {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        ByteStr {
            bytes: Bytes::from_iter(iter),
        }
    }
}

impl From<&'static str> for ByteStr {
    #[inline]
    fn from(src: &'static str) -> ByteStr {
        ByteStr {
            bytes: Bytes::from_static(src.as_bytes()),
        }
    }
}
