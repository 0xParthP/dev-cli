# Learning Rust Through dev-cli

This guide teaches Rust concepts by exploring how they're used in the `dev-cli` project.

Each section starts with a concept, then shows real examples from this codebase.

---

## Table of Contents

1. [Modules and Organization](#modules-and-organization)
2. [Structs and Data](#structs-and-data)
3. [Enums and Pattern Matching](#enums-and-pattern-matching)
4. [Traits and Derives](#traits-and-derives)
5. [Error Handling](#error-handling)
6. [Ownership and Borrowing](#ownership-and-borrowing)
7. [The `?` Operator](#the--operator)
8. [Lifetimes](#lifetimes)

---

## Modules and Organization

### Concept

Rust uses **modules** to organize code into logical units. Modules can be in the same file or separate files.

```rust
mod my_module {
    pub fn my_function() { }
}
```

Or in a separate file `my_module.rs`:

```rust
pub fn my_function() { }
```

### Example: dev-cli

In `src/main.rs`, we declare our modules:

```rust
mod cli;           // Reads cli.rs
mod commands;      // Reads commands/mod.rs
mod config;        // Reads config.rs
mod ide;           // Reads ide/mod.rs
mod installer;     // Reads installer.rs
mod models;        // Reads models/mod.rs
mod scanner;       // Reads scanner.rs
```

Then we use them:

```rust
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,
        Commands::Config(cmd) => commands::config::execute(cmd)?,
        // ...
    }
    
    Ok(())
}
```

### Key Takeaway

Modules provide **namespacing** and **organization**. They don't affect performance.

---

## Structs and Data

### Concept

A **struct** groups related data together:

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
println!("{}", p.x);
```

### Example: dev-cli

In `src/config.rs`:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub projects_root: Vec<PathBuf>,
    pub default_ide: Ide,
}
```

This struct:
- Stores configuration data
- Uses `#[derive(...)]` to automatically implement traits
- Has `pub` fields so other modules can read them

Creating a Config:

```rust
let config = Config {
    projects_root: vec![PathBuf::from("/home/user/Projects")],
    default_ide: Ide::Vscode,
};
```

### Key Takeaway

Structs are the primary way to organize related data in Rust. Derives save you from writing boilerplate.

---

## Enums and Pattern Matching

### Concept

An **enum** represents a value that can be one of several variants:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

match color {
    Color::Red => println!("Red!"),
    Color::Green => println!("Green!"),
    Color::Blue => println!("Blue!"),
}
```

### Example: dev-cli

In `src/models/ide.rs`:

```rust
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Ide {
    Cursor,
    Vscode,
    Claude,
    Terminal,
    Idea,
    Rider,
    Zed,
}
```

This enum represents different IDEs. In `src/main.rs`, we dispatch commands with pattern matching:

```rust
match cli.command {
    Commands::Project(cmd) => commands::project::execute(cmd)?,
    Commands::Config(cmd) => commands::config::execute(cmd)?,
    Commands::Ide(cmd) => commands::ide::execute(cmd)?,
    Commands::Install => commands::install::execute()?,
    Commands::Open(args) => commands::project::open_shortcut(args)?,
}
```

Each variant can have data:

```rust
#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),
    Config(ConfigCommand),
    Ide(IdeCommand),
    Install,
    Open(OpenArgs),
}
```

### Key Takeaway

Enums + pattern matching are powerful for representing different possibilities and ensuring you handle all cases.

---

## Traits and Derives

### Concept

A **trait** is a contract that types can implement. **Derives** automatically implement common traits:

```rust
#[derive(Debug)]     // Implement Debug
#[derive(Clone)]     // Implement Clone
#[derive(Copy)]      // Implement Copy (only small types)
struct MyType { }
```

### Example: dev-cli

In `src/models/ide.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Ide {
    Cursor,
    Vscode,
    // ...
}
```

What each derive does:

| Derive | Purpose |
|--------|---------|
| `Debug` | Enables printing with `{:?}` |
| `Clone` | Enables `.clone()` to duplicate values |
| `Copy` | Enables automatic copying (small types only) |
| `PartialEq` | Enables `==` comparisons |
| `Eq` | Enables use in sets/maps |
| `Serialize` | Enables saving to TOML with Serde |
| `Deserialize` | Enables loading from TOML with Serde |
| `ValueEnum` | Enables parsing from CLI strings via Clap |

### Custom Trait Implementation

In `src/config.rs`, we manually implement `Default`:

```rust
impl Default for Config {
    fn default() -> Self {
        let home = BaseDirs::new()
            .expect("Couldn't find home directory")
            .home_dir()
            .to_path_buf();

        Self {
            projects_root: vec![home.join("Projects")],
            default_ide: Ide::Vscode,
        }
    }
}
```

Then we can create a default Config:

```rust
let config = Config::default();
```

### Key Takeaway

Derives save boilerplate. Traits enable code reuse and polymorphism. Together they make Rust code concise.

---

## Error Handling

### Concept

Rust uses **Result** for error handling:

```rust
enum Result<T, E> {
    Ok(T),      // Success with value
    Err(E),     // Error with error info
}

fn do_something() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

The `?` operator unwraps `Ok` or returns early with `Err`.

### Example: dev-cli

In `src/config.rs`:

```rust
impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::path()?;  // ← Returns early if path() fails

        if !path.exists() {
            let config = Self::default();
            config.save()?;  // ← Returns early if save() fails
            return Ok(config);
        }

        let text = fs::read_to_string(path)?;  // ← Returns early if read fails
        Ok(toml::from_str(&text)?)  // ← Returns early if parsing fails
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
```

The `anyhow` crate makes errors even nicer:

```rust
use anyhow::{Result, Context};

pub fn load() -> Result<Config> {
    let path = Self::path()
        .context("Could not determine config directory")?;
    
    let text = fs::read_to_string(&path)
        .context("Failed to read config file")?;
    
    toml::from_str(&text)
        .context("Config file is not valid TOML")
}
```

### Key Takeaway

`Result` forces you to handle errors explicitly. `?` and `.context()` make error handling clean and ergonomic.

---

## Ownership and Borrowing

### Concept

Every value in Rust has an owner. When the owner goes out of scope, the value is dropped.

```rust
{
    let s = String::from("hello");  // s owns the string
}  // s is dropped here; string memory is freed

// Can't use s here — it's dropped!
```

**Borrowing** lets you use a value without taking ownership:

```rust
let s = String::from("hello");
let len = calculate_length(&s);  // Borrow s

println!("The length of '{}' is {}", s, len);  // Can still use s!

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s is returned, but it doesn't own the string, so nothing happens
```

### Example: dev-cli

In `src/commands/project.rs`:

```rust
fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;  // config is owned here

    for root in config.projects_root {  // Borrowed reference to projects_root
        let candidate = root.join(&args.project);  // Borrow args.project

        if candidate.exists() {
            let ide = args.ide.unwrap_or(config.default_ide);

            launcher::launch(ide, &candidate)?;  // Borrow candidate

            println!("{} {}", "Opened".green(), candidate.display());

            return Ok(());
        }
    }
    
    bail!("Project '{}' not found.", args.project)
}
```

Key points:
- `config` is owned by the function
- `config.projects_root` is borrowed with `for root in`
- `&args.project` and `&candidate` are borrowed references
- Everything is automatically freed at function end

### Key Takeaway

Ownership prevents memory leaks and data races at compile-time. Borrowing lets you share data temporarily. This is Rust's "killer feature".

---

## The `?` Operator

### Concept

The `?` operator is shorthand for error propagation:

```rust
// Instead of this:
let value = match some_result {
    Ok(v) => v,
    Err(e) => return Err(e),
};

// Write this:
let value = some_result?;
```

### Example: dev-cli

In `src/main.rs`:

```rust
fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();  // If this fails, return Err

    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,  // ← ?
        Commands::Config(cmd) => commands::config::execute(cmd)?,    // ← ?
        Commands::Ide(cmd) => commands::ide::execute(cmd)?,          // ← ?
        Commands::Install => commands::install::execute()?,          // ← ?
        Commands::Open(args) => commands::project::open_shortcut(args)?,  // ← ?
    }

    Ok(())
}
```

Each `?` means "if this is an Err, return it immediately".

### Key Takeaway

The `?` operator makes error handling concise and readable. It works with any `Result` type.

---

## Lifetimes

### Concept

**Lifetimes** ensure borrowed references don't outlive their data:

```rust
fn bad_function() -> &String {
    let s = String::from("hello");
    &s  // ERROR! Trying to return a reference to s
}  // s is dropped here; the reference is now invalid!

fn good_function(s: &String) -> &str {
    &s[0..5]  // OK: returning a reference to the input
}
```

Most of the time, Rust infers lifetimes automatically:

```rust
fn takes_and_returns(s: &String) -> &String {
    s  // Rust knows: return borrow of the input parameter
}
```

Sometimes you need to be explicit:

```rust
fn takes_two(s1: &String, s2: &String) -> &String {
    // Which string does the result borrow from?
    // Rust makes you specify!
    // (This function doesn't work without lifetimes)
}

fn takes_two<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    // Now Rust knows: result borrows from either s1 or s2
}
```

### Example: dev-cli

In `src/ide/launcher.rs`:

```rust
pub fn launch(ide: Ide, path: &Path) -> Result<()> {
    // `path` is borrowed; we don't own it
    // The function can't use `path` after it returns
    // Lifetime is implicit: `&'_ Path`
    
    let cmd = match ide {
        Ide::Vscode => "code",
        Ide::Cursor => "cursor",
        // ...
    };

    Command::new(cmd)
        .arg(path)  // Pass borrowed path to spawned process
        .spawn()?
        .wait()?;

    Ok(())
}
```

Lifetimes are inferred here because:
- We take a borrowed reference `&Path`
- We use it once and don't return it
- Rust knows it lives long enough

### Key Takeaway

Lifetimes can seem complex, but Rust infers them in most cases. When you need them, they prevent use-after-free bugs at compile time.

---

## Practical Patterns

### Pattern 1: Load Config and Use It

```rust
pub fn do_something() -> Result<()> {
    let config = Config::load()?;
    
    println!("{:?}", config);
    
    Ok(())
}
```

**Rust Concepts:**
- `Result<()>` for errors
- `?` for error propagation
- Ownership (config is dropped at function end)

### Pattern 2: Pattern Matching on Enums

```rust
match cli.command {
    Commands::Project(cmd) => commands::project::execute(cmd)?,
    Commands::Config(cmd) => commands::config::execute(cmd)?,
    // ...
}
```

**Rust Concepts:**
- `match` for exhaustive pattern matching
- Enums with associated data
- `?` for error propagation

### Pattern 3: Iterating and Borrowing

```rust
for root in config.projects_root {
    let candidate = root.join(&args.project);
    if candidate.exists() {
        // Do something
    }
}
```

**Rust Concepts:**
- Iterating with `for`
- Automatic borrowing (`for root in` borrows)
- Method calls on borrowed values

### Pattern 4: Derive-Based Deserialization

```rust
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub projects_root: Vec<PathBuf>,
    pub default_ide: Ide,
}

let config: Config = toml::from_str(&text)?;
```

**Rust Concepts:**
- Derive macros
- Trait implementations (automatic with `derive`)
- Type annotations for parsing

---

## Further Learning

- **Rust Book:** https://doc.rust-lang.org/book/
- **Rustlings:** https://github.com/rust-lang/rustlings
- **Project Structure:** See [docs/project-structure.md](project-structure.md)
- **Architecture:** Read [ARCHITECTURE.md](../ARCHITECTURE.md)

---

## Exercises

Try these challenges:

1. **Add a new command:** Follow [CONTRIBUTING.md](../CONTRIBUTING.md#adding-a-new-command)
2. **Add IDE detection:** Extend `src/ide/detect.rs` with Windows Registry support
3. **Write a test:** Add a test case to one of the integration test files in `tests/` (see [testing.md](testing.md))
4. **Refactor:** Split `src/commands/project.rs` into smaller functions

---

**Happy learning! 🚀**
