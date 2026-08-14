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

//! Token-level assertions for `CodeGenBuilder::local` output (#2790).

use proc_macro2::TokenStream;
use quote::quote;
use tonic_build::{CodeGenBuilder, Method, Service};

struct FixtureMethod {
    name: &'static str,
    identifier: &'static str,
    client_streaming: bool,
    server_streaming: bool,
}

impl Method for FixtureMethod {
    type Comment = String;

    fn name(&self) -> &str {
        self.name
    }
    fn identifier(&self) -> &str {
        self.identifier
    }
    fn codec_path(&self) -> &str {
        "tonic_prost::ProstCodec"
    }
    fn client_streaming(&self) -> bool {
        self.client_streaming
    }
    fn server_streaming(&self) -> bool {
        self.server_streaming
    }
    fn comment(&self) -> &[Self::Comment] {
        &[]
    }
    fn request_response_name(
        &self,
        _proto_path: &str,
        _compile_well_known_types: bool,
    ) -> (TokenStream, TokenStream) {
        (quote!(super::EchoRequest), quote!(super::EchoResponse))
    }
}

struct FixtureService {
    methods: Vec<FixtureMethod>,
}

impl Service for FixtureService {
    type Comment = String;
    type Method = FixtureMethod;

    fn name(&self) -> &str {
        "Echo"
    }
    fn package(&self) -> &str {
        "test"
    }
    fn identifier(&self) -> &str {
        "Echo"
    }
    fn methods(&self) -> &[Self::Method] {
        &self.methods
    }
    fn comment(&self) -> &[Self::Comment] {
        &[]
    }
}

fn fixture() -> FixtureService {
    FixtureService {
        methods: vec![
            FixtureMethod {
                name: "unary_echo",
                identifier: "UnaryEcho",
                client_streaming: false,
                server_streaming: false,
            },
            FixtureMethod {
                name: "server_streaming_echo",
                identifier: "ServerStreamingEcho",
                client_streaming: false,
                server_streaming: true,
            },
            FixtureMethod {
                name: "client_streaming_echo",
                identifier: "ClientStreamingEcho",
                client_streaming: true,
                server_streaming: false,
            },
            FixtureMethod {
                name: "bidi_echo",
                identifier: "BidiEcho",
                client_streaming: true,
                server_streaming: true,
            },
        ],
    }
}

/// Generated tokens with all spaces removed, for spacing-insensitive matching.
fn normalized(tokens: TokenStream) -> String {
    tokens.to_string().replace(' ', "")
}

fn gen_server(local: bool) -> String {
    let mut builder = CodeGenBuilder::new();
    builder.local(local);
    normalized(builder.generate_server(&fixture(), "super"))
}

fn gen_client(local: bool) -> String {
    let mut builder = CodeGenBuilder::new();
    builder.local(local);
    normalized(builder.generate_client(&fixture(), "super"))
}

#[test]
fn local_server_tokens() {
    let out = gen_server(true);

    assert!(
        out.contains("async_trait(?Send)"),
        "expected ?Send async_trait"
    );
    assert!(
        out.contains("pubtraitEcho:'static"),
        "expected 'static-only supertrait"
    );
    assert!(
        !out.contains("std::marker::Send+std::marker::Sync"),
        "Send + Sync supertrait must be gone"
    );
    assert!(out.contains("LocalBoxFuture"), "expected LocalBoxFuture");
    assert!(
        out.contains("tonic::local::server::Grpc"),
        "expected local server Grpc"
    );
    assert!(
        out.contains("tonic::local::server::ClientStreamingService")
            && out.contains("tonic::local::server::StreamingService"),
        "expected local streaming service traits"
    );
    assert!(
        out.contains("tonic::local::codec::Streaming"),
        "expected local Streaming in request types"
    );
    assert!(
        out.contains("tonic::local::body::Body"),
        "expected local body type"
    );
    assert!(out.contains("Rc<T>"), "expected Rc-held handler");
    assert!(out.contains("from_rc"), "expected from_rc constructor");
    assert!(!out.contains("Arc"), "no Arc may remain in local mode");
    assert_eq!(
        out.matches("BoxFuture<").count(),
        out.matches("LocalBoxFuture<").count(),
        "every BoxFuture must be LocalBoxFuture in local mode"
    );
}

#[test]
fn local_server_default_stubs_use_local_box_stream() {
    let mut builder = CodeGenBuilder::new();
    builder.local(true).generate_default_stubs(true);
    let out = normalized(builder.generate_server(&fixture(), "super"));
    assert!(out.contains("LocalBoxStream"), "expected LocalBoxStream");
    assert_eq!(
        out.matches("BoxStream<").count(),
        out.matches("LocalBoxStream<").count(),
        "every BoxStream must be LocalBoxStream in local mode"
    );
}

#[test]
fn default_server_tokens_unchanged() {
    let out = gen_server(false);

    assert!(!out.contains("async_trait(?Send)"));
    assert!(out.contains("async_trait"));
    assert!(out.contains("std::marker::Send+std::marker::Sync+'static"));
    assert!(out.contains("Arc<T>"));
    assert!(out.contains("from_arc"));
    assert!(!out.contains("LocalBoxFuture"));
    assert!(!out.contains("tonic::local::"));
}

#[test]
fn local_client_tokens() {
    let out = gen_client(true);

    assert!(
        out.contains("tonic::local::client::Grpc"),
        "expected local client Grpc"
    );
    assert!(
        out.contains("tonic::client::GrpcService<tonic::local::body::Body>"),
        "expected GrpcService over local body"
    );
    assert!(
        out.contains("tonic::local::request::IntoStreamingRequest"),
        "expected local IntoStreamingRequest"
    );
    assert!(
        out.contains("tonic::local::codec::Streaming"),
        "expected local Streaming responses"
    );
    assert!(
        !out.contains("tonic::body::Body"),
        "no Send body type may remain"
    );
    assert!(
        !out.contains("std::marker::Send+'static"),
        "ResponseBody Send bound must be gone"
    );
    assert!(
        !out.contains("pubasyncfnconnect"),
        "connect() must not be generated in local mode"
    );
}

#[test]
fn default_client_tokens_unchanged() {
    let out = gen_client(false);

    assert!(out.contains("tonic::client::Grpc"));
    assert!(out.contains("tonic::body::Body"));
    assert!(out.contains("tonic::IntoStreamingRequest"));
    assert!(!out.contains("tonic::local::"));
}

#[test]
#[should_panic(expected = "use_arc_self is not supported with local codegen")]
fn local_rejects_use_arc_self() {
    let mut builder = CodeGenBuilder::new();
    builder.local(true).use_arc_self(true);
    builder.generate_server(&fixture(), "super");
}
