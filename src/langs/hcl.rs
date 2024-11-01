use std::fmt::Debug;

use clap::ValueEnum;
use const_format::formatcp;

use super::{tree_sitter_hcl, LanguageScoper, RawQuery, TSLanguage, TSQuery, TSQueryError};
use crate::find::Find;
use crate::langs::IGNORE;

/// A compiled query for the Hashicorp Configuration language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the Hashicorp Configuration language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError)variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_hcl::language(), &query)?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_hcl::language(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for Hcl.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// `variable` blocks (in their entirety).
    Variable,
    /// `resource` blocks (in their entirety).
    Resource,
    /// `data` blocks (in their entirety).
    Data,
    /// `output` blocks (in their entirety).
    Output,
    /// `provider` blocks (in their entirety).
    Provider,
    /// `terraform` blocks (in their entirety).
    Terraform,
    /// `locals` blocks (in their entirety).
    Locals,
    /// `module` blocks (in their entirety).
    Module,
    /// Variable declarations and usages.
    Variables,
    /// `resource` name declarations and usages.
    ///
    /// In `resource "a" "b"`, only "b" is matched.
    ResourceNames,
    /// `resource` type declarations and usages.
    ///
    /// In `resource "a" "b"`, only "a" is matched.
    ResourceTypes,
    /// `data` name declarations and usages.
    ///
    /// In `data "a" "b"`, only "b" is matched.
    DataNames,
    /// `data` source declarations and usages.
    ///
    /// In `data "a" "b"`, only "a" is matched.
    DataSources,
    /// Comments.
    Comments,
    /// Literal strings.
    ///
    /// Excluding resource, variable, ... names as well as interpolation parts.
    Strings,
}

impl PreparedQuery {
    #[allow(clippy::too_many_lines)] // No good way to avoid
    const fn as_str(self) -> &'static str {
        // Seems to not play nice with the macro. Put up here, else interpolation is
        // affected.
        #[allow(clippy::needless_raw_string_hashes)]
        match self {
            Self::Variable => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "variable")
                    ) @block
                "#
            }
            Self::Resource => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "resource")
                    ) @block
                "#
            }
            Self::Data => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "data")
                    ) @block
                "#
            }
            Self::Output => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "output")
                    ) @block
                "#
            }
            Self::Provider => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "provider")
                    ) @block
                "#
            }
            Self::Terraform => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "terraform")
                    ) @block
                "#
            }
            Self::Locals => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "locals")
                    ) @block
                "#
            }
            Self::Module => {
                r#"
                    (block
                        (identifier) @name
                        (#eq? @name "module")
                    ) @block
                "#
            }
            Self::Variables => {
                // Capturing nodes with names, such as `@id`, requires names to be
                // unique across the *entire* query, else things break. Hence, us
                // `@a.b` syntax (which seems undocumented).
                formatcp!(
                    r#"
                        [
                            (block
                                (identifier) @{0}.declaration
                                (string_lit (template_literal) @name.declaration)
                                (#match? @{0}.declaration "variable")
                            )
                            (
                                (variable_expr
                                    (identifier) @{0}.usage
                                    (#match? @{0}.usage "var")
                                )
                                .
                                (get_attr
                                    (identifier) @name.usage
                                )
                            )
                        ]
                    "#,
                    IGNORE
                )
            }
            Self::ResourceNames => {
                // Capturing nodes with names, such as `@id`, requires names to be
                // unique across the *entire* query, else things break. Hence, us
                // `@a.b` syntax (which seems undocumented).
                formatcp!(
                    r#"
                        [
                            (block
                                (identifier) @{0}.declaration
                                (string_lit)
                                (string_lit (template_literal) @name.declaration)
                                (#match? @{0}.declaration "resource")
                            )
                            (
                                (variable_expr
                                    (identifier) @{0}.usage
                                    (#not-any-of? @{0}.usage
                                        "var"
                                        "data"
                                        "count"
                                        "module"
                                        "local"
                                    )
                                )
                                .
                                (get_attr
                                    (identifier) @name.usage
                                )
                            )
                        ]
                    "#,
                    IGNORE
                )
            }
            Self::ResourceTypes => {
                // Capturing nodes with names, such as `@id`, requires names to be
                // unique across the *entire* query, else things break. Hence, us
                // `@a.b` syntax (which seems undocumented).
                formatcp!(
                    r#"
                        [
                            (block
                                (identifier) @{0}.declaration
                                (string_lit (template_literal) @name.type)
                                (string_lit)
                                (#match? @{0}.declaration "resource")
                            )
                            (
                                (variable_expr
                                    .
                                    (identifier) @name.usage
                                    (#not-any-of? @name.usage
                                        "var"
                                        "data"
                                        "count"
                                        "module"
                                        "local"
                                    )
                                )
                                .
                                (get_attr
                                    (identifier)
                                )
                            )
                        ]
                    "#,
                    IGNORE
                )
            }
            Self::DataNames => {
                // Capturing nodes with names, such as `@id`, requires names to be
                // unique across the *entire* query, else things break. Hence, us
                // `@a.b` syntax (which seems undocumented).
                formatcp!(
                    r#"
                        [
                            (block
                                (identifier) @{0}.declaration
                                (string_lit)
                                (string_lit (template_literal) @name.declaration)
                                (#match? @{0}.declaration "data")
                            )
                            (
                                (variable_expr
                                    (identifier) @{0}.usage
                                    (#match? @{0}.usage "data")
                                )
                                .
                                (get_attr
                                    (identifier)
                                )
                                .
                                (get_attr
                                    (identifier) @name.usage
                                )
                            )
                        ]
                    "#,
                    IGNORE
                )
            }
            Self::DataSources => {
                // Capturing nodes with names, such as `@id`, requires names to be
                // unique across the *entire* query, else things break. Hence, us
                // `@a.b` syntax (which seems undocumented).
                formatcp!(
                    r#"
                        [
                            (block
                                (identifier) @{0}.declaration
                                (string_lit (template_literal) @name.provider)
                                (string_lit)
                                (#match? @{0}.declaration "data")
                            )
                            (
                                (variable_expr
                                    (identifier) @{0}.usage
                                    (#match? @{0}.usage "data")
                                )
                                .
                                (get_attr
                                    (identifier) @name.provider
                                )
                                .
                                (get_attr
                                    (identifier)
                                )
                            )
                        ]
                    "#,
                    IGNORE
                )
            }
            Self::Comments => "(comment) @comment",
            Self::Strings => {
                r"
                [
                    (literal_value
                        (string_lit
                            (template_literal) @string.literal
                        )
                    )
                    (quoted_template
                        (template_literal) @string.template_literal
                    )
                    (heredoc_template
                        (template_literal) @string.heredoc_literal
                    )
                ]
                "
            }
        }
    }
}

impl LanguageScoper for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_hcl::language()
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
        &["hcl", "tf"]
    }
}
