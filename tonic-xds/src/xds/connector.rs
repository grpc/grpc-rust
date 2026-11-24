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
//! The same watch is where cluster-level load-balancing config is applied: the
//! outlier-detection config is sent over a [`watch`] channel the load
//! balancer's sweep listens to, and only once the connector built from the same
//! update has been published, so the two never disagree.
//!
//! Construction is synchronous even though the first connector only exists
//! once CDS arrives — `connect` awaits publication instead. That lets a load
//! balancer take the connector up front and start discovery before CDS
//! resolves.

use std::sync::Arc;

use tokio::sync::watch;

use crate::client::endpoint::{ClusterConfig, Connector, MakeConnector, XdsEndpoint};
use crate::common::async_util::{AbortOnDrop, BoxFuture};
use crate::xds::cache::CacheWatch;
#[cfg(feature = "_tls-any")]
use crate::xds::cert_provider::CertProviderRegistry;
use crate::xds::resource::ClusterResource;
use crate::xds::resource::outlier_detection::OutlierDetectionConfig;

/// A type-erased per-cluster connector, as published by CDS.
type SharedConnector<S> = Arc<dyn Connector<Service = S> + Send + Sync>;

/// A [`Connector`] that tracks one cluster's CDS resource.
///
/// Holds the receiving end of a [`watch`] channel fed by a spawned task; the
/// task exits (dropping the sender) when `cluster_watch` closes, and is
/// aborted when the connector is dropped.
pub(crate) struct ClusterConnector<S> {
    cluster_name: Arc<str>,
    rx: watch::Receiver<Option<SharedConnector<S>>>,
    outlier_detection: watch::Receiver<OutlierDetectionConfig>,
    _task: AbortOnDrop,
}

impl<S: Send + 'static> ClusterConnector<S> {
    /// Spawns a task consuming `cluster_watch` for `cluster_name` and returns
    /// a connector backed by it.
    pub(crate) fn spawn<MC>(
        cluster_watch: CacheWatch<Arc<ClusterResource>>,
        cluster_name: String,
        make_connector: Arc<MC>,
        #[cfg(feature = "_tls-any")] registry: Arc<CertProviderRegistry>,
    ) -> Self
    where
        MC: MakeConnector<Service = S>,
    {
        let (tx, rx) = watch::channel(None);
        let (outlier_tx, outlier_rx) = watch::channel(OutlierDetectionConfig::default());
        let task = tokio::spawn(publish_connectors(
            cluster_watch,
            cluster_name.clone(),
            make_connector,
            outlier_tx,
            #[cfg(feature = "_tls-any")]
            registry,
            tx,
        ));
        Self {
            cluster_name: Arc::from(cluster_name),
            rx,
            outlier_detection: outlier_rx,
            _task: AbortOnDrop(task),
        }
    }

    /// Handle to the cluster's live outlier-detection config, seeded disabled
    /// until the first CDS update lands. Callers that run a load balancer hand
    /// this to it; the balancer-less
    /// [`discover_cluster`](crate::xds::cluster_discovery) path ignores it.
    pub(crate) fn outlier_detection(&self) -> watch::Receiver<OutlierDetectionConfig> {
        self.outlier_detection.clone()
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
                        "cluster watches closed before any CDS update produced a \
                         connector; parking the connect until discovery drops it",
                    );
                    // The router dropped this cluster, which also closes its
                    // endpoints watch, so discovery drops this future.
                    return std::future::pending().await;
                }
            }
        })
    }
}

/// Background task: applies every accepted CDS update by republishing the
/// connector and the cluster's outlier-detection config.
async fn publish_connectors<MC, S>(
    mut cluster_watch: CacheWatch<Arc<ClusterResource>>,
    cluster_name: String,
    make_connector: Arc<MC>,
    outlier_tx: watch::Sender<OutlierDetectionConfig>,
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
                // Skip no-op updates so they don't wake the sweep.
                outlier_tx.send_if_modified(|current| {
                    let changed = *current != cluster.outlier_detection;
                    if changed {
                        *current = cluster.outlier_detection.clone();
                    }
                    changed
                });
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
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::sync::oneshot::{self, error::TryRecvError};
    use tower::BoxError;

    use super::*;
    use crate::client::endpoint::EndpointAddress;
    use crate::xds::resource::circuit_breaking::CircuitBreakingConfig;
    use crate::xds::resource::cluster::LbPolicy;
    use crate::xds::resource::outlier_detection::{FailurePercentageConfig, Percentage};

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
        cluster_with_outlier_detection(OutlierDetectionConfig::default())
    }

    fn cluster_with_outlier_detection(
        outlier_detection: OutlierDetectionConfig,
    ) -> Arc<ClusterResource> {
        Arc::new(ClusterResource {
            name: CLUSTER.to_string(),
            eds_service_name: None,
            lb_policy: LbPolicy::RoundRobin,
            security: None,
            circuit_breaking: CircuitBreakingConfig::default(),
            outlier_detection,
        })
    }

    /// Polls `cond` across task yields so the spawned CDS watch gets a
    /// chance to apply the update it was woken for.
    async fn eventually(mut cond: impl FnMut() -> bool) -> bool {
        for _ in 0..100 {
            if cond() {
                return true;
            }
            tokio::task::yield_now().await;
        }
        false
    }

    fn enabled(interval_secs: u64) -> OutlierDetectionConfig {
        OutlierDetectionConfig {
            interval: Duration::from_secs(interval_secs),
            failure_percentage: Some(FailurePercentageConfig {
                threshold: Percentage::new(85).unwrap(),
                enforcing_failure_percentage: Percentage::new(100).unwrap(),
                minimum_hosts: 5,
                request_volume: 50,
            }),
            ..OutlierDetectionConfig::default()
        }
    }

    /// Builds [`AddrConnector`]s, counting calls and failing on demand.
    #[derive(Default)]
    struct ScriptedMakeConnector {
        fail: AtomicBool,
        calls: AtomicUsize,
    }

    impl MakeConnector for ScriptedMakeConnector {
        type Service = String;

        fn make_connector(
            &self,
            _cluster: ClusterConfig<'_>,
        ) -> Result<SharedConnector<String>, BoxError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.fail.load(Ordering::SeqCst) {
                return Err("rejected".into());
            }
            Ok(Arc::new(AddrConnector))
        }
    }

    fn spawn_with<MC: MakeConnector<Service = String>>(
        cluster_watch: CacheWatch<Arc<ClusterResource>>,
        make_connector: Arc<MC>,
    ) -> ClusterConnector<String> {
        ClusterConnector::spawn(
            cluster_watch,
            CLUSTER.to_string(),
            make_connector,
            #[cfg(feature = "_tls-any")]
            Arc::new(
                CertProviderRegistry::from_bootstrap(&Default::default(), Default::default())
                    .unwrap(),
            ),
        )
    }

    /// Spawns a connector for [`CLUSTER`], along with a receiver that
    /// resolves once the connector's CDS task stops.
    fn spawn_connector(
        cluster_watch: CacheWatch<Arc<ClusterResource>>,
    ) -> (ClusterConnector<String>, oneshot::Receiver<()>) {
        let (dropped, task_stopped) = oneshot::channel();
        let connector = spawn_with(
            cluster_watch,
            Arc::new(AddrMakeConnector { _dropped: dropped }),
        );
        (connector, task_stopped)
    }

    async fn stops_within_a_second(task_stopped: oneshot::Receiver<()>) -> bool {
        tokio::time::timeout(Duration::from_secs(1), task_stopped)
            .await
            .is_ok()
    }

    #[tokio::test]
    async fn cds_updates_republish_the_outlier_detection_config() {
        let (tx, watch) = CacheWatch::channel(Some(cluster()));
        let (connector, _stopped) = spawn_connector(watch);
        let live = connector.outlier_detection();

        // The config is sent right after the connector it belongs to, so a
        // connect that has resolved implies the first update was applied.
        let endpoint = XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080));
        connector.connect(&endpoint).await;
        assert_eq!(*live.borrow(), OutlierDetectionConfig::default());

        tx.send(cluster_with_outlier_detection(enabled(7)));
        assert!(eventually(|| *live.borrow() == enabled(7)).await);

        tx.send(cluster());
        assert!(eventually(|| *live.borrow() == OutlierDetectionConfig::default()).await);
    }

    #[tokio::test]
    async fn outlier_detection_config_follows_the_connector_it_came_with() {
        let (tx, watch) = CacheWatch::channel(Some(cluster()));
        let make = Arc::new(ScriptedMakeConnector::default());
        let connector = spawn_with(watch, make.clone());
        let live = connector.outlier_detection();
        let endpoint = XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080));
        connector.connect(&endpoint).await;

        make.fail.store(true, Ordering::SeqCst);
        tx.send(cluster_with_outlier_detection(enabled(7)));
        assert!(eventually(|| make.calls.load(Ordering::SeqCst) == 2).await);
        for _ in 0..20 {
            tokio::task::yield_now().await;
        }
        assert!(
            !live.borrow().is_enabled(),
            "a rejected update must not change ejection config"
        );

        make.fail.store(false, Ordering::SeqCst);
        tx.send(cluster_with_outlier_detection(enabled(8)));
        assert!(eventually(|| *live.borrow() == enabled(8)).await);
    }

    #[tokio::test]
    async fn an_unchanged_outlier_detection_config_does_not_wake_listeners() {
        let (tx, watch) = CacheWatch::channel(Some(cluster_with_outlier_detection(enabled(7))));
        let make = Arc::new(ScriptedMakeConnector::default());
        let connector = spawn_with(watch, make.clone());
        let mut live = connector.outlier_detection();
        let endpoint = XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080));
        connector.connect(&endpoint).await;
        assert!(eventually(|| *live.borrow_and_update() == enabled(7)).await);

        // Same outlier detection, different cluster.
        let mut next = (*cluster_with_outlier_detection(enabled(7))).clone();
        next.eds_service_name = Some("other".to_string());
        tx.send(Arc::new(next));
        assert!(eventually(|| make.calls.load(Ordering::SeqCst) == 2).await);
        for _ in 0..20 {
            tokio::task::yield_now().await;
        }
        assert!(!live.has_changed().unwrap());
    }

    #[tokio::test]
    async fn dropping_connector_stops_its_task() {
        let (_tx, watch) = CacheWatch::channel(Some(cluster()));
        let (connector, mut task_stopped) = spawn_connector(watch);
        let endpoint = XdsEndpoint::new(EndpointAddress::new("10.0.0.1", 8080));
        assert_eq!(connector.connect(&endpoint).await, "10.0.0.1:8080");
        assert_eq!(task_stopped.try_recv(), Err(TryRecvError::Empty));

        // `_tx` stays open, so only dropping `connector` can stop the task.
        drop(connector);
        assert!(stops_within_a_second(task_stopped).await);
    }

    #[tokio::test]
    async fn cluster_watch_closed_before_task_runs_stops_task() {
        let (tx, watch) = CacheWatch::channel(Some(cluster()));
        let (_connector, task_stopped) = spawn_connector(watch);

        // Dropping the sender before the task runs closes `watch` immediately.
        drop(tx);
        assert!(stops_within_a_second(task_stopped).await);
    }
}
