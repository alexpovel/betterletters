use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use super::scope::{RangesWithContext, ScopeContext};
use super::Scoper;
use crate::{RegexPattern, GLOBAL_SCOPE};

/// A regular expression for querying.
#[derive(Debug)]
pub struct Regex {
    pattern: RegexPattern,
    captures: Vec<CaptureGroup>,
}

/// A capture group in a regex, which can be either named (`(?<name>REGEX)`) or numbered
/// (`(REGEX)`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureGroup {
    /// A named capture group.
    Named(String),
    /// A numbered capture group, where 0 stands for the entire match.
    Numbered(usize),
}

impl fmt::Display for CaptureGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, r#type) = match self {
            Self::Named(name) => (name.clone(), "named"),
            Self::Numbered(number) => (number.to_string(), "numbered"),
        };
        write!(f, "{value} ({type})")
    }
}

impl Regex {
    /// Create a new regular expression.
    #[must_use]
    pub fn new(pattern: RegexPattern) -> Self {
        let capture_names = pattern
            .capture_names()
            .enumerate()
            .map(|(i, name)| {
                name.map_or(CaptureGroup::Numbered(i), |name| {
                    CaptureGroup::Named(name.to_owned())
                })
            })
            .collect();

        Self {
            pattern,
            captures: capture_names,
        }
    }
}

/// An error that can occur when parsing a regular expression.
///
/// Simple wrapper.
#[derive(Debug)]
pub struct RegexError(fancy_regex::Error);

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid regex: {}", self.0)
    }
}

impl Error for RegexError {}

impl TryFrom<String> for Regex {
    type Error = RegexError;

    fn try_from(pattern: String) -> Result<Self, Self::Error> {
        let pattern = RegexPattern::new(&pattern).map_err(RegexError)?;

        Ok(Self::new(pattern))
    }
}

impl Default for Regex {
    fn default() -> Self {
        Self::new(RegexPattern::new(GLOBAL_SCOPE).unwrap())
    }
}

impl Scoper for Regex {
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        let mut ranges = Vec::new();
        for cap in self.pattern.captures_iter(input) {
            match cap {
                Ok(cap) => {
                    let capture_context: HashMap<CaptureGroup, &str> = self
                        .captures
                        .iter()
                        .filter_map(|cg| {
                            match cg {
                                CaptureGroup::Named(name) => cap.name(name.as_str()),
                                CaptureGroup::Numbered(number) => cap.get(*number),
                            }
                            .map(|r#match| (cg.clone(), r#match.as_str()))
                        })
                        .collect();

                    ranges.push((
                        cap.get(0)
                            .expect("index 0 guaranteed to contain whole match")
                            .range(),
                        Some(ScopeContext::CaptureGroups(capture_context)),
                    ));
                }
                // Let's blow up on purpose instead of silently continuing; any of
                // these errors a user will likely want to know about, as they
                // indicate serious failure.
                Err(fancy_regex::Error::RuntimeError(e)) => {
                    panic!("regex exceeded runtime limits: {e}")
                }
                Err(fancy_regex::Error::ParseError(_, _) | fancy_regex::Error::CompileError(_)) => {
                    unreachable!("pattern was compiled successfully before")
                }
                Err(_) => {
                    unreachable!("implementation detail of fancy-regex")
                }
            }
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow::Borrowed as B;

    use rstest::rstest;

    use super::*;
    use crate::scope::Scope::{In, Out};
    use crate::scope::{RWScope, RWScopes};
    use crate::view::{ScopedView, ScopedViewBuilder};

    /// Get 'Capture Group 0', the default which is always present.
    #[allow(clippy::unnecessary_wraps)]
    fn cg0(string: &str) -> Option<ScopeContext<'_>> {
        Some(ScopeContext::CaptureGroups(HashMap::from([(
            CaptureGroup::Numbered(0),
            string,
        )])))
    }

    /// Get naively numbered capture groups.
    #[allow(clippy::unnecessary_wraps)]
    fn cgs<'a>(strings: &[&'a str]) -> Option<ScopeContext<'a>> {
        let mut cgs = HashMap::new();

        for (i, string) in strings.iter().enumerate() {
            cgs.insert(CaptureGroup::Numbered(i), *string);
        }

        Some(ScopeContext::CaptureGroups(cgs))
    }

    #[rstest]
    #[case(
        "a",
        "a",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    #[case(
        "aa",
        "a",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    #[case(
        "aba",
        "a",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
                RWScope(Out("b")),
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    //
    #[case(
        "a",
        "",
        ScopedView::new(RWScopes(
            vec![
                RWScope(Out("a")),
            ])
        )
    )]
    #[case(
        "",
        "a",
        ScopedView::new(RWScopes(
            // Empty results are discarded
            vec![
            ])
        )
    )]
    //
    #[case(
        "a",
        "a",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    #[case(
        "a",
        "b",
        ScopedView::new(RWScopes(
            vec![
                RWScope(Out("a")),
            ])
        )
    )]
    //
    #[case(
        "a",
        ".*",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    #[case(
        "a",
        ".+?",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    //
    #[case(
        "a\na",
        ".*",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
                RWScope(Out("\n")),
                RWScope(In(B("a"), cg0("a"))),
            ])
        )
    )]
    #[case(
        "a\na",
        "(?s).*",
        ScopedView::new(RWScopes(
            vec![
                // Dot matches newline
                RWScope(In(B("a\na"), cg0("a\na"))),
            ])
        )
    )]
    //
    #[case(
        "abc",
        "a",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
                RWScope(Out("bc")),
            ])
        )
    )]
    //
    #[case(
        "abc",
        r"\w",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("a"), cg0("a"))),
                RWScope(In(B("b"), cg0("b"))),
                RWScope(In(B("c"), cg0("c"))),
            ])
        )
    )]
    #[case(
        "abc",
        r"\W",
        ScopedView::new(RWScopes(
            vec![
                RWScope(Out("abc")),
            ])
        )
    )]
    #[case(
        "abc",
        r"\w+",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("abc"), cg0("abc"))),
            ])
        )
    )]
    //
    #[case(
        "Work 69 on 420 words",
        r"\w+",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("Work"), cg0("Work"))),
                RWScope(Out(" ")),
                RWScope(In(B("69"), cg0("69"))),
                RWScope(Out(" ")),
                RWScope(In(B("on"), cg0("on"))),
                RWScope(Out(" ")),
                RWScope(In(B("420"), cg0("420"))),
                RWScope(Out(" ")),
                RWScope(In(B("words"), cg0("words"))),
            ])
        )
    )]
    #[case(
        "Ignore 69 the 420 digits",
        r"\p{letter}+",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("Ignore"), cg0("Ignore"))),
                RWScope(Out(" 69 ")),
                RWScope(In(B("the"), cg0("the"))),
                RWScope(Out(" 420 ")),
                RWScope(In(B("digits"), cg0("digits"))),
            ])
        )
    )]
    //
    #[case(
        ".",
        ".",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("."), cg0("."))),
            ])
        )
    )]
    #[case(
        r"\.",
        ".",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B(r"\"), cg0(r"\"))),
                RWScope(In(B("."), cg0("."))),
            ])
        )
    )]
    #[case(
        r".",
        r"\.",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B("."), cg0("."))),
            ])
        )
    )]
    #[case(
        r"\.",
        r"\.",
        ScopedView::new(RWScopes(
            vec![
                RWScope(Out(r"\")),
                RWScope(In(B("."), cg0("."))),
            ])
        )
    )]
    #[case(
        r"\w",
        r"\w",
        ScopedView::new(RWScopes(
            vec![
                RWScope(Out(r"\")),
                RWScope(In(B("w"), cg0("w"))),
            ])
        )
    )]
    //
    // Capture groups
    #[case(
        r"Hello",
        r"\w+",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B(r"Hello"), cg0("Hello"))),
            ])
        )
    )]
    #[case(
        r"Hello",
        r"(\w+)",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B(r"Hello"), cgs(&["Hello", "Hello"]))),
            ]
        ))
    )]
    #[case(
        r"Hello World",
        r"Hello (\w+)",
        ScopedView::new(RWScopes(
            vec![
                RWScope(In(B(r"Hello World"), cgs(&["Hello World", "World"]))),
            ]
        ))
    )]
    #[case(
        // https://github.com/alexpovel/srgn/issues/71; this used to panic
        r#""error"; "x" => %x, "y" => %y"#,
        r"(?P<msg>.+);(?P<structure>.+)",
        ScopedView::new(RWScopes(
            vec![
                RWScope(
                    In(B(r#""error"; "x" => %x, "y" => %y"#),
                    Some(
                        ScopeContext::CaptureGroups(HashMap::from([
                            (
                                CaptureGroup::Numbered(0),
                                r#""error"; "x" => %x, "y" => %y"#,
                            ),
                            (
                                CaptureGroup::Named("msg".into()),
                                r#""error""#,
                            ),
                            (
                                CaptureGroup::Named("structure".into()),
                                r#" "x" => %x, "y" => %y"#,
                            ),
                        ]))
                    ))
                ),
            ]
        ))
    )]
    fn test_regex_scoping(
        #[case] input: &str,
        #[case] pattern: &str,
        #[case] expected: ScopedView<'_>,
    ) {
        let mut builder = ScopedViewBuilder::new(input);
        let regex = Regex::new(RegexPattern::new(pattern).unwrap());
        builder.explode(&regex);
        let actual = builder.build();

        assert_eq!(actual, expected);
    }

    mod fuzzyish {
        use std::time::{Duration, Instant};

        use log::info;
        use rand;
        use rand::seq::SliceRandom;
        use rand::Rng;

        use super::*;
        use crate::scope::ROScope;

        fn generate_random_regex(mut rng: &mut rand::rngs::ThreadRng) -> Option<RegexPattern> {
            let atoms: [&str; 7] = [".", "\\d", "\\D", "\\w", "\\W", "\\s", "\\S"];
            let quantifiers: [&str; 5] = ["*", "+", "?", "{2,5}", "{3}"];
            let others: [&str; 3] = ["|", "^", "$"];
            let letters: [&str; 26] = [
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            ];

            let mut regex = String::new();

            for _ in 0..rng.gen_range(1..=2) {
                if rng.gen_bool(0.3) {
                    regex.push_str(atoms.choose(&mut rng).unwrap());
                }

                if rng.gen_bool(0.6) {
                    let letter = letters.choose(&mut rng).unwrap();
                    if rng.gen_bool(0.5) {
                        let uc = letter.to_uppercase();
                        regex.push_str(uc.as_str());
                    } else {
                        regex.push_str(letter);
                    }
                }

                if rng.gen_bool(0.3) {
                    regex.push_str(quantifiers.choose(&mut rng).unwrap());
                }

                if rng.gen_bool(0.1) {
                    regex.push_str(others.choose(&mut rng).unwrap());
                }
            }

            RegexPattern::new(regex.as_str()).ok()
        }

        /// Run fuzz-like testing.
        ///
        /// This is much like fuzzing, but a bit more manually controlled and part of
        /// the core test harness, hence running always. Property testing like
        /// `proptest` would be much better ("given some input in this shape, and some
        /// regex, test the property that reassembly works"), but setup for that crate
        /// is substantial. The below approach is 'good enough' and emulates property
        /// testing a fair bit. We just need some random inputs and some short-ish but
        /// random regex to split by (generating random, valid regex is... interesting).
        ///
        /// Run for a duration instead of a fixed number of tries, as we would have to
        /// choose that fixed number rather low for CI to not be too slow. That would
        /// waste potential when running locally.
        #[test]
        fn test_regex_scoping_randomly() {
            let mut n_tries = 0;
            let mut n_matches = 0;

            let duration = if std::env::var("CI").is_ok() {
                Duration::from_secs(5)
            } else {
                // SORRY if this crashed the test on your machine. Flaky one :(
                Duration::from_millis(500)
            };

            let mut rng = rand::thread_rng();

            // "Anything but 'other'", see also:
            // https://docs.rs/regex/latest/regex/#matching-one-character
            // https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values
            let pattern = r"\P{other}+";
            let generated = rand_regex::Regex::compile(pattern, 100).unwrap();

            let now = Instant::now();

            loop {
                n_tries += 1;

                let Some(regex) = generate_random_regex(&mut rng) else {
                    continue;
                };
                let scope = Regex::new(regex);
                let input: String = rng.sample(&generated);

                let scopes = scope.scope(&input);

                if scopes.0.iter().any(|s| match s {
                    ROScope(In(_, _)) => true,
                    ROScope(Out(_)) => false,
                }) {
                    n_matches += 1;
                }

                let mut reassembled = String::new();
                for scope in scopes.0 {
                    reassembled.push_str((&scope).into());
                }

                assert_eq!(input, reassembled);

                if now.elapsed() > duration {
                    break;
                }
            }

            info!(
                // To test anything, we actually need matches so splits happen.
                "Processed {} inputs, of which {} were matched and successfully reassembled",
                n_tries, n_matches
            );

            assert!(
                n_matches >= n_tries / 20,
                "Too few regex matches; try lowering regex length"
            );

            assert!(
                n_tries > 100,
                // Might happen in CI, but we should ensure a certain lower bound;
                // locally, many more tests can run.
                "Too few tries; is the host machine very slow?"
            );
        }
    }
}
