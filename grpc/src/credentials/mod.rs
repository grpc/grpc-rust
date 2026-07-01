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

//! Authentication and security credentials (e.g. TLS and OAuth2).
//!
//! This module provides traits and types for handling credentials in gRPC,
//! including channel credentials (for securing connections) and call
//! credentials (for authenticating individual RPCs).
//!
//! # Key Concepts
//!
//! - **[`ChannelCredentials`]:** Trait for client-side transport security
//!   (e.g., TLS). May also include [`CallCredentials`] by using
//!   [`CompositeChannelCredentials`].
//! - **[`ServerCredentials`]:** Trait for server-side transport security.

pub mod call;
pub(crate) mod client;
pub(crate) mod dyn_wrapper;
mod local;
#[cfg(feature = "tls-rustls")]
pub mod rustls;
pub(crate) mod server;

use std::sync::Arc;

pub use client::CompositeChannelCredentials;
pub use local::LocalChannelCredentials;
pub use local::LocalServerCredentials;
use tonic::async_trait;

use crate::credentials::call::CallCredentials;
use crate::credentials::client::ClientHandshakeInfo;
use crate::credentials::client::HandshakeOutput;
use crate::credentials::common::Authority;
use crate::private;
use crate::rt::BoxEndpoint;
use crate::rt::GrpcEndpoint;
use crate::rt::GrpcRuntime;

/// Client-side trait for all live gRPC wire protocols and supported transport
/// security protocols (e.g., TLS, ALTS).
///
/// Also includes the ability to attach [`CallCredentials`] when used with the
/// [`CompositeChannelCredentials`].
#[async_trait]
pub trait ChannelCredentials: Send + Sync + 'static {
    /// Provides the ProtocolInfo of these credentials.
    fn info(&self) -> &ProtocolInfo;

    /// Returns call credentials to be used for all RPCs made on a connection.
    #[doc(hidden)]
    fn get_call_credentials(&self, token: private::Internal) -> Option<&Arc<dyn CallCredentials>>;

    /// Performs the client-side authentication handshake on a raw endpoint.
    ///
    /// This method wraps the provided `source` endpoint with the security protocol
    /// (e.g., TLS) and returns the authenticated endpoint along with its
    /// security details.
    ///
    /// # Arguments
    ///
    /// * `authority` - The `:authority` header value to be used when creating
    ///   new streams.
    ///   **Important:** Implementations must use this value as the server name
    ///   (e.g., for SNI) during the handshake.
    /// * `source` - The raw connection handle.
    /// * `info` - Additional context passed from the resolver or load balancer.
    #[doc(hidden)]
    async fn connect(
        &self,
        authority: &Authority,
        source: BoxEndpoint,
        info: &ClientHandshakeInfo,
        runtime: &GrpcRuntime,
        token: private::Internal,
    ) -> Result<HandshakeOutput, String>;
}

/// Server-side trait for all live gRPC wire protocols and supported
/// transport security protocols (e.g., TLS, ALTS).
#[trait_variant::make(Send)]
pub trait ServerCredentials: Sync + 'static {
    #[doc(hidden)]
    type Output<I>;

    /// Provides the ProtocolInfo of these credentials.
    fn info(&self) -> &ProtocolInfo;

    /// Performs the server-side authentication handshake.
    ///
    /// This method wraps the incoming raw `source` connection with the configured
    /// security protocol (e.g., TLS).
    #[doc(hidden)]
    async fn accept<Input: GrpcEndpoint>(
        &self,
        source: Input,
        runtime: GrpcRuntime,
        token: private::Internal,
    ) -> Result<server::HandshakeOutput<Self::Output<Input>>, String>;
}

/// Defines the level of protection provided by an established connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum SecurityLevel {
    /// The connection is insecure; no protection is applied.
    NoSecurity,
    /// The connection guarantees data integrity (tamper-proofing) but not
    /// privacy.
    ///
    /// Payloads are visible to observers but cannot be modified without
    /// detection.
    IntegrityOnly,
    /// The connection guarantees both privacy (confidentiality) and data
    /// integrity.
    ///
    /// This is the standard level for secure transports like TLS.
    PrivacyAndIntegrity,
}

pub(crate) mod common {
    /// Represents the value passed as the `:authority` pseudo-header, typically
    /// in the form `host:port`.
    #[derive(Clone, PartialEq, Debug)]
    pub struct Authority {
        host: String,
        port: Option<u16>,
    }

    impl Authority {
        pub fn new(host: impl Into<String>, port: Option<u16>) -> Self {
            Self {
                host: host.into(),
                port,
            }
        }

        /// Parses the host and port from a string. When the input can not be parsed
        /// as (host, port) pair, it returns the entire input as the host.
        pub(crate) fn from_host_port_str(host_and_port: &str) -> Self {
            // Handle bracketed IPv6 addresses (e.g., "[::1]:80").
            if let Some(stripped) = host_and_port.strip_prefix('[')
                && let Some((host, port_str)) = stripped.split_once("]:")
                && let Ok(port) = port_str.parse::<u16>()
            {
                return Self::new(host, Some(port));
            }
            // Handle unbracketed addresses (IPv4 or hostnames, e.g.,
            // "localhost:8080").
            if let Some((host, port_str)) = host_and_port.rsplit_once(':')
                && !host.contains(':')
                && let Ok(port) = port_str.parse::<u16>()
            {
                return Self::new(host, Some(port));
            }
            Self::new(host_and_port.to_string(), None)
        }

        pub fn host(&self) -> &str {
            &self.host
        }

        pub fn port(&self) -> Option<u16> {
            self.port
        }

        pub fn set_port(&mut self, port: Option<u16>) {
            self.port = port;
        }

        pub fn host_port_string(&self) -> String {
            let host_str = &self.host;
            match self.port() {
                None => host_str.to_string(),
                // Add [] for IPv6 addresses.
                Some(port) if host_str.contains(':') => {
                    format!("[{}]:{}", host_str, port)
                }
                Some(port) => format!("{}:{}", host_str, port),
            }
        }
    }
}

/// Contains information about a [`ChannelCredentials`] or
/// [`ServerCredentials`].
pub struct ProtocolInfo {
    security_protocol: &'static str,
}

impl ProtocolInfo {
    pub(crate) const fn new(security_protocol: &'static str) -> Self {
        Self { security_protocol }
    }

    /// Returns the security protocol name currently in use, e.g. "tls".
    pub fn security_protocol(&self) -> &'static str {
        self.security_protocol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_host_port_str() {
        let authority = Authority::new("localhost", None);
        assert_eq!(&authority.host_port_string(), "localhost");

        let authority = Authority::new("localhost", Some(443));
        assert_eq!(&authority.host_port_string(), "localhost:443");

        let authority = Authority::new("::1", Some(50051));
        assert_eq!(&authority.host_port_string(), "[::1]:50051");

        let authority = Authority::new("::1", None);
        assert_eq!(&authority.host_port_string(), "::1");
    }

    #[test]
    fn test_parse_authority() {
        struct TestCase {
            input: &'static str,
            expected: Authority,
        }

        let cases = [
            TestCase {
                input: "localhost:http",
                expected: Authority::new("localhost:http", None),
            },
            TestCase {
                input: "localhost:80",
                expected: Authority::new("localhost", Some(80)),
            },
            // host name with zone identifier.
            TestCase {
                input: "localhost%lo0:80",
                expected: Authority::new("localhost%lo0", Some(80)),
            },
            TestCase {
                input: "localhost%lo0:http",
                expected: Authority::new("localhost%lo0:http", None),
            },
            TestCase {
                input: "[localhost%lo0]:http",
                expected: Authority::new("[localhost%lo0]:http", None),
            },
            TestCase {
                input: "[localhost%lo0]:80",
                expected: Authority::new("localhost%lo0", Some(80)),
            },
            // IP literal
            TestCase {
                input: "127.0.0.1:http",
                expected: Authority::new("127.0.0.1:http", None),
            },
            TestCase {
                input: "127.0.0.1:80",
                expected: Authority::new("127.0.0.1", Some(80)),
            },
            TestCase {
                input: "[::1]:http",
                expected: Authority::new("[::1]:http", None),
            },
            TestCase {
                input: "[::1]:80",
                expected: Authority::new("::1", Some(80)),
            },
            // IP literal with zone identifier.
            TestCase {
                input: "[::1%lo0]:http",
                expected: Authority::new("[::1%lo0]:http", None),
            },
            TestCase {
                input: "[::1%lo0]:80",
                expected: Authority::new("::1%lo0", Some(80)),
            },
            TestCase {
                input: ":http",
                expected: Authority::new(":http", None),
            },
            TestCase {
                input: ":80",
                expected: Authority::new("", Some(80)),
            },
            TestCase {
                input: "grpc.io:",
                expected: Authority::new("grpc.io:", None),
            },
            TestCase {
                input: "127.0.0.1:",
                expected: Authority::new("127.0.0.1:", None),
            },
            TestCase {
                input: "[::1]:",
                expected: Authority::new("[::1]:", None),
            },
            TestCase {
                input: "grpc.io:https%foo",
                expected: Authority::new("grpc.io:https%foo", None),
            },
            TestCase {
                input: "grpc.io",
                expected: Authority::new("grpc.io", None),
            },
            TestCase {
                input: "127.0.0.1",
                expected: Authority::new("127.0.0.1", None),
            },
            TestCase {
                input: "[::1]",
                expected: Authority::new("[::1]", None),
            },
            TestCase {
                input: "[fe80::1%lo0]",
                expected: Authority::new("[fe80::1%lo0]", None),
            },
            TestCase {
                input: "[localhost%lo0]",
                expected: Authority::new("[localhost%lo0]", None),
            },
            TestCase {
                input: "localhost%lo0",
                expected: Authority::new("localhost%lo0", None),
            },
            TestCase {
                input: "::1",
                expected: Authority::new("::1", None),
            },
            TestCase {
                input: "fe80::1%lo0",
                expected: Authority::new("fe80::1%lo0", None),
            },
            TestCase {
                input: "fe80::1%lo0:80",
                expected: Authority::new("fe80::1%lo0:80", None),
            },
            TestCase {
                input: "[foo:bar]",
                expected: Authority::new("[foo:bar]", None),
            },
            TestCase {
                input: "[foo:bar]baz",
                expected: Authority::new("[foo:bar]baz", None),
            },
            TestCase {
                input: "[foo]bar:baz",
                expected: Authority::new("[foo]bar:baz", None),
            },
            TestCase {
                input: "[foo]:[bar]:baz",
                expected: Authority::new("[foo]:[bar]:baz", None),
            },
            TestCase {
                input: "[foo]:[bar]baz",
                expected: Authority::new("[foo]:[bar]baz", None),
            },
            TestCase {
                input: "foo[bar]:baz",
                expected: Authority::new("foo[bar]:baz", None),
            },
            TestCase {
                input: "foo]bar:baz",
                expected: Authority::new("foo]bar:baz", None),
            },
        ];

        for TestCase { input, expected } in cases {
            let auth = Authority::from_host_port_str(input);
            assert_eq!(auth, expected, "authority mismatch for {}", input);
        }
    }
}
