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

use super::compression::{
    CompressionEncoding, CompressionSettings, SingleMessageCompressionOverride, compress,
};
use super::{BufferSettings, DEFAULT_MAX_SEND_MESSAGE_SIZE, EncodeBuf, Encoder, HEADER_SIZE};
use crate::Status;
use bytes::{BufMut, Bytes, BytesMut};
#[cfg(feature = "h2")]
use h2::{Error as H2Error, Reason as H2Reason};
use http::HeaderMap;
use http_body::{Body, Frame};
use pin_project::pin_project;
use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll, ready},
};
use tokio_stream::{Stream, StreamExt, adapters::Fuse};
use tokio_util::sync::{CancellationToken, WaitForCancellationFutureOwned};

/// Combinator for efficient encoding of messages into reasonably sized buffers.
/// EncodedBytes encodes ready messages from its delegate stream into a BytesMut,
/// splitting off and yielding a buffer when:
///  * The delegate stream polls as not ready,
///  * The encoded buffer surpasses YIELD_THRESHOLD, or
///  * Zero-copy encoding takes a buffer from the encoded message.
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
    chunks: Vec<(usize, Bytes)>,
    pending: VecDeque<Bytes>,
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
            chunks: Vec::new(),
            pending: VecDeque::new(),
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
            chunks,
            pending,
            error,
        } = self.project();
        let buffer_settings = encoder.buffer_settings();

        if let Some(chunk) = pending.pop_front() {
            return Poll::Ready(Some(Ok(chunk)));
        }

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
                        chunks,
                        *compression_encoding,
                        *max_message_size,
                        buffer_settings,
                        item,
                    ) {
                        return Poll::Ready(Some(Err(status)));
                    }

                    if !chunks.is_empty() {
                        split_chunks(buf, chunks, pending);
                        return Poll::Ready(pending.pop_front().map(Ok));
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

#[allow(clippy::too_many_arguments)]
fn encode_item<T>(
    encoder: &mut T,
    buf: &mut BytesMut,
    uncompression_buf: &mut BytesMut,
    chunks: &mut Vec<(usize, Bytes)>,
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buffer_settings: BufferSettings,
    item: T::Item,
) -> Result<(), Status>
where
    T: Encoder<Error = Status>,
{
    let offset = buf.len();
    chunks.clear();

    buf.reserve(HEADER_SIZE);
    buf.put_slice(&[0u8; HEADER_SIZE]);

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
            buf,
            uncompressed_len,
        )
        .map_err(|err| Status::internal(format!("Error compressing: {err}")))?;
    } else {
        let mut dst = match buffer_settings.zero_copy_threshold {
            Some(threshold) => EncodeBuf::with_zero_copy(buf, chunks, threshold),
            None => EncodeBuf::new(buf),
        };
        encoder
            .encode(item, &mut dst)
            .map_err(|err| Status::internal(format!("Error encoding: {err}")))?;
    }

    // now that we know length, we can write the header
    finish_encoding(
        compression_encoding,
        max_message_size,
        &mut buf[offset..],
        chunks,
    )
}

// The whole buffer is moved, not only the bytes up to the last chunk, so that a
// message is never left partially sent if encoding a later message fails.
fn split_chunks(
    buf: &mut BytesMut,
    chunks: &mut Vec<(usize, Bytes)>,
    pending: &mut VecDeque<Bytes>,
) {
    let mut split = 0;
    for (offset, chunk) in chunks.drain(..) {
        if offset > split {
            pending.push_back(buf.split_to(offset - split).freeze());
            split = offset;
        }
        pending.push_back(chunk);
    }
    if !buf.is_empty() {
        pending.push_back(buf.split_to(buf.len()).freeze());
    }
}

fn finish_encoding(
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buf: &mut [u8],
    chunks: &[(usize, Bytes)],
) -> Result<(), Status> {
    let len = chunks
        .iter()
        .fold((buf.len() - HEADER_SIZE) as u64, |len, (_, chunk)| {
            len.saturating_add(chunk.len() as u64)
        });
    let limit = max_message_size.unwrap_or(DEFAULT_MAX_SEND_MESSAGE_SIZE);
    if len > limit as u64 {
        return Err(Status::out_of_range(format!(
            "Error, encoded message length too large: found {len} bytes, the limit is: {limit} bytes"
        )));
    }

    if len > u32::MAX as u64 {
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
    #[pin]
    cancellation_fut: Option<WaitForCancellationFutureOwned>,
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
        Self::new_client_with_cancellation(
            encoder,
            source,
            compression_encoding,
            max_message_size,
            None,
        )
    }

    pub(crate) fn new_client_with_cancellation(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        let cancellation_fut = cancellation_token.map(|c| c.cancelled_owned());
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
            cancellation_fut,
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
            cancellation_fut: None,
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

        if let Some(cancellation_fut) = self_proj.cancellation_fut.as_pin_mut()
            && let Poll::Ready(()) = cancellation_fut.poll(cx)
        {
            #[cfg(feature = "h2")]
            let status = {
                let mut status = Status::cancelled("client cancelled");
                // h2 inspects the error's source chain to determine the RST
                // code, so we set it here.
                status.set_source(std::sync::Arc::new(H2Error::from(H2Reason::CANCEL)));
                status
            };

            #[cfg(not(feature = "h2"))]
            let status = Status::cancelled("client cancelled");
            return Poll::Ready(Some(Err(status)));
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    struct PanickingEncoder;

    impl Encoder for PanickingEncoder {
        type Item = String;
        type Error = Status;

        fn encode(
            &mut self,
            _item: Self::Item,
            _dst: &mut EncodeBuf<'_>,
        ) -> Result<(), Self::Error> {
            panic!("encoder deliberate panic for testing exception safety");
        }
    }

    #[test]
    fn encode_item_exception_safety_on_panic() {
        let mut encoder = PanickingEncoder;
        let mut buf = BytesMut::new();
        let mut uncompression_buf = BytesMut::new();

        let result = catch_unwind(std::panic::AssertUnwindSafe(|| {
            encode_item(
                &mut encoder,
                &mut buf,
                &mut uncompression_buf,
                &mut Vec::new(),
                None,
                None,
                BufferSettings::default(),
                "test".to_string(),
            )
        }));

        assert!(result.is_err(), "Encoder panic should unwind correctly.");
        // Buffer must only contain initialized bytes (5 zero bytes written by put_slice).
        assert_eq!(
            buf.len(),
            HEADER_SIZE,
            "Buffer length should reflect reserved header bytes."
        );
        assert_eq!(
            &buf[..],
            &[0u8; HEADER_SIZE],
            "Buffer must contain only initialized zero bytes."
        );
    }

    mod zero_copy {
        use super::super::*;
        use crate::Code;
        use http_body_util::BodyExt;
        use std::pin::pin;

        const THRESHOLD: usize = 1024;
        const PREFIX: &[u8] = b"prefix";
        const SUFFIX: &[u8] = b"suffix";

        /// Encodes a message as `PREFIX`, the payload passed to `put`, then
        /// `SUFFIX`, like a message with a `Bytes` field between other fields.
        struct PayloadEncoder {
            buffer_settings: BufferSettings,
            fail_len: Option<usize>,
        }

        impl Encoder for PayloadEncoder {
            type Item = Bytes;
            type Error = Status;

            fn encode(&mut self, item: Bytes, dst: &mut EncodeBuf<'_>) -> Result<(), Status> {
                let fail = self.fail_len == Some(item.len());
                dst.put_slice(PREFIX);
                dst.put(item);
                if fail {
                    return Err(Status::internal("payload rejected"));
                }
                dst.put_slice(SUFFIX);
                Ok(())
            }

            fn buffer_settings(&self) -> BufferSettings {
                self.buffer_settings
            }
        }

        struct Encoded {
            data: Vec<Bytes>,
            trailers: HeaderMap,
        }

        impl Encoded {
            fn concat(&self) -> Vec<u8> {
                self.data.concat()
            }

            fn status(&self) -> (Code, String) {
                let status = Status::from_header_map(&self.trailers).expect("grpc-status");
                (status.code(), status.message().to_string())
            }

            fn shares(&self, payload: &Bytes) -> usize {
                self.data
                    .iter()
                    .filter(|d| d.as_ptr() == payload.as_ptr() && d.len() == payload.len())
                    .count()
            }
        }

        async fn encode(
            buffer_settings: BufferSettings,
            items: Vec<Result<Bytes, Status>>,
            compression_encoding: Option<CompressionEncoding>,
            max_message_size: Option<usize>,
            fail_len: Option<usize>,
        ) -> Encoded {
            let encoder = PayloadEncoder {
                buffer_settings,
                fail_len,
            };
            let mut body = pin!(EncodeBody::new_server(
                encoder,
                tokio_stream::iter(items),
                compression_encoding,
                SingleMessageCompressionOverride::default(),
                max_message_size,
            ));

            let mut data = Vec::new();
            while let Some(frame) = body.frame().await {
                let frame = frame.expect("server body does not fail");
                match frame.into_data() {
                    Ok(d) => data.push(d),
                    Err(frame) => {
                        assert!(body.is_end_stream());
                        return Encoded {
                            data,
                            trailers: frame.into_trailers().expect("trailers"),
                        };
                    }
                }
            }
            panic!("no trailers");
        }

        fn payload(len: usize) -> Bytes {
            (0..len).map(|i| i as u8).collect::<Vec<_>>().into()
        }

        fn enabled(settings: BufferSettings) -> BufferSettings {
            settings.with_zero_copy_threshold(THRESHOLD)
        }

        async fn encode_both(
            settings: BufferSettings,
            items: Vec<Result<Bytes, Status>>,
            compression_encoding: Option<CompressionEncoding>,
            max_message_size: Option<usize>,
        ) -> (Encoded, Encoded) {
            let off = encode(
                settings,
                items.clone(),
                compression_encoding,
                max_message_size,
                None,
            )
            .await;
            let on = encode(
                enabled(settings),
                items,
                compression_encoding,
                max_message_size,
                None,
            )
            .await;
            assert_eq!(off.concat(), on.concat());
            assert_eq!(off.status(), on.status());
            (off, on)
        }

        #[tokio::test]
        async fn same_bytes_around_threshold() {
            for len in [
                0,
                1,
                THRESHOLD - 1,
                THRESHOLD,
                THRESHOLD + 1,
                3 * 1024 * 1024 + 7,
            ] {
                let payload = payload(len);
                let (_, on) = encode_both(
                    BufferSettings::default(),
                    vec![Ok(payload.clone())],
                    None,
                    None,
                )
                .await;

                let mut expected = vec![0u8];
                expected
                    .extend_from_slice(&((PREFIX.len() + len + SUFFIX.len()) as u32).to_be_bytes());
                expected.extend_from_slice(PREFIX);
                expected.extend_from_slice(&payload);
                expected.extend_from_slice(SUFFIX);
                assert_eq!(on.concat(), expected, "len {len}");
                assert_eq!(on.status(), (Code::Ok, String::new()));

                let shares = if len >= THRESHOLD { 1 } else { 0 };
                assert_eq!(on.shares(&payload), shares, "len {len}");
            }
        }

        #[tokio::test]
        async fn streaming_mixed_sizes() {
            let lens = [10, 5000, 3, 70_000, 0, THRESHOLD, 2048, 100, 40_000, 1];
            let payloads: Vec<Bytes> = lens.iter().map(|&len| payload(len)).collect();
            let items: Vec<_> = payloads.iter().cloned().map(Ok).collect();

            for settings in [BufferSettings::default(), BufferSettings::new(512, 4096)] {
                let (_, on) = encode_both(settings, items.clone(), None, None).await;

                let shared: Vec<usize> = on
                    .data
                    .iter()
                    .filter_map(|d| payloads.iter().position(|p| p.as_ptr() == d.as_ptr()))
                    .collect();
                let large: Vec<usize> = (0..lens.len()).filter(|&i| lens[i] >= THRESHOLD).collect();
                assert_eq!(shared, large);
            }
        }

        #[tokio::test]
        async fn max_message_size() {
            let payload = payload(2048);
            let len = PREFIX.len() + payload.len() + SUFFIX.len();

            let (_, on) = encode_both(
                BufferSettings::default(),
                vec![Ok(payload.clone())],
                None,
                Some(len),
            )
            .await;
            assert_eq!(on.status(), (Code::Ok, String::new()));
            assert_eq!(on.shares(&payload), 1);

            let (_, on) = encode_both(
                BufferSettings::default(),
                vec![Ok(payload.clone())],
                None,
                Some(len - 1),
            )
            .await;
            assert!(on.data.is_empty());
            let message = format!(
                "Error, encoded message length too large: found {len} bytes, the limit is: {} bytes",
                len - 1
            );
            assert_eq!(on.status(), (Code::OutOfRange, message));
        }

        #[cfg(feature = "gzip")]
        #[tokio::test]
        async fn compressed_is_not_split() {
            let payloads = [payload(100), payload(64 * 1024)];
            let items = payloads.iter().cloned().map(Ok).collect();
            let (off, on) = encode_both(
                BufferSettings::default(),
                items,
                Some(CompressionEncoding::Gzip),
                None,
            )
            .await;
            assert_eq!(off.data.len(), on.data.len());
            for payload in &payloads {
                assert_eq!(on.shares(payload), 0);
            }
        }

        #[tokio::test]
        async fn source_error_after_chunk() {
            let small = payload(10);
            let large = payload(100_000);
            let items = vec![
                Ok(small),
                Ok(large.clone()),
                Err(Status::data_loss("source failed")),
            ];

            let (_, on) = encode_both(BufferSettings::default(), items, None, None).await;
            assert_eq!(on.status(), (Code::DataLoss, "source failed".to_string()));
            assert_eq!(on.shares(&large), 1);
        }

        #[tokio::test]
        async fn encoder_error_after_chunk() {
            let items = vec![
                Ok(payload(10)),
                Ok(payload(5000)),
                Ok(payload(THRESHOLD + 3)),
            ];
            let failed = encode(
                enabled(BufferSettings::default()),
                items.clone(),
                None,
                None,
                Some(THRESHOLD + 3),
            )
            .await;
            let (code, message) = failed.status();
            assert_eq!(code, Code::Internal);
            assert!(message.contains("payload rejected"), "{message}");

            // Messages yielded before the failure are complete.
            let complete = encode(
                BufferSettings::default(),
                items[..2].to_vec(),
                None,
                None,
                None,
            )
            .await;
            assert_eq!(failed.concat(), complete.concat());
        }

        /// Writes each part with `put`, so every part of at least `THRESHOLD`
        /// bytes becomes its own chunk.
        struct PartsEncoder(BufferSettings);

        impl Encoder for PartsEncoder {
            type Item = Vec<Bytes>;
            type Error = Status;

            fn encode(&mut self, item: Vec<Bytes>, dst: &mut EncodeBuf<'_>) -> Result<(), Status> {
                for part in item {
                    dst.put(part);
                }
                Ok(())
            }

            fn buffer_settings(&self) -> BufferSettings {
                self.0
            }
        }

        async fn encode_parts(settings: BufferSettings, parts: Vec<Bytes>) -> Vec<Bytes> {
            let mut body = pin!(EncodeBody::new_server(
                PartsEncoder(settings),
                tokio_stream::iter(vec![Ok(parts)]),
                None,
                SingleMessageCompressionOverride::default(),
                None,
            ));
            let mut data = Vec::new();
            while let Some(frame) = body.frame().await {
                match frame.expect("server body does not fail").into_data() {
                    Ok(d) => data.push(d),
                    Err(_) => break,
                }
            }
            data
        }

        #[tokio::test]
        async fn several_chunks_in_one_message() {
            // Two chunks back to back, a small part between chunks, and a chunk
            // at the end of the message with nothing after it.
            let (a, b, c) = (
                payload(THRESHOLD),
                payload(THRESHOLD + 1),
                payload(2 * THRESHOLD),
            );
            let parts = vec![
                Bytes::from_static(b"head"),
                a.clone(),
                b.clone(),
                Bytes::from_static(b"mid"),
                c.clone(),
            ];

            let off = encode_parts(BufferSettings::default(), parts.clone()).await;
            let on = encode_parts(enabled(BufferSettings::default()), parts).await;

            assert_eq!(off.concat(), on.concat());
            let large = [a.as_ptr(), b.as_ptr(), c.as_ptr()];
            let shared: Vec<*const u8> = on
                .iter()
                .map(|d| d.as_ptr())
                .filter(|p| large.contains(p))
                .collect();
            assert_eq!(shared, large);
            assert_eq!(on.last().map(|d| d.as_ptr()), Some(c.as_ptr()));
        }
    }
}
