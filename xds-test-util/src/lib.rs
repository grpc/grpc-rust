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
    ADS_TYPE_URL_CDS, ADS_TYPE_URL_EDS, ADS_TYPE_URL_LDS, ADS_TYPE_URL_RDS,
    XdsTestControlPlaneService,
};

/// Re-export of the generated ADS server adapter, so tests can register the
/// control plane with a [`tonic`] server without depending on `envoy-types`
/// module paths directly:
///
/// ```ignore
/// let service = XdsTestControlPlaneService::new();
/// Server::builder()
///     .add_service(AggregatedDiscoveryServiceServer::new(service.clone()))
///     .serve(addr)
///     .await?;
/// ```
pub use envoy_types::pb::envoy::service::discovery::v3::aggregated_discovery_service_server::AggregatedDiscoveryServiceServer;
