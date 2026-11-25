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

//! grpc-rls implements the Route Lookup Service (RLS) load balancing policy
//! for Rust gRPC.

#[allow(
    dead_code,
    reason = "no caller until RLS control channel is implemented."
)]
mod adaptive;

/// Module `generated` contains the generated Protobuf messages and the
/// `RouteLookupService` client stub for the `grpc.lookup.v1` package. They are
/// an implementation detail consumed only within grpc-rls. The blanket `allow`
/// covers machine-generated code, which `build.rs` writes into `OUT_DIR`.
#[allow(
    missing_docs,
    missing_debug_implementations,
    unreachable_pub,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::all,
    // Denied by the workspace lints but not covered by `clippy::all`.
    clippy::uninlined_format_args
)]
mod generated {
    grpc::include_proto!("grpc/lookup/v1", "rls");
}

#[cfg(test)]
mod tests {
    //! Sanity checks that the generated RLS modules are importable and usable:
    //! construct a message, then round-trip it through the protobuf wire format.
    //!
    //! This lives as an in-crate unit test rather than a doctest because the
    //! `generated` module is private — doctests compile as an external crate
    //! and can only reach the public API.

    use crate::generated::grpc_lookup_v1_rls_config_proto::RouteLookupClusterSpecifier;
    use crate::generated::grpc_lookup_v1_rls_config_proto::RouteLookupConfig;
    use crate::generated::grpc_lookup_v1_rls_proto::RouteLookupRequest;
    use crate::generated::grpc_lookup_v1_rls_proto::RouteLookupResponse;

    /// Encodes then decodes a message. `serialize` / `parse` are methods of the
    /// `protobuf::Message` trait (in scope via the `M: protobuf::Message` bound).
    fn round_trip<M: protobuf::Message>(msg: &M) -> M {
        M::parse(&msg.serialize().expect("serialize")).expect("parse")
    }

    #[test]
    fn rls_route_lookup_request_round_trip() {
        let mut req = RouteLookupRequest::new();
        req.set_target_type("grpc");
        req.set_stale_header_data("stale-data");
        let decoded = round_trip(&req);
        assert_eq!(decoded.target_type().as_bytes(), b"grpc");
        assert_eq!(decoded.stale_header_data().as_bytes(), b"stale-data");
    }

    #[test]
    fn rls_route_lookup_response_round_trip() {
        let mut resp = RouteLookupResponse::new();
        resp.set_header_data("rls-header");
        resp.targets_mut().push("target-a");
        let decoded = round_trip(&resp);
        assert_eq!(decoded.header_data().as_bytes(), b"rls-header");
        assert_eq!(decoded.targets().len(), 1);
        assert_eq!(decoded.targets().get(0).unwrap().as_bytes(), b"target-a");
    }

    #[test]
    fn rls_route_lookup_config_round_trip() {
        let mut cfg = RouteLookupConfig::new();
        cfg.set_lookup_service("rls.example.com:443");
        let mut spec = RouteLookupClusterSpecifier::new();
        spec.set_route_lookup_config(cfg);
        let decoded = round_trip(&spec);
        assert_eq!(
            decoded.route_lookup_config().lookup_service().as_bytes(),
            b"rls.example.com:443"
        );
    }
}
