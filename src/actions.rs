mod deletion;
#[cfg(feature = "german")]
mod german;
mod lower;
mod normalization;
/// Replacing inputs.
pub mod replace;
mod style;
#[cfg(feature = "symbols")]
mod symbols;
mod titlecase;
mod upper;

use std::error::Error;
use std::fmt;

pub use deletion::Deletion;
#[cfg(feature = "german")]
pub use german::German;
pub use lower::Lower;
pub use normalization::Normalization;
pub use replace::{Replacement, ReplacementError};
pub use style::Style;
#[cfg(feature = "symbols")]
pub use symbols::{inversion::Symbols as SymbolsInversion, Symbols};
pub use titlecase::Titlecase;
pub use upper::Upper;

use crate::scope::ScopeContext;

/// An action in the processing pipeline.
///
/// Actions are the core of the text processing pipeline and can be applied in any
/// order, [any number of times each](https://en.wikipedia.org/wiki/Idempotence) (more
/// than once being wasted work, though).
pub trait Action: Send + Sync {
    /// Apply this action to the given input.
    ///
    /// This is infallible: it cannot fail in the sense of [`Result`]. It can only
    /// return incorrect results, which would be bugs (please report).
    fn act(&self, input: &str) -> String;

    /// Acts taking into account additional context.
    ///
    /// By default, the context is ignored and [`Action::act`] is called. Implementors
    /// which need and know how to handle additional context can overwrite this method.
    ///
    /// # Errors
    ///
    /// This is fallible, as the context is dynamically created at runtime and
    /// potentially contains bad data. See docs of the [`Err`] variant type.
    fn act_with_context(
        &self,
        input: &str,
        context: &ScopeContext<'_>,
    ) -> Result<String, ActionError> {
        let _ = context; // Mark variable as used
        Ok(self.act(input))
    }
}

/// An error during application of an action.
#[derive(Debug, PartialEq, Eq)]
pub enum ActionError {
    /// Produced if [`Replacement`] fails.
    ReplacementError(ReplacementError),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReplacementError(re) => {
                write!(f, "Action failed in replacement: {re}")
            }
        }
    }
}

impl Error for ActionError {}

/// Any function that can be used as an [`Action`].
impl<T> Action for T
where
    T: Fn(&str) -> String + Send + Sync,
{
    fn act(&self, input: &str) -> String {
        self(input)
    }
}

/// Any [`Action`] that can be boxed.
// https://www.reddit.com/r/rust/comments/droxdg/why_arent_traits_impld_for_boxdyn_trait/
impl Action for Box<dyn Action> {
    fn act(&self, input: &str) -> String {
        self.as_ref().act(input)
    }

    fn act_with_context(
        &self,
        input: &str,
        context: &ScopeContext<'_>,
    ) -> Result<String, ActionError> {
        self.as_ref().act_with_context(input, context)
    }
}
