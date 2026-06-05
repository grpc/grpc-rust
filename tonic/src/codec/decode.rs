use super::compression::{CompressionEncoding, CompressionSettings, decompress};
use super::{BufferSettings, DEFAULT_MAX_RECV_MESSAGE_SIZE, DecodeBuf, Decoder, HEADER_SIZE};
use crate::{Code, Status, body::Body, metadata::MetadataMap};
use bytes::{Buf, BufMut, BytesMut};
use http::{HeaderMap, StatusCode};
use http_body::Body as HttpBody;
use http_body_util::BodyExt;
use pin_project::{pin_project, pinned_drop};
use std::{
    fmt,
    future::{self, Future},
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
    task::{Context, Poll, ready},
};
use sync_wrapper::SyncWrapper;
use tokio_stream::Stream;
use tracing::{debug, trace};

#[doc(hidden)]
pub trait StreamingSource<T>:
    Stream<Item = Result<StreamingEvent<T>, Status>> + Send + 'static
{
}

impl<T, S> StreamingSource<T> for S where
    S: Stream<Item = Result<StreamingEvent<T>, Status>> + Send + 'static
{
}

type BoxStreaming<T> = Pin<Box<dyn StreamingSource<T>>>;

/// Streaming requests and responses.
///
/// This will wrap some inner [`Body`] and [`Decoder`] and provide an interface
/// to fetch the message stream and trailing metadata
#[pin_project]
pub struct Streaming<T, S = BoxStreaming<T>> {
    #[pin]
    inner: SyncWrapper<S>,
    trailers: Option<HeaderMap>,
    _marker: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone)]
enum State {
    ReadHeader,
    ReadBody {
        compression: Option<CompressionEncoding>,
        len: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Request,
    Response(StatusCode),
    EmptyResponse,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum StreamingEvent<T> {
    Message(T),
    Trailers(HeaderMap),
}

#[pin_project(PinnedDrop)]
struct StreamingInner<T, D>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    #[pin]
    decode: Option<D::DecodeFuture<'static>>,
    #[pin]
    body: SyncWrapper<Body>,
    decoder: D,
    buffer_settings: BufferSettings,
    state: State,
    direction: Direction,
    buf: BytesMut,
    trailers: Option<HeaderMap>,
    decompress_buf: BytesMut,
    encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
    _marker: PhantomData<fn() -> T>,
    #[pin]
    _pin: PhantomPinned,
}

impl<T> Streaming<T, ()> {
    /// Create a new streaming response in the grpc response format for decoding a response [Body]
    /// into message of type T
    pub fn new_response<B, D>(
        decoder: D,
        body: B,
        status_code: StatusCode,
        encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Streaming<T, impl StreamingSource<T>>
    where
        T: Send + 'static,
        B: HttpBody + Send + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + Send + 'static,
    {
        Self::new(
            decoder,
            body,
            Direction::Response(status_code),
            encoding,
            max_message_size,
        )
    }

    pub(crate) fn new_response_or_empty<B, D>(
        decoder: D,
        body: B,
        status_code: StatusCode,
        expect_additional_trailers: bool,
        encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Streaming<T, impl StreamingSource<T>>
    where
        T: Send + 'static,
        B: HttpBody + Send + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + Send + 'static,
    {
        let direction = if expect_additional_trailers {
            Direction::Response(status_code)
        } else {
            Direction::EmptyResponse
        };

        Self::new(decoder, body, direction, encoding, max_message_size)
    }

    /// Create empty response. For creating responses that have no content (headers + trailers only)
    pub fn new_empty<B, D>(decoder: D, body: B) -> Streaming<T, impl StreamingSource<T>>
    where
        T: Send + 'static,
        B: HttpBody + Send + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + Send + 'static,
    {
        Self::new(decoder, body, Direction::EmptyResponse, None, None)
    }

    /// Create a new streaming request in the grpc response format for decoding a request [Body]
    /// into message of type T
    pub fn new_request<B, D>(
        decoder: D,
        body: B,
        encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Streaming<T, impl StreamingSource<T>>
    where
        T: Send + 'static,
        B: HttpBody + Send + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + Send + 'static,
    {
        Self::new(
            decoder,
            body,
            Direction::Request,
            encoding,
            max_message_size,
        )
    }

    fn new<B, D>(
        decoder: D,
        body: B,
        direction: Direction,
        encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Streaming<T, impl StreamingSource<T>>
    where
        T: Send + 'static,
        B: HttpBody + Send + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + Send + 'static,
    {
        let body = Body::new(
            body.map_frame(|frame| frame.map_data(|mut buf| buf.copy_to_bytes(buf.remaining())))
                .map_err(|err| Status::map_error(err.into())),
        );

        Streaming {
            inner: SyncWrapper::new(streaming_events(
                decoder,
                body,
                direction,
                encoding,
                max_message_size,
            )),
            trailers: None,
            _marker: PhantomData,
        }
    }
}
impl<T, S> Streaming<T, S>
where
    S: StreamingSource<T>,
{
    /// Boxes the stream source behind the compatibility `Streaming<T>` type.
    pub fn boxed(self) -> Streaming<T> {
        Streaming {
            inner: SyncWrapper::new(Box::pin(self.inner.into_inner())),
            trailers: self.trailers,
            _marker: PhantomData,
        }
    }
}

fn streaming_events<T, D>(
    decoder: D,
    body: Body,
    direction: Direction,
    encoding: Option<CompressionEncoding>,
    max_message_size: Option<usize>,
) -> impl StreamingSource<T>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    let buffer_settings = decoder.buffer_settings();

    StreamingInner {
        decode: None,
        body: SyncWrapper::new(body),
        decoder,
        buffer_settings,
        state: State::ReadHeader,
        direction,
        buf: BytesMut::with_capacity(buffer_settings.buffer_size),
        trailers: None,
        decompress_buf: BytesMut::new(),
        encoding,
        max_message_size,
        _marker: PhantomData,
        _pin: PhantomPinned,
    }
}

unsafe fn extend_decode_future_lifetime<'a, T, D>(
    future: D::DecodeFuture<'a>,
) -> D::DecodeFuture<'static>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    // SAFETY: `StreamingInner` stores the future together with the decoder and
    // buffers it borrows. The type is !Unpin, the future is always dropped in
    // `PinnedDrop` before those fields, and no method moves or mutates the
    // borrowed fields while `decode` is `Some`.
    unsafe { std::mem::transmute::<D::DecodeFuture<'a>, D::DecodeFuture<'static>>(future) }
}

#[pinned_drop]
impl<T, D> PinnedDrop for StreamingInner<T, D>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    fn drop(self: Pin<&mut Self>) {
        self.project().decode.set(None);
    }
}

impl<T, D> StreamingInner<T, D>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    fn poll_decode_future(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<T>, Status>> {
        let mut this = self.project();
        let Some(future) = this.decode.as_mut().as_pin_mut() else {
            return Poll::Ready(Ok(None));
        };

        let result = ready!(future.poll(cx));
        this.decode.set(None);

        if let Ok(Some(_)) = &result {
            *this.state = State::ReadHeader;
        }

        Poll::Ready(result)
    }

    fn poll_decode_chunk(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<T>, Status>> {
        if self.as_ref().project_ref().decode.get_ref().is_some() {
            return self.poll_decode_future(cx);
        }

        {
            let mut this = self.as_mut().project();

            if let State::ReadHeader = *this.state {
                if this.buf.remaining() < HEADER_SIZE {
                    return Poll::Ready(Ok(None));
                }

                let compression_encoding = match this.buf.get_u8() {
                    0 => None,
                    1 => {
                        if this.encoding.is_some() {
                            *this.encoding
                        } else {
                            // https://grpc.github.io/grpc/core/md_doc_compression.html
                            // An ill-constructed message with its Compressed-Flag bit set but lacking a grpc-encoding
                            // entry different from identity in its metadata MUST fail with INTERNAL status,
                            // its associated description indicating the invalid Compressed-Flag condition.
                            return Poll::Ready(Err(Status::internal(
                                "protocol error: received message with compressed-flag but no grpc-encoding was specified",
                            )));
                        }
                    }
                    f => {
                        trace!("unexpected compression flag");
                        let message = if let Direction::Response(status) = *this.direction {
                            format!(
                                "protocol error: received message with invalid compression flag: {f} (valid flags are 0 and 1) while receiving response with status: {status}"
                            )
                        } else {
                            format!(
                                "protocol error: received message with invalid compression flag: {f} (valid flags are 0 and 1), while sending request"
                            )
                        };
                        return Poll::Ready(Err(Status::internal(message)));
                    }
                };

                let len = this.buf.get_u32() as usize;
                let limit = this
                    .max_message_size
                    .unwrap_or(DEFAULT_MAX_RECV_MESSAGE_SIZE);
                if len > limit {
                    return Poll::Ready(Err(Status::out_of_range(format!(
                        "Error, decoded message length too large: found {len} bytes, the limit is: {limit} bytes"
                    ))));
                }

                this.buf.reserve(len);

                *this.state = State::ReadBody {
                    compression: compression_encoding,
                    len,
                };
            }

            let (len, compression) = match *this.state {
                State::ReadBody { len, compression } => (len, compression),
                State::ReadHeader => return Poll::Ready(Ok(None)),
            };

            // if we haven't read enough of the message then return and keep
            // reading
            if this.buf.remaining() < len || this.buf.len() < len {
                return Poll::Ready(Ok(None));
            }

            let future = if let Some(encoding) = compression {
                this.decompress_buf.clear();
                let limit = this
                    .max_message_size
                    .unwrap_or(DEFAULT_MAX_RECV_MESSAGE_SIZE);
                let limited_out_buf = (&mut *this.decompress_buf).limit(limit);

                if let Err(err) = decompress(
                    CompressionSettings {
                        encoding,
                        buffer_growth_interval: this.buffer_settings.buffer_size,
                    },
                    this.buf,
                    limited_out_buf,
                    len,
                ) {
                    if matches!(err.kind(), std::io::ErrorKind::WriteZero) {
                        return Poll::Ready(Err(Status::resource_exhausted(format!(
                            "Error decompressing: size limit, of {limit} bytes, exceeded while decompressing message"
                        ))));
                    }
                    let message = if let Direction::Response(status) = *this.direction {
                        format!(
                            "Error decompressing: {err}, while receiving response with status: {status}"
                        )
                    } else {
                        format!("Error decompressing: {err}, while sending request")
                    };
                    return Poll::Ready(Err(Status::internal(message)));
                }
                let decompressed_len = this.decompress_buf.len();
                let decode_buf = DecodeBuf::new(this.decompress_buf, decompressed_len);
                this.decoder.decode(decode_buf)
            } else {
                let decode_buf = DecodeBuf::new(this.buf, len);
                this.decoder.decode(decode_buf)
            };

            // SAFETY: see `extend_decode_future_lifetime`.
            let future = unsafe { extend_decode_future_lifetime::<T, D>(future) };
            this.decode.set(Some(future));
        }

        self.poll_decode_future(cx)
    }

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Option<()>, Status>> {
        let mut this = self.project();
        let frame = match ready!(this.body.as_mut().get_pin_mut().poll_frame(cx)) {
            Some(Ok(frame)) => frame,
            Some(Err(status)) => {
                if *this.direction == Direction::Request && status.code() == Code::Cancelled {
                    return Poll::Ready(Ok(None));
                }

                debug!("decoder inner stream error: {:?}", status);
                return Poll::Ready(Err(status));
            }
            None => {
                // FIXME: improve buf usage.
                return Poll::Ready(if this.buf.has_remaining() {
                    trace!("unexpected EOF decoding stream, state: {:?}", this.state);
                    Err(Status::internal("Unexpected EOF decoding stream."))
                } else {
                    Ok(None)
                });
            }
        };

        if frame.is_data() {
            this.buf.put(frame.into_data().unwrap());
            Poll::Ready(Ok(Some(())))
        } else if frame.is_trailers() {
            let trailers = frame.into_trailers().unwrap();
            if let Some(existing) = this.trailers {
                existing.extend(trailers);
            } else {
                *this.trailers = Some(trailers);
            }

            Poll::Ready(Ok(None))
        } else {
            panic!("unexpected frame: {frame:?}");
        }
    }

    fn response(self: Pin<&mut Self>) -> Result<(), Status> {
        let this = self.project();
        if let Direction::Response(status) = *this.direction {
            if let Err(Some(e)) = crate::status::infer_grpc_status(this.trailers.as_ref(), status) {
                // If the trailers contain a grpc-status, then we should return that as the error
                // and otherwise stop the stream (by taking the error state)
                this.trailers.take();
                return Err(e);
            }
        }
        Ok(())
    }

    fn take_trailers(self: Pin<&mut Self>) -> Option<HeaderMap> {
        self.project().trailers.take()
    }
}

impl<T, D> Stream for StreamingInner<T, D>
where
    T: Send + 'static,
    D: Decoder<Item = T, Error = Status> + Send + 'static,
{
    type Item = Result<StreamingEvent<T>, Status>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match ready!(self.as_mut().poll_decode_chunk(cx)) {
                Ok(Some(item)) => return Poll::Ready(Some(Ok(StreamingEvent::Message(item)))),
                Ok(None) => {}
                Err(status) => return Poll::Ready(Some(Err(status))),
            }

            match ready!(self.as_mut().poll_frame(cx)) {
                Ok(Some(())) => {}
                Ok(None) => {
                    if let Err(status) = self.as_mut().response() {
                        return Poll::Ready(Some(Err(status)));
                    }

                    if let Some(trailers) = self.as_mut().take_trailers() {
                        return Poll::Ready(Some(Ok(StreamingEvent::Trailers(trailers))));
                    }

                    return Poll::Ready(None);
                }
                Err(status) => return Poll::Ready(Some(Err(status))),
            }
        }
    }
}

impl<T, S> Streaming<T, S>
where
    S: StreamingSource<T>,
{
    /// Fetch the next message from this stream.
    ///
    /// # Return value
    ///
    /// - `Result::Err(val)` means a gRPC error was sent by the sender instead
    ///   of a valid response message. Refer to [`Status::code`] and
    ///   [`Status::message`] to examine possible error causes.
    ///
    /// - `Result::Ok(None)` means the stream was closed by the sender and no
    ///   more messages will be delivered. Further attempts to call
    ///   [`Streaming::message`] will result in the same return value.
    ///
    /// - `Result::Ok(Some(val))` means the sender streamed a valid response
    ///   message `val`.
    ///
    /// ```rust
    /// # use tonic::{Streaming, Status, codec::Decoder};
    /// # use std::fmt::Debug;
    /// # async fn next_message_ex<T, D>(mut request: Streaming<T>) -> Result<(), Status>
    /// # where T: Debug + 'static,
    /// # D: Decoder<Item = T, Error = Status> + Send  + 'static,
    /// # {
    /// if let Some(next_message) = request.message().await? {
    ///     println!("{:?}", next_message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn message(&mut self) -> Result<Option<T>, Status> {
        match future::poll_fn(|cx| {
            // SAFETY: `message` borrows `self` mutably for the entire returned
            // future, so callers cannot move the stream while this poll is in
            // progress.
            unsafe { Pin::new_unchecked(&mut *self) }.poll_next(cx)
        })
        .await
        {
            Some(Ok(m)) => Ok(Some(m)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// Fetch the trailing metadata.
    ///
    /// This will drain the stream of all its messages to receive the trailing
    /// metadata. If [`Streaming::message`] returns `None` then this function
    /// will not need to poll for trailers since the body was totally consumed.
    ///
    /// ```rust
    /// # use tonic::{Streaming, Status};
    /// # async fn trailers_ex<T: 'static>(mut request: Streaming<T>) -> Result<(), Status> {
    /// if let Some(metadata) = request.trailers().await? {
    ///     println!("{:?}", metadata);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trailers(&mut self) -> Result<Option<MetadataMap>, Status> {
        // Shortcut to see if we already pulled the trailers in the stream step
        // we need to do that so that the stream can error on trailing grpc-status
        if let Some(trailers) = self.trailers.take() {
            return Ok(Some(MetadataMap::from_headers(trailers)));
        }

        // To fetch the trailers we must clear the body and drop it.
        while self.message().await?.is_some() {}

        // Since we call poll_trailers internally on poll_next we need to
        // check if it got cached again.
        if let Some(trailers) = self.trailers.take() {
            return Ok(Some(MetadataMap::from_headers(trailers)));
        }

        // We've polled through all the frames, and still no trailers, return None
        Ok(None)
    }
}

impl<T, S> Stream for Streaming<T, S>
where
    S: StreamingSource<T>,
{
    type Item = Result<T, Status>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            match ready!(this.inner.as_mut().get_pin_mut().poll_next(cx)) {
                Some(Ok(StreamingEvent::Message(item))) => return Poll::Ready(Some(Ok(item))),
                Some(Ok(StreamingEvent::Trailers(trailers))) => *this.trailers = Some(trailers),
                Some(Err(status)) => return Poll::Ready(Some(Err(status))),
                None => return Poll::Ready(None),
            }
        }
    }
}

impl<T, S> fmt::Debug for Streaming<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Streaming").finish()
    }
}

#[cfg(test)]
static_assertions::assert_impl_all!(Streaming<()>: Send, Sync);
