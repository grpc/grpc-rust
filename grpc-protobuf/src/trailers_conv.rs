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

use grpc::core::Trailers;
use protobuf::Parse;
use protobuf::Serialize;
use protobuf_well_known_types::Any;

use crate::status::*;

#[allow(dead_code)]
mod google_rpc {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/google/rpc/generated.rs"
    ));
}

/// Converts grpc-status-details-bin from the trailer's metadata. If the rpc status code doesn't
/// match the result, the status will become an INTERNAL error.
pub(crate) fn status_from_trailers(mut t: Trailers) -> Status {
    let bin_val = t.metadata_mut().remove_bin("grpc-status-details-bin");
    match t.into_status() {
        Ok(()) => {
            if bin_val.is_some() {
                return Err(StatusError::new(
                    StatusCodeError::Internal,
                    "grpc-status-details-bin metadata cannot be present when gRPC status code is OK",
                ));
            }
            Ok(())
        }
        Err(grpc_err) => {
            let (code, message) = grpc_err.into_parts();
            let expected_code = StatusCodeError::from(code as i32);
            match bin_val {
                None => Err(StatusError::new(expected_code, message)),
                Some(meta_val) => {
                    let rpc_status_err = parse_rpc_status(meta_val.as_bytes())?;
                    if rpc_status_err.code() != expected_code {
                        return Err(StatusError::new(
                            StatusCodeError::Internal,
                            format!(
                                "RPC status code mismatch: gRPC code {:?}, google.rpc.Code {:?}",
                                expected_code,
                                rpc_status_err.code()
                            ),
                        ));
                    }
                    Err(rpc_status_err)
                }
            }
        }
    }
}

fn parse_rpc_status(buf: &[u8]) -> StatusOr<StatusError> {
    let rpc_status = google_rpc::Status::parse(buf).map_err(|e| {
        StatusError::new(
            StatusCodeError::Internal,
            format!("Failed to parse grpc-status-details-bin: {}", e),
        )
    })?;
    let code_i32 = rpc_status.code();
    if code_i32 == 0 {
        return Err(StatusError::new(
            StatusCodeError::Internal,
            "grpc-status-details-bin status code was OK, but should always be an error",
        ));
    }
    let code = StatusCodeError::from(code_i32);
    let message = rpc_status.message().to_string();
    let mut status_err = StatusError::new(code, message);

    for any in rpc_status.details() {
        status_err.set_payload(any.type_url().as_bytes(), any.value());
    }

    Ok(status_err)
}

/// Converts the status to trailers and inserts grpc-status-details-bin into the metadata.
#[allow(dead_code)]
pub(crate) fn trailers_from_status(s: Status) -> Trailers {
    match s {
        Ok(()) => Trailers::new(Ok(())),
        Err(status_err) => {
            let has_payloads = status_err.has_payloads();
            let (code, message, payloads) = status_err.into_parts();
            let mut m = grpc::metadata::MetadataMap::new();
            if has_payloads {
                let code_i32 = code as i32;
                let bytes = match encode_rpc_status(code_i32, &message, payloads) {
                    Ok(bytes) => bytes,
                    Err(err) => return Trailers::new(Err(err)),
                };
                m.insert_bin(
                    "grpc-status-details-bin",
                    bytes::Bytes::from_owner(bytes)
                        .try_into()
                        .expect("Bytes to metadata value cannot fail"),
                );
            }
            let grpc_code = grpc::StatusCodeError::from(code as i32);
            Trailers::new(Err(grpc::StatusError::new(grpc_code, message))).with_metadata(m)
        }
    }
}

fn encode_rpc_status(
    code: i32,
    message: &str,
    payloads: impl IntoIterator<Item = (Vec<u8>, Vec<u8>)>,
) -> grpc::Result<Vec<u8>> {
    let mut rpc_status = google_rpc::Status::new();
    rpc_status.set_code(code);
    rpc_status.set_message(message);

    for (type_url, payload) in payloads {
        let Ok(type_url_str) = String::from_utf8(type_url) else {
            continue;
        };
        let mut any = Any::new();
        any.set_type_url(type_url_str);
        any.set_value(payload);
        rpc_status.details_mut().push(any);
    }

    // TODO: reconsider this error handling; it will be sent to the client. But it may also never
    // trigger. Note that `e` contains no details.
    rpc_status.serialize().map_err(|e| {
        grpc::StatusError::new(
            grpc::StatusCodeError::Internal,
            format!("Failed to serialize grpc-status-details-bin: {}", e),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailers_from_status_details_copied_to_grpc_status() {
        let mut err = StatusError::new(StatusCodeError::NotFound, "not found detail");
        err.set_payload(b"type.googleapis.com/test", b"hello world");

        let trailers = trailers_from_status(Err(err));
        let grpc_status_error = trailers.status().as_ref().unwrap_err();
        assert_eq!(grpc_status_error.code(), grpc::StatusCodeError::NotFound);
        assert_eq!(grpc_status_error.message(), "not found detail");
        assert!(
            trailers
                .metadata()
                .get_bin("grpc-status-details-bin")
                .is_some()
        );
    }

    #[test]
    fn test_trailers_from_status_ok_copied_to_grpc_status() {
        let trailers = trailers_from_status(Ok(()));
        assert!(trailers.status().is_ok());
        assert!(
            trailers
                .metadata()
                .get_bin("grpc-status-details-bin")
                .is_none()
        );
    }

    #[test]
    fn test_trailers_from_status_empty_payload_skips_metadata() {
        let err = StatusError::new(
            StatusCodeError::NotFound,
            "Resource missing without details",
        );
        let status_or: Status = Err(err);

        let trailers = trailers_from_status(status_or);
        assert!(trailers.status().is_err());
        assert!(
            trailers
                .metadata()
                .get_bin("grpc-status-details-bin")
                .is_none()
        );
    }

    #[test]
    fn test_roundtrip_payload() {
        let mut og_err = StatusError::new(StatusCodeError::NotFound, "not found detail");
        og_err.set_payload(b"type.googleapis.com/foo", b"hello");
        og_err.set_payload(b"type.googleapis.com/bar", b"world");

        let trailers = trailers_from_status(Err(og_err.clone()));
        let rt_err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(rt_err.code(), og_err.code());
        assert_eq!(rt_err.message(), og_err.message());
        assert_eq!(
            rt_err.get_payload(b"type.googleapis.com/foo"),
            og_err.get_payload(b"type.googleapis.com/foo")
        );
        assert_eq!(
            rt_err.get_payload(b"type.googleapis.com/bar"),
            og_err.get_payload(b"type.googleapis.com/bar")
        );
    }

    #[test]
    fn test_roundtrip_invalid_utf8_dropped() {
        let mut og_err = StatusError::new(StatusCodeError::NotFound, "not found detail");
        og_err.set_payload(b"type.googleapis.com/foo", b"world");
        og_err.set_payload(b"type.googleapis.com/bar\x80", b"ain't gonna work");
        og_err.set_payload(b"type.googleapis.com/bar", b"hello");

        let trailers = trailers_from_status(Err(og_err.clone()));
        let rt_err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(rt_err.code(), og_err.code());
        assert_eq!(rt_err.message(), og_err.message());
        // Other payloads are preserved
        assert_eq!(
            rt_err.get_payload(b"type.googleapis.com/foo"),
            og_err.get_payload(b"type.googleapis.com/foo")
        );
        assert_eq!(
            rt_err.get_payload(b"type.googleapis.com/bar"),
            og_err.get_payload(b"type.googleapis.com/bar")
        );
        // But not the one with invalid UTF-8
        assert!(rt_err.get_payload(b"type.googleapis.com/bar\x80").is_none());
    }

    #[test]
    fn test_roundtrip_no_payload() {
        let og_err = StatusError::new(StatusCodeError::NotFound, "not found detail");

        let trailers = trailers_from_status(Err(og_err.clone()));
        let rt_err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(rt_err.code(), og_err.code());
        assert_eq!(rt_err.message(), og_err.message());
        assert!(!rt_err.has_payloads());
    }

    #[test]
    fn test_roundtrip_ok() {
        let trailers = trailers_from_status(Ok(()));
        let status = status_from_trailers(trailers);
        assert!(status.is_ok());
    }

    #[test]
    fn test_status_from_trailers_ok() {
        let trailers = Trailers::new(Ok(()));

        let status = status_from_trailers(trailers);
        assert!(status.is_ok());
    }

    #[test]
    fn test_status_from_trailers_no_details_bin() {
        let trailers = Trailers::new(Err(grpc::StatusError::new(
            grpc::StatusCodeError::NotFound,
            "Resource missing gRPC status",
        )));

        let restored_err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(restored_err.code(), StatusCodeError::NotFound);
        assert_eq!(restored_err.message(), "Resource missing gRPC status");
        assert!(!restored_err.has_payloads());
    }

    #[test]
    fn test_status_from_trailers_matching_code() {
        let mut rpc_status = google_rpc::Status::new();
        rpc_status.set_code(StatusCodeError::NotFound as i32);
        rpc_status.set_message("Resource missing RPC status");
        let rpc_status_bytes = rpc_status
            .serialize()
            .expect("rpc status serialization succeeds");

        let mut m = grpc::metadata::MetadataMap::new();
        m.insert_bin(
            "grpc-status-details-bin",
            bytes::Bytes::from_owner(rpc_status_bytes)
                .try_into()
                .expect("Bytes to metadata value cannot fail"),
        );
        let trailers = Trailers::new(Err(grpc::StatusError::new(
            grpc::StatusCodeError::NotFound,
            "Resource missing gRPC status",
        )))
        .with_metadata(m);

        let restored_err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(restored_err.code(), StatusCodeError::NotFound);
        assert_eq!(restored_err.message(), "Resource missing RPC status");
    }

    #[test]
    fn test_status_from_trailers_mismatch_code() {
        let mut rpc_status = google_rpc::Status::new();
        rpc_status.set_code(StatusCodeError::NotFound as i32);
        rpc_status.set_message("Resource missing RPC status");
        let mut any = Any::new();
        any.set_type_url("the_type_url");
        any.set_value(b"the any value");
        rpc_status.details_mut().push(any);
        let rpc_status_bytes = rpc_status
            .serialize()
            .expect("rpc status serialization succeeds");

        let mut m = grpc::metadata::MetadataMap::new();
        m.insert_bin(
            "grpc-status-details-bin",
            bytes::Bytes::from_owner(rpc_status_bytes)
                .try_into()
                .expect("Bytes to metadata value cannot fail"),
        );
        let trailers = Trailers::new(Err(grpc::StatusError::new(
            grpc::StatusCodeError::PermissionDenied,
            "Permission denied gRPC status",
        )))
        .with_metadata(m);

        let err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert_eq!(
            err.message(),
            "RPC status code mismatch: gRPC code PermissionDenied, google.rpc.Code NotFound"
        );
    }

    #[test]
    fn test_status_from_trailers_mismatch_code_ok() {
        // Empty message is has OK status
        let rpc_status_bytes = b"";
        let mut m = grpc::metadata::MetadataMap::new();
        m.insert_bin(
            "grpc-status-details-bin",
            bytes::Bytes::from_owner(rpc_status_bytes)
                .try_into()
                .expect("Bytes to metadata value cannot fail"),
        );
        let trailers = Trailers::new(Err(grpc::StatusError::new(
            grpc::StatusCodeError::PermissionDenied,
            "Permission denied gRPC status",
        )))
        .with_metadata(m);

        let err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert_eq!(
            err.message(),
            "grpc-status-details-bin status code was OK, but should always be an error"
        );
    }

    #[test]
    fn test_status_from_trailers_ok_with_details_bin() {
        let mut rpc_status = google_rpc::Status::new();
        rpc_status.set_code(StatusCodeError::NotFound as i32);
        rpc_status.set_message("Resource missing RPC status");
        let rpc_status_bytes = rpc_status
            .serialize()
            .expect("rpc status serialization succeeds");

        let mut m = grpc::metadata::MetadataMap::new();
        m.insert_bin(
            "grpc-status-details-bin",
            bytes::Bytes::from_owner(rpc_status_bytes)
                .try_into()
                .expect("Bytes to metadata value cannot fail"),
        );
        let trailers = Trailers::new(Ok(())).with_metadata(m);

        let err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert_eq!(
            err.message(),
            "grpc-status-details-bin metadata cannot be present when gRPC status code is OK"
        );
    }

    #[test]
    fn test_status_from_trailers_corrupt() {
        let mut m = grpc::metadata::MetadataMap::new();
        m.insert_bin(
            "grpc-status-details-bin",
            bytes::Bytes::from_owner(b"not actually encoded proto")
                .try_into()
                .expect("Bytes to metadata value cannot fail"),
        );
        let trailers = Trailers::new(Err(grpc::StatusError::new(
            grpc::StatusCodeError::PermissionDenied,
            "Permission denied gRPC status",
        )))
        .with_metadata(m);

        let err = status_from_trailers(trailers).unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert!(
            err.message()
                .contains("Failed to parse grpc-status-details-bin:")
        );
    }
}
