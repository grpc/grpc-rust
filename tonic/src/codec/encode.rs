use super::compression::{
    CompressionEncoding, CompressionSettings, SingleMessageCompressionOverride, compress,
};
use super::{BufferSettings, DEFAULT_MAX_SEND_MESSAGE_SIZE, EncodeBuf, Encoder, HEADER_SIZE};
use crate::Status;
use bytes::{BufMut, Bytes, BytesMut};
use http::HeaderMap;
use http_body::{Body, Frame};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll, ready},
};
use tokio_stream::{Stream, StreamExt, adapters::Fuse};

/// Combinator for efficient encoding of messages into reasonably sized buffers.
/// EncodedBytes encodes ready messages from its delegate stream into a BytesMut,
/// splitting off and yielding a buffer when either:
///  * The delegate stream polls as not ready, or
///  * The encoded buffer surpasses YIELD_THRESHOLD.
#[pin_project(project = EncodedBytesProj)]
#[derive(Debug)]
struct EncodedBytes<T, U> {
    #[pin]
    source: Fuse<U>,
    encoder: T,
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buf: BytesMut,
    uncompression_buf: BytesMut,
    error: Option<Status>,
}

impl<T: Encoder, U: Stream> EncodedBytes<T, U> {
    fn new(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        compression_override: SingleMessageCompressionOverride,
        max_message_size: Option<usize>,
    ) -> Self {
        let buffer_settings = encoder.buffer_settings();
        let buf = BytesMut::with_capacity(buffer_settings.buffer_size);

        let compression_encoding =
            if compression_override == SingleMessageCompressionOverride::Disable {
                None
            } else {
                compression_encoding
            };

        let uncompression_buf = if compression_encoding.is_some() {
            BytesMut::with_capacity(buffer_settings.buffer_size)
        } else {
            BytesMut::new()
        };

        Self {
            source: source.fuse(),
            encoder,
            compression_encoding,
            max_message_size,
            buf,
            uncompression_buf,
            error: None,
        }
    }
}

impl<T, U> Stream for EncodedBytes<T, U>
where
    T: Encoder<Error = Status>,
    U: Stream<Item = Result<T::Item, Status>>,
{
    type Item = Result<Bytes, Status>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let EncodedBytesProj {
            mut source,
            encoder,
            compression_encoding,
            max_message_size,
            buf,
            uncompression_buf,
            error,
        } = self.project();
        let buffer_settings = encoder.buffer_settings();

        if let Some(status) = error.take() {
            return Poll::Ready(Some(Err(status)));
        }

        loop {
            match source.as_mut().poll_next(cx) {
                Poll::Pending if buf.is_empty() => {
                    return Poll::Pending;
                }
                Poll::Ready(None) if buf.is_empty() => {
                    return Poll::Ready(None);
                }
                Poll::Pending | Poll::Ready(None) => {
                    return Poll::Ready(Some(Ok(buf.split_to(buf.len()).freeze())));
                }
                Poll::Ready(Some(Ok(item))) => {
                    if let Err(status) = encode_item(
                        encoder,
                        buf,
                        uncompression_buf,
                        *compression_encoding,
                        *max_message_size,
                        buffer_settings,
                        item,
                    ) {
                        return Poll::Ready(Some(Err(status)));
                    }

                    if buf.len() >= buffer_settings.yield_threshold {
                        return Poll::Ready(Some(Ok(buf.split_to(buf.len()).freeze())));
                    }
                }
                Poll::Ready(Some(Err(status))) => {
                    if buf.is_empty() {
                        return Poll::Ready(Some(Err(status)));
                    }
                    *error = Some(status);
                    return Poll::Ready(Some(Ok(buf.split_to(buf.len()).freeze())));
                }
            }
        }
    }
}

fn encode_item<T>(
    encoder: &mut T,
    buf: &mut BytesMut,
    uncompression_buf: &mut BytesMut,
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buffer_settings: BufferSettings,
    item: T::Item,
) -> Result<(), Status>
where
    T: Encoder<Error = Status>,
{
    let offset = buf.len();
    let mut buf = EncodeItemGuard::new(buf, offset);

    buf.buf.reserve(HEADER_SIZE);
    unsafe {
        buf.buf.advance_mut(HEADER_SIZE);
    }

    if let Some(encoding) = compression_encoding {
        uncompression_buf.clear();

        encoder
            .encode(item, &mut EncodeBuf::new(uncompression_buf))
            .map_err(|err| Status::internal(format!("Error encoding: {err}")))?;

        let uncompressed_len = uncompression_buf.len();

        compress(
            CompressionSettings {
                encoding,
                buffer_growth_interval: buffer_settings.buffer_size,
            },
            uncompression_buf,
            buf.buf,
            uncompressed_len,
        )
        .map_err(|err| Status::internal(format!("Error compressing: {err}")))?;
    } else {
        encoder
            .encode(item, &mut EncodeBuf::new(buf.buf))
            .map_err(|err| Status::internal(format!("Error encoding: {err}")))?;
    }

    // now that we know length, we can write the header
    finish_encoding(
        compression_encoding,
        max_message_size,
        &mut buf.buf[offset..],
    )?;
    buf.disarm();
    Ok(())
}

struct EncodeItemGuard<'a> {
    buf: &'a mut BytesMut,
    offset: usize,
    armed: bool,
}

impl<'a> EncodeItemGuard<'a> {
    fn new(buf: &'a mut BytesMut, offset: usize) -> Self {
        Self {
            buf,
            offset,
            armed: true,
        }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for EncodeItemGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.buf.truncate(self.offset);
        }
    }
}

fn finish_encoding(
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buf: &mut [u8],
) -> Result<(), Status> {
    let len = buf.len() - HEADER_SIZE;
    let limit = max_message_size.unwrap_or(DEFAULT_MAX_SEND_MESSAGE_SIZE);
    if len > limit {
        return Err(Status::out_of_range(format!(
            "Error, encoded message length too large: found {len} bytes, the limit is: {limit} bytes"
        )));
    }

    if len > u32::MAX as usize {
        return Err(Status::resource_exhausted(format!(
            "Cannot return body with more than 4GB of data but got {len} bytes"
        )));
    }
    {
        let mut buf = &mut buf[..HEADER_SIZE];
        buf.put_u8(compression_encoding.is_some() as u8);
        buf.put_u32(len as u32);
    }

    Ok(())
}

#[derive(Debug)]
enum Role {
    Client,
    Server,
}

/// A specialized implementation of [Body] for encoding [Result<Bytes, Status>].
#[pin_project]
#[derive(Debug)]
pub struct EncodeBody<T, U> {
    #[pin]
    inner: EncodedBytes<T, U>,
    state: EncodeState,
}

#[derive(Debug)]
struct EncodeState {
    error: Option<Status>,
    role: Role,
    is_end_stream: bool,
}

impl<T: Encoder, U: Stream> EncodeBody<T, U> {
    /// Turns a stream of grpc messages into [EncodeBody] which is used by grpc clients for
    /// turning the messages into http frames for sending over the network.
    pub fn new_client(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Self {
        Self {
            inner: EncodedBytes::new(
                encoder,
                source,
                compression_encoding,
                SingleMessageCompressionOverride::default(),
                max_message_size,
            ),
            state: EncodeState {
                error: None,
                role: Role::Client,
                is_end_stream: false,
            },
        }
    }

    /// Turns a stream of grpc results (message or error status) into [EncodeBody] which is used by grpc
    /// servers for turning the messages into http frames for sending over the network.
    pub fn new_server(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        compression_override: SingleMessageCompressionOverride,
        max_message_size: Option<usize>,
    ) -> Self {
        Self {
            inner: EncodedBytes::new(
                encoder,
                source,
                compression_encoding,
                compression_override,
                max_message_size,
            ),
            state: EncodeState {
                error: None,
                role: Role::Server,
                is_end_stream: false,
            },
        }
    }
}

impl EncodeState {
    fn trailers(&mut self) -> Option<Result<HeaderMap, Status>> {
        match self.role {
            Role::Client => None,
            Role::Server => {
                if self.is_end_stream {
                    return None;
                }

                self.is_end_stream = true;
                let status = if let Some(status) = self.error.take() {
                    status
                } else {
                    Status::ok("")
                };
                Some(status.to_header_map())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    struct PanicEncoder;

    impl Encoder for PanicEncoder {
        type Item = ();
        type Error = Status;

        fn encode(&mut self, _: Self::Item, _: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            panic!("encoding failed")
        }
    }

    struct ErrorEncoder;

    impl Encoder for ErrorEncoder {
        type Item = ();
        type Error = Status;

        fn encode(&mut self, _: Self::Item, _: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            Err(Status::internal("encoding failed"))
        }
    }

    struct BytesEncoder;

    impl Encoder for BytesEncoder {
        type Item = &'static [u8];
        type Error = Status;

        fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            dst.put_slice(item);
            Ok(())
        }
    }

    fn assert_panicking_encode_restores_buffer(compression_encoding: Option<CompressionEncoding>) {
        let mut buf = BytesMut::from(&b"prefix"[..]);
        let offset = buf.len();
        let mut uncompression_buf = BytesMut::new();

        let result = catch_unwind(AssertUnwindSafe(|| {
            encode_item(
                &mut PanicEncoder,
                &mut buf,
                &mut uncompression_buf,
                compression_encoding,
                None,
                BufferSettings::default(),
                (),
            )
        }));

        assert!(result.is_err());
        assert_eq!(buf.len(), offset);
    }

    fn assert_erroring_encode_restores_buffer(compression_encoding: Option<CompressionEncoding>) {
        let mut buf = BytesMut::from(&b"prefix"[..]);
        let offset = buf.len();
        let mut uncompression_buf = BytesMut::new();

        let result = encode_item(
            &mut ErrorEncoder,
            &mut buf,
            &mut uncompression_buf,
            compression_encoding,
            None,
            BufferSettings::default(),
            (),
        );

        assert!(result.is_err());
        assert_eq!(buf.len(), offset);
    }

    #[test]
    fn encode_item_restores_buffer_after_encoder_panic() {
        assert_panicking_encode_restores_buffer(None);
    }

    #[test]
    fn encode_item_restores_buffer_after_encoder_error() {
        assert_erroring_encode_restores_buffer(None);
    }

    #[test]
    fn encode_item_restores_buffer_after_finish_encoding_error() {
        let mut buf = BytesMut::from(&b"prefix"[..]);
        let offset = buf.len();
        let mut uncompression_buf = BytesMut::new();

        let result = encode_item(
            &mut BytesEncoder,
            &mut buf,
            &mut uncompression_buf,
            None,
            Some(0),
            BufferSettings::default(),
            b"hello",
        );

        assert!(result.is_err());
        assert_eq!(buf.len(), offset);
    }

    #[test]
    fn encode_item_writes_header_and_payload() {
        let mut buf = BytesMut::from(&b"prefix"[..]);
        let mut uncompression_buf = BytesMut::new();

        encode_item(
            &mut BytesEncoder,
            &mut buf,
            &mut uncompression_buf,
            None,
            None,
            BufferSettings::default(),
            b"hello",
        )
        .unwrap();

        assert_eq!(&buf[..], b"prefix\0\0\0\0\x05hello");
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn encode_item_restores_buffer_after_compressed_encoder_failures() {
        let compression_encoding = Some(CompressionEncoding::Gzip);
        assert_panicking_encode_restores_buffer(compression_encoding);
        assert_erroring_encode_restores_buffer(compression_encoding);
    }
}

impl<T, U> Body for EncodeBody<T, U>
where
    T: Encoder<Error = Status>,
    U: Stream<Item = Result<T::Item, Status>>,
{
    type Data = Bytes;
    type Error = Status;

    fn is_end_stream(&self) -> bool {
        self.state.is_end_stream
    }

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let self_proj = self.project();
        match ready!(self_proj.inner.poll_next(cx)) {
            Some(Ok(d)) => Some(Ok(Frame::data(d))).into(),
            Some(Err(status)) => match self_proj.state.role {
                Role::Client => Some(Err(status)).into(),
                Role::Server => {
                    self_proj.state.is_end_stream = true;
                    Some(Ok(Frame::trailers(status.to_header_map()?))).into()
                }
            },
            None => self_proj
                .state
                .trailers()
                .map(|t| t.map(Frame::trailers))
                .into(),
        }
    }
}
