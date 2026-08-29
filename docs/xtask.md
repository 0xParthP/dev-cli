# XTask

`xtask` is the cross-platform task runner used by dev-cli.

## Commands

| Command | Description |
|----------|-------------|
| cargo xtask ci | Run formatting, linting, security, tests and coverage. |
| cargo xtask coverage | Generate HTML coverage report. |
| cargo xtask coverage-summary | Print terminal coverage summary. |

## Why xtask?

- Cross-platform.
- Written in Rust.
- Same command works locally and in GitHub Actions.
- Extensible for releases, docs and installers.