# Configuration

`dev-cli` stores user settings in a single TOML file. The file is created on first run with sensible defaults, and every subsequent run reads it.

## Where the File Lives

The path is resolved through the `directories` crate (using `ProjectDirs`), so the right location is picked for the current OS without us hard-coding anything.

| OS | Path |
|----|------|
| Windows | `%AppData%\dev-cli\config\config.toml` |
| macOS | `~/Library/Application Support/dev-cli/config/config.toml` |
| Linux | `~/.config/dev-cli/config.toml` |

> Tests and CI runs override the location by setting `DEVCLI_CONFIG_DIR` in the environment. The loader reads from a `config.toml` inside that directory and creates it if missing.

If the file is missing in a real run, `Config::load()` writes `Config::default()` to that path and returns it. The next run finds a real file.

## File Format

```toml
projects_root = [
    "C:/Users/you/Projects",
    "C:/Users/you/Work",
]
default_ide = "vscode"
```

### `projects_root`

Array of absolute paths. `dev open <name>` searches each root for a directory whose name matches `<name>`, using the scanner (which honours `.gitignore`). The first match wins.

Default: `[~/Projects]`.

### `default_ide`

One of the supported IDE identifiers: `vscode`, `cursor`, `claude`, `terminal`, `idea`, `rider`, `zed`. Used when you don't pass `--ide` to `dev open`.

Default: `vscode`.

## Lifecycle

`Config::load()` is the only entry point. The current behaviour is deliberately forgiving:

1. Resolve the platform-specific path (or honour `DEVCLI_CONFIG_DIR`).
2. If the file is missing, write the default and return it.
3. Otherwise read, parse, and return. A parse error is **not** propagated: a clear message is logged on stderr and the file is rewritten with defaults so the next run starts fresh. The user never ends up stuck behind an unparseable config.
4. The onboarding wizard runs first, before `Config::load`, only when both stdin and stdout are attached to a TTY and no config file exists. It writes the answers it collected.

`Config::save()` serialises with `toml::to_string_pretty` and creates the parent directory if needed.

```rust
pub fn load() -> Result<Self> {
    let path = Self::path()?;
    if !path.exists() {
        let config = Self::default();
        config.save()?;
        return Ok(config);
    }
    let text = fs::read_to_string(&path)?;
    match toml::from_str(&text) {
        Ok(cfg) => Ok(cfg),
        Err(err) => {
            eprintln!("config parse error: {err}; rewriting with defaults");
            let cfg = Self::default();
            cfg.save()?;
            Ok(cfg)
        }
    }
}
```

`Config::exists()` is the helper the startup flow uses to decide whether the onboarding wizard needs to run.

## Editing the Config

Three options:

1. **Edit the file directly.** It's a normal TOML file. Save and run `dev config show` to confirm.
2. **Use the CLI.** `dev config set-default-ide cursor` writes the change for you.
3. **Reset.** Delete the file and run `dev config init` (or just re-run any command — the loader recreates the file).

## Adding New Fields

New config fields should default to a sensible value via `#[serde(default)]`. That way older config files keep working — Serde fills the missing field with `Default::default()` instead of failing to parse.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub projects_root: Vec<PathBuf>,
    pub default_ide: Ide,

    #[serde(default)]
    pub auto_discover: bool,  // new field, defaults to false
}
```

For genuinely incompatible changes, follow the standard migration pattern: try the new format first, fall back to the old format, and rewrite the file in the new format on a successful old-format load.

## Troubleshooting

**"Configuration file is not valid TOML"** — should no longer happen on a normal run; the loader rewrites the file with defaults. If you do see it (e.g. when editing by hand), open the file, fix the syntax, save, and rerun. A stray comma or missing quote is the usual culprit.

**"Couldn't locate config directory"** — extremely rare; means the platform didn't report a home directory. `dev config init` will recreate the default.

**Changes don't apply** — make sure the file you edited is the one `dev` is reading (`dev config show` prints the resolved path on most platforms). Watch out for shell environment overrides — config values can be set via env vars in some test setups (`DEVCLI_CONFIG_DIR` in particular).

## See Also

- [src/config.rs](../src/config.rs) — the implementation
- [src/startup.rs](../src/startup.rs) — the onboarding gate that wraps `Config::load`
- [ARCHITECTURE.md](../ARCHITECTURE.md) — how `Config` fits in the layered design
- [serde](https://serde.rs/) / [toml](https://toml.io/) / [directories](https://docs.rs/directories/) — the three crates doing the work
