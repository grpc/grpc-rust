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

#![cfg(feature = "__unstable")]

use std::future::Future;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use bytes::Buf;
use grpc::__unstable::UnstableToken;
use grpc::__unstable::UnstableTrait;
use grpc::__unstable::client::name_resolution::TCP_IP_NETWORK_TYPE;
use grpc::__unstable::client::transport::GLOBAL_TRANSPORT_REGISTRY;
use grpc::__unstable::client::transport::SecurityOpts;
use grpc::__unstable::client::transport::Transport;
use grpc::__unstable::client::transport::TransportOptions;
use grpc::__unstable::metadata::from_headers;
use grpc::__unstable::metadata::into_headers;
use grpc::__unstable::rt::BoxFuture;
use grpc::__unstable::rt::DnsResolver;
use grpc::__unstable::rt::EndpointIoStream;
use grpc::__unstable::rt::EndpointListener;
use grpc::__unstable::rt::GrpcEndpoint;
use grpc::__unstable::rt::ResolverOptions;
use grpc::__unstable::rt::Runtime;
use grpc::__unstable::rt::StreamEndpoint;
use grpc::__unstable::rt::TcpOptions;
use grpc::__unstable::rt::UnixSocketOptions;
use grpc::__unstable::rt::tcp_stream;
use grpc::client::CallOptions;
use grpc::client::Channel;
use grpc::client::Invoke;
use grpc::client::RecvStream;
use grpc::client::RequestHeaders;
use grpc::client::ResponseStreamItem;
use grpc::client::SendOptions;
use grpc::client::SendStream;
use grpc::client::Trailers;
use grpc::core::Address;
use grpc::core::ConnectionInfo;
use grpc::core::RecvMessage;
use grpc::core::SendMessage;
use grpc::core::StreamEnded;
use grpc::credentials::LocalChannelCredentials;
use grpc::metadata::MetadataMap;
use grpc::metadata::MetadataValue;
use grpc::rt::BoxedTaskHandle;
use grpc::rt::GrpcRuntime;
use grpc::rt::Sleep;
use grpc::rt::TaskHandle;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::oneshot;

struct TestTaskHandle(tokio::task::JoinHandle<()>);

impl UnstableTrait for TestTaskHandle {}

impl TaskHandle for TestTaskHandle {
    fn abort(&self) {
        self.0.abort();
    }
}

struct TestSleep(Pin<Box<tokio::time::Sleep>>);

impl Future for TestSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.0.as_mut().poll(cx)
    }
}

impl UnstableTrait for TestSleep {}

impl Sleep for TestSleep {}

struct TestDnsResolver;

#[grpc::async_trait]
impl DnsResolver for TestDnsResolver {
    async fn lookup_host_name(&self, name: &str) -> Result<Vec<IpAddr>, String> {
        let ip = name.parse::<IpAddr>().map_err(|e| e.to_string())?;
        Ok(vec![ip])
    }

    async fn lookup_txt(&self, _name: &str) -> Result<Vec<String>, String> {
        Err("TXT lookup not supported in test runtime".to_string())
    }
}

#[derive(Debug)]
struct TestRuntime;

#[allow(clippy::disallowed_methods)]
impl Runtime for TestRuntime {
    fn spawn(&self, task: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> BoxedTaskHandle {
        Box::new(TestTaskHandle(tokio::spawn(task)))
    }

    fn get_dns_resolver(&self, _opts: ResolverOptions) -> Result<Box<dyn DnsResolver>, String> {
        Ok(Box::new(TestDnsResolver))
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>> {
        Box::pin(TestSleep(Box::pin(tokio::time::sleep(duration))))
    }

    fn now(&self) -> Instant {
        tokio::time::Instant::now().into_std()
    }

    fn tcp_stream(
        &self,
        target: SocketAddr,
        opts: TcpOptions,
    ) -> BoxFuture<Result<Box<dyn GrpcEndpoint>, String>> {
        Box::pin(async move {
            let stream = TcpStream::connect(target)
                .await
                .map_err(|e| e.to_string())?;
            stream
                .set_nodelay(opts.enable_nodelay())
                .map_err(|e| e.to_string())?;
            let local = stream
                .local_addr()
                .map_err(|e| e.to_string())?
                .to_string()
                .into_boxed_str();
            let peer = stream
                .peer_addr()
                .map_err(|e| e.to_string())?
                .to_string()
                .into_boxed_str();
            Ok(Box::new(StreamEndpoint::new(
                stream,
                local,
                peer,
                TCP_IP_NETWORK_TYPE,
            )) as Box<dyn GrpcEndpoint>)
        })
    }

    fn tcp_listener(
        &self,
        _addr: SocketAddr,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        unimplemented!()
    }

    fn unix_listener(
        &self,
        _path: PathBuf,
        _opts: UnixSocketOptions,
    ) -> BoxFuture<Result<Box<dyn EndpointListener>, String>> {
        unimplemented!()
    }
}

fn test_runtime() -> GrpcRuntime {
    GrpcRuntime::new(TestRuntime, UnstableToken)
}

struct CustomTransportBuilder {
    connects: Arc<AtomicUsize>,
}

struct CustomTransport {
    connection_info: ConnectionInfo,
    _disconnect_tx: oneshot::Sender<Result<(), String>>,
}

struct NopSendStream;

impl SendStream for NopSendStream {
    async fn send(
        &mut self,
        _msg: &dyn SendMessage,
        _options: SendOptions,
    ) -> Result<(), StreamEnded> {
        Ok(())
    }
}

struct EchoTrailersRecvStream {
    connection_info: Option<ConnectionInfo>,
    metadata: Option<MetadataMap>,
}

impl RecvStream for EchoTrailersRecvStream {
    async fn recv(&mut self, _msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        let Some(metadata) = self.metadata.take() else {
            return ResponseStreamItem::StreamClosed;
        };
        ResponseStreamItem::Trailers(
            Trailers::new(Ok(()))
                .with_metadata(metadata)
                .with_connection_info(self.connection_info.take()),
        )
    }
}

impl Transport for CustomTransportBuilder {
    type Service = CustomTransport;

    async fn connect(
        &self,
        address: &Address,
        runtime: GrpcRuntime,
        security_opts: &SecurityOpts,
        opts: &TransportOptions,
    ) -> Result<
        (
            Self::Service,
            ConnectionInfo,
            oneshot::Receiver<Result<(), String>>,
        ),
        String,
    > {
        self.connects.fetch_add(1, Ordering::SeqCst);

        let addr = SocketAddr::from_str(&address.address).map_err(|e| e.to_string())?;
        let endpoint = tcp_stream(
            &runtime,
            addr,
            TcpOptions::default()
                .with_enable_nodelay(opts.tcp_nodelay)
                .with_keepalive(opts.tcp_keepalive),
        )
        .await?;

        let handshake = security_opts
            .credentials
            .connect(
                &security_opts.authority,
                endpoint,
                &security_opts.handshake_info,
                &runtime,
                UnstableToken,
            )
            .await?;

        let mut local_address = Address::default();
        local_address.network_type = handshake.endpoint.get_network_type();
        local_address.address = handshake.endpoint.get_local_address().to_string().into();

        let mut remote_address = Address::default();
        remote_address.network_type = handshake.endpoint.get_network_type();
        remote_address.address = handshake.endpoint.get_peer_address().to_string().into();

        let connection_info =
            ConnectionInfo::new(local_address, remote_address, handshake.security_info);
        let _io = EndpointIoStream::new(handshake.endpoint);
        let (tx, rx) = oneshot::channel();
        Ok((
            CustomTransport {
                connection_info: connection_info.clone(),
                _disconnect_tx: tx,
            },
            connection_info,
            rx,
        ))
    }
}

impl Invoke for CustomTransport {
    type SendStream = NopSendStream;
    type RecvStream = EchoTrailersRecvStream;

    async fn invoke(
        &self,
        mut headers: RequestHeaders,
        _options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        let http_headers = into_headers(std::mem::take(headers.metadata_mut()));
        let roundtripped = from_headers(&http_headers).unwrap();
        (
            NopSendStream,
            EchoTrailersRecvStream {
                connection_info: Some(self.connection_info.clone()),
                metadata: Some(roundtripped),
            },
        )
    }
}

struct NopRecvMessage;

impl RecvMessage for NopRecvMessage {
    fn decode(&mut self, _data: &mut dyn Buf) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn custom_transport_overrides_global_tcp_transport() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let accept_task = tokio::spawn(async move { listener.accept().await.unwrap() });

    let connects = Arc::new(AtomicUsize::new(0));
    GLOBAL_TRANSPORT_REGISTRY.add_transport(
        TCP_IP_NETWORK_TYPE,
        CustomTransportBuilder {
            connects: connects.clone(),
        },
    );

    // Building a Channel must preserve the registered custom transport instead
    // of overwriting it with the default tonic transport.
    let channel = Channel::builder(
        format!("dns:///{addr}"),
        Arc::new(LocalChannelCredentials::new()),
        test_runtime(),
    )
    .build();

    let mut metadata = MetadataMap::new();
    metadata.insert("x-custom-ascii", MetadataValue::from_static("hello"));
    metadata.insert_bin("x-custom-bin", MetadataValue::from_bytes(b"\x01\x02\x03"));
    let headers = RequestHeaders::new()
        .with_method_name("/test.Service/Method")
        .with_metadata(metadata);

    let (_tx, mut rx) = channel.invoke(headers, CallOptions::default()).await;
    let ResponseStreamItem::Trailers(trailers) = rx.recv(&mut NopRecvMessage).await else {
        panic!("expected trailers from custom transport");
    };

    assert!(trailers.status().is_ok());
    assert_eq!(connects.load(Ordering::SeqCst), 1);
    assert_eq!(
        trailers.metadata().get("x-custom-ascii").unwrap().to_str(),
        "hello"
    );
    assert_eq!(
        trailers
            .metadata()
            .get_bin("x-custom-bin")
            .unwrap()
            .as_bytes(),
        b"\x01\x02\x03"
    );

    let _ = accept_task.await.unwrap();
}
