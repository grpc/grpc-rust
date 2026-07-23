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

#[cfg(feature = "_tls-any")]
use std::future::Future;
#[cfg(feature = "_tls-any")]
use std::pin::pin;
use std::{
    io,
    ops::ControlFlow,
    pin::Pin,
    task::{Context, Poll, ready},
};

use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
#[cfg(feature = "_tls-any")]
use tokio::task::JoinSet;
use tokio_stream::Stream;
#[cfg(feature = "_tls-any")]
use tokio_stream::StreamExt as _;

use super::service::ServerIo;
#[cfg(feature = "_tls-any")]
use super::service::TlsAcceptor;

#[cfg(feature = "_tls-any")]
struct State<IO>(TlsAcceptor, JoinSet<Result<ServerIo<IO>, crate::BoxError>>);

#[pin_project]
pub(crate) struct ServerIoStream<S, IO, IE>
where
    S: Stream<Item = Result<IO, IE>>,
{
    #[pin]
    inner: S,
    #[cfg(feature = "_tls-any")]
    state: Option<State<IO>>,
}

impl<S, IO, IE> ServerIoStream<S, IO, IE>
where
    S: Stream<Item = Result<IO, IE>>,
{
    pub(crate) fn new(incoming: S, #[cfg(feature = "_tls-any")] tls: Option<TlsAcceptor>) -> Self {
        Self {
            inner: incoming,
            #[cfg(feature = "_tls-any")]
            state: tls.map(|tls| State(tls, JoinSet::new())),
        }
    }

    fn poll_next_without_tls(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<ServerIo<IO>, crate::BoxError>>>
    where
        IE: Into<crate::BoxError>,
    {
        match ready!(self.as_mut().project().inner.poll_next(cx)) {
            Some(Ok(io)) => Poll::Ready(Some(Ok(ServerIo::new_io(io)))),
            Some(Err(e)) => match handle_tcp_accept_error(e) {
                ControlFlow::Continue(()) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                ControlFlow::Break(e) => Poll::Ready(Some(Err(e))),
            },
            None => Poll::Ready(None),
        }
    }
}

impl<S, IO, IE> Stream for ServerIoStream<S, IO, IE>
where
    S: Stream<Item = Result<IO, IE>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    IE: Into<crate::BoxError>,
{
    type Item = Result<ServerIo<IO>, crate::BoxError>;

    #[cfg(not(feature = "_tls-any"))]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.poll_next_without_tls(cx)
    }

    #[cfg(feature = "_tls-any")]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut projected = self.as_mut().project();

        let Some(State(tls, tasks)) = projected.state else {
            return self.poll_next_without_tls(cx);
        };

        let select_output = ready!(pin!(select(&mut projected.inner, tasks)).poll(cx));

        match select_output {
            SelectOutput::Incoming(stream) => {
                let tls = tls.clone();
                tasks.spawn(async move {
                    let io = tls.accept(stream).await?;
                    Ok(ServerIo::new_tls_io(io))
                });
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            SelectOutput::Io(io) => Poll::Ready(Some(Ok(io))),

            SelectOutput::TcpErr(e) => match handle_tcp_accept_error(e) {
                ControlFlow::Continue(()) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                ControlFlow::Break(e) => Poll::Ready(Some(Err(e))),
            },

            SelectOutput::TlsErr(e) => {
                tracing::debug!(error = %e, "tls accept error");
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            SelectOutput::Done => Poll::Ready(None),
        }
    }
}

fn handle_tcp_accept_error(e: impl Into<crate::BoxError>) -> ControlFlow<crate::BoxError> {
    let e = e.into();
    tracing::debug!(error = %e, "accept loop error");
    if let Some(e) = e.downcast_ref::<io::Error>() {
        if matches!(
            e.kind(),
            io::ErrorKind::ConnectionAborted
                | io::ErrorKind::ConnectionReset
                | io::ErrorKind::BrokenPipe
                | io::ErrorKind::Interrupted
                | io::ErrorKind::WouldBlock
                | io::ErrorKind::TimedOut
        ) {
            return ControlFlow::Continue(());
        }
    }

    ControlFlow::Break(e)
}

#[cfg(feature = "_tls-any")]
async fn select<IO: 'static, IE>(
    incoming: &mut (impl Stream<Item = Result<IO, IE>> + Unpin),
    tasks: &mut JoinSet<Result<ServerIo<IO>, crate::BoxError>>,
) -> SelectOutput<IO>
where
    IE: Into<crate::BoxError>,
{
    let incoming_stream_future = async {
        match incoming.try_next().await {
            Ok(Some(stream)) => SelectOutput::Incoming(stream),
            Ok(None) => SelectOutput::Done,
            Err(e) => SelectOutput::TcpErr(e.into()),
        }
    };

    if tasks.is_empty() {
        return incoming_stream_future.await;
    }

    tokio::select! {
        stream = incoming_stream_future => stream,
        accept = tasks.join_next() => {
            match accept.expect("JoinSet should never end") {
                Ok(Ok(io)) => SelectOutput::Io(io),
                Ok(Err(e)) => SelectOutput::TlsErr(e),
                Err(e) => SelectOutput::TlsErr(e.into()),
            }
        }
    }
}

#[cfg(feature = "_tls-any")]
enum SelectOutput<A> {
    Incoming(A),
    Io(ServerIo<A>),
    TcpErr(crate::BoxError),
    TlsErr(crate::BoxError),
    Done,
}
