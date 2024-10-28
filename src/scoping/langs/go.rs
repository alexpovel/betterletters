use std::fmt::Debug;
use std::path::{Component, Path};

use clap::ValueEnum;
use const_format::formatcp;

use super::{Query, RawQuery, TSLanguage, TSQuery, TSQueryError};
use crate::find::Find;
use crate::scoping::langs::IGNORE;

/// A compiled query for the Go language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the Go language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError)variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_go::LANGUAGE.into(), &query)?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_go::LANGUAGE.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for Go.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments (single- and multi-line).
    Comments,
    /// Strings (interpreted and raw; excluding struct tags).
    Strings,
    /// Imports.
    Imports,
    /// Type definitions.
    TypeDef,
    /// Type alias assignments.
    TypeAlias,
    /// `struct` type definitions.
    Struct,
    /// `interface` type definitions.
    Interface,
    /// `const` specifications.
    Const,
    /// `var` specifications.
    Var,
    /// `func` definitions.
    Func,
    /// Method `func` definitions (`func (recv Recv) SomeFunc()`).
    Method,
    /// Free `func` definitions (`func SomeFunc()`).
    FreeFunc,
    /// `func init()` definitions.
    InitFunc,
    /// Type parameters (generics).
    TypeParams,
    /// `defer` blocks.
    Defer,
    /// `select` blocks.
    Select,
    /// `go` blocks.
    Go,
    /// `switch` blocks.
    Switch,
    /// Labeled statements.
    Labeled,
    /// `goto` statements.
    Goto,
    /// Struct tags.
    StructTags,
}

impl PreparedQuery {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => "(comment) @comment",
            Self::Strings => {
                formatcp!(
                    r"
                    [
                        (raw_string_literal)
                        (interpreted_string_literal)
                        (import_spec (interpreted_string_literal)) @{0}
                        (field_declaration tag: (raw_string_literal)) @{0}
                    ]
                    @string",
                    IGNORE
                )
            }
            Self::Imports => r"(import_spec path: (interpreted_string_literal) @path)",
            Self::TypeDef => r"(type_declaration) @type_decl",
            Self::TypeAlias => r"(type_alias) @type_alias",
            Self::Struct => r"(type_declaration (type_spec type: (struct_type))) @struct",
            Self::Interface => r"(type_declaration (type_spec type: (interface_type))) @interface",
            Self::Const => "(const_spec) @const",
            Self::Var => "(var_spec) @var",
            Self::Func => {
                r"
                [
                    (method_declaration)
                    (function_declaration)
                    (func_literal)
                ] @func"
            }
            Self::Method => "(method_declaration) @method",
            Self::FreeFunc => "(function_declaration) @free_func",
            Self::InitFunc => {
                r#"(function_declaration
                    name: (identifier) @id (#eq? @id "init")
                ) @init_func"#
            }
            Self::TypeParams => "(type_parameter_declaration) @type_params",
            Self::Defer => "(defer_statement) @defer",
            Self::Select => "(select_statement) @select",
            Self::Go => "(go_statement) @go",
            Self::Switch => "(expression_switch_statement) @switch",
            Self::Labeled => "(labeled_statement) @labeled",
            Self::Goto => "(goto_statement) @goto",
            Self::StructTags => "(field_declaration tag: (raw_string_literal) @tag)",
        }
    }
}

impl Query for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_go::LANGUAGE.into()
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
        &["go"]
    }

    fn is_path_invalid(&self, path: &Path) -> bool {
        for component in path.components() {
            if let Component::Normal(item) = component {
                // https://go.dev/ref/mod#vendoring
                if item.as_encoded_bytes() == b"vendor" {
                    return true;
                }
            }
        }

        false
    }
}
