use super::{CodeQuery, Language, LanguageScoper, TSLanguage, TSQuery};
use crate::{
    find::Find,
    scoping::{langs::IGNORE, scope::RangesWithContext, Scoper},
};
use clap::ValueEnum;
use const_format::formatcp;
use std::{fmt::Debug, str::FromStr};
use tree_sitter::QueryError;

/// The Go language.
pub type Go = Language<GoQuery>;
/// A query for Go.
pub type GoQuery = CodeQuery<CustomGoQuery, PreparedGoQuery>;

/// Prepared tree-sitter queries for Go.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedGoQuery {
    /// Comments (single- and multi-line).
    Comments,
    /// Strings (interpreted and raw; excluding struct tags).
    Strings,
    /// Imports.
    Imports,
    /// Struct tags.
    StructTags,
}

impl From<PreparedGoQuery> for TSQuery {
    fn from(value: PreparedGoQuery) -> Self {
        TSQuery::new(
            &Go::lang(),
            match value {
                PreparedGoQuery::Comments => "(comment) @comment",
                PreparedGoQuery::Strings => {
                    formatcp!(
                        r"
                        [
                            (raw_string_literal)
                            (interpreted_string_literal)
                            (import_spec (interpreted_string_literal) @{0})
                            (field_declaration tag: (raw_string_literal) @{0})
                        ]
                        @string",
                        IGNORE
                    )
                }
                PreparedGoQuery::Imports => {
                    r"(import_spec path: (interpreted_string_literal) @path)"
                }
                PreparedGoQuery::StructTags => "(field_declaration tag: (raw_string_literal) @tag)",
            },
        )
        .expect("Prepared queries to be valid")
    }
}

/// A custom tree-sitter query for Go.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomGoQuery(String);

impl FromStr for CustomGoQuery {
    type Err = QueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TSQuery::new(&Go::lang(), s) {
            Ok(_) => Ok(Self(s.to_string())),
            Err(e) => Err(e),
        }
    }
}

impl From<CustomGoQuery> for TSQuery {
    fn from(value: CustomGoQuery) -> Self {
        TSQuery::new(&Go::lang(), &value.0)
            .expect("Valid query, as object cannot be constructed otherwise")
    }
}

impl Scoper for Go {
    fn scope_raw<'viewee>(&self, input: &'viewee str) -> RangesWithContext<'viewee> {
        Self::scope_via_query(&mut self.query(), input).into()
    }
}

impl LanguageScoper for Go {
    fn lang() -> TSLanguage {
        tree_sitter_go::language()
    }

    fn query(&self) -> TSQuery {
        self.query.clone().into()
    }
}

impl Find for Go {
    fn extensions(&self) -> &'static [&'static str] {
        &["go"]
    }
}
