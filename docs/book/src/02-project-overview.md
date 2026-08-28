# What We'll Build

## Project Overview

The **dev-cli** is a command-line tool designed to solve a real developer problem: managing multiple Git repositories and opening them quickly in your favorite IDE.

## The Problem It Solves

As a developer, you might have:

- 20+ Git repositories
- Different directories (some in ~/Projects, some in ~/Work, some elsewhere)
- Multiple IDEs (VS Code for web, Cursor for AI-assisted coding, Terminal for quick edits)
- Wasted time finding and opening projects

## The Solution

```bash
# Configure your project directories once
dev config init

# List all your projects (auto-discovered!)
dev project list

# Open any project instantly
dev open MyProject

# Open in specific IDE
dev open MyProject --ide cursor
```

## Architecture Overview

The tool has four layers:

```
┌─────────────────────────────────────┐
│         CLI Layer (clap)            │
│    Parse command line arguments     │
└────────────────┬────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│     Commands Layer (commands/)      │
│    Dispatch to appropriate handler  │
└────────────────┬────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Services Layer (config, ide)   │
│    Business logic and file I/O      │
└────────────────┬────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│       Models Layer (models/)        │
│  Data structures and types          │
└─────────────────────────────────────┘
```

## Key Components

### CLI Parser (Clap)

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

enum Commands {
    Config(ConfigCommand),
    Ide(IdeCommand),
    Project(ProjectCommand),
    Install,
}
```

**Learns:** Derive macros, enums, command parsing

### Configuration System

```rust
pub struct Config {
    pub default_ide: Ide,
    pub projects_root: Vec<PathBuf>,
}
```

**Learns:** Struct serialization, file I/O, TOML format

### IDE Detection

Three-stage process:
1. Check PATH for IDE executables
2. Check common Windows installation directories
3. Deduplicate results

**Learns:** Process spawning, file system access, algorithms

### Project Launching

```rust
pub fn launch(ide: Ide, path: &Path) -> Result<()> {
    Command::new("cursor")
        .arg(path)
        .spawn()?
        .wait()?;
    Ok(())
}
```

**Learns:** Error handling, external processes, Result type

## What Makes This a Good Learning Project

### ✅ It's Real

This is production code. Real constraints, real error handling, real testing.

### ✅ It's Self-Contained

~600 lines of Rust. Understandable without external context. Can see the whole system.

### ✅ It Teaches Patterns

Layered architecture, error handling, testing, CLI patterns—all things professionals use.

### ✅ It's Practical

You can use this tool yourself. Run it, modify it, extend it.

## Commands We'll Implement

| Command | Purpose | Complexity |
|---------|---------|-----------|
| `dev config show` | Display current config | ⭐ Basic |
| `dev config init` | Initialize config file | ⭐ Basic |
| `dev ide list` | List detected IDEs | ⭐⭐ Medium |
| `dev project list` | List configured projects | ⭐⭐ Medium |
| `dev open <NAME>` | Open project | ⭐⭐⭐ Hard |
| `dev install` | Global installation | ⭐⭐ Medium |

## Technology Stack

| Component | Technology | Why |
|-----------|----------|-----|
| CLI Parsing | `clap` with derive | Industry standard, minimal boilerplate |
| Config Format | TOML | Human-readable, standard in Rust |
| Serialization | `serde` | Most flexible Rust serialization |
| Error Handling | `anyhow` | Best practices for applications |
| Platform Paths | `directories` | Cross-platform path handling |

## Learning Curve

```
Easy ├─────────────────────────────── Hard
     │
     ├─ CLI parsing (clap)
     ├─ Basic structs
     ├─ File I/O
     │
     ├─ Module organization  ← You are here
     ├─ Error handling
     ├─ Trait implementation
     │
     ├─ Testing
     ├─ Architecture patterns
     └─ Performance optimization
```

## By the End

You'll be able to:

- ✅ Understand a real Rust project from top to bottom
- ✅ Write your own CLI tool
- ✅ Read and understand professional Rust code
- ✅ Apply patterns to other projects
- ✅ Extend dev-cli with new features
- ✅ Contribute to open-source Rust projects

Ready? Let's start [Setting Up](03-setup.md).
