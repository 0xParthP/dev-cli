# IDE System

`dev-cli` discovers installed IDEs at runtime and spawns the right one when you ask it to open a project. There are three pieces: detection (find what's installed), the registry (the data type that represents a found IDE), and the launcher (spawn the process).

## Detection

Detection runs in two stages and merges the results:

1. **PATH lookup** — for each supported CLI shim (`code`, `cursor`, `claude`, `wt`), ask the `which` crate where it lives. PATH is checked first because it's the fastest, most common case, and any user with a CLI install will be found here.
2. **Common install paths** — for IDEs that often ship without a CLI on PATH, check the platform's standard locations. On Windows that means `%LocalAppData%\Programs\Microsoft VS Code\bin\code.cmd`, `%LocalAppData%\Programs\Cursor\Cursor.exe`, `~/.local/bin/claude.exe`, and similar.

The two stages run sequentially and we deduplicate as we go — if `which code` already found VS Code in PATH, the second stage skips it. The output is a `Vec<InstalledIde>` sorted in a stable order.

Detection runs on every `dev` invocation. It is fast (single-digit milliseconds for PATH; filesystem `exists()` checks for the rest), and re-running it means a newly installed IDE is picked up immediately with no cache invalidation. We don't cache.

```rust
fn detect_ides() -> Vec<InstalledIde> {
    let mut list = Vec::new();

    // Stage 1: PATH
    detect_cli(&mut list, Ide::Vscode, "VS Code", "code");
    detect_cli(&mut list, Ide::Cursor,  "Cursor",  "cursor");
    detect_cli(&mut list, Ide::Claude,  "Claude Code", "claude");
    detect_cli(&mut list, Ide::Terminal, "Windows Terminal", "wt");

    // Stage 2: standard install paths (Windows-leaning today)
    detect_common_install_paths(&mut list);

    list
}
```

## Registry

```rust
pub struct InstalledIde {
    pub ide: Ide,
    pub name: String,
    pub path: PathBuf,
}
```

`InstalledIde` is what detection produces and what the launcher consumes. `name` is the display string ("VS Code", "Cursor", "Claude Code") used by `dev ide list`; `path` is the absolute path to the executable that the launcher will spawn.

## Launching

`launcher::launch(ide, &project_path)` maps the `Ide` enum to a command name, then runs it as a subprocess with the project path as the argument.

```rust
pub fn launch(ide: Ide, path: &Path) -> Result<()> {
    let cmd = match ide {
        Ide::Vscode   => "code",
        Ide::Cursor   => "cursor",
        Ide::Claude   => "claude",
        Ide::Terminal => "wt",
        _ => bail!("IDE {:?} is not yet wired up", ide),
    };

    Command::new(cmd)
        .arg(path)
        .spawn()?;

    Ok(())
}
```

We `spawn` and return — we don't `wait`. The IDE is a long-lived process; waiting for it would mean `dev open` doesn't return until the user closes their editor. If the IDE crashes immediately, the error surfaces on the next read of the child process's stderr; for a quick check, the user can run the same command in a terminal and see the IDE's own error.

## Adding a New IDE

1. Add a variant to `Ide` in `src/models/ide.rs` (this is what makes `--ide <name>` parse).
2. Add a `detect_cli` call in `detect.rs` for the CLI shim, or a path check in `detect_common_install_paths` for a GUI-only install.
3. Add a `match` arm in `launcher.rs` mapping the variant to the command name.
4. Add a unit test for the enum parsing and an integration test for `dev ide list`.

That's the whole contract. Clap picks up the new variant for `--ide` automatically, the detection stages see it on the next run, and the launcher has the spawn command.

## Why No Cached Paths

Caching the executable path in `config.toml` sounds reasonable, but it creates a class of bugs we don't want:

- The user uninstalls the IDE. The path is now stale. The error surfaces far from the cause.
- The IDE auto-updates to a new location. Same problem.
- The user copies their config to a new machine. Paths are machine-specific.

Re-running detection on every invocation costs single-digit milliseconds and avoids the whole class of problem. Adding a cache is straightforward if measurement ever shows it matters.

## Platform Coverage

The PATH stage works everywhere. The "common install paths" stage is currently Windows-leaning — that's the primary target — but the structure is the same on macOS and Linux: check `/Applications/*.app`, `/opt`, `~/.local/bin`, `/usr/local/bin`, and so on. Adding those stages is a matter of writing the right `Path::join` per platform.

## See Also

- [src/ide/](../src/ide/) — `detect.rs`, `launcher.rs`, `registry.rs`
- [docs/configuration.md](configuration.md) — where the default IDE is set
- [ARCHITECTURE.md](../ARCHITECTURE.md) — where the IDE system sits in the layered design
