# Testing

> Where tests live, what they cover, and the conventions. Ground truth: `tests/`, in-source `#[cfg(test)]`, `docs/testing.md`.

## Layers of Testing

1. **Unit tests** — `#[cfg(test)] mod tests` inside the source file.
2. **Integration tests** — `tests/*.rs` compile against the `dev_cli` library crate and/or spawn the `dev` binary via `assert_cmd`.

## Test Files

| File | Coverage |
|------|----------|
| `tests/main_cli.rs` | root `--help`, `--version`, invalid command fails |
| `tests/cli_config.rs` | `config show/init/--help` (no config-dir isolation — reads real config) |
| `tests/commands_config.rs` | `config init/show` isolated via `DEVCLI_CONFIG_DIR` |
| `tests/cli_ide.rs` | `ide list` runs |
| `tests/cli_open.rs` | `open` errors on unknown project, `--help` works |
| `tests/project_commands.rs` | `project list` with temp config + fake repos |
| `tests/scanner.rs` | `discover_projects`: empty dir, single/multi repo, `node_modules` ignored, duplicate roots |
| `tests/project.rs` | `discover_projects` duplicate coverage of scanner basics |
| `tests/config.rs` | Config round-trip, defaults, invalid TOML |
| `tests/install.rs` | installer with `DEVCLI_INSTALL_DIR` isolation (mutex-serialised env vars) |
| `tests/launcher.rs` | `launch` with `DEVCLI_TEST_EXECUTABLE` fake launcher |
| `tests/path.rs` | `display_path` |
| `tests/commands_install.rs` | `commands::install::execute` returns Ok |

## Shared Helpers (`tests/common/`)

- `assertions.rs` — `contains_usage()`, `CliAssert` trait (`success_contains`).
- `temp_config.rs` — `test_config()` (hardcoded `C:/Projects`, Vscode).
- `temp_project.rs` — `TempProject` creates a temp dir + fake `.git` repos via `create_git_repo(name)`.

## Isolation Mechanisms

- **Config** → `DEVCLI_CONFIG_DIR` env var points `Config::path()` at a temp dir.
- **Installer** → `DEVCLI_INSTALL_DIR`; tests serialise env mutation with a static `Mutex`.
- **Launcher** → `DEVCLI_TEST_EXECUTABLE` points at a fake `.bat`/exe so no real IDE spawns.
- **Scanner** → `TempProject` + tempfile; fake `.git` directories (no real `git init` needed).

> ⚠️ `tests/cli_config.rs` and `tests/cli_ide.rs` do **not** set `DEVCLI_CONFIG_DIR`/isolation — they touch the real user config / real IDE detection. Prefer the isolated variants when adding tests.

## Requirements (from CLAUDE.md / AGENTS.md)

- Every new public function → unit test.
- Every new command → integration test (happy + error path).
- Run locally: `cargo test` (or `cargo test-all` = nextest).
- Coverage gate: **≥80% line coverage** enforced in CI (`coverage.yml`).

## Running Tests

```bash
cargo test                        # lib + all integration tests
cargo test-all                    # nextest, all features, whole workspace
cargo test <name>                 # filter
cargo test -- --nocapture         # show output
cargo llvm-cov ... --summary-only # coverage report
```
