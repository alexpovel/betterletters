use log::trace;

use super::scope::RangesWithContext;
use crate::literal::Literal;
use crate::{ROScopes, Scoper};
#[cfg(doc)]
use crate::{
    actions::Deletion,
    scope::Scope::{In, Out},
};

/// Fixes `\r` being [`In`] scope while `\n` is actually [`Out`].
///
/// This was observed to happen with
/// [`tree-sitter-python`](https://github.com/tree-sitter/tree-sitter-python). For an
/// input of, say:
///
/// ```python
/// x = 3  # the value␍␊
/// ```
///
/// its `comment` type will *eat into `\r`*, and scope it approximately as:
///
/// ```text
/// [x = 3  ](# the value␍)[␊]
/// ```
///
/// where `(...)` means [`In`] scope, and `[...]` [`Out`]. Combined with an action such
/// as [`Deletion`], this will rip line endings apart, and worst of all end up in
/// *mixed* line endings for the resulting document (e.g., if everything in scope is
/// deleted).
///
/// This [`Scoper`] ensures `\r` is [`Out`] of scope (note: it cannot, by itself, decide
/// if that's actually the correct behavior). It is much cheaper done here than fixing
/// *every* upstream scope (language grammar).
#[derive(Debug, Clone, Copy)]
pub struct DosFix;

impl Scoper for DosFix {
    fn scope<'viewee>(&self, input: &'viewee str) -> ROScopes<'viewee> {
        ROScopes::from_raw_ranges(input, self.scope_raw(input)).invert()
    }

    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        trace!(
            "Applying DOS-style line endings fix on '{}'",
            input.escape_debug()
        );
        let literal = Literal::try_from("\r".to_string()).unwrap();
        let scopes = literal.scope_raw(input);

        trace!(
            "Scopes after applying DOS-style line endings fix: {:?}",
            scopes
        );
        scopes
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
    #[case("a", ScopedView::new(RWScopes(vec![RWScope(In(Borrowed("a"), None))])))]
    #[case("a\n", ScopedView::new(RWScopes(vec![RWScope(In(Borrowed("a\n"), None))])))]
    //
    #[case("\r", ScopedView::new(RWScopes(vec![RWScope(Out("\r"))])))]
    #[case("a\r", ScopedView::new(RWScopes(vec![RWScope(In(Borrowed("a"), None)), RWScope(Out("\r"))])))]
    #[case("a\r\n", ScopedView::new(RWScopes(vec![RWScope(In(Borrowed("a"), None)), RWScope(Out("\r")), RWScope(In(Borrowed("\n"), None))])))]
    fn test_dos_fix(#[case] input: &str, #[case] expected: ScopedView<'_>) {
        let mut builder = crate::view::ScopedViewBuilder::new(input);
        let dosfix = DosFix;
        builder.explode(&dosfix);
        let view = builder.build();

        assert_eq!(view, expected);
    }
}
