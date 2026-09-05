# AGENTS.md

Architecture rules and agent configuration for `dev-cli`.

This document specifies the invariants, module responsibilities, and rules that all code changes must respect.

---

## Core Invariants

These rules **must** never be violated:

1. **No upward dependencies** — Low-level modules cannot import from high-level modules
2. **Result everywhere** — All fallible operations use `Result<T>` (via `anyhow`)
3. **Public APIs documented** — Every public type/function has rustdoc comments
4. **No unwrap in production** — Use `.context()` and `?` for error handling
5. **Tests for new features** — All features must have unit and/or integration tests
6. **Modules small** — No module should exceed 500 lines without splitting
7. **Clear responsibilities** — Each module has one reason to change
8. **🔴 Documentation updates with code** — Every code change MUST include documentation updates (see [.claude/DOCUMENTATION-MAINTENANCE.md](.claude/DOCUMENTATION-MAINTENANCE.md))

---

## Directory Responsibilities

### `src/main.rs`

**Responsibility:** Application entry point and command dispatcher

**Allowed actions:**
- ✅ Parse CLI arguments via `Cli::parse()`
- ✅ Dispatch to appropriate command handler
- ✅ Handle top-level errors
- ✅ Initialize logging

**Forbidden:**
- ❌ Business logic
- ❌ Direct config/IDE operations
- ❌ File I/O
- ❌ Complex error messages

**Depends on:**
- `cli` — command definitions
- `commands/*` — command implementations

**Typical size:** < 50 lines

---

### `src/cli.rs`

**Responsibility:** Define CLI arguments and command structure (Clap)

**Allowed actions:**
- ✅ Define `struct` and `enum` for arguments
- ✅ Add validation constraints (e.g., `required`)
- ✅ Use Clap derive macros
- ✅ Provide help text and documentation
- ✅ Reference models (Ide enum, etc.)

**Forbidden:**
- ❌ Any business logic
- ❌ Filesystem operations
- ❌ External process spawning
- ❌ Configuration loading

**Depends on:**
- `models::ide` — IDE enum for ValueEnum derive
- `clap` — external crate

**Typical size:** < 200 lines

---

### `src/commands/*.rs`

**Responsibility:** Implement command handlers and orchestrate services

**Allowed actions:**
- ✅ Call service layer (config, ide, installer, etc.)
- ✅ Format output for terminal display
- ✅ Aggregate multiple services
- ✅ User-facing error messages
- ✅ Add colors and formatting

**Forbidden:**
- ❌ Direct filesystem operations (use services)
- ❌ Process spawning (use ide::launcher)
- ❌ Config parsing (use config::Config)
- ❌ Complex business logic

**Depends on:**
- `cli` — argument definitions
- Service layer (config, ide, etc.)
- `models` — data structures

**Typical size:** < 300 lines

**Structure:**
```
commands/
├── mod.rs       # Module exports
├── project.rs   # dev project list, dev open
├── config.rs    # dev config ...
└── ide.rs       # dev ide list
```

**Rule:** Each command module has public `execute()` function:
```rust
pub fn execute(cmd: SomeCommand) -> Result<()>
```

---

### `src/config.rs`

**Responsibility:** Configuration management and persistence

**Allowed actions:**
- ✅ Load config from disk
- ✅ Save config to disk
- ✅ Provide default configuration
- ✅ Determine config path (platform-aware)
- ✅ Validate configuration

**Forbidden:**
- ❌ Terminal output (except errors)
- ❌ IDE operations
- ❌ Project discovery
- ❌ Complex mutations after load

**Depends on:**
- `models` — Project, Ide types
- `serde`, `toml` — serialization
- `directories` — platform paths
- `anyhow` — error handling

**Typical size:** < 150 lines

**Key type:**
```rust
pub struct Config {
    pub projects_root: Vec<PathBuf>,
    pub default_ide: Ide,
}
```

**Important:** Config is loaded fresh each invocation (no caching across commands)

---

### `src/ide/`

**Responsibility:** IDE detection and project launching

**Submodules:**

#### `src/ide/mod.rs`
- Coordinates IDE subsystem
- Re-exports public types

#### `src/ide/detect.rs`
- **Responsibility:** Discover installed IDEs
- **Algorithm:** Multi-stage (PATH → common Windows locations)
- **Returns:** `Vec<InstalledIde>`
- **Key function:** `pub fn detect_ides() -> Vec<InstalledIde>`

#### `src/ide/launcher.rs`
- **Responsibility:** Spawn external IDE process
- **Key function:** `pub fn launch(ide: Ide, path: &Path) -> Result<()>`
- **Behavior:** Uses `std::process::Command` to spawn

#### `src/ide/registry.rs`
- **Responsibility:** Define detected IDE type
- **Key type:** `pub struct InstalledIde`
- **Fields:** IDE type, display name, executable path

**Forbidden across ide/:**
- ❌ Config operations
- ❌ Project discovery
- ❌ File modification

**Typical total size:** < 300 lines

---

### `src/models/`

**Responsibility:** Define data structures

**Allowed actions:**
- ✅ Define structs, enums, type aliases
- ✅ Implement derived traits (Copy, Clone, Debug, etc.)
- ✅ Serialize/Deserialize derives
- ✅ ValueEnum derives for CLI
- ✅ From/Into implementations for simple conversions

**Forbidden:**
- ❌ Business logic
- ❌ Filesystem operations
- ❌ External dependencies (except serde)
- ❌ Complex algorithms

**Depends on:**
- Only external serialization/enum crates (serde, clap)
- No other internal modules

**Structure:**
```
models/
├── mod.rs         # Module exports
├── ide.rs         # Ide enum
└── project.rs     # Project struct
```

**Typical size:** < 100 lines per file

---

### `src/scanner.rs`

**Responsibility:** Discover Git repositories under configured project roots

**Implemented (as of Sprint 1.7):**
- Recursively walks project roots looking for `.git` directories
- Uses the `ignore` crate's `WalkBuilder` (respects `.gitignore`)
- Skips `IGNORED_DIRS` (`.git`, `target`, `node_modules`, etc.)
- Deduplicates repositories by canonical path
- Returns projects sorted alphabetically by name

**Not implemented (future Sprint 3+):**
- Git metadata extraction (branch, status, commits)

**Depends on:**
- `ignore` crate for traversal
- `models::Project`

---

## Dependency Direction

```
main.rs
  ↓
cli.rs ← commands/* ← {config.rs, ide/*, installer.rs, scanner.rs}
  ↓                     ↓
models/*            ← (all services use models)
```

**Rule:** Arrows point downward only. Never import upward.

### Explicit Forbidden Imports

```rust
// ❌ NOT ALLOWED
// models imports from commands or services
use crate::commands::project;

// ❌ NOT ALLOWED  
// services import from commands
use crate::commands::config;

// ❌ NOT ALLOWED
// main.rs imports command implementations
use crate::commands::project::execute;
```

### Allowed Imports

```rust
// ✅ ALLOWED
// commands import from services
use crate::config::Config;

// ✅ ALLOWED
// services import from models
use crate::models::ide::Ide;

// ✅ ALLOWED
// anything imports from models
use crate::models::Project;
```

---

## File Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Module file | `snake_case.rs` | `project.rs`, `ide_detection.rs` |
| Module folder | `snake_case/` | `commands/`, `ide/` |
| Test file in folder | `tests/cli_<name>.rs` | `tests/cli_config.rs` |
| Binary name | `kebab-case` | `dev-cli` |
| Crate name | `kebab-case` | `dev-cli` |

---

## Public API Conventions

### Functions

```rust
/// Single-responsibility functions
/// Returns Result<T> for fallible operations
pub fn do_something(input: &str) -> Result<String>

// No public functions without documentation
```

### Structs

```rust
/// Public structs are documented
pub struct Config {
    /// Field documentation
    pub field: String,
}

// All public fields should be documented
// Private structs can skip if internal only
```

### Enums

```rust
/// All enum variants are documented
pub enum Ide {
    /// VS Code
    Vscode,
    /// Cursor
    Cursor,
}

// Implements ValueEnum for CLI parsing
// Implements Serialize/Deserialize for config
// Usually derives Copy, Clone, Debug
```

### Modules

```rust
//! Module-level documentation with //!
//!
//! # Responsibilities
//! - ...
//!
//! # Important Types
//! - ...

// Every module starts with //! docs
```

---

## Error Handling Rules

### Pattern: Result Propagation

```rust
pub fn do_operation() -> Result<T> {
    let config = Config::load()
        .context("Failed to load user configuration")?;
    
    let value = compute_something(&config)?;
    
    Ok(value)
}
```

**Never do:**
```rust
// ❌ Unwrap is forbidden
let config = Config::load().unwrap();
```

### Pattern: Context Addition

```rust
// ✅ Add context to lower-level errors
fs::read_to_string(&path)
    .context(format!("Could not read config from {}", path.display()))?

// ❌ Just propagate without context
fs::read_to_string(&path)?
```

### Pattern: User Error Messages

```rust
// ✅ User-friendly error in commands layer
if !project_path.exists() {
    bail!("Project '{}' not found in configured roots", project_name);
}

// ✅ Technical errors in services layer (user doesn't see)
file.metadata()
    .context("Could not get file metadata")?
```

---

## Documentation Conventions

### Rustdoc Format

Every public item must have docs with these sections (if applicable):

```rust
/// Brief one-line description.
///
/// Longer explanation of the type/function and why it exists.
///
/// # Arguments
/// * `arg1` - description of argument 1
/// * `arg2` - description of argument 2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When/why this returns an error
///
/// # Example
/// ```
/// let result = my_function("value")?;
/// assert_eq!(result, expected);
/// ```
pub fn my_function(arg1: &str, arg2: u32) -> Result<String> {
    // ...
}
```

### Module Documentation

Every module starts with `//!`:

```rust
//! Handle project discovery and listing.
//!
//! # Responsibilities
//! - Scan project roots for repositories
//! - List configured project directories
//! - Validate project paths
//!
//! # Important Types
//! - `Project` — represents a Git repository
//!
//! # Example
//! ```
//! let projects = discover_projects(&config)?;
//! ```
```

---

## Testing Conventions

### Unit Test Location

```rust
// In the same file as the code being tested
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // test implementation
    }
}
```

### Integration Test Location

```
tests/
├── cli_config.rs
├── cli_ide.rs
└── cli_open.rs
```

### Test Naming

```rust
#[test]
fn test_<function>_<scenario> { }

// Examples:
#[test]
fn test_config_load_default() { }

#[test]
fn test_ide_detect_in_path() { }

#[test]
fn test_project_open_not_found() { }
```

### Testing Rule

- ✅ Every public function must have a unit test
- ✅ Every command must have an integration test
- ✅ Error cases must be tested
- ✅ Use `#[test]` attribute (not custom test runners)

---

## Adding Features

### Checklist for New Command

1. **CLI Definition** (`src/cli.rs`)
   - [ ] Add `Commands::NewCommand` variant
   - [ ] Create `NewCommandArgs` struct
   - [ ] Document with rustdoc
   - [ ] Validate that it matches the command name

2. **Command Handler** (`src/commands/newcmd.rs`)
   - [ ] Create file with `pub fn execute(cmd: NewCommandArgs) -> Result<()>`
   - [ ] Call service layer (don't implement logic)
   - [ ] Format output appropriately
   - [ ] Add module documentation

3. **Module Export** (`src/commands/mod.rs`)
   - [ ] Add `pub mod newcmd;`

4. **Main Dispatcher** (`src/main.rs`)
   - [ ] Add match arm for new command
   - [ ] Use `?` for error propagation

5. **Tests** (`tests/cli_newcmd.rs`)
   - [ ] Test success path
   - [ ] Test error cases
   - [ ] Use `assert_cmd` for subprocess testing

6. **Documentation**
   - [ ] Add to README.md
   - [ ] Update CHANGELOG.md
   - [ ] Update relevant docs/ files

7. **Quality Checks**
   - [ ] `cargo fmt`
   - [ ] `cargo clippy`
   - [ ] `cargo test`
   - [ ] `cargo doc --no-deps`

---

## Code Review Checklist

Reviewers must verify:

- ✅ Follows dependency direction (no upward imports)
- ✅ All public APIs have rustdoc
- ✅ No `unwrap()` in production code (with justification)
- ✅ Result type used for fallible operations
- ✅ Tests included for new functionality
- ✅ Code formatted with `cargo fmt`
- ✅ Clippy passes with no warnings
- ✅ Modules are reasonably sized
- ✅ Clear responsibility boundaries

---

## IDE Support

### Supported IDEs

```rust
pub enum Ide {
    Vscode,      // ✅ Implemented
    Cursor,      // ✅ Implemented
    Claude,      // ✅ Implemented
    Terminal,    // ✅ Implemented (Windows Terminal)
    Idea,        // 🔄 Planned
    Rider,       // 🔄 Planned
    Zed,         // 🔄 Planned
}
```

### Adding IDE Support

1. Add variant to `Ide` enum in `src/models/ide.rs`
2. Implement detection in `src/ide/detect.rs`
3. Add to launcher's command mapping in `src/ide/launcher.rs`
4. Test detection on target OS
5. Update documentation

---

## Performance Expectations

| Operation | Target | Acceptable | Unacceptable |
|-----------|--------|-----------|--------------|
| CLI startup | < 50ms | < 100ms | > 200ms |
| IDE detection | < 100ms | < 150ms | > 300ms |
| Config load | < 5ms | < 10ms | > 50ms |
| Project open | User limited | User limited | > 1s before IDE starts |

**Measurement:** Use actual binary timing, not debug builds.

```bash
# Time the CLI
time dev ide list
```

---

## Version Compatibility

- **Rust Edition:** 2024
- **MSRV (Minimum Supported Rust Version):** 1.88
- **Platforms:** Windows (primary), macOS, Linux

**Rule:** Never use nightly features.

---

## Future Scalability

### If user has 1000 projects

- Scanning must remain < 1 second
- Config must remain small (< 1 MB)
- Detection must not degrade

### If adding new major feature

- Can add new module (don't expand existing ones beyond 500 lines)
- Maintain dependency direction
- All tests must still pass
- Documentation must be updated

---

## Questions About These Rules?

If a change seems to violate these rules, check:

1. Is there a comment explaining the exception?
2. Has it been discussed in the PR?
3. Is it necessary for functionality?

If unclear, ask for clarification before merging.

---

## References

- [ARCHITECTURE.md](ARCHITECTURE.md) — System design
- [CLAUDE.md](CLAUDE.md) — AI assistant guide
- [CONTRIBUTING.md](CONTRIBUTING.md) — Development workflow
- [docs/](docs/) — User and developer guides
