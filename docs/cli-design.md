# CLI Design

`dev-cli` uses [Clap](https://docs.rs/clap/) with derive macros. The whole command surface is declarative structs and enums, and `Cli::parse()` turns `argv` into a typed value.

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
    Open(OpenArgs),
}
```

Each variant names a subcommand group (carrying its own args struct). `Open` is a flat shortcut for `project open <NAME>`.

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

    #[arg(short, long, value_enum)]
    pub ide: Option<Ide>,
}
```

`Ide` is `ValueEnum` so `--ide cursor` parses to `Ide::Cursor` directly.

## Startup Flow

```
argv  →  Cli::parse()  →  Cli { command: Commands::… }
                            ↓
                onboarding::ensure_onboarded()
                            ↓
            match Commands { … }  →  commands::<x>::execute(...)
```

The dispatcher in `main.rs` is a single `match` on `Commands`. Each arm calls the corresponding `commands::<group>::execute(args)` and propagates the `Result`. The onboarding call runs **before** the match so the wizard can populate `config.toml` on a first run.

```rust
fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).without_time().init();
    let cli = Cli::parse();
    onboarding::ensure_onboarded()?;
    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,
        Commands::Config(cmd)  => commands::config::execute(cmd)?,
        Commands::Ide(cmd)     => commands::ide::execute(cmd)?,
        Commands::Open(args)   => commands::project::open_shortcut(args)?,
    }
    Ok(())
}
```

Adding a new top-level command means: new variant on `Commands`, a new args struct in `cli.rs`, a new `execute` function in `commands/`, a new match arm in `main.rs`, and a re-export in `src/lib.rs` so integration tests can reach it. See [CONTRIBUTING.md](../CONTRIBUTING.md#adding-a-new-command) for the full walkthrough.

## Help and Errors

Clap generates `--help` and `--version` from the struct attributes; we don't write any of that. The same is true for error messages — invalid values, missing arguments, and unknown subcommands all produce a colored, structured error followed by a usage hint. Nothing for the commands layer to do.

`dev --help` is structured to print a one-line "usage" header, the list of subcommands, then the global options — in that order — so the most useful information is at the top.

## Why Derive

We considered the builder API but the derive form is more readable for a small, fixed set of commands. Each command becomes a few lines of struct/enum definition that's easy to grep and review. Compile-time validation is a real bonus: if a field is missing a `ValueEnum` derive that the dispatcher needs, the build fails before the user sees it.

## Adding a New IDE

Add a variant to `Ide` in `src/models/ide.rs`, a detection rule in `src/ide/detect.rs`, and (if the IDE needs a non-default command-line shape) a launch arm in `src/ide/launcher.rs`. Clap picks up the new variant automatically — `dev open <name> --ide <new-ide>` parses the moment the code compiles.

## Adding a New Top-Level Command

1. Add a variant to `Commands` and the args struct in `src/cli.rs`.
2. Create `src/commands/<group>.rs` and export it from `src/commands/mod.rs`.
3. Add a `match` arm in `src/main.rs`.
4. Re-export the new module from `src/lib.rs` so integration tests can `use dev_cli::…`.
5. Add a `tests/<command>.rs` integration test.
6. Update `README.md` and `CHANGELOG.md`.

## See Also

- [Clap documentation](https://docs.rs/clap/latest/clap/)
- [src/cli.rs](../src/cli.rs) — the actual parser
- [CONTRIBUTING.md](../CONTRIBUTING.md) — how to add a new command
- [ARCHITECTURE.md](../ARCHITECTURE.md) — overall layering
