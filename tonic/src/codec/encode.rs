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
    future::poll_fn,
    pin::Pin,
    task::{Context, Poll, ready},
};
use tokio_stream::{Stream, StreamExt};

fn encoded_bytes<T, U>(
    mut encoder: T,
    source: U,
    compression_encoding: Option<CompressionEncoding>,
    compression_override: SingleMessageCompressionOverride,
    max_message_size: Option<usize>,
) -> impl Stream<Item = Result<Bytes, Status>>
where
    T: Encoder<Error = Status>,
    U: Stream<Item = Result<T::Item, Status>>,
{
    async_stream::stream! {
        let buffer_settings = encoder.buffer_settings();
        let mut buf = BytesMut::with_capacity(buffer_settings.buffer_size);

        let compression_encoding =
            if compression_override == SingleMessageCompressionOverride::Disable {
                None
            } else {
                compression_encoding
            };

        let mut uncompression_buf = if compression_encoding.is_some() {
            BytesMut::with_capacity(buffer_settings.buffer_size)
        } else {
            BytesMut::new()
        };

        let source = source.fuse();
        let mut source = std::pin::pin!(source);

        loop {
            let source_poll = poll_fn(|cx| match source.as_mut().poll_next(cx) {
                Poll::Pending if buf.is_empty() => Poll::Pending,
                poll => Poll::Ready(poll),
            })
            .await;

            match source_poll {
                Poll::Pending => yield Ok(take_buf(&mut buf)),
                Poll::Ready(None) => {
                    if !buf.is_empty() {
                        yield Ok(take_buf(&mut buf));
                    }
                    return;
                }
                Poll::Ready(Some(Ok(item))) => {
                    if let Err(status) = encode_item(
                        &mut encoder,
                        &mut buf,
                        &mut uncompression_buf,
                        compression_encoding,
                        max_message_size,
                        buffer_settings,
                        item,
                    )
                    .await
                    {
                        yield Err(status);
                        continue;
                    }

                    if buf.len() >= buffer_settings.yield_threshold {
                        yield Ok(take_buf(&mut buf));
                    }
                }
                Poll::Ready(Some(Err(status))) => {
                    if !buf.is_empty() {
                        yield Ok(take_buf(&mut buf));
                    }
                    yield Err(status);
                }
            }
        }
    }
}

fn take_buf(buf: &mut BytesMut) -> Bytes {
    buf.split_to(buf.len()).freeze()
}

async fn encode_item<T>(
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

    buf.reserve(HEADER_SIZE);
    unsafe {
        buf.advance_mut(HEADER_SIZE);
    }

    if let Some(encoding) = compression_encoding {
        uncompression_buf.clear();

        {
            let dst = EncodeBuf::new(uncompression_buf);
            encoder
                .encode(item, dst)
                .await
                .map_err(|err| Status::internal(format!("Error encoding: {err}")))?;
        }

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
        let dst = EncodeBuf::new(buf);
        encoder
            .encode(item, dst)
            .await
            .map_err(|err| Status::internal(format!("Error encoding: {err}")))?;
    }

    // now that we know length, we can write the header
    finish_encoding(compression_encoding, max_message_size, &mut buf[offset..])
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
pub struct EncodeBody<S = ()> {
    #[pin]
    inner: S,
    state: EncodeState,
}

#[derive(Debug)]
struct EncodeState {
    error: Option<Status>,
    role: Role,
    is_end_stream: bool,
}

impl EncodeBody<()> {
    /// Turns a stream of grpc messages into [EncodeBody] which is used by grpc clients for
    /// turning the messages into http frames for sending over the network.
    pub fn new_client<T, U>(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> EncodeBody<impl Stream<Item = Result<Bytes, Status>>>
    where
        T: Encoder<Error = Status>,
        U: Stream<Item = Result<T::Item, Status>>,
    {
        EncodeBody {
            inner: encoded_bytes(
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
    pub fn new_server<T, U>(
        encoder: T,
        source: U,
        compression_encoding: Option<CompressionEncoding>,
        compression_override: SingleMessageCompressionOverride,
        max_message_size: Option<usize>,
    ) -> EncodeBody<impl Stream<Item = Result<Bytes, Status>>>
    where
        T: Encoder<Error = Status>,
        U: Stream<Item = Result<T::Item, Status>>,
    {
        EncodeBody {
            inner: encoded_bytes(
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

impl<S> Body for EncodeBody<S>
where
    S: Stream<Item = Result<Bytes, Status>>,
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
