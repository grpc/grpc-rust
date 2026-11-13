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

//! Prost-based codec using envoy-types.

use std::time::Duration;

use crate::codec::XdsCodec;
use crate::error::{Error, Result};
use crate::lrs::LrsCodec;
use crate::lrs::message::{
    ClusterStats, EndpointLoadMetricStats, LoadStatsRequest, LoadStatsResponse,
    UnnamedEndpointLoadMetricStats, UpstreamLocalityStats,
};
use crate::message::{
    DiscoveryRequest, DiscoveryResponse, Locality, MetadataValue, Node, ResourceAny,
};
use bytes::Bytes;
use prost::Message;

/// A codec that uses prost/envoy-types for serialization of ADS and LRS
/// messages.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProstCodec;

impl ProstCodec {
    /// Encodes `node` as an `envoy.config.core.v3.Node` message, exactly as it
    /// is sent in ADS requests. A config dump such as CSDS (gRFC A40) reports
    /// this node.
    pub fn encode_node(&self, node: &Node) -> Bytes {
        node_to_proto(node).encode_to_vec().into()
    }
}

impl XdsCodec for ProstCodec {
    fn encode_request(&self, request: &DiscoveryRequest<'_>) -> Result<Bytes> {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        use envoy_types::pb::google::rpc::Status;

        let proto_request = discovery::DiscoveryRequest {
            version_info: request.version_info.to_owned(),
            node: Some(node_to_proto(request.node)),
            resource_names: request.resource_names.to_vec(),
            type_url: request.type_url.to_owned(),
            response_nonce: request.response_nonce.to_owned(),
            error_detail: request.error_detail.as_ref().map(|e| Status {
                code: e.code,
                message: e.message.clone(),
                details: vec![],
            }),
            ..Default::default()
        };

        Ok(proto_request.encode_to_vec().into())
    }

    fn decode_response(&self, bytes: Bytes) -> Result<DiscoveryResponse> {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;

        let proto_response = discovery::DiscoveryResponse::decode(bytes).map_err(Error::Decode)?;

        Ok(DiscoveryResponse {
            version_info: proto_response.version_info,
            resources: proto_response
                .resources
                .into_iter()
                .map(|any| ResourceAny {
                    type_url: any.type_url,
                    value: any.value.into(),
                })
                .collect(),
            type_url: proto_response.type_url,
            nonce: proto_response.nonce,
        })
    }
}

impl LrsCodec for ProstCodec {
    fn encode_load_stats_request(&self, request: &LoadStatsRequest<'_>) -> Result<Bytes> {
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;

        let mut node = node_to_proto(request.node);
        node.client_features = request
            .client_features
            .iter()
            .map(|&feature| feature.to_owned())
            .collect();
        let proto_request = load_stats::LoadStatsRequest {
            node: Some(node),
            cluster_stats: request
                .cluster_stats
                .iter()
                .map(cluster_stats_to_proto)
                .collect(),
        };

        Ok(proto_request.encode_to_vec().into())
    }

    /// Decodes a `LoadStatsResponse`.
    ///
    /// Fails if `load_reporting_interval` is missing or invalid. A negative
    /// interval decodes as zero. `report_endpoint_granularity` is ignored, as
    /// per-endpoint stats aren't supported.
    fn decode_load_stats_response(&self, bytes: Bytes) -> Result<LoadStatsResponse> {
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;

        let proto_response = load_stats::LoadStatsResponse::decode(bytes).map_err(Error::Decode)?;
        let interval = proto_response
            .load_reporting_interval
            .as_ref()
            .ok_or_else(|| Error::Validation("missing load_reporting_interval".to_owned()))?;
        let interval = duration_from_proto(interval)?;

        Ok(LoadStatsResponse::new(interval)
            .with_clusters(proto_response.clusters)
            .with_send_all_clusters(proto_response.send_all_clusters))
    }
}

/// Convert a [`Node`] to its `envoy.config.core.v3.Node` wire form.
fn node_to_proto(node: &Node) -> envoy_types::pb::envoy::config::core::v3::Node {
    use envoy_types::pb::envoy::config::core::v3 as core;
    use envoy_types::pb::google::protobuf::Struct;

    let metadata = if node.metadata.is_empty() {
        None
    } else {
        Some(Struct {
            fields: node
                .metadata
                .iter()
                .map(|(k, v)| (k.clone(), metadata_to_proto(v)))
                .collect(),
        })
    };

    core::Node {
        id: node.id.clone().unwrap_or_default(),
        cluster: node.cluster.clone().unwrap_or_default(),
        user_agent_name: node.user_agent_name.clone(),
        user_agent_version_type: Some(core::node::UserAgentVersionType::UserAgentVersion(
            node.user_agent_version.clone(),
        )),
        locality: node.locality.as_ref().map(locality_to_proto),
        metadata,
        ..Default::default()
    }
}

/// Convert a [`Locality`] to its `envoy.config.core.v3.Locality` wire form.
fn locality_to_proto(locality: &Locality) -> envoy_types::pb::envoy::config::core::v3::Locality {
    use envoy_types::pb::envoy::config::core::v3 as core;

    core::Locality {
        region: locality.region.clone(),
        zone: locality.zone.clone(),
        sub_zone: locality.sub_zone.clone(),
    }
}

/// Convert a [`MetadataValue`] to its `google.protobuf.Value` wire form.
fn metadata_to_proto(value: &MetadataValue) -> envoy_types::pb::google::protobuf::Value {
    use envoy_types::pb::google::protobuf::{ListValue, Struct, Value, value::Kind};

    let kind = match value {
        MetadataValue::Null => Kind::NullValue(0),
        MetadataValue::Bool(b) => Kind::BoolValue(*b),
        MetadataValue::Number(n) => Kind::NumberValue(*n),
        MetadataValue::String(s) => Kind::StringValue(s.clone()),
        MetadataValue::Array(items) => Kind::ListValue(ListValue {
            values: items.iter().map(metadata_to_proto).collect(),
        }),
        MetadataValue::Object(fields) => Kind::StructValue(Struct {
            fields: fields
                .iter()
                .map(|(k, v)| (k.clone(), metadata_to_proto(v)))
                .collect(),
        }),
    };
    Value { kind: Some(kind) }
}

/// Convert a [`ClusterStats`] to its `envoy.config.endpoint.v3.ClusterStats`
/// wire form.
fn cluster_stats_to_proto(
    stats: &ClusterStats,
) -> envoy_types::pb::envoy::config::endpoint::v3::ClusterStats {
    use envoy_types::pb::envoy::config::endpoint::v3 as endpoint;

    endpoint::ClusterStats {
        cluster_name: stats.cluster_name.clone(),
        cluster_service_name: stats.eds_service_name.clone(),
        upstream_locality_stats: stats
            .upstream_locality_stats
            .iter()
            .map(upstream_locality_stats_to_proto)
            .collect(),
        total_dropped_requests: stats.total_dropped_requests,
        dropped_requests: stats
            .dropped_requests
            .iter()
            .map(|dropped| endpoint::cluster_stats::DroppedRequests {
                category: dropped.category.clone(),
                dropped_count: dropped.dropped_count,
            })
            .collect(),
        load_report_interval: Some(duration_to_proto(stats.load_report_interval)),
    }
}

/// Convert an [`UpstreamLocalityStats`] to its
/// `envoy.config.endpoint.v3.UpstreamLocalityStats` wire form.
fn upstream_locality_stats_to_proto(
    stats: &UpstreamLocalityStats,
) -> envoy_types::pb::envoy::config::endpoint::v3::UpstreamLocalityStats {
    use envoy_types::pb::envoy::config::endpoint::v3 as endpoint;

    endpoint::UpstreamLocalityStats {
        locality: Some(locality_to_proto(&stats.locality)),
        total_successful_requests: stats.total_successful_requests,
        total_requests_in_progress: stats.total_requests_in_progress,
        total_error_requests: stats.total_error_requests,
        total_issued_requests: stats.total_issued_requests,
        cpu_utilization: stats.cpu_utilization.as_ref().map(unnamed_metric_to_proto),
        mem_utilization: stats.mem_utilization.as_ref().map(unnamed_metric_to_proto),
        application_utilization: stats
            .application_utilization
            .as_ref()
            .map(unnamed_metric_to_proto),
        load_metric_stats: stats
            .load_metric_stats
            .iter()
            .map(named_metric_to_proto)
            .collect(),
        ..Default::default()
    }
}

/// Convert an [`UnnamedEndpointLoadMetricStats`] to its wire form.
fn unnamed_metric_to_proto(
    stats: &UnnamedEndpointLoadMetricStats,
) -> envoy_types::pb::envoy::config::endpoint::v3::UnnamedEndpointLoadMetricStats {
    use envoy_types::pb::envoy::config::endpoint::v3 as endpoint;

    endpoint::UnnamedEndpointLoadMetricStats {
        num_requests_finished_with_metric: stats.num_requests_finished_with_metric,
        total_metric_value: stats.total_metric_value,
    }
}

/// Convert an [`EndpointLoadMetricStats`] to its wire form.
fn named_metric_to_proto(
    stats: &EndpointLoadMetricStats,
) -> envoy_types::pb::envoy::config::endpoint::v3::EndpointLoadMetricStats {
    use envoy_types::pb::envoy::config::endpoint::v3 as endpoint;

    endpoint::EndpointLoadMetricStats {
        metric_name: stats.metric_name.clone(),
        num_requests_finished_with_metric: stats.num_requests_finished_with_metric,
        total_metric_value: stats.total_metric_value,
    }
}

/// Convert a [`Duration`] to its `google.protobuf.Duration` wire form,
/// saturating at the longest duration the wire form can represent.
fn duration_to_proto(duration: Duration) -> envoy_types::pb::google::protobuf::Duration {
    use envoy_types::pb::google::protobuf::Duration as ProtoDuration;

    ProtoDuration {
        seconds: i64::try_from(duration.as_secs()).unwrap_or(i64::MAX),
        // Always less than 1,000,000,000, so it fits.
        nanos: duration.subsec_nanos() as i32,
    }
}

/// Convert a `google.protobuf.Duration` to a [`Duration`].
///
/// Fails if `nanos` is out of range, or has a different sign than `seconds`.
/// Negative durations convert to zero.
fn duration_from_proto(duration: &envoy_types::pb::google::protobuf::Duration) -> Result<Duration> {
    let (seconds, nanos) = (duration.seconds, duration.nanos);
    let nanos_in_range = (-999_999_999..=999_999_999).contains(&nanos);
    let signs_match = !(seconds > 0 && nanos < 0 || seconds < 0 && nanos > 0);
    if !nanos_in_range || !signs_match {
        return Err(Error::Validation(format!(
            "invalid duration: {seconds}s {nanos}ns"
        )));
    }
    if seconds < 0 || nanos < 0 {
        return Ok(Duration::ZERO);
    }
    Ok(Duration::new(seconds.unsigned_abs(), nanos.unsigned_abs()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{ErrorDetail, Locality, Node};

    #[test]
    fn test_encode_request_minimal() {
        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0");
        let resource_names = vec!["listener-1".to_string()];
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener",
            resource_names: &resource_names,
            response_nonce: "",
            error_detail: None,
        };

        let bytes = codec.encode_request(&request).unwrap();
        assert!(!bytes.is_empty());

        // Verify we can decode it back with prost
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        let decoded = discovery::DiscoveryRequest::decode(bytes).unwrap();
        assert_eq!(decoded.type_url, request.type_url);
        assert_eq!(decoded.resource_names, request.resource_names);
    }

    #[test]
    fn test_encode_request_with_node() {
        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0")
            .with_id("node-1")
            .with_cluster("cluster-1")
            .with_locality(Locality {
                region: "us-west".to_string(),
                zone: "us-west-1a".to_string(),
                sub_zone: "rack-1".to_string(),
            });
        let resource_names: Vec<String> = Vec::new();
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster",
            resource_names: &resource_names,
            response_nonce: "",
            error_detail: None,
        };

        let bytes = codec.encode_request(&request).unwrap();

        use envoy_types::pb::envoy::config::core::v3 as core;
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        let decoded = discovery::DiscoveryRequest::decode(bytes).unwrap();
        let node = decoded.node.unwrap();
        assert_eq!(node.id, "node-1");
        assert_eq!(node.cluster, "cluster-1");
        assert_eq!(node.user_agent_name, "grpc");
        // Verify user_agent_version is properly encoded
        match node.user_agent_version_type {
            Some(core::node::UserAgentVersionType::UserAgentVersion(version)) => {
                assert_eq!(version, "1.0");
            }
            _ => panic!("Expected UserAgentVersion to be set"),
        }
        let locality = node.locality.unwrap();
        assert_eq!(locality.region, "us-west");
        assert_eq!(locality.zone, "us-west-1a");
        assert_eq!(locality.sub_zone, "rack-1");
    }

    #[test]
    fn test_encode_request_with_metadata() {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        use envoy_types::pb::google::protobuf::value::Kind;
        use std::collections::HashMap;

        let codec = ProstCodec;
        let mut metadata = HashMap::new();
        metadata.insert(
            "GENERATOR".to_string(),
            MetadataValue::String("grpc".to_string()),
        );
        let node = Node::new("grpc", "1.0").with_metadata(metadata);
        let resource_names: Vec<String> = Vec::new();
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster",
            resource_names: &resource_names,
            response_nonce: "",
            error_detail: None,
        };

        let bytes = codec.encode_request(&request).unwrap();
        let decoded = discovery::DiscoveryRequest::decode(bytes).unwrap();
        let metadata_struct = decoded.node.unwrap().metadata.unwrap();

        let value = metadata_struct.fields.get("GENERATOR").unwrap();
        match &value.kind {
            Some(Kind::StringValue(s)) => assert_eq!(s, "grpc"),
            other => panic!("expected StringValue, got {other:?}"),
        }
    }

    #[test]
    fn test_encode_request_with_nested_metadata() {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        use envoy_types::pb::google::protobuf::value::Kind;
        use std::collections::HashMap;

        let codec = ProstCodec;
        let mut annotations = HashMap::new();
        annotations.insert(
            "istio.io/rev".to_string(),
            MetadataValue::String("default".to_string()),
        );

        let mut metadata = HashMap::new();
        metadata.insert(
            "ANNOTATIONS".to_string(),
            MetadataValue::Object(annotations),
        );
        metadata.insert(
            "ENVOY_PROMETHEUS_PORT".to_string(),
            MetadataValue::Number(15090.0),
        );
        metadata.insert(
            "PILOT_SAN".to_string(),
            MetadataValue::Array(vec![MetadataValue::String(
                "istiod.istio-system.svc".to_string(),
            )]),
        );

        let node = Node::new("grpc", "1.0").with_metadata(metadata);
        let resource_names: Vec<String> = Vec::new();
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster",
            resource_names: &resource_names,
            response_nonce: "",
            error_detail: None,
        };

        let bytes = codec.encode_request(&request).unwrap();
        let decoded = discovery::DiscoveryRequest::decode(bytes).unwrap();
        let fields = decoded.node.unwrap().metadata.unwrap().fields;

        match &fields.get("ANNOTATIONS").unwrap().kind {
            Some(Kind::StructValue(s)) => match &s.fields.get("istio.io/rev").unwrap().kind {
                Some(Kind::StringValue(v)) => assert_eq!(v, "default"),
                other => panic!("expected StringValue, got {other:?}"),
            },
            other => panic!("expected StructValue, got {other:?}"),
        }

        match &fields.get("ENVOY_PROMETHEUS_PORT").unwrap().kind {
            Some(Kind::NumberValue(n)) => assert_eq!(*n, 15090.0),
            other => panic!("expected NumberValue, got {other:?}"),
        }

        match &fields.get("PILOT_SAN").unwrap().kind {
            Some(Kind::ListValue(l)) => {
                assert_eq!(l.values.len(), 1);
                match &l.values[0].kind {
                    Some(Kind::StringValue(s)) => assert_eq!(s, "istiod.istio-system.svc"),
                    other => panic!("expected StringValue, got {other:?}"),
                }
            }
            other => panic!("expected ListValue, got {other:?}"),
        }
    }

    #[test]
    fn test_encode_request_empty_metadata_omits_struct() {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;

        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0");
        let resource_names: Vec<String> = Vec::new();
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster",
            resource_names: &resource_names,
            response_nonce: "",
            error_detail: None,
        };

        let bytes = codec.encode_request(&request).unwrap();
        let decoded = discovery::DiscoveryRequest::decode(bytes).unwrap();
        assert!(decoded.node.unwrap().metadata.is_none());
    }

    #[test]
    fn test_decode_response() {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;
        use envoy_types::pb::google::protobuf::Any;

        let proto_response = discovery::DiscoveryResponse {
            version_info: "1".to_string(),
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
            nonce: "nonce-1".to_string(),
            resources: vec![Any {
                type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
                value: b"fake-listener-bytes".to_vec(),
            }],
            ..Default::default()
        };

        let bytes: Bytes = proto_response.encode_to_vec().into();

        let codec = ProstCodec;
        let response = codec.decode_response(bytes).unwrap();

        assert_eq!(response.version_info, "1");
        assert_eq!(
            response.type_url,
            "type.googleapis.com/envoy.config.listener.v3.Listener"
        );
        assert_eq!(response.nonce, "nonce-1");
        assert_eq!(response.resources.len(), 1);
        assert_eq!(
            response.resources[0].type_url,
            "type.googleapis.com/envoy.config.listener.v3.Listener"
        );
        assert_eq!(response.resources[0].value.as_ref(), b"fake-listener-bytes");
    }

    #[test]
    fn test_roundtrip() {
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;

        let codec = ProstCodec;

        let node = Node::new("grpc", "1.0");
        let resource_names = vec!["res-1".to_string(), "res-2".to_string()];
        let request = DiscoveryRequest {
            version_info: "42",
            node: &node,
            type_url: "type.googleapis.com/test.Resource",
            resource_names: &resource_names,
            response_nonce: "nonce-abc",
            error_detail: Some(ErrorDetail {
                code: 3, // INVALID_ARGUMENT
                message: "validation failed".to_string(),
            }),
        };

        let request_bytes = codec.encode_request(&request).unwrap();

        let proto_request = discovery::DiscoveryRequest::decode(request_bytes).unwrap();
        assert_eq!(proto_request.version_info, "42");
        assert_eq!(proto_request.response_nonce, "nonce-abc");
        let error = proto_request.error_detail.unwrap();
        assert_eq!(error.code, 3);
        assert_eq!(error.message, "validation failed");
    }

    #[test]
    fn test_encode_node_matches_request_node() {
        use std::collections::HashMap;

        use envoy_types::pb::envoy::config::core::v3 as core;
        use envoy_types::pb::envoy::service::discovery::v3 as discovery;

        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0")
            .with_id("node-1")
            .with_cluster("cluster-1")
            .with_locality(Locality {
                region: "us-west".to_string(),
                zone: "us-west-1a".to_string(),
                sub_zone: "rack-1".to_string(),
            })
            .with_metadata(HashMap::from([(
                "key".to_string(),
                MetadataValue::String("value".to_string()),
            )]));
        let request = DiscoveryRequest {
            version_info: "",
            node: &node,
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster",
            resource_names: &[],
            response_nonce: "",
            error_detail: None,
        };

        let request =
            discovery::DiscoveryRequest::decode(codec.encode_request(&request).unwrap()).unwrap();
        let encoded = core::Node::decode(codec.encode_node(&node)).unwrap();
        assert_eq!(Some(encoded), request.node);
    }

    #[test]
    fn test_encode_initial_load_stats_request() {
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;

        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0").with_id("node-1");
        let request = LoadStatsRequest::new(&node)
            .with_client_features(&["envoy.lrs.supports_send_all_clusters"]);

        let bytes = codec.encode_load_stats_request(&request).unwrap();
        let decoded = load_stats::LoadStatsRequest::decode(bytes).unwrap();

        // The node is encoded as in ADS requests, with the client features.
        let mut want_node = node_to_proto(&node);
        want_node.client_features = vec!["envoy.lrs.supports_send_all_clusters".to_string()];
        assert_eq!(decoded.node, Some(want_node));
        assert!(decoded.cluster_stats.is_empty());
    }

    #[test]
    fn test_encode_load_stats_request_with_cluster_stats() {
        use crate::lrs::message::DroppedRequests;
        use envoy_types::pb::envoy::config::core::v3 as core;
        use envoy_types::pb::envoy::config::endpoint::v3 as endpoint;
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;
        use envoy_types::pb::google::protobuf::Duration as ProtoDuration;

        let codec = ProstCodec;
        let node = Node::new("grpc", "1.0");
        let cluster_stats = [ClusterStats {
            cluster_name: "cluster-1".to_string(),
            eds_service_name: "eds-1".to_string(),
            upstream_locality_stats: vec![UpstreamLocalityStats {
                locality: Locality {
                    region: "us-west".to_string(),
                    zone: "us-west-1a".to_string(),
                    sub_zone: "rack-1".to_string(),
                },
                total_successful_requests: 1,
                total_requests_in_progress: 2,
                total_error_requests: 3,
                total_issued_requests: 4,
                cpu_utilization: Some(UnnamedEndpointLoadMetricStats {
                    num_requests_finished_with_metric: 5,
                    total_metric_value: 0.5,
                }),
                mem_utilization: None,
                application_utilization: Some(UnnamedEndpointLoadMetricStats {
                    num_requests_finished_with_metric: 6,
                    total_metric_value: 1.5,
                }),
                load_metric_stats: vec![EndpointLoadMetricStats {
                    metric_name: "named_metrics.foo".to_string(),
                    num_requests_finished_with_metric: 7,
                    total_metric_value: 2.5,
                }],
            }],
            total_dropped_requests: 10,
            dropped_requests: vec![DroppedRequests {
                category: "throttle".to_string(),
                dropped_count: 8,
            }],
            load_report_interval: Duration::new(10, 500_000_000),
        }];
        let request = LoadStatsRequest::new(&node).with_cluster_stats(&cluster_stats);

        let bytes = codec.encode_load_stats_request(&request).unwrap();
        let decoded = load_stats::LoadStatsRequest::decode(bytes).unwrap();

        // Without client features, the node is encoded exactly as in ADS
        // requests.
        assert_eq!(decoded.node, Some(node_to_proto(&node)));
        let want = endpoint::ClusterStats {
            cluster_name: "cluster-1".to_string(),
            cluster_service_name: "eds-1".to_string(),
            upstream_locality_stats: vec![endpoint::UpstreamLocalityStats {
                locality: Some(core::Locality {
                    region: "us-west".to_string(),
                    zone: "us-west-1a".to_string(),
                    sub_zone: "rack-1".to_string(),
                }),
                total_successful_requests: 1,
                total_requests_in_progress: 2,
                total_error_requests: 3,
                total_issued_requests: 4,
                cpu_utilization: Some(endpoint::UnnamedEndpointLoadMetricStats {
                    num_requests_finished_with_metric: 5,
                    total_metric_value: 0.5,
                }),
                mem_utilization: None,
                application_utilization: Some(endpoint::UnnamedEndpointLoadMetricStats {
                    num_requests_finished_with_metric: 6,
                    total_metric_value: 1.5,
                }),
                load_metric_stats: vec![endpoint::EndpointLoadMetricStats {
                    metric_name: "named_metrics.foo".to_string(),
                    num_requests_finished_with_metric: 7,
                    total_metric_value: 2.5,
                }],
                ..Default::default()
            }],
            total_dropped_requests: 10,
            dropped_requests: vec![endpoint::cluster_stats::DroppedRequests {
                category: "throttle".to_string(),
                dropped_count: 8,
            }],
            load_report_interval: Some(ProtoDuration {
                seconds: 10,
                nanos: 500_000_000,
            }),
        };
        assert_eq!(decoded.cluster_stats, vec![want]);
    }

    #[test]
    fn test_decode_load_stats_response() {
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;
        use envoy_types::pb::google::protobuf::Duration as ProtoDuration;

        let proto_response = load_stats::LoadStatsResponse {
            clusters: vec!["cluster-1".to_string(), "cluster-2".to_string()],
            send_all_clusters: true,
            load_reporting_interval: Some(ProtoDuration {
                seconds: 10,
                nanos: 500_000_000,
            }),
            report_endpoint_granularity: true,
        };
        let bytes: Bytes = proto_response.encode_to_vec().into();

        let response = ProstCodec.decode_load_stats_response(bytes).unwrap();
        assert_eq!(response.clusters, vec!["cluster-1", "cluster-2"]);
        assert!(response.send_all_clusters);
        assert_eq!(
            response.load_reporting_interval,
            Duration::new(10, 500_000_000)
        );
    }

    #[test]
    fn test_decode_load_stats_response_without_interval_fails() {
        use envoy_types::pb::envoy::service::load_stats::v3 as load_stats;

        let proto_response = load_stats::LoadStatsResponse {
            clusters: vec!["cluster-1".to_string()],
            ..Default::default()
        };
        let bytes: Bytes = proto_response.encode_to_vec().into();

        let err = ProstCodec.decode_load_stats_response(bytes).unwrap_err();
        assert!(matches!(err, Error::Validation(_)), "got {err:?}");
    }

    #[test]
    fn test_decode_load_stats_response_invalid_bytes_fails() {
        let bytes = Bytes::from_static(b"\xff\xff\xff");
        let err = ProstCodec.decode_load_stats_response(bytes).unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "got {err:?}");
    }

    #[test]
    fn test_duration_from_proto() {
        use envoy_types::pb::google::protobuf::Duration as ProtoDuration;

        let convert = |seconds, nanos| duration_from_proto(&ProtoDuration { seconds, nanos });
        assert_eq!(convert(0, 0).unwrap(), Duration::ZERO);
        assert_eq!(convert(5, 1).unwrap(), Duration::new(5, 1));
        assert_eq!(
            convert(i64::MAX, 999_999_999).unwrap(),
            Duration::new(i64::MAX as u64, 999_999_999)
        );
        // Negative durations convert to zero.
        assert_eq!(convert(-5, 0).unwrap(), Duration::ZERO);
        assert_eq!(convert(-5, -1).unwrap(), Duration::ZERO);
        assert_eq!(convert(0, -1).unwrap(), Duration::ZERO);
        // Invalid durations.
        assert!(convert(0, 1_000_000_000).is_err());
        assert!(convert(0, -1_000_000_000).is_err());
        assert!(convert(5, -1).is_err());
        assert!(convert(-5, 1).is_err());
    }

    #[test]
    fn test_duration_to_proto() {
        use envoy_types::pb::google::protobuf::Duration as ProtoDuration;

        assert_eq!(
            duration_to_proto(Duration::new(5, 1)),
            ProtoDuration {
                seconds: 5,
                nanos: 1
            }
        );
        // Saturates at the longest representable duration.
        assert_eq!(
            duration_to_proto(Duration::MAX),
            ProtoDuration {
                seconds: i64::MAX,
                nanos: 999_999_999
            }
        );
    }
}
