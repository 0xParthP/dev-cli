# IDE System

`dev-cli` discovers installed IDEs at runtime and spawns the right one when you ask it to open a project. There are three pieces: detection (find what's installed), the registry (the data type that represents a found IDE), and the launcher (spawn the process).

## Detection

Detection runs in two stages and merges the results, deduplicating by executable path:

1. **PATH lookup** — for each supported CLI shim (`code`, `cursor`, `claude`, `wt`), ask the `which` crate where it lives. PATH is checked first because it's the fastest, most common case, and any user with a CLI install will be found here.
2. **Common install paths** — for IDEs that often ship without a CLI on PATH, check the platform's standard locations. On Windows that means `%LocalAppData%\Programs\Microsoft VS Code\bin\code.cmd`, `%LocalAppData%\Programs\Cursor\Cursor.exe`, `~/.local/bin/claude.exe`, and similar.

The two stages run sequentially; if `which code` already found VS Code in PATH, the second stage skips it. The output is a `Vec<InstalledIde>` sorted in a stable order.

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

For tests, the path is resolved through `which::which_in(...)` first and then a small set of standard locations. Tests stub the executable lookup by setting `DEVCLI_TEST_EXECUTABLE` to an absolute path, so the launcher tests can be hermetic.

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

`launcher::launch(ide, &project_path)` looks the IDE up in the detected list, then spawns it as a subprocess. The spawn shape depends on the IDE:

- **VS Code** / **Cursor** — `code <path>` / `cursor <path>`
- **Claude Code** — `claude` in the project's working directory
- **Windows Terminal** — `wt -d <path>`

```rust
pub fn launch(ide: Ide, path: &Path) -> Result<()> {
    let installed = detect_ides()
        .into_iter()
        .find(|i| i.ide == ide)
        .ok_or_else(|| anyhow!("{:?} is not installed", ide))?;
    launch_spawn(ide, path, &installed.path)
}
```

`launch_spawn` is the small per-IDE switch that maps an `Ide` to its CLI shape. Variants that aren't wired up (`idea`, `rider`, `zed`) currently return an "unsupported IDE" error — they parse via `ValueEnum` so the CLI accepts them, but launching will need a new arm.

We `spawn` and return — we don't `wait`. The IDE is a long-lived process; waiting for it would mean `dev open` doesn't return until the user closes their editor. If the IDE crashes immediately, the error surfaces on the next read of the child process's stderr; for a quick check, the user can run the same command in a terminal and see the IDE's own error.

## Adding a New IDE

1. Add a variant to `Ide` in `src/models/ide.rs` (this is what makes `--ide <name>` parse).
2. Add a `detect_cli` call in `detect.rs` for the CLI shim, or a path check in `detect_common_install_paths` for a GUI-only install.
3. Add a `match` arm in `launcher.rs` mapping the variant to its spawn shape (and, if the IDE needs a non-default CLI invocation, extend `launch_spawn`).
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
