//! A code surgeon.
//!
//! This crate is binary-first, but the library (what you are viewing) is a close
//! second. It is not an afterthought and is supposed to be ergonomic, well-documented,
//! well-tested, and usable by other Rust code. Refer to this crate's repository and its
//! README for (much) more information.
//!
//! For the library, much like for the binary, there are two main concepts: actions and
//! scoping. The latter are manifested in [`ScopedView`]s. Over these, one can
//! [map][`ScopedView::map_without_context`] actions. Actions are all types implementing
//! [`Action`].
//!
//! # Examples
//!
//! A couple end-to-end examples specific to library usage are shown.
//!
//! ## Building a scoped view
//!
//! The starting point is always some [`str`] input. Over it, a [`ScopedView`] is built.
//! The latter is best constructed through a [`ScopedViewBuilder`]:
//!
//! ```rust
//! use std::borrow::Cow::{Borrowed as B};
//! use srgn::scoping::view::ScopedViewBuilder;
//! use srgn::scoping::scope::{Scope::{In}, RWScope, RWScopes};
//!
//! let input = "Hello, world!!";
//! let builder = ScopedViewBuilder::new(input);
//! let view = builder.build();
//!
//! // Everything is `In` scope. This is the starting point.
//! assert_eq!(
//!     view.scopes(),
//!     &RWScopes(vec![RWScope(In(B("Hello, world!!"), None))])
//! );
//! ```
//!
//! ## Exploding a scoped view
//!
//! The previous example did not achieve much of anything: neither was the view usefully
//! scoped (everything was in scope), nor was any action applied. The former is achieved
//! by, for example:
//!
//! ```rust
//! use std::borrow::Cow::{Borrowed as B};
//! use std::collections::HashMap;
//! use srgn::scoping::view::ScopedViewBuilder;
//! use srgn::scoping::scope::{Scope::{In, Out}, RWScope, RWScopes, ScopeContext::CaptureGroups};
//! use srgn::scoping::regex::{CaptureGroup::Numbered as CGN, Regex};
//! use srgn::RegexPattern;
//!
//! let input = "Hello, world!!";
//!
//! let mut builder = ScopedViewBuilder::new(input);
//!
//! let pattern = RegexPattern::new(r"[a-zA-Z]+").unwrap();
//! let scoper = Regex::new(pattern);
//!
//! builder.explode(&scoper);
//!
//! let view = builder.build();
//!
//! // Only parts matching the regex are `In` scope, the rest is `Out` of scope.
//! // These types are laborious to construct; there should be no need to ever do so manually.
//! assert_eq!(
//!     view.scopes(),
//!     &RWScopes(vec![
//!         RWScope(In(B("Hello"), Some(CaptureGroups(HashMap::from([(CGN(0), "Hello")]))))),
//!         RWScope(Out(&B(", "))),
//!         RWScope(In(B("world"), Some(CaptureGroups(HashMap::from([(CGN(0), "world")]))))),
//!         RWScope(Out(&B("!!"))),
//!     ])
//! );
//! ```
//!
//! ### Scoping with a language grammar
//!
//! Anything implementing [`Scoper`] is eligible for use in
//! [`ScopedViewBuilder::explode`]. This especially includes the language grammar-aware
//! types, which are [`Query`]'s. Those may be used as, for example:
//!
//! ```rust
//! use srgn::scoping::langs::{
//!     python::{CompiledQuery, PreparedQuery},
//!     RawQuery
//! };
//! use srgn::scoping::view::ScopedViewBuilder;
//!
//! let input = "def foo(bar: int) -> int: return bar + 1  # Do a thing";
//! let query = CompiledQuery::from(PreparedQuery::Comments);
//!
//! let mut builder = ScopedViewBuilder::new(input);
//! builder.explode(&query);
//!
//! let mut view = builder.build();
//! view.delete();
//!
//! // Comment gone, *however* trailing whitespace remains.
//! assert_eq!(
//!     view.to_string(),
//!     "def foo(bar: int) -> int: return bar + 1  "
//! );
//! ```
//!
//! ## Applying an action (associated function)
//!
//! With a usefully scoped view in hand, one can apply any number of actions. The
//! easiest is going through the provided associated functions directly:
//!
//! ```rust
//! use srgn::scoping::view::ScopedViewBuilder;
//! use srgn::scoping::regex::Regex;
//! use srgn::RegexPattern;
//!
//! let input = "Hello, world!!";
//!
//! let mut builder = ScopedViewBuilder::new(input);
//!
//! let pattern = RegexPattern::new(r"[a-z]+").unwrap();
//! let scoper = Regex::new(pattern);
//!
//! builder.explode(&scoper);
//!
//! let mut view = builder.build();
//! view.replace("👋".to_string());
//!
//! // All runs of lowercase letters are replaced by a single emoji.
//! assert_eq!(view.to_string(), "H👋, 👋!!");
//! ```
//!
//! Another example, using multiple actions and no scoping, is:
//!
//! ```rust
//! # #[cfg(feature = "symbols")] {
//! use srgn::scoping::view::ScopedViewBuilder;
//!
//! let input = "Assume π <= 4 < α -> β, ∀ x ∈ ℝ";
//!
//! // Short form: all `input` is in scope! No narrowing was applied.
//! let mut view = ScopedViewBuilder::new(input).build();
//! view.symbols();
//! view.upper();
//!
//! // Existing Unicode was uppercased properly, "ASCII symbols" were replaced.
//! assert_eq!(view.to_string(), "ASSUME Π ≤ 4 < Α → Β, ∀ X ∈ ℝ");
//! # }
//! ```
//!
//! ## Applying an action (passing)
//!
//! For maximum control, one can construct an action specifically and apply it that way.
//! For actions with options, this is the only way to set those options and not rely on
//! the [`Default`].
//!
//! ```rust
//! # #[cfg(feature = "german")] {
//! use srgn::scoping::view::ScopedViewBuilder;
//! use srgn::actions::German;
//!
//! let input = "Der Ueberflieger-Kaefer! 🛩️";
//!
//! let mut view = ScopedViewBuilder::new(input).build();
//! let action = German::new(true, false); // Excuse the bool ugliness.
//! view.map_without_context(&action);
//!
//! assert_eq!(view.to_string(), "Der Überflieger-Käfer! 🛩️");
//! # }
//! ```

#[cfg(doc)]
use std::ops::Range;

#[cfg(doc)]
use crate::{
    actions::Action,
    scoping::{
        langs::Query,
        view::{ScopedView, ScopedViewBuilder},
        Scoper,
    },
};

/// Main components around [`Action`]s.
pub mod actions;
/// Utilities around finding files.
pub mod find;
/// Components to work with collections of [`Range`]s.
pub mod ranges;
/// Main components around [`ScopedView`].
pub mod scoping;

/// Pattern signalling global scope, aka matching entire inputs.
pub const GLOBAL_SCOPE: &str = r".*";

/// The type of regular expression used throughout the crate. Abstracts away the
/// underlying implementation.
pub use fancy_regex::Regex as RegexPattern;
