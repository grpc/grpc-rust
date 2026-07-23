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

#![allow(missing_docs)]

use prost::Message;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio_stream::{StreamExt, wrappers::TcpListenerStream};
use tonic::{Request, transport::Server};
use tonic_reflection::{
    pb::v1::{
        FILE_DESCRIPTOR_SET, ServerReflectionRequest, ServiceResponse,
        server_reflection_client::ServerReflectionClient,
        server_reflection_request::MessageRequest, server_reflection_response::MessageResponse,
    },
    server::Builder,
};

pub(crate) fn get_encoded_reflection_service_fd() -> Vec<u8> {
    let mut expected = Vec::new();
    prost_types::FileDescriptorSet::decode(FILE_DESCRIPTOR_SET)
        .expect("decode reflection service file descriptor set")
        .file[0]
        .encode(&mut expected)
        .expect("encode reflection service file descriptor");
    expected
}

#[tokio::test]
async fn test_list_services() {
    let response = make_test_reflection_request(ServerReflectionRequest {
        host: "".to_string(),
        message_request: Some(MessageRequest::ListServices(String::new())),
    })
    .await;

    if let MessageResponse::ListServicesResponse(services) = response {
        assert_eq!(
            services.service,
            vec![ServiceResponse {
                name: String::from("grpc.reflection.v1.ServerReflection")
            }]
        );
    } else {
        panic!("Expected a ListServicesResponse variant");
    }
}

#[tokio::test]
async fn test_file_by_filename() {
    let response = make_test_reflection_request(ServerReflectionRequest {
        host: "".to_string(),
        message_request: Some(MessageRequest::FileByFilename(String::from(
            "reflection_v1.proto",
        ))),
    })
    .await;

    if let MessageResponse::FileDescriptorResponse(descriptor) = response {
        let file_descriptor_proto = descriptor
            .file_descriptor_proto
            .first()
            .expect("descriptor");
        assert_eq!(
            file_descriptor_proto.as_ref(),
            get_encoded_reflection_service_fd()
        );
    } else {
        panic!("Expected a FileDescriptorResponse variant");
    }
}

#[tokio::test]
async fn test_file_containing_symbol() {
    let response = make_test_reflection_request(ServerReflectionRequest {
        host: "".to_string(),
        message_request: Some(MessageRequest::FileContainingSymbol(String::from(
            "grpc.reflection.v1.ServerReflection",
        ))),
    })
    .await;

    if let MessageResponse::FileDescriptorResponse(descriptor) = response {
        let file_descriptor_proto = descriptor
            .file_descriptor_proto
            .first()
            .expect("descriptor");
        assert_eq!(
            file_descriptor_proto.as_ref(),
            get_encoded_reflection_service_fd()
        );
    } else {
        panic!("Expected a FileDescriptorResponse variant");
    }
}

/// Builds the raw bytes of a `FileDescriptorProto` that carries a field prost
/// does not know about, standing in for a custom option / extension such as
/// `google.api.http` (field `72295728`). prost drops unknown fields on decode,
/// so a decode/re-encode round-trip strips these bytes.
fn file_descriptor_with_custom_option() -> Vec<u8> {
    use prost::encoding::{WireType, encode_key, encode_varint};

    let fd = prost_types::FileDescriptorProto {
        name: Some("custom_option.proto".to_string()),
        package: Some("test.custom".to_string()),
        message_type: vec![prost_types::DescriptorProto {
            name: Some("Msg".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut raw = fd.encode_to_vec();

    // Append an extension-range field number that `prost_types` has no field for.
    encode_key(72295728, WireType::LengthDelimited, &mut raw);
    let payload = b"custom-option-value";
    encode_varint(payload.len() as u64, &mut raw);
    raw.extend_from_slice(payload);

    raw
}

/// Wraps encoded `FileDescriptorProto` bytes into an encoded `FileDescriptorSet`
/// (`repeated FileDescriptorProto file = 1`) verbatim, without a prost round-trip.
fn wrap_in_file_descriptor_set(file_bytes: &[u8]) -> Vec<u8> {
    use prost::encoding::{WireType, encode_key, encode_varint};

    let mut set = Vec::new();
    encode_key(1, WireType::LengthDelimited, &mut set);
    encode_varint(file_bytes.len() as u64, &mut set);
    set.extend_from_slice(file_bytes);
    set
}

#[tokio::test]
async fn test_custom_options_are_preserved() {
    let raw_file = file_descriptor_with_custom_option();

    // Sanity check the fixture: a naive prost decode/re-encode round-trip (the
    // behaviour this test guards against) strips the unknown field.
    let round_tripped = prost_types::FileDescriptorProto::decode(raw_file.as_slice())
        .expect("decode file descriptor")
        .encode_to_vec();
    assert_ne!(
        round_tripped, raw_file,
        "fixture invalid: the unknown field should be dropped by a prost round-trip"
    );

    let encoded_set = wrap_in_file_descriptor_set(&raw_file);

    // The bytes served for both a file lookup and a symbol lookup must be the
    // original bytes, with the custom option intact.
    for message_request in [
        MessageRequest::FileByFilename("custom_option.proto".to_string()),
        MessageRequest::FileContainingSymbol("test.custom.Msg".to_string()),
    ] {
        let response = make_test_reflection_request_for(
            encoded_set.clone(),
            None,
            ServerReflectionRequest {
                host: "".to_string(),
                message_request: Some(message_request),
            },
        )
        .await;

        let MessageResponse::FileDescriptorResponse(descriptor) = response else {
            panic!("Expected a FileDescriptorResponse variant");
        };
        let served = descriptor
            .file_descriptor_proto
            .first()
            .expect("descriptor");
        assert_eq!(
            served, &raw_file,
            "reflection must serve the original bytes, preserving custom options"
        );
    }
}

#[tokio::test]
async fn test_encoded_bytes_win_over_decoded_duplicate() {
    // The same file is registered in both forms: as raw bytes carrying a custom
    // option, and as an already-decoded `FileDescriptorSet` (which cannot carry
    // the option). The lossless raw bytes must take precedence.
    let raw_file = file_descriptor_with_custom_option();
    let encoded_set = wrap_in_file_descriptor_set(&raw_file);

    let decoded_dup = prost_types::FileDescriptorSet {
        file: vec![
            prost_types::FileDescriptorProto::decode(raw_file.as_slice())
                .expect("decode file descriptor"),
        ],
    };

    let response = make_test_reflection_request_for(
        encoded_set,
        Some(decoded_dup),
        ServerReflectionRequest {
            host: "".to_string(),
            message_request: Some(MessageRequest::FileByFilename(
                "custom_option.proto".to_string(),
            )),
        },
    )
    .await;

    let MessageResponse::FileDescriptorResponse(descriptor) = response else {
        panic!("Expected a FileDescriptorResponse variant");
    };
    let served = descriptor
        .file_descriptor_proto
        .first()
        .expect("descriptor");
    assert_eq!(
        served, &raw_file,
        "the option-preserving encoded bytes must win over a decoded duplicate"
    );
}

async fn make_test_reflection_request(request: ServerReflectionRequest) -> MessageResponse {
    make_test_reflection_request_for(FILE_DESCRIPTOR_SET.to_vec(), None, request).await
}

async fn make_test_reflection_request_for(
    encoded_fds: Vec<u8>,
    decoded_fds: Option<prost_types::FileDescriptorSet>,
    request: ServerReflectionRequest,
) -> MessageResponse {
    // Run a test server
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let addr: SocketAddr = "127.0.0.1:0".parse().expect("SocketAddr parse");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    let local_addr = format!("http://{}", listener.local_addr().expect("local address"));
    let jh = tokio::spawn(async move {
        let mut builder = Builder::configure().register_encoded_file_descriptor_set(&encoded_fds);
        if let Some(decoded) = decoded_fds {
            builder = builder.register_file_descriptor_set(decoded);
        }
        let service = builder.build_v1().unwrap();

        Server::builder()
            .add_service(service)
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                drop(shutdown_rx.await)
            })
            .await
            .unwrap();
    });

    // Give the test server a few ms to become available
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Construct client and send request, extract response
    let conn = tonic::transport::Endpoint::new(local_addr)
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut client = ServerReflectionClient::new(conn);

    let request = Request::new(tokio_stream::once(request));
    let mut inbound = client
        .server_reflection_info(request)
        .await
        .expect("request")
        .into_inner();

    let response = inbound
        .next()
        .await
        .expect("steamed response")
        .expect("successful response")
        .message_response
        .expect("some MessageResponse");

    // We only expect one response per request
    assert!(inbound.next().await.is_none());

    // Shut down test server
    shutdown_tx.send(()).expect("send shutdown");
    jh.await.expect("server shutdown");

    response
}
