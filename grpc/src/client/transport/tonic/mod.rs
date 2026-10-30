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

use std::error::Error;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::OnceLock;
use std::task::Context;
use std::task::Poll;

use bytes::Buf;
use bytes::BufMut as _;
use bytes::Bytes;
use http::Request as HttpRequest;
use http::Response as HttpResponse;
use http::Uri;
use http::uri::PathAndQuery;
use hyper::client::conn::http2::Builder;
use hyper::client::conn::http2::SendRequest;
use tokio::sync::Notify;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
use tonic::CancellationHandle;
use tonic::Code;
use tonic::Request as TonicRequest;
use tonic::Status as TonicStatus;
use tonic::Streaming;
use tonic::body::Body;
use tonic::client::Grpc;
use tonic::client::GrpcService;
use tonic::codec::Codec;
use tonic::codec::Decoder;
use tonic::codec::EncodeBuf;
use tonic::codec::Encoder;
use tonic::metadata::MetadataMap as TonicMeta;
use tower::ServiceBuilder;
use tower::buffer::Buffer;
use tower::buffer::future::ResponseFuture as BufferResponseFuture;
use tower::limit::ConcurrencyLimitLayer;
use tower::limit::RateLimitLayer;
use tower::util::BoxService;
use tower_service::Service as TowerService;

use crate::StatusCodeError;
use crate::StatusError;
use crate::attributes::Attributes;
use crate::byte_str::ByteStr;
use crate::client::CallOptions;
use crate::client::Invoke;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream;
use crate::client::Trailers;
use crate::client::name_resolution::TCP_IP_NETWORK_TYPE;
use crate::client::name_resolution::UNIX_NETWORK_TYPE;
use crate::client::transport::SecurityOpts;
use crate::client::transport::Transport;
use crate::client::transport::TransportOptions;
use crate::client::transport::registry::GLOBAL_TRANSPORT_REGISTRY;
use crate::core::Address;
use crate::core::ConnectionInfo;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::core::StreamEnded;
use crate::private;
use crate::rt::BoxedTaskHandle;
use crate::rt::GrpcRuntime;
use crate::rt::TcpOptions;
use crate::rt::UnixSocketOptions;
use crate::rt::hyper_wrapper::HyperCompatExec;
use crate::rt::hyper_wrapper::HyperCompatTimer;
use crate::rt::hyper_wrapper::HyperStream;

#[cfg(test)]
mod test;

const DEFAULT_BUFFER_SIZE: usize = 1024;

type BoxError = Box<dyn Error + Send + Sync>;
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type BoxStream<T> = Pin<Box<dyn Stream<Item = Result<T, TonicStatus>> + Send>>;
type BoxBuf = Box<dyn Buf + Send + Sync>;

pub(crate) fn reg() {
    GLOBAL_TRANSPORT_REGISTRY.add_transport(
        TCP_IP_NETWORK_TYPE,
        TransportBuilder {
            network_type: NetworkType::Tcp,
        },
    );
    GLOBAL_TRANSPORT_REGISTRY.add_transport(
        UNIX_NETWORK_TYPE,
        TransportBuilder {
            network_type: NetworkType::Unix,
        },
    );
}

#[derive(Debug, Copy, Clone)]
enum NetworkType {
    Tcp,
    Unix,
}

struct TransportBuilder {
    network_type: NetworkType,
}

struct TonicTransport {
    grpc: Grpc<TonicService>,
    task_handle: BoxedTaskHandle,
    runtime: GrpcRuntime,
    connection_info: ConnectionInfo,
}

impl Drop for TonicTransport {
    fn drop(&mut self) {
        self.task_handle.abort();
    }
}

impl Invoke for TonicTransport {
    type SendStream = TonicSendStream;
    type RecvStream = TonicRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        let (req_tx, req_rx) = mpsc::channel(1);
        let send_failure = Arc::new(SendFailure::default());
        let request_stream = RequestStream {
            rx: ReceiverStream::new(req_rx),
            send_failure: send_failure.clone(),
        };
        let mut request = TonicRequest::new(Box::pin(request_stream));
        let (method, metadata) = headers.into_parts();
        *request.metadata_mut() = metadata.into();

        let cancel_tx = request.cancellation_handle();

        let Ok(path) = PathAndQuery::from_maybe_shared(method) else {
            return self
                .local_err_streams(StatusError::new(StatusCodeError::Internal, "invalid path"));
        };

        let mut grpc = self.grpc.clone();
        if let Err(e) = grpc.ready().await {
            return self.local_err_streams(StatusError::new(
                StatusCodeError::Unavailable,
                format!("Service was not ready: {e}"),
            ));
        }

        // Note that Tonic's streaming call blocks until the server's headers
        // are received.  The client needs a SendStream to provide the request
        // message(s), which the server may be awaiting before sending its
        // headers.  So, we spawn a task for this period of time, and then we
        // send the response (headers, stream) to the TonicRecvStream when it is
        // available.
        let (resp_tx, resp_rx) = oneshot::channel();
        self.runtime.spawn(Box::pin(async move {
            let response = grpc.streaming(request, path, BufCodec {}).await;
            let _ = resp_tx.send(response);
        }));

        let call = Arc::new(CallState {
            cancel_tx,
            send_failure,
        });
        (
            TonicSendStream {
                state: SendState::Open {
                    tx: req_tx,
                    call: call.clone(),
                },
            },
            TonicRecvStream {
                state: StreamState::AwaitingHeaders(resp_rx),
                call: Some(call),
                connection_info: Some(self.connection_info.clone()),
            },
        )
    }
}

impl TonicTransport {
    /// Creates a send/recv stream pair representing locally-produced errors.
    fn local_err_streams(&self, status: StatusError) -> (TonicSendStream, TonicRecvStream) {
        (
            TonicSendStream {
                state: SendState::Closed,
            },
            TonicRecvStream {
                state: StreamState::LocalError(status),
                call: None,
                connection_info: Some(self.connection_info.clone()),
            },
        )
    }
}

/// An error from a call's send stream, which the call's recv stream reports as
/// the call's status.
#[derive(Default)]
struct SendFailure {
    error: OnceLock<StatusError>,
    notify: Notify,
}

impl SendFailure {
    /// Records `error`.  Only the first error recorded is kept.
    fn set(&self, error: StatusError) {
        let _ = self.error.set(error);
        self.notify.notify_one();
    }

    fn get(&self) -> Option<&StatusError> {
        self.error.get()
    }

    /// Waits for an error to be recorded.
    async fn wait(&self) -> StatusError {
        loop {
            if let Some(error) = self.error.get() {
                return error.clone();
            }
            // notify_one stores a permit if it runs before this wait starts.
            self.notify.notified().await;
        }
    }
}

/// The request message stream given to tonic.  Once the send stream fails the
/// call, it never ends, so the request body cannot end normally before tonic
/// observes the cancellation.
///
/// TODO: remove once https://github.com/grpc/grpc-rust/pull/2985 lands.
struct RequestStream {
    rx: ReceiverStream<BoxBuf>,
    send_failure: Arc<SendFailure>,
}

impl Stream for RequestStream {
    type Item = BoxBuf;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<BoxBuf>> {
        match Pin::new(&mut self.rx).poll_next(cx) {
            // The send stream records the failure before it drops the sender.
            Poll::Ready(None) if self.send_failure.get().is_some() => Poll::Pending,
            poll => poll,
        }
    }
}

/// State shared by the send and recv streams of a call.
struct CallState {
    cancel_tx: CancellationHandle,
    send_failure: Arc<SendFailure>,
}

impl CallState {
    fn cancel(&self) {
        self.cancel_tx.clone().cancel();
    }

    /// Fails the call with an error from the send stream.
    fn fail_send(&self, status: StatusError) {
        self.send_failure.set(status);
        self.cancel();
    }
}

struct TonicSendStream {
    state: SendState,
}

enum SendState {
    Open {
        tx: mpsc::Sender<BoxBuf>,
        call: Arc<CallState>,
    },
    Closed,
}

impl SendState {
    /// Fails the call with an error from the send stream and closes it.
    fn fail(&mut self, status: StatusError) {
        let SendState::Open { tx, call } = std::mem::replace(self, SendState::Closed) else {
            return;
        };
        // RequestStream relies on the failure being recorded before the
        // request channel closes.
        call.fail_send(status);
        drop(tx);
    }
}

impl SendStream for TonicSendStream {
    async fn send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        let SendState::Open { tx, .. } = &self.state else {
            return Err(StreamEnded);
        };
        let buf = match msg.encode() {
            Ok(buf) => buf,
            Err(e) => {
                self.state.fail(StatusError::new(
                    StatusCodeError::Internal,
                    format!("error encoding request: {e}"),
                ));
                return Err(StreamEnded);
            }
        };
        if tx.send(buf).await.is_err() {
            self.state = SendState::Closed;
            return Err(StreamEnded);
        }
        if options.final_msg {
            self.state = SendState::Closed;
        }
        Ok(())
    }
}

struct TonicRecvStream {
    state: StreamState,
    call: Option<Arc<CallState>>,
    connection_info: Option<ConnectionInfo>,
}

impl TonicRecvStream {
    // Converts from a tonic status to a trailers stream item.
    fn trailers_from_tonic_status(
        &mut self,
        status: &TonicStatus,
        mut md: TonicMeta,
    ) -> ResponseStreamItem {
        if !status.details().is_empty() {
            md.insert_bin(
                "grpc-status-details-bin",
                tonic::metadata::MetadataValue::from_bytes(status.details()),
            );
        }
        // Tonic reports the cancellation caused by a send failure as Cancelled,
        // with the h2 error as its source.  A Cancelled status sent by the
        // server has no source.
        let send_error = self
            .call
            .as_ref()
            .and_then(|call| call.send_failure.get())
            .filter(|_| status.code() == Code::Cancelled && status.source().is_some());
        let status_res = match (status.code(), send_error) {
            (_, Some(send_error)) => Err(send_error.clone()),
            (Code::Ok, None) => Ok(()),
            (code, None) => Err(StatusError::new(
                StatusCodeError::from(code as i32),
                status.message(),
            )),
        };
        self.trailers_from_grpc_result(status_res, Some(&md))
    }

    // Builds a trailers stream item with a status.
    fn trailers_from_grpc_result(
        &mut self,
        status: crate::Result<()>,
        md: Option<&TonicMeta>,
    ) -> ResponseStreamItem {
        if let Some(call) = self.call.take() {
            call.cancel();
        }
        let trailers = if let Some(md) = md {
            match md.try_into() {
                Err(e) => Trailers::new(Err(StatusError::new(
                    StatusCodeError::Internal,
                    format!("failed to parse metadata: {e}"),
                ))),
                Ok(metadata) => Trailers::new(status).with_metadata(metadata),
            }
        } else {
            Trailers::new(status)
        };
        ResponseStreamItem::Trailers(trailers.with_connection_info(self.connection_info.take()))
    }
}

enum StreamState {
    LocalError(StatusError),
    AwaitingHeaders(oneshot::Receiver<Result<tonic::Response<Streaming<Bytes>>, TonicStatus>>),
    Streaming(Streaming<Bytes>),
    Closed,
}

impl RecvStream for TonicRecvStream {
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        let Some(call) = self.call.clone() else {
            return self.recv_state(msg).await;
        };
        // After a send failure, items already received (including the
        // server's status) are still returned; otherwise the send failure ends
        // the call.  Tonic only observes the cancellation from a send failure
        // once hyper polls the request body, which it does not do while the
        // stream is blocked on flow control or on the server's stream limit, so
        // recv_state alone may never complete.  See
        // https://github.com/grpc/grpc-rust/issues/2983.
        tokio::select! {
            biased;
            item = self.recv_state(msg) => item,
            err = call.send_failure.wait() => {
                self.state = StreamState::Closed;
                self.trailers_from_grpc_result(Err(err), None)
            }
        }
    }
}

impl TonicRecvStream {
    async fn recv_state(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        // Take the current state, leaving `Closed` in its place temporarily
        let state = std::mem::replace(&mut self.state, StreamState::Closed);

        match state {
            // Closed is terminal.
            StreamState::Closed => ResponseStreamItem::StreamClosed,
            // Stay closed after sending trailers (do not set self.state).
            StreamState::LocalError(error) => self.trailers_from_grpc_result(Err(error), None),
            StreamState::AwaitingHeaders(rx) => match rx.await {
                Ok(Ok(response)) => {
                    let (metadata, stream, _extensions) = response.into_parts();
                    // Tonic decodes base64-encoded binary headers lazily. It
                    // does not fail the RPC upon receiving invalid base64 data;
                    // the error only surfaces when the application attempts to
                    // read the metadata.
                    // In contrast, standard gRPC implementations eagerly decode
                    // these headers and immediately fail the RPC with an
                    // Internal status.
                    match (&metadata).try_into() {
                        Ok(md) => {
                            // Start streaming and return the headers.
                            self.state = StreamState::Streaming(stream);
                            let Some(connection_info) = self.connection_info.take() else {
                                return self.trailers_from_grpc_result(
                                    Err(StatusError::new(
                                        StatusCodeError::Internal,
                                        "required connection info missing",
                                    )),
                                    None,
                                );
                            };
                            let headers = ResponseHeaders::new(connection_info).with_metadata(md);
                            ResponseStreamItem::Headers(headers)
                        }
                        Err(e) => self.trailers_from_grpc_result(
                            Err(StatusError::new(
                                StatusCodeError::Internal,
                                format!("error decoding response: {e}"),
                            )),
                            None,
                        ),
                    }
                }
                Err(_) => {
                    // Stay closed after sending trailers (do not set self.state).
                    self.trailers_from_grpc_result(
                        Err(StatusError::new(StatusCodeError::Unknown, "Task cancelled")),
                        None,
                    )
                }
                Ok(Err(mut status)) => {
                    // In a Trailers-only response, the tonic status contains
                    // the metadata.
                    // Stay closed after sending trailers (do not set self.state).
                    let md = std::mem::take(status.metadata_mut());
                    self.trailers_from_tonic_status(&status, md)
                }
            },
            StreamState::Streaming(mut stream) => match stream.message().await {
                Ok(Some(mut buf)) => match msg.decode(&mut buf) {
                    Ok(()) => {
                        // More messages may remain in the stream; set receiver
                        // again.
                        self.state = StreamState::Streaming(stream);
                        ResponseStreamItem::Message
                    }
                    Err(e) => self.trailers_from_grpc_result(
                        Err(StatusError::new(
                            StatusCodeError::Internal,
                            format!("error decoding response: {e}"),
                        )),
                        None,
                    ),
                },
                Err(mut status) => {
                    // Stay closed after sending trailers (do not set self.state).
                    let md = std::mem::take(status.metadata_mut());
                    self.trailers_from_tonic_status(&status, md)
                }
                Ok(None) => {
                    // Stay closed after sending trailers (do not set self.state).
                    let trailers = stream.trailers().await;
                    let md = trailers.unwrap_or_default().unwrap_or_default();
                    self.trailers_from_grpc_result(Ok(()), Some(&md))
                }
            },
        }
    }
}

impl Drop for TonicRecvStream {
    fn drop(&mut self) {
        if let Some(call) = self.call.take() {
            call.cancel();
        }
    }
}

impl Transport for TransportBuilder {
    type Service = TonicTransport;

    async fn connect(
        &self,
        address: &Address,
        runtime: GrpcRuntime,
        security_info: &SecurityOpts,
        opts: &TransportOptions,
    ) -> Result<
        (
            Self::Service,
            ConnectionInfo,
            oneshot::Receiver<Result<(), String>>,
        ),
        String,
    > {
        let runtime = runtime.clone();
        let mut settings = Builder::<HyperCompatExec>::new(HyperCompatExec {
            inner: runtime.clone(),
        })
        .timer(HyperCompatTimer {
            inner: runtime.clone(),
        })
        .initial_stream_window_size(opts.init_stream_window_size)
        .initial_connection_window_size(opts.init_connection_window_size)
        .adaptive_window(opts.http2_adaptive_window)
        .keep_alive_interval(opts.http2_keep_alive_interval)
        .clone();

        if let Some(val) = opts.http2_keep_alive_timeout {
            settings.keep_alive_timeout(val);
        }

        if let Some(val) = opts.http2_keep_alive_while_idle {
            settings.keep_alive_while_idle(val);
        }

        if let Some(val) = opts.http2_max_header_list_size {
            settings.max_header_list_size(val);
        }

        let transport_fut = match self.network_type {
            NetworkType::Tcp => {
                let addr: SocketAddr =
                    SocketAddr::from_str(&address.address).map_err(|err| err.to_string())?;
                runtime.tcp_stream(
                    addr,
                    TcpOptions {
                        enable_nodelay: opts.tcp_nodelay,
                        keepalive: opts.tcp_keepalive,
                    },
                )
            }
            NetworkType::Unix => runtime.unix_stream(
                PathBuf::from(&*address.address),
                UnixSocketOptions::default(),
            ),
        };
        let transport = transport_fut.await?;
        let credentials = &security_info.credentials;
        let handshake_ouput = credentials
            .connect(
                &security_info.authority,
                transport,
                &security_info.handshake_info,
                &runtime,
                private::Internal,
            )
            .await?;

        let local_address = Address {
            network_type: handshake_ouput.endpoint.get_network_type(),
            address: ByteStr::from(handshake_ouput.endpoint.get_local_address().to_string()),
            attributes: Attributes::new(),
        };
        let remote_address = Address {
            network_type: handshake_ouput.endpoint.get_network_type(),
            address: ByteStr::from(handshake_ouput.endpoint.get_peer_address().to_string()),
            attributes: Attributes::new(),
        };

        let transport = HyperStream::new(handshake_ouput.endpoint);

        let (sender, connection) = settings
            .handshake(transport)
            .await
            .map_err(|err| err.to_string())?;
        let (tx, rx) = oneshot::channel();

        let task_handle = runtime.spawn(Box::pin(async move {
            if let Err(err) = connection.await {
                let _ = tx.send(Err(err.to_string()));
            } else {
                let _ = tx.send(Ok(()));
            }
        }));
        let sender = SendRequestWrapper::from(sender);

        let service = ServiceBuilder::new()
            .option_layer(opts.concurrency_limit.map(ConcurrencyLimitLayer::new))
            .option_layer(opts.rate_limit.map(|(l, d)| RateLimitLayer::new(l, d)))
            .map_err(Into::<BoxError>::into)
            .service(sender);

        let service = BoxService::new(service);
        let (service, worker) = Buffer::pair(service, DEFAULT_BUFFER_SIZE);
        runtime.spawn(Box::pin(worker));
        let authority = &security_info.authority.host_port_string();
        let uri = Uri::from_maybe_shared(format!("http://{}", authority))
            .map_err(|e| format!("failed to create URL with authority {}: {}", authority, e))?;
        let grpc = Grpc::with_origin(TonicService { inner: service }, uri);

        let connection_info = ConnectionInfo::new(
            local_address,
            remote_address,
            handshake_ouput.security_info.clone(),
        );

        let service = TonicTransport {
            grpc,
            task_handle,
            runtime,
            connection_info: connection_info.clone(),
        };
        Ok((service, connection_info, rx))
    }
}

struct SendRequestWrapper {
    inner: SendRequest<Body>,
}

impl From<SendRequest<Body>> for SendRequestWrapper {
    fn from(inner: SendRequest<Body>) -> Self {
        Self { inner }
    }
}

impl TowerService<HttpRequest<Body>> for SendRequestWrapper {
    type Response = HttpResponse<Body>;
    type Error = BoxError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        let fut = self.inner.send_request(req);
        Box::pin(async move { fut.await.map_err(Into::into).map(|res| res.map(Body::new)) })
    }
}

#[derive(Clone)]
struct TonicService {
    inner: Buffer<http::Request<Body>, BoxFuture<'static, Result<http::Response<Body>, BoxError>>>,
}

impl GrpcService<Body> for TonicService {
    type ResponseBody = Body;
    type Error = BoxError;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tower::Service::poll_ready(&mut self.inner, cx)
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        ResponseFuture {
            inner: tower::Service::call(&mut self.inner, request),
        }
    }
}

/// A future that resolves to an HTTP response.
///
/// This is returned by the `Service::call` on [`Channel`].
pub(crate) struct ResponseFuture {
    inner: BufferResponseFuture<BoxFuture<'static, Result<HttpResponse<Body>, BoxError>>>,
}

impl Future for ResponseFuture {
    type Output = Result<http::Response<Body>, BoxError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}

pub(crate) struct BufCodec {}

impl Codec for BufCodec {
    type Encode = BoxBuf;
    type Decode = Bytes;
    type Encoder = BufEncoder;
    type Decoder = BytesDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        BufEncoder {}
    }

    fn decoder(&mut self) -> Self::Decoder {
        BytesDecoder {}
    }
}

pub struct BufEncoder {}

impl Encoder for BufEncoder {
    type Item = BoxBuf;
    type Error = TonicStatus;

    fn encode(&mut self, mut item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        dst.put(&mut *item);
        Ok(())
    }
}

#[derive(Debug)]
pub struct BytesDecoder {}

impl Decoder for BytesDecoder {
    type Item = Bytes;
    type Error = TonicStatus;

    fn decode(
        &mut self,
        src: &mut tonic::codec::DecodeBuf<'_>,
    ) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(src.copy_to_bytes(src.remaining())))
    }
}
