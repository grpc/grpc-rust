use bytes::Bytes;
use http_body_util::BodyExt;
use tonic::codec::{Codec, EncodeBody, GzipLevel, Streaming};
use tonic_prost::ProstCodec;

use crate::SomeData;

// This crate is outside tonic: using these constructors here also checks that
// low-level users can pass compression settings without private API access.
#[tokio::test]
async fn public_encode_body_constructors_honor_gzip_levels() {
    for server in [false, true] {
        let message = SomeData {
            data: vec![42; 16 * 1024],
        };
        let mut lengths = Vec::new();
        for level in [GzipLevel::NONE, GzipLevel::FAST, GzipLevel::BEST] {
            let mut codec = ProstCodec::<SomeData, SomeData>::default();
            let source = tokio_stream::iter([Ok(message.clone())]);
            let config = Some(level.into());
            let body = if server {
                EncodeBody::new_server_with_config(
                    codec.encoder(),
                    source,
                    config,
                    Default::default(),
                    None,
                )
            } else {
                EncodeBody::new_client_with_config(codec.encoder(), source, config, None)
            };
            let collected = body.collect().await.unwrap();
            if server {
                assert_eq!(collected.trailers().unwrap()["grpc-status"], "0");
            } else {
                assert!(collected.trailers().is_none());
            }
            let bytes = collected.to_bytes();
            assert_eq!(bytes[0], 1);
            assert_eq!(
                u32::from_be_bytes(bytes[1..5].try_into().unwrap()) as usize,
                bytes.len() - 5
            );
            lengths.push(bytes.len());
            let mut decoded = Streaming::new_request(
                codec.decoder(),
                http_body_util::Full::<Bytes>::new(bytes),
                Some(tonic::codec::CompressionEncoding::Gzip),
                None,
            );
            assert_eq!(decoded.message().await.unwrap().unwrap(), message);
            assert!(decoded.message().await.unwrap().is_none());
        }
        assert!(lengths[0] > lengths[1]);
        assert!(lengths[1] > lengths[2]);
    }
}
