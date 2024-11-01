use std::fmt::Debug;

use clap::ValueEnum;

use super::{LanguageScoper, RawQuery, TSLanguage, TSQuery, TSQueryError};
use crate::find::Find;

/// A compiled query for the C language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the C language
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError) variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_c::LANGUAGE.into(), &query)?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_c::LANGUAGE.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for C.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments (single- and multi-line).
    Comments,
    /// Strings.
    Strings,
    /// Includes.
    Includes,
    /// Type definitions.
    TypeDef,
    /// `enum` definitions.
    Enum,
    /// `struct` type definitions.
    Struct,
    /// Variable definitions.
    Variable,
    /// All functions usages (declarations and calls).
    Function,
    /// Function definitions.
    FunctionDef,
    /// Function declaration.
    FunctionDecl,
    /// `switch` blocks.
    Switch,
    /// `if` blocks.
    If,
    /// `for` blocks.
    For,
    /// `while` blocks.
    While,
    /// `do` blocks.
    Do,
    /// `union` blocks.
    Union,
    /// Identifier.
    Identifier,
    /// Declaration.
    Declaration,
    /// Call expression.
    CallExpression,
}

impl PreparedQuery {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => "(comment) @comment",
            Self::Strings => "[(string_literal) (system_lib_string)] @string",
            Self::Includes => "(preproc_include) @include",
            Self::TypeDef => "(type_definition) @typedef",
            Self::Enum => "(enum_specifier) @enum",
            Self::Struct => "(struct_specifier) @struct",
            Self::Variable => "(declaration) @var",
            Self::Function => {
                "[(function_declarator (identifier)) (call_expression (identifier))] @function"
            }
            Self::FunctionDef => "(function_definition) @function_definition",
            Self::FunctionDecl => "(function_declarator) @function_decl",
            Self::Switch => "(switch_statement) @switch",
            Self::If => "(if_statement) @if",
            Self::For => "(for_statement) @for",
            Self::While => "(while_statement) @while",
            Self::Union => "(union_specifier) @union",
            Self::Do => "(do_statement) @do",
            Self::Identifier => "(identifier) @ident",
            Self::Declaration => "(declaration) @decl",
            Self::CallExpression => "(call_expression) @call",
        }
    }
}

impl LanguageScoper for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_c::LANGUAGE.into()
    }

    fn pos_query(&self) -> &TSQuery {
        &self.0.positive_query
    }

    fn neg_query(&self) -> Option<&TSQuery> {
        self.0.negative_query.as_ref()
    }
}

impl Find for CompiledQuery {
    fn extensions(&self) -> &'static [&'static str] {
        &["c", "h"]
    }
}
