# Modules and Organization

Understanding how Rust code is organized into modules.

## What are Modules?

A **module** is a container for code. It helps organize related functionality and control visibility.

```rust
// File: src/my_module.rs
pub fn public_function() { }
fn private_function() { }
```

## The Module Hierarchy

In dev-cli, modules form a tree:

```
crate (root)
├── main            — Entry point
├── cli             — Argument parsing
├── commands        — Command handlers
│   ├── config
│   ├── ide
│   ├── project
│   └── install
├── config          — Configuration service
├── ide             — IDE detection
│   ├── detect
│   ├── launcher
│   └── registry
├── installer       — Installation service
├── models          — Data structures
│   ├── ide
│   └── project
└── scanner         — Repository discovery
```

## File Names = Module Names

In Rust, the file name determines the module name:

```
src/config.rs    →  mod config { }
src/cli.rs       →  mod cli { }
src/models/      →  mod models { mod.rs declares contents }
  └── ide.rs     →  pub mod ide { }
```

## Declaring Modules

### In main.rs

```rust
mod cli;
mod commands;
mod config;
mod ide;
mod models;
```

This tells Rust: "These modules exist, find the files for them."

### In mod.rs

Subdirectories need `mod.rs` to declare their contents:

```rust
// src/ide/mod.rs
pub mod detect;
pub mod launcher;
pub mod registry;
```

This says: "The `ide` module contains three submodules."

## Visibility: pub vs private

```rust
// Private: Can only be used within this module
fn private_function() { }

// Public: Can be used from other modules
pub fn public_function() { }

pub struct PublicStruct {
    pub field1: String,      // Public field
    field2: i32,             // Private field
}
```

## Accessing Other Modules

```rust
// In src/commands/config.rs
use crate::config::Config;  // Access from root
use crate::cli::ConfigArgs;

Config::load()?;
```

**Path components:**
- `crate` — Root of our package
- `crate::config` — config.rs module
- `crate::config::Config` — Config struct in config module
- `super` — Parent module

## Using Your Own Types

```rust
// In commands/ide.rs
use crate::models::ide::Ide;  // Import the type
use crate::ide::detect;        // Import the module

let ides: Vec<Ide> = detect::detect_ides();
```

## Module Documentation

Every module file should start with `//!`:

```rust
//! IDE detection and launching.
//!
//! This module finds IDEs and launches them.
```

The `//!` is special: it documents the *module itself*, not the next item.

```rust
//! Module doc

/// This documents the function
pub fn my_function() { }
```

## Example: The IDE Module

Let's trace through the ide module structure:

```
src/ide/              — Directory
├── mod.rs            — "Here's what's in ide"
├── detect.rs         — IDE detection code
├── launcher.rs       — Launching code
└── registry.rs       — Type definitions
```

**src/ide/mod.rs:**

```rust
//! IDE detection and launching system.

pub mod detect;
pub mod launcher;
pub mod registry;
```

**Using from commands:**

```rust
// commands/ide.rs
use crate::ide::detect;

let ides = detect::detect_ides();
```

## Organizing Large Modules

When a module grows large, split it:

**Before:** `src/commands.rs` (500 lines)

**After:**
```
src/commands/
├── mod.rs       — declare submodules
├── config.rs    — config logic
├── ide.rs       — ide logic
└── project.rs   — project logic
```

src/commands/mod.rs:
```rust
pub mod config;
pub mod ide;
pub mod project;
```

## Common Mistakes

### ❌ Wrong Path

```rust
// WRONG: config is in crate root, not in commands
use crate::commands::config::Config;

// RIGHT:
use crate::config::Config;
```

### ❌ Missing mod declaration

```
// If src/new_module.rs exists but main.rs has no:
// mod new_module;
// Then compiler doesn't know it exists!
```

### ❌ Forgot pub

```rust
// Private: Can't be used from other files
fn discover_ides() { }

// Public: Can be used
pub fn discover_ides() { }
```

## Best Practices

✅ **One responsibility per module**  
✅ **File size < 500 lines**  
✅ **Clear module names**  
✅ **Limit nesting (3 levels max)**  
✅ **Document modules with `//!`**  

❌ **Circular dependencies**  
❌ **Too many private items**  
❌ **Vague names like `utils.rs`**  

## Next Steps

Now that you understand modules, let's look at [Data with Structs](07-structs.md).
