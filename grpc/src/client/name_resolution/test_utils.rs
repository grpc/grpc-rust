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

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::client::name_resolution::ChannelController;
use crate::client::name_resolution::Resolver;
use crate::client::name_resolution::ResolverBuilder;
use crate::client::name_resolution::ResolverOptions;
use crate::client::name_resolution::ResolverUpdate;
use crate::client::name_resolution::Target;
use crate::client::name_resolution::WorkScheduler;
use crate::client::name_resolution::global_registry;
use crate::client::service_config::ParseResult;
use crate::client::service_config::ServiceConfig;
use crate::rt::test_runtime;

/// A work scheduler for testing.
pub(crate) struct TestWorkScheduler {
    work_tx: mpsc::UnboundedSender<()>,
}

impl TestWorkScheduler {
    /// Creates a new `TestWorkScheduler`.
    pub(crate) fn new_pair() -> (Arc<dyn WorkScheduler>, mpsc::UnboundedReceiver<()>) {
        let (work_tx, work_rx) = mpsc::unbounded_channel();
        let sched = Self { work_tx };
        (Arc::new(sched), work_rx)
    }
}

impl WorkScheduler for TestWorkScheduler {
    fn schedule_work(&self) {
        self.work_tx.send(()).unwrap();
    }
}

/// A channel controller for testing.
pub(crate) struct TestChannelController {
    update_result: Result<(), String>,
    update_tx: mpsc::UnboundedSender<ResolverUpdate>,
}

impl TestChannelController {
    /// Creates a new `TestChannelController` that returns `Ok(())` on update.
    pub(crate) fn new_pair() -> (Self, mpsc::UnboundedReceiver<ResolverUpdate>) {
        let (update_tx, update_rx) = mpsc::unbounded_channel();
        let cc = Self {
            update_result: Ok(()),
            update_tx,
        };
        (cc, update_rx)
    }

    pub(crate) fn set_update_result(&mut self, update_result: Result<(), String>) {
        self.update_result = update_result;
    }
}

impl ChannelController for TestChannelController {
    fn update(&mut self, update: ResolverUpdate) -> Result<(), String> {
        println!("Received resolver update: {:?}", update);
        self.update_tx.send(update).unwrap();
        self.update_result.clone()
    }

    fn parse_service_config(&self, config: &str) -> ParseResult {
        ServiceConfig::parse(config)
    }
}

/// Returns a resolver scheme that is unique within the test binary.
/// Resolvers can't be removed from the global registry, so each test
/// registers its own.
fn unique_scheme() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    format!(
        "fake-resolver-test-{}",
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    )
}

/// A resolver update, and a sender for the channel's result of applying
/// it.
type PendingUpdate = (ResolverUpdate, oneshot::Sender<Result<(), String>>);

/// Builds [`FakeResolver`]s, sending a handle to each one to the test.
pub(crate) struct FakeResolverBuilder {
    scheme: String,
    handle_tx: mpsc::UnboundedSender<FakeResolverHandle>,
}

impl FakeResolverBuilder {
    /// Registers a new builder in the global registry. Returns its scheme
    /// and a receiver for the handles of the resolvers it builds.
    pub(crate) fn register() -> (String, mpsc::UnboundedReceiver<FakeResolverHandle>) {
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
pub(crate) struct FakeResolverHandle {
    work_scheduler: Arc<dyn WorkScheduler>,
    update_tx: mpsc::UnboundedSender<PendingUpdate>,
}

impl FakeResolverHandle {
    /// Sends `update` to the channel and returns the channel's result of
    /// applying it.
    pub(crate) async fn update(&self, update: ResolverUpdate) -> Result<(), String> {
        let (result_tx, result_rx) = oneshot::channel();
        self.update_tx.send((update, result_tx)).unwrap();
        self.work_scheduler.schedule_work();
        test_runtime()
            .timeout(Duration::from_secs(10), result_rx)
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
