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

//! HTTP specific body utilities.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use http_body_util::BodyExt as _;

// A type erased HTTP body.
type BoxBody = http_body_util::combinators::UnsyncBoxBody<bytes::Bytes, crate::Status>;

/// Shared body plumbing behind `body::Body` and `local::body::Body`: either type is
/// empty or wraps a (possibly `!Send`) boxed body `B`.
#[derive(Debug)]
pub(crate) enum BodyKind<B> {
    Empty,
    Wrap(B),
}

impl<B> BodyKind<B>
where
    B: http_body::Body<Data = bytes::Bytes, Error = crate::Status> + Unpin,
{
    pub(crate) fn poll_frame(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body::Frame<bytes::Bytes>, crate::Status>>> {
        match self {
            BodyKind::Empty => Poll::Ready(None),
            BodyKind::Wrap(body) => Pin::new(body).poll_frame(cx),
        }
    }

    pub(crate) fn size_hint(&self) -> http_body::SizeHint {
        match self {
            BodyKind::Empty => http_body::SizeHint::with_exact(0),
            BodyKind::Wrap(body) => body.size_hint(),
        }
    }

    pub(crate) fn is_end_stream(&self) -> bool {
        match self {
            BodyKind::Empty => true,
            BodyKind::Wrap(body) => body.is_end_stream(),
        }
    }
}

/// A body type used in `tonic`.
#[derive(Debug)]
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
        B: http_body::Body<Data = bytes::Bytes> + Send + 'static,
        B::Error: Into<crate::BoxError>,
    {
        if body.is_end_stream() {
            return Self::empty();
        }

        let mut body = Some(body);

        if let Some(body) = <dyn std::any::Any>::downcast_mut::<Option<Body>>(&mut body) {
            return body.take().unwrap();
        }

        if let Some(body) = <dyn std::any::Any>::downcast_mut::<Option<BoxBody>>(&mut body) {
            return Self {
                kind: BodyKind::Wrap(body.take().unwrap()),
            };
        }

        let body = body
            .unwrap()
            .map_err(crate::Status::map_error)
            .boxed_unsync();

        Self {
            kind: BodyKind::Wrap(body),
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::empty()
    }
}

impl http_body::Body for Body {
    type Data = bytes::Bytes;
    type Error = crate::Status;

    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
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
