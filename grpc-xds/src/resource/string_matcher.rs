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

//! Envoy string matching shared by routing and TLS SAN matching.

use xds_client::Error;

use super::safe_regex::SafeRegex;
use crate::generated::envoy::r#type::matcher::v3::StringMatcherView;
use crate::generated::envoy::r#type::matcher::v3::string_matcher::MatchPatternOneof;

/// An owned `envoy.type.matcher.v3.StringMatcher`.
///
/// [`Self::from_proto`] validates protobuf input. Literal constructors perform
/// no validation; callers must enforce any context-specific restrictions.
#[derive(Debug, Clone)]
pub(crate) enum StringMatcher {
    Exact { value: String, ignore_case: bool },
    Prefix { value: String, ignore_case: bool },
    Suffix { value: String, ignore_case: bool },
    Contains { value: String, ignore_case: bool },
    SafeRegex(SafeRegex),
}

impl StringMatcher {
    /// Matches inputs equal to `value`.
    pub(crate) fn exact(value: impl Into<String>, ignore_case: bool) -> Self {
        Self::Exact {
            value: value.into(),
            ignore_case,
        }
    }

    /// Matches inputs starting with `value`.
    pub(crate) fn prefix(value: impl Into<String>, ignore_case: bool) -> Self {
        Self::Prefix {
            value: value.into(),
            ignore_case,
        }
    }

    /// Matches inputs ending with `value`.
    pub(crate) fn suffix(value: impl Into<String>, ignore_case: bool) -> Self {
        Self::Suffix {
            value: value.into(),
            ignore_case,
        }
    }

    /// Matches inputs containing `value`.
    pub(crate) fn contains(value: impl Into<String>, ignore_case: bool) -> Self {
        Self::Contains {
            value: value.into(),
            ignore_case,
        }
    }

    /// Parses and validates an `envoy.type.matcher.v3.StringMatcher`.
    ///
    /// Returns an error if the `match_pattern` oneof is unset or carries an
    /// unsupported variant, a pattern contains invalid UTF-8, a
    /// prefix/suffix/contains value is empty, or a `safe_regex` fails to compile.
    pub(crate) fn from_proto(proto: StringMatcherView<'_>) -> xds_client::Result<Self> {
        let ignore_case = proto.ignore_case();
        match proto.match_pattern() {
            MatchPatternOneof::Exact(value) => {
                let value = value.to_str().map_err(|e| {
                    Error::Validation(format!("invalid UTF-8 in exact string matcher: {e}"))
                })?;
                Ok(Self::exact(value, ignore_case))
            }
            MatchPatternOneof::Prefix(value) => {
                let value = value.to_str().map_err(|e| {
                    Error::Validation(format!("invalid UTF-8 in prefix string matcher: {e}"))
                })?;
                Ok(Self::prefix(
                    non_empty_match_value(value, "prefix")?,
                    ignore_case,
                ))
            }
            MatchPatternOneof::Suffix(value) => {
                let value = value.to_str().map_err(|e| {
                    Error::Validation(format!("invalid UTF-8 in suffix string matcher: {e}"))
                })?;
                Ok(Self::suffix(
                    non_empty_match_value(value, "suffix")?,
                    ignore_case,
                ))
            }
            MatchPatternOneof::Contains(value) => {
                let value = value.to_str().map_err(|e| {
                    Error::Validation(format!("invalid UTF-8 in contains string matcher: {e}"))
                })?;
                Ok(Self::contains(
                    non_empty_match_value(value, "contains")?,
                    ignore_case,
                ))
            }
            MatchPatternOneof::SafeRegex(r) => {
                let pattern = r.regex();
                let pattern = pattern.to_str().map_err(|e| {
                    Error::Validation(format!("invalid UTF-8 in string matcher regex: {e}"))
                })?;
                Ok(Self::SafeRegex(pattern.parse().map_err(|e| {
                    Error::Validation(format!("invalid string matcher regex '{pattern}': {e}"))
                })?))
            }
            MatchPatternOneof::not_set(_) => Err(Error::Validation(
                "StringMatcher has no match_pattern set".into(),
            )),
            _ => Err(Error::Validation(
                "unsupported StringMatcher pattern".into(),
            )),
        }
    }

    /// `ignore_case` uses ASCII case folding and does not affect regexes.
    /// Literal patterns never interpret wildcards.
    pub(crate) fn is_match(&self, input: &str) -> bool {
        match self {
            Self::Exact { value, ignore_case } => {
                if *ignore_case {
                    input.eq_ignore_ascii_case(value)
                } else {
                    input == value
                }
            }
            Self::Prefix { value, ignore_case } => {
                if *ignore_case {
                    starts_with_ignore_ascii_case(input, value)
                } else {
                    input.starts_with(value)
                }
            }
            Self::Suffix { value, ignore_case } => {
                if *ignore_case {
                    ends_with_ignore_ascii_case(input, value)
                } else {
                    input.ends_with(value)
                }
            }
            Self::Contains { value, ignore_case } => {
                if *ignore_case {
                    contains_ignore_ascii_case(input, value)
                } else {
                    input.contains(value)
                }
            }
            Self::SafeRegex(regex) => regex.is_match(input),
        }
    }
}

pub(crate) fn non_empty_match_value(value: &str, kind: &str) -> xds_client::Result<String> {
    if value.is_empty() {
        return Err(Error::Validation(format!(
            "empty {kind} match is not allowed"
        )));
    }
    Ok(value.to_string())
}

fn starts_with_ignore_ascii_case(input: &str, prefix: &str) -> bool {
    input
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
}

fn ends_with_ignore_ascii_case(input: &str, suffix: &str) -> bool {
    input
        .len()
        .checked_sub(suffix.len())
        .is_some_and(|start| input.as_bytes()[start..].eq_ignore_ascii_case(suffix.as_bytes()))
}

fn contains_ignore_ascii_case(input: &str, pattern: &str) -> bool {
    let (input, pattern) = (input.as_bytes(), pattern.as_bytes());
    let Some(max_start) = input.len().checked_sub(pattern.len()) else {
        return false;
    };
    (0..=max_start).any(|i| input[i..i + pattern.len()].eq_ignore_ascii_case(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::envoy::r#type::matcher::v3::StringMatcher as EnvoyStringMatcher;

    fn proto(
        kind: &str,
        value: impl protobuf::IntoProxied<protobuf::ProtoString>,
        ignore_case: bool,
    ) -> EnvoyStringMatcher {
        let mut proto = EnvoyStringMatcher::new();
        proto.set_ignore_case(ignore_case);
        match kind {
            "exact" => proto.set_exact(value),
            "prefix" => proto.set_prefix(value),
            "suffix" => proto.set_suffix(value),
            "contains" => proto.set_contains(value),
            "regex" => proto.safe_regex_mut().set_regex(value),
            _ => panic!("unknown matcher kind"),
        }
        proto
    }

    #[test]
    fn generic_patterns_reject_invalid_utf8() {
        for bytes in [
            b"\xff".as_slice(),
            b"valid\xff".as_slice(),
            b"\xc3".as_slice(),
        ] {
            for (kind, field) in [
                ("exact", "exact string matcher"),
                ("prefix", "prefix string matcher"),
                ("suffix", "suffix string matcher"),
                ("contains", "contains string matcher"),
                ("regex", "string matcher regex"),
            ] {
                let value = protobuf::ProtoStr::from_utf8_unchecked(bytes);
                let proto = proto(kind, value, false);
                let Error::Validation(message) =
                    StringMatcher::from_proto(proto.as_view()).unwrap_err()
                else {
                    panic!("expected a validation error");
                };
                assert!(
                    message.starts_with(&format!("invalid UTF-8 in {field}:")),
                    "{kind}, {bytes:?}: {message}"
                );
            }
        }
    }

    #[test]
    fn empty_non_exact_patterns_are_rejected() {
        let mut prefix = EnvoyStringMatcher::new();
        prefix.set_prefix("");
        let mut suffix = EnvoyStringMatcher::new();
        suffix.set_suffix("");
        let mut contains = EnvoyStringMatcher::new();
        contains.set_contains("");
        let mut regex = EnvoyStringMatcher::new();
        regex.safe_regex_mut().set_regex("");

        for (kind, proto) in [
            ("prefix", prefix),
            ("suffix", suffix),
            ("contains", contains),
            ("regex", regex),
        ] {
            let err = StringMatcher::from_proto(proto.as_view()).unwrap_err();
            assert!(err.to_string().contains("empty"), "{kind}: {err}");
        }
    }

    #[test]
    fn missing_and_unsupported_patterns_are_rejected() {
        let missing = EnvoyStringMatcher::new();
        let err = StringMatcher::from_proto(missing.as_view()).unwrap_err();
        assert!(err.to_string().contains("no match_pattern"));

        let mut unsupported = EnvoyStringMatcher::new();
        unsupported.custom_mut().set_name("custom");
        let err = StringMatcher::from_proto(unsupported.as_view()).unwrap_err();
        assert!(err.to_string().contains("unsupported StringMatcher"));
    }

    #[test]
    fn regex_errors_preserve_resource_context() {
        for (pattern, reason) in [
            ("", "empty regex is not allowed"),
            ("[", "unclosed character class"),
        ] {
            let mut proto = EnvoyStringMatcher::new();
            proto.safe_regex_mut().set_regex(pattern);
            let Error::Validation(message) =
                StringMatcher::from_proto(proto.as_view()).unwrap_err()
            else {
                panic!("expected a validation error");
            };
            assert!(
                message.starts_with(&format!("invalid string matcher regex '{pattern}':")),
                "{message}"
            );
            assert!(message.contains(reason), "{message}");
        }
    }

    #[test]
    fn literal_patterns_match_with_configured_case_sensitivity() {
        for ignore_case in [false, true] {
            for (kind, matcher, accepted, rejected) in [
                (
                    "exact",
                    StringMatcher::exact("Foo", ignore_case),
                    "Foo",
                    "FooBar",
                ),
                (
                    "prefix",
                    StringMatcher::prefix("Foo", ignore_case),
                    "FooBar",
                    "BarFoo",
                ),
                (
                    "suffix",
                    StringMatcher::suffix("Foo", ignore_case),
                    "BarFoo",
                    "FooBar",
                ),
                (
                    "contains",
                    StringMatcher::contains("Foo", ignore_case),
                    "xFooBar",
                    "Bar",
                ),
            ] {
                let proto = proto(kind, "Foo", ignore_case);
                for matcher in [matcher, StringMatcher::from_proto(proto.as_view()).unwrap()] {
                    assert!(matcher.is_match(accepted), "{kind}");
                    assert!(matcher.is_match("Foo"), "{kind}");
                    assert!(!matcher.is_match(rejected), "{kind}");
                    assert!(!matcher.is_match(""), "{kind}");
                    assert_eq!(
                        matcher.is_match(&accepted.to_ascii_lowercase()),
                        ignore_case,
                        "{kind}"
                    );
                }
            }
        }
    }

    #[test]
    fn ascii_case_folding_handles_utf8_without_slicing_panics() {
        for kind in ["exact", "prefix", "suffix", "contains"] {
            let proto = proto(kind, "\u{e9}A", true);
            let matcher = StringMatcher::from_proto(proto.as_view()).unwrap();
            assert!(matcher.is_match("\u{e9}a"), "{kind}");
            assert!(!matcher.is_match("\u{c9}a"), "{kind}");
            assert!(!matcher.is_match("A"), "{kind}");
            assert!(!matcher.is_match("\u{1f600}"), "{kind}");
        }
        assert!(!starts_with_ignore_ascii_case("\u{e9}", "a"));
        assert!(!ends_with_ignore_ascii_case("\u{e9}", "a"));
        assert!(contains_ignore_ascii_case("\u{1f600}AbC\u{e9}", "abc"));
    }

    #[test]
    fn exact_patterns_can_be_empty_and_wildcards_are_literal() {
        let empty = proto("exact", "", false);
        let matcher = StringMatcher::from_proto(empty.as_view()).unwrap();
        assert!(matcher.is_match(""));
        assert!(!matcher.is_match("x"));

        let wildcard = proto("exact", "*.example.com", true);
        let matcher = StringMatcher::from_proto(wildcard.as_view()).unwrap();
        assert!(matcher.is_match("*.EXAMPLE.COM"));
        assert!(!matcher.is_match("api.example.com"));
    }

    #[test]
    fn regex_ignores_ignore_case_and_requires_a_full_match() {
        let proto = proto("regex", "Foo|Bar", true);
        let matcher = StringMatcher::from_proto(proto.as_view()).unwrap();
        assert!(matcher.is_match("Foo"));
        assert!(matcher.is_match("Bar"));
        assert!(!matcher.is_match("foo"));
        assert!(!matcher.is_match("FooBar"));
        assert!(!matcher.is_match("xBar"));
    }
}
