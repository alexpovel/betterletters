//! End-to-end tests for the CLI. Main purpose is exercising multiple combinations of
//! inputs/flags/options.

#[cfg(test)]
#[cfg(feature = "all")]
mod tests {
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};

    use anyhow::Context;
    use assert_cmd::Command;
    use insta::with_settings;
    use itertools::Itertools;
    use rstest::rstest;
    use serde::Serialize;
    use tempfile::TempDir;

    #[derive(Debug, Serialize)]
    struct CommandSnap {
        args: Vec<String>,
        stdin: Option<Vec<String>>,
        stdout: Vec<String>,
        exit_code: i32, // `u8` would make sense but this is what the library returns 🤷‍♀️
    }

    #[derive(Debug, Serialize)]
    struct CommandInfo {
        stderr: Vec<String>,
    }

    impl CommandInfo {
        pub fn new(stderr: &str) -> Self {
            Self {
                stderr: stderr
                    .lines()
                    .filter(|l| !l.starts_with('[')) // Normal log lines
                    .map(ToOwned::to_owned)
                    .collect(),
            }
        }
    }

    #[rstest]
    #[case(
        "baseline-replacement",
        false,
        &[
            "A",
            "--",
            "B",
        ],
        Some(r"A;  B 😫"),
    )]
    #[case(
        "baseline-replacement-no-stdin",
        false,
        &[
            "A",
            "--",
            "B",
        ],
        None,
    )]
    #[case(
        "baseline-regex-replacement",
        false,
        &[
            r"\W",
            "--",
            "B",
        ],
        Some(r"A;  B 😫"),
    )]
    #[case(
        "german-symbols",
        false,
        &[
            "--german",
            "--symbols",
        ],
        Some(r"Duebel -> 1.5mm;  Wand != 3m²... UEBELTAETER! 😫"),
    )]
    #[case(
        "german-text",
        false,
        &[
            "--german",
        ],
        Some(r#"Zwei flinke Boxer jagen die quirlige Eva und ihren Mops durch Sylt.
Franz jagt im komplett verwahrlosten Taxi quer durch Bayern.
Zwoelf Boxkaempfer jagen Viktor quer ueber den grossen Sylter Deich.
Vogel Quax zwickt Johnys Pferd Bim.
Sylvia wagt quick den Jux bei Pforzheim.
Polyfon zwitschernd assen Maexchens Voegel Rueben, Joghurt und Quark.
"Fix, Schwyz!" quaekt Juergen bloed vom Pass.
Victor jagt zwoelf Boxkaempfer quer ueber den grossen Sylter Deich.
Falsches Ueben von Xylophonmusik quaelt jeden groesseren Zwerg.
Heizoelrueckstossabdaempfung.
"#),
    )]
    #[case(
        "deleting-emojis",
        false,
        &[
            "--delete",
            r"\p{Emoji_Presentation}",
        ],
        Some("Some text  :) :-) and emojis 🤩!\nMore: 👽"),
    )]
    #[case(
        "failing-on-anything-found-trigger",
        false,
        &[
            "--fail-any",
            "X",
        ],
        Some("XYZ"),
    )]
    #[case(
        "failing-on-anything-found-no-trigger",
        false,
        &[
            "--fail-any",
            "A",
        ],
        Some("XYZ"),
    )]
    #[case(
        "failing-on-nothing-found-trigger",
        false,
        &[
            "--fail-none",
            "A",
        ],
        Some("XYZ"),
    )]
    #[case(
        "failing-on-nothing-found-no-trigger",
        false,
        &[
            "--fail-none",
            "X",
        ],
        Some("XYZ"),
    )]
    #[case(
        "go-search",
        false,
        &[
            "--go",
            "comments",
            "[fF]izz",
        ],
        Some(include_str!("langs/go/fizzbuzz.go")),
    )]
    #[case(
        "go-replacement",
        false,
        &[
            "--go",
            "comments",
            "[fF]izz",
            "--",
            "🤡",
        ],
        Some(include_str!("langs/go/fizzbuzz.go")),
    )]
    #[case(
        "go-search-files",
        true, // Prints different file paths! (but thanks to `autocrlf = false` has identical line endings)
        &[
            "--sorted", // Need determinism
            "--go",
            "comments",
            "[fF]izz",
        ],
        None,
    )]
    #[case(
        "python-search-files", // searches all files, in all Python strings
        true, // Prints different file paths! (but thanks to `autocrlf = false` has identical line endings)
        &[
            "--sorted", // Need determinism
            "--python",
            "strings",
            "is",
        ],
        None,
    )]
    #[case(
        "python-search-stdin", // stdin takes precedence
        false,
        &[
            "--python",
            "strings",
            "is",
        ],
        Some(include_str!("langs/python/base.py")),
    )]
    #[case(
        "python-search-stdin-and-files", // stdin takes precedence
        false,
        &[
            "--python",
            "strings",
            "--glob",
            "**/*.py",
            "is",
        ],
        Some(include_str!("langs/python/base.py")),
    )]
    #[case(
        "python-search-stdin-across-lines",
        false,
        &[
            "--python",
            "class",
            r"(?s)@classmethod\n\s+def class_method", // ?s: include newline
        ],
        Some(include_str!("langs/python/base.py")),
    )]
    #[case(
        "python-multiple-scopes",
        false,
        &[
            "--python",
            "def",
            "--python",
            "strings",
            "A",
        ],
        Some("# A comment\nx = \"A string\"\ndef A(): return \"A string in a func\"\nclass A: pass"),
    )]
    //
    // Set up baseline for subsequent tests
    #[case(
        "only-matching-baseline-outside-search-mode",
        false,
        &[
            "A",
            "--",
            "X",
        ],
        Some("A\nB"),
    )]
    #[case(
        "only-matching-outside-search-mode",
        false,
        &[
            "--only-matching",
            "A",
            "--",
            "X",
        ],
        Some("A\nB"),
    )]
    #[case(
        "line-numbers-outside-search-mode",
        false,
        &[
            "--line-numbers",
            "A",
            "--",
            "X",
        ],
        Some("A\nB"),
    )]
    #[case(
        "only-matching-and-line-numbers-outside-search-mode",
        false,
        &[
            "--only-matching",
            "--line-numbers",
            "A",
            "--",
            "X",
        ],
        Some("A\nB"),
    )]
    // Taking no action etc., but sure enough prints line numbers...
    #[case(
        "only-matching-and-line-numbers-no-actions-outside-search-mode",
        false,
        &[
            "--only-matching",
            "--line-numbers",
        ],
        Some("A\nB"),
    )]
    fn test_cli(
        #[case] mut snapshot_name: String,
        #[case] os_dependent: bool,
        #[case] args: &[&str],
        #[case] stdin: Option<&str>,
    ) {
        if os_dependent {
            // Thanks to Windows, (some) snapshots are actually OS-dependent if they
            // involve file system paths :( Careful: `cargo insta test
            // --unreferenced=delete` will wipe snapshot of foreign OSes, but that'll
            // break in CI!
            snapshot_name.push('-');
            snapshot_name.push_str(std::env::consts::OS);
        }

        // Should rebuild the binary to `target/debug/<name>`. This works if running as
        // an integration test (insides `tests/`), but not if running as a unit test
        // (inside `src/main.rs` etc.).
        let mut cmd = get_cmd();

        let mut args: VecDeque<String> = args.iter().map(|&s| s.to_owned()).collect();

        // Be deterministic for testing purposes. We have to push this awkwardly, as the
        // last argument might be strongly positional, and nothing else is allowed to
        // come after it.
        args.push_front("1".into());
        args.push_front("--threads".into());

        if let Some(stdin) = stdin {
            cmd.write_stdin(stdin);
        } else {
            // Override; `Command` is detected as providing stdin but we're working on
            // files here.
            args.push_front("false".into());
            args.push_front("--stdin-override-to".into());
        }

        cmd.args(args.clone());

        let output = cmd.output().expect("failed to execute process");

        let exit_code = output
            .status
            .code()
            .expect("Process unexpectedly terminated via signal, not `exit`.");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        // For debugging, include this, but do not rely on it for snapshot
        // validity/correctness. We do not want changes in error messages etc. break
        // tests, seems excessive.
        let info = CommandInfo::new(&stderr);

        with_settings!({
            info => &info,
        }, {
            insta::assert_yaml_snapshot!(
                snapshot_name,
                CommandSnap {
                    args: args.into(),
                    stdin: stdin.map(|s| s.split_inclusive('\n').map(ToOwned::to_owned).collect_vec()),
                    stdout: stdout.split_inclusive('\n').map(ToOwned::to_owned).collect_vec(),
                    exit_code,
                }
            );
        });
    }

    #[rstest]
    #[case::files_inplace_python(
        "files-inplace-python",
        "tests/files/files-python/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--sorted",
            "--glob",
            "**/*.py",
            "foo",
            "--",
            "baz"
        ],
        false,
    )]
    #[case::language_scoping_inplace_python(
        "language-scoping-inplace-python",
        "tests/files/language-scoping-python/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--sorted",
            "--python",
            "function-names",
            "foo",
            "--",
            "baz"
        ],
        false,
    )]
    #[case::language_scoping_and_files_inplace_python(
        "language-scoping-and-files-inplace-python",
        "tests/files/language-scoping-and-files-python/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--sorted",
            "--python",
            "function-names",
            "--glob", // Will override language scoper
            "subdir/**/*.py",
            "foo",
            "--",
            "baz"
        ],
        false,
    )]
    #[case::language_scoping_and_files_inplace_python(
        "language-scoping-and-files-inplace-python",
        "tests/files/language-scoping-and-files-python/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--python",
            "function-names",
            "--glob", // Will override language scoper
            "subdir/**/*.py",
            "foo",
            "--",
            "baz"
        ],
        // NOT `--sorted`, so not deterministic; use to test that directories are
        // equivalent even if running parallel, unsorted. Output will be random,
        // breaking snapshot testing.
        true,
    )]
    #[case::binary_data_sorted(
        "binary-data-sorted",
        "tests/files/binary-data/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--sorted",
            "--glob",
            "**/*",
            "0a1a09c8-2995-4ac5-9d60-01a0f02920e8",
            "gone"
        ],
        false,
    )]
    #[case::binary_data_unsorted(
        "binary-data-sorted",
        "tests/files/binary-data/in",
        &[
            "-vvvv", // Trigger logging lines, just for more coverage
            "--glob",
            "**/*",
            "0a1a09c8-2995-4ac5-9d60-01a0f02920e8",
            "gone"
        ],
        // NOT `--sorted`, so not deterministic; use to test that directories are
        // equivalent even if running parallel, unsorted. Output will be random,
        // breaking snapshot testing.
        true,
    )]
    fn test_cli_files(
        #[case] mut snapshot_name: String,
        #[case] input: PathBuf,
        #[case] args: &[&str],
        #[case] skip_output_check: bool,
        #[values(true, false)] dry_run: bool, // Check all permutations for all inputs
    ) -> anyhow::Result<()> {
        let args = args.iter().map(ToString::to_string).collect_vec();

        // Arrange
        let mut cmd = get_cmd();

        let baseline = if dry_run {
            // Stays the same! In dry runs, we compare against the very same directory,
            // as it should not change.
            input.clone()
        } else {
            let mut baseline = input.clone();
            baseline.pop();
            baseline.push("out");
            baseline
        };

        let candidate = copy_to_tmp(&input);
        drop(input); // Prevent misuse

        cmd.current_dir(&candidate);
        cmd.args(
            // Override; `Command` is detected as providing stdin but we're working on
            // files here.
            ["--stdin-override-to", "false"],
        );
        cmd.args(&args);
        if dry_run {
            cmd.arg("--dry-run");
        }

        // Act
        let output = cmd.output().expect("failed to execute binary under test");

        // Assert

        // Thing itself works
        output.status.success().then_some(()).ok_or_else(|| {
            anyhow::anyhow!(
                "Binary execution itself failed: {}",
                String::from_utf8_lossy(&output.stderr).escape_debug()
            )
        })?;

        // Do not drop on panic, to keep tmpdir in place for manual inspection. Can then
        // diff directories.
        check_directories_equality(baseline, candidate.path().to_owned())?;

        // Test was successful: ok to drop. Caveat: fails test if deletion fails, which
        // is unwarranted coupling?
        candidate.close()?;

        // Let's look at command output now.
        if !skip_output_check {
            if dry_run {
                snapshot_name.push_str("-dry-run");
            }

            // These are inherently platform-specific, as they deal with file paths.
            snapshot_name.push('-');
            snapshot_name.push_str(std::env::consts::OS);

            let exit_code = output
                .status
                .code()
                .expect("Process unexpectedly terminated via signal, not `exit`.");
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();

            let info = CommandInfo::new(&stderr);

            with_settings!({
                info => &info,
            }, {
                insta::assert_yaml_snapshot!(
                    snapshot_name,
                    CommandSnap {
                        args,
                        stdin: None,
                        stdout: stdout.split_inclusive('\n').map(ToOwned::to_owned).collect_vec(),
                        exit_code,
                    }
                );
            });
        }

        Ok(())
    }

    /// Tests various *expected* failure modes.
    #[rstest]
    //
    // stdin
    #[case(
        "fail-none-implicitly-in-search-mode-stdin",
        Some(r#"x = "y""#),
        &[
            "--python",
            "strings",
            "z",
        ],
        None,
    )]
    #[case(
        "fail-none-explicitly-in-search-mode-stdin",
        Some(r#"x = "y""#),
        &[
            "--fail-none",
            "--python",
            "strings",
            "z",
        ],
        None,
    )]
    #[case(
        "fail-any-in-search-mode-stdin",
        Some(r#"x = "y""#),
        &[
            "--fail-any",
            "--python",
            "strings",
            "y",
        ],
        None,
    )]
    //
    // Multiple files, sorted
    #[case(
        "fail-none-implicitly-in-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--python",
            "strings",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-none-explicitly-in-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-none",
            "--python",
            "strings",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-none-explicitly-outside-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-none",
            "--glob",
            "**/*.py",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-any-in-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-any",
            "--python",
            "strings",
            r".",
        ],
        None,
    )]
    #[case(
        "fail-any-outside-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-any",
            "--glob",
            "**/*.py",
            r".",
        ],
        None,
    )]
    #[case(
        "fail-no-files-in-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-no-files",
            "--python",
            "strings",
            r".",
        ],
        Some(Path::new("tests/langs/go")), // No Python files here...
    )]
    #[case(
        "fail-no-files-outside-search-mode-sorted",
        None,
        &[
            "--sorted",
            "--fail-no-files",
            "--glob",
            "**/*.there-is-no-such-suffix",
            r".",
        ],
        None,
    )]
    //
    // Multiple files, not sorted aka multi-threaded
    #[case(
        "fail-none-implicitly-in-search-mode-multithreaded",
        None,
        &[
            "--python",
            "strings",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-none-explicitly-in-search-mode-multithreaded",
        None,
        &[
            "--fail-none",
            "--python",
            "strings",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-none-explicitly-outside-search-mode-multithreaded",
        None,
        &[
            "--fail-none",
            "--glob",
            "**/*.py",
            "unfindable-string-dheuihiuhiulerfiuehrilufhiusdho438ryh9vuoih",
        ],
        None,
    )]
    #[case(
        "fail-any-in-search-mode-multithreaded",
        None,
        &[
            "--fail-any",
            "--python",
            "strings",
            r".",
        ],
        None,
    )]
    #[case(
        "fail-any-outside-search-mode-multithreaded",
        None,
        &[
            "--fail-any",
            "--glob",
            "**/*.py",
            r".",
        ],
        None,
    )]
    #[case(
        "fail-no-files-in-search-mode-multithreaded",
        None,
        &[
            "--fail-no-files",
            "--python",
            "strings",
            r".",
        ],
        Some(Path::new("tests/langs/go")), // No Python files here...
    )]
    #[case(
        "fail-no-files-outside-search-mode-multithreaded",
        None,
        &[
            "--fail-no-files",
            "--glob",
            "**/*.there-is-no-such-suffix",
            r".",
        ],
        None,
    )]
    //
    //
    #[case(
        "fail-multiple-languages",
        None,
        &[
            // This should be stopped very early on, in CLI entry
            "--python",
            "strings",
            "--go",
            "strings",
        ],
        None,
    )]
    #[case(
        "go-ignores-vendor-directory",
        None,
        &[
            "--go",
            "comments",
        ],
        Some(Path::new("tests/langs/go/vendor-dir-test/")), // Contains vendor dir
    )]
    fn test_cli_failure_modes(
        #[case] snapshot_name: String,
        #[case] stdin: Option<&str>,
        #[case] args: &[&str],
        #[case] cwd: Option<&Path>,
    ) {
        let args = args.iter().map(ToString::to_string).collect_vec();

        // Arrange
        let mut cmd = get_cmd();

        if let Some(stdin) = stdin {
            cmd.write_stdin(stdin);
        } else {
            cmd.args(
                // Override; `Command` is detected as providing stdin but we're working on
                // files here.
                ["--stdin-override-to", "false"],
            );
        }
        cmd.args(&args);

        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }

        // Act
        let output = cmd.output().expect("failed to execute binary under test");

        // Assert
        let exit_code = output
            .status
            .code()
            .expect("Process unexpectedly terminated via signal, not `exit`.");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        let info = CommandInfo::new(&stderr);

        with_settings!({
            info => &info,
        }, {
            insta::assert_yaml_snapshot!(
                snapshot_name,
                CommandSnap {
                    args,
                    stdin: None,
                    stdout: stdout.split_inclusive('\n').map(ToOwned::to_owned).collect_vec(),
                    exit_code,
                }
            );
        });
    }

    #[test]
    fn test_shell_completion() {
        use predicates::str::contains;

        let mut cmd = get_cmd();
        cmd.args(["--completions", "zsh"]);

        cmd.assert().success();
        // Let's just see if this prints something that could *roughly* make sense.
        cmd.assert().stdout(contains("python"));
    }

    #[test]
    fn test_cli_on_invalid_utf8() {
        let mut cmd = get_cmd();

        let input = b"invalid utf8 \xFF";

        #[allow(invalid_from_utf8)] // Attribute didn't work on `assert` macro?
        let check = std::str::from_utf8(input);
        assert!(check.is_err(), "Input is valid UTF8, test is broken");

        cmd.write_stdin(*input);

        cmd.assert().failure();
    }

    /// Tests the helper function itself.
    #[test]
    fn test_directory_comparison() -> anyhow::Result<()> {
        for result in ignore::WalkBuilder::new("./src")
            .add("./tests")
            .add("./data")
            .add("./docs")
            .build()
        {
            let entry = result.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let path = path.to_owned();

                {
                    // Any directory compares to itself fine.
                    check_directories_equality(path.clone(), path.clone())?;
                }

                {
                    let parent = path
                        .clone()
                        .parent()
                        .expect("(our) directories under test always have parents")
                        .to_owned();

                    assert!(check_directories_equality(
                        // Impossible: a directory always compares unequal to a subdirectory
                        // of itself.
                        parent, path
                    )
                    .is_err());
                }
            }
        }

        Ok(())
    }

    fn get_cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    /// Same as [`compare_directories`], but checks in both directions.
    ///
    /// This ensures exact equality, instead of more loose 'superset' shenanigans.
    fn check_directories_equality<P: Into<PathBuf>>(
        baseline: P,
        candidate: P,
    ) -> anyhow::Result<()> {
        let baseline = baseline.into();
        let candidate = candidate.into();

        compare_directories(baseline.clone(), candidate.clone())?;
        compare_directories(candidate, baseline)
    }

    /// Recursively compares file contents of some `baseline` directory to some
    /// `candidate`.
    ///
    /// The `candidate` tree has to be a superset (not strict) of `baseline`: all files
    /// with their full paths, i.e. all intermediary directories, need to exist in
    /// `candidate` as they do in `baseline`, but extraneous files in `candidate` are
    /// allowed and ignored.
    ///
    /// **File contents are checked for exactly**. File metadata is not compared.
    ///
    /// Any failure fails the entire comparison.
    ///
    /// Lots of copying happens, so not efficient.
    fn compare_directories<P: Into<PathBuf>>(baseline: P, candidate: P) -> anyhow::Result<()> {
        let baseline: PathBuf = baseline.into();
        let mut candidate: PathBuf = candidate.into();

        for entry in baseline
            .read_dir()
            .with_context(|| format!("Failure reading left dir: {baseline:?}"))?
        {
            // This shadows on purpose: less risk of misuse
            let left = entry
                .with_context(|| format!("Failure reading left dir entry (left: {baseline:?})"))?;

            candidate.push(left.file_name());

            let metadata = left.metadata().context("Failure reading file metadata")?;

            if !candidate.exists() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!(
                        "Right counterpart does not exist: left: {:?}, right: {:?}, left meta: {:?}",
                        left.path(),
                        candidate,
                        metadata
                    ),
                )
                .into());
            }

            if metadata.is_file() {
                // Recursion end
                let left_contents = std::fs::read(left.path())
                    .with_context(|| format!("Failure reading left file: {:?}", left.path()))?;
                let right_contents = std::fs::read(&candidate)
                    .with_context(|| format!("Failure reading right file: {candidate:?}"))?;

                if left_contents != right_contents {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            r"File contents differ:
left path: {:?}
right path: {:?}
---------
left contents:
{}
---------
right contents:
{}
---------
",
                            left.path(),
                            candidate,
                            String::from_utf8_lossy(&left_contents).escape_debug(),
                            String::from_utf8_lossy(&right_contents).escape_debug()
                        ),
                    )
                    .into());
                }
            } else if metadata.is_dir() {
                // Recursion step
                compare_directories(left.path().clone(), candidate.clone())?;
            } else {
                // Do not silently ignore.
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!(
                        "Unsupported file type for testing, found: {:?}",
                        left.metadata().unwrap()
                    ),
                )
                .into());
            }

            candidate.pop();
        }

        Ok(())
    }

    /// Recursively copies a directory tree from `src` to `dst`.
    fn copy_tree(src: &Path, dst: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;

            if entry.file_type()?.is_dir() {
                copy_tree(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }

        Ok(())
    }

    /// Creates a temporary directory and copies the contents of `src` into it,
    /// returning the path to the newly created directory.
    fn copy_to_tmp(src: &Path) -> TempDir {
        let pkg = env!("CARGO_PKG_NAME");
        assert!(
            !pkg.contains(std::path::MAIN_SEPARATOR),
            // Not like this will ever happen, but always good to encode assumptions
            "Package name contains path separator, which is not advisable for path prefix"
        );

        let tmp_dir = tempfile::Builder::new()
            .prefix(pkg)
            .keep(true) // Keep for manual inspection if needed
            .tempdir()
            .expect("Failed to create temporary directory");

        copy_tree(src, tmp_dir.path()).expect("Failed to copy test files to tempdir");

        // Important: transfer ownership out, else `drop` will delete created dir
        tmp_dir
    }
}
