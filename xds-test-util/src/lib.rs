//! Test utilities for xDS implementations.
//!
//! This crate provides helpers for exercising xDS clients against an in-process
//! management server. It is intended for tests only and not for production use.
//!
//! The main entry point is [`XdsTestControlPlaneService`], a fake Aggregated
//! Discovery Service (ADS) control plane. It is a Rust port of grpc-java's
//! `XdsTestControlPlaneService`.
//!
//! ```ignore
//! let running = XdsTestControlPlaneService::new().start().await?;
//! running.set_xds_config(&AdsTypeUrl::Lds, listeners);
//! let addr = running.addr(); // point your xDS bootstrap here
//! // `running` shuts the server down when dropped.
//! ```

pub mod config;
mod control_plane;

pub use control_plane::{RunningControlPlane, XdsTestControlPlaneService};
