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

//! Validated ClusterLoadAssignment resource (EDS).

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use protobuf::Parse;
use xds_client::Error;
use xds_client::Resource;
use xds_client::resource::TypeUrl;

use crate::generated::envoy_config_core_v3_health_check_proto::HealthStatus as EnvoyHealthStatus;
use crate::generated::envoy_config_endpoint_v3_endpoint_components_proto::lb_endpoint::HostIdentifierOneof;
use crate::generated::envoy_config_endpoint_v3_endpoint_proto::ClusterLoadAssignment;
use crate::generated::envoy_type_v3_percent_proto::FractionalPercentView;
use crate::generated::envoy_type_v3_percent_proto::fractional_percent::DenominatorType;

/// Validated ClusterLoadAssignment (EDS resource).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointsResource {
    pub(crate) cluster_name: String,
    pub(crate) localities: Vec<LocalityLbEndpoints>,
    pub(crate) drop_overloads: Vec<DropOverload>,
}

/// An EDS drop overload category and rate (gRFC A27).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DropOverload {
    pub(crate) category: String,
    /// Number of requests to drop per million (`0..=1_000_000`).
    pub(crate) requests_per_million: u32,
}

/// Endpoints within a single locality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalityLbEndpoints {
    pub(crate) locality: Locality,
    pub(crate) endpoints: Vec<LbEndpoint>,
    pub(crate) load_balancing_weight: u32,
    pub(crate) priority: u32,
}

// TODO: consider unifying with `xds_client::message::Locality` (same
// shape, used for `Node.locality`) and tonic-xds's own copy of this type.
/// Locality information for a set of endpoints.
///
/// Kept local for now instead of reusing `xds_client::message::Locality`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Locality {
    pub(crate) region: String,
    pub(crate) zone: String,
    pub(crate) sub_zone: String,
}

/// Formats the locality as `{region="<region>", zone="<zone>",
/// sub_zone="<sub_zone>"}`: the name of the locality's child policy under
/// `xds_wrr_locality`, and its metric label value (gRFC A78).
impl fmt::Display for Locality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{region=\"{}\", zone=\"{}\", sub_zone=\"{}\"}}",
            self.region, self.zone, self.sub_zone
        )
    }
}

/// A single validated endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LbEndpoint {
    pub(crate) address: EndpointAddress,
    pub(crate) health_status: HealthStatus,
    pub(crate) load_balancing_weight: u32,
}

// TODO: reuse `grpc::client::name_resolution::Address` when wiring resolver.
/// A resolved `host:port` endpoint address extracted from a `SocketAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct EndpointAddress {
    pub(crate) host: String,
    pub(crate) port: u16,
}

/// Health status of an endpoint (gRFC A27).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HealthStatus {
    Unknown,
    Healthy,
    Unhealthy,
    Draining,
    Degraded,
    /// A status this build does not recognize, carrying the raw wire value.
    ///
    /// Treated as unusable, like every status other than `Unknown` and
    /// `Healthy`.
    Other(i32),
}

impl From<EnvoyHealthStatus> for HealthStatus {
    fn from(value: EnvoyHealthStatus) -> Self {
        match value {
            EnvoyHealthStatus::Unknown => Self::Unknown,
            EnvoyHealthStatus::Healthy => Self::Healthy,
            // Envoy's TIMEOUT is documented as "interpreted by Envoy as
            // UNHEALTHY".
            EnvoyHealthStatus::Unhealthy | EnvoyHealthStatus::Timeout => Self::Unhealthy,
            EnvoyHealthStatus::Draining => Self::Draining,
            EnvoyHealthStatus::Degraded => Self::Degraded,
            // `HealthStatus` is an open enum, so a newer control plane can send
            // a status this build does not know. Per gRFC A27 only HEALTHY and
            // UNKNOWN are usable, so an unrecognized status fails closed.
            other => Self::Other(i32::from(other)),
        }
    }
}

impl Resource for EndpointsResource {
    type Message = ClusterLoadAssignment;

    const TYPE_URL: TypeUrl =
        TypeUrl::new("type.googleapis.com/envoy.config.endpoint.v3.ClusterLoadAssignment");

    const ALL_RESOURCES_REQUIRED_IN_SOTW: bool = false;

    fn deserialize(bytes: bytes::Bytes) -> xds_client::Result<Self::Message> {
        ClusterLoadAssignment::parse(&bytes)
            .map_err(|e| Error::Validation(format!("failed to decode ClusterLoadAssignment: {e}")))
    }

    fn name(message: &Self::Message) -> &str {
        message.cluster_name().to_str().unwrap_or_default()
    }

    fn validate(message: Self::Message) -> xds_client::Result<Self> {
        let cluster_name = message
            .cluster_name()
            .to_str()
            .unwrap_or_default()
            .to_string();
        if cluster_name.is_empty() {
            return Err(Error::Validation(
                "ClusterLoadAssignment missing cluster_name".into(),
            ));
        }

        let mut localities = Vec::new();
        // Per gRFC A27, all of these must hold across the whole resource:
        // endpoint addresses are unique, a locality appears at most once per
        // priority, and locality weights at one priority fit in a u32.
        let mut seen_addresses = HashSet::new();
        let mut seen_localities = HashSet::new();
        let mut weight_sums: HashMap<u32, u64> = HashMap::new();

        for locality_endpoints in message.endpoints().iter() {
            let Some(l) = locality_endpoints.locality_opt() else {
                return Err(Error::Validation(
                    "ClusterLoadAssignment contains a locality without an ID".into(),
                ));
            };
            let locality = Locality {
                region: l.region().to_str().unwrap_or_default().to_string(),
                zone: l.zone().to_str().unwrap_or_default().to_string(),
                sub_zone: l.sub_zone().to_str().unwrap_or_default().to_string(),
            };

            // Per gRFC A27: skip localities with no usable weight. An unset
            // weight reads as 0 here, which is the same "skip" case.
            let load_balancing_weight = locality_endpoints.load_balancing_weight().value();
            if load_balancing_weight == 0 {
                continue;
            }

            let priority = locality_endpoints.priority();

            let sum = weight_sums.entry(priority).or_default();
            *sum += u64::from(load_balancing_weight);
            if *sum > u64::from(u32::MAX) {
                return Err(Error::Validation(format!(
                    "sum of locality weights at priority {priority} exceeds {}",
                    u32::MAX
                )));
            }

            if !seen_localities.insert((locality.clone(), priority)) {
                return Err(Error::Validation(format!(
                    "duplicate locality {locality:?} at priority {priority}"
                )));
            }

            let mut endpoints = Vec::new();
            let mut endpoint_weight_sum: u64 = 0;
            for lb_ep in locality_endpoints.lb_endpoints().iter() {
                let Some(ep) = validate_lb_endpoint(lb_ep)? else {
                    continue;
                };
                endpoint_weight_sum += u64::from(ep.load_balancing_weight);
                if endpoint_weight_sum > u64::from(u32::MAX) {
                    return Err(Error::Validation(format!(
                        "sum of endpoint weights in locality {locality:?} exceeds {}",
                        u32::MAX
                    )));
                }
                if !seen_addresses.insert(ep.address.clone()) {
                    return Err(Error::Validation(format!(
                        "duplicate endpoint address {}:{}",
                        ep.address.host, ep.address.port
                    )));
                }
                endpoints.push(ep);
            }

            localities.push(LocalityLbEndpoints {
                locality,
                endpoints,
                load_balancing_weight,
                priority,
            });
        }

        // Per gRFC A27: priorities must run 0..N with no gaps.
        let priorities: HashSet<u32> = localities.iter().map(|l| l.priority).collect();
        for priority in 0..priorities.len() as u32 {
            if !priorities.contains(&priority) {
                return Err(Error::Validation(format!(
                    "priority {priority} missing from ClusterLoadAssignment"
                )));
            }
        }

        let drop_overloads = validate_drop_overloads(&message)?;

        Ok(EndpointsResource {
            cluster_name,
            localities,
            drop_overloads,
        })
    }
}

/// Extracts `ClusterLoadAssignment.policy.drop_overloads` (gRFC A27),
/// normalizing each `drop_percentage` to requests per million. Empty or
/// non-UTF-8 categories, missing `drop_percentage` fields, and unknown
/// denominators are rejected, matching gRPC C-core.
fn validate_drop_overloads(
    message: &ClusterLoadAssignment,
) -> xds_client::Result<Vec<DropOverload>> {
    if !message.has_policy() {
        return Ok(Vec::new());
    }
    message
        .policy()
        .drop_overloads()
        .iter()
        .map(|d| -> xds_client::Result<DropOverload> {
            let category = d
                .category()
                .to_str()
                .map_err(|e| {
                    Error::Validation(format!("drop_overloads category is not valid UTF-8: {e}"))
                })?
                .to_string();
            if category.is_empty() {
                return Err(Error::Validation(
                    "drop_overloads category must be non-empty".to_string(),
                ));
            }
            let frac = d.drop_percentage_opt().ok_or_else(|| {
                Error::Validation(format!(
                    "drop_overloads category {category:?}: drop_percentage not present"
                ))
            })?;
            let requests_per_million = fractional_percent_to_million(frac).map_err(|e| {
                Error::Validation(format!("drop_overloads category {category:?}: {e}"))
            })?;
            Ok(DropOverload {
                category,
                requests_per_million,
            })
        })
        .collect()
}

fn fractional_percent_to_million(frac: FractionalPercentView<'_>) -> Result<u32, String> {
    let scale = match frac.denominator() {
        DenominatorType::Hundred => 10_000,
        DenominatorType::TenThousand => 100,
        DenominatorType::Million => 1,
        other => return Err(format!("unknown drop_percentage denominator: {other:?}")),
    };
    Ok(frac.numerator().saturating_mul(scale).min(1_000_000))
}

fn validate_lb_endpoint(
    lb_ep: crate::generated::envoy_config_endpoint_v3_endpoint_components_proto::LbEndpointView<'_>,
) -> xds_client::Result<Option<LbEndpoint>> {
    let health_status = HealthStatus::from(lb_ep.health_status());

    let endpoint = match lb_ep.host_identifier() {
        HostIdentifierOneof::Endpoint(ep) => ep,
        // Skip unsupported host_identifier variants (e.g. `endpoint_name`,
        // used for LRS-only named endpoints) rather than NACKing the whole
        // resource -- the control plane may be serving both Envoy proxies
        // and gRPC clients.
        _ => return Ok(None),
    };

    if !endpoint.has_address() {
        return Err(Error::Validation("endpoint missing address".into()));
    }
    let address = endpoint.address();
    if !address.has_socket_address() {
        return Err(Error::Validation(
            "only socket addresses are supported for gRPC endpoints".into(),
        ));
    }
    let socket_address = address.socket_address();

    if !socket_address.has_port_value() {
        return Err(Error::Validation(
            "endpoint address missing numeric port".into(),
        ));
    }

    let host = socket_address
        .address()
        .to_str()
        .unwrap_or_default()
        .to_string();
    // Per gRFC A27: the address field must be set.
    if host.is_empty() {
        return Err(Error::Validation("endpoint address is empty".into()));
    }
    let port = socket_address.port_value();
    let port = u16::try_from(port)
        .map_err(|_| Error::Validation(format!("endpoint port is out of range: {port}")))?;
    let address = EndpointAddress { host, port };

    // Per gRFC A27: if set, the weight must be at least 1. Unset means the
    // endpoint carries equal weight within its locality.
    let weight = match lb_ep.load_balancing_weight_opt() {
        Some(w) if w.value() == 0 => {
            return Err(Error::Validation(
                "endpoint has a zero load_balancing_weight".into(),
            ));
        }
        Some(w) => w.value(),
        None => 1,
    };

    Ok(Some(LbEndpoint {
        address,
        health_status,
        load_balancing_weight: weight,
    }))
}

impl EndpointsResource {
    /// Returns all healthy endpoints (`Unknown` and `Healthy` status), per
    /// gRFC A27's definition of usable endpoints.
    pub(crate) fn healthy_endpoints(&self) -> impl Iterator<Item = &LbEndpoint> {
        self.localities
            .iter()
            .flat_map(|l| &l.endpoints)
            .filter(|e| {
                matches!(
                    e.health_status,
                    HealthStatus::Unknown | HealthStatus::Healthy
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::envoy_config_core_v3_address_proto::Address;
    use crate::generated::envoy_config_core_v3_address_proto::SocketAddress;
    use crate::generated::envoy_config_core_v3_base_proto::Locality as EnvoyLocality;
    use crate::generated::envoy_config_endpoint_v3_endpoint_components_proto::Endpoint;
    use crate::generated::envoy_config_endpoint_v3_endpoint_components_proto::LbEndpoint as EnvoyLbEndpoint;
    use crate::generated::envoy_config_endpoint_v3_endpoint_components_proto::LocalityLbEndpoints as EnvoyLocalityLbEndpoints;
    use protobuf::Serialize;
    use protobuf_well_known_types::UInt32Value;

    fn make_weight(weight: u32) -> UInt32Value {
        let mut value = UInt32Value::new();
        value.set_value(weight);
        value
    }

    fn make_locality_endpoints(region: &str, priority: u32) -> EnvoyLocalityLbEndpoints {
        let mut locality = EnvoyLocality::new();
        locality.set_region(region);

        let mut locality_lb_endpoints = EnvoyLocalityLbEndpoints::new();
        locality_lb_endpoints.set_locality(locality);
        locality_lb_endpoints.set_load_balancing_weight(make_weight(1));
        locality_lb_endpoints.set_priority(priority);
        locality_lb_endpoints
    }

    fn make_lb_endpoint(host: &str, port: u32, health: EnvoyHealthStatus) -> EnvoyLbEndpoint {
        let mut socket_address = SocketAddress::new();
        socket_address.set_address(host);
        socket_address.set_port_value(port);
        let mut address = Address::new();
        address.set_socket_address(socket_address);
        let mut endpoint = Endpoint::new();
        endpoint.set_address(address);

        let mut lb_endpoint = EnvoyLbEndpoint::new();
        lb_endpoint.set_endpoint(endpoint);
        lb_endpoint.set_health_status(health);
        lb_endpoint
    }

    fn make_cla(cluster_name: &str) -> ClusterLoadAssignment {
        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.1",
                8080,
                EnvoyHealthStatus::Healthy,
            ));
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.2",
                8080,
                EnvoyHealthStatus::Unknown,
            ));
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.3",
                8080,
                EnvoyHealthStatus::Unhealthy,
            ));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name(cluster_name);
        cla.endpoints_mut().push(locality_lb_endpoints);
        cla
    }

    #[test]
    fn validate_basic() {
        let cla = make_cla("my-cluster");
        let validated = EndpointsResource::validate(cla).expect("should validate");
        assert_eq!(validated.cluster_name, "my-cluster");
        assert_eq!(validated.localities.len(), 1);
        assert_eq!(validated.localities[0].endpoints.len(), 3);
        assert_eq!(validated.localities[0].locality.region, "us-east-1");
    }

    #[test]
    fn validate_endpoint_addresses() {
        let cla = make_cla("my-cluster");
        let validated = EndpointsResource::validate(cla).unwrap();
        let addr = &validated.localities[0].endpoints[0].address;
        assert_eq!(addr.host, "10.0.0.1");
        assert_eq!(addr.port, 8080);
    }

    #[test]
    fn healthy_endpoints_excludes_unhealthy() {
        let cla = make_cla("my-cluster");
        let validated = EndpointsResource::validate(cla).unwrap();
        // Healthy + Unknown = 2 (Unhealthy excluded).
        assert_eq!(validated.healthy_endpoints().count(), 2);
    }

    #[test]
    fn healthy_endpoints_excludes_degraded_and_unrecognized() {
        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.1",
                8080,
                EnvoyHealthStatus::Degraded,
            ));
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.2",
                8080,
                EnvoyHealthStatus::from(99),
            ));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let validated = EndpointsResource::validate(cla).unwrap();
        assert_eq!(validated.localities[0].endpoints.len(), 2);
        assert_eq!(validated.healthy_endpoints().count(), 0);
        assert_eq!(
            validated.localities[0].endpoints[0].health_status,
            HealthStatus::Degraded
        );
    }

    #[test]
    fn unrecognized_health_status_survives_the_wire_and_is_not_usable() {
        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.1",
                8080,
                EnvoyHealthStatus::from(99),
            ));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let bytes = cla.serialize().expect("serialize");
        let decoded =
            EndpointsResource::deserialize(bytes::Bytes::from(bytes)).expect("deserialize");
        assert_eq!(
            i32::from(
                decoded
                    .endpoints()
                    .get(0)
                    .expect("locality")
                    .lb_endpoints()
                    .get(0)
                    .expect("endpoint")
                    .health_status()
            ),
            99,
            "protobuf runtime must preserve unrecognized enum values"
        );

        let validated = EndpointsResource::validate(decoded).unwrap();
        assert_eq!(
            validated.localities[0].endpoints[0].health_status,
            HealthStatus::Other(99),
            "an unrecognized status must keep its raw value for debugging"
        );
        assert_eq!(validated.healthy_endpoints().count(), 0);
    }

    #[test]
    fn validate_empty_cluster_name() {
        let cla = ClusterLoadAssignment::new();
        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("cluster_name"));
    }

    #[test]
    fn validate_skips_named_endpoint() {
        let mut lb_endpoint = EnvoyLbEndpoint::new();
        lb_endpoint.set_endpoint_name("named-endpoint");

        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints.lb_endpoints_mut().push(lb_endpoint);

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let validated = EndpointsResource::validate(cla).expect("should validate");
        assert!(validated.localities[0].endpoints.is_empty());
    }

    #[test]
    fn validate_rejects_missing_port() {
        let mut socket_address = SocketAddress::new();
        socket_address.set_address("10.0.0.1");
        let mut address = Address::new();
        address.set_socket_address(socket_address);
        let mut endpoint = Endpoint::new();
        endpoint.set_address(address);
        let mut lb_endpoint = EnvoyLbEndpoint::new();
        lb_endpoint.set_endpoint(endpoint);

        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints.lb_endpoints_mut().push(lb_endpoint);

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("missing numeric port"));
    }

    #[test]
    fn validate_rejects_out_of_range_port() {
        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.1",
                65_536,
                EnvoyHealthStatus::Healthy,
            ));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn validate_rejects_locality_without_id() {
        let mut locality_lb_endpoints = EnvoyLocalityLbEndpoints::new();
        locality_lb_endpoints.set_load_balancing_weight(make_weight(1));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("without an ID"));
    }

    #[test]
    fn validate_skips_locality_without_weight() {
        let mut locality = EnvoyLocality::new();
        locality.set_region("us-east-1");
        let mut locality_lb_endpoints = EnvoyLocalityLbEndpoints::new();
        locality_lb_endpoints.set_locality(locality);
        locality_lb_endpoints
            .lb_endpoints_mut()
            .push(make_lb_endpoint(
                "10.0.0.1",
                8080,
                EnvoyHealthStatus::Healthy,
            ));

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let validated = EndpointsResource::validate(cla).expect("should validate");
        assert!(validated.localities.is_empty());
    }

    #[test]
    fn validate_rejects_duplicate_locality_at_same_priority() {
        let mut cla = make_cla("my-cluster");
        let mut duplicate = make_locality_endpoints("us-east-1", 0);
        duplicate.lb_endpoints_mut().push(make_lb_endpoint(
            "10.0.0.4",
            8080,
            EnvoyHealthStatus::Healthy,
        ));
        cla.endpoints_mut().push(duplicate);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("duplicate locality"));
    }

    #[test]
    fn validate_rejects_duplicate_endpoint_address() {
        let mut cla = make_cla("my-cluster");
        let mut other = make_locality_endpoints("us-west-1", 0);
        other.lb_endpoints_mut().push(make_lb_endpoint(
            "10.0.0.1",
            8080,
            EnvoyHealthStatus::Healthy,
        ));
        cla.endpoints_mut().push(other);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("duplicate endpoint address"));
    }

    #[test]
    fn validate_rejects_priority_gap() {
        let mut cla = make_cla("my-cluster");
        let mut gapped = make_locality_endpoints("us-west-1", 2);
        gapped.lb_endpoints_mut().push(make_lb_endpoint(
            "10.0.0.4",
            8080,
            EnvoyHealthStatus::Healthy,
        ));
        cla.endpoints_mut().push(gapped);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("priority 1 missing"));
    }

    #[test]
    fn validate_rejects_zero_endpoint_weight() {
        let mut lb_endpoint = make_lb_endpoint("10.0.0.1", 8080, EnvoyHealthStatus::Healthy);
        lb_endpoint.set_load_balancing_weight(make_weight(0));

        let mut locality_lb_endpoints = make_locality_endpoints("us-east-1", 0);
        locality_lb_endpoints.lb_endpoints_mut().push(lb_endpoint);

        let mut cla = ClusterLoadAssignment::new();
        cla.set_cluster_name("my-cluster");
        cla.endpoints_mut().push(locality_lb_endpoints);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(err.to_string().contains("zero load_balancing_weight"));
    }

    #[test]
    fn deserialize_roundtrip() {
        let cla = make_cla("test");
        let bytes = cla.serialize().expect("serialize");
        let deserialized = EndpointsResource::deserialize(bytes::Bytes::from(bytes)).unwrap();
        assert_eq!(EndpointsResource::name(&deserialized), "test");
    }

    #[test]
    fn validate_drop_overloads() {
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::Policy;
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::policy::DropOverload as EnvoyDropOverload;
        use crate::generated::envoy_type_v3_percent_proto::FractionalPercent;

        let make_drop = |category: &str, num: u32, denom: DenominatorType| {
            let mut frac = FractionalPercent::new();
            frac.set_numerator(num);
            frac.set_denominator(denom);
            let mut drop_overload = EnvoyDropOverload::new();
            drop_overload.set_category(category);
            drop_overload.set_drop_percentage(frac);
            drop_overload
        };

        let mut policy = Policy::new();
        policy
            .drop_overloads_mut()
            .push(make_drop("hundred", 25, DenominatorType::Hundred));
        policy.drop_overloads_mut().push(make_drop(
            "ten_thousand",
            500,
            DenominatorType::TenThousand,
        ));
        policy
            .drop_overloads_mut()
            .push(make_drop("million", 123_456, DenominatorType::Million));
        policy
            .drop_overloads_mut()
            .push(make_drop("capped", 150, DenominatorType::Hundred));

        let mut cla = make_cla("my-cluster");
        cla.set_policy(policy);

        let validated = EndpointsResource::validate(cla).unwrap();
        assert_eq!(
            validated.drop_overloads,
            vec![
                DropOverload {
                    category: "hundred".to_string(),
                    requests_per_million: 250_000,
                },
                DropOverload {
                    category: "ten_thousand".to_string(),
                    requests_per_million: 50_000,
                },
                DropOverload {
                    category: "million".to_string(),
                    requests_per_million: 123_456,
                },
                DropOverload {
                    category: "capped".to_string(),
                    requests_per_million: 1_000_000,
                },
            ]
        );
    }

    #[test]
    fn validate_rejects_missing_drop_percentage() {
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::Policy;
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::policy::DropOverload as EnvoyDropOverload;

        let mut drop_overload = EnvoyDropOverload::new();
        drop_overload.set_category("unset_pct");
        let mut policy = Policy::new();
        policy.drop_overloads_mut().push(drop_overload);

        let mut cla = make_cla("my-cluster");
        cla.set_policy(policy);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(
            err.to_string().contains("drop_percentage not present"),
            "got: {err}"
        );
    }

    #[test]
    fn validate_rejects_empty_drop_category() {
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::Policy;
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::policy::DropOverload as EnvoyDropOverload;
        use crate::generated::envoy_type_v3_percent_proto::FractionalPercent;

        let mut frac = FractionalPercent::new();
        frac.set_numerator(10);
        frac.set_denominator(DenominatorType::Hundred);
        let mut drop_overload = EnvoyDropOverload::new();
        drop_overload.set_drop_percentage(frac);
        let mut policy = Policy::new();
        policy.drop_overloads_mut().push(drop_overload);

        let mut cla = make_cla("my-cluster");
        cla.set_policy(policy);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(
            err.to_string().contains("category must be non-empty"),
            "got: {err}"
        );
    }

    #[test]
    fn validate_rejects_unknown_drop_denominator() {
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::Policy;
        use crate::generated::envoy_config_endpoint_v3_endpoint_proto::cluster_load_assignment::policy::DropOverload as EnvoyDropOverload;
        use crate::generated::envoy_type_v3_percent_proto::FractionalPercent;

        let mut frac = FractionalPercent::new();
        frac.set_numerator(10);
        frac.set_denominator(DenominatorType::from(99));
        let mut drop_overload = EnvoyDropOverload::new();
        drop_overload.set_category("bad_denominator");
        drop_overload.set_drop_percentage(frac);
        let mut policy = Policy::new();
        policy.drop_overloads_mut().push(drop_overload);

        let mut cla = make_cla("my-cluster");
        cla.set_policy(policy);

        let err = EndpointsResource::validate(cla).unwrap_err();
        assert!(
            err.to_string()
                .contains("unknown drop_percentage denominator"),
            "got: {err}"
        );
    }
}
