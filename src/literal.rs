use std::error::Error;
use std::fmt;
use std::ops::Range;

use log::trace;
use unescape::unescape;

use super::scope::RangesWithContext;
use super::Scoper;
use crate::ranges::Ranges;

/// A literal string for querying.
#[derive(Debug)]
pub struct Literal(String);

/// An error that can occur when parsing a literal.
#[derive(Debug)]
pub enum LiteralError {
    /// The literal contains invalid escape sequences.
    InvalidEscapeSequences(String),
}

impl fmt::Display for LiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEscapeSequences(literal) => {
                write!(f, "Contains invalid escape sequences: '{literal}'")
            }
        }
    }
}

impl Error for LiteralError {}

impl TryFrom<String> for Literal {
    type Error = LiteralError;

    fn try_from(literal: String) -> Result<Self, Self::Error> {
        let unescaped = unescape(&literal)
            .ok_or_else(|| LiteralError::InvalidEscapeSequences(literal.to_string()))?;

        Ok(Self(unescaped))
    }
}

impl Scoper for Literal {
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        let ranges = {
            let len = self.0.len();

            let ranges: Ranges<usize> = input
                .match_indices(&self.0)
                .map(|(i, _)| Range {
                    start: i,
                    end: i + len,
                })
                .collect();

            trace!("Ranges in scope for {:?}: {:?}", self, ranges);

            ranges
        };

        ranges.into()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow::Borrowed;

    use rstest::rstest;

    use super::*;
    use crate::scope::Scope::{In, Out};
    use crate::scope::{RWScope, RWScopes};
    use crate::view::ScopedView;

    #[rstest]
    #[case(
        "a",
        "a",
        ScopedView::new(
            RWScopes(vec![
                RWScope(In(Borrowed("a"), None)),
            ])
        )
    )]
    #[case(
        "aa",
        "a",
        ScopedView::new(
            RWScopes(vec![
                RWScope(In(Borrowed("a"), None)),
                RWScope(In(Borrowed("a"), None)),
            ])
        )
    )]
    #[case(
        "aba",
        "a",
        ScopedView::new(
            RWScopes(vec![
                RWScope(In(Borrowed("a"), None)),
                RWScope(Out("b")),
                RWScope(In(Borrowed("a"), None)),
            ])
        )
    )]
    //
    #[case(
        ".",
        ".",
        ScopedView::new(
            RWScopes(vec![
                RWScope(In(Borrowed("."), None)),
            ])
        )
    )]
    #[case(
        r"\.",
        ".",
        ScopedView::new(
            RWScopes(vec![
                RWScope(Out(r"\")),
                RWScope(In(Borrowed("."), None)),
            ])
        )
    )]
    #[case(
        r".",
        r"\\.",
        ScopedView::new(
            RWScopes(vec![
                RWScope(Out(r".")),
            ])
        )
    )]
    //
    #[case(
        "Hello\nWorld\n",
        "\n",
        ScopedView::new(
            RWScopes(vec![
                RWScope(Out("Hello")),
                RWScope(In(Borrowed("\n"), None)),
                RWScope(Out("World")),
                RWScope(In(Borrowed("\n"), None)),
            ])
        )
    )]
    fn test_literal_scoping(
        #[case] input: &str,
        #[case] literal: &str,
        #[case] expected: ScopedView<'_>,
    ) {
        let mut builder = crate::view::ScopedViewBuilder::new(input);
        let literal = Literal::try_from(literal.to_owned()).unwrap();
        builder.explode(&literal);
        let actual = builder.build();

        assert_eq!(actual, expected);
    }
}
