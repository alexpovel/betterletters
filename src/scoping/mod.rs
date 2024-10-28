//! Items for defining the scope actions are applied within.

/// Fixes for DOS-style line endings.
pub mod dosfix;
/// Create scoped views using programming language grammar-aware types.
pub mod langs;
/// Create scoped views using string literals.
pub mod literal;
/// Create scoped views using regular expressions.
pub mod regex;
/// [`Scope`] and its various wrappers.
pub mod scope;
/// [`ScopedView`] and its related types.
pub mod view;

use scope::{ROScopes, RangesWithContext};
#[cfg(doc)]
use {scope::Scope, view::ScopedView};

/// An item capable of scoping down a given input into individual scopes.
pub trait Scoper: Send + Sync {
    /// Scope the given `input`.
    ///
    /// After application, the returned scopes are a collection of either in-scope or
    /// out-of-scope parts of the input. Assembling them back together should yield the
    /// original input.
    fn scope<'viewee>(&self, input: &'viewee str) -> ROScopes<'viewee> {
        let ranges = self.scope_raw(input);

        ROScopes::from_raw_ranges(input, ranges)
    }

    /// Scope the given `input`, returning raw ranges.
    ///
    /// Raw ranges are those not turned into [`ROScopes`] yet.
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee>;
}

// https://www.reddit.com/r/rust/comments/droxdg/why_arent_traits_impld_for_boxdyn_trait/
impl Scoper for Box<dyn Scoper> {
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        self.as_ref().scope_raw(input)
    }
}
