---
name: generate-tests
description: Generates unit and integration tests for a new dev-cli command or function. Triggered when the user asks to add tests, "write tests for X", or "how do I test this".
---

# `generate-tests`

You are writing tests for `dev-cli`. Ground the work in `.claude/knowledge/testing.md` (test-file map, isolation env vars, coverage gate).

## Where Tests Live

1. **Unit tests** — `#[cfg(test)] mod tests` **inside the source file** (e.g. `src/config.rs`). Use for pure functions/`Config` round-trips.
2. **Integration tests** — `tests/commands_*.rs` (new command) or an existing file from the map in `.claude/knowledge/testing.md`. Spawn the binary via `assert_cmd::Command` + `predicates`.
3. **Shared helpers** — reuse `tests/common/` (`temp_config.rs` → `test_config()`, `temp_project.rs` → `TempProject::create_git_repo(name)`, `assertions.rs` → `CliAssert`).

## Isolation Env Vars

Always isolate external side effects — set these via env (they point `Config::path()` / installer / launcher at temp locations):

- `DEVCLI_CONFIG_DIR` — temp dir for `Config::path()` (config tests).
- `DEVCLI_INSTALL_DIR` — temp install dir; serialise env mutation with a static `Mutex`.
- `DEVCLI_TEST_EXECUTABLE` — fake `.bat`/exe so `launch` never spawns a real IDE.

> ⚠️ Do **not** model new tests on `tests/cli_config.rs` / `tests/cli_ide.rs` — they touch the real user config / real IDE detection. Prefer the isolated variants.

## Test Content

- **New function** → unit test: happy path, edge/empty case, error case.
- **New command** → integration test: happy path (`success_contains(...)`) + error path (unknown arg / missing project → failure).
- Every test must be hermetic: no reliance on the developer's real config or installed IDEs.

## Coverage Gate

- Project gate: **≥80% line coverage**, enforced in CI (`coverage.yml`). After adding tests, suggest running `cargo llvm-cov` (or `cargo xtask coverage`) to verify the gate holds.

## Workflow

1. Read the source the tests target.
2. Pick unit vs integration per the rules above.
3. Write the tests, then confirm `cargo test <name>` passes.
