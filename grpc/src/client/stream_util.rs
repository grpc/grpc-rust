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

//! Interceptors providing client-side stream validation.

use crate::StatusCodeError;
use crate::StatusError;
use crate::client::CallOptions;
use crate::client::InvokeOnce;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream;
use crate::client::Trailers;
use crate::client::boxed::BoxRecvStream;
use crate::client::boxed::BoxSendStream;
use crate::client::interceptor::Intercept;
use crate::client::load_balancing::CallOutcome;
use crate::client::load_balancing::CompletionCallback;
use crate::core::ConnectionInfo;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::core::StreamEnded;

/// An interceptor that wraps the underlying invoker's [`RecvStream`] in a
/// [`RecvStreamValidator`].
#[derive(Clone)]
pub struct ResponseValidator {
    unary: bool,
}

impl ResponseValidator {
    /// Creates an instance of a `ResponseValidator` that simply wraps all
    /// invocations' [`RecvStream`s](InvokeOnce::RecvStream) in a
    /// [`RecvStreamValidator`] with `unary` propagated to it.
    pub fn new(unary: bool) -> Self {
        Self { unary }
    }
}

impl<I: InvokeOnce> Intercept<I> for ResponseValidator {
    type SendStream = I::SendStream;
    type RecvStream = RecvStreamValidator<I::RecvStream>;

    async fn intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        next: I,
    ) -> (Self::SendStream, Self::RecvStream) {
        let (tx, rx) = next.invoke_once(headers, options).await;
        (tx, RecvStreamValidator::new(rx, self.unary))
    }
}

/// Wraps a client's [`RecvStream`] and performs protocol validation on it.
pub struct RecvStreamValidator<R> {
    recv_stream: R,
    state: RecvStreamState,
    unary: bool,
}

enum RecvStreamState {
    AwaitingHeaders,
    AwaitingMessagesOrTrailers,
    AwaitingTrailers,
    Done,
}

impl<R> RecvStreamValidator<R>
where
    R: RecvStream,
{
    /// Wraps `recv_stream` and performs protocol validation when it is
    /// accessed.
    ///
    /// If a protocol violation occurs, an error will be synthesized as
    /// [`Trailers`].  Any calls to the [`RecvStream::recv`] method beyond
    /// [`ResponseStreamItem::Trailers`] will not be propagated and will
    /// immediately return [`ResponseStreamItem::StreamClosed`].
    ///
    /// If `unary` is set, expects the server to send exactly one response
    /// message (after headers), or a trailers-only response.
    pub fn new(recv_stream: R, unary: bool) -> Self {
        Self {
            recv_stream,
            state: RecvStreamState::AwaitingHeaders,
            unary,
        }
    }

    /// Sets the state to Done and produces a synthesized trailer item
    /// containing the error message.
    fn error(&mut self, s: impl Into<String>) -> ResponseStreamItem {
        self.state = RecvStreamState::Done;
        ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
            StatusCodeError::Internal,
            s,
        ))))
    }
}

impl<R> RecvStream for RecvStreamValidator<R>
where
    R: RecvStream,
{
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        // Never call the underlying RecvStream if done.
        if matches!(self.state, RecvStreamState::Done) {
            return ResponseStreamItem::StreamClosed;
        }

        let item = self.recv_stream.recv(msg).await;

        match item {
            ResponseStreamItem::Headers(_) => {
                if matches!(self.state, RecvStreamState::AwaitingHeaders) {
                    self.state = RecvStreamState::AwaitingMessagesOrTrailers;
                    item
                } else {
                    self.error("stream received multiple headers")
                }
            }
            ResponseStreamItem::Message => {
                if matches!(self.state, RecvStreamState::AwaitingMessagesOrTrailers) {
                    if self.unary {
                        self.state = RecvStreamState::AwaitingTrailers;
                    }
                    item
                } else if matches!(self.state, RecvStreamState::AwaitingTrailers) {
                    self.error("unary stream received multiple messages")
                } else {
                    self.error("stream received messages without headers")
                }
            }
            ResponseStreamItem::Trailers(t) => {
                if self.unary
                    && !matches!(self.state, RecvStreamState::AwaitingTrailers)
                    && t.status().is_ok()
                {
                    return self.error("unary stream received zero messages");
                }
                // Always return a trailers result immediately - it is valid any
                // time but sets the stream's state to Done.
                self.state = RecvStreamState::Done;
                ResponseStreamItem::Trailers(t)
            }
            ResponseStreamItem::StreamClosed => {
                // Trailers were never received or we would be Done.
                self.error("stream ended without trailers")
            }
        }
    }
}

struct NopSendStream;

impl SendStream for NopSendStream {
    async fn send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        Err(StreamEnded)
    }
}

pub(crate) struct FailingRecvStream {
    status: Option<StatusError>,
    connection_info: Option<ConnectionInfo>,
}

impl RecvStream for FailingRecvStream {
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        match self.status.take() {
            Some(status) => ResponseStreamItem::Trailers(
                Trailers::new(Err(status)).with_connection_info(self.connection_info.take()),
            ),
            None => ResponseStreamItem::StreamClosed,
        }
    }
}

impl FailingRecvStream {
    pub(crate) fn new_stream_pair(
        status: StatusError,
        connection_info: Option<ConnectionInfo>,
    ) -> (BoxSendStream, BoxRecvStream) {
        (
            BoxSendStream::new(NopSendStream),
            BoxRecvStream::new(Self {
                status: Some(status),
                connection_info,
            }),
        )
    }
}

/// Wraps a client's [`RecvStream`] and invokes an LB policy's
/// [`CompletionCallback`] (from [`Pick::on_complete`]) exactly once when the
/// RPC completes.
///
/// The RPC is considered complete when trailers are received, in which case
/// the callback receives the trailers' status and metadata. If the stream is
/// closed without trailers, or dropped before completing (e.g. the RPC was
/// cancelled or abandoned by the application), the callback receives an error
/// status and no trailing metadata.
///
/// Note that an application that drops the stream after reading its last
/// message but before reading trailers is reported as `CANCELLED`, so LB
/// policies (e.g. for LRS load reports) count such RPCs as failed. This
/// matches gRPC C-core, where the call is likewise cancelled if the
/// application stops before receiving the status.
///
/// [`Pick::on_complete`]: crate::client::load_balancing::Pick::on_complete
pub(crate) struct CompletionRecvStream<R> {
    recv_stream: R,
    on_complete: Option<CompletionCallback>,
}

impl<R> CompletionRecvStream<R> {
    pub(crate) fn new(recv_stream: R, on_complete: CompletionCallback) -> Self {
        Self {
            recv_stream,
            on_complete: Some(on_complete),
        }
    }

    fn complete(&mut self, outcome: CallOutcome<'_>) {
        if let Some(on_complete) = self.on_complete.take() {
            on_complete(&outcome);
        }
    }

    fn complete_with_error(&mut self, code: StatusCodeError, message: &str) {
        if self.on_complete.is_some() {
            let status = Err(StatusError::new(code, message));
            self.complete(CallOutcome::new(&status));
        }
    }
}

impl<R> RecvStream for CompletionRecvStream<R>
where
    R: RecvStream,
{
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        let item = self.recv_stream.recv(msg).await;
        match &item {
            ResponseStreamItem::Trailers(trailers) => self.complete(
                CallOutcome::new(trailers.status()).with_trailing_metadata(trailers.metadata()),
            ),
            ResponseStreamItem::StreamClosed => self
                .complete_with_error(StatusCodeError::Internal, "stream closed without trailers"),
            ResponseStreamItem::Headers(_) | ResponseStreamItem::Message => {}
        }
        item
    }
}

impl<R> Drop for CompletionRecvStream<R> {
    fn drop(&mut self) {
        self.complete_with_error(
            StatusCodeError::Cancelled,
            "RPC abandoned before completion",
        );
    }
}

#[cfg(test)]
mod test {
    use std::mem::discriminant;
    use std::vec;

    use super::*;
    use crate::client::ResponseHeaders;
    use crate::client::interceptor::InvokeOnceExt as _;
    use crate::client::test_util::MockInvoker;
    use crate::client::test_util::NopRecvMessage;

    // Tests that an error occurs if messages are received before headers.
    #[tokio::test]
    async fn test_validator_messages_before_headers() {
        let scenarios = [vec![ResponseStreamItem::Message]];

        for scenario in scenarios {
            validate_scenario(
                &scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    "received messages without headers",
                )))),
                false,
            )
            .await;
        }
    }

    // Tests that an error occurs if StreamClosed is received early.
    #[tokio::test]
    async fn test_validator_stream_closed_before_trailers() {
        let scenarios = [
            vec![ResponseStreamItem::StreamClosed],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::StreamClosed,
            ],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Message,
                ResponseStreamItem::StreamClosed,
            ],
        ];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    "ended without trailers",
                )))),
                false,
            )
            .await;
        }
    }

    // Tests that an error occurs if headers are received twice.
    #[tokio::test]
    async fn test_validator_headers_repeated() {
        let scenarios = [
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
            ],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Message,
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
            ],
        ];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    "received multiple headers",
                )))),
                false,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_unary_ok_without_message() {
        let scenarios = [
            vec![ResponseStreamItem::Trailers(Trailers::new(Ok(())))],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
            ],
        ];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    "received zero messages",
                )))),
                true,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_unary_multiple_messages() {
        let scenarios = [vec![
            ResponseStreamItem::Headers(ResponseHeaders::new(crate::core::test_connection_info())),
            ResponseStreamItem::Message,
            ResponseStreamItem::Message,
        ]];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    "received multiple messages",
                )))),
                true,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_successful_stream() {
        let scenarios = [vec![
            ResponseStreamItem::Headers(ResponseHeaders::new(crate::core::test_connection_info())),
            ResponseStreamItem::Message,
            ResponseStreamItem::Message,
            ResponseStreamItem::Message,
            ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
        ]];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
                false,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_erroring_stream() {
        let scenarios = [vec![
            ResponseStreamItem::Headers(ResponseHeaders::new(crate::core::test_connection_info())),
            ResponseStreamItem::Message,
            ResponseStreamItem::Message,
            ResponseStreamItem::Message,
            ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                StatusCodeError::Aborted,
                "some err",
            )))),
        ]];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Aborted,
                    "some err",
                )))),
                false,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_successful_unary() {
        let scenarios = [vec![
            ResponseStreamItem::Headers(ResponseHeaders::new(crate::core::test_connection_info())),
            ResponseStreamItem::Message,
            ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
        ]];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Ok(()))),
                true,
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_validator_erroring_unary() {
        let scenarios = [
            vec![ResponseStreamItem::Trailers(Trailers::new(Err(
                StatusError::new(StatusCodeError::Aborted, "some err"),
            )))],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Aborted,
                    "some err",
                )))),
            ],
            vec![
                ResponseStreamItem::Headers(ResponseHeaders::new(
                    crate::core::test_connection_info(),
                )),
                ResponseStreamItem::Message,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Aborted,
                    "some err",
                )))),
            ],
        ];

        for scenario in &scenarios {
            validate_scenario(
                scenario,
                ResponseStreamItem::Trailers(Trailers::new(Err(StatusError::new(
                    StatusCodeError::Aborted,
                    "some err",
                )))),
                true,
            )
            .await;
        }
    }

    async fn validate_scenario(
        scenario: &[ResponseStreamItem],
        expect: ResponseStreamItem,
        unary: bool,
    ) {
        let (invoker, mut tx) = MockInvoker::new();
        let invoker = invoker.with_interceptor(ResponseValidator::new(unary));
        let (_, recv_stream) = invoker
            .invoke_once(RequestHeaders::default(), CallOptions::default())
            .await;

        let mut validator = RecvStreamValidator::new(recv_stream, unary);
        // Send all but the last item, verifying it is returned by the
        // validator.
        for item in &scenario[..scenario.len() - 1] {
            tx.send_resp(item.clone()).await;
            let got = validator.recv(&mut NopRecvMessage).await;
            // Assert that the item sent is the same type as the item received.
            println!("{got:?} vs {item:?}");
            assert_eq!(discriminant(&got), discriminant(item));
        }
        // Send the final item.
        tx.send_resp(scenario[scenario.len() - 1].clone()).await;
        let got = validator.recv(&mut NopRecvMessage).await;
        assert!(matches!(&got, expect));
        if let ResponseStreamItem::Trailers(got_t) = got {
            let ResponseStreamItem::Trailers(expect_t) = expect else {
                unreachable!(); // per matches check above
            };
            if expect_t.status().is_ok() {
                assert!(got_t.status().is_ok());
            } else {
                // Assert the codes match.
                assert_eq!(
                    got_t.status().as_ref().unwrap_err().code(),
                    expect_t.status().as_ref().unwrap_err().code()
                );
                // Assert the status received contains the expected status error message.
                assert!(
                    got_t
                        .status()
                        .as_ref()
                        .unwrap_err()
                        .message()
                        .contains(expect_t.status().as_ref().unwrap_err().message())
                );
            }
        }
    }

    /// What a completion callback observed: the error code (`None` for OK)
    /// and whether trailing metadata was provided.
    type Observed = std::sync::Arc<std::sync::Mutex<Vec<(Option<StatusCodeError>, bool)>>>;

    fn recording_callback() -> (CompletionCallback, Observed) {
        let observed = Observed::default();
        let observed_clone = observed.clone();
        let cb: CompletionCallback = Box::new(move |outcome| {
            let code = outcome.status().as_ref().err().map(|e| e.code());
            observed_clone
                .lock()
                .unwrap()
                .push((code, outcome.trailing_metadata().is_some()));
        });
        (cb, observed)
    }

    // Tests that the completion callback runs exactly once with the trailers'
    // status and metadata when trailers are received, and not again on later
    // reads or when the stream is dropped.
    #[tokio::test]
    async fn test_completion_recv_stream_completes_on_trailers() {
        let (cb, observed) = recording_callback();
        let (_, rx) = FailingRecvStream::new_stream_pair(
            StatusError::new(StatusCodeError::Unavailable, "fail"),
            None,
        );
        let mut stream = CompletionRecvStream::new(rx, cb);
        assert!(observed.lock().unwrap().is_empty());

        let item = stream.recv(&mut NopRecvMessage).await;
        assert!(matches!(item, ResponseStreamItem::Trailers(_)));
        assert_eq!(
            *observed.lock().unwrap(),
            vec![(Some(StatusCodeError::Unavailable), true)]
        );

        let item = stream.recv(&mut NopRecvMessage).await;
        assert!(matches!(item, ResponseStreamItem::StreamClosed));
        drop(stream);
        assert_eq!(observed.lock().unwrap().len(), 1);
    }

    // Tests that the completion callback runs with CANCELLED and no trailing
    // metadata when the stream is dropped before the RPC completes.
    #[tokio::test]
    async fn test_completion_recv_stream_completes_on_drop() {
        let (cb, observed) = recording_callback();
        let (_, rx) = FailingRecvStream::new_stream_pair(
            StatusError::new(StatusCodeError::Unavailable, "fail"),
            None,
        );
        let stream = CompletionRecvStream::new(rx, cb);
        assert!(observed.lock().unwrap().is_empty());
        drop(stream);
        assert_eq!(
            *observed.lock().unwrap(),
            vec![(Some(StatusCodeError::Cancelled), false)]
        );
    }
}
