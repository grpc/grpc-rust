//! Test utilities for xDS implementations.
//!
//! This crate provides helpers for exercising xDS clients against an in-process
//! management server. It is intended for tests only and not for production use.
//!
//! The main entry point is [`XdsTestControlPlaneService`], a fake Aggregated
//! Discovery Service (ADS) control plane. It is a Rust port of grpc-java's
//! `XdsTestControlPlaneService`.

mod control_plane;

pub use control_plane::{
    ADS_TYPE_URL_CDS, ADS_TYPE_URL_EDS, ADS_TYPE_URL_LDS, ADS_TYPE_URL_RDS, RunningControlPlane,
    XdsTestControlPlaneService,
};

/// Re-export of the generated ADS server adapter, for advanced wiring — e.g.
/// registering the control plane on an existing [`tonic`] server alongside
/// other services. For the common case, prefer
/// [`XdsTestControlPlaneService::start`], which serves on an ephemeral port and
/// returns a [`RunningControlPlane`]:
///
/// ```ignore
/// let running = XdsTestControlPlaneService::new().start().await?;
/// running.set_xds_config(ADS_TYPE_URL_LDS, listeners);
/// let addr = running.addr(); // point your xDS bootstrap here
/// // `running` shuts the server down when dropped.
/// ```
pub use envoy_types::pb::envoy::service::discovery::v3::aggregated_discovery_service_server::AggregatedDiscoveryServiceServer;
