//! grpc-xds implements xDS support for Rust gRPC.

/// Module `generated` contains the generated Protobuf messages for xDS,
/// mirroring the vendored proto package tree. The messages are an implementation detail
/// consumed only within grpc-xds. The blanket `allow` covers machine-generated code;
/// the tree is built into `OUT_DIR` by `build.rs` and its root `mod.rs` is included here.
#[allow(
    missing_docs,
    missing_debug_implementations,
    unreachable_pub,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::all
)]
pub(crate) mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

#[cfg(test)]
mod tests {
    //! Sanity checks that the generated xDS modules are importable and usable
    //! for a sampling of the core discovery resources (LDS/RDS/CDS/EDS):
    //! construct a message, then round-trip it through the protobuf wire format.
    //!
    //! This lives as an in-crate unit test rather than a doctest because the
    //! `generated` module is `pub(crate)` — doctests compile as an external
    //! crate and can only reach the public API.

    use crate::generated::envoy::config::{
        cluster::v3::Cluster, endpoint::v3::ClusterLoadAssignment, listener::v3::Listener,
        route::v3::RouteConfiguration,
    };

    /// Encodes then decodes a message. `serialize` / `parse` are methods of the
    /// `protobuf::Message` trait (in scope via the `M: protobuf::Message` bound).
    fn round_trip<M: protobuf::Message>(msg: &M) -> M {
        M::parse(&msg.serialize().expect("serialize")).expect("parse")
    }

    #[test]
    fn lds_listener_round_trip() {
        let mut listener = Listener::new();
        listener.set_name("grpc-listener");
        assert_eq!(round_trip(&listener).name().as_bytes(), b"grpc-listener");
    }

    #[test]
    fn rds_route_configuration_round_trip() {
        let mut route_config = RouteConfiguration::new();
        route_config.set_name("grpc-routes");
        assert_eq!(round_trip(&route_config).name().as_bytes(), b"grpc-routes");
    }

    #[test]
    fn cds_cluster_round_trip() {
        let mut cluster = Cluster::new();
        cluster.set_name("grpc-cluster");
        assert_eq!(round_trip(&cluster).name().as_bytes(), b"grpc-cluster");
    }

    #[test]
    fn eds_cluster_load_assignment_round_trip() {
        let mut endpoints = ClusterLoadAssignment::new();
        endpoints.set_cluster_name("grpc-cluster");
        assert_eq!(
            round_trip(&endpoints).cluster_name().as_bytes(),
            b"grpc-cluster"
        );
    }
}
