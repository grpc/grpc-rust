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
//! The pool holds only weak references to its clients. A client stays in the
//! pool while any caller holds a [`SharedXdsClient`] returned by
//! [`XdsClientPool::get_or_create`]; once the last one is dropped, the next
//! call for that target creates a fresh client. Dead entries are pruned
//! whenever a new client is added.

#![allow(dead_code)] // TODO: remove once the xDS resolver uses the pool.

/// Adapter from the `grpc` crate's runtime to [`xds_client::Runtime`].
pub mod runtime;

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;
use std::sync::Weak;

use grpc::__unstable::rt::GrpcRuntime;
use xds_client::MetricsRecorder;
use xds_client::ProstCodec;
use xds_client::XdsClient;

use crate::bootstrap::BootstrapConfig;
use runtime::XdsRuntime;

/// The process-wide [`XdsClientPool`] used by xDS channels.
pub static GLOBAL_XDS_CLIENT_POOL: XdsClientPool = XdsClientPool::new();

/// A pool of [`XdsClient`]s keyed by channel target string.
pub struct XdsClientPool {
    clients: Mutex<BTreeMap<String, Weak<XdsClient>>>,
}

impl Default for XdsClientPool {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for XdsClientPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        f.debug_struct("XdsClientPool")
            .field("targets", &clients.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl XdsClientPool {
    /// Creates an empty pool.
    pub const fn new() -> Self {
        Self {
            clients: Mutex::new(BTreeMap::new()),
        }
    }

    /// Returns the live [`XdsClient`] for `target`, creating it if there is
    /// none.
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
        let mut clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(client) = clients.get(target).and_then(Weak::upgrade) {
            return SharedXdsClient(client);
        }
        let mut builder = XdsClient::builder(
            bootstrap.to_client_config().with_target(target),
            bootstrap.to_transport_builder(runtime.clone()),
            ProstCodec,
            XdsRuntime::new(runtime),
        );
        if let Some(recorder) = metrics_recorder {
            builder = builder.with_metrics_recorder(recorder);
        }
        let client = Arc::new(builder.build());
        clients.retain(|_, client| client.strong_count() > 0);
        clients.insert(target.to_owned(), Arc::downgrade(&client));
        SharedXdsClient(client)
    }

    /// Returns the targets that currently have an entry in the pool, in
    /// sorted order, including dead entries that have not been pruned yet.
    #[cfg(test)]
    fn targets(&self) -> Vec<String> {
        self.clients.lock().unwrap().keys().cloned().collect()
    }
}

/// A reference to a pooled [`XdsClient`].
///
/// The client stays in the pool while any `SharedXdsClient` for it is alive.
/// Cloning creates another reference to the same client.
///
/// Note that the pool tracks usage via `SharedXdsClient` references only.
/// Cloning the inner [`XdsClient`] keeps its background worker alive but does
/// not keep the pool entry alive.
#[derive(Clone)]
pub struct SharedXdsClient(Arc<XdsClient>);

impl Deref for SharedXdsClient {
    type Target = XdsClient;

    fn deref(&self) -> &XdsClient {
        &self.0
    }
}

impl fmt::Debug for SharedXdsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedXdsClient").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use grpc::credentials::LocalChannelCredentials;

    use super::*;
    use crate::bootstrap::XdsServerConfig;

    fn get(pool: &XdsClientPool, target: &str) -> SharedXdsClient {
        let bootstrap = BootstrapConfig::new(vec![XdsServerConfig::new(
            "dns:///localhost:1".to_string(),
            Arc::new(LocalChannelCredentials::new()),
        )]);
        pool.get_or_create(target, &bootstrap, GrpcRuntime::default(), None)
    }

    fn same_client(a: &SharedXdsClient, b: &SharedXdsClient) -> bool {
        Arc::ptr_eq(&a.0, &b.0)
    }

    #[tokio::test]
    async fn same_target_shares_client() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        assert!(same_client(&a, &get(&pool, "xds:///a")));
        assert_eq!(pool.targets(), vec!["xds:///a"]);
    }

    #[tokio::test]
    async fn different_targets_get_different_clients() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        let b = get(&pool, "xds:///b");
        assert!(!same_client(&a, &b));
        assert_eq!(pool.targets(), vec!["xds:///a", "xds:///b"]);
    }

    #[tokio::test]
    async fn client_lives_while_referenced() {
        let pool = XdsClientPool::new();
        let a1 = get(&pool, "xds:///a");
        let a2 = a1.clone();
        let weak = Arc::downgrade(&a1.0);

        drop(a1);
        assert!(same_client(&a2, &get(&pool, "xds:///a")));
        drop(a2);
        assert!(weak.upgrade().is_none());
    }

    #[tokio::test]
    async fn unused_client_is_replaced_and_pruned() {
        let pool = XdsClientPool::new();
        let first = Arc::downgrade(&get(&pool, "xds:///a").0);
        // Adding "b" prunes the dead "a" entry; "b" itself is dropped at once.
        drop(get(&pool, "xds:///b"));
        assert!(first.upgrade().is_none());
        assert_eq!(pool.targets(), vec!["xds:///b"]);

        // Adding "a" again creates a fresh client and prunes the dead "b".
        let a = get(&pool, "xds:///a");
        assert_eq!(pool.targets(), vec!["xds:///a"]);
        let clients = pool.clients.lock().unwrap();
        assert!(Arc::ptr_eq(&a.0, &clients["xds:///a"].upgrade().unwrap()));
    }

    #[tokio::test]
    async fn reference_outliving_pool_drops_cleanly() {
        let pool = XdsClientPool::new();
        let a = get(&pool, "xds:///a");
        drop(pool);
        drop(a);
    }
}
