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

use prost::Message;
use std::marker::PhantomData;
use tonic::Status;
use tonic::codec::{BufferSettings, Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

/// A [`Codec`] that implements `application/grpc+proto` via the prost library.
#[derive(Debug, Clone)]
pub struct ProstCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<T, U> ProstCodec<T, U> {
    /// Configure a ProstCodec with encoder/decoder buffer settings. This is used to control
    /// how memory is allocated and grows per RPC.
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T, U> Default for ProstCodec<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> ProstCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
{
    /// A tool for building custom codecs based on prost encoding and decoding.
    /// See the codec_buffers example for one possible way to use this.
    pub fn raw_encoder(buffer_settings: BufferSettings) -> <Self as Codec>::Encoder {
        ProstEncoder {
            _pd: PhantomData,
            buffer_settings,
        }
    }

    /// A tool for building custom codecs based on prost encoding and decoding.
    /// See the codec_buffers example for one possible way to use this.
    pub fn raw_decoder(buffer_settings: BufferSettings) -> <Self as Codec>::Decoder {
        ProstDecoder {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<T, U> Codec for ProstCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
{
    type Encode = T;
    type Decode = U;

    type Encoder = ProstEncoder<T>;
    type Decoder = ProstDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        ProstEncoder {
            _pd: PhantomData,
            buffer_settings: BufferSettings::default(),
        }
    }

    fn decoder(&mut self) -> Self::Decoder {
        ProstDecoder {
            _pd: PhantomData,
            buffer_settings: BufferSettings::default(),
        }
    }
}

/// A [`Encoder`] that knows how to encode `T`.
#[derive(Debug, Clone, Default)]
pub struct ProstEncoder<T> {
    _pd: PhantomData<T>,
    buffer_settings: BufferSettings,
}

impl<T> ProstEncoder<T> {
    /// Get a new encoder with explicit buffer settings
    pub fn new(buffer_settings: BufferSettings) -> Self {
        Self {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<T: Message> Encoder for ProstEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // prost::Message::encode checks remaining_mut() (unbounded for
        // BytesMut) and never reserves. Use encode_raw after one
        // encoded_len() so we pre-size without walking the message twice.
        buf.reserve(item.encoded_len());
        item.encode_raw(buf);
        Ok(())
    }

    fn buffer_settings(&self) -> BufferSettings {
        self.buffer_settings
    }
}

/// A [`Decoder`] that knows how to decode `U`.
#[derive(Debug, Clone, Default)]
pub struct ProstDecoder<U> {
    _pd: PhantomData<U>,
    buffer_settings: BufferSettings,
}

impl<U> ProstDecoder<U> {
    /// Get a new decoder with explicit buffer settings
    pub fn new(buffer_settings: BufferSettings) -> Self {
        Self {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<U: Message + Default> Decoder for ProstDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let item = Message::decode(buf)
            .map(Option::Some)
            .map_err(from_decode_error)?;

        Ok(item)
    }

    fn buffer_settings(&self) -> BufferSettings {
        self.buffer_settings
    }
}

fn from_decode_error(error: prost::DecodeError) -> Status {
    // Map Protobuf parse errors to an INTERNAL status code, as per
    // https://github.com/grpc/grpc/blob/master/doc/statuscodes.md
    Status::internal(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, BufMut, BytesMut};
    use http_body::Body;
    use http_body_util::BodyExt as _;
    use prost::encoding::{DecodeContext, WireType};
    use std::pin::pin;
    use tonic::codec::SingleMessageCompressionOverride;
    use tonic::codec::{EncodeBody, HEADER_SIZE, Streaming};

    const LEN: usize = 10000;
    // The maximum uncompressed size in bytes for a message. Set to 2MB.
    const MAX_MESSAGE_SIZE: usize = 2 * 1024 * 1024;

    #[tokio::test]
    async fn decode() {
        let decoder = MockDecoder::default();

        let msg = vec![0u8; LEN];

        let mut buf = BytesMut::new();

        buf.reserve(msg.len() + HEADER_SIZE);
        buf.put_u8(0);
        buf.put_u32(msg.len() as u32);

        buf.put(&msg[..]);

        let body = body::MockBody::new(&buf[..], 10005, 0);

        let mut stream = Streaming::new_request(decoder, body, None, None);

        let mut i = 0usize;
        while let Some(output_msg) = stream.message().await.unwrap() {
            assert_eq!(output_msg.len(), msg.len());
            i += 1;
        }
        assert_eq!(i, 1);
    }

    #[tokio::test]
    async fn decode_max_message_size_exceeded() {
        let decoder = MockDecoder::default();

        let msg = vec![0u8; MAX_MESSAGE_SIZE + 1];

        let mut buf = BytesMut::new();

        buf.reserve(msg.len() + HEADER_SIZE);
        buf.put_u8(0);
        buf.put_u32(msg.len() as u32);

        buf.put(&msg[..]);

        let body = body::MockBody::new(&buf[..], MAX_MESSAGE_SIZE + HEADER_SIZE + 1, 0);

        let mut stream = Streaming::new_request(decoder, body, None, Some(MAX_MESSAGE_SIZE));

        let actual = stream.message().await.unwrap_err();

        let expected = Status::out_of_range(format!(
            "Error, decoded message length too large: found {} bytes, the limit is: {} bytes",
            msg.len(),
            MAX_MESSAGE_SIZE
        ));

        assert_eq!(actual.code(), expected.code());
        assert_eq!(actual.message(), expected.message());
    }

    #[tokio::test]
    async fn encode() {
        let encoder = MockEncoder::default();

        let msg = Vec::from(&[0u8; 1024][..]);

        let messages = std::iter::repeat_with(move || Ok::<_, Status>(msg.clone())).take(10000);
        let source = tokio_stream::iter(messages);

        let mut body = pin!(EncodeBody::new_server(
            encoder,
            source,
            None,
            SingleMessageCompressionOverride::default(),
            None,
        ));

        while let Some(r) = body.frame().await {
            r.unwrap();
        }
    }

    #[tokio::test]
    async fn encode_max_message_size_exceeded() {
        let encoder = MockEncoder::default();

        let msg = vec![0u8; MAX_MESSAGE_SIZE + 1];

        let messages = std::iter::once(Ok::<_, Status>(msg));
        let source = tokio_stream::iter(messages);

        let mut body = pin!(EncodeBody::new_server(
            encoder,
            source,
            None,
            SingleMessageCompressionOverride::default(),
            Some(MAX_MESSAGE_SIZE),
        ));

        let frame = body
            .frame()
            .await
            .expect("at least one frame")
            .expect("no error polling frame");
        assert_eq!(
            frame
                .into_trailers()
                .expect("got trailers")
                .get(Status::GRPC_STATUS)
                .expect("grpc-status header"),
            "11"
        );
        assert!(body.is_end_stream());
    }

    // skip on windows because CI stumbles over our 4GB allocation
    #[cfg(not(target_family = "windows"))]
    #[tokio::test]
    async fn encode_too_big() {
        let encoder = MockEncoder::default();

        let msg = vec![0u8; u32::MAX as usize + 1];

        let messages = std::iter::once(Ok::<_, Status>(msg));
        let source = tokio_stream::iter(messages);

        let mut body = pin!(EncodeBody::new_server(
            encoder,
            source,
            None,
            SingleMessageCompressionOverride::default(),
            Some(usize::MAX),
        ));

        let frame = body
            .frame()
            .await
            .expect("at least one frame")
            .expect("no error polling frame");
        assert_eq!(
            frame
                .into_trailers()
                .expect("got trailers")
                .get(Status::GRPC_STATUS)
                .expect("grpc-status header"),
            "8"
        );
        assert!(body.is_end_stream());
    }

    /// `prost::Message::encode` never calls `BufMut::reserve`; `BytesMut::chunk_mut`
    /// grows 64 bytes at a time when full. `ProstEncoder` must pre-size from
    /// `encoded_len()` so a large payload is not written into a near-empty buffer.
    #[tokio::test]
    async fn encode_presizes_buffer_from_encoded_len() {
        const PAYLOAD: usize = 32 * 1024;
        let encoder = ProstEncoder::<PresizeProbe>::new(BufferSettings::new(0, PAYLOAD * 2));
        let source = tokio_stream::iter(std::iter::once(Ok::<_, Status>(PresizeProbe {
            len: PAYLOAD,
        })));
        let mut body = pin!(EncodeBody::new_server(
            encoder,
            source,
            None,
            SingleMessageCompressionOverride::default(),
            None,
        ));

        while let Some(frame) = body.frame().await {
            frame.unwrap();
        }
    }

    #[test]
    fn encode_raw_after_reserve_matches_encode() {
        fn check(msg: impl Message) {
            let mut expected = BytesMut::new();
            msg.encode(&mut expected).unwrap();

            let mut actual = BytesMut::new();
            actual.reserve(msg.encoded_len());
            msg.encode_raw(&mut actual);

            assert_eq!(expected, actual);
        }

        check(BytesMsg {
            data: vec![0xAB; 1024],
        });
        check(UnpackedVarints {
            values: (0..256).collect(),
        });
    }

    #[test]
    fn reserve_does_not_increase_buffer_grows() {
        let small = BytesMsg {
            data: vec![0xAB; 64],
        };
        let large = BytesMsg {
            data: vec![0xAB; 256 * 1024],
        };
        let varints = UnpackedVarints {
            values: (0..16_384).collect(),
        };

        let small_reserved = grows(true, &small);
        let small_unreserved = grows(false, &small);
        let large_reserved = grows(true, &large);
        let large_unreserved = grows(false, &large);
        let varints_reserved = grows(true, &varints);
        let varints_unreserved = grows(false, &varints);

        assert!(
            small_reserved <= small_unreserved,
            "small reserved={small_reserved} unreserved={small_unreserved}"
        );
        assert!(
            large_reserved <= large_unreserved,
            "large reserved={large_reserved} unreserved={large_unreserved}"
        );
        assert!(
            varints_reserved < varints_unreserved,
            "varints reserved={varints_reserved} unreserved={varints_unreserved}"
        );
    }

    fn grows(reserve: bool, msg: &impl Message) -> usize {
        let mut buf = GrowCounter::new();
        if reserve {
            buf.reserve(msg.encoded_len());
        }
        msg.encode(&mut buf).unwrap();
        buf.grows
    }

    #[derive(Clone, PartialEq, prost::Message)]
    struct BytesMsg {
        #[prost(bytes = "vec", tag = "1")]
        data: Vec<u8>,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    struct UnpackedVarints {
        #[prost(int64, repeated, packed = "false", tag = "1")]
        values: Vec<i64>,
    }

    struct GrowCounter {
        inner: BytesMut,
        grows: usize,
    }

    impl GrowCounter {
        fn new() -> Self {
            Self {
                inner: BytesMut::new(),
                grows: 0,
            }
        }

        fn reserve(&mut self, additional: usize) {
            let cap = self.inner.capacity();
            self.inner.reserve(additional);
            if self.inner.capacity() > cap {
                self.grows += 1;
            }
        }
    }

    unsafe impl BufMut for GrowCounter {
        fn remaining_mut(&self) -> usize {
            self.inner.remaining_mut()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            unsafe { self.inner.advance_mut(cnt) }
        }

        fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
            if self.inner.capacity() == self.inner.len() {
                self.grows += 1;
            }
            self.inner.chunk_mut()
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.inner.capacity() - self.inner.len() < src.len() {
                self.grows += 1;
            }
            self.inner.put_slice(src);
        }

        fn put<T: Buf>(&mut self, src: T)
        where
            Self: Sized,
        {
            if self.inner.capacity() - self.inner.len() < src.remaining() {
                self.grows += 1;
            }
            self.inner.put(src);
        }

        fn put_bytes(&mut self, val: u8, cnt: usize) {
            if self.inner.capacity() - self.inner.len() < cnt {
                self.grows += 1;
            }
            self.inner.put_bytes(val, cnt);
        }
    }

    struct PresizeProbe {
        len: usize,
    }

    impl prost::Message for PresizeProbe {
        fn encoded_len(&self) -> usize {
            self.len
        }

        fn encode_raw(&self, buf: &mut impl BufMut) {
            let spare = buf.chunk_mut().len();
            assert!(
                spare >= self.len,
                "ProstEncoder should reserve encoded_len() before writing; spare={spare}, needed={}",
                self.len
            );
            buf.put_bytes(0, self.len);
        }

        fn merge_field(
            &mut self,
            _tag: u32,
            _wire_type: WireType,
            _buf: &mut impl Buf,
            _ctx: DecodeContext,
        ) -> Result<(), prost::DecodeError> {
            unimplemented!("decode is not exercised")
        }

        fn clear(&mut self) {}
    }

    #[derive(Debug, Clone, Default)]
    struct MockEncoder {}

    impl Encoder for MockEncoder {
        type Item = Vec<u8>;
        type Error = Status;

        fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            buf.put(&item[..]);
            Ok(())
        }

        fn buffer_settings(&self) -> BufferSettings {
            Default::default()
        }
    }

    #[derive(Debug, Clone, Default)]
    struct MockDecoder {}

    impl Decoder for MockDecoder {
        type Item = Vec<u8>;
        type Error = Status;

        fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
            let out = Vec::from(buf.chunk());
            buf.advance(LEN);
            Ok(Some(out))
        }

        fn buffer_settings(&self) -> BufferSettings {
            Default::default()
        }
    }

    mod body {
        use bytes::Bytes;
        use http_body::{Body, Frame};
        use std::{
            pin::Pin,
            task::{Context, Poll},
        };
        use tonic::Status;

        #[derive(Debug)]
        pub(super) struct MockBody {
            data: Bytes,

            // the size of the partial message to send
            partial_len: usize,

            // the number of times we've sent
            count: usize,
        }

        impl MockBody {
            pub(super) fn new(b: &[u8], partial_len: usize, count: usize) -> Self {
                MockBody {
                    data: Bytes::copy_from_slice(b),
                    partial_len,
                    count,
                }
            }
        }

        impl Body for MockBody {
            type Data = Bytes;
            type Error = Status;

            fn poll_frame(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
                // every other call to poll_data returns data
                let should_send = self.count.is_multiple_of(2);
                let data_len = self.data.len();
                let partial_len = self.partial_len;
                let count = self.count;
                if data_len > 0 {
                    let result = if should_send {
                        let response =
                            self.data
                                .split_to(if count == 0 { partial_len } else { data_len });
                        Poll::Ready(Some(Ok(Frame::data(response))))
                    } else {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    };
                    // make some fake progress
                    self.count += 1;
                    result
                } else {
                    Poll::Ready(None)
                }
            }
        }
    }
}
