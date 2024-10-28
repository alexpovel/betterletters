use std::fmt::Debug;

use clap::ValueEnum;
use const_format::formatcp;

use super::{Query, RawQuery, TSLanguage, TSQuery, TSQueryError};
use crate::find::Find;
use crate::scoping::langs::IGNORE;

/// A compiled query for the C# language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the C# language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError)variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_c_sharp::LANGUAGE.into(), &query)
            .expect("syntax of prepared queries is validated by tests");
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_c_sharp::LANGUAGE.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for C#.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments (including XML, inline, doc comments).
    Comments,
    /// Strings (incl. verbatim, interpolated; incl. quotes, except for interpolated).
    ///
    /// Raw strings are [not yet
    /// supported](https://github.com/tree-sitter/tree-sitter-c-sharp/pull/240).
    Strings,
    /// `using` directives (including periods).
    Usings,
    /// `struct` definitions (in their entirety).
    Struct,
    /// `enum` definitions (in their entirety).
    Enum,
    /// `interface` definitions (in their entirety).
    Interface,
    /// `class` definitions (in their entirety).
    Class,
    /// Method definitions (in their entirety).
    Method,
    /// Variable declarations (in their entirety).
    VariableDeclaration,
    /// Property definitions (in their entirety).
    Property,
    /// Constructor definitions (in their entirety).
    Constructor,
    /// Destructor definitions (in their entirety).
    Destructor,
    /// Field definitions on types (in their entirety).
    Field,
    /// Attribute names.
    Attribute,
    /// Identifier names.
    Identifier,
}

impl PreparedQuery {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => "(comment) @comment",
            Self::Usings => r"(using_directive [(identifier) (qualified_name)] @import)",
            Self::Strings => {
                formatcp!(
                    r"
                    [
                        (interpolated_string_expression (interpolation) @{0})
                        (string_literal)
                        (raw_string_literal)
                        (verbatim_string_literal)
                    ]
                    @string
                    ",
                    IGNORE
                )
            }
            Self::Struct => "(struct_declaration) @struct",
            Self::Enum => "(enum_declaration) @enum",
            Self::Interface => "(interface_declaration) @interface",
            Self::Class => "(class_declaration) @class",
            Self::Method => "(method_declaration) @method",
            Self::VariableDeclaration => "(variable_declaration) @variable",
            Self::Property => "(property_declaration) @property",
            Self::Constructor => "(constructor_declaration) @constructor",
            Self::Destructor => "(destructor_declaration) @destructor",
            Self::Field => "(field_declaration) @field",
            Self::Attribute => "(attribute) @attribute",
            Self::Identifier => "(identifier) @identifier",
        }
    }
}

impl Query for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_c_sharp::LANGUAGE.into()
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
        &["cs"]
    }
}
