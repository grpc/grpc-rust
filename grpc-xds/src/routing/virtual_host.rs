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

//! Virtual-host domain matching for channel configuration (gRFC A27).

use std::cmp::Reverse;

use crate::resource::DomainMatchType;
use crate::resource::StringMatcher;
use crate::resource::VirtualHost;

/// Selects the best virtual host for the channel's data-plane authority.
///
/// Matching is ASCII case-insensitive: exact > suffix > prefix > universal.
/// Longer patterns win within a category; equal matches choose the first host.
/// Wildcards consume at least one character. Ports and IPv6 brackets are preserved.
pub(crate) fn find_virtual_host_index(
    authority: &str,
    virtual_hosts: &[VirtualHost],
) -> Option<usize> {
    virtual_hosts
        .iter()
        .enumerate()
        .filter_map(|(index, host)| {
            host.domains
                .iter()
                .filter_map(|pattern| {
                    DomainMatchType::try_from(pattern.as_str())
                        .ok()?
                        .match_domain(authority, pattern)
                })
                .min()
                .map(|score| (score, index))
        })
        .min()
        .map(|(_, index)| index)
}

impl DomainMatchType {
    /// Matches the pattern from which this match type was derived.
    fn match_domain(self, authority: &str, pattern: &str) -> Option<DomainMatchScore> {
        let matches = match self {
            Self::Exact => StringMatcher::exact(pattern, true).is_match(authority),
            Self::Suffix => {
                authority.len() >= pattern.len()
                    && StringMatcher::suffix(&pattern[1..], true).is_match(authority)
            }
            Self::Prefix => {
                authority.len() >= pattern.len()
                    && StringMatcher::prefix(&pattern[..pattern.len() - 1], true)
                        .is_match(authority)
            }
            Self::Universal => !authority.is_empty(),
        };
        matches.then_some(DomainMatchScore(self, Reverse(pattern.len())))
    }
}

/// Better matches sort first, with specificity breaking ties within a category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DomainMatchScore(DomainMatchType, Reverse<usize>);

#[cfg(test)]
mod tests {
    use super::*;

    fn virtual_host(name: &str, domains: &[&str]) -> VirtualHost {
        VirtualHost {
            name: name.into(),
            domains: domains.iter().map(|domain| (*domain).into()).collect(),
            routes: Vec::new(),
        }
    }

    #[test]
    fn domain_patterns_match_authorities() {
        for (authority, pattern, expected) in [
            ("api.example.com", "api.example.com", true),
            ("API.EXAMPLE.COM", "api.example.com", true),
            ("api.example.com", "API.EXAMPLE.COM", true),
            ("api.example.com", "other.example.com", false),
            ("api.example.com", "example.com", false),
            ("api.example.com", "*.example.com", true),
            ("a.b.example.com", "*.example.com", true),
            ("api.EXAMPLE.com", "*.example.COM", true),
            ("example.com", "*.example.com", false),
            (".example.com", "*.example.com", false),
            ("api.example.com.evil", "*.example.com", false),
            ("myexample.com", "*example.com", true),
            ("example.com", "*example.com", false),
            ("baz-bar.foo.com", "*-bar.foo.com", true),
            ("-bar.foo.com", "*-bar.foo.com", false),
            ("foo.example.com", "foo.*", true),
            ("FOO.example.com", "foo.*", true),
            ("foo.", "foo.*", false),
            ("xfoo.example.com", "foo.*", false),
            ("foo-bar", "foo-*", true),
            ("foo-", "foo-*", false),
            ("foobar", "foo*", true),
            ("foo", "foo*", false),
            ("example.com", "*", true),
            ("", "*", false),
            ("", "", false),
            ("api.example.com:443", "api.example.com", false),
            ("api.example.com:443", "API.EXAMPLE.COM:443", true),
            ("api.example.com:443", "*.example.com:443", true),
            ("api.example.com:443", "*.example.com", false),
            ("api.example.com:443", "*:443", true),
            ("[2001:db8::1]:443", "[2001:DB8::1]:443", true),
            ("[2001:db8::1]:443", "[2001:db8::1]", false),
            ("api.example.com.", "api.example.com", false),
            ("\u{e9}x.example.com", "*.example.com", true),
            ("\u{e9}", "a*", false),
            ("\u{e9}", "*a", false),
            ("\u{c9}.example.com", "\u{e9}.example.com", false),
        ] {
            assert_eq!(
                DomainMatchType::try_from(pattern)
                    .is_ok_and(|kind| kind.match_domain(authority, pattern).is_some()),
                expected,
                "authority={authority:?}, pattern={pattern:?}",
            );
        }
    }

    #[test]
    fn invalid_domain_patterns_are_not_matches() {
        for pattern in ["", "a*b.example.com", "**", "*foo*", "foo**", "*foo*bar"] {
            let hosts = [virtual_host("invalid", &[pattern])];
            assert_eq!(
                find_virtual_host_index(pattern, &hosts),
                None,
                "{pattern:?}"
            );
        }
    }

    #[test]
    fn exact_match_wins_regardless_of_host_order() {
        let mut hosts = vec![
            virtual_host("universal", &["*"]),
            virtual_host("suffix", &["*.example.com"]),
            virtual_host("prefix", &["api.example.*"]),
            virtual_host("exact", &["api.example.com"]),
        ];
        for _ in 0..hosts.len() {
            let index = find_virtual_host_index("api.example.com", &hosts).unwrap();
            assert_eq!(hosts[index].name, "exact");
            hosts.rotate_left(1);
        }
    }

    #[test]
    fn suffix_beats_a_longer_prefix_regardless_of_host_order() {
        let mut hosts = vec![
            virtual_host("universal", &["*"]),
            virtual_host("prefix", &["api.example.*"]),
            virtual_host("suffix", &["*.com"]),
        ];
        for _ in 0..hosts.len() {
            let index = find_virtual_host_index("api.example.com", &hosts).unwrap();
            assert_eq!(hosts[index].name, "suffix");
            hosts.rotate_left(1);
        }
    }

    #[test]
    fn longest_pattern_wins_within_a_category() {
        for patterns in [["*.com", "*.example.com"], ["api.*", "api.example.*"]] {
            let mut hosts = vec![
                virtual_host("short", &[patterns[0]]),
                virtual_host("long", &[patterns[1]]),
            ];
            for _ in 0..hosts.len() {
                let index = find_virtual_host_index("api.example.com", &hosts).unwrap();
                assert_eq!(hosts[index].name, "long");
                hosts.reverse();
            }
        }
    }

    #[test]
    fn all_domains_are_considered_and_nonmatches_do_not_affect_precedence() {
        let hosts = vec![
            virtual_host("suffix", &["*.example.com"]),
            virtual_host("other", &["other.api.example.com", "*.other.example.com"]),
            virtual_host("exact", &["*", "other.example.com", "API.EXAMPLE.COM"]),
        ];
        assert_eq!(find_virtual_host_index("api.example.com", &hosts), Some(2));
    }

    #[test]
    fn ties_choose_the_first_virtual_host() {
        for pattern in ["api.example.com", "*.example.com", "api.*", "*"] {
            let hosts = vec![
                virtual_host("first", &[pattern]),
                virtual_host("second", &[&pattern.to_ascii_uppercase()]),
            ];
            assert_eq!(find_virtual_host_index("api.example.com", &hosts), Some(0));
        }
    }

    #[test]
    fn no_matching_virtual_host_returns_none() {
        assert_eq!(find_virtual_host_index("api.example.com", &[]), None);
        let hosts = vec![virtual_host("other", &["other.example.com"])];
        assert_eq!(find_virtual_host_index("api.example.com", &hosts), None);
        let hosts = vec![virtual_host("universal", &["*"])];
        assert_eq!(find_virtual_host_index("", &hosts), None);
    }

    #[test]
    fn prefix_and_universal_matches_are_selected_when_more_specific_matches_fail() {
        let hosts = vec![
            virtual_host("universal", &["*"]),
            virtual_host("prefix", &["api.*"]),
            virtual_host("suffix", &["*.example.com"]),
        ];
        assert_eq!(find_virtual_host_index("api.other.com", &hosts), Some(1));
        assert_eq!(find_virtual_host_index("other.test", &hosts), Some(0));
    }

    #[test]
    fn selected_index_is_used_by_the_snapshot() {
        use std::collections::HashMap;
        use std::sync::Arc;

        use crate::resource::ListenerResource;
        use crate::resource::RouteConfigResource;
        use crate::resource::RouteSource;
        use crate::xds_config::XdsConfig;

        let route_config = Arc::new(RouteConfigResource {
            name: "routes".into(),
            virtual_hosts: vec![
                virtual_host("other", &["other.example.com"]),
                virtual_host("selected", &["api.example.com"]),
            ],
        });
        let listener = Arc::new(ListenerResource {
            name: "listener".into(),
            route_source: RouteSource::Inline(Arc::clone(&route_config)),
        });
        let index =
            find_virtual_host_index("api.example.com", &route_config.virtual_hosts).unwrap();
        let config = XdsConfig::try_new(listener, route_config, index, HashMap::new()).unwrap();
        assert_eq!(config.virtual_host().name, "selected");
    }
}
