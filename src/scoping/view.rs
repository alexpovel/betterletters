use std::borrow::Cow;
use std::fmt;
use std::ops::Range;

use itertools::Itertools;
use log::{debug, trace, warn};

use crate::actions::{self, Action, ActionError};
use crate::scoping::dosfix::DosFix;
use crate::scoping::scope::Scope::{In, Out};
#[cfg(doc)]
use crate::scoping::scope::ScopeContext;
use crate::scoping::scope::{ROScope, ROScopes, RWScope, RWScopes};
use crate::scoping::Scoper;

/// A view of some input, sorted into parts, which are either [`In`] or [`Out`] of scope
/// for processing.
///
/// The view is **writable**. It can be manipulated by
/// [mapping][`Self::map_without_context`] [`Action`]s over it.
///
/// The main avenue for constructing a view is [`Self::builder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedView<'viewee> {
    scopes: RWScopes<'viewee>,
}

/// Core implementations.
impl<'viewee> ScopedView<'viewee> {
    /// Create a new view from the given scopes.
    #[must_use]
    pub const fn new(scopes: RWScopes<'viewee>) -> Self {
        Self { scopes }
    }

    /// Access the scopes contained in this view.
    #[must_use]
    pub const fn scopes(&self) -> &RWScopes<'viewee> {
        &self.scopes
    }

    /// docs...
    #[must_use]
    pub fn scopes_with_ranges(&self) -> Vec<(Range<usize>, &RWScope<'viewee>)> {
        let mut res = Vec::with_capacity(self.scopes.0.len());
        let mut start = 0;

        for scope in &self.scopes().0 {
            let s: &str = scope.into();
            let end = start + s.len();
            res.push((start..end, scope));
            start = end;
        }

        res
    }

    /// Return a builder for a view of the given input.
    ///
    /// For API discoverability.
    #[must_use]
    pub fn builder(input: &'viewee str) -> ScopedViewBuilder<'viewee> {
        ScopedViewBuilder::new(input)
    }

    /// Apply an `action` to all [`In`] scope items contained in this view.
    ///
    /// They are **replaced** with whatever the action returns for the particular scope.
    /// This method is infallible, as it does not access any [`ScopeContext`].
    ///
    /// See implementors of [`Action`] for available types.
    #[allow(clippy::missing_panics_doc)] // 🤞
    pub fn map_without_context(&mut self, action: &impl Action) -> &mut Self {
        self.map_impl(action, false)
            .expect("not accessing context, so is infallible");

        self
    }

    /// Same as [`Self::map_without_context`], but will access any [`ScopeContext`],
    /// which is fallible.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`Err`] variant for when this method errors.
    pub fn map_with_context(&mut self, action: &impl Action) -> Result<&mut Self, ActionError> {
        self.map_impl(action, true)?;

        Ok(self)
    }

    fn map_impl(
        &mut self,
        action: &impl Action,
        use_context: bool,
    ) -> Result<&mut Self, ActionError> {
        for scope in &mut self.scopes.0 {
            match scope {
                RWScope(In(s, ctx)) => {
                    debug!("Mapping with context: {:?}", ctx);
                    let res = match (&ctx, use_context) {
                        (Some(c), true) => action.act_with_context(s, c)?,
                        _ => action.act(s),
                    };
                    debug!(
                        "Replacing '{}' with '{}'",
                        s.escape_debug(),
                        res.escape_debug()
                    );
                    *scope = RWScope(In(Cow::Owned(res), ctx.clone()));
                }
                RWScope(Out(s)) => {
                    debug!("Appending '{}'", s.escape_debug());
                }
            }
        }

        Ok(self)
    }

    /// Squeeze all consecutive [`In`] scopes into a single occurrence (the first one).
    pub fn squeeze(&mut self) -> &mut Self {
        debug!("Squeezing view by collapsing all consecutive in-scope occurrences.");

        let mut prev_was_in = false;
        self.scopes.0.retain(|scope| {
            let keep = !(prev_was_in && matches!(scope, RWScope(In { .. })));
            prev_was_in = matches!(scope, RWScope(In { .. }));
            trace!("keep: {}, scope: {:?}", keep, scope);
            keep
        });

        debug!("Squeezed: {:?}", self.scopes);

        self
    }

    /// Check whether anything is [`In`] scope for this view.
    #[must_use]
    pub fn has_any_in_scope(&self) -> bool {
        self.scopes.0.iter().any(|s| match s {
            RWScope(In { .. }) => true,
            RWScope(Out { .. }) => false,
        })
    }

    /// Split this item at newlines, into multiple [`ScopedView`]s.
    ///
    /// Scopes are retained, and broken across lines as needed.
    #[must_use]
    pub fn lines(&self) -> ScopedViewLines<'_> {
        let mut lines = Vec::new();
        let mut curr = Vec::new();

        for parent_scope in &self.scopes.0 {
            let s: &str = parent_scope.into();

            for potential_line in s.split_inclusive('\n') {
                // Is it supposed to be in or out of scope?
                let child_scope = match &parent_scope.0 {
                    In(_, ctx) => In(Cow::Borrowed(potential_line), ctx.clone()),
                    Out(_) => Out(potential_line),
                };

                // String might not have *any* newlines, so this isn't redundant
                let seen_newline = potential_line.ends_with('\n');

                curr.push(RWScope(child_scope));

                if seen_newline {
                    // Flush out
                    lines.push(RWScopes(curr));
                    curr = Vec::new();
                }
            }
        }

        if !curr.is_empty() {
            // Tail that wasn't flushed yet
            lines.push(RWScopes(curr));
        }

        ScopedViewLines(lines.into_iter().map(ScopedView::new).collect_vec())
    }
}

/// A view over a [`ScopedView`], split by its individual lines. Each line is its own
/// [`ScopedView`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedViewLines<'viewee>(Vec<ScopedView<'viewee>>);

impl<'viewee> IntoIterator for ScopedViewLines<'viewee> {
    type Item = ScopedView<'viewee>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Implementations of all available actions as dedicated methods.
///
/// Where actions don't take arguments, neither do the methods.
impl ScopedView<'_> {
    /// Apply the default [`actions::Deletion`] action to this view (see
    /// [`Self::map_without_context`]).
    pub fn delete(&mut self) -> &mut Self {
        let action = actions::Deletion::default();

        self.map_without_context(&action)
    }

    /// Apply the default [`actions::German`] action to this view (see
    /// [`Self::map_without_context`]).
    #[cfg(feature = "german")]
    pub fn german(&mut self) -> &mut Self {
        let action = actions::German::default();

        self.map_without_context(&action)
    }

    /// Apply the default [`actions::Lower`] action to this view (see
    /// [`Self::map_without_context`]).
    pub fn lower(&mut self) -> &mut Self {
        let action = actions::Lower::default();

        self.map_without_context(&action)
    }

    /// Apply the default [`actions::Normalization`] action to this view (see
    /// [`Self::map_without_context`]).
    pub fn normalize(&mut self) -> &mut Self {
        let action = actions::Normalization::default();

        self.map_without_context(&action)
    }

    /// Apply the [`actions::Replacement`] action to this view (see
    /// [`Self::map_with_context`]).
    ///
    /// ## Errors
    ///
    /// For why and how this can fail, see the implementation of [`TryFrom<String>`] for
    /// [`actions::Replacement`].
    pub fn replace(&mut self, replacement: String) -> Result<&mut Self, ActionError> {
        let action = actions::Replacement::try_from(replacement)?;

        self.map_with_context(&action)
    }

    /// Apply the [`actions::Symbols`] action to this view (see
    /// [`Self::map_without_context`]).
    #[cfg(feature = "symbols")]
    pub fn symbols(&mut self) -> &mut Self {
        let action = actions::Symbols::default();

        self.map_without_context(&action)
    }

    /// Apply the [`actions::SymbolsInversion`] action to this view (see
    /// [`Self::map_without_context`]).
    #[cfg(feature = "symbols")]
    pub fn invert_symbols(&mut self) -> &mut Self {
        let action = actions::SymbolsInversion::default();

        self.map_without_context(&action)
    }

    /// Apply the default [`actions::Titlecase`] action to this view (see
    /// [`Self::map_without_context`]).
    pub fn titlecase(&mut self) -> &mut Self {
        let action = actions::Titlecase::default();

        self.map_without_context(&action)
    }

    /// Apply the default [`actions::Upper`] action to this view (see
    /// [`Self::map_without_context`]).
    pub fn upper(&mut self) -> &mut Self {
        let action = actions::Upper::default();

        self.map_without_context(&action)
    }
}

impl fmt::Display for ScopedView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for scope in &self.scopes.0 {
            let s: &str = scope.into();
            write!(f, "{s}")?;
        }
        Ok(())
    }
}

/// A builder for [`ScopedView`]. Chain [`Self::explode`] to build up the view, then
/// finally call [`Self::build`].
///
/// Note: while building, the view is **read-only**: no manipulation of the contents is
/// possible yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedViewBuilder<'viewee> {
    scopes: ROScopes<'viewee>,
    viewee: &'viewee str,
}

/// Core implementations.
impl<'viewee> ScopedViewBuilder<'viewee> {
    /// Create a new builder from the given input.
    ///
    /// Initially, the entire `input` is [`In`] scope.
    #[must_use]
    pub fn new(input: &'viewee str) -> Self {
        Self {
            scopes: ROScopes(vec![ROScope(In(input, None))]),
            viewee: input,
        }
    }

    /// Build the view.
    ///
    /// This makes the view writable.
    #[must_use]
    pub fn build(mut self) -> ScopedView<'viewee> {
        self.apply_dos_line_endings_fix();

        ScopedView {
            scopes: RWScopes(self.scopes.0.into_iter().map(Into::into).collect()),
        }
    }

    /// See [`DosFix`].
    fn apply_dos_line_endings_fix(&mut self) {
        if self.scopes.0.windows(2).any(|window| match window {
            [ROScope(In(left, ..)), ROScope(Out(right))] => {
                left.ends_with('\r') && right.starts_with('\n')
            }
            _ => false,
        }) {
            warn!("Split CRLF detected. Likely scoper bug. Auto-fixing (globally).");
            // One issue with this: it's fixing *everything*, not just the location
            // where the split was detected. Implementing it differently is less
            // performant and more complex, and hitting a case where this distinction
            // (fixing globally vs. fixing locally) matters is quite unlikely.
            self.explode(&DosFix);
        }
    }

    /// Using a `scoper`, iterate over all scopes currently contained in this view under
    /// construction, apply the scoper to all [`In`] scopes, and **replace** each with
    /// whatever the scoper returned for the particular scope. These are *multiple*
    /// entries (hence 'exploding' this view: after application, it will likely be
    /// longer).
    ///
    /// Note this necessarily means a view can only be *narrowed*. What was previously
    /// [`In`] scope can be:
    ///
    /// - either still fully [`In`] scope,
    /// - or partially [`In`] scope, partially [`Out`] of scope
    ///
    /// after application. Anything [`Out`] out of scope can never be brought back.
    ///
    /// ## Panics
    ///
    /// Panics if the [`Scoper`] scopes such that the view is no longer consistent, i.e.
    /// gaps were created and the original input can no longer be reconstructed from the
    /// new view. This would be an internal bug.
    pub fn explode(&mut self, scoper: &impl Scoper) -> &mut Self {
        trace!("Exploding scopes: {:?}", self.scopes);
        let mut new = Vec::with_capacity(self.scopes.0.len());
        for scope in self.scopes.0.drain(..) {
            trace!("Exploding scope: {:?}", scope);

            if scope.is_empty() {
                trace!("Skipping empty scope");
                continue;
            }

            match scope {
                ROScope(In(s, ..)) => {
                    let mut new_scopes = scoper.scope(s);
                    new_scopes.0.retain(|s| !s.is_empty());
                    new.extend(new_scopes.0);
                }
                // Be explicit about the `Out(_)` case, so changing the enum is a
                // compile error
                ROScope(Out("")) => {}
                out @ ROScope(Out(_)) => new.push(out),
            }

            trace!("Exploded scope, new scopes are: {:?}", new);
        }
        trace!("Done exploding scopes.");

        self.scopes.0 = new;

        assert_eq!(
            // Tried to do this 'more proper' using the `contracts` crate, but this
            // method `mut`ably borrows `self` and returns it as such, which is
            // worst-case and didn't play well with its macros. The crate doesn't do
            // much more than this manual `assert` anyway.
            self.scopes,
            self.viewee,
            "Post-condition violated: exploding scopes resulted in inconsistent view. \
            Aborting, as this is an unrecoverable bug in a scoper. \
            Please report at {}.",
            env!("CARGO_PKG_REPOSITORY")
        );

        self
    }
}

impl<'viewee> IntoIterator for ScopedViewBuilder<'viewee> {
    type Item = ROScope<'viewee>;

    type IntoIter = std::vec::IntoIter<ROScope<'viewee>>;

    fn into_iter(self) -> Self::IntoIter {
        self.scopes.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::rstest;

    use super::ScopedView;
    use crate::scoping::scope::RWScopes;
    use crate::scoping::scope::Scope::{self, In, Out};
    use crate::scoping::view::ScopedViewBuilder;
    use crate::RegexPattern;

    #[rstest]
    // Pattern only
    #[case("a", "a", "a")]
    #[case("aa", "a", "a")]
    #[case("aaa", "a", "a")]
    //
    // Pattern once; nothing to squeeze
    #[case("aba", "a", "aba")]
    #[case("bab", "a", "bab")]
    #[case("babab", "a", "babab")]
    #[case("ababa", "a", "ababa")]
    //
    // Squeezes only the pattern, no other repetitions
    #[case("aaabbb", "a", "abbb")]
    //
    // Squeezes start
    #[case("aab", "a", "ab")]
    //
    // Squeezes middle
    #[case("baab", "a", "bab")]
    //
    // Squeezes end
    #[case("abaa", "a", "aba")]
    //
    // Squeezes as soon as pattern occurs at least twice
    #[case("a", "ab", "a")]
    #[case("ab", "ab", "ab")]
    #[case("aba", "ab", "aba")]
    #[case("abab", "ab", "ab")]
    #[case("ababa", "ab", "aba")]
    #[case("ababab", "ab", "ab")]
    //
    // Squeezes nothing if pattern not present
    #[case("", "b", "")]
    #[case("a", "b", "a")]
    #[case("aa", "b", "aa")]
    #[case("aaa", "b", "aaa")]
    //
    // Deals with character classes (space)
    #[case("Hello World", r"\s", "Hello World")]
    #[case("Hello  World", r"\s", "Hello World")]
    #[case("Hello       World", r"\s", "Hello World")]
    #[case("Hello\tWorld", r"\t", "Hello\tWorld")]
    #[case("Hello\t\tWorld", r"\t", "Hello\tWorld")]
    //
    // Deals with character classes (inverted space)
    #[case("Hello World", r"\S", "H W")]
    #[case("Hello\t\tWorld", r"\S", "H\t\tW")]
    //
    // Deals with overlapping matches; behavior of `regex` crate
    #[case("abab", r"aba", "abab")]
    #[case("ababa", r"aba", "ababa")]
    #[case("ababab", r"aba", "ababab")]
    #[case("abababa", r"aba", "abababa")]
    #[case("aba", r"aba", "aba")]
    #[case("abaaba", r"aba", "aba")]
    //
    // Requires non-greedy matches for meaningful results
    #[case("ab", r"\s+?", "ab")]
    #[case("a b", r"\s+?", "a b")]
    #[case("a\t\tb", r"\s+?", "a\tb")]
    #[case("a\t\t  b", r"\s+?", "a\tb")]
    //
    // Deals with more complex patterns
    #[case("ab", "", "ab")] // Matches nothing
    //
    #[case("ab", r"[ab]", "a")]
    #[case("ab", r"[ab]+", "ab")]
    #[case("ab", r"[ab]+?", "a")]
    //
    #[case("abab", r"\D", "a")]
    //
    // Builds up properly; need non-capturing group
    #[case("abab", r"(?:ab){2}", "abab")]
    #[case("ababa", r"(?:ab){2}", "ababa")]
    #[case("ababab", r"(?:ab){2}", "ababab")]
    #[case("abababa", r"(?:ab){2}", "abababa")]
    #[case("abababab", r"(?:ab){2}", "abab")]
    #[case("ababababa", r"(?:ab){2}", "ababa")]
    #[case("ababababab", r"(?:ab){2}", "ababab")]
    #[case("abababababab", r"(?:ab){2}", "abab")]
    //
    #[case("Anything whatsoever gets rEkT", r".", "A")]
    #[case(
        "Anything whatsoever gets rEkT",
        r".*", // Greediness inverted
        "Anything whatsoever gets rEkT"
    )]
    //
    // Deals with Unicode shenanigans
    #[case("😎😎", r"😎", "😎")]
    #[case("\0😎\0😎\0", r"😎", "\0😎\0😎\0")]
    //
    #[case("你你好", r"你", "你好")]
    //
    // Longer ("integration") tests; things that come up in the wild
    #[case(
        " dirty Strings  \t with  \t\t messed up  whitespace\n\n\n",
        r"\s",
        " dirty Strings with messed up whitespace\n"
    )]
    #[case(
        " dirty Strings  \t with  \t\t messed up  whitespace\n\n\n",
        r" ",
        " dirty Strings \t with \t\t messed up whitespace\n\n\n"
    )]
    fn test_squeeze(#[case] input: &str, #[case] pattern: RegexPattern, #[case] expected: &str) {
        let mut builder = ScopedViewBuilder::new(input);
        builder.explode(&crate::scoping::regex::Regex::new(pattern));
        let mut view = builder.build();

        view.squeeze();
        let result = view.to_string();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        // New newline at all: still works
        vec![
            In("Hello", None),
        ],
        vec![
            vec![
                In("Hello", None),
            ],
        ],
    )]
    #[case(
        // Single newline is fine
        vec![
            In("Hello\n", None),
        ],
        vec![
            vec![
                In("Hello\n", None),
            ],
        ],
    )]
    #[case(
        // Single scope is broken up across lines properly (multiple lines in single
        // scope)
        vec![
            In("Hello\nWorld", None),
        ],
        vec![
            vec![
                In("Hello\n", None),
            ],
            vec![
                In("World", None),
            ],
        ],
    )]
    #[case(
        // Single line across multiple scopes
        vec![
            In("Hello", None),
            Out("World"),
            In("!!\n", None),
            Out(" Goodbye"),
        ],
        vec![
            vec![
                In("Hello", None),
                Out("World"),
                In("!!\n", None),
            ],
            vec![
                Out(" Goodbye"),
            ],
        ],
    )]
    #[case(
        // Mixed scopes & trailing newline works
        vec![
            In("Hello\n", None),
            Out("World"),
        ],
        vec![
            vec![
                In("Hello\n", None),
            ],
            vec![
                Out("World"),
            ],
        ],
    )]
    #[case(
        // Mixed scopes & leading newline works
        vec![
            In("Hello", None),
            Out("\nWorld"),
        ],
        vec![
            vec![
                In("Hello", None),
                Out("\n"),
            ],
            vec![
                Out("World"),
            ],
        ],
    )]
    #[case(
        // Empty lines & empty scopes works
        vec![
            In("\n", None),
            Out("\n"),
            Out(""),
            In("World\n", None),
            Out(""),
        ],
        vec![
            vec![
                In("\n", None),
            ],
            vec![
                Out("\n"),
            ],
            vec![
                #[cfg(not(test))]Out(""), // Dropped! (do not compile)
                In("World\n", None),
            ],
            #[cfg(not(test))] // Dropped! (do not compile)
            vec![
                Out(""),
            ],
        ],
    )]
    fn test_lines(
        #[case] input: Vec<Scope<'_, &str>>,
        #[case] expected: Vec<Vec<Scope<'_, &str>>>,
    ) {
        let view = ScopedView {
            scopes: input.into(),
        };
        let result = view.lines().into_iter().collect_vec();
        let expected = expected
            .into_iter()
            .map(RWScopes::from)
            .map(ScopedView::new)
            .collect_vec();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "hello",
        &["hello"] // aka: it loops at least once!
    )]
    #[case(
        "hello\n",
        &["hello\n"]
    )]
    #[case(
        "hello\nworld",
        &["hello\n", "world"]
    )]
    #[case(
        "hello\nworld\n",
        &["hello\n", "world\n"]
    )]
    #[case(
        "",
        &[] // ⚠️ no iteration happens; empty string is dropped
    )]
    fn test_split_inclusive(#[case] input: &str, #[case] expected: &[&str]) {
        // This is not a useful unit test; it's just encoding and confirming the
        // behavior of `split_inclusive`, to convince myself.
        for (si, exp) in input.split_inclusive('\n').zip_eq(expected) {
            assert_eq!(si, *exp);
        }
    }
}
