# Setting Up Rust

Before we start building, let's make sure your Rust environment is properly configured.

## Installing Rust

### macOS and Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts to complete installation.

### Windows

Download from https://www.rust-lang.org/tools/install and run the installer.

Alternatively, using Windows Package Manager:

```powershell
winget install Rustlang.Rust.MSVC
```

### Verification

```bash
rustc --version
cargo --version
```

Both should show version numbers (no errors means success!).

## Understanding the Rust Toolchain

### rustc

The Rust compiler. Translates `.rs` files to executable binaries.

```bash
rustc main.rs
./main
```

### cargo

The Rust package manager and build tool. Think `npm` for Node, `pip` for Python.

```bash
cargo build
cargo run
cargo test
cargo doc
```

### rustup

The toolchain manager. Keeps Rust up to date.

```bash
rustup update
```

## Your First Project

Let's create a project called `dev-cli`:

```bash
cargo new dev-cli
cd dev-cli
```

This creates:

```
dev-cli/
├── Cargo.toml    # Project configuration
├── Cargo.lock    # Dependency versions (auto-generated)
└── src/
    └── main.rs   # Your Rust code
```

### Cargo.toml

Project configuration file:

```toml
[package]
name = "dev-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
# External packages go here
```

The `edition` specifies which Rust language version you're using:

- `2015` — Original Rust
- `2018` — Major improvements (recommended for learning)
- `2021` — Current (latest features)

## Running Your Project

### Development Mode

```bash
cargo run
# or shorter:
cargo r
```

Compiles and runs. Optimized for fast compile time, not runtime speed.

### Release Mode

```bash
cargo run --release
# or:
cargo r --release
```

Takes longer to compile but runs faster. For production.

### Just Build (Don't Run)

```bash
cargo build
cargo build --release
```

Produces binary in `target/debug/` or `target/release/`.

## Project Structure

As we develop dev-cli, we'll create:

```
dev-cli/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── clippy.toml        # Lint configuration
├── rustfmt.toml       # Formatter configuration
├── docs/              # Documentation
├── src/
│   ├── main.rs        # Entry point
│   ├── cli.rs         # CLI argument parsing
│   ├── config.rs      # Configuration management
│   ├── commands/      # Command implementations
│   ├── ide/           # IDE detection and launching
│   └── models/        # Data structures
├── tests/             # Integration tests
└── target/            # Build artifacts (git-ignored)
```

## Essential Commands

| Command | Purpose |
|---------|---------|
| `cargo new <name>` | Create new project |
| `cargo build` | Build in debug mode |
| `cargo build --release` | Build optimized |
| `cargo run` | Build and run |
| `cargo test` | Run tests |
| `cargo doc --open` | Generate and open docs |
| `cargo fmt` | Format code |
| `cargo clippy` | Run linter |
| `cargo check` | Check compilation without building |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies |

## Editing Rust Code

### Recommended Setup

**VS Code** + **Rust Analyzer** extension:

1. Install VS Code: https://code.visualstudio.com/
2. Open Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Rust Analyzer" and install

This gives you:
- Code completion
- Error checking
- Format on save
- "Go to definition"
- Inline hints

### Alternative IDEs

- **IntelliJ IDEA** — Full-featured IDE with Rust plugin
- **Vim/Neovim** — For terminal enthusiasts
- **Sublime Text** — Lightweight, fast

## Testing the Setup

Let's write a quick test:

```rust
// src/main.rs
fn main() {
    println!("Hello, Rust!");
}
```

Run it:

```bash
cargo run
# Output: Hello, Rust!
```

Change the message, run again. Notice how quickly it compiles—that's development mode!

## Staying Updated

Rust releases every 6 weeks. Keep updated:

```bash
rustup update
```

Always safe to run. Your projects continue using their specified edition.

## Next Steps

Now that your environment is ready, let's [Create the Project Structure](04-structure.md).
