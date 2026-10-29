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

//! A process-wide pool of [`XdsClient`]s, keyed by the channel's target
//! string.
//!
//! Channels with the same target share a single [`XdsClient`] (and therefore a
//! single ADS stream). The first caller for a target wins: the arguments it
//! passes are used to construct the client, and the arguments passed by later
//! callers for the same target are ignored while that client is alive.
//!
//! Each call to [`XdsClientPool::get_or_create`] returns a [`SharedXdsClient`]
//! reference. When the last reference for a target is dropped, the pool drops
//! its entry for that target, so the next call creates a fresh client.

#![allow(dead_code)] // TODO: remove once the xDS resolver uses the pool.

/// Adapter from the `grpc` crate's runtime to [`xds_client::Runtime`].
pub mod runtime;

use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;

use grpc::__unstable::rt::GrpcRuntime;
use xds_client::MetricsRecorder;
use xds_client::ProstCodec;
use xds_client::XdsClient;

use crate::bootstrap::BootstrapConfig;
use runtime::XdsRuntime;

/// The process-wide [`XdsClientPool`] used by xDS channels.
pub static GLOBAL_XDS_CLIENT_POOL: LazyLock<XdsClientPool> = LazyLock::new(XdsClientPool::new);

/// A pool of [`XdsClient`]s keyed by channel target string.
///
/// Each entry is reference counted by the [`SharedXdsClient`]s handed out by
/// [`get_or_create`](Self::get_or_create). All reference count changes and
/// map updates happen under a single lock.
///
/// Cloning the pool creates another handle to the same pool.
#[derive(Clone)]
pub struct XdsClientPool {
    clients: Arc<Mutex<HashMap<String, PoolEntry>>>,
}

/// A pooled client and the number of [`SharedXdsClient`]s referencing it.
struct PoolEntry {
    client: XdsClient,
    ref_count: usize,
}

impl fmt::Debug for XdsClientPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdsClientPool")
            .field("targets", &self.lock().keys().collect::<Vec<_>>())
            .finish()
    }
}

impl XdsClientPool {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns the [`XdsClient`] for `target`, creating it if it does not
    /// exist.
    ///
    /// All of the arguments needed to construct an [`XdsClient`] must be
    /// provided, but they are only used if there is no live client for
    /// `target`; otherwise they are dropped and the existing client is
    /// returned.
    pub fn get_or_create(
        &self,
        target: &str,
        bootstrap: &BootstrapConfig,
        runtime: GrpcRuntime,
        metrics_recorder: Option<Arc<dyn MetricsRecorder>>,
    ) -> SharedXdsClient {
        let mut clients = self.lock();
        let entry = clients.entry(target.to_owned()).or_insert_with(|| {
            let mut builder = XdsClient::builder(
                bootstrap.to_client_config().with_target(target),
                bootstrap.to_transport_builder(runtime.clone()),
                ProstCodec,
                XdsRuntime::new(runtime),
            );
            if let Some(recorder) = metrics_recorder {
                builder = builder.with_metrics_recorder(recorder);
            }
            PoolEntry {
                client: builder.build(),
                ref_count: 0,
            }
        });
        entry.ref_count += 1;
        SharedXdsClient {
            pool: self.clone(),
            target: target.to_owned(),
            client: entry.client.clone(),
        }
    }

    /// Adds a reference to `target`'s entry, which must exist.
    fn add_ref(&self, target: &str) {
        let mut clients = self.lock();
        let entry = clients
            .get_mut(target)
            .expect("referenced xDS client missing from pool");
        entry.ref_count += 1;
    }

    /// Removes a reference to `target`'s entry, removing the entry when its
    /// last reference is released.
    fn release(&self, target: &str) {
        let mut clients = self.lock();
        let Some(entry) = clients.get_mut(target) else {
            debug_assert!(false, "released xDS client missing from pool");
            return;
        };
        entry.ref_count -= 1;
        if entry.ref_count == 0 {
            clients.remove(target);
        }
    }

    /// Locks the map, ignoring poisoning. The map is always left consistent,
    /// and this is called from `Drop`, where panicking must be avoided.
    fn lock(&self) -> MutexGuard<'_, HashMap<String, PoolEntry>> {
        self.clients.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Returns the targets that currently have a client in the pool.
    #[cfg(test)]
    fn targets(&self) -> Vec<String> {
        let mut targets: Vec<_> = self.lock().keys().cloned().collect();
        targets.sort();
        targets
    }
}

/// A reference to a pooled [`XdsClient`].
///
/// Cloning creates another reference to the same client. When the last
/// reference for a target is dropped, the pool removes its entry for that
/// target.
///
/// Note that the pool tracks usage via `SharedXdsClient` references only.
/// Cloning the inner [`XdsClient`] keeps its background worker alive but does
/// not keep the pool entry alive.
pub struct SharedXdsClient {
    pool: XdsClientPool,
    target: String,
    client: XdsClient,
}

impl SharedXdsClient {
    /// Returns the channel target this client is shared for.
    pub fn target(&self) -> &str {
        &self.target
    }
}

impl Clone for SharedXdsClient {
    fn clone(&self) -> Self {
        self.pool.add_ref(&self.target);
        Self {
            pool: self.pool.clone(),
            target: self.target.clone(),
            client: self.client.clone(),
        }
    }
}

impl Drop for SharedXdsClient {
    fn drop(&mut self) {
        self.pool.release(&self.target);
    }
}

impl Deref for SharedXdsClient {
    type Target = XdsClient;

    fn deref(&self) -> &XdsClient {
        &self.client
    }
}

impl fmt::Debug for SharedXdsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedXdsClient")
            .field("target", &self.target)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use grpc::credentials::LocalChannelCredentials;

    use super::*;
    use crate::bootstrap::XdsServerConfig;
    use crate::resource::ListenerResource;

    /// Returns true if `a` and `b` are handles to the same client, by watching
    /// a uniquely named resource on `a` and checking whether `b` reports the
    /// subscription. The worker handles commands in order, so the watch is
    /// registered before the snapshot is taken.
    async fn same_client(a: &XdsClient, b: &XdsClient) -> bool {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let name = format!(
            "same-client-probe-{}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        );
        let _watcher = a.watch::<ListenerResource>(name.clone()).await;
        b.resource_snapshot()
            .await
            .iter()
            .any(|snapshot| snapshot.name == name)
    }

    fn get(pool: &XdsClientPool, target: &str) -> SharedXdsClient {
        let bootstrap = BootstrapConfig::new(vec![XdsServerConfig::new(
            "dns:///localhost:1".to_string(),
            Arc::new(LocalChannelCredentials::new()),
        )]);
        pool.get_or_create(target, &bootstrap, GrpcRuntime::default(), None)
    }

    #[tokio::test]
    async fn same_target_shares_client() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        let b = get(&pool, "xds:///a");
        assert!(same_client(&a, &b).await);
        assert_eq!(a.target(), "xds:///a");
        assert_eq!(pool.targets(), vec!["xds:///a"]);
    }

    #[tokio::test]
    async fn different_targets_get_different_clients() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        let b = get(&pool, "xds:///b");
        assert!(!same_client(&a, &b).await);
        assert_eq!(pool.targets(), vec!["xds:///a", "xds:///b"]);
    }

    #[tokio::test]
    async fn entry_removed_after_last_reference_dropped() {
        let pool = XdsClientPool::new();
        let a1 = get(&pool, "xds:///a");
        let a2 = get(&pool, "xds:///a");
        let a3 = a1.clone();
        let b = get(&pool, "xds:///b");

        drop(a1);
        drop(a2);
        assert_eq!(pool.targets(), vec!["xds:///a", "xds:///b"]);
        drop(a3);
        assert_eq!(pool.targets(), vec!["xds:///b"]);
        drop(b);
        assert!(pool.targets().is_empty());
    }

    #[tokio::test]
    async fn new_client_created_after_entry_removed() {
        let pool = XdsClientPool::new();
        let first = get(&pool, "xds:///a");
        // Keeps the first client's worker running, but not its pool entry.
        let first_client = XdsClient::clone(&first);
        drop(first);
        assert!(pool.targets().is_empty());

        let second = get(&pool, "xds:///a");
        assert_eq!(pool.targets(), vec!["xds:///a"]);
        assert!(!same_client(&first_client, &second).await);
    }

    #[tokio::test]
    async fn reference_outliving_pool_handle_drops_cleanly() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        drop(pool);
        drop(a);
    }
}
