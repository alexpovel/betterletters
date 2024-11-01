//! Output of `tree-sitter generate` (`tree-sitter` version 0.22.5) inside of
//! <https://github.com/tree-sitter-grammars/tree-sitter-hcl>, see also
//! <https://tree-sitter.github.io/tree-sitter/creating-parsers#command-generate>. The
//! resulting `bindings/rust/lib.rs` is the below code, slimmed down to only what's
//! strictly needed.
//!
//! **Remove this module once <https://github.com/tree-sitter-grammars/tree-sitter-hcl>
//! is available on <https://crates.io> and updated to a high enough `tree-sitter`
//! version**.

extern "C" {
    fn tree_sitter_hcl() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    #[allow(unsafe_code)]
    unsafe {
        tree_sitter_hcl()
    }
}
