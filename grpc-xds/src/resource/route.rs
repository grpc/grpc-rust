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

//! Validated RouteConfiguration resource (RDS).
//!
//! Models the validated data shapes only (gRFC A28).

// TODO: implement request routing components (virtual-host domain matching, then
// path/header/fraction matching, stripping `-bin` headers from request metadata before
// evaluating header matchers) in the future resolver/interceptor layer that consumes `XdsConfig`.

use std::collections::HashSet;

use protobuf::Parse;
use regex::Regex;
use xds_client::resource::TypeUrl;
use xds_client::{Error, Resource};

use crate::generated::envoy::config::route::v3::header_matcher::HeaderMatchSpecifierOneof;
use crate::generated::envoy::config::route::v3::route::ActionOneof;
use crate::generated::envoy::config::route::v3::route_action::ClusterSpecifierOneof;
use crate::generated::envoy::config::route::v3::route_match::PathSpecifierOneof;
use crate::generated::envoy::config::route::v3::{
    HeaderMatcherView, RouteActionView, RouteConfiguration, RouteMatchView, RouteView,
    VirtualHostView,
};
use crate::generated::envoy::r#type::matcher::v3::StringMatcherView;
use crate::generated::envoy::r#type::matcher::v3::string_matcher::MatchPatternOneof;
use crate::generated::envoy::r#type::v3::fractional_percent::DenominatorType;

/// Validated RouteConfiguration.
#[derive(Debug, Clone)]
pub(crate) struct RouteConfigResource {
    pub(crate) name: String,
    pub(crate) virtual_hosts: Vec<VirtualHost>,
}

/// Validated virtual host with domain matching and routes.
#[derive(Debug, Clone)]
pub(crate) struct VirtualHost {
    pub(crate) name: String,
    pub(crate) domains: Vec<String>,
    pub(crate) routes: Vec<Route>,
}

/// A validated route with match criteria and action.
#[derive(Debug, Clone)]
pub(crate) struct Route {
    pub(crate) route_match: RouteMatch,
    pub(crate) action: RouteAction,
}

/// Validated route match criteria.
#[derive(Debug, Clone)]
pub(crate) struct RouteMatch {
    pub(crate) path_specifier: PathSpecifier,
    pub(crate) headers: Vec<HeaderMatcher>,
    pub(crate) case_sensitive: bool,
    /// Fraction of requests this route should match, as numerator out of
    /// 1,000,000. `None` means always match (100%).
    pub(crate) match_fraction: Option<u32>,
}

/// Path matching specifier.
#[derive(Debug, Clone)]
pub(crate) enum PathSpecifier {
    Prefix(String),
    Path(String),
    SafeRegex(Regex),
}

/// Header matching criteria.
#[derive(Debug, Clone)]
pub(crate) struct HeaderMatcher {
    pub(crate) name: String,
    pub(crate) match_specifier: HeaderMatchSpecifier,
    pub(crate) invert_match: bool,
}

/// Header match specifier variants.
///
/// The `String` variant carries a generic [`StringMatcher`] (exact / prefix /
/// suffix / contains / safe_regex, with optional ASCII case-insensitive
/// matching per gRFC A63). `Present`, `Absent`, and `Range` are header-specific
/// extensions beyond the generic StringMatcher.
#[derive(Debug, Clone)]
pub(crate) enum HeaderMatchSpecifier {
    String(StringMatcher),
    /// Match if header is present (any value).
    Present,
    /// Match if header is absent.
    Absent,
    /// Match if the header value, parsed as an integer, falls within `[start, end)`.
    Range {
        start: i64,
        end: i64,
    },
}

/// Route action deciding where to send traffic.
#[derive(Debug, Clone)]
pub(crate) enum RouteAction {
    Cluster(String),
    WeightedClusters(Vec<WeightedCluster>),
}

/// A cluster with an associated weight for traffic splitting.
#[derive(Debug, Clone)]
pub(crate) struct WeightedCluster {
    pub(crate) name: String,
    pub(crate) weight: u32,
}

/// Validated `envoy.type.matcher.v3.StringMatcher`.
#[derive(Debug, Clone)]
pub(crate) enum StringMatcher {
    Exact { value: String, ignore_case: bool },
    Prefix { value: String, ignore_case: bool },
    Suffix { value: String, ignore_case: bool },
    Contains { value: String, ignore_case: bool },
    SafeRegex(Regex),
}

impl StringMatcher {
    /// Parses and validates an `envoy.type.matcher.v3.StringMatcher`.
    ///
    /// Returns an error if the `match_pattern` oneof is unset or carries an
    /// unsupported variant, a prefix/suffix/contains value is empty, or a
    /// `safe_regex` fails to compile.
    fn from_proto(proto: StringMatcherView<'_>) -> xds_client::Result<Self> {
        let ignore_case = proto.ignore_case();
        match proto.match_pattern() {
            MatchPatternOneof::Exact(value) => Ok(Self::Exact {
                value: value.to_str().unwrap_or_default().to_string(),
                ignore_case,
            }),
            MatchPatternOneof::Prefix(value) => Ok(Self::Prefix {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "prefix")?,
                ignore_case,
            }),
            MatchPatternOneof::Suffix(value) => Ok(Self::Suffix {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "suffix")?,
                ignore_case,
            }),
            MatchPatternOneof::Contains(value) => Ok(Self::Contains {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "contains")?,
                ignore_case,
            }),
            MatchPatternOneof::SafeRegex(r) => {
                let pattern = r.regex();
                let pattern = pattern.to_str().unwrap_or_default();
                Ok(Self::SafeRegex(compile_regex(pattern, "string matcher")?))
            }
            MatchPatternOneof::not_set(_) => Err(Error::Validation(
                "StringMatcher has no match_pattern set".into(),
            )),
            _ => Err(Error::Validation(
                "unsupported StringMatcher pattern".into(),
            )),
        }
    }
}

fn non_empty_match_value(value: &str, kind: &str) -> xds_client::Result<String> {
    if value.is_empty() {
        return Err(Error::Validation(format!(
            "empty {kind} match is not allowed"
        )));
    }
    Ok(value.to_string())
}

/// Compiles a `RegexMatcher` pattern, rejecting the empty pattern.
fn compile_regex(pattern: &str, kind: &str) -> xds_client::Result<Regex> {
    if pattern.is_empty() {
        return Err(Error::Validation(format!(
            "empty {kind} regex is not allowed"
        )));
    }
    Regex::new(pattern)
        .map_err(|e| Error::Validation(format!("invalid {kind} regex '{pattern}': {e}")))
}

impl Resource for RouteConfigResource {
    type Message = RouteConfiguration;

    const TYPE_URL: TypeUrl =
        TypeUrl::new("type.googleapis.com/envoy.config.route.v3.RouteConfiguration");

    const ALL_RESOURCES_REQUIRED_IN_SOTW: bool = false;

    fn deserialize(bytes: bytes::Bytes) -> xds_client::Result<Self::Message> {
        RouteConfiguration::parse(&bytes)
            .map_err(|e| Error::Validation(format!("failed to decode RouteConfiguration: {e}")))
    }

    fn name(message: &Self::Message) -> &str {
        message.name().to_str().unwrap_or_default()
    }

    fn validate(message: Self::Message) -> xds_client::Result<Self> {
        let name = message.name().to_str().unwrap_or_default().to_string();

        let virtual_hosts_view = message.virtual_hosts();
        let mut virtual_hosts = Vec::new();
        for vh in virtual_hosts_view.iter() {
            virtual_hosts.push(validate_virtual_host(vh)?);
        }

        Ok(RouteConfigResource {
            name,
            virtual_hosts,
        })
    }
}

fn validate_virtual_host(vh: VirtualHostView<'_>) -> xds_client::Result<VirtualHost> {
    let name = vh.name().to_str().unwrap_or_default().to_string();

    let domains_view = vh.domains();
    if domains_view.is_empty() {
        return Err(Error::Validation(format!(
            "virtual host '{name}' has no domains"
        )));
    }
    let domains: Vec<String> = domains_view
        .iter()
        .map(|d| d.to_str().unwrap_or_default().to_string())
        .collect();

    let mut routes = Vec::new();
    for route in vh.routes().iter() {
        if let Some(validated) = validate_route(route)? {
            routes.push(validated);
        }
    }

    Ok(VirtualHost {
        name,
        domains,
        routes,
    })
}

/// Returns `Ok(None)` for routes that should be silently skipped (query
/// param matchers, unsupported cluster specifiers like `cluster_header`),
/// per gRFC A28.
fn validate_route(route: RouteView<'_>) -> xds_client::Result<Option<Route>> {
    if !route.has_match() {
        return Err(Error::Validation("route missing match field".into()));
    }
    let route_match = route.r#match();

    // Per A28: ignore routes with query parameter matchers.
    if !route_match.query_parameters().is_empty() {
        return Ok(None);
    }

    let match_criteria = validate_route_match(route_match)?;

    let action = match route.action() {
        ActionOneof::Route(route_action) => match validate_route_action(route_action)? {
            Some(action) => action,
            None => return Ok(None),
        },
        // Per A28: action field must be "route", otherwise NACK.
        _ => {
            return Err(Error::Validation(
                "only route action is supported for client routing".into(),
            ));
        }
    };

    Ok(Some(Route {
        route_match: match_criteria,
        action,
    }))
}

fn validate_route_match(rm: RouteMatchView<'_>) -> xds_client::Result<RouteMatch> {
    let path_specifier = match rm.path_specifier() {
        PathSpecifierOneof::Prefix(p) => {
            PathSpecifier::Prefix(p.to_str().unwrap_or_default().to_string())
        }
        PathSpecifierOneof::Path(p) => {
            PathSpecifier::Path(p.to_str().unwrap_or_default().to_string())
        }
        PathSpecifierOneof::SafeRegex(r) => {
            let pattern = r.regex();
            let pattern = pattern.to_str().unwrap_or_default();
            PathSpecifier::SafeRegex(compile_regex(pattern, "path")?)
        }
        // Per A28: not having path_specifier will cause a NACK.
        PathSpecifierOneof::not_set(_) => {
            return Err(Error::Validation(
                "route match missing path_specifier".into(),
            ));
        }
        _ => {
            return Err(Error::Validation(
                "unsupported path specifier variant".into(),
            ));
        }
    };

    let case_sensitive = if rm.has_case_sensitive() {
        rm.case_sensitive().value()
    } else {
        true
    };

    // Per A28, a matcher naming a `-bin` header must behave as if that header
    // were absent, so the matcher is kept here and neutralized at match time.
    // Dropping it instead would widen the route to traffic it must not match.
    let headers = rm
        .headers()
        .iter()
        .map(validate_header_matcher)
        .collect::<xds_client::Result<Vec<_>>>()?;

    // Per A28: use runtime_fraction.default_value, normalize to numerator out
    // of 1,000,000. runtime_key is ignored (gRPC has no runtime config).
    let match_fraction = match rm.runtime_fraction_opt() {
        None => None,
        Some(rf) => {
            if !rf.has_default_value() {
                return Err(Error::Validation(
                    "runtime_fraction is missing its required default_value".into(),
                ));
            }
            let frac = rf.default_value();
            let scale = match frac.denominator() {
                DenominatorType::Hundred => 10_000,
                DenominatorType::TenThousand => 100,
                DenominatorType::Million => 1,
                _ => 1,
            };
            Some(frac.numerator().saturating_mul(scale).min(1_000_000))
        }
    };

    Ok(RouteMatch {
        path_specifier,
        headers,
        case_sensitive,
        match_fraction,
    })
}

fn validate_header_matcher(hm: HeaderMatcherView<'_>) -> xds_client::Result<HeaderMatcher> {
    let name = hm.name().to_str().unwrap_or_default().to_string();
    if name.is_empty() {
        return Err(Error::Validation("header matcher name is empty".into()));
    }

    // Legacy matchers are deprecated in favor of StringMatch but remain
    // widely used and are still required by gRFC A63.
    #[allow(deprecated, unreachable_patterns)]
    let match_specifier = match hm.header_match_specifier() {
        HeaderMatchSpecifierOneof::ExactMatch(v) => {
            HeaderMatchSpecifier::String(StringMatcher::Exact {
                value: v.to_str().unwrap_or_default().to_string(),
                ignore_case: false,
            })
        }
        HeaderMatchSpecifierOneof::SafeRegexMatch(r) => {
            let pattern = r.regex();
            let pattern = pattern.to_str().unwrap_or_default();
            HeaderMatchSpecifier::String(StringMatcher::SafeRegex(compile_regex(
                pattern, "header",
            )?))
        }
        HeaderMatchSpecifierOneof::RangeMatch(r) => HeaderMatchSpecifier::Range {
            start: r.start(),
            end: r.end(),
        },
        HeaderMatchSpecifierOneof::PresentMatch(present) => {
            if present {
                HeaderMatchSpecifier::Present
            } else {
                HeaderMatchSpecifier::Absent
            }
        }
        HeaderMatchSpecifierOneof::PrefixMatch(v) => {
            HeaderMatchSpecifier::String(StringMatcher::Prefix {
                value: non_empty_match_value(v.to_str().unwrap_or_default(), "prefix")?,
                ignore_case: false,
            })
        }
        HeaderMatchSpecifierOneof::SuffixMatch(v) => {
            HeaderMatchSpecifier::String(StringMatcher::Suffix {
                value: non_empty_match_value(v.to_str().unwrap_or_default(), "suffix")?,
                ignore_case: false,
            })
        }
        HeaderMatchSpecifierOneof::ContainsMatch(v) => {
            HeaderMatchSpecifier::String(StringMatcher::Contains {
                value: non_empty_match_value(v.to_str().unwrap_or_default(), "contains")?,
                ignore_case: false,
            })
        }
        HeaderMatchSpecifierOneof::StringMatch(sm) => {
            HeaderMatchSpecifier::String(StringMatcher::from_proto(sm)?)
        }
        HeaderMatchSpecifierOneof::not_set(_) => HeaderMatchSpecifier::Present,
        _ => {
            return Err(Error::Validation(
                "unsupported header match specifier".into(),
            ));
        }
    };

    Ok(HeaderMatcher {
        name,
        match_specifier,
        invert_match: hm.invert_match(),
    })
}

/// Returns `Ok(None)` for routes whose cluster specifier is unsupported
/// (e.g. `cluster_header`) or unset, both of which A28 requires be skipped.
fn validate_route_action(ra: RouteActionView<'_>) -> xds_client::Result<Option<RouteAction>> {
    match ra.cluster_specifier() {
        ClusterSpecifierOneof::Cluster(name) => {
            let name = name.to_str().unwrap_or_default();
            if name.is_empty() {
                return Err(Error::Validation("cluster name is empty".into()));
            }
            Ok(Some(RouteAction::Cluster(name.to_string())))
        }
        ClusterSpecifierOneof::WeightedClusters(wc) => {
            let clusters_view = wc.clusters();
            if clusters_view.is_empty() {
                return Err(Error::Validation("weighted_clusters is empty".into()));
            }
            let mut clusters = Vec::new();
            let mut total_weight: u64 = 0;
            for c in clusters_view.iter() {
                // Per A28: zero-weight entries never receive traffic, so drop them.
                let weight = c.weight_opt().map(|w| w.value()).unwrap_or(0);
                if weight == 0 {
                    continue;
                }
                let name = c.name().to_str().unwrap_or_default();
                if name.is_empty() {
                    return Err(Error::Validation("weighted cluster name is empty".into()));
                }
                total_weight += u64::from(weight);
                if total_weight > u64::from(u32::MAX) {
                    return Err(Error::Validation(format!(
                        "sum of weighted cluster weights exceeds {}",
                        u32::MAX
                    )));
                }
                clusters.push(WeightedCluster {
                    name: name.to_string(),
                    weight,
                });
            }
            // Per A28: the weights must add up to a non-zero total, otherwise
            // there is nothing for a picker to distribute traffic across.
            if clusters.is_empty() {
                return Err(Error::Validation(
                    "weighted_clusters has no cluster with a non-zero weight".into(),
                ));
            }
            Ok(Some(RouteAction::WeightedClusters(clusters)))
        }
        // Per A28: ignore the route when the cluster specifier is unsupported
        // (e.g. `cluster_header`) or unset. A specifier field added to the oneof
        // by a newer control plane decodes as an unknown field, which protobuf
        // reports as `not_set`, so that case must be ignored rather than NACKed.
        _ => Ok(None),
    }
}

impl VirtualHost {
    /// Returns cluster names referenced by this virtual host, for cascading
    /// CDS subscriptions after the dependency manager selects the host that
    /// matches the channel authority.
    #[allow(dead_code)] // TODO: remove once dependency manager calls this.
    pub(crate) fn cluster_names(&self) -> HashSet<String> {
        let mut clusters = HashSet::new();
        for route in &self.routes {
            match &route.action {
                RouteAction::Cluster(name) => {
                    clusters.insert(name.clone());
                }
                RouteAction::WeightedClusters(wcs) => {
                    for wc in wcs {
                        clusters.insert(wc.name.clone());
                    }
                }
            }
        }
        clusters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::envoy::config::core::v3::RuntimeFractionalPercent;
    use crate::generated::envoy::config::route::v3::weighted_cluster::ClusterWeight;
    use crate::generated::envoy::config::route::v3::{
        HeaderMatcher as EnvoyHeaderMatcher, QueryParameterMatcher, RedirectAction,
        Route as EnvoyRoute, RouteAction as EnvoyRouteAction, RouteMatch as EnvoyRouteMatch,
        VirtualHost as EnvoyVirtualHost, WeightedCluster as EnvoyWeightedCluster,
    };
    use crate::generated::envoy::r#type::matcher::v3::RegexMatcher;
    use crate::generated::envoy::r#type::matcher::v3::StringMatcher as EnvoyStringMatcher;
    use crate::generated::envoy::r#type::v3::FractionalPercent;
    use protobuf::Serialize;
    use protobuf_well_known_types::UInt32Value;

    fn make_route(prefix: &str, cluster: &str) -> EnvoyRoute {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix(prefix);

        let mut route_action = EnvoyRouteAction::new();
        route_action.set_cluster(cluster);

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);
        route
    }

    fn make_route_config(name: &str) -> RouteConfiguration {
        wrap_route(name, make_route("/", "cluster-1"))
    }

    fn wrap_route(name: &str, route: EnvoyRoute) -> RouteConfiguration {
        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.domains_mut().push("*");
        vh.routes_mut().push(route);

        let mut rc = RouteConfiguration::new();
        rc.set_name(name);
        rc.virtual_hosts_mut().push(vh);
        rc
    }

    fn route_config_with_header(header: EnvoyHeaderMatcher) -> RouteConfiguration {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.headers_mut().push(header);

        let mut route_action = EnvoyRouteAction::new();
        route_action.set_cluster("cluster-1");

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);

        wrap_route("rc-1", route)
    }

    fn validate_header(header: EnvoyHeaderMatcher) -> HeaderMatchSpecifier {
        let validated = RouteConfigResource::validate(route_config_with_header(header))
            .expect("should validate");
        validated.virtual_hosts[0].routes[0].route_match.headers[0]
            .match_specifier
            .clone()
    }

    fn make_weighted_route(weights: &[(&str, Option<u32>)]) -> EnvoyRoute {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");

        let mut wc = EnvoyWeightedCluster::new();
        for (name, weight) in weights {
            let mut cw = ClusterWeight::new();
            cw.set_name(*name);
            if let Some(weight) = weight {
                let mut value = UInt32Value::new();
                value.set_value(*weight);
                cw.set_weight(value);
            }
            wc.clusters_mut().push(cw);
        }

        let mut route_action = EnvoyRouteAction::new();
        route_action.set_weighted_clusters(wc);

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);
        route
    }

    #[test]
    fn validate_basic() {
        let rc = make_route_config("rc-1");
        let validated = RouteConfigResource::validate(rc).expect("should validate");
        assert_eq!(validated.name, "rc-1");
        assert_eq!(validated.virtual_hosts.len(), 1);
        assert_eq!(validated.virtual_hosts[0].routes.len(), 1);
    }

    #[test]
    fn validate_empty_virtual_hosts() {
        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        let validated = RouteConfigResource::validate(rc).expect("should validate");
        assert!(validated.virtual_hosts.is_empty());
    }

    #[test]
    fn validate_virtual_host_no_domains() {
        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.routes_mut().push(make_route("/", "cluster-1"));

        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        rc.virtual_hosts_mut().push(vh);

        let err = RouteConfigResource::validate(rc).unwrap_err();
        assert!(err.to_string().contains("no domains"));
    }

    #[test]
    fn validate_route_missing_match() {
        let mut route = EnvoyRoute::new();
        route.set_route(EnvoyRouteAction::new());

        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.domains_mut().push("*");
        vh.routes_mut().push(route);

        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        rc.virtual_hosts_mut().push(vh);

        let err = RouteConfigResource::validate(rc).unwrap_err();
        assert!(err.to_string().contains("missing match field"));
    }

    #[test]
    fn validate_route_skips_query_parameter_matchers() {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        let mut qp = QueryParameterMatcher::new();
        qp.set_name("q");
        route_match.query_parameters_mut().push(qp);

        let mut route_action = EnvoyRouteAction::new();
        route_action.set_cluster("cluster-1");

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);

        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.domains_mut().push("*");
        vh.routes_mut().push(route);

        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        rc.virtual_hosts_mut().push(vh);

        let validated = RouteConfigResource::validate(rc).expect("should validate");
        assert!(validated.virtual_hosts[0].routes.is_empty());
    }

    #[test]
    fn validate_route_rejects_non_route_action() {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_redirect(RedirectAction::new());

        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.domains_mut().push("*");
        vh.routes_mut().push(route);

        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        rc.virtual_hosts_mut().push(vh);

        let err = RouteConfigResource::validate(rc).unwrap_err();
        assert!(err.to_string().contains("only route action is supported"));
    }

    #[test]
    fn validate_weighted_clusters() {
        let rc = wrap_route(
            "rc-1",
            make_weighted_route(&[("cluster-a", Some(80)), ("cluster-b", Some(20))]),
        );

        let validated = RouteConfigResource::validate(rc).expect("should validate");
        match &validated.virtual_hosts[0].routes[0].action {
            RouteAction::WeightedClusters(clusters) => {
                assert_eq!(clusters.len(), 2);
                assert_eq!(clusters[0].name, "cluster-a");
                assert_eq!(clusters[0].weight, 80);
                assert_eq!(clusters[1].name, "cluster-b");
                assert_eq!(clusters[1].weight, 20);
            }
            other => panic!("expected WeightedClusters, got {other:?}"),
        }
    }

    #[test]
    fn validate_weighted_clusters_drops_zero_weight_entries() {
        let rc = wrap_route(
            "rc-1",
            make_weighted_route(&[
                ("cluster-a", Some(80)),
                ("cluster-zero", Some(0)),
                ("cluster-unset", None),
            ]),
        );

        let validated = RouteConfigResource::validate(rc).expect("should validate");
        match &validated.virtual_hosts[0].routes[0].action {
            RouteAction::WeightedClusters(clusters) => {
                assert_eq!(clusters.len(), 1);
                assert_eq!(clusters[0].name, "cluster-a");
            }
            other => panic!("expected WeightedClusters, got {other:?}"),
        }
        // Zero-weight clusters never receive traffic, so they must not be subscribed to.
        assert_eq!(validated.virtual_hosts[0].cluster_names().len(), 1);
    }

    #[test]
    fn validate_weighted_clusters_rejects_zero_total_weight() {
        let rc = wrap_route(
            "rc-1",
            make_weighted_route(&[("cluster-a", Some(0)), ("cluster-b", None)]),
        );
        let err = RouteConfigResource::validate(rc).unwrap_err();
        assert!(err.to_string().contains("non-zero weight"));
    }

    #[test]
    fn validate_weighted_clusters_rejects_overflowing_total_weight() {
        let rc = wrap_route(
            "rc-1",
            make_weighted_route(&[("cluster-a", Some(u32::MAX)), ("cluster-b", Some(1))]),
        );
        let err = RouteConfigResource::validate(rc).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn validate_skips_route_with_unset_cluster_specifier() {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(EnvoyRouteAction::new());

        // An unset specifier -- which is also how an unknown specifier from a
        // newer control plane decodes -- must skip the route, not NACK.
        let validated =
            RouteConfigResource::validate(wrap_route("rc-1", route)).expect("should validate");
        assert!(validated.virtual_hosts[0].routes.is_empty());
    }

    #[test]
    fn validate_keeps_bin_header_matchers() {
        let mut header = EnvoyHeaderMatcher::new();
        header.set_name("x-authz-bin");
        header.set_present_match(true);

        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.headers_mut().push(header);

        let mut route_action = EnvoyRouteAction::new();
        route_action.set_cluster("cluster-1");

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);

        // Dropping the matcher would widen the route to every request; per A28 it
        // must be kept and evaluated as though the header were absent.
        let validated =
            RouteConfigResource::validate(wrap_route("rc-1", route)).expect("should validate");
        assert_eq!(
            validated.virtual_hosts[0].routes[0]
                .route_match
                .headers
                .len(),
            1
        );
    }

    #[test]
    fn validate_legacy_prefix_header_matcher() {
        let mut header = EnvoyHeaderMatcher::new();
        header.set_name("x-tenant");
        header.set_prefix_match("prod-");

        assert!(matches!(
            validate_header(header),
            HeaderMatchSpecifier::String(StringMatcher::Prefix {
                value,
                ignore_case: false,
            }) if value == "prod-"
        ));
    }

    #[test]
    fn validate_legacy_suffix_header_matcher() {
        let mut header = EnvoyHeaderMatcher::new();
        header.set_name("x-tenant");
        header.set_suffix_match("-canary");

        assert!(matches!(
            validate_header(header),
            HeaderMatchSpecifier::String(StringMatcher::Suffix {
                value,
                ignore_case: false,
            }) if value == "-canary"
        ));
    }

    #[test]
    fn validate_legacy_contains_header_matcher() {
        let mut header = EnvoyHeaderMatcher::new();
        header.set_name("x-tenant");
        header.set_contains_match("staging");

        assert!(matches!(
            validate_header(header),
            HeaderMatchSpecifier::String(StringMatcher::Contains {
                value,
                ignore_case: false,
            }) if value == "staging"
        ));
    }

    #[test]
    fn validate_legacy_string_header_matchers_reject_empty_values() {
        let mut prefix = EnvoyHeaderMatcher::new();
        prefix.set_name("x-tenant");
        prefix.set_prefix_match("");

        let mut suffix = EnvoyHeaderMatcher::new();
        suffix.set_name("x-tenant");
        suffix.set_suffix_match("");

        let mut contains = EnvoyHeaderMatcher::new();
        contains.set_name("x-tenant");
        contains.set_contains_match("");

        for (kind, header) in [
            ("prefix", prefix),
            ("suffix", suffix),
            ("contains", contains),
        ] {
            let err = RouteConfigResource::validate(route_config_with_header(header)).unwrap_err();
            assert!(err.to_string().contains(&format!("empty {kind} match")));
        }
    }

    #[test]
    fn validate_string_matcher_rejects_empty_prefix_suffix_and_contains() {
        let mut prefix = EnvoyStringMatcher::new();
        prefix.set_prefix("");
        let mut prefix_header = EnvoyHeaderMatcher::new();
        prefix_header.set_name("x-tenant");
        prefix_header.set_string_match(prefix);

        let mut suffix = EnvoyStringMatcher::new();
        suffix.set_suffix("");
        let mut suffix_header = EnvoyHeaderMatcher::new();
        suffix_header.set_name("x-tenant");
        suffix_header.set_string_match(suffix);

        let mut contains = EnvoyStringMatcher::new();
        contains.set_contains("");
        let mut contains_header = EnvoyHeaderMatcher::new();
        contains_header.set_name("x-tenant");
        contains_header.set_string_match(contains);

        for (kind, header) in [
            ("prefix", prefix_header),
            ("suffix", suffix_header),
            ("contains", contains_header),
        ] {
            let err = RouteConfigResource::validate(route_config_with_header(header)).unwrap_err();
            assert!(err.to_string().contains(&format!("empty {kind} match")));
        }
    }

    #[test]
    fn cluster_names_are_scoped_to_virtual_host() {
        let mut rc = make_route_config("rc-1");
        let mut other = EnvoyVirtualHost::new();
        other.set_name("vh2");
        other.domains_mut().push("other.example.com");
        other.routes_mut().push(make_route("/", "cluster-2"));
        rc.virtual_hosts_mut().push(other);

        let validated = RouteConfigResource::validate(rc).unwrap();
        assert_eq!(
            validated.virtual_hosts[0].cluster_names(),
            HashSet::from(["cluster-1".to_string()])
        );
        assert_eq!(
            validated.virtual_hosts[1].cluster_names(),
            HashSet::from(["cluster-2".to_string()])
        );
    }

    #[test]
    fn deserialize_roundtrip() {
        let rc = make_route_config("test");
        let bytes = rc.serialize().expect("serialize");
        let deserialized = RouteConfigResource::deserialize(bytes::Bytes::from(bytes)).unwrap();
        assert_eq!(RouteConfigResource::name(&deserialized), "test");
    }

    fn route_config_with_match(route_match: EnvoyRouteMatch) -> RouteConfiguration {
        let mut route_action = EnvoyRouteAction::new();
        route_action.set_cluster("cluster-1");

        let mut route = EnvoyRoute::new();
        route.set_match(route_match);
        route.set_route(route_action);

        let mut vh = EnvoyVirtualHost::new();
        vh.set_name("vh1");
        vh.domains_mut().push("*");
        vh.routes_mut().push(route);

        let mut rc = RouteConfiguration::new();
        rc.set_name("rc-1");
        rc.virtual_hosts_mut().push(vh);
        rc
    }

    #[test]
    fn validate_rejects_empty_path_regex() {
        let mut regex = RegexMatcher::new();
        regex.set_regex("");
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_safe_regex(regex);

        let err = RouteConfigResource::validate(route_config_with_match(route_match)).unwrap_err();
        assert!(err.to_string().contains("empty path regex"));
    }

    #[test]
    fn validate_rejects_empty_header_regex() {
        let mut regex = RegexMatcher::new();
        regex.set_regex("");
        let mut header = EnvoyHeaderMatcher::new();
        header.set_name("x-test");
        header.set_safe_regex_match(regex);

        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.headers_mut().push(header);

        let err = RouteConfigResource::validate(route_config_with_match(route_match)).unwrap_err();
        assert!(err.to_string().contains("empty header regex"));
    }

    #[test]
    fn validate_rejects_empty_header_matcher_name() {
        let mut header = EnvoyHeaderMatcher::new();
        header.set_present_match(true);

        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.headers_mut().push(header);

        let err = RouteConfigResource::validate(route_config_with_match(route_match)).unwrap_err();
        assert!(err.to_string().contains("header matcher name is empty"));
    }

    #[test]
    fn validate_runtime_fraction_normalizes_denominator() {
        let mut frac = FractionalPercent::new();
        frac.set_numerator(25);
        frac.set_denominator(DenominatorType::Hundred);
        let mut rf = RuntimeFractionalPercent::new();
        rf.set_default_value(frac);

        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.set_runtime_fraction(rf);

        let validated =
            RouteConfigResource::validate(route_config_with_match(route_match)).unwrap();
        assert_eq!(
            validated.virtual_hosts[0].routes[0]
                .route_match
                .match_fraction,
            Some(250_000)
        );
    }

    #[test]
    fn validate_rejects_runtime_fraction_without_default_value() {
        let mut route_match = EnvoyRouteMatch::new();
        route_match.set_prefix("/");
        route_match.set_runtime_fraction(RuntimeFractionalPercent::new());

        let err = RouteConfigResource::validate(route_config_with_match(route_match)).unwrap_err();
        assert!(err.to_string().contains("default_value"));
    }
}
