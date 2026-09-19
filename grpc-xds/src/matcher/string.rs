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

//! Runtime evaluation of validated Envoy string matchers.

use crate::resource::StringMatcher;

impl StringMatcher {
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

pub(crate) fn starts_with_ignore_ascii_case(input: &str, prefix: &str) -> bool {
    input
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
}

pub(crate) fn ends_with_ignore_ascii_case(input: &str, suffix: &str) -> bool {
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

    fn proto(kind: &str, value: &str, ignore_case: bool) -> EnvoyStringMatcher {
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
    fn literal_patterns_match_with_configured_case_sensitivity() {
        for (kind, accepted, rejected) in [
            ("exact", "Foo", "FooBar"),
            ("prefix", "FooBar", "BarFoo"),
            ("suffix", "BarFoo", "FooBar"),
            ("contains", "xFooBar", "Bar"),
        ] {
            for ignore_case in [false, true] {
                let proto = proto(kind, "Foo", ignore_case);
                let matcher = StringMatcher::from_proto(proto.as_view()).unwrap();
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
