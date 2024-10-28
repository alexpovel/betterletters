use std::fmt::Debug;

use clap::ValueEnum;

use super::{Find, Query, RawQuery, TSLanguage, TSQuery, TSQueryError};

/// A compiled query for the TypeScript language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the TypeScript language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError)variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(
            &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            &query,
        )?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for TypeScript.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments.
    Comments,
    /// Strings (literal, template).
    Strings,
    /// Imports (module specifiers).
    Imports,
    /// Any `function` definitions.
    Function,
    /// `async function` definitions.
    AsyncFunction,
    /// Non-`async function` definitions.
    SyncFunction,
    /// Method definitions.
    Method,
    /// `constructor` method definitions.
    Constructor,
    /// `class` definitions.
    Class,
    /// `enum` definitions.
    Enum,
    /// `interface` definitions.
    Interface,
    /// `try`/`catch`/`finally` blocks.
    TryCatch,
    /// Variable declarations (`let`, `const`, `var`).
    VarDecl,
    /// `let` variable declarations.
    Let,
    /// `const` variable declarations.
    Const,
    /// `var` variable declarations.
    Var,
    /// Type (generic) parameters.
    TypeParams,
    /// Type alias declarations.
    TypeAlias,
    /// `namespace` blocks.
    Namespace,
    /// `export` blocks.
    Export,
}

impl PreparedQuery {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => "(comment) @comment",
            Self::Imports => r"(import_statement source: (string (string_fragment) @sf))",
            Self::Strings => "(string_fragment) @string",
            Self::Function => "(function_declaration) @func",
            Self::AsyncFunction => {
                r#"(
                    (function_declaration) @func (#match? @func "^async")
                )"#
            }
            Self::SyncFunction => {
                r#"(
                    (function_declaration) @func (#not-match? @func "^async")
                )"#
            }
            Self::Method => "(method_definition) @method",
            Self::Constructor => {
                r#"(method_definition
                    name: (_) @name (#eq? @name "constructor")
                ) @constructor"#
            }
            Self::Class => "(class_declaration) @class",
            Self::Enum => "(enum_declaration) @enum",
            Self::Interface => "(interface_declaration) @interface",
            Self::TryCatch => "(try_statement) @try",
            Self::VarDecl => "(variable_declarator) @var_decl",
            Self::Let => {
                r#"(
                    (lexical_declaration) @let_decl (#match? @let_decl "^let ")
                )"#
            }
            Self::Const => {
                r#"(
                    (lexical_declaration) @const_decl (#match? @const_decl "^const ")
                )"#
            }
            Self::Var => {
                r#"(
                    (variable_declaration) @var_decl (#match? @var_decl "^var ")
                )"#
            }
            Self::TypeParams => "(type_parameters) @type_parameters",
            Self::TypeAlias => "(type_alias_declaration) @type_alias_declaration",
            Self::Namespace => "(internal_module) @internal_module",
            Self::Export => "(export_statement) @export",
        }
    }
}

impl Query for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
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
        &["ts", "tsx"]
    }
}
