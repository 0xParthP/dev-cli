# Testing

Tests are part of "done." Every new feature lands with its tests, and the full suite passes locally before the PR opens.

## Where Tests Live

**All tests live in `tests/`.** There are **no** `#[cfg(test)] mod tests` blocks inside `src/`. The `dev_cli` library crate (`src/lib.rs`) re-exports the surface those tests need, and the test files `use dev_cli::…` directly.

This rule is enforced by code review and by the layout of the `src/` modules — none of them have inline unit tests. The reasons:

- Keeping tests in one place makes the suite easier to navigate.
- The library crate is the contract; testing the public surface catches accidental `pub` creep.
- Coverage tooling has a single source of truth to instrument.

Tests that mutate process-wide state (`DEVCLI_CONFIG_DIR`, `DEVCLI_TEST_EXECUTABLE`, `DEVCLI_SKIP_ONBOARDING`, …) are marked `#[serial_test::serial]` so the runner never interleaves them.

## Layout

- **One file per command or service** under `tests/`: `tests/config.rs`, `tests/project.rs`, `tests/launcher.rs`, `tests/onboarding.rs`, `tests/scanner.rs`, …
- **Shared helpers** in `tests/common/`. For example `temp_project::TempProject` creates a temporary project tree, points the right env vars at it, and cleans up on drop.
- **Black-box integration** in the `cli_*` and `main_cli` files: spawn the compiled `dev` binary with `assert_cmd` and assert on its output with `predicates`.

A typical round of changes adds **one new integration test file** for the new command plus any per-test helpers in `tests/common/`.

## Integration Tests

These run the actual CLI binary. They catch problems the unit tests can't, like argument parsing, exit codes, and end-to-end output.

`tests/cli_config.rs` is a canonical example:

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

`tests/launcher.rs` shows the pattern when a test needs to drive the IDE process without actually opening an editor:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use common::temp_project::TempProject;

#[test]
fn open_launches_ide_for_known_project() {
    let _project = TempProject::new("demo");     // sets DEVCLI_TEST_EXECUTABLE
    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_TEST_EXECUTABLE", /* stubbed path */)
        .arg("open")
        .arg("demo")
        .assert()
        .success();
}
```

`tests/launcher.rs` serialises access to `DEVCLI_TEST_EXECUTABLE` with a `static MUTEX` (same pattern as `tests/install.rs`) so two tests that both want to stub the IDE binary don't clobber each other.

### Predicates

The `predicates` crate covers most assertions:

```rust
.stdout(predicate::str::contains("default_ide"))   // substring
.stderr(predicate::str::is_empty())                 // no output on stderr
.success()                                           // exit 0
.failure()                                           // non-zero exit
```

### Temporary Files

Use `tempfile::TempDir` for tests that need a real filesystem. The directory is removed when the `TempDir` is dropped. `tests/common/temp_project.rs` wraps the common "spin up a fake project tree" pattern.

## Service-Level Tests

For services like `scanner::discover_projects` or `onboarding::is_interactive_terminal`, the tests `use dev_cli::…` to call the function directly and assert on the return value. These tests are deterministic and don't shell out.

`tests/onboarding.rs` is the canonical example of mixing both styles: the interactive wizard body is `#[cfg(not(coverage))]` so it doesn't count against the 80% line-coverage gate, and the helper `is_interactive_terminal` is exercised directly with `DEVCLI_SKIP_ONBOARDING=1` and a TTY flag.

## What Lives Where

| Code under test | Test files |
|-----------------|------------|
| `Config`, `Ide`, `Project` | `tests/config.rs`, `tests/path.rs` |
| `dev config …` | `tests/cli_config.rs`, `tests/commands_config.rs` |
| `dev project …`, `dev open …` | `tests/cli_open.rs`, `tests/project.rs`, `tests/project_commands.rs` |
| IDE detection | `tests/ide_detect.rs` |
| IDE launching | `tests/launcher.rs` |
| Onboarding wizard | `tests/onboarding.rs` |
| `src/scanner.rs` | `tests/scanner.rs` |
| Path handling | `tests/path.rs` |
| Top-level dispatch | `tests/main_cli.rs` |

The split isn't rigid — add a new test file when a new command lands.

## Error Cases

A test that only covers the happy path isn't enough. For every public function and every command, write at least one test that exercises the failure path: missing config, missing project, invalid input. These tests are what catch regressions when the codebase is refactored.

## Running the Suite

```bash
cargo test                            # everything
cargo test --test config              # one integration test file
cargo test -- --nocapture             # show output
```

`cargo xtask ci` runs the full pre-commit suite: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and the 80% line-coverage gate. See [xtask.md](xtask.md).

## Coverage

`cargo xtask ci` enforces an **80% line-coverage** minimum using `cargo-llvm-cov`. Coverage is wired in `xtask/src/main.rs` (`coverage_step(80.0)`) and the report shows the actual percentage at the end of a run.

The interactive body of the onboarding wizard is gated `#[cfg(not(coverage))]` so a manual `cargo test` doesn't drag coverage down — the TTY-gated code is exercised by hand, not by the suite.

Locally:

```bash
cargo coverage            # HTML report
cargo coverage-summary    # one-line percentage
cargo coverage-lcov       # machine-readable for tooling
```

## Guidelines

A few rules that keep the suite healthy:

- **Test behavior, not implementation.** If you refactor the internals, the tests should still pass.
- **One assertion focus per test.** Multiple `assert_eq!` lines are fine when they all check the same behavior; separate tests when they're checking different behaviors.
- **No shared state between tests.** Each test sets up its own temp project, its own config, its own env. Tests must pass in any order, in parallel — unless they're marked `#[serial]`.
- **No network access.** The suite must run offline.
- **No real user config.** `Config::load()` should never touch the developer's actual `~/.config/dev-cli/config.toml`. Tests set `DEVCLI_CONFIG_DIR` to a temp dir so the loader is pointed at a sandbox.
- **If you must `set_var`, mark `#[serial]`.** Cargo's test runner runs tests on multiple threads; env-var mutations cross threads without `serial_test`.

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — the pre-commit checklist
- [docs/project-structure.md](project-structure.md) — where the test files live
- [docs/xtask.md](xtask.md) — the `cargo xtask ci` runner
- [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/) — the binary-spawning crate we use
- [predicates](https://docs.rs/predicates/latest/predicates/) — assertion combinators
- [serial_test](https://docs.rs/serial_test/latest/serial_test/) — runs marked `#[serial]` one at a time
