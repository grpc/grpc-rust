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

//! CDS-driven [`Connector`] for a single cluster.
//!
//! [`ClusterConnector`] watches one cluster's CDS resource and asks a
//! [`MakeConnector`] for a fresh [`Connector`] on every update (e.g. when
//! `transport_socket` changes), publishing it to a [`watch`] channel. Each
//! `connect` call reads the latest published connector, so new connections
//! pick up the current security config while established ones are untouched.
//!
//! An update whose config fails validation is rejected: the previous connector
//! is kept and a warning is logged.
//!
//! Construction is synchronous even though the first connector only exists
//! once CDS arrives — `connect` awaits publication instead. That lets a load
//! balancer take the connector up front and start discovery before CDS
//! resolves.

use std::sync::Arc;

use tokio::sync::watch;

use crate::client::endpoint::{ClusterConfig, Connector, MakeConnector, XdsEndpoint};
use crate::common::async_util::{AbortOnDrop, BoxFuture};
use crate::xds::cache::{CacheWatch, XdsCache};
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider::CertProviderRegistry;
use crate::xds::resource::ClusterResource;

/// A type-erased per-cluster connector, as published by CDS.
type SharedConnector<S> = Arc<dyn Connector<Service = S> + Send + Sync>;

/// A [`Connector`] that tracks one cluster's CDS resource.
///
/// Holds the receiving end of a [`watch`] channel fed by a spawned task; the
/// task exits (dropping the sender) when the cluster leaves the cache, and is
/// aborted when the connector is dropped.
pub(crate) struct ClusterConnector<S> {
    cluster_name: Arc<str>,
    rx: watch::Receiver<Option<SharedConnector<S>>>,
    _task: AbortOnDrop,
}

impl<S: Send + 'static> ClusterConnector<S> {
    /// Spawns the CDS watch for `cluster_name` and returns a connector backed
    /// by it.
    pub(crate) fn spawn<MC>(
        cache: &XdsCache,
        cluster_name: String,
        make_connector: Arc<MC>,
        #[cfg(feature = "_tls-any")] registry: Arc<CertProviderRegistry>,
    ) -> Self
    where
        MC: MakeConnector<Service = S>,
    {
        let (tx, rx) = watch::channel(None);
        // Subscribe here, not in the task: a subscription made after the
        // resource manager removed the cluster would recreate its entry. The
        // task also must not hold the cache, which would keep every entry,
        // and so every watch, open.
        let cluster_watch = cache.watch_cluster(&cluster_name);
        let task = tokio::spawn(publish_connectors(
            cluster_watch,
            cluster_name.clone(),
            make_connector,
            #[cfg(feature = "_tls-any")]
            registry,
            tx,
        ));
        Self {
            cluster_name: Arc::from(cluster_name),
            rx,
            _task: AbortOnDrop(task),
        }
    }
}

impl<S: Send + 'static> Connector for ClusterConnector<S> {
    type Service = S;

    fn connect(&self, endpoint: &XdsEndpoint) -> BoxFuture<Self::Service> {
        // Common case: CDS already published a connector, so hand it the
        // borrowed endpoint directly. Bound first so the watch guard is
        // released before connecting.
        let current = self.rx.borrow().clone();
        if let Some(connector) = current {
            return connector.connect(endpoint);
        }

        // Only before the first CDS update: wait for a connector, which needs
        // an owned endpoint.
        let mut rx = self.rx.clone();
        let cluster_name = self.cluster_name.clone();
        let endpoint = endpoint.clone();

        Box::pin(async move {
            loop {
                // Clone out of the borrow guard; it is not held across an await.
                let current = rx.borrow_and_update().clone();
                if let Some(connector) = current {
                    return connector.connect(&endpoint).await;
                }
                if rx.changed().await.is_err() {
                    tracing::error!(
                        cluster = %cluster_name,
                        endpoint = %endpoint.address(),
                        "cluster removed from cache before any CDS update produced a \
                         connector; parking the connect until discovery drops it",
                    );
                    // The cluster left the cache before publishing a connector.
                    // Its endpoints are removed too, so discovery drops this
                    // future; parking avoids inventing a bogus connection.
                    return std::future::pending().await;
                }
            }
        })
    }
}

/// Background task: republishes a connector on every accepted CDS update.
async fn publish_connectors<MC, S>(
    mut cluster_watch: CacheWatch<ClusterResource>,
    cluster_name: String,
    make_connector: Arc<MC>,
    #[cfg(feature = "_tls-any")] registry: Arc<CertProviderRegistry>,
    tx: watch::Sender<Option<SharedConnector<S>>>,
) where
    MC: MakeConnector<Service = S>,
{
    while let Some(cluster) = cluster_watch.next().await {
        let cluster_config = ClusterConfig::from_resource(
            &cluster,
            #[cfg(feature = "_tls-any")]
            &registry,
        );
        match make_connector.make_connector(cluster_config) {
            Ok(connector) => {
                if tx.send(Some(connector)).is_err() {
                    return;
                }
            }
            Err(e) => tracing::warn!(
                cluster = %cluster_name,
                error = %e,
                "CDS update rejected; keeping previous connector",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::oneshot::{self, error::TryRecvError};
    use tower::BoxError;

    use super::*;
    use crate::client::endpoint::EndpointAddress;
    use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
    use crate::xds::resource::cluster::LbPolicy;

    const CLUSTER: &str = "c1";

    /// Connects to an address by returning it as a string.
    struct AddrConnector;

    impl Connector for AddrConnector {
        type Service = String;

        fn connect(&self, endpoint: &XdsEndpoint) -> BoxFuture<String> {
            let addr = endpoint.address().to_string();
            Box::pin(async move { addr })
        }
    }

    /// Makes [`AddrConnector`]s. Holds a sender only to drop it, which tells
    /// the receiver that the task holding this has stopped.
    struct AddrMakeConnector {
        _dropped: oneshot::Sender<()>,
    }

    impl MakeConnector for AddrMakeConnector {
        type Service = String;

        fn make_connector(
            &self,
            _cluster: ClusterConfig<'_>,
        ) -> Result<SharedConnector<String>, BoxError> {
            Ok(Arc::new(AddrConnector))
        }
    }

    fn cluster() -> Arc<ClusterResource> {
        Arc::new(ClusterResource {
            name: CLUSTER.to_string(),
            eds_service_name: None,
            lb_policy: LbPolicy::RoundRobin,
            security: None,
            circuit_breaking: CircuitBreakingConfig::default(),
        })
    }

    /// Spawns a connector for [`CLUSTER`], along with a receiver that
    /// resolves once the connector's CDS task stops.
    fn spawn_connector(cache: &XdsCache) -> (ClusterConnector<String>, oneshot::Receiver<()>) {
        let (dropped, task_stopped) = oneshot::channel();
        let connector = ClusterConnector::spawn(
            cache,
            CLUSTER.to_string(),
            Arc::new(AddrMakeConnector { _dropped: dropped }),
            #[cfg(feature = "_tls-any")]
            Arc::new(
                CertProviderRegistry::from_bootstrap(&Default::default(), Default::default())
                    .unwrap(),
            ),
        );
        (connector, task_stopped)
    }

    async fn stops_within_a_second(task_stopped: oneshot::Receiver<()>) -> bool {
        tokio::time::timeout(Duration::from_secs(1), task_stopped)
            .await
            .is_ok()
    }

    #[tokio::test]
    async fn dropping_connector_stops_its_task() {
        let cache = XdsCache::new();
        cache.update_cluster(CLUSTER, cluster());
        let (connector, mut task_stopped) = spawn_connector(&cache);
        let endpoint = XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080));
        assert_eq!(connector.connect(&endpoint).await, "10.0.0.1:8080");
        assert_eq!(task_stopped.try_recv(), Err(TryRecvError::Empty));

        // The cluster is still in the cache, so only the drop can stop it.
        drop(connector);
        assert!(stops_within_a_second(task_stopped).await);
    }

    #[tokio::test]
    async fn cluster_removed_before_task_runs_stops_task() {
        let cache = XdsCache::new();
        cache.update_cluster(CLUSTER, cluster());
        let (_connector, task_stopped) = spawn_connector(&cache);

        // The task has not run yet. Had it subscribed only now, it would
        // recreate the entry and wait on it forever.
        cache.remove_cluster(CLUSTER);
        assert!(stops_within_a_second(task_stopped).await);
    }
}
