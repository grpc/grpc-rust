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

//! Validated Envoy string matcher values shared by header and SAN configuration.

use xds_client::Error;

use super::safe_regex::{SafeRegex, compile_regex};
use crate::generated::envoy::r#type::matcher::v3::StringMatcherView;
use crate::generated::envoy::r#type::matcher::v3::string_matcher::MatchPatternOneof;

/// Validated `envoy.type.matcher.v3.StringMatcher`.
#[derive(Debug, Clone)]
pub(crate) enum StringMatcher {
    Exact { value: String, ignore_case: bool },
    Prefix { value: String, ignore_case: bool },
    Suffix { value: String, ignore_case: bool },
    Contains { value: String, ignore_case: bool },
    SafeRegex(SafeRegex),
}

impl StringMatcher {
    /// Parses and validates an `envoy.type.matcher.v3.StringMatcher`.
    ///
    /// Returns an error if the `match_pattern` oneof is unset or carries an
    /// unsupported variant, a prefix/suffix/contains value is empty, or a
    /// `safe_regex` fails to compile.
    pub(crate) fn from_proto(proto: StringMatcherView<'_>) -> xds_client::Result<Self> {
        let ignore_case = proto.ignore_case();
        match proto.match_pattern() {
            MatchPatternOneof::Exact(value) => Ok(Self::Exact {
                value: value.to_str().unwrap_or_default().to_string(),
                ignore_case,
            }),
            MatchPatternOneof::Prefix(value) => Ok(Self::Prefix {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "prefix")?,
                ignore_case,
            }),
            MatchPatternOneof::Suffix(value) => Ok(Self::Suffix {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "suffix")?,
                ignore_case,
            }),
            MatchPatternOneof::Contains(value) => Ok(Self::Contains {
                value: non_empty_match_value(value.to_str().unwrap_or_default(), "contains")?,
                ignore_case,
            }),
            MatchPatternOneof::SafeRegex(r) => {
                let pattern = r.regex();
                Ok(Self::SafeRegex(compile_regex(
                    pattern.to_str().unwrap_or_default(),
                    "string matcher",
                )?))
            }
            MatchPatternOneof::not_set(_) => Err(Error::Validation(
                "StringMatcher has no match_pattern set".into(),
            )),
            _ => Err(Error::Validation(
                "unsupported StringMatcher pattern".into(),
            )),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::envoy::r#type::matcher::v3::StringMatcher as EnvoyStringMatcher;

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
    fn missing_unsupported_and_invalid_patterns_are_rejected() {
        let missing = EnvoyStringMatcher::new();
        let err = StringMatcher::from_proto(missing.as_view()).unwrap_err();
        assert!(err.to_string().contains("no match_pattern"));

        let mut unsupported = EnvoyStringMatcher::new();
        unsupported.custom_mut().set_name("custom");
        let err = StringMatcher::from_proto(unsupported.as_view()).unwrap_err();
        assert!(err.to_string().contains("unsupported StringMatcher"));

        let mut invalid = EnvoyStringMatcher::new();
        invalid.safe_regex_mut().set_regex("[");
        let err = StringMatcher::from_proto(invalid.as_view()).unwrap_err();
        assert!(err.to_string().contains("invalid string matcher regex"));
    }
}
