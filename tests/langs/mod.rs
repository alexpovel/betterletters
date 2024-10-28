use std::ops::Range;

use rstest::rstest;
use serde::{Deserialize, Serialize};
use srgn::scoping::langs::{c, csharp, go, hcl, python, rust, typescript, Query};
use srgn::scoping::scope::Scope;
use srgn::scoping::view::ScopedViewBuilder;

/// A type that when serialized, will visually highlight the portions of a line which
/// were matched.
///
/// Single letter names to not disrupt alignment.
///
/// For example, when serialized as YAML, produces output such as
///
/// ```yaml
/// n: 156
/// l: "        print(f\"Loop iteration {i}\")\n"
/// m: "        ^^^^^                           "
/// ```
///
/// with `m` indicating the [`Scope::In`] part of the `l`ine.
#[derive(Debug, Serialize, Deserialize)]
struct InScopeLinePart {
    /// Line number.
    n: usize,
    /// Line itself, in original form.
    l: String,
    /// The string highlighting the matching. Has to be serialized *below*.
    m: String,
}

impl InScopeLinePart {
    fn new(line_number: usize, line_contents: String, match_: &str, span: Range<usize>) -> Self {
        // Split into the three components
        let (start, mid, end) = (
            &line_contents[..span.start],
            &line_contents[span.clone()],
            &line_contents[span.end..],
        );

        // ASSUMPTION is that `.escape_default()` will escape the same way (YAML, ...)
        // serialization does. Important for alignment to be correct.

        // Leading space
        let mut m = " ".repeat(start.escape_default().to_string().len());
        // The highlights for the line above.
        m.push_str(&"^".repeat(mid.escape_default().to_string().len()));
        // Trailing spaces; not strictly necessary but slightly nicer.
        m.push_str(&" ".repeat(end.escape_default().to_string().len()));

        assert_eq!(
            &line_contents[span], match_,
            "What is highlighted is what was matched"
        );

        Self {
            n: line_number,
            l: line_contents,
            m,
        }
    }
}

#[rstest]
#[case(
    "base.py_comments",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Comments),
)]
#[case(
    "base.py_strings",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Strings),
)]
#[case(
    "base.py_imports",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Imports),
)]
#[case(
    "base.py_docstrings",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::DocStrings),
)]
#[case(
    "base.py_function-names",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::FunctionNames),
)]
#[case(
    "base.py_function-calls",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::FunctionCalls),
)]
#[case(
    "base.py_class",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Class),
)]
#[case(
    "base.py_def",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Def),
)]
#[case(
    "base.py_async-def",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::AsyncDef),
)]
#[case(
    "base.py_methods",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Methods),
)]
#[case(
    "base.py_classmethods",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::ClassMethods),
)]
#[case(
    "base.py_staticmethods",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::StaticMethods),
)]
#[case(
    "base.py_with",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::With),
)]
#[case(
    "base.py_try",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Try),
)]
#[case(
    "base.py_lambda",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Lambda),
)]
#[case(
    "base.py_globals",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Globals),
)]
#[case(
    "base.py_variable_identifiers",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::VariableIdentifiers),
)]
#[case(
    "base.py_types",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Types),
)]
#[case(
    "base.py_identifiers",
    include_str!("python/base.py"),
    python::CompiledQuery::from(python::PreparedQuery::Identifiers),
)]
#[case(
    "identifiers.py_identifiers",
    include_str!("python/identifiers.py"),
    python::CompiledQuery::from(python::PreparedQuery::Identifiers),
)]
#[case(
    "base.ts_strings",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Strings),
)]
#[case(
    "base.ts_comments",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Comments),
)]
#[case(
    "base.ts_imports",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Imports),
)]
#[case(
    "base.ts_function",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Function),
)]
#[case(
    "base.ts_async-function",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::AsyncFunction),
)]
#[case(
    "base.ts_sync-function",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::SyncFunction),
)]
#[case(
    "base.ts_method",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Method),
)]
#[case(
    "base.ts_constructor",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Constructor),
)]
#[case(
    "base.ts_class",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Class),
)]
#[case(
    "base.ts_enum",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Enum),
)]
#[case(
    "base.ts_interface",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Interface),
)]
#[case(
    "base.ts_try-block",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::TryCatch),
)]
#[case(
    "base.ts_var_decl",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::VarDecl),
)]
#[case(
    "base.ts_let",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Let),
)]
#[case(
    "base.ts_const",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Const),
)]
#[case(
    "base.ts_var",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Var),
)]
#[case(
    "base.ts_type-params",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::TypeParams),
)]
#[case(
    "base.ts_type-alias",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::TypeAlias),
)]
#[case(
    "base.ts_namespace",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Namespace),
)]
#[case(
    "base.ts_export",
    include_str!("typescript/base.ts"),
    typescript::CompiledQuery::from(typescript::PreparedQuery::Export),
)]
#[case(
    "base.rs_strings",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Strings),
)]
#[case(
    "base.rs_comments",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Comments),
)]
#[case(
    "base.rs_uses",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Uses),
)]
#[case(
    "base.rs_doc-comments",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::DocComments),
)]
#[case(
    "base.rs_attribute",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Attribute),
)]
#[case(
    "base.rs_struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Struct),
)]
#[case(
    "base.rs_pub-struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubStruct),
)]
#[case(
    "base.rs_pub-priv-struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PrivStruct),
)]
#[case(
    "base.rs_pub-crate-struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubCrateStruct),
)]
#[case(
    "base.rs_pub-self-struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSelfStruct),
)]
#[case(
    "base.rs_pub-super-struct",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSuperStruct),
)]
#[case(
    "base.enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Enum),
)]
#[case(
    "base.rs_pub-priv-enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PrivEnum),
)]
#[case(
    "base.rs_pub-enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubEnum),
)]
#[case(
    "base.rs_pub-crate-enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubCrateEnum),
)]
#[case(
    "base.rs_pub-self-enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSelfEnum),
)]
#[case(
    "base.rs_pub-super-enum",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSuperEnum),
)]
#[case(
    "base.rs_enum-variant",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::EnumVariant),
)]
#[case(
    "base.rs_fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Fn),
)]
#[case(
    "base.rs_impl-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ImplFn),
)]
#[case(
    "base.rs_pub-priv-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PrivFn),
)]
#[case(
    "base.rs_pub-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubFn),
)]
#[case(
    "base.rs_pub-crate-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubCrateFn),
)]
#[case(
    "base.rs_pub-self-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSelfFn),
)]
#[case(
    "base.rs_pub-super-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::PubSuperFn),
)]
#[case(
    "base.rs_const-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ConstFn),
)]
#[case(
    "base.rs_async-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::AsyncFn),
)]
#[case(
    "base.rs_unsafe-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::UnsafeFn),
)]
#[case(
    "base.rs_extern-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ExternFn),
)]
#[case(
    "base.rs_test-fn",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::TestFn),
)]
#[case(
    "base.rs_trait",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Trait),
)]
#[case(
    "base.rs_impl",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Impl),
)]
#[case(
    "base.rs_impl-trait",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ImplTrait),
)]
#[case(
    "base.rs_impl-type",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ImplType),
)]
#[case(
    "base.rs_mod",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Mod),
)]
#[case(
    "base.rs_mod-tests",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::ModTests),
)]
#[case(
    "base.rs_typedefs",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::TypeDef),
)]
#[case(
    "base.rs_identifier",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Identifier),
)]
#[case(
    "base.rs_type-identifier",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::TypeIdentifier),
)]
#[case(
    "base.rs_closure",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Closure),
)]
#[case(
    "base.rs_unsafe",
    include_str!("rust/base.rs"),
    rust::CompiledQuery::from(rust::PreparedQuery::Unsafe),
)]
#[case(
    "base.tf_variable-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Variable),
)]
#[case(
    "base.tf_resource-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Resource),
)]
#[case(
    "base.tf_data-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Data),
)]
#[case(
    "base.tf_output-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Output),
)]
#[case(
    "base.tf_provider-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Provider),
)]
#[case(
    "base.tf_terraform-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Terraform),
)]
#[case(
    "base.tf_locals-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Locals),
)]
#[case(
    "base.tf_module-block",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Module),
)]
#[case(
    "base.tf_variables",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Variables),
)]
#[case(
    "base.tf_resource-types",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::ResourceTypes),
)]
#[case(
    "base.tf_resource-names",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::ResourceNames),
)]
#[case(
    "base.tf_data-names",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::DataNames),
)]
#[case(
    "base.tf_data-sources",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::DataSources),
)]
#[case(
    "base.tf_comments",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Comments),
)]
#[case(
    "base.tf_strings",
    include_str!("hcl/base.tf"),
    hcl::CompiledQuery::from(hcl::PreparedQuery::Strings),
)]
#[case(
    "base.go_comments",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Comments),
)]
#[case(
    "base.go_strings",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Strings),
)]
#[case(
    "base.go_imports",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Imports),
)]
#[case(
    "base.go_type-def",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::TypeDef),
)]
#[case(
    "base.go_type-alias",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::TypeAlias),
)]
#[case(
    "base.go_struct",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Struct),
)]
#[case(
    "base.go_interface",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Interface),
)]
#[case(
    "base.go_const",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Const),
)]
#[case(
    "base.go_var",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Var),
)]
#[case(
    "base.go_func",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Func),
)]
#[case(
    "base.go_method",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Method),
)]
#[case(
    "base.go_free-func",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::FreeFunc),
)]
#[case(
    "base.go_init-func",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::InitFunc),
)]
#[case(
    "base.go_type-params",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::TypeParams),
)]
#[case(
    "base.go_defer",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Defer),
)]
#[case(
    "base.go_select",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Select),
)]
#[case(
    "base.go_go",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Go),
)]
#[case(
    "base.go_switch",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Switch),
)]
#[case(
    "base.go_labeled",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Labeled),
)]
#[case(
    "base.go_goto",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::Goto),
)]
#[case(
    "base.go_struct-tags",
    include_str!("go/base.go"),
    go::CompiledQuery::from(go::PreparedQuery::StructTags),
)]
#[case(
    "base.cs_strings",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Strings),
)]
#[case(
    "base.cs_usings",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Usings),
)]
#[case(
    "base.cs_comments",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Comments),
)]
#[case(
    "base.cs_struct",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Struct),
)]
#[case(
    "base.cs_enum",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Enum),
)]
#[case(
    "base.cs_field",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Field),
)]
#[case(
    "base.cs_attribute",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Attribute),
)]
#[case(
    "base.cs_interface",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Interface),
)]
#[case(
    "base.cs_class",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Class),
)]
#[case(
    "base.cs_variable_declaration",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::VariableDeclaration),
)]
#[case(
    "base.cs_property",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Property),
)]
#[case(
    "base.cs_constructor",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Constructor),
)]
#[case(
    "base.cs_destructor",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Destructor),
)]
#[case(
    "base.cs_method",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Method),
)]
#[case(
    "base.cs_identifier",
    include_str!("csharp/base.cs"),
    csharp::CompiledQuery::from(csharp::PreparedQuery::Identifier),
)]
#[case(
    "base.c_comments",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Comments),
)]
#[case(
    "base.c_strings",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Strings),
)]
#[case(
    "base.c_includes",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Includes),
)]
#[case(
    "base.c_typedefs",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::TypeDef),
)]
#[case(
    "base.c_enum",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Enum),
)]
#[case(
    "base.c_struct",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Struct),
)]
#[case(
    "base.c_variable",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Variable),
)]
#[case(
    "base.c_function",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Function),
)]
#[case(
    "base.c_function_definition",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::FunctionDef),
)]
#[case(
    "base.c_function_declaration",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::FunctionDecl),
)]
#[case(
    "base.c_switch",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Switch),
)]
#[case(
    "base.c_if",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::If),
)]
#[case(
    "base.c_for",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::For),
)]
#[case(
    "base.c_while",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::While),
)]
#[case(
    "base.c_do",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Do),
)]
#[case(
    "base.c_union",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Union),
)]
#[case(
    "base.c_identifier",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Identifier),
)]
#[case(
    "base.c_declaration",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::Declaration),
)]
#[case(
    "base.c_callexpr",
    include_str!("c/base.c"),
   c::CompiledQuery::from (c::PreparedQuery::CallExpression),
)]
fn test_queries(#[case] snapshot_name: &str, #[case] contents: &str, #[case] query: impl Query) {
    let mut builder = ScopedViewBuilder::new(contents);
    builder.explode(&query);
    let view = builder.build();

    // Collect only those lines which are in scope. This avoids enormous clutter from
    // out of scope items. The in-scope lines are *pinned* by lines numbers and
    // highlights/their span, so false-positives and confusion around multiple matches
    // per line are avoided.
    let inscope_parts: Vec<InScopeLinePart> = view
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let mut start = 0;

            line.scopes()
                .0
                .clone()
                .into_iter()
                .filter_map(move |scope| {
                    let contents: &str = (&scope).into();
                    let end = start + contents.len();
                    let span = start..end;
                    let part = InScopeLinePart::new(i + 1, line.to_string(), contents, span);
                    start = end;

                    match scope.0 {
                        Scope::In(..) => Some(part),
                        Scope::Out(..) => None,
                    }
                })
        })
        .collect();

    insta::assert_yaml_snapshot!(snapshot_name, inscope_parts);
}
