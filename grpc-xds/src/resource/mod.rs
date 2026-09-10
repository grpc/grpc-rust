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

//! The xDS data model layer: validated, owned representations of the raw
//! discovery-protocol resources (LDS/RDS/CDS/EDS), each implementing
//! [`xds_client::Resource`] so it can be deserialized, named, and validated
//! (gRFC A27) independently of any live xDS traffic.
//! The dependency manager assembles these into an [`crate::xds_config::XdsConfig`].

// TODO: remove once the xDS dependency manager subscribes to these resource
// types and assembles them into an XdsConfig.
#![allow(dead_code, unused_imports)]

mod cluster;
mod endpoint;
mod listener;
mod route;

pub(crate) use cluster::{ClusterDiscovery, ClusterResource};
pub(crate) use endpoint::{
    EndpointAddress, EndpointsResource, HealthStatus, LbEndpoint, Locality, LocalityLbEndpoints,
};
pub(crate) use listener::{ListenerResource, RouteSource};
pub(crate) use route::{
    HeaderMatchSpecifier, HeaderMatcher, PathSpecifier, Route, RouteAction, RouteConfigResource,
    RouteMatch, StringMatcher, VirtualHost, WeightedCluster,
};
