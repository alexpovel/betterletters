use std::fmt::Debug;

use clap::ValueEnum;
use const_format::formatcp;

use super::{Find, LanguageScoper, RawQuery, TSLanguage, TSQuery, TSQueryError, IGNORE};

/// A compiled query for the Rust language.
#[derive(Debug)]
pub struct CompiledQuery(super::CompiledQuery);

impl TryFrom<RawQuery> for CompiledQuery {
    type Error = TSQueryError;

    /// Create a new compiled query for the Rust language.
    ///
    /// # Errors
    ///
    /// See the concrete type of the [`TSQueryError`](tree_sitter::QueryError) variant for when this method errors.
    fn try_from(query: RawQuery) -> Result<Self, Self::Error> {
        let q = super::CompiledQuery::from_raw_query(&tree_sitter_rust::LANGUAGE.into(), &query)?;
        Ok(Self(q))
    }
}

impl From<PreparedQuery> for CompiledQuery {
    fn from(query: PreparedQuery) -> Self {
        Self(super::CompiledQuery::from_prepared_query(
            &tree_sitter_rust::LANGUAGE.into(),
            query.as_str(),
        ))
    }
}

/// Prepared tree-sitter queries for Rust.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PreparedQuery {
    /// Comments (line and block styles; excluding doc comments; comment chars incl.).
    Comments,
    /// Doc comments (comment chars included).
    DocComments,
    /// Use statements (paths only; excl. `use`/`as`/`*`).
    Uses,
    /// Strings (regular, raw, byte; includes interpolation parts in format strings!).
    ///
    /// There is currently no support for an 'interpolation' type node in
    /// tree-sitter-rust (like there is in TypeScript and Python, for example).
    Strings,
    /// Attributes like `#[attr]`.
    Attribute,
    /// `struct` definitions.
    Struct,
    /// `struct` definitions not marked `pub`.
    PrivStruct,
    /// `struct` definitions marked `pub`.
    PubStruct,
    /// `struct` definitions marked `pub(crate)`.
    PubCrateStruct,
    /// `struct` definitions marked `pub(self)`.
    PubSelfStruct,
    /// `struct` definitions marked `pub(super)`.
    PubSuperStruct,
    /// `enum` definitions.
    Enum,
    /// `enum` definitions not marked `pub`.
    PrivEnum,
    /// `enum` definitions marked `pub`.
    PubEnum,
    /// `enum` definitions marked `pub(crate)`.
    PubCrateEnum,
    /// `enum` definitions marked `pub(self)`.
    PubSelfEnum,
    /// `enum` definitions marked `pub(super)`.
    PubSuperEnum,
    /// Variant members of `enum` definitions
    EnumVariant,
    /// Function definitions.
    Fn,
    /// Function definitions inside `impl` blocks (associated functions/methods).
    ImplFn,
    /// Function definitions not marked `pub`.
    PrivFn,
    /// Function definitions marked `pub`.
    PubFn,
    /// Function definitions marked `pub(crate)`.
    PubCrateFn,
    /// Function definitions marked `pub(self)`.
    PubSelfFn,
    /// Function definitions marked `pub(super)`.
    PubSuperFn,
    /// Function definitions marked `const`
    ConstFn,
    /// Function definitions marked `async`
    AsyncFn,
    /// Function definitions marked `unsafe`
    UnsafeFn,
    /// Function definitions marked `extern`
    ExternFn,
    /// Function definitions with attributes containing `test` (`#[test]`, `#[rstest]`,
    /// ...).
    TestFn,
    /// `trait` definitions.
    Trait,
    /// `impl` blocks.
    Impl,
    /// `impl` blocks for types (`impl SomeType {}`).
    ImplType,
    /// `impl` blocks for traits on types (`impl SomeTrait for SomeType {}`).
    ImplTrait,
    /// `mod` blocks.
    Mod,
    /// `mod tests` blocks.
    ModTests,
    /// Type definitions (`struct`, `enum`, `union`).
    TypeDef,
    /// Identifiers.
    Identifier,
    /// Identifiers for types.
    TypeIdentifier,
    /// Closure definitions.
    Closure,
    /// `unsafe` keyword usages (`unsafe fn`, `unsafe` blocks, `unsafe Trait`, `unsafe
    /// impl Trait`).
    Unsafe,
}

impl PreparedQuery {
    #[allow(clippy::too_many_lines)]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Comments => {
                r#"
                [
                    (line_comment)+ @line
                    (block_comment)
                    (#not-match? @line "^///")
                ]
                @comment
                "#
            }
            Self::DocComments => {
                r#"
                (
                    (line_comment)+ @line
                    (#match? @line "^//(/|!)")
                )
                "#
            }
            Self::Uses => {
                // Match any (wildcard `_`) `argument`, which includes:
                //
                // - `scoped_identifier`
                // - `scoped_use_list`
                // - `use_wildcard`
                // - `use_as_clause`
                //
                // all at once.
                r"
                [
                    (use_declaration
                        argument: (_) @use
                    )
                ]
                "
            }
            Self::Strings => "(string_content) @string",
            Self::Attribute => "(attribute) @attribute",
            Self::Struct => "(struct_item) @struct_item",
            Self::PrivStruct => {
                r"(struct_item
                    .
                    name: (type_identifier)
                ) @struct_item_without_visibility_modifier"
            }
            Self::PubStruct => {
                r#"(struct_item
                    (visibility_modifier) @vis
                    (#eq? @vis "pub")
                ) @struct_item"#
            }
            Self::PubCrateStruct => {
                r"(struct_item
                    (visibility_modifier (crate))
                ) @struct_item"
            }
            Self::PubSelfStruct => {
                r"(struct_item
                    (visibility_modifier (self))
                ) @struct_item"
            }
            Self::PubSuperStruct => {
                r"(struct_item
                    (visibility_modifier (super))
                ) @struct_item"
            }
            Self::Enum => "(enum_item) @enum_item",
            Self::PrivEnum => {
                r"(enum_item
                    .
                    name: (type_identifier)
                ) @enum_item_without_visibility_modifier"
            }
            Self::PubEnum => {
                r#"(enum_item
                    (visibility_modifier) @vis
                    (#eq? @vis "pub")
                ) @enum_item"#
            }
            Self::PubCrateEnum => {
                r"(enum_item
                    (visibility_modifier (crate))
                ) @enum_item"
            }
            Self::PubSelfEnum => {
                r"(enum_item
                    (visibility_modifier (self))
                ) @enum_item"
            }
            Self::PubSuperEnum => {
                r"(enum_item
                    (visibility_modifier (super))
                ) @enum_item"
            }
            Self::EnumVariant => "(enum_variant) @enum_variant",
            Self::Fn => "(function_item) @function_item",
            Self::ImplFn => {
                r"(impl_item
                    body: (_ (function_item) @function)
                )"
            }
            Self::PrivFn => {
                r"(function_item
                    .
                    name: (identifier)
                ) @function_item_without_visibility_modifier"
            }
            Self::PubFn => {
                r#"(function_item
                    (visibility_modifier) @vis
                    (#eq? @vis "pub")
                ) @function_item"#
            }
            Self::PubCrateFn => {
                r"(function_item
                    (visibility_modifier (crate))
                ) @function_item"
            }
            Self::PubSelfFn => {
                r"(function_item
                    (visibility_modifier (self))
                ) @function_item"
            }
            Self::PubSuperFn => {
                r"(function_item
                    (visibility_modifier (super))
                ) @function_item"
            }
            Self::ConstFn => {
                r#"(function_item
                    (function_modifiers) @funcmods
                    (#match? @funcmods "const")
                ) @function_item"#
            }
            Self::AsyncFn => {
                r#"(function_item
                    (function_modifiers) @funcmods
                    (#match? @funcmods "async")
                ) @function_item"#
            }
            Self::UnsafeFn => {
                r#"(function_item
                    (function_modifiers) @funcmods
                    (#match? @funcmods "unsafe")
                ) @function_item"#
            }
            Self::ExternFn => {
                r"(function_item
                    (function_modifiers (extern_modifier))
                ) @extern_function"
            }
            Self::TestFn => {
                // Any attribute which matches aka contains `test`, preceded or
                // followed by more attributes, eventually preceded by a function.
                // The anchors of `.` ensure nothing but the items we're after occur
                // in between.
                formatcp!(
                    "
                    (
                        (attribute_item)*
                        .
                        (attribute_item (attribute) @{0}.attr (#match? @{0}.attr \"test\"))
                        .
                        (attribute_item)*
                        .
                        (function_item) @func
                    )",
                    IGNORE
                )
            }
            Self::Trait => "(trait_item) @trait_item",
            Self::Impl => "(impl_item) @impl_item",
            Self::ImplType => {
                r"(impl_item
                    type: (_)
                    !trait
                ) @impl_item"
            }
            Self::ImplTrait => {
                r"(impl_item
                    trait: (_)
                    .
                    type: (_)
                ) @impl_item"
            }
            Self::Mod => "(mod_item) @mod_item",
            Self::ModTests => {
                r#"(mod_item
                    name: (identifier) @mod_name
                    (#eq? @mod_name "tests")
                ) @mod_tests
                "#
            }
            Self::TypeDef => {
                r"
                [
                    (struct_item)
                    (enum_item)
                    (union_item)
                ]
                @typedef
                "
            }
            Self::Identifier => "(identifier) @identifier",
            Self::TypeIdentifier => "(type_identifier) @identifier",
            Self::Closure => "(closure_expression) @closure",
            Self::Unsafe => {
                r#"
                    [
                        (
                            (trait_item) @ti (#match? @ti "^unsafe")
                        )
                        (
                            (impl_item) @ii (#match? @ii "^unsafe")
                        )
                        (function_item
                            (function_modifiers) @funcmods
                            (#match? @funcmods "unsafe")
                        ) @function_item
                        (function_signature_item
                            (function_modifiers) @funcmods
                            (#match? @funcmods "unsafe")
                        ) @function_signature_item
                        (unsafe_block) @block
                    ] @unsafe
                "#
            }
        }
    }
}

impl LanguageScoper for CompiledQuery {
    fn lang() -> TSLanguage {
        tree_sitter_rust::LANGUAGE.into()
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
        &["rs"]
    }
}
