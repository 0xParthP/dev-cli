# CLI Design

`dev-cli` uses [Clap](https://docs.rs/clap/) with derive macros. The whole command surface is declarative structs and enums, and `Cli::parse()` turns the argv into a typed value.

## Shape of the Parser

```rust
#[derive(Parser)]
#[command(name = "dev", version, about = "Modern Git Project Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),
    Config(ConfigCommand),
    Ide(IdeCommand),
    Install,
    Open(OpenArgs),
}
```

Each variant either names a subcommand group (carrying its own args struct) or — like `Install` — is a unit variant for a flag-style command with no arguments.

For subcommand groups we use the same pattern recursively:

```rust
#[derive(Args)]
pub struct ProjectCommand {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    List,
    Open(OpenArgs),
}

#[derive(Args)]
pub struct OpenArgs {
    pub project: String,

    #[arg(short, long)]
    pub ide: Option<Ide>,
}
```

`Ide` is `ValueEnum` so `--ide cursor` parses to `Ide::Cursor` directly.

## Flow

```
argv  →  Cli::parse()  →  Cli { command: Commands::… }  →  match  →  commands::<x>::execute(...)
```

The dispatcher in `main.rs` is a single `match` on `Commands`. Each arm calls the corresponding `commands::<group>::execute(args)` and propagates the `Result`.

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,
        Commands::Config(cmd)  => commands::config::execute(cmd)?,
        Commands::Ide(cmd)     => commands::ide::execute(cmd)?,
        Commands::Install      => commands::install::execute()?,
        Commands::Open(args)   => commands::project::open_shortcut(args)?,
    }
    Ok(())
}
```

Adding a new command means: new variant on `Commands`, a new args struct in `cli.rs`, a new `execute` function in `commands/`, a new match arm. See [CONTRIBUTING.md](../CONTRIBUTING.md#adding-a-new-command) for the full walkthrough.

## Help and Errors

Clap generates `--help` and `--version` from the struct attributes; we don't write any of that. The same is true for error messages — invalid values, missing arguments, and unknown subcommands all produce a colored, structured error followed by a usage hint. Nothing for the commands layer to do.

## Why Derive

We considered the builder API but the derive form is more readable for a small, fixed set of commands. Each command becomes a few lines of struct/enum definition that's easy to grep and review. Compile-time validation is a real bonus: if a field is missing a `ValueEnum` derive that the dispatcher needs, the build fails before the user sees it.

## Adding a New IDE

Add a variant to `Ide` in `src/models/ide.rs` and a detection rule in `src/ide/detect.rs`. Clap picks up the new variant automatically — `dev open <name> --ide <new-ide>` works the moment the code compiles. No changes to the dispatcher or to `cli.rs`.

## See Also

- [Clap documentation](https://docs.rs/clap/latest/clap/)
- [src/cli.rs](../src/cli.rs) — the actual parser
- [CONTRIBUTING.md](../CONTRIBUTING.md) — how to add a new command
