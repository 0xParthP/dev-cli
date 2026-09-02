# Coding Conventions

This document outlines the coding standards for `dev-cli`.

---

## Formatting & Linting

**REQUIRED before commit:**

```bash
cargo fmt              # Format all code
cargo clippy           # Run linter
cargo test             # Run all tests
cargo doc --no-deps    # Generate documentation
```

## Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Modules | `snake_case` | `ide_detection` → `ide_detection.rs` |
| Types | `PascalCase` | `struct Config`, `enum Ide` |
| Functions | `snake_case` | `fn detect_ides()` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_RETRIES: u32 = 3;` |
| Variables | `snake_case` | `let project_name = ...` |

## Error Handling

**STRICT:** Never use `unwrap()` in production code. Use `anyhow::Context` and `?` for propagation.

```rust
// ✅ REQUIRED
let config = Config::load()
    .context("Failed to load configuration")?;
```

## Documentation

- Every public API MUST have rustdoc comments (`///`).
- Every module MUST have module-level documentation (`//!`).
