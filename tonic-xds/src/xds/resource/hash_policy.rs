//! Validated request hash policy (gRFC A42) and request-hash computation.
//!
//! Hash policies are an RDS `RouteAction` concept: each policy examines part of
//! a request and produces a hash, and the per-request hashes are combined into
//! the value a hash-based LB policy (e.g. ring-hash) indexes its ring with. They
//! live in the routing/resource layer rather than the LB layer because the same
//! mechanism feeds any hash-based policy (Envoy uses it for both ring-hash and
//! maglev); ring-hash is currently the only consumer in gRPC.
//!
//! Currently it supports header-based hashing only; `ChannelId` and
//! `regex_rewrite` are deferred. RDS parsing of `RouteAction.hash_policy` lands
//! in a later PR — until then the routing layer passes an empty policy list.

/// A validated request hash policy (gRFC A42).
#[derive(Debug, Clone)]
pub(crate) enum HashPolicyConfig {
    /// Hash the value of the named request header.
    Header {
        header_name: String,
        /// If `true` and this policy produces a hash, stop combining further
        /// policies (gRFC A42 / Envoy hash-policy semantics).
        terminal: bool,
    },
}

impl HashPolicyConfig {
    /// Whether this policy is terminal (short-circuits combination once it
    /// produces a hash).
    pub(crate) fn terminal(&self) -> bool {
        match self {
            HashPolicyConfig::Header { terminal, .. } => *terminal,
        }
    }

    /// Hash for this single policy against the request, or `None` if the policy
    /// does not match (e.g. the header is absent).
    ///
    /// All values of the header (HTTP allows repeats) are joined with `,` and
    /// hashed together via `XXH64` (seed 0, per A42 / Envoy).
    fn hash(&self, headers: &http::HeaderMap) -> Option<u64> {
        match self {
            HashPolicyConfig::Header { header_name, .. } => {
                // Per A42 (deferring to gRFC A28), `-bin` headers are ignored.
                if header_name.ends_with("-bin") {
                    return None;
                }
                let mut values = headers.get_all(header_name).iter();
                let first = values.next()?;
                // Stream the comma-joined values straight into the hasher;
                // XXH64 is order-/boundary-independent, so this yields the same
                // digest as hashing one concatenated buffer, with no allocation.
                let mut hasher = xxhash_rust::xxh64::Xxh64::new(0);
                hasher.update(first.as_bytes());
                for v in values {
                    hasher.update(b",");
                    hasher.update(v.as_bytes());
                }
                Some(hasher.digest())
            }
        }
    }

    /// Compute the request hash from a list of hash policies (gRFC A42).
    ///
    /// Each matching policy contributes a hash; multiple policies are combined
    /// in order using Envoy's rotate-XOR rule (`combined = rotl(combined, 1) ^
    /// new`) — the reference deterministic combination A42 documents — which
    /// preserves entropy and prevents duplicate policies from cancelling out.
    /// Per A42, once a hash has been generated a `terminal` policy stops further
    /// combination — even if that terminal policy itself produced no hash (e.g.
    /// its header is absent), matching Envoy/grpc-go. Returns `None` if no policy
    /// produced a hash, in which case the picker falls back to a random hash.
    pub(crate) fn request_hash(
        headers: &http::HeaderMap,
        policies: &[HashPolicyConfig],
    ) -> Option<u64> {
        let mut hash: Option<u64> = None;
        for policy in policies {
            if let Some(new_hash) = policy.hash(headers) {
                hash = Some(match hash {
                    Some(prev) => prev.rotate_left(1) ^ new_hash,
                    None => new_hash,
                });
            }
            // A42: a terminal policy short-circuits once *any* hash has been
            // generated, regardless of whether this policy contributed one.
            if policy.terminal() && hash.is_some() {
                break;
            }
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header_policy(name: &str, terminal: bool) -> HashPolicyConfig {
        HashPolicyConfig::Header {
            header_name: name.to_string(),
            terminal,
        }
    }

    fn headers_with(pairs: &[(&str, &str)]) -> http::HeaderMap {
        let mut h = http::HeaderMap::new();
        for (k, v) in pairs {
            h.insert(
                http::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                http::HeaderValue::from_str(v).unwrap(),
            );
        }
        h
    }

    #[test]
    fn request_hash_single_header_matches_xxh64() {
        let headers = headers_with(&[("x-key", "user-42")]);
        let policies = [header_policy("x-key", false)];
        assert_eq!(
            HashPolicyConfig::request_hash(&headers, &policies),
            Some(xxhash_rust::xxh64::xxh64(b"user-42", 0)),
        );
    }

    #[test]
    fn request_hash_is_deterministic() {
        let headers = headers_with(&[("x-key", "abc")]);
        let policies = [header_policy("x-key", false)];
        assert_eq!(
            HashPolicyConfig::request_hash(&headers, &policies),
            HashPolicyConfig::request_hash(&headers, &policies),
        );
    }

    #[test]
    fn request_hash_distinct_values_differ() {
        let policies = [header_policy("x-key", false)];
        let a = HashPolicyConfig::request_hash(&headers_with(&[("x-key", "a")]), &policies);
        let b = HashPolicyConfig::request_hash(&headers_with(&[("x-key", "b")]), &policies);
        assert!(a.is_some() && b.is_some());
        assert_ne!(a, b);
    }

    #[test]
    fn request_hash_missing_header_is_none() {
        let headers = headers_with(&[("other", "v")]);
        let policies = [header_policy("x-key", false)];
        assert_eq!(HashPolicyConfig::request_hash(&headers, &policies), None);
    }

    #[test]
    fn request_hash_binary_header_value_still_hashes() {
        // A header value that isn't plain text still produces a hash rather than
        // being silently dropped.
        let value: &[u8] = &[0xFF, 0xFE];
        let mut h = http::HeaderMap::new();
        h.insert("x-key", http::HeaderValue::from_bytes(value).unwrap());
        assert_eq!(
            HashPolicyConfig::request_hash(&h, &[header_policy("x-key", false)]),
            Some(xxhash_rust::xxh64::xxh64(value, 0)),
        );
    }

    #[test]
    fn request_hash_multi_value_header_joins_with_comma() {
        // Repeated header values are joined with ',' and hashed together
        // (matches Envoy / grpc-go), not just the first value.
        let mut h = http::HeaderMap::new();
        h.append("x-key", http::HeaderValue::from_static("a"));
        h.append("x-key", http::HeaderValue::from_static("b"));
        assert_eq!(
            HashPolicyConfig::request_hash(&h, &[header_policy("x-key", false)]),
            Some(xxhash_rust::xxh64::xxh64(b"a,b", 0)),
        );
    }

    #[test]
    fn request_hash_bin_header_is_ignored() {
        // A `-bin` header is ignored even when present (A42 / A28).
        let mut h = http::HeaderMap::new();
        h.insert("x-key-bin", http::HeaderValue::from_static("present"));
        assert_eq!(
            HashPolicyConfig::request_hash(&h, &[header_policy("x-key-bin", false)]),
            None,
        );
    }

    #[test]
    fn request_hash_empty_policies_is_none() {
        let headers = headers_with(&[("x-key", "v")]);
        assert_eq!(HashPolicyConfig::request_hash(&headers, &[]), None);
    }

    #[test]
    fn request_hash_combines_two_headers_order_dependent() {
        let h = headers_with(&[("a", "1"), ("b", "2")]);
        let ab = HashPolicyConfig::request_hash(
            &h,
            &[header_policy("a", false), header_policy("b", false)],
        );
        let ba = HashPolicyConfig::request_hash(
            &h,
            &[header_policy("b", false), header_policy("a", false)],
        );
        assert!(ab.is_some() && ba.is_some());
        assert_ne!(ab, ba);
    }

    #[test]
    fn request_hash_terminal_stops_combination() {
        let h = headers_with(&[("a", "1"), ("b", "2")]);
        // With `a` terminal, `b` is never folded in, so the result equals the
        // single-policy hash of `a`.
        let terminal = HashPolicyConfig::request_hash(
            &h,
            &[header_policy("a", true), header_policy("b", false)],
        );
        let just_a = HashPolicyConfig::request_hash(&h, &[header_policy("a", false)]);
        assert_eq!(terminal, just_a);
    }

    #[test]
    fn request_hash_terminal_on_unmatched_policy_does_not_short_circuit() {
        let h = headers_with(&[("b", "2")]);
        // `a` is terminal but absent, and no hash has been generated yet → does
        // not short-circuit, so `b` still contributes.
        let result = HashPolicyConfig::request_hash(
            &h,
            &[header_policy("a", true), header_policy("b", false)],
        );
        let just_b = HashPolicyConfig::request_hash(&h, &[header_policy("b", false)]);
        assert_eq!(result, just_b);
        assert!(result.is_some());
    }

    #[test]
    fn request_hash_terminal_absent_after_match_short_circuits() {
        // A42: once a hash has been generated, a terminal policy short-circuits
        // even if it produces no hash of its own (its header is absent here).
        // `a` matches, then terminal `b` is absent → `c` must NOT contribute.
        let h = headers_with(&[("a", "1"), ("c", "3")]);
        let result = HashPolicyConfig::request_hash(
            &h,
            &[
                header_policy("a", false),
                header_policy("b", true),
                header_policy("c", false),
            ],
        );
        let just_a = HashPolicyConfig::request_hash(&h, &[header_policy("a", false)]);
        assert_eq!(result, just_a);
    }
}
