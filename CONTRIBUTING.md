# Contributing

For local development, there isn't much to prepare:

1. Refer to the [README](README.md#cargo-compile-from-source) to see how to build from
   source.
2. Optionally, set up
   [`pre-commit`](https://pre-commit.com/#3-install-the-git-hook-scripts) for the repo:

   ```bash
   pre-commit install
   ```

   Its main function is to shorten feedback cycles for issues CI would eventually, but
   much later, fail on.
3. When adding new snapshot tests, run [`insta`](https://crates.io/crates/cargo-insta)
   like

   ```bash
   cargo insta test || cargo insta review
   ```

   to generate and review new and existing snapshots. Use `cargo insta test
   --unreferenced=delete` to remove any junk snapshots.
4. You will need a [nightly
   toolchain](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)
   available, as some development (but not build) tooling requires it:
   - [`rustfmt`](./rustfmt.toml)

## Adding support for a new language

1. Ensure support for your language exists: search crates.io for
   `tree-sitter-your-language`. For example,
   <https://crates.io/crates/tree-sitter-python> allows `srgn` to support Python.
2. Come up with queries you would like to see supported for your language. Obvious items
   are "all comments" or "strings", which make sense as a baseline and are supported by
   virtually all languages. Feel free to go wild. An example for a more involved query
   is for [Python's
   `staticmethod`s](https://github.com/alexpovel/srgn/blob/da2580a85c2101e91889519fcba11e876f865249/src/langs/python.rs#L148-L157).
   These more complex queries are what can make `srgn` more uniquely useful (offer
   functionality otherwise hardly attainable). The README contains
   [guidance](./README.md#custom-queries) on how get started writing these queries.
3. Other than that, follow previous work, e.g. the excellent PR at
   <https://github.com/alexpovel/srgn/pull/116>
   (da2580a85c2101e91889519fcba11e876f865249). It showcases where to add tests, for
   example.
