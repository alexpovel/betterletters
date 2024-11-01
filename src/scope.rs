use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;
use log::{debug, trace};

use super::regex::CaptureGroup;
use crate::ranges::Ranges;
use crate::scope::Scope::{In, Out};

/// Indicates whether a given string part is in scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope<'viewee, T> {
    /// The given part is in scope for processing.
    In(T, Option<ScopeContext<'viewee>>),
    /// The given part is out of scope for processing.
    ///
    /// Treated as immutable, view-only.
    Out(&'viewee str),
}

/// A read-only scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ROScope<'viewee>(pub Scope<'viewee, &'viewee str>);

/// Multiple read-only scopes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ROScopes<'viewee>(pub Vec<ROScope<'viewee>>);

/// A read-write scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RWScope<'viewee>(pub Scope<'viewee, Cow<'viewee, str>>);

#[cfg(test)] // For convenience; not legal in normal code
impl<'viewee> From<Scope<'viewee, &'viewee str>> for RWScope<'viewee> {
    fn from(value: Scope<'viewee, &'viewee str>) -> Self {
        Self(match value {
            In(s, ctx) => In(Cow::Borrowed(s), ctx),
            Out(s) => Out(s),
        })
    }
}

/// Multiple read-write scopes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RWScopes<'viewee>(pub Vec<RWScope<'viewee>>);

#[cfg(test)] // For convenience; not legal in normal code
impl<'viewee, I> From<I> for RWScopes<'viewee>
where
    I: IntoIterator<Item = Scope<'viewee, &'viewee str>>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().map(Into::into).collect_vec())
    }
}

impl ROScope<'_> {
    /// Check whether the scope is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        let s: &str = self.into();
        s.is_empty()
    }
}

/// Raw ranges, paired with optional context for content at that range.
pub type RangesWithContext<'viewee> = Vec<(Range<usize>, Option<ScopeContext<'viewee>>)>;

/// Converts, leaving unknown values [`Default`].
///
/// A convenience to support [`Ranges`] where there's no meaningful context to be
/// inserted for [`RangesWithContext`].
impl From<Ranges<usize>> for RangesWithContext<'_> {
    fn from(val: Ranges<usize>) -> Self {
        val.into_iter()
            .map(|range| (range, Option::default()))
            .collect()
    }
}

impl<'viewee> ROScopes<'viewee> {
    /// Construct a new instance from the given raw ranges.
    ///
    /// The passed `input` will be traversed according to `ranges`: all specified
    /// `ranges` are taken as [`In`] scope, everything not covered by a range is [`Out`]
    /// of scope.
    ///
    /// ## Panics
    ///
    /// Panics if the given `ranges` contain indices out-of-bounds for `input`.
    #[must_use]
    pub fn from_raw_ranges(input: &'viewee str, ranges: RangesWithContext<'viewee>) -> Self {
        trace!("Constructing scopes from raw ranges: {:?}", ranges);

        let mut scopes = Vec::with_capacity(ranges.len());

        let mut last_end = 0;
        for (Range { start, end }, context) in ranges.into_iter().sorted_by_key(|(r, _)| r.start) {
            let range = last_end..start;
            let out = &input[range.clone()];
            if !out.is_empty() {
                scopes.push(ROScope(Out(out)));
            }

            let range = start..end;
            let r#in = &input[range.clone()];
            if !r#in.is_empty() {
                scopes.push(ROScope(In(r#in, context)));
            }

            last_end = end;
        }

        let range = last_end..input.len();
        let tail = &input[range];
        if !tail.is_empty() {
            scopes.push(ROScope(Out(tail)));
        }

        debug!("Scopes: {:?}", scopes);

        ROScopes(scopes)
    }

    /// Inverts the scopes: what was previously [`In`] is now [`Out`], and vice versa.
    #[must_use]
    pub fn invert(self) -> Self {
        trace!("Inverting scopes: {:?}", self.0);
        let scopes = self
            .0
            .into_iter()
            .map(|s| match s {
                ROScope(In(s, ..)) => ROScope(Out(s)),
                ROScope(Out(s)) => ROScope(In(s, None)),
            })
            .collect();
        trace!("Inverted scopes: {:?}", scopes);

        Self(scopes)
    }
}

/// Checks for equality, regarding only raw [`str`] parts, i.e. disregards whether an
/// element is [`In`] or [`Out`] of scope.
impl PartialEq<&str> for ROScopes<'_> {
    fn eq(&self, other: &&str) -> bool {
        let mut start = 0;
        let mut end = None;

        for scope in &self.0 {
            let s: &str = scope.into();
            end = Some(start + s.len());

            let Some(substring) = other.get(start..end.unwrap()) else {
                return false;
            };

            if substring != s {
                return false;
            }

            start = end.unwrap();
        }

        end.map_or(other.is_empty(), |e| other.len() == e)
    }
}

impl PartialEq<ROScopes<'_>> for &str {
    fn eq(&self, other: &ROScopes<'_>) -> bool {
        other == self
    }
}

impl<'viewee> From<&'viewee ROScope<'viewee>> for &'viewee str {
    /// Get the underlying string slice.
    ///
    /// All variants contain such a slice, so this is a convenient method.
    fn from(s: &'viewee ROScope<'_>) -> Self {
        match s.0 {
            In(s, ..) | Out(s) => s,
        }
    }
}

impl<'viewee> From<ROScope<'viewee>> for RWScope<'viewee> {
    fn from(s: ROScope<'viewee>) -> Self {
        match s.0 {
            In(s, ctx) => RWScope(In(Cow::Borrowed(s), ctx)),
            Out(s) => RWScope(Out(s)),
        }
    }
}

impl<'viewee> From<&'viewee RWScope<'viewee>> for &'viewee str {
    /// Get the underlying string slice.
    ///
    /// All variants contain such a slice, so this is a convenient method.
    fn from(s: &'viewee RWScope<'_>) -> Self {
        match &s.0 {
            In(s, ..) => s,
            Out(s) => s,
        }
    }
}

/// Context accompanying a scope.
///
/// For example, a scope might have been created by a regular expression, in which case
/// capture groups might have matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeContext<'viewee> {
    /// Regular expression capture groups mapped to the content they matched.
    CaptureGroups(HashMap<CaptureGroup, &'viewee str>),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // Base cases
    #[case(ROScopes(vec![ROScope(In("abc", None))]), "abc", true)]
    #[case(ROScopes(vec![ROScope(In("cba", None))]), "cba", true)]
    #[case(ROScopes(vec![ROScope(In("🦀", None))]), "🦀", true)]
    #[case(ROScopes(vec![ROScope(In("🦀", None))]), "🤗", false)]
    //
    // Substring matching
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None))]), "ab", true)]
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None)), ROScope(In("c", None))]), "abc", true)]
    //
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None))]), "ac", false)]
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None))]), "a", false)]
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None))]), "b", false)]
    #[case(ROScopes(vec![ROScope(In("a", None)), ROScope(In("b", None)), ROScope(In("c", None))]), "acc", false)]
    //
    // Length mismatch
    #[case(ROScopes(vec![ROScope(In("abc", None))]), "abcd", false)]
    #[case(ROScopes(vec![ROScope(In("abcd", None))]), "abc", false)]
    //
    // Partial emptiness
    #[case(ROScopes(vec![ROScope(In("abc", None))]), "", false)]
    #[case(ROScopes(vec![ROScope(In("", None))]), "abc", false)]
    #[case(ROScopes(vec![ROScope(Out(""))]), "abc", false)]
    #[case(ROScopes(vec![ROScope(In("", None)), ROScope(Out(""))]), "abc", false)]
    //
    // Full emptiness
    #[case(ROScopes(vec![ROScope(In("", None))]), "", true)]
    #[case(ROScopes(vec![ROScope(Out(""))]), "", true)]
    #[case(ROScopes(vec![ROScope(In("", None)), ROScope(Out(""))]), "", true)]
    //
    // Types of scope doesn't matter
    #[case(ROScopes(vec![ROScope(In("a", None))]), "a", true)]
    #[case(ROScopes(vec![ROScope(Out("a"))]), "a", true)]
    fn test_scoped_view_str_equality(
        #[case] scopes: ROScopes<'_>,
        #[case] string: &str,
        #[case] equal: bool,
    ) {
        assert!((scopes == string) == equal);
    }
}
