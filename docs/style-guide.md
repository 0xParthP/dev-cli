# Style Guide

Consistent, idiomatic Rust makes the codebase approachable and easy to audit.

---

## Formatting

- Run `cargo fmt` before any commit. No manual spacing tweaks.
- **Line length:** keep under 100 characters; hard limit 120.
- **Indentation:** 4 spaces (enforced by `rustfmt`).
- **Blank lines:** one blank line between logical sections; never more than two consecutive blanks.

---

## Naming Conventions

| Item | Convention |
|------|------------|
| Crates / modules / files | `snake_case` (`src/ide/detect.rs`) |
| Types (struct, enum, trait) | `PascalCase` (`Ide`, `Config`) |
| Functions / methods | `snake_case` (`load_config`, `is_valid`) |
| Constants | `SCREAMING_SNAKE_CASE` (`MAX_RETRIES`) |
| Variables / fields | `snake_case` (`project_name`) |
| Modules exported in `mod.rs` | `pub mod <name>;` |

---

## Documentation

Every public item must have rustdoc comments (`///`). Include the following sections where appropriate:

```rust
/// Short one-line summary.
///
/// # Arguments
/// * `path` – Path to the config file.
///
/// # Returns
/// Loaded `Config` instance.
///
/// # Errors
/// Returns an `anyhow::Error` with context if the file cannot be read or parsed.
///
/// # Example
/// ```
/// let cfg = Config::load()?;
/// ```
pub fn load() -> Result<Config> { … }
```

- Modules start with `//!` describing responsibilities.
- Mention any important invariants or error conditions.
- Keep examples short and runnable.

---

## Error Handling

- All fallible functions return `anyhow::Result<T>`.
- Use `.context("...")?` to add human-readable context before propagating.
- **Never** use `unwrap()` in production code. Use `?` with `.context(...)`. `expect()` is fine only for invariants that genuinely cannot fail (e.g. "home directory must exist") and only with a comment.

---

## Common Patterns

### Early Returns

```rust
if arg.is_empty() {
    bail!("argument cannot be empty");
}
```

Reduces nesting and improves readability.

### Option Handling

Prefer combinators over manual `match` when possible:

```rust
let ide = args.ide.unwrap_or(config.default_ide);
```

Use `match` for more complex branching.

### Module Size

- Target 200–300 lines per file.
- Split into submodules when a file approaches 500 lines.
- Don't create trivial single-function modules.

---

## Testing Enforcement

The CI pipeline runs, bundled into the canonical pre-commit command:

```bash
cargo xtask ci
```

Which runs:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
# 80% line-coverage gate (via cargo-llvm-cov)
cargo doc --no-deps
```

All PRs must pass these steps. Tests live **only** in `tests/` — no `#[cfg(test)] mod tests` inside `src/`. Tests that mutate process-wide state (env vars) are marked `#[serial_test::serial]` so the runner never interleaves them.

---

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [rustfmt config](https://rust-lang.github.io/rustfmt/)
- [Clippy lints](https://doc.rust-lang.org/clippy/)
- [docs/testing.md](testing.md) — the full testing rules
- [docs/xtask.md](xtask.md) — `cargo xtask ci` and the other dev commands
