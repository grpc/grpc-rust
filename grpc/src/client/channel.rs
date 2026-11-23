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

//! The client-side gRPC Channel implementation.

use core::panic;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

use arc_swap::ArcSwap;
use arc_swap::ArcSwapOption;
use parking_lot::Mutex;
use tokio::sync::Notify;
use tokio::sync::SetOnce;
use tokio::sync::mpsc;

use crate::StatusError;
use crate::client::CallOptions;
use crate::client::ConnectivityState;
use crate::client::Invoke;
use crate::client::InvokeOnce;
use crate::client::RecvStream;
use crate::client::RequestHeaders;
use crate::client::ResponseStreamItem;
use crate::client::SendOptions;
use crate::client::SendStream;
use crate::client::boxed::ArcInvoker;
use crate::client::boxed::BoxRecvStream;
use crate::client::boxed::BoxSendStream;
use crate::client::deadline::DeadlineInvoker;
use crate::client::deadline::DeadlineRecvStream;
use crate::client::deadline::DeadlineSendStream;
use crate::client::interceptor::Intercept as _;
use crate::client::load_balancing::LbPolicy as _;
use crate::client::load_balancing::LbState;
use crate::client::load_balancing::PickOptions;
use crate::client::load_balancing::PickResult;
use crate::client::load_balancing::Picker;
use crate::client::load_balancing::QueuingPicker;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::WorkScheduler;
use crate::client::load_balancing::graceful_switch::GracefulSwitchLbConfig;
use crate::client::load_balancing::graceful_switch::GracefulSwitchPolicy;
use crate::client::load_balancing::subchannel::Subchannel;
use crate::client::load_balancing::subchannel::SubchannelState;
use crate::client::load_balancing::subchannel_sharing::SubchannelSharing;
use crate::client::load_balancing::{self};
use crate::client::name_resolution::ResolverBuilder;
use crate::client::name_resolution::ResolverUpdate;
use crate::client::name_resolution::Target;
use crate::client::name_resolution::config_selector::ConfigSelector;
use crate::client::name_resolution::config_selector::DefaultConfigSelector;
use crate::client::name_resolution::config_selector::RpcConfig;
use crate::client::name_resolution::config_selector::SelectConfigOptions;
use crate::client::name_resolution::dns;
use crate::client::name_resolution::global_registry;
use crate::client::name_resolution::proxy_resolver;
use crate::client::name_resolution::{self};
use crate::client::service_config::ParseResult;
use crate::client::service_config::ServiceConfig;
use crate::client::stream_util::CompletionRecvStream;
use crate::client::stream_util::FailingRecvStream;
use crate::client::subchannel::InternalSubchannel;
use crate::client::subchannel::NopBackoff;
use crate::client::transport::GLOBAL_TRANSPORT_REGISTRY;
use crate::client::transport::SecurityOpts;
use crate::client::transport::TransportRegistry;
use crate::client::transport::tonic as tonic_transport;
use crate::core::Address;
use crate::core::RecvMessage;
use crate::core::SendMessage;
use crate::core::StreamEnded;
use crate::credentials::ChannelCredentials;
use crate::credentials::client::ClientHandshakeInfo;
use crate::credentials::common::Authority;
use crate::rt;
use crate::rt::GrpcRuntime;

/// A virtual, persistent connection to a gRPC service.
///
/// A `Channel` begins in an "idle" state and connects when the first RPC is
/// made or its [`get_state`](Channel::get_state) method is called.
///
/// To perform RPCs, use the [`Invoke`] trait implemented by `Channel`.
#[derive(Clone)]
pub struct Channel {
    /// The layers that RPCs pass through, outermost first:
    ///
    /// 1. [`DeadlineInvoker`]: enforces the call's deadline.
    /// 2. [`PersistentChannel`]: starts RPCs on the current [`ActiveChannel`].
    /// 3. [`ActiveChannel`]: applies the config chosen by its config selector.
    /// 4. [`LbInvoker`]: picks a subchannel using the LB policy's picker.
    inner: Arc<DeadlineInvoker<PersistentChannel>>,
}

impl Channel {
    /// Creates a new channel builder for the given target, credentials, and
    /// runtime. All three are required to build a channel.
    ///
    /// The [target name](https://github.com/grpc/grpc/blob/master/doc/naming.md)
    /// is a fully qualified, self contained URI defined by [rfc3986](https://datatracker.ietf.org/doc/html/rfc3986).
    /// The target's scheme determines the name resolver used. If none is
    /// detected the default name resolver ("dns") is used, unless overridden
    /// by the user. Valid examples of target names include:
    ///
    /// "foo.googleapis.com:8080"
    /// "dns:///foo.googleapis.com:8080"
    /// "dns:///foo.googleapis.com"
    /// "dns:///10.0.0.213:8080"
    /// "dns:///%5B2001:db8:85a3:8d3:1319:8a2e:370:7348%5D:443"
    /// "dns://8.8.8.8/foo.googleapis.com:8080"
    /// "dns://8.8.8.8/foo.googleapis.com"
    /// "zookeeper://zk.example.com:9900/example_service"
    ///
    /// Credentials must implement the [`ChannelCredentials`] trait.
    ///
    /// The runtime is used for all I/O, timers, and background tasks. Runtime
    /// implementations are provided by separate crates, e.g. `grpc-tokio`,
    /// whose `grpc_tokio::runtime()` returns a runtime backed by Tokio.
    ///
    ///# Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use grpc::client::Channel;
    /// use grpc::credentials::LocalChannelCredentials;
    ///
    /// # fn example(runtime: grpc::rt::GrpcRuntime) {
    /// let channel = Channel::builder(
    ///     "dns:///localhost:123",
    ///     Arc::new(LocalChannelCredentials::new()),
    ///     runtime, // e.g.: grpc_tokio::runtime();
    /// )
    /// .build();
    /// # }
    /// ```
    pub fn builder(
        target: impl Into<String>,
        credentials: Arc<dyn ChannelCredentials>,
        runtime: GrpcRuntime,
    ) -> ChannelBuilder {
        ChannelBuilder {
            target: target.into(),
            credentials,
            authority: None,
            runtime,
        }
    }

    /// Returns the current state of the channel. If `connect` is true and the
    /// state was [`Idle`](ConnectivityState::Idle), the channel will attempt to
    /// create connections.
    pub fn get_state(&mut self, connect: bool) -> ConnectivityState {
        self.persistent().get_state(connect)
    }

    /// Waits for the state of the channel to change from source.  Times out and
    /// returns an error after the deadline.
    pub async fn wait_for_state_change(
        &self,
        source: ConnectivityState,
        deadline: Instant,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl Channel {
    fn persistent(&self) -> &PersistentChannel {
        self.inner.next()
    }
}

impl Invoke for Channel {
    type SendStream = ChannelSendStream;
    type RecvStream = ChannelRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        let (tx, rx) = self.inner.invoke(headers, options).await;
        (ChannelSendStream(tx), ChannelRecvStream(rx))
    }
}

/// The [`SendStream`] of an RPC started on a [`Channel`].
pub struct ChannelSendStream(DeadlineSendStream<BoxSendStream>);

impl SendStream for ChannelSendStream {
    async fn send(
        &mut self,
        msg: &dyn SendMessage,
        options: SendOptions,
    ) -> Result<(), StreamEnded> {
        self.0.send(msg, options).await
    }
}

/// The [`RecvStream`] of an RPC started on a [`Channel`].
pub struct ChannelRecvStream(DeadlineRecvStream<BoxSendStream, BoxRecvStream>);

impl RecvStream for ChannelRecvStream {
    async fn recv(&mut self, msg: &mut dyn RecvMessage) -> ResponseStreamItem {
        self.0.recv(msg).await
    }
}

pub struct ChannelBuilder {
    // Required values.
    target: String,
    credentials: Arc<dyn ChannelCredentials>,
    runtime: GrpcRuntime,

    // Optional values.
    authority: Option<String>,
}

impl ChannelBuilder {
    /// Builds the channel with the provided configuration.
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use grpc::client::Channel;
    /// use grpc::credentials::LocalChannelCredentials;
    ///
    /// # fn example(runtime: grpc::rt::GrpcRuntime) {
    /// let channel = Channel::builder(
    ///     "dns:///localhost:123",
    ///     Arc::new(LocalChannelCredentials::new()),
    ///     runtime, // e.g.: grpc_tokio::runtime();
    /// )
    /// .build();
    /// # }
    /// ```
    pub fn build(self) -> Channel {
        // TODO(nathanielford) This construction is currently a rough-cut placeholder.
        // The design of PersistentChannel and how and where it is initialized
        // will be finalized with the 'Internal Channel Design' with
        // consideration for:
        // - error handling (inc. always-failing resolvers due to invalid targets))
        // - testing (inc. credential and transport configuration)

        setup_registeries();

        // TODO(nathanielford): Return errors here instead of panicking.
        let target = Target::from_str(self.target.as_str()).unwrap();
        let resolver_builder = global_registry().get(target.scheme()).unwrap();
        let resolver_builder = proxy_resolver::Builder::new_arc(resolver_builder);

        let authority = self
            .authority
            .unwrap_or_else(|| resolver_builder.default_authority(&target).to_owned());
        let security_opts = SecurityOpts {
            credentials: self.credentials,
            authority: Authority::from_host_port_str(&authority),
            handshake_info: ClientHandshakeInfo::default(),
        };
        let persistent = PersistentChannel {
            active_channel: ArcSwapOption::empty(),
            active_channel_init: Mutex::new(()),
            target,
            security_opts,
            runtime: self.runtime.clone(),
            resolver_builder,
        };
        Channel {
            inner: Arc::new(DeadlineInvoker::new(self.runtime, persistent)),
        }
    }

    /// Sets the channel's authority value, to be used as the :authority
    /// pseudo-header and the server name in authentication handshakes. This
    /// overrides all other ways of setting authority on the channel, but can be
    /// overridden by per-call authority values.
    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }
}

struct PersistentChannel {
    /// The current active channel, or `None` if the channel is idle. Reads are
    /// lock-free. Writers must hold `active_channel_init`.
    active_channel: ArcSwapOption<ActiveChannel>,
    /// Serializes creation (and, in the future, idle teardown) of the active
    /// channel so that at most one `ActiveChannel` is built at a time.
    active_channel_init: Mutex<()>,

    // Configuration
    target: Target,
    security_opts: SecurityOpts,
    runtime: GrpcRuntime,

    // Inferred Configuration
    resolver_builder: Arc<dyn ResolverBuilder>,
}

impl PersistentChannel {
    /// Returns the current state of the channel. If there is no underlying active
    /// channel, returns Idle. If `connect` is true, will create a new active
    /// channel iff none exists.
    fn get_state(&self, connect: bool) -> ConnectivityState {
        if connect {
            return self.get_active_channel().connectivity_state();
        }
        self.active_channel
            .load()
            .as_ref()
            .map_or(ConnectivityState::Idle, |ac| ac.connectivity_state())
    }

    /// Gets the underlying active channel. If there is no current connection,
    /// it will create one. This cannot fail and will always return a valid
    /// active channel.
    fn get_active_channel(&self) -> Arc<ActiveChannel> {
        // Fast path: lock-free.
        if let Some(ac) = self.active_channel.load_full() {
            return ac;
        }
        // Slow path: serialize creation so only one ActiveChannel is built.
        let _init = self.active_channel_init.lock();
        if let Some(ac) = self.active_channel.load_full() {
            return ac; // Another caller created it while we waited.
        }
        let ac = ActiveChannel::new_arc_for(self);
        self.active_channel.store(Some(ac.clone()));
        ac
    }
}

/// Starts RPCs on the current [`ActiveChannel`], creating it if necessary.
impl Invoke for PersistentChannel {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        self.get_active_channel().invoke(headers, options).await
    }
}

// A channel that is not idle (connecting, ready, or erroring).
struct ActiveChannel {
    /// Starts RPCs on subchannels picked by the LB policy.  Used directly when
    /// there is no config selector interceptor, avoiding dynamic dispatch.
    lb: Arc<LbInvoker>,
    /// `lb`, but type-erased (sharing the same allocation) to pass to the
    /// config selector interceptor.  Passed by reference to avoid the need to
    /// clone when calling.
    erased_lb: ArcInvoker,
    config_selector: SharedConfigSelector,
}

impl ActiveChannel {
    fn new_arc_for(persistent_channel: &PersistentChannel) -> Arc<Self> {
        let runtime = persistent_channel.runtime.clone();

        let lb_watcher = Arc::new(Watcher::new(LbState {
            connectivity_state: ConnectivityState::Connecting,
            picker: Arc::new(QueuingPicker) as Arc<dyn Picker>,
        }));

        let (wqtx, mut wqrx) = mpsc::unbounded_channel::<WorkQueueItem>();
        let config_selector = SharedConfigSelector::new();
        let mut resolver_channel_controller = ResolverChannelController::new(
            wqtx.clone(),
            runtime.clone(),
            lb_watcher.clone(),
            persistent_channel.security_opts.clone(),
            config_selector.clone(),
        );

        let work_scheduler = Arc::new(ResolverWorkScheduler { wqtx });
        let resolver_opts = name_resolution::ResolverOptions {
            authority: persistent_channel
                .security_opts
                .authority
                .host_port_string(),
            work_scheduler,
            runtime: runtime.clone(),
        };
        let mut resolver = persistent_channel
            .resolver_builder
            .build(&persistent_channel.target, resolver_opts);

        let abort_handle = runtime.spawn(Box::pin(async move {
            while let Some(w) = wqrx.recv().await {
                match w {
                    WorkQueueItem::ScheduleResolver => {
                        resolver.work(&mut resolver_channel_controller);
                    }
                    WorkQueueItem::ResolveNow => resolver.resolve_now(),
                    WorkQueueItem::ScheduleLbPolicy(data) => {
                        resolver_channel_controller
                            .lb_policy
                            .work(data, &mut resolver_channel_controller.lb_channel_controller);
                    }
                }
            }
        }));

        let lb = Arc::new(LbInvoker {
            abort_handle,
            lb_watcher,
        });
        Arc::new(Self {
            erased_lb: ArcInvoker::from_arc(lb.clone()),
            lb,
            config_selector,
        })
    }

    fn connectivity_state(&self) -> ConnectivityState {
        self.lb.lb_watcher.snapshot().value().connectivity_state
    }
}

/// Applies the config chosen by the config selector to each RPC.
impl Invoke for ActiveChannel {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        let selected = self
            .config_selector
            .select_config(SelectConfigOptions::default())
            .await;
        let config = match selected {
            Ok(config) => config,
            Err(status) => return FailingRecvStream::new_stream_pair(status, None),
        };
        match config.interceptor {
            Some(interceptor) => {
                interceptor
                    .intercept(headers, options, &self.erased_lb)
                    .await
            }
            // Avoid heap allocations and dynamic dispatch.
            None => self.lb.invoke(headers, options).await,
        }
    }
}

/// Starts RPCs on subchannels picked by the LB policy's current picker.
struct LbInvoker {
    /// The task running the resolver and LB policy, which update `lb_watcher`.
    /// Aborted when this is dropped.
    abort_handle: Box<dyn rt::TaskHandle>,
    /// For getting the channel connectivity state and pickers for RPCs.
    lb_watcher: Arc<Watcher<LbState>>,
}

impl Invoke for LbInvoker {
    type SendStream = BoxSendStream;
    type RecvStream = BoxRecvStream;

    /// Starts an RPC on a subchannel picked by the current LB picker, waiting
    /// for a new picker while picks are queued.
    async fn invoke(
        &self,
        headers: RequestHeaders,
        options: CallOptions,
    ) -> (Self::SendStream, Self::RecvStream) {
        // Fast path: snapshot the latest LB state without locking and pick
        // with its picker.
        let mut snapshot = self.lb_watcher.snapshot();
        loop {
            match snapshot
                .value()
                .picker
                .pick(PickOptions::new(&headers, &options))
            {
                PickResult::Pick(mut pr) => {
                    let Some(sc) = pr.subchannel.downcast_ref::<InternalSubchannel>() else {
                        panic!(
                            "picked subchannel is not an implementation provided by the channel"
                        );
                    };
                    if let Ok(invoker) = sc.ready_invoker() {
                        let on_complete = pr.on_complete.take();
                        let (tx, rx) = invoker.invoke_once(headers, options).await;
                        // Notify the LB policy when the RPC completes (or is
                        // abandoned) so it can release any per-call state.
                        let rx = match on_complete {
                            Some(on_complete) => {
                                BoxRecvStream::new(CompletionRecvStream::new(rx, on_complete))
                            }
                            None => rx,
                        };
                        return (tx, rx);
                    }
                    // The subchannel likely left READY before the LB policy
                    // updated its picker, so queue the pick and wait for a new
                    // picker.  Dropping the pick drops its completion callback
                    // without invoking it, since the RPC never started.
                }
                PickResult::Queue => {}
                PickResult::Fail(status) => {
                    // TODO: Check wait-for-ready and queue instead.
                    return FailingRecvStream::new_stream_pair(
                        status.sanitize_control_plane_status("picker returned illegal status"),
                        None,
                    );
                }
                PickResult::Drop(status) => {
                    return FailingRecvStream::new_stream_pair(
                        status.sanitize_control_plane_status("picker returned illegal status"),
                        None,
                    );
                }
            }
            // Queued: wait for this snapshot to be superseded, then retry the
            // pick with the newest state.
            snapshot = snapshot.next().await;
        }
    }
}

impl Drop for LbInvoker {
    fn drop(&mut self) {
        self.abort_handle.abort();
    }
}

struct ResolverWorkScheduler {
    wqtx: WorkQueueTx,
}

pub(super) type WorkQueueTx = mpsc::UnboundedSender<WorkQueueItem>;

impl name_resolution::WorkScheduler for ResolverWorkScheduler {
    fn schedule_work(&self) {
        let _ = self.wqtx.send(WorkQueueItem::ScheduleResolver);
    }
}

struct ResolverChannelController {
    wqtx: WorkQueueTx, // To queue re-resolution requests
    runtime: GrpcRuntime,
    lb_policy: SubchannelSharing<GracefulSwitchPolicy>,
    lb_work_scheduler: Arc<LbWorkScheduler>,
    lb_channel_controller: LbChannelController,
    config_selector: SharedConfigSelector,
}

impl ResolverChannelController {
    fn new(
        wqtx: WorkQueueTx,
        runtime: GrpcRuntime,
        lb_watcher: Arc<Watcher<LbState>>,
        security_opts: SecurityOpts,
        config_selector: SharedConfigSelector,
    ) -> Self {
        let lb_work_scheduler = Arc::new(LbWorkScheduler { wqtx: wqtx.clone() });
        let lb_channel_controller = LbChannelController {
            lb_work_scheduler: lb_work_scheduler.clone(),
            transport_registry: GLOBAL_TRANSPORT_REGISTRY.clone(),
            lb_watcher,
            runtime: runtime.clone(),
            security_opts,
        };
        Self {
            lb_policy: SubchannelSharing::new(
                GracefulSwitchPolicy::new(runtime.clone(), lb_work_scheduler.clone()),
                lb_work_scheduler.clone(),
            ),
            lb_work_scheduler,
            lb_channel_controller,
            wqtx: wqtx.clone(),
            runtime: runtime.clone(),
            config_selector,
        }
    }
}

impl name_resolution::ChannelController for ResolverChannelController {
    fn update(&mut self, mut update: ResolverUpdate) -> Result<(), String> {
        let (builder, config) = match update.service_config.as_ref() {
            Ok(Some(sc)) => sc.lb_config(),
            _ => ServiceConfig::default_lb_policy(),
        };

        let cfg_selector = update.take_config_selector();

        let gsb_config = GracefulSwitchLbConfig::new(builder, config);

        let lb_result =
            self.lb_policy
                .resolver_update(update, &gsb_config, &mut self.lb_channel_controller);
        // As per gRFC A31, the LB policy's update must be called and return
        // before the new config selector is used, since the selector may route
        // RPCs to destinations that only the LB policy's new picker knows
        // about. `resolver_update` is synchronous, so any picker produced for
        // this update has already been published.
        //
        // The selector is applied even if the LB policy returned an error: the
        // policy has still processed the update (and may have published a new
        // picker); the error only asks the resolver to re-resolve.
        //
        // TODO: once service config error handling (gRFC A21) is implemented,
        // keep the previous selector when the resolver returns an invalid
        // service config.
        self.config_selector
            .update(cfg_selector.unwrap_or_else(|| Arc::new(DefaultConfigSelector)));
        lb_result
    }

    fn parse_service_config(&self, config: &str) -> ParseResult {
        ServiceConfig::parse(config)
    }
}

struct LbChannelController {
    lb_work_scheduler: Arc<LbWorkScheduler>,
    transport_registry: TransportRegistry, // For creating subchannels
    lb_watcher: Arc<Watcher<LbState>>,
    runtime: GrpcRuntime, // For creating subchannels
    security_opts: SecurityOpts,
}

impl load_balancing::ChannelController for LbChannelController {
    fn new_subchannel(
        &mut self,
        address: &Address,
        work_scheduler: Arc<dyn WorkScheduler>,
    ) -> (Arc<dyn Subchannel>, SubchannelState) {
        let transport = self
            .transport_registry
            .get_transport(address.network_type)
            .unwrap();
        (
            InternalSubchannel::new_arc(
                address.clone(),
                transport,
                Arc::new(NopBackoff {
                    runtime: self.runtime.clone(),
                }),
                self.runtime.clone(),
                self.security_opts.clone(),
                work_scheduler,
            ),
            SubchannelState::idle(),
        )
    }

    fn update_picker(&mut self, update: LbState) {
        self.lb_watcher.update(update);
    }

    fn request_resolution(&mut self) {
        _ = self.lb_work_scheduler.wqtx.send(WorkQueueItem::ResolveNow);
    }
}

#[derive(Debug)]
struct LbWorkScheduler {
    wqtx: WorkQueueTx,
}

impl WorkScheduler for LbWorkScheduler {
    fn schedule_work(&self, data: Option<WorkData>) {
        _ = self.wqtx.send(WorkQueueItem::ScheduleLbPolicy(data));
    }
}

pub(super) enum WorkQueueItem {
    // Call the LB policy to do work.  This includes subchannel state updates,
    // which are delivered as WorkData by the subchannels themselves.
    ScheduleLbPolicy(Option<WorkData>),
    // Call the resolver to do work.
    ScheduleResolver,
    // Call the resolver to resolve now.
    ResolveNow,
}

pub(crate) struct Todo;

// Enables multiple consumers to observe values published by a single producer.
//
// The producer calls update(). Consumers call snapshot() to take a lock-free
// copy of the latest value; if that value is not usable they await
// Snapshot::next(), which resolves with a snapshot of the latest value once the
// Watcher holds a different value. A consumer waiting on a snapshot is never
// woken for the value it already holds.
pub(crate) struct Watcher<T> {
    // Latest value, for lock-free reads.
    current: ArcSwap<T>,
    // Signalled after every update to wake consumers waiting in
    // [`Snapshot::next`].
    updated: Notify,
}

impl<T> Watcher<T> {
    fn new(initial: T) -> Self {
        Self {
            current: ArcSwap::from_pointee(initial),
            updated: Notify::new(),
        }
    }

    /// Returns a snapshot of the latest value.
    pub(crate) fn snapshot(&self) -> Snapshot<'_, T> {
        Snapshot {
            watcher: self,
            value: self.current.load_full(),
        }
    }

    /// Publishes a new value and wakes every [`Snapshot`] taken against an
    /// older one.
    fn update(&self, item: T) {
        // Publish the new value before signalling so that woken waiters always
        // observe it.
        let old = self.current.swap(Arc::new(item));
        self.updated.notify_waiters();
        drop(old);
    }
}

/// A snapshot of one published value, taken via [`Watcher::snapshot`].
pub(crate) struct Snapshot<'a, T> {
    watcher: &'a Watcher<T>,
    // Holding the Arc keeps the allocation alive, so a pointer comparison with
    // the Watcher's current value cannot suffer from ABA.
    value: Arc<T>,
}

impl<'a, T> Snapshot<'a, T> {
    /// Returns the value captured when this snapshot was taken.
    pub(crate) fn value(&self) -> &T {
        &self.value
    }

    /// Resolves once the [`Watcher`] holds a different value than this
    /// snapshot, returning a snapshot of the latest value. Intermediate values
    /// published while waiting are skipped. Resolves immediately if this
    /// snapshot is already stale.
    pub(crate) async fn next(mut self) -> Snapshot<'a, T> {
        loop {
            // Register before checking: a `Notified` receives every
            // `notify_waiters()` call made after it is created, and update()
            // stores before notifying, so an update that the check misses
            // still wakes this waiter.
            let notified = self.watcher.updated.notified();
            let current = self.watcher.current.load_full();
            if !Arc::ptr_eq(&current, &self.value) {
                self.value = current;
                return self;
            }
            // The waking update may be one this snapshot already holds (if it
            // was taken between that update's store and notify), so check
            // again.
            notified.await;
        }
    }
}

/// Parses the host and port from a string. When the input can not be parsed
/// as (host, port) pair, it returns the entire input as the host.
fn parse_authority(host_and_port: &str) -> Authority {
    // Handle bracketed IPv6 addresses (e.g., "[::1]:80").
    if let Some(stripped) = host_and_port.strip_prefix('[')
        && let Some((host, port_str)) = stripped.split_once("]:")
        && let Ok(port) = port_str.parse::<u16>()
    {
        return Authority::new(host, Some(port));
    }
    // Handle unbracketed addresses (IPv4 or hostnames, e.g., "localhost:8080").
    if let Some((host, port_str)) = host_and_port.rsplit_once(':')
        && !host.contains(':')
        && let Ok(port) = port_str.parse::<u16>()
    {
        return Authority::new(host, Some(port));
    }
    Authority::new(host_and_port.to_string(), None)
}

/// Shares an [`ActiveChannel`]'s config selector between the work serializer,
/// which replaces it on every resolver update, and RPCs, which each read it
/// once.
#[derive(Debug, Clone)]
struct SharedConfigSelector {
    inner: Arc<SharedConfigSelectorInner>,
}

#[derive(Debug)]
struct SharedConfigSelectorInner {
    /// The current config selector. Until the resolver produces its first
    /// result, this is a [`DefaultConfigSelector`] that RPCs never use: they
    /// wait for `resolved` instead.
    current: ArcSwap<Arc<dyn ConfigSelector>>,
    /// Set once `current` holds the selector from the resolver's first result.
    resolved: SetOnce<()>,
}

impl SharedConfigSelector {
    fn new() -> Self {
        Self {
            inner: Arc::new(SharedConfigSelectorInner {
                current: ArcSwap::from_pointee(Arc::new(DefaultConfigSelector)),
                resolved: SetOnce::new(),
            }),
        }
    }

    /// Waits for the resolver's first result, then selects the configuration
    /// for an RPC using `current`.
    async fn select_config(&self, options: SelectConfigOptions) -> Result<RpcConfig, StatusError> {
        // Returns immediately if `resolved` is already set. `SetOnce` stays
        // set, so an `update` that races with this call can't be missed.
        self.inner.resolved.wait().await;
        self.inner
            .current
            .load()
            .select_config(options)
            .map_err(|s| s.sanitize_control_plane_status("config selector returned illegal status"))
    }

    fn update(&self, selector: Arc<dyn ConfigSelector>) {
        self.inner.current.store(Arc::new(selector));
        // Set `resolved` after the store, so that RPCs that observe it also
        // observe the new selector. This fails, harmlessly, on every update
        // after the first.
        let _ = self.inner.resolved.set(());
    }
}

// Sets up the default registries for transports, name resolvers, and load
// balancers.
fn setup_registeries() {
    load_balancing::register();
    dns::reg();
    #[cfg(unix)]
    name_resolution::unix::reg();
    #[cfg(target_os = "linux")]
    name_resolution::unix_abstract::reg();
    tonic_transport::reg();
}

#[cfg(test)]
mod tests {
    use crate::StatusCodeError;
    use std::sync::Barrier;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    use super::*;
    use crate::client::load_balancing::Pick;
    use crate::client::load_balancing::test_utils::StubPolicyFuncs;
    use crate::client::load_balancing::test_utils::reg_stub_policy;
    use crate::client::name_resolution::Endpoint;
    use crate::client::name_resolution::test_utils::FakeResolverBuilder;
    use crate::client::test_util::NopRecvMessage;
    use crate::credentials::LocalChannelCredentials;
    use crate::metadata::MetadataMap;
    use crate::rt::test_runtime;

    fn new_channel() -> Channel {
        Channel::builder(
            "dns:///localhost:1234",
            Arc::new(LocalChannelCredentials::new()),
            test_runtime(),
        )
        .build()
    }

    #[tokio::test]
    async fn get_state_without_connect_stays_idle() {
        let mut channel = new_channel();
        assert_eq!(channel.get_state(false), ConnectivityState::Idle);
        assert!(channel.persistent().active_channel.load().is_none());
    }

    #[tokio::test]
    async fn concurrent_get_active_channel_creates_one() {
        const THREADS: usize = 16;
        let channel = new_channel();
        let handle = tokio::runtime::Handle::current();
        let barrier = Barrier::new(THREADS);

        let active_channels: Vec<Arc<ActiveChannel>> = std::thread::scope(|s| {
            let workers: Vec<_> = (0..THREADS)
                .map(|_| {
                    s.spawn(|| {
                        // ActiveChannel creation spawns tasks on the runtime.
                        let _rt = handle.enter();
                        barrier.wait();
                        channel.persistent().get_active_channel()
                    })
                })
                .collect();
            workers.into_iter().map(|w| w.join().unwrap()).collect()
        });

        let first = &active_channels[0];
        for ac in &active_channels {
            assert!(Arc::ptr_eq(first, ac), "multiple ActiveChannels created");
        }
        assert!(Arc::ptr_eq(
            first,
            &channel.persistent().active_channel.load_full().unwrap()
        ));
        assert_ne!(
            channel.persistent().get_state(false),
            ConnectivityState::Idle
        );
    }

    // Fails the test instead of hanging if next() never resolves.
    async fn assert_next<'a, T: 'a>(
        next: impl Future<Output = Snapshot<'a, T>>,
    ) -> Snapshot<'a, T> {
        GrpcRuntime::default()
            .timeout(Duration::from_secs(5), next)
            .await
            .expect("next() did not resolve")
    }

    // Fails the test if next() resolves without a further update.
    async fn assert_no_next<'a, T: 'a>(next: impl Future<Output = Snapshot<'a, T>>) {
        assert!(
            GrpcRuntime::default()
                .timeout(Duration::from_millis(10), next)
                .await
                .is_err(),
            "next() resolved without an update"
        );
    }

    #[test]
    fn snapshot_reflects_update() {
        let watcher = Watcher::new(1u32);
        assert_eq!(*watcher.snapshot().value(), 1);
        watcher.update(2);
        assert_eq!(*watcher.snapshot().value(), 2);
    }

    // Covers the handoff in ActiveChannel::invoke: an update that lands after
    // snapshot() was called must be returned by next(), even if nobody was
    // awaiting next() at the time.
    #[tokio::test]
    async fn next_returns_update_made_before_await() {
        let watcher = Watcher::new(1u32);
        let snapshot = watcher.snapshot();
        assert_eq!(*snapshot.value(), 1);
        watcher.update(2);
        let snapshot = assert_next(snapshot.next()).await;
        assert_eq!(*snapshot.value(), 2);
    }

    // next() consumes the snapshot, so the same pinned future is polled both
    // before and after the update.
    #[tokio::test]
    async fn next_waits_for_update() {
        let watcher = Arc::new(Watcher::new(1u32));
        let mut next = std::pin::pin!(watcher.snapshot().next());
        assert_no_next(&mut next).await;

        let updater = {
            let watcher = watcher.clone();
            tokio::spawn(async move { watcher.update(2) })
        };
        let snapshot = assert_next(next).await;
        updater.await.unwrap();
        assert_eq!(*snapshot.value(), 2);
    }

    // next() skips intermediate values and returns the latest one. The
    // returned snapshot is current, so it waits for a further update.
    #[tokio::test]
    async fn next_skips_to_latest_value() {
        let watcher = Watcher::new(1u32);
        let snapshot = watcher.snapshot();
        watcher.update(2);
        watcher.update(3);
        let snapshot = assert_next(snapshot.next()).await;
        assert_eq!(*snapshot.value(), 3);
        assert_no_next(snapshot.next()).await;
    }

    // next() tracks updates, not value equality: publishing an equal value
    // still supersedes older snapshots.
    #[tokio::test]
    async fn equal_value_update_resolves_next() {
        let watcher = Watcher::new(1u32);
        let snapshot = watcher.snapshot();
        watcher.update(1);
        let snapshot = assert_next(snapshot.next()).await;
        assert_eq!(*snapshot.value(), 1);
    }

    // Sets its flag when dropped.
    struct DropFlag(Arc<AtomicBool>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    // Picks `subchannel`, with a completion callback that records whether it
    // was invoked or dropped.
    #[derive(Debug)]
    struct SubchannelPicker {
        subchannel: Arc<dyn Subchannel>,
        invoked: Arc<AtomicBool>,
        dropped: Arc<AtomicBool>,
    }

    impl Picker for SubchannelPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            let invoked = self.invoked.clone();
            let guard = DropFlag(self.dropped.clone());
            PickResult::Pick(Pick {
                subchannel: self.subchannel.clone(),
                metadata: MetadataMap::new(),
                on_complete: Some(Box::new(move |_| {
                    let _guard = guard;
                    invoked.store(true, Ordering::SeqCst);
                })),
            })
        }
    }

    // Fails every pick with its status.
    #[derive(Debug)]
    struct FailPicker(StatusError);

    impl Picker for FailPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            PickResult::Fail(self.0.clone())
        }
    }

    // Drops every pick with its status.
    #[derive(Debug)]
    struct DropPicker(StatusError);

    impl Picker for DropPicker {
        fn pick(&self, _options: PickOptions<'_>) -> PickResult {
            PickResult::Drop(self.0.clone())
        }
    }

    // A pick of a subchannel that is no longer READY (the LB policy hasn't
    // updated its picker yet) is queued until the next picker, instead of
    // failing the RPC.
    #[tokio::test]
    async fn pick_of_non_ready_subchannel_waits_for_new_picker() {
        const POLICY_NAME: &str = "channel_test_non_ready_pick";
        let runtime = GrpcRuntime::default();
        let invoked = Arc::new(AtomicBool::new(false));
        let dropped = Arc::new(AtomicBool::new(false));

        // An LB policy that creates a subchannel for the first address but
        // never connects it, so it stays IDLE, while reporting READY with a
        // picker that picks it.  It sends its work scheduler to the test, and
        // publishes the LB states sent by the test when its work is scheduled.
        let (work_scheduler_tx, mut work_scheduler_rx) = mpsc::unbounded_channel();
        let (lb_state_tx, lb_state_rx) = mpsc::unbounded_channel::<LbState>();
        let lb_state_rx = Arc::new(Mutex::new(lb_state_rx));
        reg_stub_policy(
            POLICY_NAME,
            StubPolicyFuncs {
                resolver_update: Some(Arc::new({
                    let invoked = invoked.clone();
                    let dropped = dropped.clone();
                    move |data, update, _config, controller| {
                        let address = update.endpoints.as_ref().unwrap()[0].addresses[0].clone();
                        let (subchannel, _state) = controller.new_subchannel(
                            &address,
                            data.lb_policy_options.work_scheduler.clone(),
                        );
                        controller.update_picker(LbState {
                            connectivity_state: ConnectivityState::Ready,
                            picker: Arc::new(SubchannelPicker {
                                subchannel,
                                invoked: invoked.clone(),
                                dropped: dropped.clone(),
                            }),
                        });
                        let _ =
                            work_scheduler_tx.send(data.lb_policy_options.work_scheduler.clone());
                        Ok(())
                    }
                })),
                work: Some(Arc::new(move |_data, _work_data, controller| {
                    while let Ok(state) = lb_state_rx.lock().try_recv() {
                        controller.update_picker(state);
                    }
                })),
                ..Default::default()
            },
        );

        let (scheme, mut resolver_rx) = FakeResolverBuilder::register();
        let mut channel = Channel::builder(
            format!("{scheme}:///unused"),
            Arc::new(LocalChannelCredentials::new()),
            runtime.clone(),
        )
        .build();
        // Exit idle so the channel builds its resolver.
        channel.get_state(true);
        let resolver = runtime
            .timeout(Duration::from_secs(5), resolver_rx.recv())
            .await
            .expect("timed out waiting for the channel to build its resolver")
            .unwrap();
        resolver
            .update(ResolverUpdate {
                endpoints: Ok(vec![Endpoint {
                    addresses: vec![Address {
                        network_type: name_resolution::TCP_IP_NETWORK_TYPE,
                        address: "127.0.0.1:1".to_string().into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }]),
                service_config: Ok(Some(
                    ServiceConfig::parse(&format!(
                        r#"{{"loadBalancingConfig": [{{"{POLICY_NAME}": {{}}}}]}}"#
                    ))
                    .unwrap(),
                )),
                ..Default::default()
            })
            .await
            .unwrap();
        let lb_work_scheduler = work_scheduler_rx.recv().await.unwrap();

        let mut call =
            std::pin::pin!(channel.invoke(RequestHeaders::new(), CallOptions::default()));
        assert!(
            runtime
                .timeout(Duration::from_millis(10), &mut call)
                .await
                .is_err(),
            "the RPC should wait for a new picker"
        );
        // The pick was abandoned without starting the RPC.
        assert!(dropped.load(Ordering::SeqCst));
        assert!(!invoked.load(Ordering::SeqCst));

        // The next picker is used.
        lb_state_tx
            .send(LbState {
                connectivity_state: ConnectivityState::TransientFailure,
                picker: Arc::new(FailPicker(StatusError::new(
                    StatusCodeError::Internal,
                    "fail picker",
                ))),
            })
            .unwrap();
        lb_work_scheduler.schedule_work(None);
        let (_tx, mut rx) = runtime
            .timeout(Duration::from_secs(5), call)
            .await
            .expect("the RPC should use the new picker");
        let ResponseStreamItem::Trailers(trailers) = rx.recv(&mut NopRecvMessage).await else {
            std::panic!("expected trailers");
        };
        assert_eq!(
            trailers.status().as_ref().unwrap_err().message(),
            "fail picker"
        );
    }

    // Performs an RPC on a channel whose LB policy uses `picker`, and returns
    // the RPC's resulting status code.
    async fn rpc_status_code_with_picker(
        policy_name: &'static str,
        picker: Arc<dyn Picker>,
    ) -> StatusCodeError {
        let runtime = GrpcRuntime::default();
        reg_stub_policy(
            policy_name,
            StubPolicyFuncs {
                resolver_update: Some(Arc::new(move |_data, _update, _config, controller| {
                    controller.update_picker(LbState {
                        connectivity_state: ConnectivityState::TransientFailure,
                        picker: picker.clone(),
                    });
                    Ok(())
                })),
                ..Default::default()
            },
        );

        let (scheme, mut resolver_rx) = FakeResolverBuilder::register();
        let mut channel = Channel::builder(
            format!("{scheme}:///unused"),
            Arc::new(LocalChannelCredentials::new()),
            runtime.clone(),
        )
        .build();
        // Exit idle so the channel builds its resolver.
        channel.get_state(true);
        let resolver = runtime
            .timeout(Duration::from_secs(5), resolver_rx.recv())
            .await
            .expect("timed out waiting for the channel to build its resolver")
            .unwrap();
        resolver
            .update(ResolverUpdate {
                endpoints: Ok(vec![Endpoint {
                    addresses: vec![Address {
                        network_type: name_resolution::TCP_IP_NETWORK_TYPE,
                        address: "127.0.0.1:1".to_string().into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }]),
                service_config: Ok(Some(
                    ServiceConfig::parse(&format!(
                        r#"{{"loadBalancingConfig": [{{"{policy_name}": {{}}}}]}}"#
                    ))
                    .unwrap(),
                )),
                ..Default::default()
            })
            .await
            .unwrap();

        let (_tx, mut rx) = runtime
            .timeout(
                Duration::from_secs(5),
                channel.invoke(RequestHeaders::new(), CallOptions::default()),
            )
            .await
            .expect("the RPC should fail");
        let ResponseStreamItem::Trailers(trailers) = rx.recv(&mut NopRecvMessage).await else {
            std::panic!("expected trailers");
        };
        trailers.status().as_ref().unwrap_err().code()
    }

    // A picker that fails a pick with a restricted status code results in an
    // INTERNAL status, per gRFC A54.
    #[tokio::test]
    async fn picker_fail_with_restricted_code_converted_to_internal() {
        let code = rpc_status_code_with_picker(
            "channel_test_restricted_fail_pick",
            Arc::new(FailPicker(StatusError::new(
                StatusCodeError::NotFound,
                "restricted",
            ))),
        )
        .await;
        assert_eq!(code, StatusCodeError::Internal);
    }

    // A picker that drops a pick with a restricted status code results in an
    // INTERNAL status, per gRFC A54.
    #[tokio::test]
    async fn picker_drop_with_restricted_code_converted_to_internal() {
        let code = rpc_status_code_with_picker(
            "channel_test_restricted_drop_pick",
            Arc::new(DropPicker(StatusError::new(
                StatusCodeError::NotFound,
                "restricted",
            ))),
        )
        .await;
        assert_eq!(code, StatusCodeError::Internal);
    }
}
