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
//! Helper functions for building xDS configuration resources for testing.

use envoy_types::pb::envoy::config::cluster::v3::Cluster;
use envoy_types::pb::envoy::config::cluster::v3::cluster::{ClusterDiscoveryType, DiscoveryType};
use envoy_types::pb::envoy::config::core::v3 as core_v3;
use envoy_types::pb::envoy::config::endpoint::v3::{
    ClusterLoadAssignment, Endpoint, LbEndpoint, LocalityLbEndpoints, lb_endpoint::HostIdentifier,
};
use envoy_types::pb::envoy::config::listener::v3::{ApiListener, Listener};
use envoy_types::pb::envoy::config::route::v3::route::Action;
use envoy_types::pb::envoy::config::route::v3::route_action::ClusterSpecifier;
use envoy_types::pb::envoy::config::route::v3::route_match::PathSpecifier;
use envoy_types::pb::envoy::config::route::v3::{
    Route, RouteAction, RouteConfiguration, RouteMatch, VirtualHost,
};
use envoy_types::pb::envoy::extensions::filters::network::http_connection_manager::v3::{
    HttpConnectionManager, Rds, http_connection_manager::RouteSpecifier,
};
use envoy_types::pb::google::protobuf::Any;
use prost::Message;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// ADS type URLs for the different xDS resource types.
#[derive(Debug, Display, EnumIter, IntoStaticStr, EnumString, PartialEq, Eq, Hash, Clone)]
pub enum AdsTypeUrl {
    /// Listener Discovery Service (LDS) type URL.
    #[strum(serialize = "type.googleapis.com/envoy.config.listener.v3.Listener")]
    Lds,
    /// Route Discovery Service (RDS) type URL.
    #[strum(serialize = "type.googleapis.com/envoy.config.route.v3.RouteConfiguration")]
    Rds,
    /// Cluster Discovery Service (CDS) type URL.
    #[strum(serialize = "type.googleapis.com/envoy.config.cluster.v3.Cluster")]
    Cds,
    /// Endpoint Discovery Service (EDS) type URL.
    #[strum(serialize = "type.googleapis.com/envoy.config.endpoint.v3.ClusterLoadAssignment")]
    Eds,
    /// HTTP Connection Manager (HCM) type URL.
    #[strum(
        serialize = "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager"
    )]
    Hcm,
}

/// Wraps a `HttpConnectionManager` route specifier in an `ApiListener` listener.
fn build_listener(name: &str, route: RouteSpecifier) -> Listener {
    let hcm = HttpConnectionManager {
        route_specifier: Some(route),
        ..Default::default()
    };
    Listener {
        name: name.to_string(),
        api_listener: Some(ApiListener {
            api_listener: Some(Any {
                type_url: AdsTypeUrl::Hcm.to_string(),
                value: hcm.encode_to_vec(),
            }),
        }),
        ..Default::default()
    }
}

/// Builds a listener whose route configuration is embedded inline and sends all
/// traffic to `cluster`.
pub fn build_inline_listener(name: &str, cluster: &str) -> Listener {
    build_listener(
        name,
        RouteSpecifier::RouteConfig(build_route_config(&format!("{name}-route"), cluster)),
    )
}

/// Builds a listener that fetches its route configuration via RDS by name.
pub fn build_rds_listener(name: &str, rds_name: &str) -> Listener {
    build_listener(
        name,
        RouteSpecifier::Rds(Rds {
            route_config_name: rds_name.to_string(),
            ..Default::default()
        }),
    )
}

/// Builds a route configuration that sends all traffic to `cluster`.
pub fn build_route_config(name: &str, cluster: &str) -> RouteConfiguration {
    RouteConfiguration {
        name: name.to_string(),
        virtual_hosts: vec![VirtualHost {
            name: "default".to_string(),
            domains: vec!["*".to_string()],
            routes: vec![Route {
                r#match: Some(RouteMatch {
                    path_specifier: Some(PathSpecifier::Prefix("/".to_string())),
                    ..Default::default()
                }),
                action: Some(Action::Route(RouteAction {
                    cluster_specifier: Some(ClusterSpecifier::Cluster(cluster.to_string())),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Builds a route configuration referencing `count` generated clusters
/// (`cluster-0` .. `cluster-{count-1}`), one prefix route per cluster.
///
/// With `count = 1` this is the generated-name equivalent of
/// [`build_route_config`]. Larger counts mirror control planes that share one
/// route config across many services (e.g. Istio's per-port route configs).
pub fn build_route_config_with_cluster_count(name: &str, count: usize) -> RouteConfiguration {
    let clusters: Vec<String> = (0..count).map(|i| format!("cluster-{i}")).collect();
    build_route_config_with_clusters(name, &clusters)
}

/// Builds a route configuration with one prefix route per cluster
/// (`/<cluster>` -> that cluster), all in one virtual host.
///
/// Mirrors control planes that share one route config across many services
/// (e.g. Istio's per-port route configs).
pub fn build_route_config_with_clusters(name: &str, clusters: &[String]) -> RouteConfiguration {
    RouteConfiguration {
        name: name.to_string(),
        virtual_hosts: vec![VirtualHost {
            name: "default".to_string(),
            domains: vec!["*".to_string()],
            routes: clusters
                .iter()
                .map(|cluster| Route {
                    r#match: Some(RouteMatch {
                        path_specifier: Some(PathSpecifier::Prefix(format!("/{cluster}"))),
                        ..Default::default()
                    }),
                    action: Some(Action::Route(RouteAction {
                        cluster_specifier: Some(ClusterSpecifier::Cluster(cluster.to_string())),
                        ..Default::default()
                    })),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Builds an EDS-discovered cluster named `name`.
pub fn build_cluster(name: &str) -> Cluster {
    Cluster {
        name: name.to_string(),
        cluster_discovery_type: Some(ClusterDiscoveryType::Type(DiscoveryType::Eds as i32)),
        ..Default::default()
    }
}

/// Builds a `ClusterLoadAssignment` for `cluster` with the given endpoints, all
/// placed in a single locality.
pub fn build_cla(cluster: &str, endpoints: &[(String, u16)]) -> ClusterLoadAssignment {
    ClusterLoadAssignment {
        cluster_name: cluster.to_string(),
        endpoints: vec![LocalityLbEndpoints {
            lb_endpoints: endpoints
                .iter()
                .map(|(host, port)| lb_endpoint(host, *port))
                .collect(),
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Builds a single `LbEndpoint` at `host:port`.
fn lb_endpoint(host: &str, port: u16) -> LbEndpoint {
    LbEndpoint {
        host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
            address: Some(core_v3::Address {
                address: Some(core_v3::address::Address::SocketAddress(
                    core_v3::SocketAddress {
                        address: host.to_string(),
                        port_specifier: Some(core_v3::socket_address::PortSpecifier::PortValue(
                            u32::from(port),
                        )),
                        ..Default::default()
                    },
                )),
            }),
            ..Default::default()
        })),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route_clusters(rc: &RouteConfiguration) -> Vec<(String, String)> {
        rc.virtual_hosts[0]
            .routes
            .iter()
            .map(|r| {
                let prefix = match r.r#match.as_ref().and_then(|m| m.path_specifier.as_ref()) {
                    Some(PathSpecifier::Prefix(p)) => p.clone(),
                    other => panic!("expected prefix match, got {other:?}"),
                };
                let cluster = match r.action.as_ref() {
                    Some(Action::Route(a)) => match a.cluster_specifier.as_ref() {
                        Some(ClusterSpecifier::Cluster(c)) => c.clone(),
                        other => panic!("expected cluster specifier, got {other:?}"),
                    },
                    other => panic!("expected route action, got {other:?}"),
                };
                (prefix, cluster)
            })
            .collect()
    }

    #[test]
    fn route_config_with_clusters_builds_one_route_per_cluster() {
        let clusters = vec!["a".to_string(), "b".to_string()];
        let rc = build_route_config_with_clusters("rc", &clusters);
        assert_eq!(rc.name, "rc");
        assert_eq!(
            route_clusters(&rc),
            vec![
                ("/a".to_string(), "a".to_string()),
                ("/b".to_string(), "b".to_string()),
            ]
        );
    }

    #[test]
    fn route_config_with_cluster_count_generates_names() {
        let rc = build_route_config_with_cluster_count("rc", 3);
        let routes = route_clusters(&rc);
        assert_eq!(routes.len(), 3);
        assert_eq!(
            routes[0],
            ("/cluster-0".to_string(), "cluster-0".to_string())
        );
        assert_eq!(
            routes[2],
            ("/cluster-2".to_string(), "cluster-2".to_string())
        );
    }
}
