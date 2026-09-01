# Configuration

`dev-cli` stores user settings in a single TOML file. The file is created on first run with sensible defaults, and every subsequent run reads it.

## Where the File Lives

The path is resolved through the `directories` crate, so the right location is picked for the current OS without us hard‑coding anything.

| OS | Path |
|----|------|
| Windows | `%LocalAppData%\dev-cli\config\config.toml` |
| macOS | `~/.config/dev-cli/config.toml` |
| Linux | `~/.config/dev-cli/config.toml` |

If the file is missing, `Config::load()` writes `Config::default()` to that path and returns it. The next run finds a real file.

## File Format

```toml
projects_root = [
    "C:/Users/you/Projects",
    "C:/Users/you/Work",
]
default_ide = "vscode"
```

### `projects_root`

Array of absolute paths. `dev open <name>` searches each root for a directory whose name matches `<name>`. The first match wins.

Default: `[~/Projects]`.

### `default_ide`

One of the supported IDE identifiers: `vscode`, `cursor`, `claude`, `terminal`, `idea`, `rider`, `zed`. Used when you don't pass `--ide` to `dev open`.

Default: `vscode`.

## Lifecycle

`Config::load()` is the only entry point:

1. Resolve the platform‑specific path.
2. If the file is missing, write the default and return it.
3. Otherwise read, parse, and return. A parse error is propagated so the user sees the bad config rather than silent defaults.

`Config::save()` serializes with `toml::to_string_pretty` and creates the parent directory if needed.

```rust
pub fn load() -> Result<Self> {
    let path = Self::path()?;

    if !path.exists() {
        let config = Self::default();
        config.save()?;
        return Ok(config);
    }

    let text = fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}
```

## Editing the Config

Three options:

1. **Edit the file directly.** It's a normal TOML file. Save and run `dev config show` to confirm.
2. **Use the CLI.** `dev config set-default-ide cursor` writes the change for you.
3. **Reset.** Delete the file and run `dev config init`.

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

For genuinely incompatible changes, follow the standard migration pattern: try the new format first, fall back to the old format, and rewrite the file in the new format on a successful old‑format load.

## Troubleshooting

**"Configuration file is not valid TOML"** — open the file in a text editor, fix the syntax (a stray comma or missing quote is the usual culprit), save, and rerun.

**"Couldn't locate config directory"** — extremely rare; means the platform didn't report a home directory. `dev config init` will recreate the default.

**Changes don't apply** — make sure the file you edited is the one `dev` is reading (`dev config show` prints the resolved path on most platforms). Watch out for shell environment overrides — config values can be set via env vars in some test setups.

## See Also

- [src/config.rs](../src/config.rs) — the implementation
- [ARCHITECTURE.md](../ARCHITECTURE.md) — how `Config` fits in the layered design
- [serde](https://serde.rs/) / [toml](https://toml.io/) / [directories](https://docs.rs/directories/) — the three crates doing the work
