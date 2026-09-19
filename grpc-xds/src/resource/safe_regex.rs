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

//! Validated full-string regex for `envoy.type.matcher.v3.RegexMatcher`.

use std::fmt;

use regex::Regex;
use xds_client::Error;

const ANCHOR_PREFIX: &str = r"\A(?:";
const ANCHOR_SUFFIX: &str = r")\z";

/// A regex that can only match an entire input, as required by Envoy.
#[derive(Clone)]
pub(crate) struct SafeRegex(Regex);

impl SafeRegex {
    fn new(pattern: &str) -> Result<Self, regex::Error> {
        // Validate before wrapping: an unmatched ')' must not be able to close
        // our group and introduce an unanchored alternative.
        Regex::new(pattern)?;
        // The group binds all alternatives; \A and \z also resist multiline flags.
        Regex::new(&format!("{ANCHOR_PREFIX}{pattern}{ANCHOR_SUFFIX}")).map(Self)
    }

    pub(crate) fn is_match(&self, value: &str) -> bool {
        self.0.is_match(value)
    }
}

impl fmt::Debug for SafeRegex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let anchored = self.0.as_str();
        let pattern = &anchored[ANCHOR_PREFIX.len()..anchored.len() - ANCHOR_SUFFIX.len()];
        f.debug_tuple("SafeRegex").field(&pattern).finish()
    }
}

/// Compiles a `RegexMatcher` pattern, rejecting the empty pattern.
pub(crate) fn compile_regex(pattern: &str, kind: &str) -> xds_client::Result<SafeRegex> {
    if pattern.is_empty() {
        return Err(Error::Validation(format!(
            "empty {kind} regex is not allowed"
        )));
    }
    SafeRegex::new(pattern)
        .map_err(|e| Error::Validation(format!("invalid {kind} regex '{pattern}': {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_matches_only_the_full_input() {
        for (pattern, accepted, rejected) in [
            (
                "/service/method",
                "/service/method",
                "/service/methodSuffix",
            ),
            (
                "/service/method",
                "/service/method",
                "prefix/service/method",
            ),
            ("a|ab", "ab", "abc"),
            ("a|b", "b", "xb"),
            ("a", "a", "a\n"),
            ("(?m)a", "a", "x\na\ny"),
            ("(?i)foo", "FOO", "FOOD"),
            (r"\)", ")", "))"),
            ("[)]", ")", "x)"),
            ("/caf\u{e9}/.*", "/caf\u{e9}/x", "x/caf\u{e9}/x"),
            (".*", "", "a\nb"),
        ] {
            let matcher = compile_regex(pattern, "test").unwrap();
            assert!(matcher.is_match(accepted), "{pattern:?}: {accepted:?}");
            assert!(!matcher.is_match(rejected), "{pattern:?}: {rejected:?}");
        }
    }

    #[test]
    fn invalid_patterns_cannot_escape_the_anchors() {
        for pattern in ["(unclosed", "foo)|bar(?:", "foo)(?:bar", ")(?:", r")\"] {
            assert!(compile_regex(pattern, "test").is_err(), "{pattern:?}");
        }
    }

    #[test]
    fn inline_comments_cannot_remove_the_anchors() {
        assert!(compile_regex("(?x)#", "test").is_err());
    }

    #[test]
    fn compile_errors_preserve_resource_context() {
        for kind in ["path", "header", "string matcher"] {
            let empty = compile_regex("", kind).unwrap_err();
            assert!(empty.to_string().contains(&format!("empty {kind} regex")));
            let invalid = compile_regex("[", kind).unwrap_err();
            assert!(
                invalid
                    .to_string()
                    .contains(&format!("invalid {kind} regex"))
            );
        }
    }

    #[test]
    fn debug_reports_the_original_pattern() {
        let matcher = compile_regex("/caf\u{e9}/a|/b", "test").unwrap();
        assert_eq!(format!("{matcher:?}"), "SafeRegex(\"/caf\u{e9}/a|/b\")");
    }
}
