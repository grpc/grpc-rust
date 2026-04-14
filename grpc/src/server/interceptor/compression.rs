use std::sync::Arc;

use bytes::BufMut;
use bytes::BytesMut;

use crate::StatusCodeError;
use crate::StatusError;
use crate::client::CallOptions;
use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;
use crate::codec::compression::registry::CompressionRegistry;
use crate::codec::message::IncomingRawMessage;
use crate::codec::message::RawMessage;
use crate::core::RecvMessage;
use crate::core::RequestHeaders;
use crate::core::ResponseHeaders;
use crate::core::Trailers;
use crate::metadata::MetadataMap;
use crate::server::Handle;
use crate::server::RecvStream;
use crate::server::ResponseStreamItem;
use crate::server::SendOptions;
use crate::server::SendStream;
use crate::server::interceptor::Intercept;

const DEFAULT_DECOMPRESSION_LIMIT: usize = 4 * 1024 * 1024;
const INITIAL_COMPRESSION_BUFFER_CAPACITY: usize = 8192;

const GRPC_ENCODING_HEADER: &str = "grpc-encoding";
const GRPC_ACCEPT_ENCODING_HEADER: &str = "grpc-accept-encoding";
const IDENTITY_ENCODING: &str = "identity";

/// A gRPC server interceptor that manages automatic payload compression and
/// decompression based on client headers and server registry capabilities.
///
/// # Examples
///
/// ```rust,ignore
/// use std::sync::Arc;
/// use tonic_server_grpc::codec::compression::global_codec_registry;
/// use tonic_server_grpc::server::interceptor::compression::ServerCompressionInterceptor;
///
/// let resolver = Arc::new(global_codec_registry());
/// let interceptor = ServerCompressionInterceptor::new(resolver)
///     .with_decompression_limit(8 * 1024 * 1024);
/// ```
#[derive(Clone)]
pub struct ServerCompressionInterceptor {
    registry: CompressionRegistry,
    decompression_limit: usize,
    default_send_compressor: Option<String>,
}

impl ServerCompressionInterceptor {
    /// Creates a new compression interceptor using the provided registry.
    pub fn new(registry: CompressionRegistry) -> Self {
        Self {
            registry,
            decompression_limit: DEFAULT_DECOMPRESSION_LIMIT,
            default_send_compressor: None,
        }
    }

    /// Configures a custom byte ceiling for decompression bomb mitigation.
    pub fn with_decompression_limit(mut self, limit: usize) -> Self {
        self.decompression_limit = limit;
        self
    }

    /// Sets a global default compressor to use for responses if the application handler does not specify one.
    pub fn with_default_send_compressor(mut self, encoding: &str) -> Self {
        self.default_send_compressor = Some(encoding.to_string());
        self
    }
}

impl Default for ServerCompressionInterceptor {
    fn default() -> Self {
        Self::new(CompressionRegistry::global())
    }
}

impl Intercept for ServerCompressionInterceptor {
    async fn intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        tx: &mut impl SendStream,
        rx: impl RecvStream + 'static,
        next: &impl Handle,
    ) -> Trailers {
        let decompressor = match resolve_decompressor(&self.registry, headers.metadata()) {
            Ok(d) => d,
            Err(err) => {
                let mut trailers = Trailers::new(Err(err.status));
                if let Some(accept_str) = &err.accept_encodings
                    && let Ok(val) = accept_str.parse()
                {
                    trailers
                        .metadata_mut()
                        .insert(GRPC_ACCEPT_ENCODING_HEADER, val);
                }
                return trailers;
            }
        };

        let accepted_encodings = headers
            .metadata()
            .get_all(GRPC_ACCEPT_ENCODING_HEADER)
            .iter()
            .map(|v| v.to_str())
            .flat_map(|v| v.split(','))
            .map(str::trim)
            .map(String::from)
            .collect::<Vec<_>>();

        let request_encoding = headers
            .metadata()
            .get(GRPC_ENCODING_HEADER)
            .map(|v| v.to_str())
            .map(String::from);

        let fallback_encoding = self.default_send_compressor.clone().or(request_encoding);

        let pending = PendingNegotiation {
            registry: self.registry.clone(),
            accepted_encodings,
            fallback_encoding,
        };

        let mut wrapped_tx = CompressedSendStream::new(tx, pending);

        let active_rx = decompressor.map(|codec| ActiveDecompressor {
            codec,
            buf: BytesMut::with_capacity(INITIAL_COMPRESSION_BUFFER_CAPACITY),
        });

        let wrapped_rx = DecompressedRecvStream {
            inner: rx,
            decompression_limit: self.decompression_limit,
            active: active_rx,
        };

        next.handle(headers, options, &mut wrapped_tx, wrapped_rx)
            .await
    }
}

/// State wrapper for an active stream compressor.
///
/// Holds the compressor implementation and a buffer used to incrementally
/// compress outbound gRPC messages. The buffer is retained to avoid reallocation
/// between messages.
struct ActiveCompressor {
    codec: Arc<dyn Compressor>,
    buf: BytesMut,
}

impl ActiveCompressor {
    fn new(codec: Arc<dyn Compressor>) -> Self {
        Self {
            codec,
            buf: BytesMut::with_capacity(INITIAL_COMPRESSION_BUFFER_CAPACITY),
        }
    }
}

struct PendingNegotiation {
    registry: CompressionRegistry,
    accepted_encodings: Vec<String>,
    fallback_encoding: Option<String>,
}

impl PendingNegotiation {
    /// Resolves the final state for the compressor based on outbound headers.
    fn resolve(&self, headers: &mut ResponseHeaders) -> Result<SendCompressorState, ()> {
        let Some((enc, should_inject_encoding_header)) = self.negotiate_encoding(headers) else {
            return Ok(SendCompressorState::Disabled);
        };

        match self.registry.get_compressor(&enc) {
            Some(codec) => {
                if should_inject_encoding_header && let Ok(val) = codec.name().parse() {
                    headers.metadata_mut().insert(GRPC_ENCODING_HEADER, val);
                }
                Ok(SendCompressorState::Active(ActiveCompressor::new(codec)))
            }
            None => Ok(SendCompressorState::Disabled),
        }
    }

    /// Determines which encoding to use and whether it needs to be injected.
    fn negotiate_encoding(&self, headers: &mut ResponseHeaders) -> Option<(String, bool)> {
        // 1. Check if handler provided a valid override
        if let Some(enc) = Self::get_handler_encoding(headers) {
            if self.accepted_encodings.contains(&enc) {
                return Some((enc, false));
            }
            // Lenient conflict resolution: strip the invalid header
            // and send uncompressed response.
            headers.metadata_mut().remove(GRPC_ENCODING_HEADER);
            return None;
        }

        // 2. Check fallback (global default or symmetric)
        if let Some(enc) = &self.fallback_encoding
            && self.accepted_encodings.contains(enc)
        {
            return Some((enc.clone(), true));
        }
        None
    }

    fn get_handler_encoding(headers: &ResponseHeaders) -> Option<String> {
        headers
            .metadata()
            .get(GRPC_ENCODING_HEADER)
            .map(|v| v.to_str())
            .filter(|&enc| enc != IDENTITY_ENCODING)
            .map(String::from)
    }
}

enum SendCompressorState {
    Pending(PendingNegotiation),
    Active(ActiveCompressor),
    Disabled,
}

/// Transparent stream adapter that intercepts outbound messages and applies compression.
///
/// If a compressor is negotiated and active, each message is compressed before being sent
/// to the underlying transport.
struct CompressedSendStream<'a, S: SendStream> {
    inner: &'a mut S,
    state: SendCompressorState,
}

impl<'a, S: SendStream> CompressedSendStream<'a, S> {
    fn new(inner: &'a mut S, pending: PendingNegotiation) -> Self {
        Self {
            inner,
            state: SendCompressorState::Pending(pending),
        }
    }
}

impl<'a, S: SendStream> SendStream for CompressedSendStream<'a, S> {
    async fn send<'b>(
        &mut self,
        item: ResponseStreamItem<'b>,
        options: SendOptions,
    ) -> Result<(), ()> {
        match item {
            ResponseStreamItem::Headers(mut headers) => {
                if let SendCompressorState::Pending(pending) = &self.state {
                    self.state = pending.resolve(&mut headers)?;
                } else {
                    // gRPC strictly allows Initial Metadata (Headers) to be sent only once.
                    // If the state is no longer Pending, Headers were already processed.
                    return Err(());
                }

                self.inner
                    .send(ResponseStreamItem::Headers(headers), options)
                    .await
            }
            ResponseStreamItem::Message(msg) => {
                let active = match &mut self.state {
                    SendCompressorState::Active(active) => active,
                    SendCompressorState::Disabled => {
                        let mut options = options;
                        options.disable_compression = true;
                        return self
                            .inner
                            .send(ResponseStreamItem::Message(msg), options)
                            .await;
                    }
                    SendCompressorState::Pending(_) => {
                        // gRPC strictly requires headers to precede messages.
                        // If the handler attempts to send a message before headers, abort the stream.
                        return Err(());
                    }
                };

                if options.disable_compression {
                    // disable_compression is already true — message is uncompressed.
                    return self
                        .inner
                        .send(ResponseStreamItem::Message(msg), options)
                        .await;
                }

                let mut buf = msg.encode().map_err(|_| ())?;
                // TODO: Implement capacity shrinking to avoid memory leaks on long-lived streams.
                // If capacity is excessive (e.g. > 8MB), replace with a new `BytesMut` instead.
                active.buf.clear();
                active
                    .codec
                    .compress(&mut *buf, &mut active.buf)
                    .map_err(|_| ())?;

                let raw_msg = RawMessage::from_buf(active.buf.split().freeze());
                // Signal downstream that this message is compressed.
                // TODO(sauravz): disable_compression defaults to false, so messages
                // that bypass the compression interceptor entirely will also appear
                // as "compressed" to the framing layer. Consider adding an explicit
                // is_compressed field to SendOptions once a breaking change is feasible.
                let mut options = options;
                options.disable_compression = false;
                self.inner
                    .send(ResponseStreamItem::Message(&raw_msg), options)
                    .await
            }
        }
    }
}

/// State wrapper for an active stream decompressor.
///
/// Holds the decompressor implementation and a buffer used to incrementally
/// decompress inbound gRPC messages. The buffer is retained to avoid reallocation
/// between messages.
struct ActiveDecompressor {
    codec: Arc<dyn Decompressor>,
    buf: BytesMut,
}

/// Transparent stream adapter that intercepts inbound messages and applies decompression.
///
/// If an `ActiveDecompressor` is present, each message is decompressed before being yielded
/// to the application handler.
struct DecompressedRecvStream<R: RecvStream> {
    inner: R,
    decompression_limit: usize,
    active: Option<ActiveDecompressor>,
}

impl<R: RecvStream> RecvStream for DecompressedRecvStream<R> {
    /// Fetches the next incoming gRPC message, destructuring the raw buffer directly.
    async fn next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>> {
        if let Some(active) = &mut self.active {
            let mut raw_msg = IncomingRawMessage::new();
            let res = self.inner.next(&mut raw_msg).await?;
            if res.is_err() {
                return Some(Err(()));
            }

            let (mut source_buf, is_compressed) = raw_msg.into_parts();

            if is_compressed {
                // TODO: Implement capacity shrinking to avoid memory leaks on long-lived streams.
                // If capacity is excessive (e.g. > 8MB), replace with a new `BytesMut` instead.
                active.buf.clear();
                let mut limited_dst = (&mut active.buf).limit(self.decompression_limit);
                if active
                    .codec
                    .decompress(&mut *source_buf, &mut limited_dst)
                    .is_err()
                {
                    return Some(Err(()));
                }
                let mut payload = active.buf.split().freeze();
                if msg.decode(&mut payload).is_err() {
                    return Some(Err(()));
                }
            } else if msg.decode(&mut *source_buf).is_err() {
                return Some(Err(()));
            }
            Some(Ok(()))
        } else {
            self.inner.next(msg).await
        }
    }
}

/// Bundled error context returned by pure codec resolvers, holding both the gRPC
/// status and any optional pushback trailer context (like supported encodings).
#[derive(Debug)]
struct ResolverError {
    /// The primary gRPC status error (e.g., Unimplemented or Internal).
    status: StatusError,
    /// An optional comma-separated string of supported encodings, to be attached
    /// as the `grpc-accept-encoding` trailing header upon Unimplemented errors.
    accept_encodings: Option<String>,
}

/// Inspects incoming headers to determine if the client encoded the request.
///
/// If `grpc-encoding` is present and is not `identity`, this function queries the registry
/// for an appropriate decompressor. If the encoding is unsupported, an error is returned.
fn resolve_decompressor(
    registry: &CompressionRegistry,
    metadata: &MetadataMap,
) -> Result<Option<Arc<dyn Decompressor>>, ResolverError> {
    let recv_encoding = metadata
        .get(GRPC_ENCODING_HEADER)
        .map(|v| v.to_str())
        .filter(|&enc| enc != IDENTITY_ENCODING);

    if let Some(encoding) = recv_encoding {
        match registry.get_decompressor(encoding) {
            Some(decompressor) => Ok(Some(decompressor)),
            None => {
                let status = StatusError::new(
                    StatusCodeError::Unimplemented,
                    format!("Compression encoding {} not supported", encoding),
                );
                let accept_encodings = Some(registry.accept_encodings().join(","));
                Err(ResolverError {
                    status,
                    accept_encodings,
                })
            }
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bytes::Buf;
    use bytes::Bytes;
    use tokio::sync::Mutex;

    use super::*;
    use crate::codec::compression::registry::CompressionRegistryBuilder;
    use crate::core::ResponseHeaders;
    use crate::core::SendMessage;
    use crate::core::Trailers;
    use crate::server::interceptor::HandleExt;

    /// A fake compressor/decompressor used for testing.
    /// It "compresses" by prepending `[compressed]` to the payload,
    /// and "decompresses" by verifying and stripping that prefix.
    #[derive(Debug, Clone, Copy)]
    struct MockCodec;

    impl Compressor for MockCodec {
        fn name(&self) -> &str {
            "mock"
        }
        fn compress(
            &self,
            src: &mut dyn Buf,
            dst: &mut dyn bytes::buf::BufMut,
        ) -> Result<(), String> {
            dst.put_slice(b"[compressed]");
            dst.put_slice(&src.copy_to_bytes(src.remaining()));
            Ok(())
        }
    }

    impl Decompressor for MockCodec {
        fn name(&self) -> &str {
            "mock"
        }
        fn decompress(
            &self,
            src: &mut dyn Buf,
            dst: &mut dyn bytes::buf::BufMut,
        ) -> Result<(), String> {
            let bytes = src.copy_to_bytes(src.remaining());
            if bytes.starts_with(b"[compressed]") {
                let payload = &bytes[12..];
                if dst.remaining_mut() < payload.len() {
                    return Err("limit reached".to_string());
                }
                dst.put_slice(payload);
                Ok(())
            } else {
                Err("not compressed".to_string())
            }
        }
    }

    /// Builds a compression registry that only supports the "mock" codec.
    fn mock_registry() -> CompressionRegistry {
        CompressionRegistryBuilder::new()
            .register_compressor(Arc::new(MockCodec))
            .register_decompressor(Arc::new(MockCodec))
            .build()
    }

    /// A fake network send stream that intercepts and stores outgoing messages
    /// and headers so that tests can assert what was sent back to the client.
    struct MockSendStream {
        messages: Arc<Mutex<Vec<Bytes>>>,
        headers: Arc<Mutex<Option<ResponseHeaders>>>,
        /// Captures the `disable_compression` value for each message sent.
        msg_disable_compression: Arc<Mutex<Vec<bool>>>,
    }
    impl SendStream for MockSendStream {
        async fn send<'a>(
            &mut self,
            item: ResponseStreamItem<'a>,
            opts: SendOptions,
        ) -> Result<(), ()> {
            match item {
                ResponseStreamItem::Headers(h) => {
                    *self.headers.lock().await = Some(h);
                }
                ResponseStreamItem::Message(msg) => {
                    let mut buf = msg.encode().unwrap();
                    self.messages
                        .lock()
                        .await
                        .push(buf.copy_to_bytes(buf.remaining()));
                    self.msg_disable_compression
                        .lock()
                        .await
                        .push(opts.disable_compression);
                }
            }
            Ok(())
        }
    }

    /// A fake network receive stream that yields hardcoded byte arrays
    /// to simulate incoming client messages.
    struct MockRecvStream {
        items: Vec<Result<Bytes, ()>>,
    }
    impl RecvStream for MockRecvStream {
        async fn next(&mut self, msg: &mut dyn RecvMessage) -> Option<Result<(), ()>> {
            if self.items.is_empty() {
                return None;
            }
            let item = self.items.remove(0);
            match item {
                Ok(bytes) => {
                    let is_compressed =
                        bytes.starts_with(b"[compressed]") || bytes.as_ref() == b"bad payload";
                    if let Some(raw_msg) = msg.downcast_mut::<IncomingRawMessage>() {
                        raw_msg.set_compressed(is_compressed);
                    }
                    let mut buf = bytes;
                    if msg.decode(&mut buf).is_err() {
                        Some(Err(()))
                    } else {
                        Some(Ok(()))
                    }
                }
                Err(()) => Some(Err(())),
            }
        }
    }

    /// A simple mock gRPC service handler that echoes back the string "echo".
    struct MockHandler;
    impl Handle for MockHandler {
        async fn handle(
            &self,
            _headers: RequestHeaders,
            _options: CallOptions,
            tx: &mut impl SendStream,
            mut rx: impl RecvStream + 'static,
        ) -> Trailers {
            let _ = tx
                .send(
                    ResponseStreamItem::Headers(ResponseHeaders::new()),
                    SendOptions::default(),
                )
                .await;
            struct StringMsg(String);
            impl RecvMessage for StringMsg {
                fn decode(&mut self, data: &mut dyn Buf) -> Result<(), String> {
                    let b = data.copy_to_bytes(data.remaining());
                    self.0 = String::from_utf8(b.to_vec()).unwrap();
                    Ok(())
                }
            }
            impl SendMessage for StringMsg {
                fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                    Ok(Box::new(Bytes::from(self.0.clone())))
                }
            }

            while let Some(Ok(())) = rx.next(&mut StringMsg(String::new())).await {
                let _ = tx
                    .send(
                        ResponseStreamItem::Message(&StringMsg("echo".into())),
                        SendOptions::default(),
                    )
                    .await;
            }
            Trailers::new(Ok(()))
        }
    }

    #[tokio::test]
    async fn test_unknown_incoming_encoding() {
        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert(GRPC_ENCODING_HEADER, "unknown".parse().unwrap());

        let trailers = chain
            .handle(
                headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            StatusCodeError::Unimplemented
        );
        assert_eq!(
            trailers
                .metadata()
                .get(GRPC_ACCEPT_ENCODING_HEADER)
                .expect("Expected grpc-accept-encoding trailer to be present")
                .to_str(),
            "mock,identity"
        );
    }

    #[tokio::test]
    async fn test_identity_incoming_encoding() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "identity".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());
        assert_eq!(messages.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn test_supported_incoming_encoding() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());
        assert_eq!(messages.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn test_decompression_failure() {
        struct FailingDecompHandler;
        impl Handle for FailingDecompHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                _tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                struct StringMsg;
                impl RecvMessage for StringMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Ok(())
                    }
                }
                let res = rx.next(&mut StringMsg).await;
                assert!(matches!(res, Some(Err(()))));
                Trailers::new(Err(StatusError::new(
                    crate::status::StatusCodeError::Internal,
                    "decompression failed",
                )))
            }
        }

        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = FailingDecompHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"bad payload"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            crate::status::StatusCodeError::Internal
        );
    }

    #[tokio::test]
    async fn test_supported_outgoing_encoding() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let h = resp_headers.lock().await.take().unwrap();
        assert_eq!(h.metadata().get("grpc-encoding").unwrap().to_str(), "mock");

        let msgs = messages.lock().await;
        assert!(msgs[0].starts_with(b"[compressed]"));
    }

    #[tokio::test]
    async fn test_disable_compression_option() {
        struct DisableHandler;
        impl Handle for DisableHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                _rx: impl RecvStream + 'static,
            ) -> Trailers {
                let _ = tx
                    .send(
                        ResponseStreamItem::Headers(ResponseHeaders::new()),
                        SendOptions::default(),
                    )
                    .await;

                struct StringMsg;
                impl SendMessage for StringMsg {
                    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                        Ok(Box::new(Bytes::from_static(b"echo")))
                    }
                }
                let opts = SendOptions {
                    disable_compression: true,
                    ..Default::default()
                };
                let _ = tx.send(ResponseStreamItem::Message(&StringMsg), opts).await;
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = DisableHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());

        let _ = chain
            .handle(
                headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;

        let msgs = messages.lock().await;
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));
    }

    /// Verifies that when compression is active and applied, the compression
    /// interceptor sets `disable_compression = false` to signal downstream
    /// that the message payload is compressed.
    #[tokio::test]
    async fn test_compressed_message_signals_disable_compression_false() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let msg_disable_compression = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: msg_disable_compression.clone(),
        };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let msgs = messages.lock().await;
        assert_eq!(msgs.len(), 1);
        assert!(msgs[0].starts_with(b"[compressed]"));

        let flags = msg_disable_compression.lock().await;
        assert_eq!(flags.len(), 1);
        assert!(
            !flags[0],
            "disable_compression should be false for a compressed message"
        );
    }

    /// Verifies that when compression is disabled (no grpc-accept-encoding),
    /// the compression interceptor sets `disable_compression = true` to signal
    /// downstream that the message payload is NOT compressed.
    #[tokio::test]
    async fn test_disabled_state_signals_disable_compression_true() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let msg_disable_compression = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: msg_disable_compression.clone(),
        };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        // No grpc-accept-encoding → compression disabled
        let headers = RequestHeaders::new();
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let msgs = messages.lock().await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));

        let flags = msg_disable_compression.lock().await;
        assert_eq!(flags.len(), 1);
        assert!(
            flags[0],
            "disable_compression should be true when compression is disabled"
        );
    }

    /// Verifies that when compression is negotiated but disabled per-message
    /// via SendOptions, the compression interceptor preserves
    /// `disable_compression = true` to signal that this particular message
    /// is NOT compressed.
    #[tokio::test]
    async fn test_per_message_disable_signals_disable_compression_true() {
        struct PerMsgDisableHandler;
        impl Handle for PerMsgDisableHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                _rx: impl RecvStream + 'static,
            ) -> Trailers {
                let _ = tx
                    .send(
                        ResponseStreamItem::Headers(ResponseHeaders::new()),
                        SendOptions::default(),
                    )
                    .await;

                struct EchoMsg;
                impl SendMessage for EchoMsg {
                    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                        Ok(Box::new(Bytes::from_static(b"echo")))
                    }
                }
                let opts = SendOptions {
                    disable_compression: true,
                    ..Default::default()
                };
                let _ = tx.send(ResponseStreamItem::Message(&EchoMsg), opts).await;
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let msg_disable_compression = Arc::new(Mutex::new(Vec::new()));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: msg_disable_compression.clone(),
        };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = PerMsgDisableHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());

        let trailers = chain
            .handle(
                headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;
        assert!(trailers.status().is_ok());

        let msgs = messages.lock().await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));

        let flags = msg_disable_compression.lock().await;
        assert_eq!(flags.len(), 1);
        assert!(
            flags[0],
            "disable_compression should be true when per-message compression is disabled"
        );
    }

    #[tokio::test]
    async fn test_multi_value_accept_encoding() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers.metadata_mut().insert(
            "grpc-accept-encoding",
            "gzip, mock, identity".parse().unwrap(),
        );
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let h = resp_headers.lock().await.take().unwrap();
        assert_eq!(h.metadata().get("grpc-encoding").unwrap().to_str(), "mock");

        let msgs = messages.lock().await;
        assert!(msgs[0].starts_with(b"[compressed]"));
    }

    #[tokio::test]
    async fn test_missing_accept_encoding() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let headers = RequestHeaders::new(); // No accept-encoding header
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let h = resp_headers.lock().await.take().unwrap();
        assert!(h.metadata().get("grpc-encoding").is_none());

        let msgs = messages.lock().await;
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));
    }

    #[tokio::test]
    async fn test_asymmetric_compression_global_default() {
        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor =
            ServerCompressionInterceptor::new(registry).with_default_send_compressor("mock");
        let chain = MockHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        // Request is uncompressed (no grpc-encoding), but client accepts "mock"
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());

        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let h = resp_headers.lock().await.take().unwrap();
        assert_eq!(h.metadata().get("grpc-encoding").unwrap().to_str(), "mock");

        let msgs = messages.lock().await;
        assert!(msgs[0].starts_with(b"[compressed]"));
    }

    #[tokio::test]
    async fn test_asymmetric_compression_handler_override() {
        struct OverrideHandler;
        impl Handle for OverrideHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                let mut headers = ResponseHeaders::new();
                headers
                    .metadata_mut()
                    .insert("grpc-encoding", "mock".parse().unwrap());
                let _ = tx
                    .send(ResponseStreamItem::Headers(headers), SendOptions::default())
                    .await;

                struct StringMsg;
                impl SendMessage for StringMsg {
                    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                        Ok(Box::new(Bytes::from_static(b"echo")))
                    }
                }
                impl RecvMessage for StringMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Ok(())
                    }
                }
                let _ = rx.next(&mut StringMsg).await;
                let _ = tx
                    .send(
                        ResponseStreamItem::Message(&StringMsg),
                        SendOptions::default(),
                    )
                    .await;
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = OverrideHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        // Request is uncompressed, but client accepts "mock"
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        let h = resp_headers.lock().await.take().unwrap();
        assert_eq!(h.metadata().get("grpc-encoding").unwrap().to_str(), "mock");

        let msgs = messages.lock().await;
        assert!(msgs[0].starts_with(b"[compressed]"));
    }

    #[tokio::test]
    async fn test_asymmetric_compression_invalid_handler_override() {
        struct OverrideHandler;
        impl Handle for OverrideHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                let mut headers = ResponseHeaders::new();
                // Handler tries to force "mock", but client won't accept it.
                headers
                    .metadata_mut()
                    .insert("grpc-encoding", "mock".parse().unwrap());
                let _ = tx
                    .send(ResponseStreamItem::Headers(headers), SendOptions::default())
                    .await;

                struct StringMsg;
                impl SendMessage for StringMsg {
                    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                        Ok(Box::new(Bytes::from_static(b"echo")))
                    }
                }
                impl RecvMessage for StringMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Ok(())
                    }
                }
                let _ = rx.next(&mut StringMsg).await;
                let _ = tx
                    .send(
                        ResponseStreamItem::Message(&StringMsg),
                        SendOptions::default(),
                    )
                    .await;
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let resp_headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: resp_headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = OverrideHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        // Client ONLY accepts gzip
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "gzip".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"hello"))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert!(trailers.status().is_ok());

        // The interceptor should have stripped the invalid "mock" header!
        let h = resp_headers.lock().await.take().unwrap();
        assert!(h.metadata().get("grpc-encoding").is_none());

        // And the message should NOT be compressed.
        let msgs = messages.lock().await;
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));
    }

    #[tokio::test]
    async fn test_underlying_stream_error_propagation() {
        struct ErrorPropHandler;
        impl Handle for ErrorPropHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                _tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                struct StringMsg;
                impl RecvMessage for StringMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Ok(())
                    }
                }
                let res = rx.next(&mut StringMsg).await;
                assert!(matches!(res, Some(Err(()))));
                Trailers::new(Err(StatusError::new(
                    crate::status::StatusCodeError::Internal,
                    "propagated",
                )))
            }
        }

        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = ErrorPropHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Err(())],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            crate::status::StatusCodeError::Internal
        );
    }

    #[tokio::test]
    async fn test_compression_encoding_failure() {
        struct FailingEncodeHandler;
        impl Handle for FailingEncodeHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                _rx: impl RecvStream + 'static,
            ) -> Trailers {
                // Send headers first to transition state machine to Active
                let _ = tx
                    .send(
                        ResponseStreamItem::Headers(ResponseHeaders::new()),
                        SendOptions::default(),
                    )
                    .await;

                struct BadMsg;
                impl SendMessage for BadMsg {
                    fn encode(&self) -> Result<Box<dyn Buf + Send + Sync>, String> {
                        Err("encode failed".into())
                    }
                }
                let res = tx
                    .send(ResponseStreamItem::Message(&BadMsg), SendOptions::default())
                    .await;
                assert!(res.is_err());
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = FailingEncodeHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        headers
            .metadata_mut()
            .insert("grpc-accept-encoding", "mock".parse().unwrap());

        let _ = chain
            .handle(
                headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;
    }

    #[tokio::test]
    async fn test_post_decompression_decoding_failure() {
        struct FailingDecodeHandler;
        impl Handle for FailingDecodeHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                _tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                struct BadMsg;
                impl RecvMessage for BadMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Err("decode failed".into())
                    }
                }
                let res = rx.next(&mut BadMsg).await;
                assert!(matches!(res, Some(Err(()))));
                Trailers::new(Ok(()))
            }
        }

        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = FailingDecodeHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]valid bytes"))],
        };

        let _ = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
    }

    #[tokio::test]
    async fn test_decompression_limit_exceeded() {
        struct LimitHandler;
        impl Handle for LimitHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                _tx: &mut impl SendStream,
                mut rx: impl RecvStream + 'static,
            ) -> Trailers {
                struct StringMsg;
                impl RecvMessage for StringMsg {
                    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
                        Ok(())
                    }
                }
                let res = rx.next(&mut StringMsg).await;
                assert!(matches!(res, Some(Err(()))));
                Trailers::new(Err(StatusError::new(
                    crate::status::StatusCodeError::Internal,
                    "limit exceeded",
                )))
            }
        }

        let registry = mock_registry();
        let mut tx = MockSendStream {
            messages: Arc::new(Mutex::new(Vec::new())),
            headers: Arc::new(Mutex::new(None)),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };
        let interceptor = ServerCompressionInterceptor::new(registry).with_decompression_limit(3);
        let chain = LimitHandler.with_interceptor(interceptor);

        let mut headers = RequestHeaders::new();
        headers
            .metadata_mut()
            .insert("grpc-encoding", "mock".parse().unwrap());
        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(
                b"[compressed]long payload exceeding limit",
            ))],
        };

        let trailers = chain
            .handle(headers, CallOptions::default(), &mut tx, rx)
            .await;
        assert_eq!(
            trailers.status().as_ref().unwrap_err().code(),
            crate::status::StatusCodeError::Internal
        );
    }

    #[tokio::test]
    async fn test_get_compressor_none() {
        // A registry that can decompress "mock" but has no matching compressor,
        // so the response send path is disabled (uncompressed) while the request
        // is still decompressed.
        let registry = CompressionRegistryBuilder::new()
            .register_decompressor(Arc::new(MockCodec))
            .build();
        let messages = Arc::new(Mutex::new(Vec::new()));
        let headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: headers.clone(),
            msg_disable_compression: Arc::new(Mutex::new(Vec::new())),
        };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut req_headers = RequestHeaders::new();
        req_headers
            .metadata_mut()
            .insert(GRPC_ENCODING_HEADER, "mock".parse().unwrap());
        req_headers
            .metadata_mut()
            .insert(GRPC_ACCEPT_ENCODING_HEADER, "mock".parse().unwrap());

        let rx = MockRecvStream {
            items: vec![Ok(Bytes::from_static(b"[compressed]hello"))],
        };

        let trailers = chain
            .handle(req_headers, CallOptions::default(), &mut tx, rx)
            .await;

        assert!(trailers.status().is_ok());

        let h = headers.lock().await.take().unwrap();
        assert!(h.metadata().get(GRPC_ENCODING_HEADER).is_none());

        let msgs = messages.lock().await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], Bytes::from_static(b"echo"));
    }

    #[test]
    fn test_default_interceptor() {
        let _interceptor = ServerCompressionInterceptor::default();
    }
}
