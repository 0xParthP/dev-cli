# dev-cli

<div align="center">

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-Active%20Development-blue)
[![CI](https://github.com/0xParthP/dev-cli/actions/workflows/ci.yml/badge.svg)](...)
[![Coverage](https://github.com/0xParthP/dev-cli/actions/workflows/coverage.yml/badge.svg)](...)
[![Security](https://github.com/0xParthP/dev-cli/actions/workflows/security.yml/badge.svg)](...)
[![Release](https://github.com/0xParthP/dev-cli/actions/workflows/release.yml/badge.svg)](...)

**A modern developer project manager written in Rust**

Fast. Simple. Extensible.

[Getting Started](#quick-start) • [Documentation](docs/getting-started.md) • [Architecture](ARCHITECTURE.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## Overview

`dev-cli` is a command-line tool that helps you manage, discover, and launch your Git repositories with speed and simplicity. Instead of manually navigating to project directories, `dev-cli` maintains a configuration of your project roots and automatically opens them in your favorite IDE.

**Key features:**
- ⚡ **Fast project discovery** — automatically find Git repositories in configured directories
- 🎯 **IDE detection** — detects VS Code, Cursor, Claude Code, Windows Terminal, and more
- 🚀 **One-command launch** — open any project in any IDE instantly
- ⚙️ **Configuration management** — TOML-based configuration for projects and defaults
- 🔄 **Cross-platform ready** — written in Rust with Windows, macOS, and Linux support

---

## Feature Comparison

| Feature | Status | Notes |
|---------|--------|-------|
| Project discovery | ✅ Active | Manually configured project roots |
| IDE detection | ✅ Active | Supports 7+ IDEs |
| Quick project launch | ✅ Active | `dev open <project>` |
| Configuration management | ✅ Active | TOML-based config |
| Global installation | ✅ Active | `dev install` |
| Automatic repository scanning | ✅ Active | Discovers Git repos under roots |
| Git integration | 🔄 Planned | Status, branch info |
| Project templates | 🔄 Planned | Quick scaffolding |
| Dashboard mode | 🔄 Planned | Interactive TUI |

---

## Screenshots

```
$ dev project list
Configured Project Roots
📁 C:\Users\parth\Projects
📁 C:\Users\parth\Work

$ dev open MyProject --ide cursor
Opened C:\Users\parth\Projects\MyProject

$ dev ide list
Installed IDEs:
✓ VS Code (C:\Program Files\Microsoft VS Code\bin\code.cmd)
✓ Cursor (C:\Program Files\Cursor\Cursor.exe)
✓ Claude Code (C:\Users\parth\.local\bin\claude.exe)
```

---

## Quick Start

### Installation

#### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/dev-cli.git
cd dev-cli

# Build and install
cargo build --release
cargo install --path .
```

#### Windows Installer
```bash
# If dev is already installed globally:
dev install
```

### First Steps

1. **Initialize configuration:**
   ```bash
   dev config init
   ```

2. **View your configuration:**
   ```bash
   dev config show
   ```

3. **List installed IDEs:**
   ```bash
   dev ide list
   ```

4. **Open a project:**
   ```bash
   dev open my-project
   dev open my-project --ide cursor
   ```

---

## Command Reference

### `dev project`

Manage and launch projects.

| Command | Description |
|---------|-------------|
| `dev project list` | List configured project root directories |
| `dev project open <NAME>` | Open a project in the default IDE |
| `dev project open <NAME> --ide <IDE>` | Open a project in a specific IDE |
| `dev open <NAME>` | Shorthand for `dev project open` |

**Examples:**
```bash
dev project list
dev open MyProject
dev open MyProject --ide vscode
```

### `dev config`

Manage development configuration.

| Command | Description |
|---------|-------------|
| `dev config show` | Display current configuration |
| `dev config init` | Initialize default configuration |
| `dev config set-default-ide <IDE>` | Set default IDE for launching projects |

**Examples:**
```bash
dev config show
dev config init
dev config set-default-ide cursor
```

### `dev ide`

Discover installed development environments.

| Command | Description |
|---------|-------------|
| `dev ide list` | List all detected installed IDEs |

**Examples:**
```bash
dev ide list
```

### `dev install`

Install `dev` globally to your system PATH.

```bash
dev install
```

After installation, ensure the installation directory is in your PATH. The installer will prompt you with the directory path.

---

## Building from Source

### Requirements
- **Rust 1.88+** (install from [rustup.rs](https://rustup.rs))
- **Git**
- **Windows, macOS, or Linux**

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/dev-cli.git
cd dev-cli

# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

The compiled binary will be in `target/release/dev` (or `target/debug/dev`).

### Useful Cargo Commands

```bash
# Run the CLI directly
cargo run -- <COMMAND>

# Run a specific test
cargo test test_name

# Generate and view documentation
cargo doc --no-deps --open

# Check code without building
cargo check

# View project dependencies
cargo tree
```

---

## Development Workflow

For detailed development instructions, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Development

1. **Fork and clone** the repository
2. **Create a feature branch** — `git checkout -b feature/my-feature`
3. **Make changes** and **add tests**
4. **Run tests and format** — `cargo test && cargo fmt && cargo clippy`
5. **Commit with a clear message** — follow conventional commits
6. **Push and create a PR**

### Code Quality

- **Format:** `cargo fmt` (enforced via CI)
- **Lint:** `cargo clippy` (enforced via CI)
- **Tests:** All tests must pass (enforced via CI)
- **Docs:** Public APIs must have rustdoc comments

---

## Project Structure

```
dev-cli/
├── src/
│   ├── main.rs           # Application entry point
│   ├── cli.rs            # CLI argument parsing (Clap)
│   ├── config.rs         # Configuration management
│   ├── installer.rs      # Installation logic
│   ├── scanner.rs        # Repository discovery
│   ├── commands/         # Command implementations
│   ├── ide/              # IDE detection and launching
│   └── models/           # Data models
├── tests/                # Integration tests
├── docs/                 # User and contributor documentation
├── Cargo.toml            # Project manifest
├── README.md             # This file
├── ARCHITECTURE.md       # Architecture and design
├── CONTRIBUTING.md       # Contributor guide
├── CHANGELOG.md          # Version history
└── .claude/              # Agent instructions (CLAUDE.md, AGENTS.md)
```

For detailed project structure documentation, see [docs/project-structure.md](docs/project-structure.md).

---

## Documentation

Complete documentation is available in the `docs/` directory:

- **[Getting Started](docs/getting-started.md)** — Setup and first steps
- **[Project Structure](docs/project-structure.md)** — Complete file reference
- **[Architecture](ARCHITECTURE.md)** — System design and module organization
- **[Rust for dev-cli](docs/rust-for-dev-cli.md)** — Learn Rust through this project
- **[CLI Design](docs/cli-design.md)** — How the CLI parser works
- **[Configuration](docs/configuration.md)** — Configuration file format and schema
- **[IDE System](docs/ide-system.md)** — IDE detection algorithm
- **[Testing](docs/testing.md)** — Testing philosophy and practices
- **[Style Guide](docs/style-guide.md)** — Coding standards
- **[Roadmap](docs/roadmap.md)** — Future direction

### Rust Documentation

Generate and view the auto-generated Rust documentation:

```bash
cargo doc --no-deps --open
```

This opens the rustdoc site showing all public APIs with examples.

---

## Architecture Overview

`dev-cli` follows a layered architecture:

```
┌─────────────────────────────────────┐
│     CLI Layer (clap Parser)         │
│  Handles argument parsing & help    │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│    Command Layer (commands/)        │
│  Implements: project, config, ide   │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│  Service Layer (config, ide, etc)   │
│  Handles business logic              │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│    Model Layer (models/)            │
│  Data structures (Ide, Project)     │
└─────────────────────────────────────┘
```

For detailed architecture documentation, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development setup
- Code standards
- Commit message format
- PR workflow
- Adding new commands

---

## Roadmap

| Sprint | Focus | Status |
|--------|-------|--------|
| 1-1.6 | Core CLI, IDE detection, configuration | ✅ Complete |
| **1.7** | **Complete documentation** | 🚀 **Active** |
| 2 | Automatic repository scanning | ✅ Complete |
| 3 | Git integration | 🔄 Planned |
| 4 | Interactive TUI dashboard | 🔄 Planned |

See [docs/roadmap.md](docs/roadmap.md) for detailed sprint plans.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

---

## Support

- **Documentation:** See the [docs/](docs/) directory
- **Architecture:** Read [ARCHITECTURE.md](ARCHITECTURE.md)
- **Issues:** Report bugs on GitHub
- **Discussions:** Open a GitHub discussion for questions

---

**Built with ❤️ in Rust**