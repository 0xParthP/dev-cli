---
name: review-pr
description: Reviews a dev-cli PR/diff against project conventions (unwrap ban, rustdoc, layer boundaries, fmt/clippy/test). Triggered by "review this PR", "review my changes", or a request to check a diff for compliance.
---

# `review-pr`

You are reviewing a pull request or uncommitted diff for `dev-cli`. Ground the review in `.claude/knowledge/conventions.md` and `.claude/knowledge/testing.md`.

## Review Checklist

Run the following checks against the diff (and `src/` where noted). Treat the first two as blockers.

1. **No `.unwrap()` in `src/`** (blocker)
   - Grep `src/` for `unwrap()` / `expect(`. Allowed: documented init only (e.g. `BaseDirs::new().expect(...)`).
   - Flag any other occurrence; must be replaced with `?` + `.context("...")` via `anyhow`.

2. **Rustdoc on public APIs** (blocker)
   - Every `pub fn` / `pub struct` / `pub enum` / `pub mod` added or changed must have `///` (or `//!` for modules).
   - Flag missing doc comments.

3. **Architecture boundary violations**
   - Commands (`src/commands/*`) must NOT contain business logic, direct fs I/O, or process spawning — they orchestrate services and format output.
   - `src/models/*` must NOT import services/commands.
   - Services must NOT import `commands/*`.
   - Flag any module importing from the wrong layer (see `.claude/knowledge/modules.md` for the import matrix).

4. **Testing compliance**
   - New public function → in-source `#[cfg(test)]` unit test.
   - New command → integration test in `tests/commands_*.rs` (happy + error path), using isolation env vars where applicable (`DEVCLI_CONFIG_DIR`, `DEVCLI_INSTALL_DIR`, `DEVCLI_TEST_EXECUTABLE`).
   - Coverage must stay ≥80% line coverage.

## Suggestions

- Remind the user that CI enforces `cargo fmt && cargo clippy -- -D warnings && cargo test && cargo doc --no-deps`. Offer to run the `pre-flight` skill to verify.

## Output Format

- **Blockers** (must fix) — the `.unwrap()`, missing rustdoc, or layer violation with file:line.
- **Warnings** — riskier/testability concerns.
- **Suggestions** — style, reuse, docs.
- End with a one-line verdict: approve, needs-changes, or not-ready.
