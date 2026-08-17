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

//! gRPC codec streaming types for `!Send` local mode.

use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Buf;
use http::StatusCode;
use http_body::Body as HttpBody;
use http_body_util::BodyExt;
use tokio_stream::Stream;

use crate::codec::{CompressionEncoding, Decoder, Direction, StreamingImpl};
use crate::metadata::MetadataMap;
use crate::{Status, local::body::Body};

/// Streaming requests and responses without `Send` bounds.
///
/// This will wrap some inner [`Body`] and [`Decoder`] and provide an interface
/// to fetch the message stream and trailing metadata.
pub struct Streaming<T> {
    inner: StreamingImpl<T, Box<dyn Decoder<Item = T, Error = Status> + 'static>, Body>,
}

impl<T> Unpin for Streaming<T> {}

impl<T> Streaming<T> {
    /// Create a new streaming response in the grpc response format for decoding a response [Body]
    /// into message of type T
    pub fn new_response<B, D>(
        decoder: D,
        body: B,
        status_code: StatusCode,
        encoding: Option<CompressionEncoding>,
        max_message_size: Option<usize>,
    ) -> Self
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + 'static,
    {
        Self::new(
            decoder,
            body,
            Direction::Response(status_code),
            encoding,
            max_message_size,
        )
    }

    /// Create empty response. For creating responses that have no content (headers + trailers only)
    pub fn new_empty<B, D>(decoder: D, body: B) -> Self
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + 'static,
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
    ) -> Self
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + 'static,
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
    ) -> Self
    where
        B: HttpBody + 'static,
        B::Error: Into<crate::BoxError>,
        D: Decoder<Item = T, Error = Status> + 'static,
    {
        let buffer_size = decoder.buffer_settings().buffer_size;
        let body = Body::new(
            body.map_frame(|frame| frame.map_data(|mut buf| buf.copy_to_bytes(buf.remaining())))
                .map_err(|err| Status::map_error(err.into())),
        );
        Self {
            inner: StreamingImpl::new(
                Box::new(decoder),
                body,
                direction,
                encoding,
                max_message_size,
                buffer_size,
            ),
        }
    }

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
    pub async fn message(&mut self) -> Result<Option<T>, Status> {
        self.inner.message().await
    }

    /// Fetch the trailing metadata.
    ///
    /// This will drain the stream of all its messages to receive the trailing
    /// metadata. If [`Streaming::message`] returns `None` then this function
    /// will not need to poll for trailers since the body was totally consumed.
    pub async fn trailers(&mut self) -> Result<Option<MetadataMap>, Status> {
        self.inner.trailers().await
    }
}

impl<T> Stream for Streaming<T> {
    type Item = Result<T, Status>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next(cx)
    }
}

impl<T> fmt::Debug for Streaming<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Streaming").finish()
    }
}

#[cfg(test)]
static_assertions::assert_not_impl_any!(Streaming<()>: Send, Sync);

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    use bytes::{Buf, BufMut, Bytes};
    use http_body::Frame;

    use super::*;
    use crate::codec::DecodeBuf;

    /// A non-`Send` body yielding queued frames.
    struct FrameBody {
        frames: VecDeque<Frame<Bytes>>,
    }

    impl HttpBody for FrameBody {
        type Data = Bytes;
        type Error = Status;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            Poll::Ready(self.frames.pop_front().map(Ok))
        }
    }

    /// A non-`Send` decoder counting calls via `Rc<Cell<u32>>`.
    struct RcVecDecoder {
        calls: Rc<Cell<u32>>,
    }

    impl Decoder for RcVecDecoder {
        type Item = Vec<u8>;
        type Error = Status;

        fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
            self.calls.set(self.calls.get() + 1);
            let len = buf.remaining();
            Ok(Some(buf.copy_to_bytes(len).to_vec()))
        }
    }

    fn framed(payload: &[u8]) -> Frame<Bytes> {
        let mut buf = bytes::BytesMut::new();
        buf.put_u8(0);
        buf.put_u32(payload.len() as u32);
        buf.put_slice(payload);
        Frame::data(buf.freeze())
    }

    #[tokio::test]
    async fn decodes_crafted_frames_with_non_send_decoder() {
        let calls = Rc::new(Cell::new(0));
        let body = FrameBody {
            frames: VecDeque::from([framed(b"hello"), framed(b"world!")]),
        };
        let decoder = RcVecDecoder {
            calls: calls.clone(),
        };
        let mut stream = Streaming::new_request(decoder, body, None, None);

        assert_eq!(stream.message().await.unwrap(), Some(b"hello".to_vec()));
        assert_eq!(stream.message().await.unwrap(), Some(b"world!".to_vec()));
        assert_eq!(stream.message().await.unwrap(), None);
        assert_eq!(calls.get(), 2);
    }

    #[tokio::test]
    async fn trailers_are_surfaced() {
        let mut trailers = http::HeaderMap::new();
        trailers.insert("grpc-status", "0".parse().unwrap());

        let body = FrameBody {
            frames: VecDeque::from([framed(b"only"), Frame::trailers(trailers)]),
        };
        let decoder = RcVecDecoder {
            calls: Rc::new(Cell::new(0)),
        };
        let mut stream = Streaming::new_request(decoder, body, None, None);

        assert_eq!(stream.message().await.unwrap(), Some(b"only".to_vec()));
        let metadata = stream.trailers().await.unwrap().expect("trailers");
        assert_eq!(
            metadata.get("grpc-status").map(|v| v.to_str().unwrap()),
            Some("0")
        );
    }
}
