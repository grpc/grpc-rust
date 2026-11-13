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

//! Crate-owned LRS message types.
//!
//! These types are codegen-agnostic and serve as the interface between
//! the LRS client logic and the codec layer. The codec converts these
//! to/from the wire format (e.g., prost/envoy-types or google-protobuf).

use std::time::Duration;

use crate::message::Locality;
use crate::message::Node;

/// A load stats request to send to the LRS server.
///
/// This struct borrows data to avoid unnecessary allocations when encoding.
/// The data only needs to live long enough for the codec to encode it.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct LoadStatsRequest<'a> {
    /// The node reporting load.
    pub node: &'a Node,
    /// Client features to advertise in the request's node, as
    /// `Node.client_features`. Only set on the initial request.
    pub client_features: &'a [&'a str],
    /// Load reports. Empty for the initial request.
    pub cluster_stats: &'a [ClusterStats],
}

impl<'a> LoadStatsRequest<'a> {
    /// Create a new load stats request for the given node, without client
    /// features or load reports.
    pub fn new(node: &'a Node) -> Self {
        Self {
            node,
            client_features: &[],
            cluster_stats: &[],
        }
    }

    /// Set the client features to advertise.
    pub fn with_client_features(mut self, client_features: &'a [&'a str]) -> Self {
        self.client_features = client_features;
        self
    }

    /// Set the load reports to send.
    pub fn with_cluster_stats(mut self, cluster_stats: &'a [ClusterStats]) -> Self {
        self.cluster_stats = cluster_stats;
        self
    }
}

/// Load stats for one (cluster, EDS service) over one reporting interval.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct ClusterStats {
    /// Name of the cluster.
    pub cluster_name: String,
    /// EDS service name of the cluster, or empty if it has none. Encoded as
    /// `cluster_service_name`.
    pub eds_service_name: String,
    /// Stats for each locality that had calls in the interval.
    pub upstream_locality_stats: Vec<UpstreamLocalityStats>,
    /// Number of calls dropped by the client, including drops without a
    /// category.
    pub total_dropped_requests: u64,
    /// Number of calls dropped for each drop category.
    pub dropped_requests: Vec<DroppedRequests>,
    /// Time covered by this report.
    pub load_report_interval: Duration,
}

/// Load stats for the calls sent to one locality.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct UpstreamLocalityStats {
    /// The locality the calls were sent to.
    pub locality: Locality,
    /// Calls that finished successfully in the interval.
    pub total_successful_requests: u64,
    /// Calls in progress at the time of the report.
    pub total_requests_in_progress: u64,
    /// Calls that failed in the interval.
    pub total_error_requests: u64,
    /// Calls started in the interval.
    pub total_issued_requests: u64,
    /// CPU utilization reported by endpoints in the interval, if propagated.
    pub cpu_utilization: Option<UnnamedEndpointLoadMetricStats>,
    /// Memory utilization reported by endpoints in the interval, if propagated.
    pub mem_utilization: Option<UnnamedEndpointLoadMetricStats>,
    /// Application utilization reported by endpoints in the interval, if
    /// propagated.
    pub application_utilization: Option<UnnamedEndpointLoadMetricStats>,
    /// Named backend metrics reported by endpoints in the interval, named
    /// `named_metrics.<key>`.
    pub load_metric_stats: Vec<EndpointLoadMetricStats>,
}

/// A backend metric reported in a top-level field of [`UpstreamLocalityStats`],
/// aggregated over the calls that reported it.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct UnnamedEndpointLoadMetricStats {
    /// Number of calls that finished with this metric.
    pub num_requests_finished_with_metric: u64,
    /// Sum of the metric's values over those calls.
    pub total_metric_value: f64,
}

/// A named backend metric, aggregated over the calls that reported it.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct EndpointLoadMetricStats {
    /// Name of the metric.
    pub metric_name: String,
    /// Number of calls that finished with this metric.
    pub num_requests_finished_with_metric: u64,
    /// Sum of the metric's values over those calls.
    pub total_metric_value: f64,
}

/// Number of calls dropped for one drop category.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct DroppedRequests {
    /// The drop category, from the EDS `drop_overloads`.
    pub category: String,
    /// Number of calls dropped for this category.
    pub dropped_count: u64,
}

/// A load stats response from the LRS server.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct LoadStatsResponse {
    /// Clusters to report stats for. Ignored if `send_all_clusters` is true.
    pub clusters: Vec<String>,
    /// If true, the client should send stats for all clusters it knows about.
    pub send_all_clusters: bool,
    /// The minimum interval of time to collect stats over.
    pub load_reporting_interval: Duration,
}

impl LoadStatsResponse {
    /// Create a new load stats response with the given reporting interval.
    ///
    /// Other fields default to empty/false and can be set using builder
    /// methods.
    pub fn new(load_reporting_interval: Duration) -> Self {
        Self {
            load_reporting_interval,
            ..Default::default()
        }
    }

    /// Set the clusters to report stats for.
    pub fn with_clusters(mut self, clusters: Vec<String>) -> Self {
        self.clusters = clusters;
        self
    }

    /// Set whether stats should be sent for all clusters.
    pub fn with_send_all_clusters(mut self, send_all_clusters: bool) -> Self {
        self.send_all_clusters = send_all_clusters;
        self
    }
}
