use std::fmt::Debug;

use clap::ValueEnum;
use const_format::formatcp;

use super::{Find, Query, RawQuery, TSLanguage, TSQuery, TSQueryError};
use crate::scoping::langs::IGNORE;

/// A compiled query for the Python language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the Python language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError)variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_python::LANGUAGE.into(), &query)?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_python::LANGUAGE.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for Python.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments.
    Comments,
    /// Strings (raw, byte, f-strings; interpolation not included).
    Strings,
    /// Module names in imports (incl. periods; excl. `import`/`from`/`as`/`*`).
    Imports,
    /// Docstrings (not including multi-line strings).
    DocStrings,
    /// Function names, at the definition site.
    FunctionNames,
    /// Function calls.
    FunctionCalls,
    /// Class definitions (in their entirety).
    Class,
    /// Function definitions (*all* `def` block in their entirety).
    Def,
    /// Async function definitions (*all* `async def` block in their entirety).
    AsyncDef,
    /// Function definitions inside `class` bodies.
    Methods,
    /// Function definitions decorated as `classmethod` (excl. the decorator).
    ClassMethods,
    /// Function definitions decorated as `staticmethod` (excl. the decorator).
    StaticMethods,
    /// `with` blocks (in their entirety).
    With,
    /// `try` blocks (in their entirety).
    Try,
    /// `lambda` statements (in their entirety).
    Lambda,
    /// Global, i.e. module-level variables.
    Globals,
    /// Identifiers for variables (left-hand side of assignments).
    VariableIdentifiers,
    /// Types in type hints.
    Types,
    /// Identifiers (variable names, ...).
    Identifiers,
}

impl PreparedQuery {
    #[allow(clippy::too_many_lines)]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => "(comment) @comment",
            Self::Strings => "(string_content) @string",
            Self::Imports => {
                r"[
                    (import_statement
                            name: (dotted_name) @dn)
                    (import_from_statement
                            module_name: (dotted_name) @dn)
                    (import_from_statement
                            module_name: (dotted_name) @dn
                                (wildcard_import))
                    (import_statement(
                        aliased_import
                            name: (dotted_name) @dn))
                    (import_from_statement
                        module_name: (relative_import) @ri)
                ]"
            }
            Self::DocStrings => {
                // Triple-quotes are also used for multi-line strings. So look only
                // for stand-alone expressions, which are not part of some variable
                // assignment.
                formatcp!(
                    "
                    (
                        (expression_statement
                            (string
                                (string_start) @{0}
                                (string_content) @string
                                (#match? @{0} \"\\^\\\"\\\"\\\"\")
                            )
                        )
                    )
                    ",
                    IGNORE
                )
            }
            Self::FunctionNames => {
                r"
                (function_definition
                    name: (identifier) @function-name
                )
                "
            }
            Self::FunctionCalls => {
                r"
                (call
                    function: (identifier) @function-name
                )
                "
            }
            Self::Class => "(class_definition) @class",
            Self::Def => "(function_definition) @def",
            Self::AsyncDef => r#"((function_definition) @def (#match? @def "^async "))"#,
            Self::Methods => {
                r"
                (class_definition
                    body: (block
                        [
                            (function_definition) @method
                            (decorated_definition definition: (function_definition)) @method
                        ]
                    )
                )
                "
            }
            Self::ClassMethods => {
                formatcp!(
                    "
                    (class_definition
                        body: (block
                            (decorated_definition
                                (decorator (identifier) @{0})
                                definition: (function_definition) @method
                                (#eq? @{0} \"classmethod\")
                            )
                        )
                    )",
                    IGNORE
                )
            }
            Self::StaticMethods => {
                formatcp!(
                    "
                    (class_definition
                        body: (block
                            (decorated_definition
                                (decorator (identifier) @{0})
                                definition: (function_definition) @method
                                (#eq? @{0} \"staticmethod\")
                            )
                        )
                    )",
                    IGNORE
                )
            }
            Self::With => "(with_statement) @with",
            Self::Try => "(try_statement) @try",
            Self::Lambda => "(lambda) @lambda",
            Self::Globals => {
                "(module (expression_statement (assignment left: (identifier) @global)))"
            }
            Self::VariableIdentifiers => "(assignment left: (identifier) @identifier)",
            Self::Types => "(type) @type",
            Self::Identifiers => "(identifier) @identifier",
        }
    }
}

impl Query for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_python::LANGUAGE.into()
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
        &["py"]
    }

    fn interpreters(&self) -> Option<&'static [&'static str]> {
        Some(&["python", "python3"])
    }
}
