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

use std::str::FromStr;

use grpc::client::CallOptions;
use grpc::client::InvokeOnce;
use grpc::client::RequestHeaders;
use grpc::client::interceptor::Intercept;
use grpc::metadata::AsciiMetadataKey as GrpcMetadataKey;
use grpc::metadata::AsciiMetadataValue as GrpcMetadataValue;
use tonic::Request;
use tonic::Status;
use tonic::async_trait;
use tonic::metadata::AsciiMetadataKey as TonicMetadataKey;
use tonic::metadata::AsciiMetadataValue as TonicMetadataValue;
use tonic::service::Interceptor;

use crate::TestAssertion;

#[async_trait]
pub trait InteropTest: Send {
    async fn empty_unary(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn large_unary(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn client_streaming(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn server_streaming(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn ping_pong(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn empty_stream(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn status_code_and_message(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn special_status_message(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn unimplemented_method(&mut self, assertions: &mut Vec<TestAssertion>);

    async fn custom_metadata(&mut self, assertions: &mut Vec<TestAssertion>);
}

#[async_trait]
pub trait InteropTestUnimplemented: Send {
    async fn unimplemented_service(&mut self, assertions: &mut Vec<TestAssertion>);
}

// Parses `--additional_metadata` as gRPC's interop spec
// (doc/interop-test-descriptions.md in grpc/grpc) defines it: `key:value`
// pairs separated by `;`. Each key ends at its first `:`, so values may
// contain colons but not semicolons. Keys are lowercased, as the C++, Go and
// Java interop clients do. Binary (`-bin`) keys are rejected, as the spec
// requires.
fn parse_additional_metadata(raw: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs = Vec::new();
    for pair in raw.split(';').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once(':').ok_or_else(|| {
            format!("invalid --additional_metadata entry (expected key:value): {pair}")
        })?;
        let key = key.to_ascii_lowercase();
        if key.ends_with("-bin") {
            return Err(format!(
                "--additional_metadata does not allow binary (-bin) keys: {key}"
            ));
        }
        pairs.push((key, value.to_string()));
    }
    Ok(pairs)
}

/// Attaches `--additional_metadata` to every RPC made with tonic
/// (`--codec=prost`).
#[derive(Clone)]
pub struct MetadataInterceptor {
    metadata: Vec<(TonicMetadataKey, TonicMetadataValue)>,
}

impl MetadataInterceptor {
    /// Builds the interceptor from the `--additional_metadata` flag value.
    /// Fails on entries that gRPC's interop spec doesn't allow.
    pub fn new(additional_metadata: Option<&str>) -> Result<Self, String> {
        let mut metadata = Vec::new();
        for (key, value) in parse_additional_metadata(additional_metadata.unwrap_or_default())? {
            let parsed_key = TonicMetadataKey::from_str(&key)
                .map_err(|e| format!("invalid --additional_metadata key '{key}': {e}"))?;
            let parsed_value = TonicMetadataValue::from_str(&value)
                .map_err(|e| format!("invalid --additional_metadata value for '{key}': {e}"))?;
            metadata.push((parsed_key, parsed_value));
        }
        Ok(Self { metadata })
    }
}

impl Interceptor for MetadataInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        for (key, value) in &self.metadata {
            request.metadata_mut().append(key.clone(), value.clone());
        }
        Ok(request)
    }
}

/// Attaches `--additional_metadata` to every RPC made with the `grpc` crate
/// (`--codec=protobuf`).
#[derive(Clone)]
pub struct GrpcMetadataInterceptor {
    metadata: Vec<(GrpcMetadataKey, GrpcMetadataValue)>,
}

impl GrpcMetadataInterceptor {
    /// Builds the interceptor from the `--additional_metadata` flag value.
    /// Fails on entries that gRPC's interop spec doesn't allow.
    pub fn new(additional_metadata: Option<&str>) -> Result<Self, String> {
        let mut metadata = Vec::new();
        for (key, value) in parse_additional_metadata(additional_metadata.unwrap_or_default())? {
            let parsed_key = GrpcMetadataKey::from_str(&key)
                .map_err(|e| format!("invalid --additional_metadata key '{key}': {e}"))?;
            let parsed_value = GrpcMetadataValue::from_str(&value)
                .map_err(|e| format!("invalid --additional_metadata value for '{key}': {e}"))?;
            metadata.push((parsed_key, parsed_value));
        }
        Ok(Self { metadata })
    }
}

impl<I: InvokeOnce> Intercept<I> for GrpcMetadataInterceptor {
    type SendStream = I::SendStream;
    type RecvStream = I::RecvStream;

    async fn intercept(
        &self,
        mut headers: RequestHeaders,
        options: CallOptions,
        next: I,
    ) -> (Self::SendStream, Self::RecvStream) {
        let request_metadata = headers.metadata_mut();
        for (key, value) in &self.metadata {
            request_metadata.append(key, value.clone());
        }
        next.invoke_once(headers, options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Runs the tonic interceptor on an empty request and returns the value it
    // attached under `key`.
    fn attached(additional_metadata: &str, key: &str) -> Option<String> {
        let mut interceptor = MetadataInterceptor::new(Some(additional_metadata)).unwrap();
        let request = interceptor.call(Request::new(())).unwrap();
        request
            .metadata()
            .get(key)
            .map(|value| value.to_str().unwrap().to_string())
    }

    #[test]
    fn additional_metadata_follows_spec_examples() {
        let flag = "abc-key:abc-value;foo-key:foo-value";
        assert_eq!(attached(flag, "abc-key").as_deref(), Some("abc-value"));
        assert_eq!(attached(flag, "foo-key").as_deref(), Some("foo-value"));
        assert!(GrpcMetadataInterceptor::new(Some(flag)).is_ok());

        // A key ends at its first colon, so values may contain colons.
        let flag = "abc-key:abc:value;foo-key:foo:value";
        assert_eq!(attached(flag, "abc-key").as_deref(), Some("abc:value"));
        assert_eq!(attached(flag, "foo-key").as_deref(), Some("foo:value"));
        assert!(GrpcMetadataInterceptor::new(Some(flag)).is_ok());
    }

    #[test]
    fn additional_metadata_lowercases_keys() {
        assert_eq!(
            attached("X-User-IP:1.2.3.4", "x-user-ip").as_deref(),
            Some("1.2.3.4")
        );
        assert!(GrpcMetadataInterceptor::new(Some("X-User-IP:1.2.3.4")).is_ok());
    }

    #[test]
    fn additional_metadata_rejects_binary_keys_and_missing_values() {
        for flag in ["trace-bin:AAAA", "no-separator"] {
            assert!(MetadataInterceptor::new(Some(flag)).is_err(), "{flag}");
            assert!(GrpcMetadataInterceptor::new(Some(flag)).is_err(), "{flag}");
        }
    }

    #[test]
    fn additional_metadata_absent_or_empty_attaches_nothing() {
        for flag in [None, Some("")] {
            let mut interceptor = MetadataInterceptor::new(flag).unwrap();
            let request = interceptor.call(Request::new(())).unwrap();
            assert!(request.metadata().is_empty());
        }
    }
}
