/*
 *
 * Copyright 2025 gRPC authors.
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

//! Full-match regex for `envoy.type.matcher.v3.RegexMatcher`.

use regex::Regex;
use std::fmt;

/// The anchors added by [`SafeRegex::new`], named so that [`SafeRegex::pattern`]
/// strips back exactly what was added.
const ANCHOR_PREFIX: &str = r"\A(?:";
const ANCHOR_SUFFIX: &str = r")\z";

/// A `RegexMatcher` that matches only the entire input.
///
/// Envoy requires a full match, but [`Regex::is_match`] searches anywhere in
/// the haystack. The anchors are applied on construction and the inner regex
/// is not exposed, so a substring match is unreachable by construction.
///
/// Not for `RegexMatchAndSubstitute`, which matches portions of a string and
/// expects callers to supply their own anchors.
#[derive(Clone)]
pub(crate) struct SafeRegex(Regex);

impl SafeRegex {
    /// Compile `pattern` to match only the entire input.
    ///
    /// The non-capturing group keeps a top-level alternation from escaping the
    /// anchors and confines any inline flags the pattern sets; `\A`/`\z` hold
    /// regardless of those flags, whereas `^`/`$` become line anchors under
    /// `(?m)`.
    pub(crate) fn new(pattern: &str) -> Result<Self, regex::Error> {
        // Splicing is only sound for a pattern that is valid alone: a free `)`
        // closes ANCHOR_PREFIX early, leaving `foo)|bar(?:` spliced as the
        // unanchored `\A(?:foo)|bar(?:)\z`.
        Regex::new(pattern)?;
        Regex::new(&format!("{ANCHOR_PREFIX}{pattern}{ANCHOR_SUFFIX}")).map(Self)
    }

    /// Returns true if the regex matches `haystack` in its entirety.
    pub(crate) fn is_match(&self, haystack: &str) -> bool {
        self.0.is_match(haystack)
    }

    /// The pattern as received, which is what `Debug` should report.
    ///
    /// Borrowed back out of the compiled regex rather than stored, since
    /// `as_str` already retains the anchored form.
    fn pattern(&self) -> &str {
        let anchored = self.0.as_str();
        &anchored[ANCHOR_PREFIX.len()..anchored.len() - ANCHOR_SUFFIX.len()]
    }
}

impl fmt::Debug for SafeRegex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SafeRegex").field(&self.pattern()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_the_entire_input() {
        let re = SafeRegex::new(r"/pkg\.Greeter/SayHello").unwrap();
        assert!(re.is_match("/pkg.Greeter/SayHello"));
        assert!(
            !re.is_match("/pkg.Greeter/SayHelloAgain"),
            "a longer input sharing the prefix must not match"
        );
        assert!(
            !re.is_match("/other/x/pkg.Greeter/SayHello"),
            "the pattern must not match as a substring"
        );
    }

    #[test]
    fn anchors_every_alternation_branch() {
        let re = SafeRegex::new("/a|/b").unwrap();
        assert!(re.is_match("/a"));
        assert!(re.is_match("/b"));
        assert!(
            !re.is_match("/aX"),
            "a top-level alternation must not escape the anchors"
        );
    }

    #[test]
    fn does_not_match_a_trailing_newline() {
        let re = SafeRegex::new("/a").unwrap();
        assert!(!re.is_match("/a\n"));
    }

    #[test]
    fn inline_flags_cannot_widen_the_anchors() {
        // A control plane could set `(?m)`; the anchors must still bind to the
        // whole haystack rather than to a line within it.
        let re = SafeRegex::new("(?m)/a").unwrap();
        assert!(!re.is_match("x\n/a\ny"));
    }

    #[test]
    fn an_invalid_pattern_is_rejected() {
        assert!(SafeRegex::new("(unclosed").is_err());
    }

    #[test]
    fn splicing_into_the_anchors_is_rejected() {
        for pattern in ["foo)|bar(?:", "foo)(?:bar", ")(?:", "a)(?:b", r")\"] {
            assert!(
                SafeRegex::new(pattern).is_err(),
                "{pattern:?} is not valid alone and must not be accepted"
            );
        }
    }

    #[test]
    fn commenting_out_the_anchors_is_rejected() {
        assert!(Regex::new("(?x)#").is_ok(), "valid on its own");
        assert!(SafeRegex::new("(?x)#").is_err());
    }

    #[test]
    fn realistic_patterns_full_match() {
        for (pattern, matches, rejects) in [
            (".*", "anything", "a\nb"),
            ("/a|/b", "/b", "/bX"),
            ("(?i)/Foo", "/fOO", "/Foo/bar"),
            ("v[0-9]+", "v12", "v12a"),
            ("[)]", ")", "))"),
            (r"\)", ")", "x)"),
            ("(?:a|b)+", "abab", "abc"),
            ("/caf€/.*", "/caf€/x", "y/caf€/x"),
            (
                r"/pkg\.[A-Za-z]+/.*",
                "/pkg.Greeter/SayHello",
                "/pkg.Greeter",
            ),
        ] {
            let re = SafeRegex::new(pattern)
                .unwrap_or_else(|e| panic!("{pattern:?} should compile: {e}"));
            assert!(re.is_match(matches), "{pattern:?} should match {matches:?}");
            assert!(
                !re.is_match(rejects),
                "{pattern:?} should not match {rejects:?}"
            );
        }
    }

    #[test]
    fn debug_reports_the_original_pattern() {
        let re = SafeRegex::new("/a|/b").unwrap();
        assert_eq!(format!("{re:?}"), r#"SafeRegex("/a|/b")"#);
    }

    #[test]
    fn the_original_pattern_survives_multibyte_characters() {
        let re = SafeRegex::new("/caf€/.*").unwrap();
        assert_eq!(format!("{re:?}"), r#"SafeRegex("/caf€/.*")"#);
    }
}
