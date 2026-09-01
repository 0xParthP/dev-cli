# Testing

Tests are part of "done." Every new feature lands with its unit and integration tests, and the full suite passes locally before the PR opens.

## Layout

We follow the standard Rust split:

- **Unit tests** live next to the code they exercise, in a `#[cfg(test)] mod tests` block at the bottom of the file. They have access to private items.
- **Integration tests** live in `tests/`, one file per top-level command. Each file spawns the compiled `dev` binary as a subprocess with `assert_cmd` and asserts on its output with `predicates`.

A typical round of changes adds tests in both places: a unit test for the new pure function, an integration test for the user-facing command.

## Unit Tests

Keep them small and focused on one behavior. The Arrange / Act / Assert shape is a good default.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_uses_vscode() {
        // Arrange + Act
        let config = Config::default();

        // Assert
        assert_eq!(config.default_ide, Ide::Vscode);
        assert!(!config.projects_root.is_empty());
    }

    #[test]
    fn ide_parses_from_string() {
        assert_eq!("vscode".parse::<Ide>(), Ok(Ide::Vscode));
        assert_eq!("cursor".parse::<Ide>(), Ok(Ide::Cursor));
    }
}
```

Name tests after the behavior: `default_config_uses_vscode`, not `test1`. When a test fails, the name should tell you what's wrong without opening the file.

Run a single test or a module with `cargo test <name>` or `cargo test --lib <module>::tests`. Add `-- --nocapture` to see `println!` output.

## Integration Tests

These run the actual CLI binary. They catch problems the unit tests can't, like argument parsing, exit codes, and end-to-end output.

`tests/config.rs` is the canonical example:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_show_contains_default_ide() {
    Command::cargo_bin("dev")
        .unwrap()
        .arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}
```

`tests/launcher.rs` shows the pattern when a test needs a real project tree on disk:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use common::temp_project::TempProject;

#[test]
fn open_launches_ide_for_known_project() {
    let project = TempProject::new("demo");
    // writes a fake project under a temp dir
    // points the env at it so Config::load() finds it

    Command::cargo_bin("dev")
        .unwrap()
        .arg("open")
        .arg("demo")
        .assert()
        .success();
}
```

Shared helpers live in `tests/common/`. `temp_project::TempProject` creates a temporary project tree, sets the right env vars so `Config::load()` finds it, and cleans up on drop. `assertions` has reusable predicate chains.

### Predicates

The `predicates` crate covers most assertions:

```rust
.stdout(predicate::str::contains("default_ide"))   // substring
.stderr(predicate::str::is_empty())                 // no output on stderr
.success()                                           // exit 0
.failure()                                           // non-zero exit
```

### Temporary Files

Use `tempfile::TempDir` for tests that need a real filesystem. The directory is removed when the `TempDir` is dropped.

## What Lives Where

| Code under test | Test files |
|-----------------|------------|
| `Config`, `Ide`, `Project` | unit tests in `src/config.rs`, `src/models/ide.rs` |
| `dev config …` | `tests/config.rs`, `tests/commands_config.rs` |
| `dev project …`, `dev open …` | `tests/project.rs`, `tests/project_commands.rs` |
| IDE launching | `tests/launcher.rs` |
| `dev install` | `tests/install.rs`, `tests/commands_install.rs` |
| Top-level dispatch | `tests/main_cli.rs` |
| `src/scanner.rs` | `tests/scanner.rs` |
| Path handling | `tests/path.rs` |

The split isn't rigid — add a new test file when a new command lands.

## Error Cases

A test that only covers the happy path isn't enough. For every public function and every command, write at least one test that exercises the failure path: missing config, missing project, invalid input. These tests are what catch regressions when the codebase is refactored.

## Running the Suite

```bash
cargo test                            # everything
cargo test --lib                      # unit tests only
cargo test --test config              # one integration test file
cargo test -- --nocapture             # show output
```

`cargo xtask check` runs the full pre-commit suite: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo doc --no-deps`. See [xtask.md](xtask.md).

## Coverage

We track coverage through `cargo llvm-cov` in CI; locally, `cargo xtask coverage` (or `cargo llvm-cov nextest --html` directly) writes an HTML report. We don't gate PRs on a number — the goal is to make sure new code has tests, not to chase a percentage. Use the report to find the obvious gaps.

## Guidelines

A few rules that keep the suite healthy:

- **Test behavior, not implementation.** If you refactor the internals, the tests should still pass.
- **One assertion focus per test.** Multiple `assert_eq!` lines are fine when they all check the same behavior; separate tests when they're checking different behaviors.
- **No shared state between tests.** Each test sets up its own temp project, its own config, its own env. Tests must pass in any order, in parallel.
- **No network access.** The suite must run offline.
- **No real user config.** `Config::load()` should never touch the developer's actual `~/.config/dev-cli/config.toml`. Tests use `temp_project` to point at a sandbox.

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — the pre-commit checklist
- [docs/project-structure.md](project-structure.md) — where the test files live
- [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/) — the binary-spawning crate we use
