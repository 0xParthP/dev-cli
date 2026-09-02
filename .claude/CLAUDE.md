# CLAUDE.md

Instructions for AI coding assistants (Claude, GitHub Copilot, etc.) working within this repository.

---

## Quick Reference

- **Project:** `dev-cli` — A developer project manager CLI written in Rust
- **Language:** Rust (2024 edition)
- **Paradigm:** Layered architecture with clear separation of concerns
- **Status:** Active development (Sprint 1.7 - Documentation Pass)

---

## Project Overview

`dev-cli` is a Windows-first command-line tool that helps developers manage and quickly launch Git repositories with their preferred IDE (VS Code, Cursor, Claude Code, etc.).

### Core Functionality
1. Maintains configuration of project root directories
2. Discovers and searches for Git repositories
3. Detects installed IDEs on the system
4. Launches projects in the specified IDE
5. Manages user configuration via TOML files

### Key Characteristics
- ⚡ Minimal startup time (< 50ms)
- 🎯 Single-file focused (not a package manager)
- 🔧 Extensible command structure
- 🛡️ Type-safe with Rust's guarantees
- 📝 Well-documented with rustdoc

---

## Architecture

### Layered Architecture

```
┌─────────────────────────────────────┐
│     CLI Layer (src/cli.rs)          │
│  Clap-based argument parsing        │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Commands Layer (src/commands/*)    │
│  Command handlers & orchestration   │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Services Layer (src/*.rs)          │
│  Business logic & system interaction │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Models Layer (src/models/*)        │
│  Data structures (Ide, Project)     │
└─────────────────────────────────────┘
```

### Directory Responsibilities

| Directory | Responsibility |
|-----------|-----------------|
| `src/` | Source code |
| `src/main.rs` | Entry point; CLI dispatch |
| `src/cli.rs` | Clap argument definitions |
| `src/commands/` | Command implementations |
| `src/config.rs` | Configuration management |
| `src/ide/` | IDE detection & launching |
| `src/models/` | Data structures |
| `src/scanner.rs` | Repository discovery |
| `src/installer.rs` | Installation logic |
| `tests/` | Integration tests |
| `docs/` | User and developer guides |

### Key Types

| Type | Module | Purpose |
|------|--------|---------|
| `Cli` | cli.rs | Top-level command parser |
| `Commands` | cli.rs | Enum of all commands |
| `Config` | config.rs | User configuration |
| `Ide` | models/ide.rs | Supported IDE enum |
| `InstalledIde` | ide/registry.rs | Detected IDE info |
| `Project` | models/project.rs | Project metadata (name + path) |

---

## Coding Standards

### Formatting & Linting

**REQUIRED before commit:**

```bash
cargo fmt              # Format all code
cargo clippy           # Run linter
cargo test             # Run all tests
cargo doc --no-deps    # Generate documentation
```

**CI enforces these.** PRs will fail without compliance.

### Documentation Requirements

**Every public API MUST have rustdoc comments:**

```rust
/// Brief summary of what this does.
///
/// More detailed explanation if complex.
///
/// # Arguments
/// * `name` - What this parameter does
///
/// # Returns
/// What this function returns
///
/// # Errors
/// When/why this can fail
///
/// # Example
/// ```
/// let result = my_function("value")?;
/// ```
pub fn my_function(name: &str) -> Result<String> {
    // implementation
}
```

**Every module MUST have `//!` documentation:**

```rust
//! Brief module description.
//!
//! # Responsibilities
//! - What this module does
//! - Key responsibilities
//!
//! # Important Types
//! - `SomeType` — what it does
```

### Error Handling Rules

**STRICT:** Never use `unwrap()` in production code without justification.

```rust
// ❌ NOT ALLOWED (except in rare justified cases)
let config = Config::load().unwrap();

// ✅ REQUIRED
let config = Config::load()
    .context("Failed to load configuration")?;

// ✅ OK: Initialization only (with comment)
let home = BaseDirs::new()
    .expect("home directory must exist");
```

**Error propagation:** Use `?` operator with `.context()` for clarity:

```rust
fn do_something() -> Result<String> {
    let config = Config::load()
        .context("Could not load configuration")?;
    
    let ide = config.default_ide;
    
    Ok(format!("Using {}", ide))
}
```

### Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Modules | `snake_case` | `ide_detection` → `ide_detection.rs` |
| Types | `PascalCase` | `struct Config`, `enum Ide` |
| Functions | `snake_case` | `fn detect_ides()` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_RETRIES: u32 = 3;` |
| Variables | `snake_case` | `let project_name = ...` |

### Module Size Guidelines

- **Target:** 200-300 lines per file
- **Maximum:** 500 lines before splitting into submodules
- **Minimum:** Don't create trivial single-function modules

### Comment Style

```rust
// ✅ Good: Explains intent, not just repeating code
// We check common Windows locations first since most IDEs
// are installed there, avoiding unnecessary PATH scanning

// ❌ Bad: Just repeats what the code does
// Check if the file exists
if path.exists() { }

// ✅ Good: Explains non-obvious behavior
// Ide::Vscode can be in PATH or standard location,
// so we check both to avoid duplicates
```

---

## Testing Requirements

**STRICT:** Every feature must have tests.

### Unit Tests

Add within the source file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.default_ide, Ide::Vscode);
    }
}
```

### Integration Tests

Add in `tests/cli_*.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_show_works() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}
```

### Test Requirements

- ✅ New functions must have unit tests
- ✅ New commands must have integration tests
- ✅ Error cases must be tested
- ✅ All tests must pass before merging
- ✅ Run: `cargo test` locally before PR

---

## When Creating New Commands

**Follow this checklist:**

1. **Define CLI arguments** in `src/cli.rs`
   - Add variant to `Commands` enum
   - Create `SomeCommand` struct with `#[derive(Args)]`
   - Document with rustdoc comments

2. **Create command handler** in `src/commands/newcmd.rs`
   - Implement `pub fn execute(cmd: SomeCommand) -> Result<()>`
   - Call service layer (don't implement logic here)
   - Format output for terminal display

3. **Export handler** in `src/commands/mod.rs`
   - Add `pub mod newcmd;`

4. **Dispatch in main** in `src/main.rs`
   - Add match arm: `Commands::Something(cmd) => commands::newcmd::execute(cmd)?`

5. **Add tests** in `tests/cli_newcmd.rs`
   - Test happy path
   - Test error cases
   - Use `assert_cmd` for subprocess testing

6. **Update documentation**
   - Add to README.md command reference
   - Update CHANGELOG.md
   - Update relevant docs/ guides

7. **Run checks**
   ```bash
   cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
   ```

---

## When Creating New Modules

**Follow this structure:**

```rust
// src/mymodule.rs
//! Brief module description.
//!
//! # Responsibilities
//! - What this does
//!
//! # Important Types
//! - `MyType` — description
//!
//! # Example
//! ```
//! let result = do_something()?;
//! ```

use anyhow::Result;

/// Brief description of this type.
pub struct MyType {
    pub field: String,
}

impl MyType {
    /// Create a new instance.
    pub fn new(field: String) -> Self {
        Self { field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let obj = MyType::new("test".to_string());
        assert_eq!(obj.field, "test");
    }
}
```

**In `src/mod.rs` or `src/main.rs`:**

```rust
mod mymodule;
pub use mymodule::{MyType};
```

---

## When Updating Documentation

**Guidelines:**

1. **Rustdoc comments** — Run `cargo doc --no-deps --open` to verify rendering
2. **Module docs** — Use `//!` at the top of every file
3. **Examples** — Include code examples in rustdoc with triple backticks
4. **Cross-references** — Link to related types/functions with `[`Type`]`
5. **Guides** — Update relevant files in `docs/` directory

**Files to update for different changes:**

- New command: README.md, CHANGELOG.md, relevant docs/ guide
- Architectural change: ARCHITECTURE.md, AGENTS.md
- Configuration change: docs/configuration.md, CHANGELOG.md
- Module change: docs/project-structure.md, module rustdoc

---

## Dependency Management

### Current Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `clap` | CLI parsing | 4.5 |
| `serde` | Serialization | 1.0 |
| `toml` | TOML parsing | 0.9 |
| `directories` | Platform-aware paths | 6 |
| `anyhow` | Error handling | 1.0 |
| `which` | Find executables in PATH | 8 |
| `owo-colors` | Terminal colors | 4 |
| `regex` | Pattern matching | 1.0 |
| `ignore` | Git ignore handling | 0.4 |
| `tracing` | Logging framework | 0.1 |

### Adding Dependencies

**Before adding a new crate:**

1. Check if existing dependency can do the job
2. Evaluate maintenance status on crates.io
3. Consider compile time impact
4. Discuss in PR if adding significant dependencies

**Preferred philosophy:**
- ✅ Use proven, well-maintained crates (e.g., `serde`, `clap`)
- ⚠️ Avoid crates with many transitive dependencies
- ❌ Don't add crates for trivial functionality

---

## Rust Concepts Used

This repository demonstrates:

- **Modules** — Organize code into logical units
- **Enums** — Model different IDE types, command variants
- **Structs** — Configuration, project metadata
- **Traits** — Serialization (Serialize, Deserialize, ValueEnum)
- **Derive Macros** — Reduce boilerplate (Parser, ValueEnum, etc.)
- **Pattern Matching** — Command dispatch with match
- **Result & Option** — Error handling and optional values
- **Ownership** — Borrowed vs owned values
- **Lifetimes** — Where needed for borrowed data
- **Module paths** — `use` statements and module organization
- **?  Operator** — Clean error propagation

See `docs/rust-for-dev-cli.md` for detailed explanations.

---

## Common Patterns

### Loading Configuration

```rust
fn do_something() -> Result<()> {
    let config = Config::load()?;  // Auto-creates if missing
    println!("{:?}", config);
    Ok(())
}
```

### Detecting and Launching IDEs

```rust
fn launch_in_ide() -> Result<()> {
    let ides = ide::detect_ides();
    
    for ide in ides {
        println!("Found: {}", ide.name);
    }
    
    let my_ide = Ide::Cursor;
    ide::launcher::launch(my_ide, &project_path)?;
    
    Ok(())
}
```

### Error Context

```rust
let config_path = Config::path()
    .context("Couldn't determine config directory")?;

let content = fs::read_to_string(&config_path)
    .context("Failed to read configuration file")?;

let config: Config = toml::from_str(&content)
    .context("Configuration file is not valid TOML")?;
```

---

## Performance Notes

- **CLI startup:** Aim for < 50ms (Rust binary is fast!)
- **Config loading:** Single TOML file parse, very fast
- **IDE detection:** ~100ms (mostly PATH scanning)
- **Project launch:** Limited by external IDE startup

**Optimization strategy:**
- ✅ Lazy load when possible
- ✅ Cache results (but refresh automatically)
- ❌ Don't over-optimize early (measure first)

---

## Testing Locally

```bash
# Build and run
cargo run -- config show

# Test a specific scenario
cargo test test_config_default

# Test with output
cargo test -- --nocapture

# Generate docs
cargo doc --no-deps --open

# Full check (same as CI)
cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

---

## Resources

- **Rust Book:** https://doc.rust-lang.org/book/
- **Rustdoc Guide:** https://doc.rust-lang.org/rustdoc/
- **Clap Documentation:** https://docs.rs/clap/latest/clap/
- **Serde Guide:** https://serde.rs/
- **anyhow docs:** https://docs.rs/anyhow/
- **Project Documentation:** See `docs/` directory

---

## Key Principles

1. **Clear responsibility** — Each module has one clear purpose
2. **Error handling** — All errors have context
3. **Documentation** — Public APIs are documented
4. **Testing** — All features have tests
5. **Simplicity** — Prefer simple solutions over complex ones
6. **Performance** — Startup time matters for CLIs
7. **User experience** — Error messages are helpful

---

## Questions or Issues?

- 📖 Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- 📖 Read [docs/](docs/) for detailed guides
- 💬 Add comments in code for non-obvious logic
- 🐛 Report issues with context and reproduction steps

---

**Happy coding! 🚀**
