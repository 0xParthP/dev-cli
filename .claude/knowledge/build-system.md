# Build System

> How `dev-cli` builds, checks, measures coverage, and ships. Ground truth: `Cargo.toml`, `.cargo/config.toml`, `xtask/`, `.github/workflows/`.

## Workspace Layout

- **Cargo workspace** with members `.` (`dev-cli`) and `xtask`.
- Binary name: `dev` (`[[bin]] name = "dev", path = "src/main.rs"`).
- Library crate: `dev_cli` (implicit from `src/lib.rs`).
- `resolver = "2"`, edition **2024**, MSRV **1.88** (`clippy.toml`).

## Cargo Aliases (`.cargo/config.toml`)

| Alias | Expands to |
|-------|-----------|
| `cargo fmt-check` | `fmt --all -- --check` |
| `cargo lint` | `clippy --workspace --all-targets --all-features -- -D warnings` |
| `cargo test-all` | `nextest run --workspace --all-features` |
| `cargo coverage` | `llvm-cov --workspace --exclude xtask --all-features --html --open` |
| `cargo coverage-summary` | `llvm-cov ... --summary-only` |
| `cargo coverage-lcov` | `llvm-cov ... --lcov --output-path lcov.info` |
| `cargo coverage-clean` | `llvm-cov clean --workspace` |
| `cargo security` | `deny check` |
| `cargo xtask` | `run --package xtask --` |

## XTask (`xtask/`)

- `cargo xtask ci` — runs, in order: fmt-check, lint, security, test-all, coverage gate (≥80% lines), then prints a PASS/FAIL report with timing.
- `cargo xtask coverage` / `cargo xtask coverage-summary` — thin wrappers.

## Tooling Config

| File | Purpose |
|------|---------|
| `rustfmt.toml` | edition 2024, max_width 100, small heuristics Max, tab_spaces 4, Native newlines |
| `clippy.toml` | msrv 1.88, too-many-arguments-threshold 5, type-complexity-threshold 250 |
| `deny.toml` | cargo-deny: license allowlist (MIT/Apache-2.0/Unicode-3.0/MPL-2.0), wildcards denied, unknown-registry denied |
| `nextest.toml` | fail-fast off, num-cpus threads, slow-timeout 5s/2 terminates |

## CI Pipeline (`.github/workflows/`)

| Workflow | Triggers | What it does |
|----------|----------|--------------|
| `ci.yml` | PR → main, push → main | fmt-check, lint, `cargo test-all`, release build (ubuntu + windows matrix, `cargo-nextest` via taiki-e) |
| `coverage.yml` | any PR, push → main | llvm-cov HTML + LCOV artifacts, step-summary, **enforce ≥80% line coverage** |
| `security.yml` | PR → main | `cargo deny check` + `cargo audit` |
| `branch-name.yml` | push (non-main), PR → main | branch must match `^feature/[a-z0-9]+(-[a-z0-9]+)*$` |
| `release.yml` | tag `v*` | build for windows-msvc, linux-gnu, macos darwin; publish via softprops/action-gh-release |

Actions used: `actions/checkout@v5`, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`.

## Local Full Check (same as CI)

```bash
cargo xtask ci          # or, step by step:
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo nextest run --workspace --all-features
cargo llvm-cov --workspace --exclude xtask --all-features --summary-only   # ≥ 80%
```

## Release Pipeline

1. Tag `v*` → `release.yml`.
2. Build per-OS with `--target <triple>`.
3. `softprops/action-gh-release` attaches binaries to the GitHub release.
