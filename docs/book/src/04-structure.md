# Project Structure

How we organize code in dev-cli to keep it maintainable.

## The Layered Architecture

dev-cli follows a strict **four-layer architecture**:

```
Layer 1: CLI
  ↓ (uses)
Layer 2: Commands
  ↓ (uses)
Layer 3: Services
  ↓ (uses)
Layer 4: Models
```

**Key Rule:** No layer can depend on a layer above it. Data flows down, control flows up.

## File Organization

```
src/
├── main.rs              # Entry point, layer 1
├── cli.rs               # CLI parsing, layer 1
├── commands/            # Command handlers, layer 2
│   ├── mod.rs
│   ├── config.rs
│   ├── ide.rs
│   ├── project.rs
│   └── install.rs
├── config.rs            # Services, layer 3
├── installer.rs
├── scanner.rs
├── ide/                 # Services, layer 3
│   ├── mod.rs
│   ├── detect.rs
│   ├── launcher.rs
│   └── registry.rs
└── models/              # Data structures, layer 4
    ├── mod.rs
    ├── ide.rs
    └── project.rs
```

## Layer Responsibilities

### Layer 1: CLI (cli.rs, main.rs)

**Responsibility:** Parse command-line arguments

**Contains:**
- `Cli` struct — Top-level arguments
- `Commands` enum — Available commands
- Various `Args` structs for each command

**Example:**

```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub enum Commands {
    Config(ConfigCommand),
    Ide(IdeCommand),
    Project(ProjectCommand),
    Install,
}
```

**Can use:** Clap crate only

**Cannot use:** Commands, services, or models

### Layer 2: Commands (commands/*.rs)

**Responsibility:** Implement command logic

**Contains:**
- One module per command
- `execute()` function entry point
- Business logic orchestration

**Example:**

```rust
// commands/ide.rs
pub fn execute() -> Result<()> {
    let ides = ide::detect::detect_ides()?;
    for ide in ides {
        println!("✓ {}", ide.name);
    }
    Ok(())
}
```

**Can use:** Services, models, Clap, anyhow

**Cannot use:** CLI layer (no parse)

### Layer 3: Services (config.rs, ide/*.rs, etc.)

**Responsibility:** Core business logic and I/O

**Contains:**
- File I/O (reading config files, directories)
- Algorithm implementation (IDE detection)
- External process spawning
- Data transformation

**Examples:**

```rust
// config.rs
pub fn load() -> Result<Config> { }
pub fn save(&self) -> Result<()> { }

// ide/detect.rs
pub fn detect_ides() -> Result<Vec<InstalledIde>> { }

// ide/launcher.rs
pub fn launch(ide: Ide, path: &Path) -> Result<()> { }
```

**Can use:** Models, std lib, crates (directories, anyhow, etc.)

**Cannot use:** CLI or commands layers

### Layer 4: Models (models/*.rs)

**Responsibility:** Data structures only

**Contains:**
- `pub struct` definitions
- `impl` blocks with constructors and helpers
- Derives: Debug, Clone, Serialize, Deserialize

**Examples:**

```rust
// models/ide.rs
#[derive(Debug, Clone, ValueEnum)]
pub enum Ide {
    Vscode,
    Cursor,
    Claude,
    #[value(name = "wt")]
    Terminal,
}

// models/project.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}
```

**Can use:** Only std lib and derives

**Cannot use:** Any other layer

## Module Files (mod.rs)

Each directory with public API needs `mod.rs`:

```rust
// src/models/mod.rs
mod ide;
mod project;

pub use ide::Ide;
pub use project::Project;
```

This declares what's public to parent modules.

## Dependencies Flow

```
main.rs
  ↓
cli.rs
  ↓
commands/
  ├─ config.rs ────→ config.rs (service)
  ├─ ide.rs ───────→ ide/ (service)
  └─ project.rs ───→ ide/ + config.rs

config.rs (service)
  ↓
models/

ide/ (service)
  ├─ detect.rs
  ├─ launcher.rs
  └─ registry.rs
      ↓
    models/
```

## Adding a New Command

Follow this pattern:

1. **Add to Layer 1 (CLI):** Define arguments in `cli.rs`
2. **Add to Layer 2 (Commands):** Create `commands/new_command.rs`
3. **Dispatcher:** Add case to `main.rs`
4. **Services:** Create `service_name.rs` if needed
5. **Models:** Reuse existing or add new

Example: `dev sync` command

```rust
// 1. cli.rs
#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "sync")]
    Sync(SyncArgs),
    // ...
}

#[derive(Args)]
pub struct SyncArgs {
    #[arg(help = "GitHub username")]
    pub username: String,
}

// 2. commands/sync.rs
pub fn execute(args: SyncArgs) -> Result<()> {
    let repos = fetch_github_repos(&args.username)?;
    println!("Synced {} repositories", repos.len());
    Ok(())
}

// 3. main.rs
match cli.command {
    Commands::Sync(args) => commands::sync::execute(args)?,
    // ...
}

// 4. Create sync_service.rs if needed
// 5. Use existing models or create sync::Repository
```

## Why This Structure?

### ✅ Testable

Each layer can be tested independently. Mock layers below.

### ✅ Maintainable

Clear boundaries. Know where to make changes.

### ✅ Reusable

Services can be called from different commands.

### ✅ Scalable

Easy to add new commands without restructuring.

### ✅ Safe

Compiler enforces layer boundaries (no upward dependencies).

## Growth Pattern

**v0.1 (current):**
- ~600 lines total
- 4 layers
- Single concern per file

**v0.2 (planned):**
- ~1000 lines
- Services might split into subdirectories
- Keep 4-layer model

**v1.0 (future):**
- Could exceed 5000 lines
- Consider adding Layer 5 (plugins/extensions)
- Still maintain separation

## Common Mistakes

### ❌ Wrong Layer

```rust
// DON'T: Commands using Clap
commands::ide::execute() {
    let args: IdeArgs = ...parse...  // Wrong! Parse happens in CLI layer
}
```

### ❌ Upward Dependencies

```rust
// DON'T: Services using Commands
config::save() {
    commands::sync::refresh()?;  // Wrong! Services can't use commands
}
```

### ❌ Logic in CLI Layer

```rust
// DON'T: Complex logic in CLI
fn main() -> Result<()> {
    let config = Config::load()?;
    // 20 lines of algorithm
    // Use services instead!
}
```

### ✅ Correct Pattern

```rust
// Layer 1: Parse
match cli.command {
    Commands::Ide(args) => execute_ide(args),
}

// Layer 2: Dispatch
fn execute_ide(args: IdeArgs) -> Result<()> {
    commands::ide::execute()?
}

// Layer 3: Implement
pub fn execute() -> Result<()> {
    let ides = ide::detect_ides()?
    // Logic here
}
```

## Next Steps

Understanding structure is crucial. Next, let's look at your [First Build](05-first-build.md).
