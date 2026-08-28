# Style Guide

Code standards and conventions for `dev-cli`.

---

## Formatting

### Code Formatting

**Use `cargo fmt` exclusively:**

```bash
cargo fmt
```

Never manually adjust formatting. `rustfmt` is the source of truth.

### Line Length

- **Prefer:** < 100 characters
- **Maximum:** 120 characters (hard limit)

Rationale: Readable on standard terminals without scrolling.

### Indentation

- **Spaces:** 4 spaces (enforced by `rustfmt`)
- **Tabs:** Never

### Blank Lines

- Separate logical sections with single blank line
- Maximum 2 consecutive blank lines
- No trailing whitespace

---

## Naming Conventions

### Modules and Files

```rust
// File: src/my_module.rs
mod my_module { }

// File: src/submodules/mod.rs
mod submodules {
    mod my_submodule { }
}
```

**Rule:** `snake_case` for files and modules

### Types

```rust
pub struct MyStruct { }
pub enum MyEnum { }
pub trait MyTrait { }
type MyAlias = String;
```

**Rule:** `PascalCase` for types

### Functions

```rust
pub fn my_function() { }
fn helper_function() { }
fn is_valid() { }  // Predicates start with is_/has_
fn get_value() { } // Getters start with get_
fn set_value() { } // Setters start with set_
```

**Rule:** `snake_case` for functions

### Constants

```rust
const MAX_RETRIES: u32 = 3;
const DEFAULT_TIMEOUT: u64 = 5000;
```

**Rule:** `SCREAMING_SNAKE_CASE` for constants

### Variables and Fields

```rust
let project_name = "MyProject";
let mut config = Config::load()?;

pub struct Config {
    pub default_ide: Ide,
    pub projects_root: Vec<PathBuf>,
}
```

**Rule:** `snake_case` for variables and fields

### Booleans

```rust
let is_valid = true;
let has_errors = false;
let should_continue = true;

fn is_empty(&self) -> bool { }
fn contains(&self, item: &T) -> bool { }
```

**Rule:** Boolean names start with `is_`, `has_`, `should_`, `contains_`, etc.

---

## Module Organization

### File Size

| Type | Recommended | Maximum |
|------|-------------|---------|
| Single-function module | < 50 lines | 100 lines |
| Utility module | 100-200 lines | 300 lines |
| Feature module | 200-300 lines | 500 lines |
| Complex module | Split into submodules | N/A |

If a file exceeds 500 lines, split into submodules.

### Module Structure

```rust
//! Module documentation

// Imports
use crate::models::*;
use std::path::Path;

// Public types
pub struct PublicType { }

// Public functions
pub fn public_function() { }

// Private types
struct PrivateType { }

// Private functions
fn private_function() { }

// Tests
#[cfg(test)]
mod tests { }
```

### Import Organization

```rust
// Standard library
use std::fs;
use std::path::PathBuf;

// External crates
use anyhow::{Context, Result};
use clap::Args;

// Internal modules
use crate::config::Config;
use crate::models::Ide;
```

**Order:**
1. Standard library (`std`)
2. External crates (alphabetical)
3. Internal crate modules

**Rules:**
- One blank line between groups
- `use ...::*` only when importing many items from one module
- Prefer specific imports over glob imports

---

## Function Size

### Ideal Length

- **Target:** 20-50 lines
- **Acceptable:** < 150 lines
- **Maximum:** 200 lines (must refactor)

### Refactoring Signals

Consider refactoring if:
- Function does multiple distinct things
- Function has deeply nested blocks
- Function has multiple levels of abstraction
- Can't see entire function on screen

### Example Refactor

**Before (too large):**

```rust
pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => {
            let config = Config::load()?;
            println!("{}", "Configured Project Roots".bold());
            for root in config.projects_root {
                println!("📁 {}", root.display());
            }
            Ok(())
        },
        ProjectSubcommand::Open(args) => {
            let config = Config::load()?;
            for root in config.projects_root {
                let candidate = root.join(&args.project);
                if candidate.exists() {
                    let ide = args.ide.unwrap_or(config.default_ide);
                    launcher::launch(ide, &candidate)?;
                    println!("{} {}", "Opened".green(), candidate.display());
                    return Ok(());
                }
            }
            bail!("Project '{}' not found.", args.project)
        }
    }
}
```

**After (refactored):**

```rust
pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => list(),
        ProjectSubcommand::Open(args) => open(args),
    }
}

fn list() -> Result<()> {
    let config = Config::load()?;
    println!("{}", "Configured Project Roots".bold());
    for root in config.projects_root {
        println!("📁 {}", root.display());
    }
    Ok(())
}

fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;
    for root in config.projects_root {
        let candidate = root.join(&args.project);
        if candidate.exists() {
            return launch_project(&args, &config, &candidate);
        }
    }
    bail!("Project '{}' not found.", args.project)
}

fn launch_project(args: &OpenArgs, config: &Config, path: &Path) -> Result<()> {
    let ide = args.ide.unwrap_or(config.default_ide);
    launcher::launch(ide, path)?;
    println!("{} {}", "Opened".green(), path.display());
    Ok(())
}
```

---

## Comments

### When to Comment

✅ **Comment these:**
- Non-obvious algorithms
- Business logic rationale
- Workarounds and hacks
- Assumptions

❌ **Don't comment these:**
- What the code does (should be self-explanatory)
- Obvious variable assignments
- Type annotations

### Comment Style

```rust
// ✅ Good: Explains WHY
// We detect CLI tools first because most are in PATH.
// Common Windows locations are slower to check.
fn detect_ides() { }

// ❌ Bad: Repeats what code does
// Loop through projects_root
for root in config.projects_root { }

// ✅ Good: Explains non-obvious behavior
// Ide::Vscode might be in PATH or common location, so check
// for existing instance to avoid duplicates
if !list.iter().any(|i| matches!(i.ide, Ide::Vscode)) {
    list.push(ide);
}

// ✅ Good: Explains workaround
// HACK: On Windows, VS Code adds a .cmd wrapper, not the exe
// So we check both locations to maximize detection reliability
let vs_cmd = home.join("AppData/Local/Programs/Microsoft VS Code/bin/code.cmd");
let vs_exe = home.join("AppData/Local/Programs/Microsoft VS Code/Code.exe");
```

### Multi-line Comments

```rust
// Use // for both single and multi-line comments
// This is clearer than /* */ for normal prose

/*
 Use /* */ only for:
 - Temporarily disabling large blocks
 - Legal headers/copyright
*/
```

---

## Documentation Comments

Every public item must have rustdoc:

```rust
/// Brief description (one line summary).
///
/// More detailed explanation if the behavior is complex or
/// not obvious from the signature.
///
/// # Arguments
/// * `name` - Description of what this parameter does
///
/// # Returns
/// Description of return value and its meaning
///
/// # Errors
/// When/why this can return an error
///
/// # Examples
/// ```
/// let result = my_function("value")?;
/// assert_eq!(result, "expected");
/// ```
///
/// # Panics
/// If applicable, when this might panic
///
/// # Safety
/// If `unsafe`, explain why unsafe is necessary
pub fn my_function(name: &str) -> Result<String> {
    // implementation
}
```

### Module Documentation

Every file starts with `//!`:

```rust
//! Module description in one line.
//!
//! # Responsibilities
//! - What this module does
//! - Why it exists
//!
//! # Important Types
//! - `MyType` — what it represents
//!
//! # Example
//! ```
//! let obj = create_something()?;
//! ```
```

---

## Error Handling

### Result Type

Always use `Result<T>` for fallible operations:

```rust
pub fn load_config() -> Result<Config> { }
pub fn launch_ide(ide: Ide, path: &Path) -> Result<()> { }
```

### Context Addition

Always add context to errors:

```rust
// ✅ Good
fs::read_to_string(&path)
    .context(format!("Could not read config from {}", path.display()))?

// ❌ Bad
fs::read_to_string(&path)?

// ✅ Good
Config::load()
    .context("Failed to load user configuration")?

// ❌ Bad
Config::load().unwrap()
```

### Error Messages

For user-facing errors:

```rust
// ✅ Clear and actionable
bail!("Project '{}' not found in configured directories", project_name);

// ✅ Explains what to do
bail!("IDE '{}' is not yet supported. Try: {}", ide, "cursor");

// ❌ Unhelpful
bail!("Error");

// ❌ Too technical for user
bail!("std::io::Error: Permission denied");
```

### No unwrap() in Production

```rust
// ❌ NEVER in production code
let config = Config::load().unwrap();

// ✅ Always use context
let config = Config::load()
    .context("Failed to load configuration")?;

// ✅ OK only for initialization that must succeed
let home = BaseDirs::new()
    .expect("home directory must exist");
```

---

## Imports

### Organizing Imports

```rust
// Group imports by source, separated by blank lines

// Standard library
use std::fs;
use std::path::Path;

// External crates (alphabetical)
use anyhow::{Context, Result, bail};
use clap::ValueEnum;

// Internal modules (alphabetical)
use crate::config::Config;
use crate::models::Ide;
```

### Use Statements

```rust
// ✅ Prefer specific imports
use std::path::PathBuf;
use std::fs;

// ⚠️ Glob imports only when importing many related items
use std::io::{Read, Write, BufRead, BufReader, BufWriter};

// ❌ Avoid unnecessary globs
use std::*;
```

---

## Type Annotations

### When Required

```rust
// ✅ Required: Function parameter types
fn do_something(name: &str, count: u32) { }

// ✅ Required: Function return types (non-trivial)
pub fn load_config() -> Result<Config> { }

// ✅ When type can't be inferred
let value: String = "test".to_string();
let vec: Vec<i32> = Vec::new();

// ❌ Unnecessary: Type can be inferred
let x = 42;  // Obviously i32
let name = String::from("test");  // Obviously String
```

---

## Testing

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_success() {
        // Arrange
        let input = "value";
        let expected = "result";

        // Act
        let result = my_function(input)?;

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_function_error() {
        let invalid = "";
        let result = my_function(invalid);
        assert!(result.is_err());
    }
}
```

### Test Naming

```rust
#[test]
fn test_<function>_<scenario> { }

// Examples:
#[test]
fn test_config_default() { }

#[test]
fn test_config_load_nonexistent_file() { }

#[test]
fn test_ide_detect_finds_vscode() { }
```

---

## Performance Considerations

### Prefer

- ✅ Borrowed references (`&T`) over owned values
- ✅ `for` loops over `map()` for simple iterations
- ✅ Early returns to reduce nesting
- ✅ Zero-copy operations where possible

### Avoid

- ❌ Unnecessary `clone()`
- ❌ Deep nesting (more than 3 levels)
- ❌ Large allocations in loops
- ❌ String concatenation in loops

---

## Logging

### Use the Tracing Crate

```rust
use tracing::info;

fn my_function() {
    info!("Starting operation");
    // ...
    info!("Operation complete");
}
```

### Currently Minimal

Current logging is minimal (just initialization). As features grow, add strategic `debug!` and `info!` logs.

---

## Versioning

This crate uses Semantic Versioning:

- **MAJOR:** Incompatible API changes
- **MINOR:** New functionality (backward compatible)
- **PATCH:** Bug fixes (backward compatible)

Format: `MAJOR.MINOR.PATCH`

Example: `0.2.3`

---

## File Organization

### Typical File Structure

```rust
//! Module documentation

// Imports (std, external, internal)
use std::path::PathBuf;
use anyhow::Result;
use crate::config::Config;

// Public types
pub struct PublicType { }

// Impl blocks for public types
impl PublicType {
    pub fn new() -> Self { }
}

// Public functions
pub fn public_function() -> Result<()> { }

// Private types
struct PrivateType { }

// Private functions
fn private_function() { }

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() { }
}
```

---

## Common Patterns

### Early Return

```rust
// ✅ Good: Early return reduces nesting
pub fn find_project(name: &str) -> Result<PathBuf> {
    if name.is_empty() {
        bail!("Project name cannot be empty");
    }

    let config = Config::load()?;

    for root in config.projects_root {
        let path = root.join(name);
        if path.exists() {
            return Ok(path);
        }
    }

    bail!("Project not found")
}
```

### Option Handling

```rust
// ✅ Use map/unwrap_or for simple cases
let ide = args.ide.unwrap_or(config.default_ide);

// ✅ Use match for complex cases
let result = match maybe_value {
    Some(v) => process(v)?,
    None => default_value(),
};

// ❌ Avoid unnecessary intermediate variables
let opt = Some(value);
let val = opt.unwrap();
```

---

## Documentation

See [CLAUDE.md](../CLAUDE.md) and [AGENTS.md](../AGENTS.md) for additional standards.

---

## Enforcement

### CI Checks

All PRs must pass:
- `cargo fmt -- --check` — No formatting issues
- `cargo clippy -- -D warnings` — No clippy warnings
- `cargo test` — All tests pass
- `cargo doc --no-deps 2>&1 | grep -i warning` — No missing docs

### Local Pre-commit

Before committing, run:

```bash
cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

---

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
- [Clippy Lints](https://doc.rust-lang.org/clippy/)
