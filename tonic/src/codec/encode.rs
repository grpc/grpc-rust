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
    fmt,
    future::Future,
    marker::PhantomPinned,
    mem,
    pin::Pin,
    ptr,
    task::{Context, Poll, ready},
};
use tokio_stream::{Stream, StreamExt, adapters::Fuse};

type EncodeItemOutput<T> = (T, BytesMut, BytesMut, Result<(), Status>);

/// Combinator for efficient encoding of messages into reasonably sized buffers.
/// EncodedBytes encodes ready messages from its delegate stream into a BytesMut,
/// splitting off and yielding a buffer when either:
///  * The delegate stream polls as not ready, or
///  * The encoded buffer surpasses YIELD_THRESHOLD.
#[pin_project(project = EncodedBytesProj)]
struct EncodedBytes<T: Encoder + 'static, U> {
    #[pin]
    source: Fuse<U>,
    encoder: Option<T>,
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buf: Option<BytesMut>,
    uncompression_buf: Option<BytesMut>,
    #[pin]
    encode: Option<EncodeItem<T>>,
    error: Option<Status>,
}

impl<T, U> fmt::Debug for EncodedBytes<T, U>
where
    T: Encoder + fmt::Debug + 'static,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncodedBytes")
            .field("source", &self.source)
            .field("encoder", &self.encoder)
            .field("compression_encoding", &self.compression_encoding)
            .field("max_message_size", &self.max_message_size)
            .field("buf", &self.buf)
            .field("uncompression_buf", &self.uncompression_buf)
            .field("encode_in_progress", &self.encode.is_some())
            .field("error", &self.error)
            .finish()
    }
}

impl<T: Encoder + 'static, U: Stream> EncodedBytes<T, U> {
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
            encoder: Some(encoder),
            compression_encoding,
            max_message_size,
            buf: Some(buf),
            uncompression_buf: Some(uncompression_buf),
            encode: None,
            error: None,
        }
    }
}

impl<T, U> Stream for EncodedBytes<T, U>
where
    T: Encoder<Error = Status> + Send + 'static,
    T::Item: Send + 'static,
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
            mut encode,
            error,
        } = self.project();

        if let Some(status) = error.take() {
            return Poll::Ready(Some(Err(status)));
        }

        loop {
            if let Some(future) = encode.as_mut().as_pin_mut() {
                let (encoded_encoder, encoded_buf, encoded_uncompression_buf, result) =
                    ready!(future.poll(cx));
                encode.set(None);
                *encoder = Some(encoded_encoder);
                *buf = Some(encoded_buf);
                *uncompression_buf = Some(encoded_uncompression_buf);

                if let Err(status) = result {
                    return Poll::Ready(Some(Err(status)));
                }

                let buffer_settings = encoder
                    .as_ref()
                    .expect("encoder restored after async encode")
                    .buffer_settings();
                if buf
                    .as_ref()
                    .expect("buffer restored after async encode")
                    .len()
                    >= buffer_settings.yield_threshold
                {
                    return Poll::Ready(Some(Ok(take_buf(buf))));
                }

                continue;
            }

            match source.as_mut().poll_next(cx) {
                Poll::Pending if buf.as_ref().expect("buffer available").is_empty() => {
                    return Poll::Pending;
                }
                Poll::Ready(None) if buf.as_ref().expect("buffer available").is_empty() => {
                    return Poll::Ready(None);
                }
                Poll::Pending | Poll::Ready(None) => {
                    return Poll::Ready(Some(Ok(take_buf(buf))));
                }
                Poll::Ready(Some(Ok(item))) => {
                    let encoded_encoder = encoder.take().expect("encoder available");
                    let buffer_settings = encoded_encoder.buffer_settings();
                    let encoded_buf = buf.take().expect("buffer available");
                    let encoded_uncompression_buf = uncompression_buf
                        .take()
                        .expect("uncompression buffer available");

                    encode.set(Some(EncodeItem::new(
                        encoded_encoder,
                        encoded_buf,
                        encoded_uncompression_buf,
                        *compression_encoding,
                        *max_message_size,
                        buffer_settings,
                        item,
                    )));
                }
                Poll::Ready(Some(Err(status))) => {
                    if buf.as_ref().expect("buffer available").is_empty() {
                        return Poll::Ready(Some(Err(status)));
                    }
                    *error = Some(status);
                    return Poll::Ready(Some(Ok(take_buf(buf))));
                }
            }
        }
    }
}

fn take_buf(buf: &mut Option<BytesMut>) -> Bytes {
    let buf = buf.as_mut().expect("buffer available");
    buf.split_to(buf.len()).freeze()
}

#[derive(Clone, Copy, Debug)]
enum EncodeTarget {
    Buffer,
    UncompressionBuffer(CompressionEncoding),
}

/// Owns one in-flight encode operation without allocating per message.
///
/// When `state` is `Encoding`, the encoder future borrows `encoder` and one of
/// the buffers. The future is stored in `state` with an extended lifetime, so
/// this type must stay pinned until that future completes or is dropped. `_pin`
/// prevents `EncodeItem` from being `Unpin`, and `state` is declared before the
/// borrowed fields so a pending future is dropped before `encoder` and the
/// buffers.
struct EncodeItem<T>
where
    T: Encoder + 'static,
{
    state: EncodeItemState<T>,
    encoder: Option<T>,
    buf: BytesMut,
    uncompression_buf: BytesMut,
    compression_encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    buffer_settings: BufferSettings,
    offset: usize,
    _pin: PhantomPinned,
}

enum EncodeItemState<T>
where
    T: Encoder + 'static,
{
    Start(Option<T::Item>),
    Encoding {
        future: T::EncodeFuture<'static>,
        target: EncodeTarget,
    },
    Done,
}

impl<T> EncodeItem<T>
where
    T: Encoder<Error = Status> + 'static,
    T::Item: 'static,
{
    fn new(
        encoder: T,
        buf: BytesMut,
        uncompression_buf: BytesMut,
        compression_encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
        buffer_settings: BufferSettings,
        item: T::Item,
    ) -> Self {
        Self {
            state: EncodeItemState::Start(Some(item)),
            encoder: Some(encoder),
            buf,
            uncompression_buf,
            compression_encoding,
            max_message_size,
            buffer_settings,
            offset: 0,
            _pin: PhantomPinned,
        }
    }

    fn start_encoding(&mut self) {
        let EncodeItemState::Start(item) = &mut self.state else {
            unreachable!("encode item already started");
        };
        let item = item.take().expect("item available");

        self.offset = self.buf.len();
        self.buf.reserve(HEADER_SIZE);
        unsafe {
            self.buf.advance_mut(HEADER_SIZE);
        }

        let target = if let Some(encoding) = self.compression_encoding {
            self.uncompression_buf.clear();
            EncodeTarget::UncompressionBuffer(encoding)
        } else {
            EncodeTarget::Buffer
        };

        let encoder = self.encoder.as_mut().expect("encoder available") as *mut T;
        let buf = &mut self.buf as *mut BytesMut;
        let uncompression_buf = &mut self.uncompression_buf as *mut BytesMut;

        // SAFETY: `EncodeItem` is only started from `poll`, after it is pinned
        // inside `EncodedBytes::encode`. `_pin` prevents moving it after that.
        // The returned future borrows `encoder` and one buffer; both are fields
        // of this pinned `EncodeItem` and are not moved while `state` is
        // `Encoding`.
        let future = unsafe {
            match target {
                EncodeTarget::Buffer => {
                    let dst = EncodeBuf::new(&mut *buf);
                    (&mut *encoder).encode(item, dst)
                }
                EncodeTarget::UncompressionBuffer(_) => {
                    let dst = EncodeBuf::new(&mut *uncompression_buf);
                    (&mut *encoder).encode(item, dst)
                }
            }
        };
        // SAFETY: the future is stored in the same pinned `EncodeItem` that
        // owns the fields it borrows. `state` is the first field, so the future
        // is dropped before those fields if the encode is cancelled.
        let future =
            unsafe { mem::transmute::<T::EncodeFuture<'_>, T::EncodeFuture<'static>>(future) };

        self.state = EncodeItemState::Encoding { future, target };
    }

    fn finish_encoding(&mut self, target: EncodeTarget) -> Result<(), Status> {
        if let EncodeTarget::UncompressionBuffer(encoding) = target {
            let uncompressed_len = self.uncompression_buf.len();
            compress(
                CompressionSettings {
                    encoding,
                    buffer_growth_interval: self.buffer_settings.buffer_size,
                },
                &mut self.uncompression_buf,
                &mut self.buf,
                uncompressed_len,
            )
            .map_err(|err| Status::internal(format!("Error compressing: {err}")))?;
        }

        finish_encoding(
            self.compression_encoding,
            self.max_message_size,
            &mut self.buf[self.offset..],
        )
    }
}

impl<T> Future for EncodeItem<T>
where
    T: Encoder<Error = Status> + 'static,
    T::Item: 'static,
{
    type Output = EncodeItemOutput<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        loop {
            if let EncodeItemState::Start(_) = &this.state {
                this.start_encoding();
                continue;
            }

            let (target, result) = match &mut this.state {
                EncodeItemState::Encoding { future, target } => {
                    let target = *target;
                    let result = ready!(unsafe { Pin::new_unchecked(future) }.poll(cx));
                    (target, result)
                }
                EncodeItemState::Done => panic!("encode item polled after completion"),
                EncodeItemState::Start(_) => unreachable!("encode item start handled above"),
            };

            // SAFETY: the future has returned `Ready`, so dropping it in place
            // ends its borrows before we inspect or move the encoder and
            // buffers. `Done` prevents the completed future from being dropped
            // a second time.
            unsafe {
                ptr::drop_in_place(&mut this.state);
                ptr::write(&mut this.state, EncodeItemState::Done);
            }

            let result = result
                .map_err(|err| Status::internal(format!("Error encoding: {err}")))
                .and_then(|()| this.finish_encoding(target));

            let encoder = this.encoder.take().expect("encoder available");
            let buf = mem::take(&mut this.buf);
            let uncompression_buf = mem::take(&mut this.uncompression_buf);

            return Poll::Ready((encoder, buf, uncompression_buf, result));
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
pub struct EncodeBody<T: Encoder + 'static, U> {
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

impl<T: Encoder + 'static, U: Stream> EncodeBody<T, U> {
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

impl<T, U> Body for EncodeBody<T, U>
where
    T: Encoder<Error = Status> + Send + 'static,
    T::Item: Send + 'static,
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
