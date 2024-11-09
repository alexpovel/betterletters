# Changelog

## [0.14.0](https://github.com/alexpovel/srgn/compare/srgn-v0.13.4...srgn-v0.14.0) (2024-11-09)


### ⚠ BREAKING CHANGES

* `grep`-like, recursive search mode
* Update `tree-sitter` & bindings
* Adjust `IGNORE` pattern
* Variables for replacement action
* `Ranges`
* Remove `Debug` implementation of `dyn Scoper`
* Remove `Replacement::new`, force going through unescaping via `TryFrom<String>`
* Make `Replacement` a newtype
* Panic upon creation of inconsistent view
* `explode` takes `&mut self`, add all remaining public docs
* Simplify `explode` (no more `explode_from_scoper`), improve docs
* View-related items into view module
* scopes into new module
* Simplify crate features, fix existing cfgs
* Make `R{O,W}Scope{,s}` a newtype so it can take `impl`s
* Unify `ScopedView::map` through `Action` trait

### Features

* `fail-any` and `fail-none` feature flags ([1dd3dcb](https://github.com/alexpovel/srgn/commit/1dd3dcb3b233d787abc91622ddc7ce019c764878))
* `grep`-like, recursive search mode ([d55b28f](https://github.com/alexpovel/srgn/commit/d55b28fd9e266545d31d679c92b91e28efee4769))
* `IGNORE` parts of matches ([21b8dde](https://github.com/alexpovel/srgn/commit/21b8dde8744b3450e311b18778ef1321c573c3f6))
* `Ranges` ([bd8b0bc](https://github.com/alexpovel/srgn/commit/bd8b0bc0b96afe5ba3e0632c5ff51d0a6842e8aa))
* **c#:** Scope `class` definitions ([f65137d](https://github.com/alexpovel/srgn/commit/f65137d4779501953ae613ea8626bca797cf6709))
* **c#:** Scope `enum` definitions ([31dc2cb](https://github.com/alexpovel/srgn/commit/31dc2cb48c6a46b4e1f51a5af87c3b83ebac7299))
* **c#:** Scope `interface` definitions ([338b5f4](https://github.com/alexpovel/srgn/commit/338b5f47fbe2afc4536a772f9384742dae7e76cb))
* **c#:** Scope `struct` definitions ([5b53286](https://github.com/alexpovel/srgn/commit/5b5328683596195fefd5a248c65093f2b12e864d))
* **c#:** Scope attribute names ([c3fe051](https://github.com/alexpovel/srgn/commit/c3fe0518fa3b823ec6a3b9656d791570944688e0))
* **c#:** Scope constructor definitions ([d8b5e7a](https://github.com/alexpovel/srgn/commit/d8b5e7af6467f9d0eb5e7c9ad20b7ba6de74ff79))
* **c#:** Scope destructor definitions ([6f9677b](https://github.com/alexpovel/srgn/commit/6f9677b9ba3581665527cfaeb63572d7e701b173))
* **c#:** Scope field definitions ([3f2d919](https://github.com/alexpovel/srgn/commit/3f2d91933934fb7b78260b3eb3c3381e7954dd39))
* **c#:** Scope identifier names ([0267196](https://github.com/alexpovel/srgn/commit/02671965d9f46450fcdaa5744ef0cda48559399f))
* **c#:** Scope method definitions ([5b530a1](https://github.com/alexpovel/srgn/commit/5b530a1bf38fae0fedde0958fbd7bdcc7528a16e))
* **c#:** Scope property definitions ([769ffef](https://github.com/alexpovel/srgn/commit/769ffef0ec0bc8823135c83fbaa51ffa3b5ed8ce))
* **c#:** Scope variable declarations ([09879ea](https://github.com/alexpovel/srgn/commit/09879ea4fbdd5f5833f8fd88a910d3a3bf8d049b))
* Child options and flags imply their parent ([20518c2](https://github.com/alexpovel/srgn/commit/20518c2ea8ce62bbef3c97663d93446cdaaf8d56))
* **c:** introduce C parser ([da2580a](https://github.com/alexpovel/srgn/commit/da2580a85c2101e91889519fcba11e876f865249))
* **ci:** Provide binaries (x86/64bit for macOS, Linux, Windows) ([f4c009f](https://github.com/alexpovel/srgn/commit/f4c009fe0002e3944ebcf79183f134ceaf4f936e))
* **ci:** Test installation via binstall ([ecc35b4](https://github.com/alexpovel/srgn/commit/ecc35b43fd322fe3d24cc43ae58411d66b6fc46f))
* **cli:** `--&lt;LANG&gt;-query-file` option for reading queries from files ([#168](https://github.com/alexpovel/srgn/issues/168)) ([4967356](https://github.com/alexpovel/srgn/commit/496735690cb9f5056b416056a6e7b47865b0532e))
* **cli:** `-j` aka `--join-language-scopes` flag ([2c1b9e8](https://github.com/alexpovel/srgn/commit/2c1b9e80f680b041cdf08d96be0d56de48758a8d))
* **cli:** Dry run mode ( `--dry-run` flag) ([#156](https://github.com/alexpovel/srgn/issues/156)) ([c2a6d2a](https://github.com/alexpovel/srgn/commit/c2a6d2aadb11ab3a5d74c53ba3ede688eb5e6cdb))
* **cli:** Language shorthands (`--py` for `--python` etc.) ([f93093a](https://github.com/alexpovel/srgn/commit/f93093a7a5cfc214fa1d715d06642780a37f9faa))
* CSharp support (comments) and `UserService.cs` sample ([1cef201](https://github.com/alexpovel/srgn/commit/1cef20113cad889058c00c024fca59ba2a8b5507))
* File globbing and processing ([a8d330c](https://github.com/alexpovel/srgn/commit/a8d330c78ba0275909fa471fddc8f58e68181c83))
* Flag for failing on empty glob ([9d2fd0a](https://github.com/alexpovel/srgn/commit/9d2fd0a71e0b39043895a65e9674c0819b27440b))
* Go language (w/ comments, strings, struct tags) ([fe91428](https://github.com/alexpovel/srgn/commit/fe914281be8d6ad315238ab1fd1a5b9a11722227))
* **go:** Ignore file paths containing `vendor` ([0be56d0](https://github.com/alexpovel/srgn/commit/0be56d0c08d0c33f32d0f6a17965365415bdb5f2))
* **go:** Scope (any) type definitions ([bf2e90c](https://github.com/alexpovel/srgn/commit/bf2e90c8f5586f97a3a08aeeb47d4cce6c271436))
* **go:** Scope `const` assignments/specifications ([61232b9](https://github.com/alexpovel/srgn/commit/61232b9ce2d59dfad76bf7a057fcff42071b1172))
* **go:** Scope `defer` blocks ([76a91b8](https://github.com/alexpovel/srgn/commit/76a91b874ec5ed651b27253951a85151d68cc1ce))
* **go:** Scope `func init()` definitions ([106c4a6](https://github.com/alexpovel/srgn/commit/106c4a61ff16ade9d3a10a74c41255b541956a37))
* **go:** Scope `func` definitions (all, free, methods) ([707e95a](https://github.com/alexpovel/srgn/commit/707e95a498f307518777dd3f8c2ee4ce6fc97614))
* **go:** Scope `go` blocks ([c38c5c3](https://github.com/alexpovel/srgn/commit/c38c5c37a69d94a30c6779aecb1323c3076d071a))
* **go:** Scope `goto` statements ([4133575](https://github.com/alexpovel/srgn/commit/413357540400e2a6c428cea6665d7a87638e2328))
* **go:** Scope `select` blocks ([ddbf9d9](https://github.com/alexpovel/srgn/commit/ddbf9d9e84f5cd80b2d23056282a89f84c7b2d7b))
* **go:** Scope `struct` and `interface` type definitions ([392330f](https://github.com/alexpovel/srgn/commit/392330fbfc3bcd69e0dcda749f091030b4913aa2))
* **go:** Scope `switch` blocks ([61f5e08](https://github.com/alexpovel/srgn/commit/61f5e085d7bcb32d6aba09355d1c3a09a8418f44))
* **go:** Scope `type` aliases ([a1d707a](https://github.com/alexpovel/srgn/commit/a1d707a6b809fb9db21ff1997c176cc26e6082d1))
* **go:** Scope `var` assignments/specifications ([367191d](https://github.com/alexpovel/srgn/commit/367191db427a31a05410f7fb9952c45b302db9f9))
* **go:** Scope labeled statements ([308e28f](https://github.com/alexpovel/srgn/commit/308e28f1e2f52f6e3557adac449656218f481da0))
* **go:** Scope type aka generic parameters ([891aa11](https://github.com/alexpovel/srgn/commit/891aa118fafc54070a04ef570d9d1f885c201e19))
* HCL (HashiCorp Configuration Language) ([814a592](https://github.com/alexpovel/srgn/commit/814a592dbc3e446c6751bc2ab40b9e83337c726b))
* **hcl:** Scope `data` blocks ([dc38287](https://github.com/alexpovel/srgn/commit/dc3828760e01fb0f258cea393ccb84ba1073cd9e))
* **hcl:** Scope `locals` blocks ([c22c475](https://github.com/alexpovel/srgn/commit/c22c4757aa113d16453bdc59aed15bfebe3f6d9e))
* **hcl:** Scope `module` blocks ([84965ed](https://github.com/alexpovel/srgn/commit/84965ed82d7e0133e29bd741311dfe48050a613a))
* **hcl:** Scope `output` blocks ([9627961](https://github.com/alexpovel/srgn/commit/9627961efc444c1dd3d8bc0c70d68fbeeda5525d))
* **hcl:** Scope `provider` blocks ([a77e603](https://github.com/alexpovel/srgn/commit/a77e6037bfe6cdf5bbe648652059db7d4549ddd4))
* **hcl:** Scope `resource` blocks ([963d9a4](https://github.com/alexpovel/srgn/commit/963d9a44721144eae166e9846bda4232f3329cec))
* **hcl:** Scope `terraform` blocks ([a60a754](https://github.com/alexpovel/srgn/commit/a60a754465462ffb6b9fce5b5adc749357ff547a))
* **hcl:** Scope `variable` blocks ([6b8dcdc](https://github.com/alexpovel/srgn/commit/6b8dcdc59a438b459623395524a761d56d7e43ac))
* Implement string interpolation handling ([2f37b2e](https://github.com/alexpovel/srgn/commit/2f37b2e4f15c7ef7ef417b5fee65c6b20448933f))
* **language-scoping:** Specify _multiple_ language scopes ([ce0db6f](https://github.com/alexpovel/srgn/commit/ce0db6fec95c6b64e022a2292ba84c976ecf5750)), closes [#104](https://github.com/alexpovel/srgn/issues/104)
* Make `german`-only dependencies `optional` ([b407b67](https://github.com/alexpovel/srgn/commit/b407b67b03896691123c3ae763b5ed64458cfc59))
* Provide ass. functions on view for all actions ([ca52905](https://github.com/alexpovel/srgn/commit/ca529057fef2e50db49148b6914eb292ee9ac755))
* Python strings ([f452b01](https://github.com/alexpovel/srgn/commit/f452b01fb7e05b0d8e54a2d01c23e75ae998f90f))
* **python:** Scope `lambda`s ([94894c0](https://github.com/alexpovel/srgn/commit/94894c06993595c6795c7a89622d72197e02aae3))
* **python:** Scope `try` blocks ([107d87f](https://github.com/alexpovel/srgn/commit/107d87f12103ee429fa05d1bd53e7ddd4b4dfcb8))
* **python:** Scope `with` blocks ([b0f9825](https://github.com/alexpovel/srgn/commit/b0f9825dcaa5eb288f025cdaa99dd6404695953d))
* **python:** Scope async function definitions (`async def`) ([4debfff](https://github.com/alexpovel/srgn/commit/4debfff76351dd504750d321861dbed8648f70ec))
* **python:** Scope classmethods (`[@classmethod](https://github.com/classmethod) def` inside `class`) ([4779d69](https://github.com/alexpovel/srgn/commit/4779d695038059fd9f01bf32e2c8074dd0064ada))
* **python:** Scope function definitions (`def`) ([10ef4d5](https://github.com/alexpovel/srgn/commit/10ef4d5caddeafe11aa5ed6a1747748bd8444ef2))
* **python:** Scope global aka module-level variable (assignments) ([fc5c027](https://github.com/alexpovel/srgn/commit/fc5c027fd19537fa45a25ba35ef6e8f3031a0dfe))
* **python:** Scope methods (`def` inside `class`) ([e151d9a](https://github.com/alexpovel/srgn/commit/e151d9a7a84cdd5248d085969eafe88c45fd55e2))
* **python:** Scope staticmethods (`[@staticmethod](https://github.com/staticmethod) def` inside `class`) ([8f53aa5](https://github.com/alexpovel/srgn/commit/8f53aa59d025eca863398df34e8031941651105f))
* **python:** Scope type hints ([5dc106f](https://github.com/alexpovel/srgn/commit/5dc106f5721c8382870744bf1f1a8b8d65e5f3e4))
* **python:** Scope variable names (from their assignment) ([0fb549c](https://github.com/alexpovel/srgn/commit/0fb549ca64f804cac964df894b852565003a13a1))
* query for C# strings ([f38136c](https://github.com/alexpovel/srgn/commit/f38136c3bf56909fdc1a9f8520cae46b1c3ea87a))
* query for TypeScript strings ([37de0d4](https://github.com/alexpovel/srgn/commit/37de0d4989e5751c05419be9ab16c4ff46ac8f0c))
* Rust language (comments, doc comments, strings) ([f8910c8](https://github.com/alexpovel/srgn/commit/f8910c8c71f7aa8a5178154bf6f11d96f1eddc5d))
* **rust:** Scope "private" (non-`pub`) function, `struct`, `enum` definitions ([fab1bc3](https://github.com/alexpovel/srgn/commit/fab1bc3820fadd64c73f0b7cf940ce1c128555ce))
* **rust:** Scope "test" functions (functions with any attribute containing `test`) ([1c264fb](https://github.com/alexpovel/srgn/commit/1c264fb04e85eff4f9973ef36182d9347a578ee2))
* **rust:** Scope `{,pub({crate,self,super})} enum` definitions ([019bacb](https://github.com/alexpovel/srgn/commit/019bacb607d398c0a0f5ae4db90cc68f59fcff3a))
* **rust:** Scope `{,pub({crate,self,super})} struct` definitions ([8c922bf](https://github.com/alexpovel/srgn/commit/8c922bfff87850f59bb5314bbd5caee73da15b04))
* **rust:** Scope `impl` blocks ([f96d0dd](https://github.com/alexpovel/srgn/commit/f96d0ddfddb8638b2b41d94c45d42dc41b8cb2fa))
* **rust:** Scope `impl` blocks for types and traits specifically ([d51290c](https://github.com/alexpovel/srgn/commit/d51290c1467e044a4abba27a183cefa5d6f08d60))
* **rust:** Scope `pub({crate,self,super})` function definitions ([0e90ed5](https://github.com/alexpovel/srgn/commit/0e90ed5a11baa817c03d109ed99376a986f2170b))
* **rust:** Scope `pub` function definitions ([2ebcd15](https://github.com/alexpovel/srgn/commit/2ebcd15718286a693be2218403105393b2a01c8c))
* **rust:** Scope `trait` definitions ([7a1ad3d](https://github.com/alexpovel/srgn/commit/7a1ad3db95d7e5ef1d39aa972cfd17fb9cf8d219))
* **rust:** Scope `unsafe` code ([c67ff9e](https://github.com/alexpovel/srgn/commit/c67ff9e2eede9079dcdbe63c6c7ecca91ed9fd5a))
* **rust:** Scope attributes ([b9c03a3](https://github.com/alexpovel/srgn/commit/b9c03a3484c5bd4eaaf27cbc03c75afefcb65ab7))
* **rust:** Scope closure definitions ([83473c3](https://github.com/alexpovel/srgn/commit/83473c3dcb34c58ebc5823f230b34c611b294410))
* **rust:** Scope contents of `mod tests` blocks ([fc0a9db](https://github.com/alexpovel/srgn/commit/fc0a9db66895f5796d7589aa4511b6e68e670def))
* **rust:** Scope contents of `mod` blocks ([08278bc](https://github.com/alexpovel/srgn/commit/08278bc6dce7c0ebe1893aa8d3e3e1e3a7e9c915))
* **rust:** Scope function definitions ([c2dd0c3](https://github.com/alexpovel/srgn/commit/c2dd0c31324ed6f2c81a72074eb20e9aa57b1e82))
* **rust:** Scope function definitions marked `async` ([156f34c](https://github.com/alexpovel/srgn/commit/156f34ce03dc963205312a0967b4cfba7bb27c7b))
* **rust:** Scope function definitions marked `const` ([0cd4ac4](https://github.com/alexpovel/srgn/commit/0cd4ac457c14dbba10e1e198b07696e59e7c8399))
* **rust:** Scope function definitions marked `extern` ([fd08b7d](https://github.com/alexpovel/srgn/commit/fd08b7d0a927198372d88b3b1f6995ebb5e1d596))
* **rust:** Scope function definitions marked `unsafe` ([c54fe4d](https://github.com/alexpovel/srgn/commit/c54fe4d920c45a3bb85b6d064235b5bf7bee3413))
* **rust:** Scope functions inside `impl` blocks ([e9c3c16](https://github.com/alexpovel/srgn/commit/e9c3c16db652777e567bdf644802de7fc329031c))
* **rust:** Scope identifiers ([007f0be](https://github.com/alexpovel/srgn/commit/007f0be7483eba79716d2a855d16543d49dbd4d5))
* **rust:** Scope individual variants of enums ([07f4eb9](https://github.com/alexpovel/srgn/commit/07f4eb96f686d785f4c0fa626bcca538985c40d1))
* **rust:** Scope type definitions (`struct`, `enum`, `union`) ([33fc03b](https://github.com/alexpovel/srgn/commit/33fc03b919d52e80cb45c923fdfb6f2e801559db))
* **rust:** Scope type identifiers ([4caefdb](https://github.com/alexpovel/srgn/commit/4caefdb3fda70f03c65ef9fff387c4be8df0cbe8))
* Scope `using` namespace names (C#) ([200d482](https://github.com/alexpovel/srgn/commit/200d482663128ceed6f6d4153dc083a94b5e68c4))
* Scope import module names (TypeScript) ([b211204](https://github.com/alexpovel/srgn/commit/b2112048a451bb02119532818d84ca30fb6e0f10))
* Scope import names (Go) ([9b76ce6](https://github.com/alexpovel/srgn/commit/9b76ce6dc6f7d70c6da4f3bf29d68e858c0b4434))
* Scope module names in imports (Python) ([b3345c4](https://github.com/alexpovel/srgn/commit/b3345c46350092698b5ad2fce8f63e349544b2a9))
* Scope names in uses-declarations (Rust) ([cda850d](https://github.com/alexpovel/srgn/commit/cda850d59375a2b0b89c52f88a025a7eea839411))
* Scope Python identifiers ([d5932b8](https://github.com/alexpovel/srgn/commit/d5932b89adf6a49984b65884cc1e848a28154037))
* Shell completion scripts ([39bc6eb](https://github.com/alexpovel/srgn/commit/39bc6eb913040ee7748bc75c6252b3db399db694))
* Simplify `explode` (no more `explode_from_scoper`), improve docs ([ab0b914](https://github.com/alexpovel/srgn/commit/ab0b914175e9c0a04ba9c00bc72843abb949e733))
* TypeScript, with TODO app example ([3d3ed21](https://github.com/alexpovel/srgn/commit/3d3ed21582cc91d76e62bc8729dbeb38f70ebfb9))
* **typescript:** Scope `class` definitions ([f5c3b6f](https://github.com/alexpovel/srgn/commit/f5c3b6f09dae4b5369159b413f6a60241c62ea96))
* **typescript:** Scope `constructor` method definitions ([afafbbf](https://github.com/alexpovel/srgn/commit/afafbbffc46fcf60c772cc11440b355c039eb794))
* **typescript:** Scope `enum` definitions ([91afb9f](https://github.com/alexpovel/srgn/commit/91afb9f65a82e6e889137bd6a8f71ad97a99e2d8))
* **typescript:** Scope `export` blocks ([bb73080](https://github.com/alexpovel/srgn/commit/bb73080feec1e945d2b969783ed9d9408352cc0d))
* **typescript:** Scope `function` definitions (`async`, sync, or both) ([3edc8f0](https://github.com/alexpovel/srgn/commit/3edc8f027621e2f67234099c576a1e8a39544467))
* **typescript:** Scope `interface` definitions ([05f3257](https://github.com/alexpovel/srgn/commit/05f3257f8be622b79d7ffe13547eb77b885d7553))
* **typescript:** Scope `let`, `const`, `var` variable declarations ([9bd04d8](https://github.com/alexpovel/srgn/commit/9bd04d8de0d25baf56e8e7a409ee822a9a561a0c))
* **typescript:** Scope `namespace` blocks ([aeb5d41](https://github.com/alexpovel/srgn/commit/aeb5d4141e507c77ba9a158b58d3d4a8d96d9790))
* **typescript:** Scope `try`/`catch`/`finally` blocks ([4c19dd9](https://github.com/alexpovel/srgn/commit/4c19dd9305411e038ffdee51d1efbdf50609faf5))
* **typescript:** Scope `type` alias declarations ([ba3c726](https://github.com/alexpovel/srgn/commit/ba3c7268cf7c6bc098c9b262dbdde6fbcea02f38))
* **typescript:** Scope method definitions ([f5ddc52](https://github.com/alexpovel/srgn/commit/f5ddc52805852660e4b5eac4bb7169f860afde5e))
* **typescript:** Scope type parameters ([449d12e](https://github.com/alexpovel/srgn/commit/449d12ecc09fb7f8d813b42e967ba51ec206c16d))
* **typescript:** Scope variable declarations ([5be5abc](https://github.com/alexpovel/srgn/commit/5be5abc617b4c3d96fe80ab3db9a2d156042f33e))
* Unify `ScopedView::map` through `Action` trait ([f6ff38d](https://github.com/alexpovel/srgn/commit/f6ff38d98cfed7ff4bddde0abbadcee297649220))
* Variables for replacement action ([7f6cfcb](https://github.com/alexpovel/srgn/commit/7f6cfcbcef8f8d010de5b12df4d3e749b655d128))
* Warn when file does not look legit ([1593ec7](https://github.com/alexpovel/srgn/commit/1593ec77da670eee78a00981051f828fb611263b))
* Write names of processed files to stdout ([b42db4e](https://github.com/alexpovel/srgn/commit/b42db4ef29570934aed44f77f1e62242ba4f9b9f))


### Bug Fixes

* `fail-any`, `fail-none` and `fail-empty-glob` flags ([9673678](https://github.com/alexpovel/srgn/commit/9673678e127079c7f49cd57883d9025255cc2906))
* `tmp` directory for flaky test, instead of `git restore` ([2458b34](https://github.com/alexpovel/srgn/commit/2458b34fc27400e841b328ee3ee8fed51a4cf95f))
* **alloc:** Right(ish)-sized buffer allocations ([3f64519](https://github.com/alexpovel/srgn/commit/3f645198087af037ee922e3f80c11ee7ce53d870))
* **build:** binstall adjusted to release-please v4 ([6c81971](https://github.com/alexpovel/srgn/commit/6c81971bf9bbe3e04b3898034d7fff80b88be8bf))
* **build:** Preprocess German word list ([0590bef](https://github.com/alexpovel/srgn/commit/0590befd804d2c4be988c6e8d883155122d216d6))
* **build:** Vendor `tree-sitter-hcl` properly ([#107](https://github.com/alexpovel/srgn/issues/107)) ([95d346d](https://github.com/alexpovel/srgn/commit/95d346de69397a6a1f759d55ca99e5f0f410ea21))
* **ci:** (Try) (again) to fix bootstrapping release-please after bump to v4 ([d4ed8d3](https://github.com/alexpovel/srgn/commit/d4ed8d3cf0d29ef7a26d6247da702379349ab582))
* **ci:** (Try) to fix bootstrapping release-please ([8f82b7c](https://github.com/alexpovel/srgn/commit/8f82b7c4a2eb0f60a374bd4b45c42ef84ce4ff37))
* **ci:** Adjust GitHub token permissions for gh CLI ([243878b](https://github.com/alexpovel/srgn/commit/243878b7095770167cf781f01ed26f8ae67ad7f9))
* **ci:** Checkout and cache code coverage run ([938b7a5](https://github.com/alexpovel/srgn/commit/938b7a57b48b20a1b7797d4a02933f60fc017871))
* **ci:** Checkout before running gh CLI ([490d822](https://github.com/alexpovel/srgn/commit/490d8226eb4517937f61d5567966e888125445ce))
* **ci:** Code coverage using `tarpaulin`, by conditionally disabling `insta` ([6ace4fa](https://github.com/alexpovel/srgn/commit/6ace4fa476cad447060d76043c731c811b64629f))
* **ci:** Force bash shell on all OSs ([f34af16](https://github.com/alexpovel/srgn/commit/f34af16aa7455f26f8788200c0f1dfb39a077871))
* **ci:** Linking/cc fails for tarpaulin; try w/o cache ([ab40957](https://github.com/alexpovel/srgn/commit/ab409571417881a599c3fd32645dd26a5c9d8349))
* **ci:** Only build binaries on release creation ([6eaac7b](https://github.com/alexpovel/srgn/commit/6eaac7b1975b4139695eced6c1caa0aaf1bf59ae))
* **ci:** Provide credentials token to gh CLI ([1c9c21f](https://github.com/alexpovel/srgn/commit/1c9c21f6be6be7e9f0b9d0f5996df8e96c379ad2))
* **ci:** Provide empty but mandatory manifest ([167f0ac](https://github.com/alexpovel/srgn/commit/167f0acfb73463122e0b70552d9088bb1bafe4cb))
* **ci:** Run coverage test in parallel ([f4167d5](https://github.com/alexpovel/srgn/commit/f4167d594ba701b6ff54bc150f2e7c9eec017134))
* **ci:** Run release chores in parallel ([afca5cf](https://github.com/alexpovel/srgn/commit/afca5cf8d94d0dfc856b71f2fb9325a5bf5c7032))
* **ci:** Skip entire job, not just its single step ([9787306](https://github.com/alexpovel/srgn/commit/978730619eac7695b3891f53cb294817c3356cfc))
* **ci:** Switch from env var to native GitHub Actions variable ([816cc54](https://github.com/alexpovel/srgn/commit/816cc54f6d29299b8b39b28ee5b6ca69b0ecf0e4))
* **ci:** Trigger release-please ([505110a](https://github.com/alexpovel/srgn/commit/505110a6781bafdd4bb50210159f3d1f0cd90ab0))
* **ci:** Use bash for all platforms ([9fcb348](https://github.com/alexpovel/srgn/commit/9fcb34853f52b1ae7298e8e6836d69ebbccf1980))
* **ci:** Use cargo-binstall non-interactively ([78dbba9](https://github.com/alexpovel/srgn/commit/78dbba966761f0f2337dea602d3eb9832e819b79))
* **ci:** Windows binary extension and version string ([d93004b](https://github.com/alexpovel/srgn/commit/d93004b5775e110e803f5a4543ad53d10d98a32e))
* **cli:** Exit gracefully on broken pipe in search mode ([d47b901](https://github.com/alexpovel/srgn/commit/d47b901bff144482ed19fb44d5906a0c340a3e9b))
* **cli:** Ignore (do not abort on) non-utf8 files when walking recursively ([b3b9d6e](https://github.com/alexpovel/srgn/commit/b3b9d6ef2c12df90677fff2a64fb690296768092)), closes [#166](https://github.com/alexpovel/srgn/issues/166)
* **clippy:** `ignored_unit_patterns` ([4bc2827](https://github.com/alexpovel/srgn/commit/4bc28274e2578b968258f50f58618070fc0e8f5c)), closes [#35](https://github.com/alexpovel/srgn/issues/35)
* Debug representation of `escape_debug` ([dfc2e09](https://github.com/alexpovel/srgn/commit/dfc2e0937bde9283c0ea46b1f6b4703d26b47316))
* Do not `pub use` `Action` ([1e2c663](https://github.com/alexpovel/srgn/commit/1e2c663473f5c6140f065301f815f3cd3726837a))
* **docs:** Dead documentation symbol links ([ce3f900](https://github.com/alexpovel/srgn/commit/ce3f90015cdef9d53f67564d1281add3f1756762))
* **docs:** Escape quotes ([e248938](https://github.com/alexpovel/srgn/commit/e248938bb45d3a99b7122e945a8e3c91ab657b0a))
* **docs:** GitHub Markdown NOTE syntax error ([896129b](https://github.com/alexpovel/srgn/commit/896129b54a4799b12f65b7816e35209aabef5517))
* **docs:** Implement placeholder for custom query ([0844a99](https://github.com/alexpovel/srgn/commit/0844a99dbcf3bad02d41cdecd348ee915bc895eb))
* **docs:** In/Out was swapped for DosFix ([9d56346](https://github.com/alexpovel/srgn/commit/9d56346138d1e01b5ad1914078ef11e19dca287a))
* Feature-gated doc tests working properly ([a46e60d](https://github.com/alexpovel/srgn/commit/a46e60db9ef5bbbcb931bc4ff217b6459ce4c2e7))
* **files:** Writing new files works across devices ([1b27825](https://github.com/alexpovel/srgn/commit/1b278251f3aaa273b221f9520512a55ab04bd6cc)), closes [#145](https://github.com/alexpovel/srgn/issues/145)
* **go:** String scoping no longer scopes parts of imports/field decl. ([f4796c0](https://github.com/alexpovel/srgn/commit/f4796c0c5beded2de1a3c149afc6d59c9e8844f7))
* **hcl:** Check blocks for exact `identifier` `eq`uality ([1f26d56](https://github.com/alexpovel/srgn/commit/1f26d5640166b9ca5fdb7a8a33a5c3d6a4d8f8cb))
* **hcl:** Exclude `count` metavariable from {`resource`,`data`}-names ([6ff7a05](https://github.com/alexpovel/srgn/commit/6ff7a052b592181c3542f57f995366a5d2e5a58c))
* **hcl:** Scopes exclude quotes ([df30f9e](https://github.com/alexpovel/srgn/commit/df30f9e946f8007d7cb1d0e8dff6797b860db52f))
* **language-scoping:** Construct `TSQuery` only once ([084df95](https://github.com/alexpovel/srgn/commit/084df951e190a29ac03a6d16704d4fd8c997f1e0)), closes [#76](https://github.com/alexpovel/srgn/issues/76)
* Leftover error messages from debugging ([e03c110](https://github.com/alexpovel/srgn/commit/e03c1106ef8a0f4d58bf3a201c767523f114efe2))
* License for crates.io ([6c13a62](https://github.com/alexpovel/srgn/commit/6c13a62bb6f6ba99573494b9ec6a7bcd23bfff67))
* **logging:** Logs display timestamps again ([70ffd1c](https://github.com/alexpovel/srgn/commit/70ffd1c1615b9db56e9f760d4bfa18ca782f6614))
* MSRV ([ac6d744](https://github.com/alexpovel/srgn/commit/ac6d744601ff08b2f486335319343339e6440ed9))
* Panic on capture groups ([ea1aa08](https://github.com/alexpovel/srgn/commit/ea1aa086bcdbca52509bf2df36858c6cdd60cbd1)), closes [#71](https://github.com/alexpovel/srgn/issues/71)
* **python:** Scoping docstrings and strings no longer includes quotes ([2a743c8](https://github.com/alexpovel/srgn/commit/2a743c83f5b1a9b6fac4b99832c2d3f476f6569a))
* **rust:** `uses` scope only scopes things actually behind a `use` ([ea1a734](https://github.com/alexpovel/srgn/commit/ea1a734bd6b5c72115f7dd3aa618e04f8507b5cc))
* **rust:** `uses` scopes to its entire argument ([0ca45a1](https://github.com/alexpovel/srgn/commit/0ca45a1d323ee1b3522d5caeb5472863844a0446))
* **rust:** doc comments match `//!`-style ([a910e82](https://github.com/alexpovel/srgn/commit/a910e826d9f0405c367c03c51aef8d01234098fd))
* **rust:** Scoping strings no longer includes quotes ([8fb5da8](https://github.com/alexpovel/srgn/commit/8fb5da8cc782f30ecd6d412b628a8955edf26deb))
* Simplify crate features, fix existing cfgs ([af1b39d](https://github.com/alexpovel/srgn/commit/af1b39db705b9795933e3dd0716ecd3c8cbd5cac))
* Splitting of DOS-style line endings ([496337c](https://github.com/alexpovel/srgn/commit/496337c5d664db3df4a884f0050ee0d1357d8c2b))
* Squeezing now has `tr`-like `squeeze-repeats` alias ([4d67a45](https://github.com/alexpovel/srgn/commit/4d67a458171a237b3b782753e140a564ed7f84d2))
* **tests:** Remove `tarpaulin-incompatible` feature ([119bb13](https://github.com/alexpovel/srgn/commit/119bb136b0b4396629ee28b3daa82085978d29c4))
* **tests:** Tarpaulin config file ([ef1de6b](https://github.com/alexpovel/srgn/commit/ef1de6b9827cd70cbff4168f7acebb65af8a51de))
* Trace message about regex pattern ([6b67dfe](https://github.com/alexpovel/srgn/commit/6b67dfe36ce96316a3b4ddcc1400dcf68a1996d0))
* **typescript:** Scoping strings no longer includes quotes ([f1626d7](https://github.com/alexpovel/srgn/commit/f1626d7cee8aed4cce47349f741a427bdf37a944))


### Miscellaneous Chores

* `explode` takes `&mut self`, add all remaining public docs ([33097c1](https://github.com/alexpovel/srgn/commit/33097c149f855a4a09ca1aef6535833c0e9a016e)), closes [#6](https://github.com/alexpovel/srgn/issues/6)
* Adjust `IGNORE` pattern ([96d4d4c](https://github.com/alexpovel/srgn/commit/96d4d4cb4ac3e66eedd668260e5ab16e94dc9ae9))
* Make `R{O,W}Scope{,s}` a newtype so it can take `impl`s ([98b04d5](https://github.com/alexpovel/srgn/commit/98b04d5d7a1cb1a88ac8158cdaa0a2673f4e2114))
* Make `Replacement` a newtype ([59d6daf](https://github.com/alexpovel/srgn/commit/59d6daf505325cbe5238a9561fe2cb486cff0b64))
* Panic upon creation of inconsistent view ([ad6a38a](https://github.com/alexpovel/srgn/commit/ad6a38ae1214622d4baada9c7107e4c21c8aab67)), closes [#51](https://github.com/alexpovel/srgn/issues/51)
* Remove `Debug` implementation of `dyn Scoper` ([31ef135](https://github.com/alexpovel/srgn/commit/31ef135ce7df5b635653f40a67f9f7a96fd380af))
* Remove `Replacement::new`, force going through unescaping via `TryFrom&lt;String&gt;` ([2ec98c1](https://github.com/alexpovel/srgn/commit/2ec98c16a47198214a646312a7b7c3de81c6178d))
* scopes into new module ([e951347](https://github.com/alexpovel/srgn/commit/e9513470d62c5a2fc165c656d8a02c810b5bf2fb))
* Update `tree-sitter` & bindings ([5debd0e](https://github.com/alexpovel/srgn/commit/5debd0e1f029bd64ff150672ce6c2d7b5952f728))
* View-related items into view module ([18ef801](https://github.com/alexpovel/srgn/commit/18ef801577b345f24a7bba37909a4387e553bf13))

## [0.13.4](https://github.com/alexpovel/srgn/compare/srgn-v0.13.3...srgn-v0.13.4) (2024-11-09)


### Features

* **cli:** `--&lt;LANG&gt;-query-file` option for reading queries from files ([#168](https://github.com/alexpovel/srgn/issues/168)) ([4967356](https://github.com/alexpovel/srgn/commit/496735690cb9f5056b416056a6e7b47865b0532e))
* **cli:** Dry run mode ( `--dry-run` flag) ([#156](https://github.com/alexpovel/srgn/issues/156)) ([c2a6d2a](https://github.com/alexpovel/srgn/commit/c2a6d2aadb11ab3a5d74c53ba3ede688eb5e6cdb))


### Bug Fixes

* **alloc:** Right(ish)-sized buffer allocations ([3f64519](https://github.com/alexpovel/srgn/commit/3f645198087af037ee922e3f80c11ee7ce53d870))
* **cli:** Ignore (do not abort on) non-utf8 files when walking recursively ([b3b9d6e](https://github.com/alexpovel/srgn/commit/b3b9d6ef2c12df90677fff2a64fb690296768092)), closes [#166](https://github.com/alexpovel/srgn/issues/166)

## [0.13.3](https://github.com/alexpovel/srgn/compare/srgn-v0.13.2...srgn-v0.13.3) (2024-10-19)


### Bug Fixes

* **files:** Writing new files works across devices ([1b27825](https://github.com/alexpovel/srgn/commit/1b278251f3aaa273b221f9520512a55ab04bd6cc)), closes [#145](https://github.com/alexpovel/srgn/issues/145)

## [0.13.2](https://github.com/alexpovel/srgn/compare/srgn-v0.13.1...srgn-v0.13.2) (2024-09-27)


### Features

* **c:** introduce C parser ([da2580a](https://github.com/alexpovel/srgn/commit/da2580a85c2101e91889519fcba11e876f865249))
* **cli:** Language shorthands (`--py` for `--python` etc.) ([f93093a](https://github.com/alexpovel/srgn/commit/f93093a7a5cfc214fa1d715d06642780a37f9faa))
* **rust:** Scope `unsafe` code ([c67ff9e](https://github.com/alexpovel/srgn/commit/c67ff9e2eede9079dcdbe63c6c7ecca91ed9fd5a))
* Scope Python identifiers ([d5932b8](https://github.com/alexpovel/srgn/commit/d5932b89adf6a49984b65884cc1e848a28154037))

## [0.13.1](https://github.com/alexpovel/srgn/compare/srgn-v0.13.0...srgn-v0.13.1) (2024-08-18)


### Bug Fixes

* **build:** Vendor `tree-sitter-hcl` properly ([#107](https://github.com/alexpovel/srgn/issues/107)) ([95d346d](https://github.com/alexpovel/srgn/commit/95d346de69397a6a1f759d55ca99e5f0f410ea21))

## [0.13.0](https://github.com/alexpovel/srgn/compare/srgn-v0.12.0...srgn-v0.13.0) (2024-08-18)


### ⚠ BREAKING CHANGES

* `grep`-like, recursive search mode
* Update `tree-sitter` & bindings
* Adjust `IGNORE` pattern
* Variables for replacement action
* `Ranges`

### Features

* `grep`-like, recursive search mode ([d55b28f](https://github.com/alexpovel/srgn/commit/d55b28fd9e266545d31d679c92b91e28efee4769))
* `Ranges` ([bd8b0bc](https://github.com/alexpovel/srgn/commit/bd8b0bc0b96afe5ba3e0632c5ff51d0a6842e8aa))
* **c#:** Scope `class` definitions ([f65137d](https://github.com/alexpovel/srgn/commit/f65137d4779501953ae613ea8626bca797cf6709))
* **c#:** Scope `enum` definitions ([31dc2cb](https://github.com/alexpovel/srgn/commit/31dc2cb48c6a46b4e1f51a5af87c3b83ebac7299))
* **c#:** Scope `interface` definitions ([338b5f4](https://github.com/alexpovel/srgn/commit/338b5f47fbe2afc4536a772f9384742dae7e76cb))
* **c#:** Scope `struct` definitions ([5b53286](https://github.com/alexpovel/srgn/commit/5b5328683596195fefd5a248c65093f2b12e864d))
* **c#:** Scope attribute names ([c3fe051](https://github.com/alexpovel/srgn/commit/c3fe0518fa3b823ec6a3b9656d791570944688e0))
* **c#:** Scope constructor definitions ([d8b5e7a](https://github.com/alexpovel/srgn/commit/d8b5e7af6467f9d0eb5e7c9ad20b7ba6de74ff79))
* **c#:** Scope destructor definitions ([6f9677b](https://github.com/alexpovel/srgn/commit/6f9677b9ba3581665527cfaeb63572d7e701b173))
* **c#:** Scope field definitions ([3f2d919](https://github.com/alexpovel/srgn/commit/3f2d91933934fb7b78260b3eb3c3381e7954dd39))
* **c#:** Scope identifier names ([0267196](https://github.com/alexpovel/srgn/commit/02671965d9f46450fcdaa5744ef0cda48559399f))
* **c#:** Scope method definitions ([5b530a1](https://github.com/alexpovel/srgn/commit/5b530a1bf38fae0fedde0958fbd7bdcc7528a16e))
* **c#:** Scope property definitions ([769ffef](https://github.com/alexpovel/srgn/commit/769ffef0ec0bc8823135c83fbaa51ffa3b5ed8ce))
* **c#:** Scope variable declarations ([09879ea](https://github.com/alexpovel/srgn/commit/09879ea4fbdd5f5833f8fd88a910d3a3bf8d049b))
* **cli:** `-j` aka `--join-language-scopes` flag ([2c1b9e8](https://github.com/alexpovel/srgn/commit/2c1b9e80f680b041cdf08d96be0d56de48758a8d))
* **go:** Ignore file paths containing `vendor` ([0be56d0](https://github.com/alexpovel/srgn/commit/0be56d0c08d0c33f32d0f6a17965365415bdb5f2))
* **go:** Scope (any) type definitions ([bf2e90c](https://github.com/alexpovel/srgn/commit/bf2e90c8f5586f97a3a08aeeb47d4cce6c271436))
* **go:** Scope `const` assignments/specifications ([61232b9](https://github.com/alexpovel/srgn/commit/61232b9ce2d59dfad76bf7a057fcff42071b1172))
* **go:** Scope `defer` blocks ([76a91b8](https://github.com/alexpovel/srgn/commit/76a91b874ec5ed651b27253951a85151d68cc1ce))
* **go:** Scope `func init()` definitions ([106c4a6](https://github.com/alexpovel/srgn/commit/106c4a61ff16ade9d3a10a74c41255b541956a37))
* **go:** Scope `func` definitions (all, free, methods) ([707e95a](https://github.com/alexpovel/srgn/commit/707e95a498f307518777dd3f8c2ee4ce6fc97614))
* **go:** Scope `go` blocks ([c38c5c3](https://github.com/alexpovel/srgn/commit/c38c5c37a69d94a30c6779aecb1323c3076d071a))
* **go:** Scope `goto` statements ([4133575](https://github.com/alexpovel/srgn/commit/413357540400e2a6c428cea6665d7a87638e2328))
* **go:** Scope `select` blocks ([ddbf9d9](https://github.com/alexpovel/srgn/commit/ddbf9d9e84f5cd80b2d23056282a89f84c7b2d7b))
* **go:** Scope `struct` and `interface` type definitions ([392330f](https://github.com/alexpovel/srgn/commit/392330fbfc3bcd69e0dcda749f091030b4913aa2))
* **go:** Scope `switch` blocks ([61f5e08](https://github.com/alexpovel/srgn/commit/61f5e085d7bcb32d6aba09355d1c3a09a8418f44))
* **go:** Scope `type` aliases ([a1d707a](https://github.com/alexpovel/srgn/commit/a1d707a6b809fb9db21ff1997c176cc26e6082d1))
* **go:** Scope `var` assignments/specifications ([367191d](https://github.com/alexpovel/srgn/commit/367191db427a31a05410f7fb9952c45b302db9f9))
* **go:** Scope labeled statements ([308e28f](https://github.com/alexpovel/srgn/commit/308e28f1e2f52f6e3557adac449656218f481da0))
* **go:** Scope type aka generic parameters ([891aa11](https://github.com/alexpovel/srgn/commit/891aa118fafc54070a04ef570d9d1f885c201e19))
* HCL (HashiCorp Configuration Language) ([814a592](https://github.com/alexpovel/srgn/commit/814a592dbc3e446c6751bc2ab40b9e83337c726b))
* **hcl:** Scope `data` blocks ([dc38287](https://github.com/alexpovel/srgn/commit/dc3828760e01fb0f258cea393ccb84ba1073cd9e))
* **hcl:** Scope `locals` blocks ([c22c475](https://github.com/alexpovel/srgn/commit/c22c4757aa113d16453bdc59aed15bfebe3f6d9e))
* **hcl:** Scope `module` blocks ([84965ed](https://github.com/alexpovel/srgn/commit/84965ed82d7e0133e29bd741311dfe48050a613a))
* **hcl:** Scope `output` blocks ([9627961](https://github.com/alexpovel/srgn/commit/9627961efc444c1dd3d8bc0c70d68fbeeda5525d))
* **hcl:** Scope `provider` blocks ([a77e603](https://github.com/alexpovel/srgn/commit/a77e6037bfe6cdf5bbe648652059db7d4549ddd4))
* **hcl:** Scope `resource` blocks ([963d9a4](https://github.com/alexpovel/srgn/commit/963d9a44721144eae166e9846bda4232f3329cec))
* **hcl:** Scope `terraform` blocks ([a60a754](https://github.com/alexpovel/srgn/commit/a60a754465462ffb6b9fce5b5adc749357ff547a))
* **hcl:** Scope `variable` blocks ([6b8dcdc](https://github.com/alexpovel/srgn/commit/6b8dcdc59a438b459623395524a761d56d7e43ac))
* **language-scoping:** Specify _multiple_ language scopes ([ce0db6f](https://github.com/alexpovel/srgn/commit/ce0db6fec95c6b64e022a2292ba84c976ecf5750)), closes [#104](https://github.com/alexpovel/srgn/issues/104)
* **python:** Scope `lambda`s ([94894c0](https://github.com/alexpovel/srgn/commit/94894c06993595c6795c7a89622d72197e02aae3))
* **python:** Scope `try` blocks ([107d87f](https://github.com/alexpovel/srgn/commit/107d87f12103ee429fa05d1bd53e7ddd4b4dfcb8))
* **python:** Scope `with` blocks ([b0f9825](https://github.com/alexpovel/srgn/commit/b0f9825dcaa5eb288f025cdaa99dd6404695953d))
* **python:** Scope async function definitions (`async def`) ([4debfff](https://github.com/alexpovel/srgn/commit/4debfff76351dd504750d321861dbed8648f70ec))
* **python:** Scope classmethods (`[@classmethod](https://github.com/classmethod) def` inside `class`) ([4779d69](https://github.com/alexpovel/srgn/commit/4779d695038059fd9f01bf32e2c8074dd0064ada))
* **python:** Scope function definitions (`def`) ([10ef4d5](https://github.com/alexpovel/srgn/commit/10ef4d5caddeafe11aa5ed6a1747748bd8444ef2))
* **python:** Scope global aka module-level variable (assignments) ([fc5c027](https://github.com/alexpovel/srgn/commit/fc5c027fd19537fa45a25ba35ef6e8f3031a0dfe))
* **python:** Scope methods (`def` inside `class`) ([e151d9a](https://github.com/alexpovel/srgn/commit/e151d9a7a84cdd5248d085969eafe88c45fd55e2))
* **python:** Scope staticmethods (`[@staticmethod](https://github.com/staticmethod) def` inside `class`) ([8f53aa5](https://github.com/alexpovel/srgn/commit/8f53aa59d025eca863398df34e8031941651105f))
* **python:** Scope type hints ([5dc106f](https://github.com/alexpovel/srgn/commit/5dc106f5721c8382870744bf1f1a8b8d65e5f3e4))
* **python:** Scope variable names (from their assignment) ([0fb549c](https://github.com/alexpovel/srgn/commit/0fb549ca64f804cac964df894b852565003a13a1))
* **rust:** Scope "private" (non-`pub`) function, `struct`, `enum` definitions ([fab1bc3](https://github.com/alexpovel/srgn/commit/fab1bc3820fadd64c73f0b7cf940ce1c128555ce))
* **rust:** Scope "test" functions (functions with any attribute containing `test`) ([1c264fb](https://github.com/alexpovel/srgn/commit/1c264fb04e85eff4f9973ef36182d9347a578ee2))
* **rust:** Scope `{,pub({crate,self,super})} enum` definitions ([019bacb](https://github.com/alexpovel/srgn/commit/019bacb607d398c0a0f5ae4db90cc68f59fcff3a))
* **rust:** Scope `{,pub({crate,self,super})} struct` definitions ([8c922bf](https://github.com/alexpovel/srgn/commit/8c922bfff87850f59bb5314bbd5caee73da15b04))
* **rust:** Scope `impl` blocks ([f96d0dd](https://github.com/alexpovel/srgn/commit/f96d0ddfddb8638b2b41d94c45d42dc41b8cb2fa))
* **rust:** Scope `impl` blocks for types and traits specifically ([d51290c](https://github.com/alexpovel/srgn/commit/d51290c1467e044a4abba27a183cefa5d6f08d60))
* **rust:** Scope `pub({crate,self,super})` function definitions ([0e90ed5](https://github.com/alexpovel/srgn/commit/0e90ed5a11baa817c03d109ed99376a986f2170b))
* **rust:** Scope `pub` function definitions ([2ebcd15](https://github.com/alexpovel/srgn/commit/2ebcd15718286a693be2218403105393b2a01c8c))
* **rust:** Scope `trait` definitions ([7a1ad3d](https://github.com/alexpovel/srgn/commit/7a1ad3db95d7e5ef1d39aa972cfd17fb9cf8d219))
* **rust:** Scope attributes ([b9c03a3](https://github.com/alexpovel/srgn/commit/b9c03a3484c5bd4eaaf27cbc03c75afefcb65ab7))
* **rust:** Scope closure definitions ([83473c3](https://github.com/alexpovel/srgn/commit/83473c3dcb34c58ebc5823f230b34c611b294410))
* **rust:** Scope contents of `mod tests` blocks ([fc0a9db](https://github.com/alexpovel/srgn/commit/fc0a9db66895f5796d7589aa4511b6e68e670def))
* **rust:** Scope contents of `mod` blocks ([08278bc](https://github.com/alexpovel/srgn/commit/08278bc6dce7c0ebe1893aa8d3e3e1e3a7e9c915))
* **rust:** Scope function definitions ([c2dd0c3](https://github.com/alexpovel/srgn/commit/c2dd0c31324ed6f2c81a72074eb20e9aa57b1e82))
* **rust:** Scope function definitions marked `async` ([156f34c](https://github.com/alexpovel/srgn/commit/156f34ce03dc963205312a0967b4cfba7bb27c7b))
* **rust:** Scope function definitions marked `const` ([0cd4ac4](https://github.com/alexpovel/srgn/commit/0cd4ac457c14dbba10e1e198b07696e59e7c8399))
* **rust:** Scope function definitions marked `extern` ([fd08b7d](https://github.com/alexpovel/srgn/commit/fd08b7d0a927198372d88b3b1f6995ebb5e1d596))
* **rust:** Scope function definitions marked `unsafe` ([c54fe4d](https://github.com/alexpovel/srgn/commit/c54fe4d920c45a3bb85b6d064235b5bf7bee3413))
* **rust:** Scope functions inside `impl` blocks ([e9c3c16](https://github.com/alexpovel/srgn/commit/e9c3c16db652777e567bdf644802de7fc329031c))
* **rust:** Scope identifiers ([007f0be](https://github.com/alexpovel/srgn/commit/007f0be7483eba79716d2a855d16543d49dbd4d5))
* **rust:** Scope individual variants of enums ([07f4eb9](https://github.com/alexpovel/srgn/commit/07f4eb96f686d785f4c0fa626bcca538985c40d1))
* **rust:** Scope type definitions (`struct`, `enum`, `union`) ([33fc03b](https://github.com/alexpovel/srgn/commit/33fc03b919d52e80cb45c923fdfb6f2e801559db))
* **rust:** Scope type identifiers ([4caefdb](https://github.com/alexpovel/srgn/commit/4caefdb3fda70f03c65ef9fff387c4be8df0cbe8))
* **typescript:** Scope `class` definitions ([f5c3b6f](https://github.com/alexpovel/srgn/commit/f5c3b6f09dae4b5369159b413f6a60241c62ea96))
* **typescript:** Scope `constructor` method definitions ([afafbbf](https://github.com/alexpovel/srgn/commit/afafbbffc46fcf60c772cc11440b355c039eb794))
* **typescript:** Scope `enum` definitions ([91afb9f](https://github.com/alexpovel/srgn/commit/91afb9f65a82e6e889137bd6a8f71ad97a99e2d8))
* **typescript:** Scope `export` blocks ([bb73080](https://github.com/alexpovel/srgn/commit/bb73080feec1e945d2b969783ed9d9408352cc0d))
* **typescript:** Scope `function` definitions (`async`, sync, or both) ([3edc8f0](https://github.com/alexpovel/srgn/commit/3edc8f027621e2f67234099c576a1e8a39544467))
* **typescript:** Scope `interface` definitions ([05f3257](https://github.com/alexpovel/srgn/commit/05f3257f8be622b79d7ffe13547eb77b885d7553))
* **typescript:** Scope `let`, `const`, `var` variable declarations ([9bd04d8](https://github.com/alexpovel/srgn/commit/9bd04d8de0d25baf56e8e7a409ee822a9a561a0c))
* **typescript:** Scope `namespace` blocks ([aeb5d41](https://github.com/alexpovel/srgn/commit/aeb5d4141e507c77ba9a158b58d3d4a8d96d9790))
* **typescript:** Scope `try`/`catch`/`finally` blocks ([4c19dd9](https://github.com/alexpovel/srgn/commit/4c19dd9305411e038ffdee51d1efbdf50609faf5))
* **typescript:** Scope `type` alias declarations ([ba3c726](https://github.com/alexpovel/srgn/commit/ba3c7268cf7c6bc098c9b262dbdde6fbcea02f38))
* **typescript:** Scope method definitions ([f5ddc52](https://github.com/alexpovel/srgn/commit/f5ddc52805852660e4b5eac4bb7169f860afde5e))
* **typescript:** Scope type parameters ([449d12e](https://github.com/alexpovel/srgn/commit/449d12ecc09fb7f8d813b42e967ba51ec206c16d))
* **typescript:** Scope variable declarations ([5be5abc](https://github.com/alexpovel/srgn/commit/5be5abc617b4c3d96fe80ab3db9a2d156042f33e))
* Variables for replacement action ([7f6cfcb](https://github.com/alexpovel/srgn/commit/7f6cfcbcef8f8d010de5b12df4d3e749b655d128))


### Bug Fixes

* `fail-any`, `fail-none` and `fail-empty-glob` flags ([9673678](https://github.com/alexpovel/srgn/commit/9673678e127079c7f49cd57883d9025255cc2906))
* **cli:** Exit gracefully on broken pipe in search mode ([d47b901](https://github.com/alexpovel/srgn/commit/d47b901bff144482ed19fb44d5906a0c340a3e9b))
* **go:** String scoping no longer scopes parts of imports/field decl. ([f4796c0](https://github.com/alexpovel/srgn/commit/f4796c0c5beded2de1a3c149afc6d59c9e8844f7))
* **hcl:** Check blocks for exact `identifier` `eq`uality ([1f26d56](https://github.com/alexpovel/srgn/commit/1f26d5640166b9ca5fdb7a8a33a5c3d6a4d8f8cb))
* **hcl:** Exclude `count` metavariable from {`resource`,`data`}-names ([6ff7a05](https://github.com/alexpovel/srgn/commit/6ff7a052b592181c3542f57f995366a5d2e5a58c))
* **hcl:** Scopes exclude quotes ([df30f9e](https://github.com/alexpovel/srgn/commit/df30f9e946f8007d7cb1d0e8dff6797b860db52f))
* **language-scoping:** Construct `TSQuery` only once ([084df95](https://github.com/alexpovel/srgn/commit/084df951e190a29ac03a6d16704d4fd8c997f1e0)), closes [#76](https://github.com/alexpovel/srgn/issues/76)
* **logging:** Logs display timestamps again ([70ffd1c](https://github.com/alexpovel/srgn/commit/70ffd1c1615b9db56e9f760d4bfa18ca782f6614))
* **python:** Scoping docstrings and strings no longer includes quotes ([2a743c8](https://github.com/alexpovel/srgn/commit/2a743c83f5b1a9b6fac4b99832c2d3f476f6569a))
* **rust:** `uses` scope only scopes things actually behind a `use` ([ea1a734](https://github.com/alexpovel/srgn/commit/ea1a734bd6b5c72115f7dd3aa618e04f8507b5cc))
* **rust:** `uses` scopes to its entire argument ([0ca45a1](https://github.com/alexpovel/srgn/commit/0ca45a1d323ee1b3522d5caeb5472863844a0446))
* **rust:** doc comments match `//!`-style ([a910e82](https://github.com/alexpovel/srgn/commit/a910e826d9f0405c367c03c51aef8d01234098fd))
* **rust:** Scoping strings no longer includes quotes ([8fb5da8](https://github.com/alexpovel/srgn/commit/8fb5da8cc782f30ecd6d412b628a8955edf26deb))
* **typescript:** Scoping strings no longer includes quotes ([f1626d7](https://github.com/alexpovel/srgn/commit/f1626d7cee8aed4cce47349f741a427bdf37a944))


### Miscellaneous Chores

* Adjust `IGNORE` pattern ([96d4d4c](https://github.com/alexpovel/srgn/commit/96d4d4cb4ac3e66eedd668260e5ab16e94dc9ae9))
* Update `tree-sitter` & bindings ([5debd0e](https://github.com/alexpovel/srgn/commit/5debd0e1f029bd64ff150672ce6c2d7b5952f728))

## [0.12.0](https://github.com/alexpovel/srgn/compare/srgn-v0.11.0...srgn-v0.12.0) (2024-03-25)


### Features

* `IGNORE` parts of matches ([21b8dde](https://github.com/alexpovel/srgn/commit/21b8dde8744b3450e311b18778ef1321c573c3f6))


### Bug Fixes

* Debug representation of `escape_debug` ([dfc2e09](https://github.com/alexpovel/srgn/commit/dfc2e0937bde9283c0ea46b1f6b4703d26b47316))
* MSRV ([ac6d744](https://github.com/alexpovel/srgn/commit/ac6d744601ff08b2f486335319343339e6440ed9))
* Panic on capture groups ([ea1aa08](https://github.com/alexpovel/srgn/commit/ea1aa086bcdbca52509bf2df36858c6cdd60cbd1)), closes [#71](https://github.com/alexpovel/srgn/issues/71)
* Trace message about regex pattern ([6b67dfe](https://github.com/alexpovel/srgn/commit/6b67dfe36ce96316a3b4ddcc1400dcf68a1996d0))

## [0.11.0](https://github.com/alexpovel/srgn/compare/srgn-v0.10.2...srgn-v0.11.0) (2024-03-08)


### Features

* Shell completion scripts ([39bc6eb](https://github.com/alexpovel/srgn/commit/39bc6eb913040ee7748bc75c6252b3db399db694))


### Bug Fixes

* `tmp` directory for flaky test, instead of `git restore` ([2458b34](https://github.com/alexpovel/srgn/commit/2458b34fc27400e841b328ee3ee8fed51a4cf95f))
* **build:** Preprocess German word list ([0590bef](https://github.com/alexpovel/srgn/commit/0590befd804d2c4be988c6e8d883155122d216d6))
* **tests:** Remove `tarpaulin-incompatible` feature ([119bb13](https://github.com/alexpovel/srgn/commit/119bb136b0b4396629ee28b3daa82085978d29c4))
* **tests:** Tarpaulin config file ([ef1de6b](https://github.com/alexpovel/srgn/commit/ef1de6b9827cd70cbff4168f7acebb65af8a51de))

## [0.10.2](https://github.com/alexpovel/srgn/compare/srgn-v0.10.1...srgn-v0.10.2) (2024-01-27)


### Bug Fixes

* **build:** binstall adjusted to release-please v4 ([6c81971](https://github.com/alexpovel/srgn/commit/6c81971bf9bbe3e04b3898034d7fff80b88be8bf))

## [0.10.1](https://github.com/alexpovel/srgn/compare/srgn-v0.10.0...srgn-v0.10.1) (2024-01-01)


### Bug Fixes

* **ci:** (Try) (again) to fix bootstrapping release-please after bump to v4 ([d4ed8d3](https://github.com/alexpovel/srgn/commit/d4ed8d3cf0d29ef7a26d6247da702379349ab582))
* **ci:** (Try) to fix bootstrapping release-please ([8f82b7c](https://github.com/alexpovel/srgn/commit/8f82b7c4a2eb0f60a374bd4b45c42ef84ce4ff37))
* **ci:** Provide empty but mandatory manifest ([167f0ac](https://github.com/alexpovel/srgn/commit/167f0acfb73463122e0b70552d9088bb1bafe4cb))

## [0.10.0](https://github.com/alexpovel/srgn/compare/v0.9.0...v0.10.0) (2023-12-18)


### Features

* Scope `using` namespace names (C#) ([200d482](https://github.com/alexpovel/srgn/commit/200d482663128ceed6f6d4153dc083a94b5e68c4))
* Scope import module names (TypeScript) ([b211204](https://github.com/alexpovel/srgn/commit/b2112048a451bb02119532818d84ca30fb6e0f10))
* Scope import names (Go) ([9b76ce6](https://github.com/alexpovel/srgn/commit/9b76ce6dc6f7d70c6da4f3bf29d68e858c0b4434))
* Scope module names in imports (Python) ([b3345c4](https://github.com/alexpovel/srgn/commit/b3345c46350092698b5ad2fce8f63e349544b2a9))
* Scope names in uses-declarations (Rust) ([cda850d](https://github.com/alexpovel/srgn/commit/cda850d59375a2b0b89c52f88a025a7eea839411))

## [0.9.0](https://github.com/alexpovel/srgn/compare/v0.8.0...v0.9.0) (2023-12-03)


### Features

* Rust language (comments, doc comments, strings) ([f8910c8](https://github.com/alexpovel/srgn/commit/f8910c8c71f7aa8a5178154bf6f11d96f1eddc5d))


### Bug Fixes

* **docs:** Escape quotes ([e248938](https://github.com/alexpovel/srgn/commit/e248938bb45d3a99b7122e945a8e3c91ab657b0a))

## [0.8.0](https://github.com/alexpovel/srgn/compare/v0.7.0...v0.8.0) (2023-12-03)


### Features

* Go language (w/ comments, strings, struct tags) ([fe91428](https://github.com/alexpovel/srgn/commit/fe914281be8d6ad315238ab1fd1a5b9a11722227))
* Implement string interpolation handling ([2f37b2e](https://github.com/alexpovel/srgn/commit/2f37b2e4f15c7ef7ef417b5fee65c6b20448933f))
* Python strings ([f452b01](https://github.com/alexpovel/srgn/commit/f452b01fb7e05b0d8e54a2d01c23e75ae998f90f))
* query for C# strings ([f38136c](https://github.com/alexpovel/srgn/commit/f38136c3bf56909fdc1a9f8520cae46b1c3ea87a))
* query for TypeScript strings ([37de0d4](https://github.com/alexpovel/srgn/commit/37de0d4989e5751c05419be9ab16c4ff46ac8f0c))

## [0.7.0](https://github.com/alexpovel/srgn/compare/v0.6.0...v0.7.0) (2023-11-06)


### ⚠ BREAKING CHANGES

* Remove `Debug` implementation of `dyn Scoper`
* Remove `Replacement::new`, force going through unescaping via `TryFrom<String>`
* Make `Replacement` a newtype
* Panic upon creation of inconsistent view

### Miscellaneous Chores

* Make `Replacement` a newtype ([59d6daf](https://github.com/alexpovel/srgn/commit/59d6daf505325cbe5238a9561fe2cb486cff0b64))
* Panic upon creation of inconsistent view ([ad6a38a](https://github.com/alexpovel/srgn/commit/ad6a38ae1214622d4baada9c7107e4c21c8aab67)), closes [#51](https://github.com/alexpovel/srgn/issues/51)
* Remove `Debug` implementation of `dyn Scoper` ([31ef135](https://github.com/alexpovel/srgn/commit/31ef135ce7df5b635653f40a67f9f7a96fd380af))
* Remove `Replacement::new`, force going through unescaping via `TryFrom&lt;String&gt;` ([2ec98c1](https://github.com/alexpovel/srgn/commit/2ec98c16a47198214a646312a7b7c3de81c6178d))

## [0.6.0](https://github.com/alexpovel/srgn/compare/v0.5.0...v0.6.0) (2023-10-28)


### Features

* Warn when file does not look legit ([1593ec7](https://github.com/alexpovel/srgn/commit/1593ec77da670eee78a00981051f828fb611263b))
* Write names of processed files to stdout ([b42db4e](https://github.com/alexpovel/srgn/commit/b42db4ef29570934aed44f77f1e62242ba4f9b9f))


### Bug Fixes

* Leftover error messages from debugging ([e03c110](https://github.com/alexpovel/srgn/commit/e03c1106ef8a0f4d58bf3a201c767523f114efe2))

## [0.5.0](https://github.com/alexpovel/srgn/compare/v0.4.5...v0.5.0) (2023-10-26)


### ⚠ BREAKING CHANGES

* `explode` takes `&mut self`, add all remaining public docs
* Simplify `explode` (no more `explode_from_scoper`), improve docs
* View-related items into view module
* scopes into new module
* Simplify crate features, fix existing cfgs
* Make `R{O,W}Scope{,s}` a newtype so it can take `impl`s
* Unify `ScopedView::map` through `Action` trait

### Features

* File globbing and processing ([a8d330c](https://github.com/alexpovel/srgn/commit/a8d330c78ba0275909fa471fddc8f58e68181c83))
* Flag for failing on empty glob ([9d2fd0a](https://github.com/alexpovel/srgn/commit/9d2fd0a71e0b39043895a65e9674c0819b27440b))
* Make `german`-only dependencies `optional` ([b407b67](https://github.com/alexpovel/srgn/commit/b407b67b03896691123c3ae763b5ed64458cfc59))
* Provide ass. functions on view for all actions ([ca52905](https://github.com/alexpovel/srgn/commit/ca529057fef2e50db49148b6914eb292ee9ac755))
* Simplify `explode` (no more `explode_from_scoper`), improve docs ([ab0b914](https://github.com/alexpovel/srgn/commit/ab0b914175e9c0a04ba9c00bc72843abb949e733))
* Unify `ScopedView::map` through `Action` trait ([f6ff38d](https://github.com/alexpovel/srgn/commit/f6ff38d98cfed7ff4bddde0abbadcee297649220))


### Bug Fixes

* **ci:** Code coverage using `tarpaulin`, by conditionally disabling `insta` ([6ace4fa](https://github.com/alexpovel/srgn/commit/6ace4fa476cad447060d76043c731c811b64629f))
* **clippy:** `ignored_unit_patterns` ([4bc2827](https://github.com/alexpovel/srgn/commit/4bc28274e2578b968258f50f58618070fc0e8f5c)), closes [#35](https://github.com/alexpovel/srgn/issues/35)
* **docs:** Dead documentation symbol links ([ce3f900](https://github.com/alexpovel/srgn/commit/ce3f90015cdef9d53f67564d1281add3f1756762))
* **docs:** GitHub Markdown NOTE syntax error ([896129b](https://github.com/alexpovel/srgn/commit/896129b54a4799b12f65b7816e35209aabef5517))
* **docs:** Implement placeholder for custom query ([0844a99](https://github.com/alexpovel/srgn/commit/0844a99dbcf3bad02d41cdecd348ee915bc895eb))
* **docs:** In/Out was swapped for DosFix ([9d56346](https://github.com/alexpovel/srgn/commit/9d56346138d1e01b5ad1914078ef11e19dca287a))
* Feature-gated doc tests working properly ([a46e60d](https://github.com/alexpovel/srgn/commit/a46e60db9ef5bbbcb931bc4ff217b6459ce4c2e7))
* License for crates.io ([6c13a62](https://github.com/alexpovel/srgn/commit/6c13a62bb6f6ba99573494b9ec6a7bcd23bfff67))
* Simplify crate features, fix existing cfgs ([af1b39d](https://github.com/alexpovel/srgn/commit/af1b39db705b9795933e3dd0716ecd3c8cbd5cac))
* Splitting of DOS-style line endings ([496337c](https://github.com/alexpovel/srgn/commit/496337c5d664db3df4a884f0050ee0d1357d8c2b))


### Miscellaneous Chores

* `explode` takes `&mut self`, add all remaining public docs ([33097c1](https://github.com/alexpovel/srgn/commit/33097c149f855a4a09ca1aef6535833c0e9a016e)), closes [#6](https://github.com/alexpovel/srgn/issues/6)
* Make `R{O,W}Scope{,s}` a newtype so it can take `impl`s ([98b04d5](https://github.com/alexpovel/srgn/commit/98b04d5d7a1cb1a88ac8158cdaa0a2673f4e2114))
* scopes into new module ([e951347](https://github.com/alexpovel/srgn/commit/e9513470d62c5a2fc165c656d8a02c810b5bf2fb))
* View-related items into view module ([18ef801](https://github.com/alexpovel/srgn/commit/18ef801577b345f24a7bba37909a4387e553bf13))

## [0.4.5](https://github.com/alexpovel/srgn/compare/v0.4.4...v0.4.5) (2023-10-22)


### Bug Fixes

* **ci:** Only build binaries on release creation ([6eaac7b](https://github.com/alexpovel/srgn/commit/6eaac7b1975b4139695eced6c1caa0aaf1bf59ae))
* **ci:** Skip entire job, not just its single step ([9787306](https://github.com/alexpovel/srgn/commit/978730619eac7695b3891f53cb294817c3356cfc))

## [0.4.4](https://github.com/alexpovel/srgn/compare/v0.4.3...v0.4.4) (2023-10-22)


### Bug Fixes

* **ci:** Switch from env var to native GitHub Actions variable ([816cc54](https://github.com/alexpovel/srgn/commit/816cc54f6d29299b8b39b28ee5b6ca69b0ecf0e4))
* **ci:** Use bash for all platforms ([9fcb348](https://github.com/alexpovel/srgn/commit/9fcb34853f52b1ae7298e8e6836d69ebbccf1980))

## [0.4.3](https://github.com/alexpovel/srgn/compare/v0.4.2...v0.4.3) (2023-10-22)


### Bug Fixes

* **ci:** Use cargo-binstall non-interactively ([78dbba9](https://github.com/alexpovel/srgn/commit/78dbba966761f0f2337dea602d3eb9832e819b79))

## [0.4.2](https://github.com/alexpovel/srgn/compare/v0.4.1...v0.4.2) (2023-10-22)


### Bug Fixes

* **ci:** Adjust GitHub token permissions for gh CLI ([243878b](https://github.com/alexpovel/srgn/commit/243878b7095770167cf781f01ed26f8ae67ad7f9))

## [0.4.1](https://github.com/alexpovel/srgn/compare/v0.4.0...v0.4.1) (2023-10-22)


### Bug Fixes

* **ci:** Checkout before running gh CLI ([490d822](https://github.com/alexpovel/srgn/commit/490d8226eb4517937f61d5567966e888125445ce))

## [0.4.0](https://github.com/alexpovel/srgn/compare/v0.3.2...v0.4.0) (2023-10-22)


### Features

* **ci:** Test installation via binstall ([ecc35b4](https://github.com/alexpovel/srgn/commit/ecc35b43fd322fe3d24cc43ae58411d66b6fc46f))

## [0.3.2](https://github.com/alexpovel/srgn/compare/v0.3.1...v0.3.2) (2023-10-22)


### Bug Fixes

* **ci:** Force bash shell on all OSs ([f34af16](https://github.com/alexpovel/srgn/commit/f34af16aa7455f26f8788200c0f1dfb39a077871))

## [0.3.1](https://github.com/alexpovel/srgn/compare/v0.3.0...v0.3.1) (2023-10-22)


### Bug Fixes

* **ci:** Provide credentials token to gh CLI ([1c9c21f](https://github.com/alexpovel/srgn/commit/1c9c21f6be6be7e9f0b9d0f5996df8e96c379ad2))

## [0.3.0](https://github.com/alexpovel/srgn/compare/v0.2.0...v0.3.0) (2023-10-22)


### Features

* **ci:** Provide binaries (x86/64bit for macOS, Linux, Windows) ([f4c009f](https://github.com/alexpovel/srgn/commit/f4c009fe0002e3944ebcf79183f134ceaf4f936e))


### Bug Fixes

* **ci:** Windows binary extension and version string ([d93004b](https://github.com/alexpovel/srgn/commit/d93004b5775e110e803f5a4543ad53d10d98a32e))

## [0.2.0](https://github.com/alexpovel/srgn/compare/v0.1.0...v0.2.0) (2023-10-14)


### Features

* `fail-any` and `fail-none` feature flags ([1dd3dcb](https://github.com/alexpovel/srgn/commit/1dd3dcb3b233d787abc91622ddc7ce019c764878))


### Bug Fixes

* **ci:** Linking/cc fails for tarpaulin; try w/o cache ([ab40957](https://github.com/alexpovel/srgn/commit/ab409571417881a599c3fd32645dd26a5c9d8349))
* Do not `pub use` `Action` ([1e2c663](https://github.com/alexpovel/srgn/commit/1e2c663473f5c6140f065301f815f3cd3726837a))

## 0.1.0 (2023-10-05)


### Features

* `german_prefer_original` option ([a25479a](https://github.com/alexpovel/srgn/commit/a25479ac3b4e1d650311f24b0a624bd8e62386e1)), closes [#25](https://github.com/alexpovel/srgn/issues/25)
* `is_compound_word` -&gt; `decompose_compound_word` ([8cf6175](https://github.com/alexpovel/srgn/commit/8cf6175a6d7e4470482f413c125cfc94c5c36335))
* `squeeze` stage, `Scoped` concept ([4c18820](https://github.com/alexpovel/srgn/commit/4c18820c9b0e7e85a475af792821debb85c0c12e))
* `symbols` stage ([daae90c](https://github.com/alexpovel/srgn/commit/daae90cb1b5e5b67e17ad19b6b8bee0e56111eba))
* Add memoization ([58b5fdf](https://github.com/alexpovel/srgn/commit/58b5fdf2ab6537a3cb4bff33acbd3e96b27cc5f7)), closes [#1](https://github.com/alexpovel/srgn/issues/1)
* Child options and flags imply their parent ([20518c2](https://github.com/alexpovel/srgn/commit/20518c2ea8ce62bbef3c97663d93446cdaaf8d56))
* CSharp support (comments) and `UserService.cs` sample ([1cef201](https://github.com/alexpovel/srgn/commit/1cef20113cad889058c00c024fca59ba2a8b5507))
* Deletion stage ([4fd7e76](https://github.com/alexpovel/srgn/commit/4fd7e7646d908e9a68dc6a8254258934de74fd3f))
* Introduce FSTs ([f3434d6](https://github.com/alexpovel/srgn/commit/f3434d6bcac9c44763d27929d6dde2f58a3f68d3))
* Introduce proper error enum for word casing ([e3a8c5a](https://github.com/alexpovel/srgn/commit/e3a8c5a9da069f2dc97d4417fb2d9c47be301cec))
* Inversion ([b0c3b6b](https://github.com/alexpovel/srgn/commit/b0c3b6b00f9393fb5f63d2e3d531005fecde2d35))
* Lowercasing stage ([e0b097a](https://github.com/alexpovel/srgn/commit/e0b097a692735257c9dce063682b662b46ebb0ed))
* naive mode for German ([cb3357c](https://github.com/alexpovel/srgn/commit/cb3357c21aae80735c1986e89564f924f27e0e83))
* replacement stage ([8886880](https://github.com/alexpovel/srgn/commit/88868805b8cafd7770f1252a9ce10986fa82cec5))
* Support upper/mixed case special characters ([90111da](https://github.com/alexpovel/srgn/commit/90111da3fa69cb7fd105856608acf1afd9a05a49)), closes [#5](https://github.com/alexpovel/srgn/issues/5)
* TypeScript, with TODO app example ([3d3ed21](https://github.com/alexpovel/srgn/commit/3d3ed21582cc91d76e62bc8729dbeb38f70ebfb9))
* uppercase stage ([c92ad21](https://github.com/alexpovel/srgn/commit/c92ad21584c875aa196ac18dea8f845bb610b4be))
* Use `decompound` ([c640363](https://github.com/alexpovel/srgn/commit/c64036351fc7d0ea32b89f4744de97c04d8c39fe))
* Use `once_cell` to build `fst::Set` only once ([3eae2b6](https://github.com/alexpovel/srgn/commit/3eae2b6408feea8c824e13a3a718011a3f9326a1))
* Verbosity switch ([8c05d69](https://github.com/alexpovel/srgn/commit/8c05d69451be0fd072ee4e62b818d0f8206e5d41))
* Word list performance increased (now a single &str) ([32716fa](https://github.com/alexpovel/srgn/commit/32716fae902bb8c744b3898b73437e375539d469))
* Working tree-sitter for language scoping ([bf17589](https://github.com/alexpovel/srgn/commit/bf17589f782aeab41e61f51044999116065b3a74))


### Bug Fixes

* Add `aufwändig` to German word list ([69a138b](https://github.com/alexpovel/srgn/commit/69a138b050eea0a1e128e9d47543db68da13601d))
* **ci:** cargo-tarpaulin v0.27 broke 'Xml' ([df585a2](https://github.com/alexpovel/srgn/commit/df585a2e839605d4f39e32e5c6e51ce5c473146e))
* **ci:** Checkout and cache code coverage run ([938b7a5](https://github.com/alexpovel/srgn/commit/938b7a57b48b20a1b7797d4a02933f60fc017871))
* **ci:** Run coverage test in parallel ([f4167d5](https://github.com/alexpovel/srgn/commit/f4167d594ba701b6ff54bc150f2e7c9eec017134))
* **ci:** Run release chores in parallel ([afca5cf](https://github.com/alexpovel/srgn/commit/afca5cf8d94d0dfc856b71f2fb9325a5bf5c7032))
* **ci:** Trigger release-please ([505110a](https://github.com/alexpovel/srgn/commit/505110a6781bafdd4bb50210159f3d1f0cd90ab0))
* **ci:** Update all dependencies ([928a6d5](https://github.com/alexpovel/srgn/commit/928a6d5d219b7cee6c9698d9942fca7fb653550a))
* Drop custom `Span`, use `std::ops::Range` ([fad059e](https://github.com/alexpovel/srgn/commit/fad059e2f07b80581dee3c71fc9ac48fa4398fd4))
* Fix tests after project rename ([f5d4f17](https://github.com/alexpovel/srgn/commit/f5d4f1787148ddec5786f5a8c14d572624ac2873))
* Make initial/missing `Transition` an unrepresentable state ([ba2948d](https://github.com/alexpovel/srgn/commit/ba2948de94205b6729a02b57f611d3a287138387))
* Squeezing now has `tr`-like `squeeze-repeats` alias ([4d67a45](https://github.com/alexpovel/srgn/commit/4d67a458171a237b3b782753e140a564ed7f84d2))
* **test:** Symbols stage is *not* fully idempotent ([5ff9277](https://github.com/alexpovel/srgn/commit/5ff92773a01435db60b78c8c3e819533cacbfcdb))


### Performance Improvements

* Add profiling tooling (as a Justfile, for automation) ([9503761](https://github.com/alexpovel/srgn/commit/9503761dba51fd36d5dd1fd77937c4cc133f624c)), closes [#14](https://github.com/alexpovel/srgn/issues/14)

## 0.1.0 (2023-10-03)


### Features

* `german_prefer_original` option ([a25479a](https://github.com/alexpovel/betterletters/commit/a25479ac3b4e1d650311f24b0a624bd8e62386e1)), closes [#25](https://github.com/alexpovel/betterletters/issues/25)
* `is_compound_word` -&gt; `decompose_compound_word` ([8cf6175](https://github.com/alexpovel/betterletters/commit/8cf6175a6d7e4470482f413c125cfc94c5c36335))
* `squeeze` stage, `Scoped` concept ([4c18820](https://github.com/alexpovel/betterletters/commit/4c18820c9b0e7e85a475af792821debb85c0c12e))
* `symbols` stage ([daae90c](https://github.com/alexpovel/betterletters/commit/daae90cb1b5e5b67e17ad19b6b8bee0e56111eba))
* Add memoization ([58b5fdf](https://github.com/alexpovel/betterletters/commit/58b5fdf2ab6537a3cb4bff33acbd3e96b27cc5f7)), closes [#1](https://github.com/alexpovel/betterletters/issues/1)
* Child options and flags imply their parent ([20518c2](https://github.com/alexpovel/betterletters/commit/20518c2ea8ce62bbef3c97663d93446cdaaf8d56))
* CSharp support (comments) and `UserService.cs` sample ([1cef201](https://github.com/alexpovel/betterletters/commit/1cef20113cad889058c00c024fca59ba2a8b5507))
* Deletion stage ([4fd7e76](https://github.com/alexpovel/betterletters/commit/4fd7e7646d908e9a68dc6a8254258934de74fd3f))
* Introduce FSTs ([f3434d6](https://github.com/alexpovel/betterletters/commit/f3434d6bcac9c44763d27929d6dde2f58a3f68d3))
* Introduce proper error enum for word casing ([e3a8c5a](https://github.com/alexpovel/betterletters/commit/e3a8c5a9da069f2dc97d4417fb2d9c47be301cec))
* Inversion ([b0c3b6b](https://github.com/alexpovel/betterletters/commit/b0c3b6b00f9393fb5f63d2e3d531005fecde2d35))
* Lowercasing stage ([e0b097a](https://github.com/alexpovel/betterletters/commit/e0b097a692735257c9dce063682b662b46ebb0ed))
* naive mode for German ([cb3357c](https://github.com/alexpovel/betterletters/commit/cb3357c21aae80735c1986e89564f924f27e0e83))
* replacement stage ([8886880](https://github.com/alexpovel/betterletters/commit/88868805b8cafd7770f1252a9ce10986fa82cec5))
* Support upper/mixed case special characters ([90111da](https://github.com/alexpovel/betterletters/commit/90111da3fa69cb7fd105856608acf1afd9a05a49)), closes [#5](https://github.com/alexpovel/betterletters/issues/5)
* TypeScript, with TODO app example ([3d3ed21](https://github.com/alexpovel/betterletters/commit/3d3ed21582cc91d76e62bc8729dbeb38f70ebfb9))
* uppercase stage ([c92ad21](https://github.com/alexpovel/betterletters/commit/c92ad21584c875aa196ac18dea8f845bb610b4be))
* Use `decompound` ([c640363](https://github.com/alexpovel/betterletters/commit/c64036351fc7d0ea32b89f4744de97c04d8c39fe))
* Use `once_cell` to build `fst::Set` only once ([3eae2b6](https://github.com/alexpovel/betterletters/commit/3eae2b6408feea8c824e13a3a718011a3f9326a1))
* Verbosity switch ([8c05d69](https://github.com/alexpovel/betterletters/commit/8c05d69451be0fd072ee4e62b818d0f8206e5d41))
* Word list performance increased (now a single &str) ([32716fa](https://github.com/alexpovel/betterletters/commit/32716fae902bb8c744b3898b73437e375539d469))
* Working tree-sitter for language scoping ([bf17589](https://github.com/alexpovel/betterletters/commit/bf17589f782aeab41e61f51044999116065b3a74))


### Bug Fixes

* Add `aufwändig` to German word list ([69a138b](https://github.com/alexpovel/betterletters/commit/69a138b050eea0a1e128e9d47543db68da13601d))
* **ci:** cargo-tarpaulin v0.27 broke 'Xml' ([df585a2](https://github.com/alexpovel/betterletters/commit/df585a2e839605d4f39e32e5c6e51ce5c473146e))
* **ci:** Update all dependencies ([928a6d5](https://github.com/alexpovel/betterletters/commit/928a6d5d219b7cee6c9698d9942fca7fb653550a))
* Drop custom `Span`, use `std::ops::Range` ([fad059e](https://github.com/alexpovel/betterletters/commit/fad059e2f07b80581dee3c71fc9ac48fa4398fd4))
* Fix tests after project rename ([f5d4f17](https://github.com/alexpovel/betterletters/commit/f5d4f1787148ddec5786f5a8c14d572624ac2873))
* Make initial/missing `Transition` an unrepresentable state ([ba2948d](https://github.com/alexpovel/betterletters/commit/ba2948de94205b6729a02b57f611d3a287138387))
* Squeezing now has `tr`-like `squeeze-repeats` alias ([4d67a45](https://github.com/alexpovel/betterletters/commit/4d67a458171a237b3b782753e140a564ed7f84d2))
* **test:** Symbols stage is *not* fully idempotent ([5ff9277](https://github.com/alexpovel/betterletters/commit/5ff92773a01435db60b78c8c3e819533cacbfcdb))


### Performance Improvements

* Add profiling tooling (as a Justfile, for automation) ([9503761](https://github.com/alexpovel/betterletters/commit/9503761dba51fd36d5dd1fd77937c4cc133f624c)), closes [#14](https://github.com/alexpovel/betterletters/issues/14)
