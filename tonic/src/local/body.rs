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

//! HTTP specific body utilities for `!Send` local mode.

use std::pin::Pin;
use std::task::Poll;

use http_body_util::BodyExt as _;

use crate::body::BodyKind;

// A type erased HTTP body, without a `Send` bound.
type BoxBody = Pin<Box<dyn http_body::Body<Data = bytes::Bytes, Error = crate::Status> + 'static>>;

/// A body type used in `tonic::local`.
pub struct Body {
    kind: BodyKind<BoxBody>,
}

impl Body {
    /// Create a new empty `Body`.
    pub const fn empty() -> Self {
        Self {
            kind: BodyKind::Empty,
        }
    }

    /// Create a new `Body` from an existing `Body`.
    pub fn new<B>(body: B) -> Self
    where
        B: http_body::Body<Data = bytes::Bytes> + 'static,
        B::Error: Into<crate::BoxError>,
    {
        if body.is_end_stream() {
            return Self::empty();
        }

        let mut body = Some(body);

        if let Some(body) = <dyn std::any::Any>::downcast_mut::<Option<Body>>(&mut body)
            && let Some(body) = body.take()
        {
            return body;
        }

        match body {
            Some(body) => {
                let body: BoxBody = Box::pin(body.map_err(crate::Status::map_error));
                Self {
                    kind: BodyKind::Wrap(body),
                }
            }
            // `body` is only taken in the same-type fast path above, which returns.
            None => Self::empty(),
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body").finish()
    }
}

impl http_body::Body for Body {
    type Data = bytes::Bytes;
    type Error = crate::Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        self.kind.poll_frame(cx)
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.kind.size_hint()
    }

    fn is_end_stream(&self) -> bool {
        self.kind.is_end_stream()
    }
}

#[cfg(test)]
static_assertions::assert_not_impl_any!(Body: Send, Sync);

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    use bytes::Bytes;
    use http_body::Body as _;

    use super::*;

    /// A hand-rolled, non-`Send` body yielding two data frames, counting polls
    /// via an `Rc<Cell<u32>>`.
    struct TwoFrameBody {
        frames: VecDeque<Bytes>,
        polls: Rc<Cell<u32>>,
    }

    impl http_body::Body for TwoFrameBody {
        type Data = Bytes;
        type Error = std::convert::Infallible;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
            self.polls.set(self.polls.get() + 1);
            Poll::Ready(
                self.frames
                    .pop_front()
                    .map(|data| Ok(http_body::Frame::data(data))),
            )
        }
    }

    #[tokio::test]
    async fn empty_is_end_stream() {
        let mut body = Body::empty();
        assert!(body.is_end_stream());
        assert_eq!(body.size_hint().exact(), Some(0));
        assert!(body.frame().await.is_none());
    }

    #[tokio::test]
    async fn wraps_non_send_body() {
        let polls = Rc::new(Cell::new(0));
        let inner = TwoFrameBody {
            frames: VecDeque::from([Bytes::from_static(b"hello "), Bytes::from_static(b"world")]),
            polls: polls.clone(),
        };

        let body = Body::new(inner);
        let collected = body.collect().await.unwrap();
        assert_eq!(collected.to_bytes(), Bytes::from_static(b"hello world"));
        // Two data-frame polls plus the final `None` poll.
        assert_eq!(polls.get(), 3);
    }

    #[test]
    fn new_on_ended_body_is_empty() {
        let body = Body::new(Body::empty());
        assert!(body.is_end_stream());
    }
}
