# XTask

`xtask` is a small Cargo workspace member that bundles the dev-time workflows (`ci`, `coverage`, `coverage-summary`, `install`, `security`) into a single Rust binary. It's the recommended entry point for everything CI runs, plus a few things only developers do locally.

## Layout

`xtask/` is a sibling crate to the main `dev-cli` package — both are listed under `workspace.members` in the root `Cargo.toml`. `xtask` re-uses the same dependencies where it can and adds `cargo-llvm-cov` for coverage.

```bash
xtask/
├── Cargo.toml
└── src/
    └── main.rs
```

`src/main.rs` is a single `match` on a small `Commands` enum. Each arm runs one of the workflows below. There is no business logic in `xtask` — it's a thin orchestrator over `cargo`, `cargo clippy`, `cargo fmt`, `cargo llvm-cov`, and friends.

## Commands

| Command | Description |
|---------|-------------|
| `cargo xtask ci` | Run the canonical pre-commit / CI check: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and the 80% line-coverage gate. |
| `cargo xtask coverage` | Generate an HTML coverage report via `cargo-llvm-cov`. |
| `cargo xtask coverage-summary` | Print a one-line coverage summary to the terminal. |
| `cargo xtask install` | Copy the freshly-built `dev` binary to `~/.local/bin/`. |
| `cargo xtask security` | Run `cargo audit` (if installed) and a `cargo deny check` pass. |

The Cargo aliases in `.cargo/config.toml` give you shorter names for the most common ones:

```bash
cargo fmt-check        # cargo fmt -- --check
cargo lint             # cargo clippy -- -D warnings
cargo test-all         # cargo test --all
cargo coverage         # cargo xtask coverage
cargo coverage-summary # cargo xtask coverage-summary
cargo coverage-lcov    # cargo llvm-cov --lcov --output-path coverage/lcov.info
cargo coverage-clean   # cargo llvm-cov clean
cargo security         # cargo xtask security
cargo xtask            # cargo run -p xtask --
```

## `cargo xtask ci`

This is the one command you run before every commit. It mirrors the GitHub Actions `ci.yml` job:

1. `cargo fmt -- --check` — formatting must be clean.
2. `cargo clippy -- -D warnings` — no clippy warnings.
3. `cargo test` — the full test suite.
4. `cargo llvm-cov` — the 80% line-coverage gate. The threshold is set in `xtask/src/main.rs` as `coverage_step(80.0)`. A drop below 80% fails the run.
5. `cargo doc --no-deps` — generated rustdoc must be warning-free.

Any failure exits non-zero; the run is short-circuit.

## `cargo xtask install`

Builds the release binary if necessary and copies it to `~/.local/bin/dev[.exe]`.

## `cargo xtask coverage` / `coverage-summary`

Wrappers around `cargo llvm-cov`. The HTML report is written to `target/llvm-cov/html/`; the summary line is `Coverage X.XX% (R/S lines)`. `coverage-lcov` writes `coverage/lcov.info` for tooling like `genhtml`, Codecov, or Sonar.

## Why xtask?

- **Cross-platform.** Pure Rust, no shell scripts, same behaviour on Windows, macOS, and Linux.
- **Same command locally and in CI.** GitHub Actions invokes `cargo xtask ci` from `.github/workflows/ci.yml`, so a green local run is a green PR.
- **Extensible.** New developer workflows go here — release prep, branch-name checks, `cargo deny`, anything that needs to run from a script without bespoke shell.
- **Type-safe.** Adding a new subcommand is a new variant on the `Commands` enum and a new match arm. Typos and missing arms are caught at compile time.

## Adding a new xtask command

1. Add a variant to `Commands` in `xtask/src/main.rs` and the args struct.
2. Add a `match` arm that runs the workflow.
3. Document it in the table above.
4. (Optional) Add a Cargo alias to `.cargo/config.toml` if it's something you'll run often.
5. Update `CONTRIBUTING.md` if the command is part of the pre-commit checklist.

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — the pre-commit checklist
- [docs/testing.md](testing.md) — what `cargo xtask ci` actually runs
- [.cargo/config.toml](../.cargo/config.toml) — the aliases
- [xtask/src/main.rs](../xtask/src/main.rs) — the actual implementation
