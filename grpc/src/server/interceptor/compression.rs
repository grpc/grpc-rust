use std::sync::Arc;

use bytes::BufMut;
use bytes::BytesMut;

use crate::StatusCodeError;
use crate::StatusError;
use crate::client::CallOptions;
use crate::codec::compression::Compressor;
use crate::codec::compression::Decompressor;
use crate::codec::compression::registry::CompressionResolver;
use crate::codec::compression::registry::global_compression_registry;
use crate::codec::message::IncomingRawMessage;
use crate::codec::message::RawMessage;
use crate::core::RecvMessage;
use crate::core::RequestHeaders;
use crate::core::ResponseHeaders;
use crate::core::ServerResponseStreamItem;
use crate::core::Trailers;
use crate::metadata::MetadataMap;
use crate::server::Handle;
use crate::server::RecvStream;
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
    registry: Arc<dyn CompressionResolver>,
    decompression_limit: usize,
    default_send_compressor: Option<String>,
}

impl ServerCompressionInterceptor {
    /// Creates a new compression interceptor utilizing the provided codec resolver.
    pub fn new(registry: Arc<dyn CompressionResolver>) -> Self {
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
        Self::new(Arc::new(global_compression_registry()) as Arc<dyn CompressionResolver>)
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
        let decompressor = match resolve_decompressor(&*self.registry, headers.metadata()) {
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
    registry: Arc<dyn CompressionResolver>,
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
            Ok(Some(codec)) => {
                if should_inject_encoding_header && let Ok(val) = codec.name().parse() {
                    headers.metadata_mut().insert(GRPC_ENCODING_HEADER, val);
                }
                Ok(SendCompressorState::Active(ActiveCompressor::new(codec)))
            }
            Ok(None) => Ok(SendCompressorState::Disabled),
            Err(_) => Err(()), // Bubble up registry errors to kill the stream
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
        item: ServerResponseStreamItem<'b>,
        options: SendOptions,
    ) -> Result<(), ()> {
        match item {
            ServerResponseStreamItem::Headers(mut headers) => {
                if let SendCompressorState::Pending(pending) = &self.state {
                    self.state = pending.resolve(&mut headers)?;
                } else {
                    // gRPC strictly allows Initial Metadata (Headers) to be sent only once.
                    // If the state is no longer Pending, Headers were already processed.
                    return Err(());
                }

                self.inner
                    .send(ServerResponseStreamItem::Headers(headers), options)
                    .await
            }
            ServerResponseStreamItem::Message(msg) => {
                let active = match &mut self.state {
                    SendCompressorState::Active(active) => active,
                    SendCompressorState::Disabled => {
                        return self
                            .inner
                            .send(ServerResponseStreamItem::Message(msg), options)
                            .await;
                    }
                    SendCompressorState::Pending(_) => {
                        // gRPC strictly requires headers to precede messages.
                        // If the handler attempts to send a message before headers, abort the stream.
                        return Err(());
                    }
                };

                if options.disable_compression {
                    return self
                        .inner
                        .send(ServerResponseStreamItem::Message(msg), options)
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
                self.inner
                    .send(ServerResponseStreamItem::Message(&raw_msg), options)
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
    registry: &dyn CompressionResolver,
    metadata: &MetadataMap,
) -> Result<Option<Arc<dyn Decompressor>>, ResolverError> {
    let recv_encoding = metadata
        .get(GRPC_ENCODING_HEADER)
        .map(|v| v.to_str())
        .filter(|&enc| enc != IDENTITY_ENCODING);

    if let Some(encoding) = recv_encoding {
        match registry.get_decompressor(encoding) {
            Ok(Some(decompressor)) => Ok(Some(decompressor)),
            Ok(None) => {
                let status = StatusError::new(
                    StatusCodeError::Unimplemented,
                    format!("Compression encoding {} not supported", encoding),
                );
                let accept_encodings = registry.accept_encodings().ok().map(|encs| encs.join(","));
                Err(ResolverError {
                    status,
                    accept_encodings,
                })
            }
            Err(e) => {
                let status = StatusError::new(
                    StatusCodeError::Internal,
                    format!("Compression registry error: {}", e),
                );
                Err(ResolverError {
                    status,
                    accept_encodings: None,
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
        fn name(&self) -> &'static str {
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
        fn name(&self) -> &'static str {
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

    /// A fake compression registry that only supports the "mock" codec.
    struct MockCodecResolver;

    impl CompressionResolver for MockCodecResolver {
        fn get_compressor(&self, name: &str) -> Result<Option<Arc<dyn Compressor>>, String> {
            if name == "mock" {
                Ok(Some(Arc::new(MockCodec)))
            } else {
                Ok(None)
            }
        }

        fn get_decompressor(&self, name: &str) -> Result<Option<Arc<dyn Decompressor>>, String> {
            if name == "mock" {
                Ok(Some(Arc::new(MockCodec)))
            } else {
                Ok(None)
            }
        }

        fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String> {
            Ok(Arc::from(vec!["mock", "identity"].into_boxed_slice()))
        }
    }

    /// A fake network send stream that intercepts and stores outgoing messages
    /// and headers so that tests can assert what was sent back to the client.
    struct MockSendStream {
        messages: Arc<Mutex<Vec<Bytes>>>,
        headers: Arc<Mutex<Option<ResponseHeaders>>>,
    }
    impl SendStream for MockSendStream {
        async fn send<'a>(
            &mut self,
            item: ServerResponseStreamItem<'a>,
            _opts: SendOptions,
        ) -> Result<(), ()> {
            match item {
                ServerResponseStreamItem::Headers(h) => {
                    *self.headers.lock().await = Some(h);
                }
                ServerResponseStreamItem::Message(msg) => {
                    let mut buf = msg.encode().unwrap();
                    self.messages
                        .lock()
                        .await
                        .push(buf.copy_to_bytes(buf.remaining()));
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
                    ServerResponseStreamItem::Headers(ResponseHeaders::new()),
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
                        ServerResponseStreamItem::Message(&StringMsg("echo".into())),
                        SendOptions::default(),
                    )
                    .await;
            }
            Trailers::new(Ok(()))
        }
    }

    /// Helper function to set up all the mock structs and shared state
    /// needed for a single test case.
    #[allow(clippy::type_complexity)]
    fn setup() -> (
        Arc<dyn CompressionResolver>,
        Arc<Mutex<Vec<Bytes>>>,
        Arc<Mutex<Option<ResponseHeaders>>>,
        MockSendStream,
    ) {
        let registry = Arc::new(MockCodecResolver) as Arc<dyn CompressionResolver>;

        let messages = Arc::new(Mutex::new(Vec::new()));
        let headers = Arc::new(Mutex::new(None));
        let tx = MockSendStream {
            messages: messages.clone(),
            headers: headers.clone(),
        };
        (registry, messages, headers, tx)
    }

    #[tokio::test]
    async fn test_unknown_incoming_encoding() {
        let (registry, _, _, mut tx) = setup();
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
        let (registry, messages, _, mut tx) = setup();
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
        let (registry, messages, _, mut tx) = setup();
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

        let (registry, _, _, mut tx) = setup();
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
        let (registry, messages, resp_headers, mut tx) = setup();
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
                        ServerResponseStreamItem::Headers(ResponseHeaders::new()),
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
                let _ = tx
                    .send(ServerResponseStreamItem::Message(&StringMsg), opts)
                    .await;
                Trailers::new(Ok(()))
            }
        }

        let (registry, messages, _, mut tx) = setup();
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

    #[tokio::test]
    async fn test_multi_value_accept_encoding() {
        let (registry, messages, resp_headers, mut tx) = setup();
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
        let (registry, messages, resp_headers, mut tx) = setup();
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
        let (registry, messages, resp_headers, mut tx) = setup();
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
                    .send(
                        ServerResponseStreamItem::Headers(headers),
                        SendOptions::default(),
                    )
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
                        ServerResponseStreamItem::Message(&StringMsg),
                        SendOptions::default(),
                    )
                    .await;
                Trailers::new(Ok(()))
            }
        }

        let (registry, messages, resp_headers, mut tx) = setup();
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
                    .send(
                        ServerResponseStreamItem::Headers(headers),
                        SendOptions::default(),
                    )
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
                        ServerResponseStreamItem::Message(&StringMsg),
                        SendOptions::default(),
                    )
                    .await;
                Trailers::new(Ok(()))
            }
        }

        let (registry, messages, resp_headers, mut tx) = setup();
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

        let (registry, _, _, mut tx) = setup();
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
                        ServerResponseStreamItem::Headers(ResponseHeaders::new()),
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
                    .send(
                        ServerResponseStreamItem::Message(&BadMsg),
                        SendOptions::default(),
                    )
                    .await;
                assert!(res.is_err());
                Trailers::new(Ok(()))
            }
        }

        let (registry, _, _, mut tx) = setup();
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

        let (registry, _, _, mut tx) = setup();
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

        let (registry, _, _, mut tx) = setup();
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
    async fn test_get_decompressor_err() {
        struct MockDecompressorErrorResolver;
        impl CompressionResolver for MockDecompressorErrorResolver {
            fn get_compressor(&self, _name: &str) -> Result<Option<Arc<dyn Compressor>>, String> {
                Ok(None)
            }
            fn get_decompressor(
                &self,
                _name: &str,
            ) -> Result<Option<Arc<dyn Decompressor>>, String> {
                Err("decompressor registry error".into())
            }
            fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String> {
                Ok(Arc::from(vec!["mock", "identity"].into_boxed_slice()))
            }
        }

        let registry = Arc::new(MockDecompressorErrorResolver) as Arc<dyn CompressionResolver>;
        let messages = Arc::new(Mutex::new(Vec::new()));
        let headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream { messages, headers };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = MockHandler.with_interceptor(interceptor);

        let mut req_headers = RequestHeaders::new();
        req_headers
            .metadata_mut()
            .insert(GRPC_ENCODING_HEADER, "mock".parse().unwrap());

        let trailers = chain
            .handle(
                req_headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;

        let err = trailers.status().as_ref().unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert!(
            err.message()
                .contains("Compression registry error: decompressor registry error")
        );
        assert!(
            trailers
                .metadata()
                .get(GRPC_ACCEPT_ENCODING_HEADER)
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_get_compressor_none() {
        struct MockCompressorNoneResolver;
        impl CompressionResolver for MockCompressorNoneResolver {
            fn get_compressor(&self, _name: &str) -> Result<Option<Arc<dyn Compressor>>, String> {
                Ok(None)
            }
            fn get_decompressor(
                &self,
                name: &str,
            ) -> Result<Option<Arc<dyn Decompressor>>, String> {
                if name == "mock" {
                    Ok(Some(Arc::new(MockCodec)))
                } else {
                    Ok(None)
                }
            }
            fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String> {
                Ok(Arc::from(vec!["mock", "identity"].into_boxed_slice()))
            }
        }

        let registry = Arc::new(MockCompressorNoneResolver) as Arc<dyn CompressionResolver>;
        let messages = Arc::new(Mutex::new(Vec::new()));
        let headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream {
            messages: messages.clone(),
            headers: headers.clone(),
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

    #[tokio::test]
    async fn test_get_compressor_err_and_early_return() {
        struct MockCompressorErrorResolver;
        impl CompressionResolver for MockCompressorErrorResolver {
            fn get_compressor(&self, _name: &str) -> Result<Option<Arc<dyn Compressor>>, String> {
                Err("compressor registry error".into())
            }
            fn get_decompressor(
                &self,
                name: &str,
            ) -> Result<Option<Arc<dyn Decompressor>>, String> {
                if name == "mock" {
                    Ok(Some(Arc::new(MockCodec)))
                } else {
                    Ok(None)
                }
            }
            fn accept_encodings(&self) -> Result<Arc<[&'static str]>, String> {
                Ok(Arc::from(vec!["mock", "identity"].into_boxed_slice()))
            }
        }

        struct HeadersTriggerHandler;
        impl Handle for HeadersTriggerHandler {
            async fn handle(
                &self,
                _h: RequestHeaders,
                _o: CallOptions,
                tx: &mut impl SendStream,
                _rx: impl RecvStream + 'static,
            ) -> Trailers {
                // This triggers the delayed resolution which will fail with Err(())
                let res = tx
                    .send(
                        ServerResponseStreamItem::Headers(ResponseHeaders::new()),
                        SendOptions::default(),
                    )
                    .await;
                if res.is_err() {
                    return Trailers::new(Err(StatusError::new(
                        StatusCodeError::Internal,
                        "stream aborted by interceptor",
                    )));
                }
                Trailers::new(Ok(()))
            }
        }

        let registry = Arc::new(MockCompressorErrorResolver) as Arc<dyn CompressionResolver>;
        let messages = Arc::new(Mutex::new(Vec::new()));
        let headers = Arc::new(Mutex::new(None));
        let mut tx = MockSendStream { messages, headers };

        let interceptor = ServerCompressionInterceptor::new(registry);
        let chain = HeadersTriggerHandler.with_interceptor(interceptor);

        let mut req_headers = RequestHeaders::new();
        req_headers
            .metadata_mut()
            .insert(GRPC_ENCODING_HEADER, "mock".parse().unwrap());
        req_headers
            .metadata_mut()
            .insert(GRPC_ACCEPT_ENCODING_HEADER, "mock".parse().unwrap());

        let trailers = chain
            .handle(
                req_headers,
                CallOptions::default(),
                &mut tx,
                MockRecvStream { items: vec![] },
            )
            .await;

        let err = trailers.status().as_ref().unwrap_err();
        assert_eq!(err.code(), StatusCodeError::Internal);
        assert!(err.message().contains("stream aborted by interceptor"));
    }

    #[test]
    fn test_default_interceptor() {
        let _interceptor = ServerCompressionInterceptor::default();
    }
}
