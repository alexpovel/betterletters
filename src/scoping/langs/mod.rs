use std::marker::PhantomData;
use std::str::FromStr;

use log::{debug, info, trace};
pub use tree_sitter::{
    Language as TSLanguage, Parser as TSParser, Query as TSQuery, QueryCursor as TSQueryCursor,
};

use super::scope::RangesWithContext;
use super::Scoper;
use crate::find::Find;
use crate::ranges::Ranges;
#[cfg(doc)]
use crate::scoping::{
    scope::Scope::{In, Out},
    view::ScopedViewBuilder,
};

/// C.
pub mod c;
/// C#.
pub mod csharp;
/// Go.
pub mod go;
/// Hashicorp Configuration Language
pub mod hcl;
/// Python.
pub mod python;
/// Rust.
pub mod rust;
mod tree_sitter_hcl;
/// TypeScript.
pub mod typescript;
/// Typst.
pub mod typst;

/// Represents a (programming) language.
#[derive(Debug)]
pub struct Language<Q> {
    /// The *positive* query: it will be run against input and its results used for
    /// scoping.
    positive_query: TSQuery,
    /// The *negative* query: if present (if [`IGNORE`] is present) will be run and
    /// *subtracted* from the positive query.
    negative_query: Option<TSQuery>,
    /// We are generic over this to allow languages to be their own type.
    ///
    /// We only store constructed [`TSQuery`]s as those are actually actionable and
    /// references to them are required. However, [`TSQuery`]s are neither [`Clone`] nor
    /// type-safe (a [`TSQuery`] is *associated* with a [`TSLanguage`], but that's not
    /// enforced at the type level: a query constructed for Python could accidentally be
    /// passed to Go etc.), so use this field to be more type-safe.
    _marker: PhantomData<Q>,
}

impl<Q> Language<Q>
where
    Q: Into<TSQuery> + Clone,
{
    /// Create a new language with the given associated query over it.
    #[allow(clippy::needless_pass_by_value)] // TODO: refactor later
    pub fn new(query: Q) -> Self {
        let positive_query = query.clone().into();

        let is_ignored = |name: &str| name.starts_with(IGNORE);
        let has_ignored_captures = positive_query
            .capture_names()
            .iter()
            .any(|name| is_ignored(name));

        let negative_query = has_ignored_captures.then(|| {
            let mut query = query.clone().into();
            let acknowledged_captures = query
                .capture_names()
                .iter()
                .filter(|name| !is_ignored(name))
                .map(|s| String::from(*s))
                .collect::<Vec<_>>();

            for name in acknowledged_captures {
                trace!("Disabling capture for: {:?}", name);
                query.disable_capture(&name);
            }

            query
        });

        Self {
            positive_query,
            negative_query,
            _marker: PhantomData,
        }
    }
}

/// A query over a language, for scoping.
///
/// Parts hit by the query are [`In`] scope, parts not hit are [`Out`] of scope.
#[derive(Debug, Clone)]
pub enum CodeQuery<C, P>
where
    C: FromStr + Into<TSQuery>,
    P: Into<TSQuery>,
{
    /// A custom, user-defined query.
    Custom(C),
    /// A prepared query.
    ///
    /// Availability depends on the language, respective languages features, and
    /// implementation in this crate.
    Prepared(P),
}

impl<C, P> From<CodeQuery<C, P>> for TSQuery
where
    C: FromStr + Into<Self>,
    P: Into<Self>,
{
    fn from(value: CodeQuery<C, P>) -> Self {
        match value {
            CodeQuery::Custom(query) => query.into(),
            CodeQuery::Prepared(query) => query.into(),
        }
    }
}

/// In a query, use this name to mark a capture to be ignored.
///
/// Useful for queries where tree-sitter doesn't natively support a fitting node type,
/// and a result is instead obtained by ignoring unwanted parts of bigger captures.
pub(super) const IGNORE: &str = "_SRGN_IGNORE";

/// A scoper for a language.
///
/// Functions much the same, but provides specific language-related functionality.
pub trait LanguageScoper: Scoper + Find + Send + Sync {
    /// The language's tree-sitter language.
    fn lang() -> TSLanguage
    where
        Self: Sized; // Exclude from trait object

    /// The language's *positive* tree-sitter query.
    ///
    /// Its results indicate items in scope.
    fn pos_query(&self) -> &TSQuery
    where
        Self: Sized; // Exclude from trait object

    /// The language's *negative* tree-sitter query.
    ///
    /// Its results will be *subtracted* from the positive ones.
    fn neg_query(&self) -> Option<&TSQuery>
    where
        Self: Sized; // Exclude from trait object

    /// The language's tree-sitter parser.
    #[must_use]
    fn parser() -> TSParser
    where
        Self: Sized, // Exclude from trait object
    {
        let mut parser = TSParser::new();
        parser
            .set_language(&Self::lang())
            .expect("Should be able to load language grammar and parser");

        parser
    }

    /// Scope the given input using the language's query.
    ///
    /// In principle, this is the same as [`Scoper::scope`].
    fn scope_via_query(&self, input: &str) -> Ranges<usize>
    where
        Self: Sized, // Exclude from trait object
    {
        // tree-sitter is about incremental parsing, which we don't use here
        let old_tree = None;

        trace!("Parsing into AST: {:?}", input);

        let tree = Self::parser()
            .parse(input, old_tree)
            .expect("No language set in parser, or other unrecoverable error");

        let root = tree.root_node();
        debug!(
            "S expression of parsed source code is: {:?}",
            root.to_sexp()
        );

        let run = |query: &TSQuery| {
            trace!("Running query: {:?}", query);

            let mut qc = TSQueryCursor::new();
            let matches = qc.matches(query, root, input.as_bytes());

            let mut ranges: Ranges<usize> = matches
                .flat_map(|query_match| query_match.captures)
                .map(|capture| capture.node.byte_range())
                .collect();

            // ⚠️ tree-sitter queries with multiple captures will return them in some
            // mixed order (not ordered, and not merged), but we later rely on cleanly
            // ordered, non-overlapping ranges (a bit unfortunate we have to know about
            // that remote part over here).
            ranges.merge();
            trace!("Querying yielded ranges: {:?}", ranges);

            ranges
        };

        let ranges = run(self.pos_query());
        match &self.neg_query() {
            Some(nq) => ranges - run(nq),
            None => ranges,
        }
    }
}

impl<T> Scoper for T
where
    T: LanguageScoper,
{
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        self.scope_via_query(input).into()
    }
}

impl Scoper for Box<dyn LanguageScoper> {
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        self.as_ref().scope_raw(input)
    }
}

impl Scoper for &[Box<dyn LanguageScoper>] {
    /// Allows *multiple* scopers to be applied all at once.
    ///
    /// They are OR'd together in the sense that if *any* of the scopers hit, a
    /// position/range is considered in scope. In some sense, this is the opposite of
    /// [`ScopedViewBuilder::explode`], which is subtractive.
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        trace!("Scoping many scopes: {:?}", input);

        if self.is_empty() {
            trace!("Short-circuiting: self is empty, nothing to scope.");
            return vec![(0..input.len(), None)].into_iter().collect();
        }

        // This is slightly leaky in that it drops down to a more 'primitive' layer and
        // uses `Ranges`.
        let mut ranges: Ranges<usize> = self
            .iter()
            .flat_map(|s| s.scope_raw(input))
            .map(|(range, ctx)| {
                assert!(
                    ctx.is_none(),
                    "When language scoping runs, no contexts exist yet."
                );
                range
            })
            .collect();
        ranges.merge();
        info!("New ranges after scoping many: {ranges:?}");

        let ranges: RangesWithContext<'_> = ranges.into_iter().map(|r| (r, None)).collect();

        ranges
    }
}
