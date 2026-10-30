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

//! Per-RPC configuration selection.
//!
//! A name resolver may attach a [`ConfigSelector`] to a [`ResolverUpdate`] to
//! control the configuration applied to each RPC made on the channel.

use std::fmt::Debug;
use std::sync::Arc;

use crate::StatusError;
use crate::client::CallOptions;
use crate::client::DynInvoke;
use crate::client::DynRecvStream;
use crate::client::DynSendStream;
use crate::client::RequestHeaders;
use crate::client::name_resolution::ResolverUpdate;

/// A dyn-compatible version of
/// [`Intercept`](crate::client::interceptor::Intercept).
#[crate::async_trait]
pub trait DynIntercept: Send + Sync {
    /// A dynamic version of
    /// [`Intercept::intercept`](crate::client::interceptor::Intercept::intercept).
    /// `next` is owned by the interceptor, which may clone it and keep it for
    /// as long as it likes.
    async fn dyn_intercept(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
        next: Arc<dyn DynInvoke>,
    ) -> (Box<dyn DynSendStream>, Box<dyn DynRecvStream>);
}

/// The configuration to apply to a single RPC, as chosen by a
/// [`ConfigSelector`].
// TODO: Add support for the following:
// * MethodConfig
#[derive(Default)]
#[non_exhaustive]
pub struct RpcConfig {
    /// The interceptor to run the RPC through, if any.
    pub interceptor: Option<Arc<dyn DynIntercept>>,
}

impl RpcConfig {
    /// Sets the interceptor to run the RPC through, replacing any previously
    /// set interceptor, and returns the updated `RpcConfig`.
    pub fn with_interceptor(mut self, interceptor: Arc<dyn DynIntercept>) -> Self {
        self.interceptor = Some(interceptor);
        self
    }
}

/// The data provided to a [`ConfigSelector`] when selecting the configuration
/// for an RPC.
#[derive(Debug)]
#[non_exhaustive]
pub struct SelectConfigOptions<'a> {
    /// The headers of the RPC, after any interceptors installed on the
    /// channel have run.
    pub request_headers: &'a RequestHeaders,
}

impl<'a> SelectConfigOptions<'a> {
    /// Creates the options for selecting the configuration of an RPC with
    /// `request_headers`.
    pub fn new(request_headers: &'a RequestHeaders) -> Self {
        Self { request_headers }
    }
}

/// Controls what configuration to use for every RPC.
///
/// A `ConfigSelector` is supplied by a name resolver through the attributes of
/// a `ResolverUpdate` and is consulted by the channel once per RPC.
///
/// The selector may inspect the RPC's headers, e.g. to route it. Work that
/// attaches per-RPC state to its [`CallOptions`](crate::client::CallOptions)
/// for later stages (such as the picker) belongs in the
/// [`RpcConfig::interceptor`] that the selector returns.
pub trait ConfigSelector: Send + Sync + Debug {
    /// Selects the configuration for the RPC, or terminates it with the
    /// returned error.
    ///
    /// Errors with a status code that the control plane may not use, as
    /// defined by gRFC A54, are converted to INTERNAL by the channel.
    fn select_config(&self, options: SelectConfigOptions<'_>) -> Result<RpcConfig, StatusError>;
}

/// The config selector a channel uses when its resolver does not supply one.
// TODO: Store the default service config once support for overriding a
// call's method config is added.
#[derive(Debug)]
pub(crate) struct DefaultConfigSelector;

impl ConfigSelector for DefaultConfigSelector {
    fn select_config(&self, _options: SelectConfigOptions<'_>) -> Result<RpcConfig, StatusError> {
        // TODO: Return a MethodConfig derived from the channel's default
        // service config.
        Ok(RpcConfig::default())
    }
}

impl ResolverUpdate {
    /// Stores `cs` in the attributes of this update, replacing any previously
    /// stored [`ConfigSelector`], and returns the updated `ResolverUpdate`.
    pub fn with_config_selector(mut self, cs: Arc<dyn ConfigSelector>) -> Self {
        self.attributes = self.attributes.add(ConfigSelectorAttr(cs));
        self
    }

    /// Removes the [`ConfigSelector`] stored in the attributes of this update,
    /// if any, and returns it.
    pub(crate) fn take_config_selector(&mut self) -> Option<Arc<dyn ConfigSelector>> {
        let cs = self.attributes.get::<ConfigSelectorAttr>()?.0.clone();
        self.attributes = self.attributes.remove::<ConfigSelectorAttr>();
        Some(cs)
    }
}

/// Wrapper that allows an `Arc<dyn ConfigSelector>` to be stored in
/// [`Attributes`](crate::attributes::Attributes), which requires values to
/// implement [`Eq`].
///
/// Equality is based on pointer identity: two wrappers are equal if and only
/// if they point to the same `ConfigSelector` allocation.
#[derive(Debug, Clone)]
struct ConfigSelectorAttr(Arc<dyn ConfigSelector>);

impl PartialEq for ConfigSelectorAttr {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ConfigSelectorAttr {}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Mutex;
    use std::sync::Weak;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    use bytes::Bytes;
    use tokio::sync::mpsc;
    use tokio::sync::oneshot;
    use tokio::sync::watch;
    use tokio::task::JoinHandle;
    use tonic_prost::prost::Message as _;

    use super::*;
    use crate::StatusCodeError;
    use crate::async_trait;
    use crate::client::CallOptions;
    use crate::client::Channel;
    use crate::client::DynInvoke;
    use crate::client::DynRecvStream;
    use crate::client::DynSendStream;
    use crate::client::Invoke as _;
    use crate::client::RecvStream as _;
    use crate::client::RequestHeaders;
    use crate::client::ResponseStreamItem;
    use crate::client::SendOptions;
    use crate::client::SendStream as _;
    use crate::client::name_resolution::ChannelController;
    use crate::client::name_resolution::Endpoint;
    use crate::client::name_resolution::Resolver;
    use crate::client::name_resolution::ResolverBuilder;
    use crate::client::name_resolution::ResolverOptions;
    use crate::client::name_resolution::TCP_IP_NETWORK_TYPE;
    use crate::client::name_resolution::Target;
    use crate::client::name_resolution::WorkScheduler;
    use crate::client::name_resolution::global_registry;
    use crate::client::test_util::ByteRecvMsg;
    use crate::client::test_util::ByteSendMsg;
    use crate::core::Address;
    use crate::credentials::ChannelCredentials;
    use crate::credentials::LocalChannelCredentials;
    use crate::credentials::ProtocolInfo;
    use crate::credentials::call::CallCredentials;
    use crate::credentials::client::ClientHandshakeInfo;
    use crate::credentials::client::HandshakeOutput;
    use crate::credentials::common::Authority;
    use crate::echo_pb::EchoRequest;
    use crate::echo_pb::EchoResponse;
    use crate::private;
    use crate::rt::BoxEndpoint;
    use crate::rt::GrpcRuntime;
    use crate::stub_server::RunningStubServer;
    use crate::stub_server::StubServer;

    const UNARY_ECHO_METHOD: &str = "/grpc.examples.echo.Echo/UnaryEcho";
    /// How long to wait for events that are expected to happen.
    const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(10);
    /// How long to wait when checking that an event does not happen.
    const DEFAULT_TEST_SHORT_TIMEOUT: Duration = Duration::from_millis(10);

    /// A config selector that fails every RPC with a fixed status and counts
    /// how many times it is consulted.
    #[derive(Debug)]
    struct FakeConfigSelector {
        status: StatusError,
        /// The method name of each RPC the selector was consulted for.
        methods: Mutex<Vec<String>>,
    }

    impl FakeConfigSelector {
        fn new_arc(code: StatusCodeError) -> Arc<Self> {
            Arc::new(Self {
                status: StatusError::new(code, "fake config selector error"),
                methods: Mutex::new(Vec::new()),
            })
        }

        /// Returns how many times this selector was consulted.
        fn calls(&self) -> usize {
            self.methods.lock().unwrap().len()
        }

        /// Returns the method name of each RPC the selector was consulted
        /// for.
        fn methods(&self) -> Vec<String> {
            self.methods.lock().unwrap().clone()
        }
    }

    impl ConfigSelector for FakeConfigSelector {
        fn select_config(
            &self,
            options: SelectConfigOptions<'_>,
        ) -> Result<RpcConfig, StatusError> {
            self.methods
                .lock()
                .unwrap()
                .push(options.request_headers.method_name().to_string());
            Err(self.status.clone())
        }
    }

    /// A subscription to an xDS cluster. The xDS resolver keeps a cluster
    /// subscribed while anything holds a reference to it.
    #[derive(Debug)]
    struct ClusterSubscription;

    /// Holds a cluster subscription on behalf of the config selector and of
    /// every RPC routed to the cluster. The selector holds one reference to
    /// the interceptor, and each RPC holds another, through its `RpcConfig`,
    /// until the RPC's `invoke` returns. The cluster stays subscribed while
    /// any reference exists.
    #[derive(Debug)]
    struct ClusterInterceptor {
        _subscription: ClusterSubscription,
        /// Notified each time an RPC reaches the interceptor.
        intercepted_tx: mpsc::UnboundedSender<()>,
    }

    #[async_trait]
    impl DynIntercept for ClusterInterceptor {
        async fn dyn_intercept(
            &self,
            headers: RequestHeaders,
            options: CallOptions,
            next: Arc<dyn DynInvoke>,
        ) -> (Box<dyn DynSendStream>, Box<dyn DynRecvStream>) {
            // The test may have stopped listening.
            let _ = self.intercepted_tx.send(());
            next.dyn_invoke(headers, options).await
        }
    }

    /// Mimics the xDS resolver's config selector: every RPC is routed through
    /// the same cluster interceptor.
    #[derive(Debug)]
    struct ClusterConfigSelector {
        interceptor: Arc<ClusterInterceptor>,
    }

    impl ClusterConfigSelector {
        /// Returns a config selector for a new cluster subscription, and a
        /// weak reference for counting the references to its interceptor.
        /// The interceptor notifies `intercepted_tx` each time an RPC reaches
        /// it.
        fn new_arc(
            intercepted_tx: mpsc::UnboundedSender<()>,
        ) -> (Arc<Self>, Weak<ClusterInterceptor>) {
            let interceptor = Arc::new(ClusterInterceptor {
                _subscription: ClusterSubscription,
                intercepted_tx,
            });
            let refs = Arc::downgrade(&interceptor);
            (Arc::new(Self { interceptor }), refs)
        }
    }

    impl ConfigSelector for ClusterConfigSelector {
        fn select_config(
            &self,
            _options: SelectConfigOptions<'_>,
        ) -> Result<RpcConfig, StatusError> {
            Ok(RpcConfig::default().with_interceptor(self.interceptor.clone()))
        }
    }

    /// Returns a resolver scheme that is unique within the test binary.
    /// Resolvers can't be removed from the global registry, so each test
    /// registers its own.
    fn unique_scheme() -> String {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        format!(
            "config-selector-test-{}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        )
    }

    /// A resolver update, and a sender for the channel's result of applying
    /// it.
    type PendingUpdate = (ResolverUpdate, oneshot::Sender<Result<(), String>>);

    /// Builds [`FakeResolver`]s, sending a handle to each one to the test.
    struct FakeResolverBuilder {
        scheme: String,
        handle_tx: mpsc::UnboundedSender<FakeResolverHandle>,
    }

    impl FakeResolverBuilder {
        /// Registers a new builder in the global registry. Returns its scheme
        /// and a receiver for the handles of the resolvers it builds.
        fn register() -> (String, mpsc::UnboundedReceiver<FakeResolverHandle>) {
            let scheme = unique_scheme();
            let (handle_tx, handle_rx) = mpsc::unbounded_channel();
            global_registry().add_builder(Box::new(Self {
                scheme: scheme.clone(),
                handle_tx,
            }));
            (scheme, handle_rx)
        }
    }

    impl ResolverBuilder for FakeResolverBuilder {
        fn build(&self, _target: &Target, options: ResolverOptions) -> Box<dyn Resolver> {
            let (update_tx, update_rx) = mpsc::unbounded_channel();
            let _ = self.handle_tx.send(FakeResolverHandle {
                work_scheduler: options.work_scheduler,
                update_tx,
            });
            Box::new(FakeResolver { update_rx })
        }

        fn scheme(&self) -> &str {
            &self.scheme
        }

        fn is_valid_uri(&self, _target: &Target) -> bool {
            true
        }
    }

    /// Lets the test push updates to the channel through a [`FakeResolver`].
    struct FakeResolverHandle {
        work_scheduler: Arc<dyn WorkScheduler>,
        update_tx: mpsc::UnboundedSender<PendingUpdate>,
    }

    impl FakeResolverHandle {
        /// Sends `update` to the channel and returns the channel's result of
        /// applying it.
        async fn update(&self, update: ResolverUpdate) -> Result<(), String> {
            let (result_tx, result_rx) = oneshot::channel();
            self.update_tx.send((update, result_tx)).unwrap();
            self.work_scheduler.schedule_work();
            GrpcRuntime::default()
                .timeout(DEFAULT_TEST_TIMEOUT, result_rx)
                .await
                .expect("timed out waiting for the channel to apply the update")
                .unwrap()
        }
    }

    /// A resolver that sends the channel the updates pushed through its
    /// [`FakeResolverHandle`].
    struct FakeResolver {
        update_rx: mpsc::UnboundedReceiver<PendingUpdate>,
    }

    impl Resolver for FakeResolver {
        fn resolve_now(&mut self) {}

        fn work(&mut self, channel_controller: &mut dyn ChannelController) {
            while let Ok((update, result_tx)) = self.update_rx.try_recv() {
                let _ = result_tx.send(channel_controller.update(update));
            }
        }
    }

    /// Local channel credentials that hold every handshake until
    /// [`open`](Self::open) is called. Until then, the channel's subchannels
    /// stay CONNECTING, so its RPCs stay queued waiting for a pick.
    struct GatedChannelCredentials {
        local: LocalChannelCredentials,
        is_open: watch::Sender<bool>,
    }

    impl GatedChannelCredentials {
        fn new_arc() -> Arc<Self> {
            Arc::new(Self {
                local: LocalChannelCredentials::new(),
                is_open: watch::Sender::new(false),
            })
        }

        /// Lets held and future handshakes proceed.
        fn open(&self) {
            self.is_open.send_replace(true);
        }
    }

    #[async_trait]
    impl ChannelCredentials for GatedChannelCredentials {
        fn info(&self) -> &ProtocolInfo {
            self.local.info()
        }

        fn get_call_credentials(
            &self,
            token: private::Internal,
        ) -> Option<&Arc<dyn CallCredentials>> {
            self.local.get_call_credentials(token)
        }

        async fn connect(
            &self,
            authority: &Authority,
            source: BoxEndpoint,
            info: &ClientHandshakeInfo,
            runtime: &GrpcRuntime,
            token: private::Internal,
        ) -> Result<HandshakeOutput, String> {
            // Waiting can't fail, since `self` owns the sender.
            let _ = self.is_open.subscribe().wait_for(|is_open| *is_open).await;
            self.local
                .connect(authority, source, info, runtime, token)
                .await
        }
    }

    /// Starts an echo server and creates a channel that uses a
    /// [`FakeResolver`]. Returns the server, the channel, and the handle of
    /// the channel's resolver, which has not sent any updates yet. The server
    /// shuts down when it's dropped.
    async fn setup() -> (RunningStubServer, Channel, FakeResolverHandle) {
        setup_with_credentials(LocalChannelCredentials::new_arc()).await
    }

    /// Like [`setup`], but the channel uses `credentials`.
    async fn setup_with_credentials(
        credentials: Arc<dyn ChannelCredentials>,
    ) -> (RunningStubServer, Channel, FakeResolverHandle) {
        let server = StubServer::new().start().await;
        let addr = server.addr();
        let (scheme, mut handle_rx) = FakeResolverBuilder::register();
        let mut channel = Channel::builder(format!("{scheme}:///{addr}"), credentials).build();
        // Exit idle so the channel builds its resolver.
        channel.get_state(true);
        let resolver = GrpcRuntime::default()
            .timeout(DEFAULT_TEST_TIMEOUT, handle_rx.recv())
            .await
            .expect("timed out waiting for the channel to build its resolver")
            .unwrap();
        (server, channel, resolver)
    }

    /// Returns a resolver update with a single endpoint for `addr`.
    fn update_for(addr: SocketAddr) -> ResolverUpdate {
        ResolverUpdate {
            endpoints: Ok(vec![Endpoint {
                addresses: vec![Address {
                    network_type: TCP_IP_NETWORK_TYPE,
                    address: addr.to_string().into(),
                    ..Default::default()
                }],
                ..Default::default()
            }]),
            ..Default::default()
        }
    }

    /// The streams of an RPC started by [`start_unary_echo`].
    type UnaryEchoRpc = (Box<dyn DynSendStream>, Box<dyn DynRecvStream>);

    /// Starts a unary echo RPC on `channel` without sending its request, so
    /// the RPC stays in progress until it's passed to [`finish_unary_echo`].
    async fn start_unary_echo(channel: &Channel) -> UnaryEchoRpc {
        let headers = RequestHeaders::new().with_method_name(UNARY_ECHO_METHOD);
        GrpcRuntime::default()
            .timeout(
                DEFAULT_TEST_TIMEOUT,
                channel.invoke(headers, CallOptions::default()),
            )
            .await
            .expect("timed out waiting for the RPC to start")
    }

    /// Starts a unary echo RPC like [`start_unary_echo`], but in a new task.
    /// Returns once `interceptions` is notified that the RPC reached its
    /// interceptor. The task outputs the RPC's streams once `invoke` returns.
    async fn spawn_unary_echo(
        channel: &Channel,
        interceptions: &mut mpsc::UnboundedReceiver<()>,
    ) -> JoinHandle<UnaryEchoRpc> {
        let channel = channel.clone();
        let rpc_handle = tokio::spawn(async move { start_unary_echo(&channel).await });
        GrpcRuntime::default()
            .timeout(DEFAULT_TEST_TIMEOUT, interceptions.recv())
            .await
            .expect("timed out waiting for the RPC to reach its interceptor")
            .unwrap();
        rpc_handle
    }

    /// Sends `message` on an RPC from [`start_unary_echo`] and waits for the
    /// RPC to end. Returns the echoed message, or the status the RPC failed
    /// with.
    async fn finish_unary_echo(rpc: UnaryEchoRpc, message: &str) -> Result<String, StatusError> {
        let (mut tx, mut rx) = rpc;
        let finish = async move {
            let request = Bytes::from(
                EchoRequest {
                    message: message.to_string(),
                }
                .encode_to_vec(),
            );
            // Sending fails if the RPC has already failed. The status is read
            // from the trailers below.
            let _ = tx
                .send(
                    &ByteSendMsg::new(&request),
                    SendOptions::new().with_final_msg(true),
                )
                .await;
            let mut response = ByteRecvMsg::new();
            let mut echoed = None;
            loop {
                match rx.recv(&mut response).await {
                    ResponseStreamItem::Headers(_) => {}
                    ResponseStreamItem::Message => {
                        let data = response.data.take().unwrap();
                        echoed = Some(EchoResponse::decode(data).unwrap().message);
                    }
                    ResponseStreamItem::Trailers(trailers) => {
                        return trailers
                            .into_status()
                            .map(|()| echoed.expect("RPC succeeded without a response"));
                    }
                    ResponseStreamItem::StreamClosed => panic!("stream closed before trailers"),
                }
            }
        };
        GrpcRuntime::default()
            .timeout(DEFAULT_TEST_TIMEOUT, finish)
            .await
            .expect("timed out waiting for the RPC to end")
    }

    /// Makes a unary echo RPC on `channel`. Returns the echoed message, or the
    /// status the RPC failed with.
    async fn unary_echo(channel: &Channel, message: &str) -> Result<String, StatusError> {
        let rpc = start_unary_echo(channel).await;
        finish_unary_echo(rpc, message).await
    }

    #[tokio::test]
    async fn rpc_succeeds_without_config_selector() {
        let (server, channel, resolver) = setup().await;
        resolver.update(update_for(server.addr())).await.unwrap();

        assert_eq!(unary_echo(&channel, "hello").await.unwrap(), "hello");
    }

    #[tokio::test]
    async fn config_selector_error_fails_rpc() {
        let (server, channel, resolver) = setup().await;
        let selector = FakeConfigSelector::new_arc(StatusCodeError::Unavailable);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector.clone()))
            .await
            .unwrap();

        let status = unary_echo(&channel, "hello").await.unwrap_err();

        assert_eq!(status.code(), StatusCodeError::Unavailable);
        assert_eq!(status.message(), "fake config selector error");
        assert_eq!(selector.methods(), [UNARY_ECHO_METHOD]);
    }

    #[tokio::test]
    async fn restricted_config_selector_status_becomes_internal() {
        let (server, channel, resolver) = setup().await;
        // gRFC A54 doesn't allow config selectors to return NOT_FOUND.
        let selector = FakeConfigSelector::new_arc(StatusCodeError::NotFound);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector))
            .await
            .unwrap();

        let status = unary_echo(&channel, "hello").await.unwrap_err();

        assert_eq!(status.code(), StatusCodeError::Internal);
        assert_eq!(
            status.message(),
            "config selector returned illegal status: NotFound: fake config selector error"
        );
    }

    #[tokio::test]
    async fn rpc_waits_for_first_resolver_update() {
        let (server, channel, resolver) = setup().await;
        let rpc_channel = channel.clone();
        let mut rpc = tokio::spawn(async move { unary_echo(&rpc_channel, "hello").await });

        assert!(
            GrpcRuntime::default()
                .timeout(DEFAULT_TEST_SHORT_TIMEOUT, &mut rpc)
                .await
                .is_err(),
            "RPC completed before the resolver's first update"
        );

        let selector = FakeConfigSelector::new_arc(StatusCodeError::Unavailable);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector.clone()))
            .await
            .unwrap();

        // The waiting RPC uses the selector from the update, not the default
        // one.
        let status = rpc.await.unwrap().unwrap_err();
        assert_eq!(status.code(), StatusCodeError::Unavailable);
        assert_eq!(status.message(), "fake config selector error");
        assert_eq!(selector.methods(), [UNARY_ECHO_METHOD]);
    }

    #[tokio::test]
    async fn resolver_update_without_config_selector_restores_default() {
        let (server, channel, resolver) = setup().await;
        let selector = FakeConfigSelector::new_arc(StatusCodeError::Unavailable);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector.clone()))
            .await
            .unwrap();
        assert!(unary_echo(&channel, "hello").await.is_err());

        resolver.update(update_for(server.addr())).await.unwrap();

        assert_eq!(unary_echo(&channel, "hello").await.unwrap(), "hello");
        assert_eq!(selector.calls(), 1);
    }

    #[tokio::test]
    async fn config_selector_applied_when_lb_policy_rejects_update() {
        let (_server, channel, resolver) = setup().await;
        let selector = FakeConfigSelector::new_arc(StatusCodeError::Unavailable);

        // pick_first rejects updates without addresses.
        let result = resolver
            .update(ResolverUpdate::default().with_config_selector(selector.clone()))
            .await;
        assert!(result.is_err());

        // The RPC fails with the config selector's status, not the LB
        // policy's.
        let status = unary_echo(&channel, "hello").await.unwrap_err();
        assert_eq!(status.message(), "fake config selector error");
        assert_eq!(selector.calls(), 1);
    }

    #[tokio::test]
    async fn rpcs_hold_cluster_subscription_until_invoke_returns() {
        let credentials = GatedChannelCredentials::new_arc();
        let (server, channel, resolver) = setup_with_credentials(credentials.clone()).await;
        let (intercepted_tx, mut intercepted_rx) = mpsc::unbounded_channel();
        let (selector, cluster_a) = ClusterConfigSelector::new_arc(intercepted_tx.clone());
        resolver
            .update(update_for(server.addr()).with_config_selector(selector))
            .await
            .unwrap();
        // The config selector holds one reference to the cluster's
        // interceptor.
        assert_eq!(cluster_a.strong_count(), 1);

        // Each RPC holds another one while it's queued waiting for the
        // connection.
        let rpc1_handle = spawn_unary_echo(&channel, &mut intercepted_rx).await;
        assert_eq!(cluster_a.strong_count(), 2);
        let rpc2_handle = spawn_unary_echo(&channel, &mut intercepted_rx).await;
        assert_eq!(cluster_a.strong_count(), 3);

        // The RPCs release their references once `invoke` returns, although
        // they're still in progress.
        credentials.open();
        let rpc1 = rpc1_handle.await.unwrap();
        let rpc2 = rpc2_handle.await.unwrap();
        assert_eq!(cluster_a.strong_count(), 1);

        assert_eq!(finish_unary_echo(rpc1, "hello").await.unwrap(), "hello");
        assert_eq!(finish_unary_echo(rpc2, "hello").await.unwrap(), "hello");

        // Replacing the config selector releases the subscription.
        let (selector, cluster_b) = ClusterConfigSelector::new_arc(intercepted_tx);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector))
            .await
            .unwrap();
        assert_eq!(cluster_a.strong_count(), 0);
        assert_eq!(cluster_b.strong_count(), 1);
    }

    #[tokio::test]
    async fn queued_rpcs_hold_cluster_subscription_after_config_selector_update() {
        let credentials = GatedChannelCredentials::new_arc();
        let (server, channel, resolver) = setup_with_credentials(credentials.clone()).await;
        let (intercepted_tx, mut intercepted_rx) = mpsc::unbounded_channel();
        let (selector, cluster_a) = ClusterConfigSelector::new_arc(intercepted_tx.clone());
        resolver
            .update(update_for(server.addr()).with_config_selector(selector))
            .await
            .unwrap();
        let rpc1_handle = spawn_unary_echo(&channel, &mut intercepted_rx).await;
        assert_eq!(cluster_a.strong_count(), 2);
        let rpc2_handle = spawn_unary_echo(&channel, &mut intercepted_rx).await;
        assert_eq!(cluster_a.strong_count(), 3);

        // Replacing the config selector drops its reference, but the queued
        // RPCs keep the subscription.
        let (selector, cluster_b) = ClusterConfigSelector::new_arc(intercepted_tx);
        resolver
            .update(update_for(server.addr()).with_config_selector(selector))
            .await
            .unwrap();
        assert_eq!(cluster_a.strong_count(), 2);
        assert_eq!(cluster_b.strong_count(), 1);

        // The subscription is released once the RPCs' `invoke` calls return.
        credentials.open();
        let rpc1 = rpc1_handle.await.unwrap();
        let rpc2 = rpc2_handle.await.unwrap();
        assert_eq!(cluster_a.strong_count(), 0);
        assert_eq!(cluster_b.strong_count(), 1);

        // The RPCs still succeed.
        assert_eq!(finish_unary_echo(rpc1, "hello").await.unwrap(), "hello");
        assert_eq!(finish_unary_echo(rpc2, "hello").await.unwrap(), "hello");
    }
}
