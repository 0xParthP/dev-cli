# Testing

The complete testing philosophy and practices for `dev-cli`.

---

## Testing Pyramid

```
       ▲
      ╱ ╲
     ╱   ╲  E2E / Manual Tests
    ╱─────╲ (rare, high value)
   ╱       ╲
  ╱         ╲ Integration Tests
 ╱───────────╲ (cli_*.rs, medium effort)
╱             ╲
╱───────────────╲ Unit Tests
                (fast, cheap, many)
```

**Philosophy:**
- ✅ Many unit tests (fast, cheap)
- ✅ Some integration tests (verify behavior)
- ✅ Few E2E tests (manual verification)

---

## Unit Tests

### What They Test

Small, focused functions in isolation.

### Location

Same file as the code being tested, in `#[cfg(test)]` module:

```rust
// src/config.rs
pub fn load() -> Result<Config> { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_default() {
        // Test when file doesn't exist
    }
}
```

### Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_ide, Ide::Vscode);
        assert!(!config.projects_root.is_empty());
    }

    #[test]
    fn test_ide_parsing() {
        assert_eq!("vscode".parse::<Ide>(), Ok(Ide::Vscode));
        assert_eq!("cursor".parse::<Ide>(), Ok(Ide::Cursor));
    }
}
```

### Running Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_default_config

# Run with output
cargo test -- --nocapture

# Run only tests in a module
cargo test --lib config::tests
```

### Guidelines

- **Test one thing per test**
- **Use descriptive names** — `test_<function>_<scenario>`
- **Arrange, Act, Assert pattern:**
  ```rust
  #[test]
  fn test_config_default() {
      // Arrange
      let expected = Ide::Vscode;
      
      // Act
      let config = Config::default();
      
      // Assert
      assert_eq!(config.default_ide, expected);
  }
  ```

---

## Integration Tests

### What They Test

Full command execution through the CLI, verifying end-to-end behavior.

### Location

Separate files in `tests/` directory:

```
tests/
├── cli_config.rs
├── cli_ide.rs
└── cli_open.rs
```

### Framework

Use `assert_cmd` to spawn CLI as subprocess:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_show_runs() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}
```

### Example: tests/cli_config.rs

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_show_contains_default_ide() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}

#[test]
fn config_show_contains_projects_root() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("projects_root"));
}
```

### Example: tests/cli_ide.rs

```rust
use assert_cmd::Command;

#[test]
fn ide_list_runs() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("ide")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn ide_list_shows_installed() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("ide")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Installed").or(predicate::str::contains("✓")));
}
```

### Example: tests/cli_open.rs

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn open_nonexistent_project_fails() {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    
    cmd.arg("open")
        .arg("nonexistent-project-xyz")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}
```

### Running Integration Tests

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test '*'

# Run specific test
cargo test --test cli_config

# Show output
cargo test --test cli_config -- --nocapture
```

### Predicates

Common assertions with `predicates` crate:

```rust
.stdout(predicate::str::contains("text"))      // Contains substring
.stdout(predicate::str::contains_utf8("text")) // UTF-8 match
.stderr(predicate::str::is_empty())            // Empty stderr
.status(predicate::status::success())          // Exit code 0
.status(predicate::status::failure())          // Exit code != 0
```

### Temporary Directories

For tests that need files:

```rust
use tempfile::TempDir;
use std::fs;

#[test]
fn test_with_temp_dir() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("config.toml");
    
    // Write test config
    fs::write(&config_path, "test content").unwrap();
    
    // Test using temp_config_path
    
    // Automatically cleaned up when temp is dropped
}
```

---

## Test Coverage

### Current Coverage

| Module | Unit Tests | Integration Tests |
|--------|------------|-------------------|
| cli.rs | None | Implicit (via other tests) |
| config.rs | None (partial) | In cli_config.rs |
| commands/ | None | In cli_*.rs |
| ide/ | None | In cli_ide.rs |

### Target Coverage

- **Public APIs:** 80%+ coverage
- **Error paths:** All error cases tested
- **Critical paths:** 100% coverage

### Measuring Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

---

## Running All Checks

Before committing, run the full check suite:

```bash
# Format
cargo fmt

# Lint
cargo clippy

# Test
cargo test

# Docs
cargo doc --no-deps

# All at once
cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

---

## CI/CD Testing

### GitHub Actions (Planned)

The `.github/workflows/` directory will contain:

- **Rust Check** — Format, clippy, build
- **Test** — Run all tests
- **Documentation** — Verify docs build

### Local Testing

To simulate CI locally:

```bash
# Check formatting
cargo fmt -- --check

# Lint with no warnings
cargo clippy -- -D warnings

# Test all
cargo test --all

# Build all
cargo build --release
```

---

## Best Practices

### Do

✅ Test behavior, not implementation
✅ Use descriptive test names
✅ Test error cases
✅ Keep tests simple and focused
✅ Mock external dependencies (file I/O, network)
✅ Use temporary directories for file tests

### Don't

❌ Test private implementation details
❌ Use random data (use fixed test data)
❌ Make tests depend on execution order
❌ Write overly complex test assertions
❌ Test third-party libraries
❌ Leave debugging code in tests

---

## Future Test Infrastructure

### cargo-nextest

Faster test runner:

```bash
cargo install cargo-nextest
cargo nextest run
```

Benefits:
- Faster test execution (parallel by default)
- Better output
- Test listing

### Property-Based Testing

Use `proptest` for generative testing:

```rust
#[cfg(test)]
mod tests {
    use proptest::proptest;

    proptest! {
        #[test]
        fn test_config_roundtrip(
            ide in "cursor|vscode|claude",
            root in ".*",
        ) {
            // Property-based test
        }
    }
}
```

### Fuzzing

Use `cargo fuzz` to find edge cases in TOML parsing.

---

## Debugging Tests

### Run with Logging

```bash
# Enable tracing output in tests
RUST_LOG=debug cargo test -- --nocapture
```

### Attach Debugger

With rust-analyzer or lldb:

```bash
# Use VS Code debug configuration
# Or attach lldb/gdb to test process
```

### Print Debug Info

```rust
#[test]
fn test_something() {
    let value = compute();
    
    // Print for debugging (shows with --nocapture)
    eprintln!("Debug: {:?}", value);
    
    assert_eq!(value, expected);
}
```

---

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Testing requirements
- [docs/project-structure.md](project-structure.md) — Test files
- [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/) — CLI testing
- [predicates](https://docs.rs/predicates/latest/predicates/) — Test assertions
- [tempfile](https://docs.rs/tempfile/latest/tempfile/) — Temporary files
