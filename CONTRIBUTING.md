# Contributing to dev-cli

Thank you for your interest in contributing! This document provides guidelines for contributing code, documentation, and bug reports.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Development Workflow](#development-workflow)
4. [Code Standards](#code-standards)
5. [Testing](#testing)
6. [Documentation](#documentation)
7. [Commit Messages](#commit-messages)
8. [Pull Request Process](#pull-request-process)
9. [Adding a New Command](#adding-a-new-command)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Prerequisites

- **Rust 1.70 or later** — Install from [rustup.rs](https://rustup.rs)
- **Git** — Install from [git-scm.com](https://git-scm.com)
- **Text editor** — Any editor, but VS Code recommended
- **Windows, macOS, or Linux** — All platforms supported

### Clone the Repository

```bash
git clone https://github.com/yourusername/dev-cli.git
cd dev-cli
```

### Build and Test

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

---

## Development Setup

### Install Rust Tools

```bash
# Update Rust toolchain
rustup update

# Install additional tools
cargo install cargo-nextest  # Faster test runner (optional)
cargo install cargo-tarpaulin  # Code coverage (optional)
```

### Configure Git Hooks (Optional)

```bash
# Copy pre-commit hook
cp .githooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit

# Or configure git to use .githooks
git config core.hooksPath .githooks
```

### Useful VS Code Extensions

- **rust-analyzer** — Rust language support
- **Clippy** — Linting support
- **Better TOML** — TOML syntax highlighting
- **Markdown Preview** — Markdown previewing

---

## Development Workflow

### 1. Create a Branch

Use descriptive branch names following this pattern:

```
feat/<feature-name>     # New feature
fix/<bug-name>          # Bug fix
docs/<doc-name>         # Documentation
refactor/<area>         # Code refactoring
chore/<maintenance>     # Maintenance tasks
```

**Examples:**
```bash
git checkout -b feat/auto-project-discovery
git checkout -b fix/ide-detection-windows
git checkout -b docs/cli-guide
```

### 2. Make Changes

Edit files and write tests as you go.

```bash
# Build frequently to catch errors early
cargo build

# Check code without building
cargo check

# Watch for changes
cargo watch -x build
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

### 4. Format Code

**Before committing, always format:**

```bash
cargo fmt
```

This ensures consistent code style across the project.

### 5. Run Linter

**Address all clippy warnings:**

```bash
cargo clippy
```

**Fix issues automatically (when possible):**

```bash
cargo clippy --fix
```

### 6. Check Documentation

**Ensure your code is documented:**

```bash
# Generate docs and open in browser
cargo doc --no-deps --open

# Check for missing docs
cargo doc --no-deps 2>&1 | grep -i "warning"
```

### 7. Commit

```bash
git add .
git commit -m "feat: add IDE detection for Rider"
```

See [Commit Messages](#commit-messages) for format details.

### 8. Push and Create PR

```bash
git push origin feat/auto-project-discovery
```

Open a pull request on GitHub. See [Pull Request Process](#pull-request-process).

---

## Code Standards

### Formatting

**All code must be formatted with rustfmt:**

```bash
cargo fmt
```

**Style guide:** See [docs/style-guide.md](docs/style-guide.md)

### Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Crates | `snake_case` | `dev-cli`, `my-lib` |
| Modules | `snake_case` | `mod project; // project.rs` |
| Types | `PascalCase` | `struct Config`, `enum Ide` |
| Functions | `snake_case` | `fn load_config()` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_RETRIES: u32 = 3;` |
| Variables | `snake_case` | `let project_name = "...";` |

### Module Size Guidelines

- **Single file modules:** < 500 lines (prefer < 300)
- **Module folders:** Contain related functionality
- **Large modules:** Split into submodules

### Function Size Guidelines

- **Ideal:** 20-50 lines
- **Maximum:** 150 lines before considering refactor
- **Rule:** If you can't see the whole function on screen, it's too large

### Error Handling

**Never use `unwrap()` in production code except:**

```rust
// OK: Obvious invariant
let file = File::open("fixed_path")
    .expect("internal: fixed_path must exist");

// OK: Initialization (will always succeed)
let dirs = BaseDirs::new().expect("home directory must exist");

// NOT OK: User input validation
let port: u16 = port_str.parse().unwrap();  // ❌ Bad!

// GOOD: User input validation
let port: u16 = port_str.parse()
    .context("port must be a number 1-65535")?;  // ✅ Good!
```

### Comments

- **Use `//` for single-line comments**
- **Use `/* */` for multi-line comments**
- **Document public APIs with `///` (rustdoc)**
- **Use `//!` for module-level docs**

**Good comments explain "why", not "what":**

```rust
// ✅ Good: Explains why
// We check in this order because PATH lookups are fast and most
// common, while Windows registry scans are slower
if let Ok(path) = which(cmd) { /* ... */ }

// ❌ Bad: Repeats what the code does
// Check if command is in PATH
if let Ok(path) = which(cmd) { /* ... */ }
```

### Documentation Comments

Every public item must have rustdoc comments:

```rust
/// Brief description.
///
/// More detailed explanation if needed.
///
/// # Arguments
/// * `arg1` - description
///
/// # Returns
/// Brief description of return value.
///
/// # Errors
/// Describes when this returns an error.
///
/// # Example
/// ```
/// let result = do_something("value")?;
/// ```
pub fn do_something(arg1: &str) -> Result<String> {
    // implementation
}
```

### Module Documentation

Every module must have `//!` documentation:

```rust
//! Brief description of this module.
//!
//! # Responsibilities
//! - Item 1
//! - Item 2
//!
//! # Important Types
//! - `MyType` — description

pub struct MyType { /* ... */ }
```

---

## Testing

### Unit Tests

Write unit tests alongside your code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.default_ide, Ide::Vscode);
    }
}
```

### Integration Tests

Add integration tests in `tests/` directory:

```rust
// tests/cli_new_feature.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn new_command_works() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.arg("new-cmd")
        .arg("arg1")
        .assert()
        .success();
}
```

---

## Documentation Maintenance (Critical)

### The Golden Rule

**Every code change MUST include documentation updates. This is not optional.**

→ See [.claude/DOCUMENTATION-MAINTENANCE.md](.claude/DOCUMENTATION-MAINTENANCE.md) for complete guidelines

### When Code Changes, Docs Must Change Too

| Code Change | Documentation Updates Required |
|------------|-------------------------------|
| Add public function | Add rustdoc with # Examples section |
| Modify function behavior | Update rustdoc and examples |
| Add public type | Document struct/enum fields, add examples |
| Change module responsibility | Update module-level `//!` documentation |
| Add new command | Update README.md, create docs/ guide, update CHANGELOG.md, update ARCHITECTURE.md |
| Modify config structure | Update docs/configuration.md, update Config rustdoc |
| Change error handling | Update function's `# Errors` rustdoc section |
| Refactor architecture | Update ARCHITECTURE.md, docs/project-structure.md |

### Documentation Maintenance Checklist

Before committing any code change, verify:

**Rustdoc (if public APIs changed):**
- [ ] All public functions have complete rustdoc
- [ ] All public types have rustdoc on struct/enum and fields
- [ ] All # Errors sections explain when function fails
- [ ] All # Example sections include working code samples
- [ ] Run `cargo doc --no-deps` and check for warnings

**Markdown Documentation (if user-facing or architectural change):**
- [ ] README.md updated (if major feature)
- [ ] CHANGELOG.md updated in "Unreleased" section
- [ ] Relevant docs/ guide updated
- [ ] ARCHITECTURE.md updated (if structure changes)
- [ ] docs/project-structure.md updated (if files added/removed)

**Quality Verification:**
- [ ] All examples in documentation are accurate
- [ ] No warnings from `cargo doc --no-deps`
- [ ] Code and docs committed together (never separate)
- [ ] Full command passes: `cargo fmt && cargo clippy && cargo test && cargo doc --no-deps`

### What "Documentation Updated" Means

❌ **NOT ENOUGH:**
- "I'll document it in the next sprint"
- Rustdoc comments without examples
- README.md not mentioning new feature
- Architecture docs out of sync with code

✅ **CORRECT:**
- Rustdoc complete with # Example section
- README.md mentions new feature (if user-facing)
- Relevant guides updated
- Examples are tested and work
- `cargo doc` passes with no warnings

### For AI Agents / LLMs

If you're working on dev-cli:

1. **Never commit code without updating docs**
2. **Always run `cargo doc --no-deps` before finishing**
3. **Check DOCUMENTATION-MAINTENANCE.md for specific requirements**
4. **Examples in rustdoc must actually work**
5. **Keep user guides synchronized with code behavior**

If you can't document a change, you haven't fully understood it. Don't commit it.

---

## Testing

- **Every new function** should have at least one test
- **Every public API** should have integration tests
- **Error cases** should be tested
- **All tests must pass** before PR is merged

### Run Tests Locally

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run only integration tests
cargo test --test '*'

# Run only doc tests
cargo test --doc
```

### Test Organization

```
tests/
├── cli_config.rs      # Tests for config command
├── cli_ide.rs         # Tests for ide command
└── cli_open.rs        # Tests for open command

src/
└── [file].rs
    #[cfg(test)]
    mod tests { }       # Unit tests for the file
```

---

## Documentation

### README

Update `README.md` if you:
- Add a new command
- Change installation instructions
- Modify architecture

### Rustdoc Comments

All public APIs **must** have rustdoc:

```bash
# Check for missing docs
cargo doc --no-deps 2>&1 | grep warning
```

### Guide Updates

Update relevant guide in `docs/` if you:
- Add features affecting usage
- Modify configuration format
- Change architecture

**Guides to update:**
- `docs/getting-started.md` — if user-facing
- `docs/project-structure.md` — if file structure changes
- `docs/style-guide.md` — if coding standards change
- `ARCHITECTURE.md` — if architecture changes

---

## Commit Messages

Use **conventional commits** format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type | When to Use |
|------|------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting (cargo fmt, etc.) |
| `refactor` | Code reorganization (no logic change) |
| `test` | Adding/updating tests |
| `chore` | Build, dependencies, tooling |

### Scope

Optional scope for affected area:

```
feat(ide): add Rider IDE detection
fix(config): handle missing config directory
docs(readme): update installation instructions
```

### Subject

- Lowercase
- Imperative mood ("add" not "added")
- No period at end
- Max 50 characters

### Body (Optional)

More details about the change:

```
The Windows Terminal detection was using a hardcoded path.
Now it checks both the registry and common installation paths,
making it more reliable across different Windows configurations.
```

### Examples

```
feat(commands): add `dev search` command
fix(ide): detect VS Code on Linux
docs: update CLI design documentation
chore(deps): bump clap from 4.4 to 4.5
```

---

## Pull Request Process

### Before Creating a PR

1. **Update your branch** with main
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Run all checks**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   cargo doc --no-deps
   ```

3. **Review your own changes**
   - Does the code make sense?
   - Are error cases handled?
   - Is there adequate documentation?

### Creating a PR

1. **Push your branch**
   ```bash
   git push origin feat/my-feature
   ```

2. **Open PR on GitHub**
   - Use a clear title: "feat: add project search"
   - Reference any issues: "Closes #123"
   - Describe what you changed and why
   - Link to relevant documentation

3. **PR Checklist:**
   - [ ] All tests pass locally
   - [ ] Code is formatted (`cargo fmt`)
   - [ ] Clippy warnings fixed (`cargo clippy`)
   - [ ] Public APIs documented
   - [ ] Tests added for new functionality
   - [ ] CHANGELOG.md updated (if applicable)
   - [ ] README.md updated (if applicable)

### Code Review

Maintainers will review your code for:
- ✅ Does it work?
- ✅ Is it tested?
- ✅ Is it documented?
- ✅ Does it follow standards?
- ✅ Does it fit the architecture?

### Addressing Feedback

1. Don't delete your PR — push new commits addressing feedback
2. Re-request review when ready
3. Use "Resolve conversation" when addressing comments

### Merging

Once approved:
- [ ] Maintainer merges your PR
- [ ] Your branch is deleted
- [ ] Your feature is in `main`!

---

## Adding a New Command

Step-by-step guide to add a new command called `dev sync`.

### Step 1: Add CLI Definition

**File:** `src/cli.rs`

Add the command variant to `Commands` enum:

```rust
#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),
    Config(ConfigCommand),
    Ide(IdeCommand),
    Sync(SyncCommand),  // ← NEW
    Install,
    Open(OpenArgs),
}

// Define the sync command args
#[derive(Args)]
pub struct SyncCommand {
    #[arg(help = "Sync all projects")]
    #[arg(long)]
    pub all: bool,
}
```

### Step 2: Create Command Handler

**File:** `src/commands/sync.rs`

```rust
//! Handle `dev sync` command.
//!
//! Synchronizes project repositories with remote sources.

use anyhow::Result;
use crate::cli::SyncCommand;

/// Execute the sync command.
pub fn execute(cmd: SyncCommand) -> Result<()> {
    if cmd.all {
        sync_all()?;
    } else {
        // default sync behavior
    }
    Ok(())
}

fn sync_all() -> Result<()> {
    println!("Syncing all projects...");
    Ok(())
}
```

### Step 3: Export Command Handler

**File:** `src/commands/mod.rs`

Add the module:

```rust
pub mod config;
pub mod ide;
pub mod install;
pub mod project;
pub mod sync;  // ← NEW
```

### Step 4: Update Main Dispatcher

**File:** `src/main.rs`

Add handling in the match statement:

```rust
match cli.command {
    Commands::Project(cmd) => commands::project::execute(cmd)?,
    Commands::Config(cmd) => commands::config::execute(cmd)?,
    Commands::Ide(cmd) => commands::ide::execute(cmd)?,
    Commands::Sync(cmd) => commands::sync::execute(cmd)?,  // ← NEW
    Commands::Install => commands::install::execute()?,,
    Commands::Open(args) => commands::project::open_shortcut(args)?,
}
```

### Step 5: Add Tests

**File:** `tests/cli_sync.rs`

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn sync_command_works() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.arg("sync")
        .arg("--all")
        .assert()
        .success();
}
```

### Step 6: Update Documentation

1. **README.md** — Add to command reference
2. **CHANGELOG.md** — Add to Unreleased section
3. **docs/project-structure.md** — Document new command

### Step 7: Run All Checks

```bash
cargo fmt
cargo clippy
cargo test
cargo doc --no-deps
```

### Step 8: Commit and PR

```bash
git add .
git commit -m "feat(commands): add sync command"
git push origin feat/sync-command
```

---

## Troubleshooting

### "Rust not found"

```bash
# Install or update Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "Failed to build" errors

```bash
# Clean build artifacts and rebuild
cargo clean
cargo build
```

### "Test fails locally but not CI"

```bash
# Run tests the same way CI does
cargo test --verbose
cargo test --all
```

### "Clippy is too strict"

**Don't disable clippy.** Usually the warning points to real issues. If you disagree:

1. Add a `#[allow(...)]` attribute with a comment explaining why
2. Discuss in the PR if you think the warning is wrong

```rust
// Only OK with justification
#[allow(clippy::unwrap_used)]
let value = risky_operation().unwrap();  // Safe: value is validated above
```

### "Format keeps changing"

Rustfmt is the source of truth:

```bash
cargo fmt
```

Never manually adjust formatting. If a file looks wrong, run `cargo fmt`.

### "Can't find my tests"

```bash
# List all tests
cargo test -- --list

# Run specific test file
cargo test --test cli_sync
```

---

## Questions?

- 📖 See [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- 📖 See [docs/](docs/) for comprehensive guides
- 💬 Open a GitHub discussion
- 🐛 Report bugs with detailed steps

Thank you for contributing! 🚀
