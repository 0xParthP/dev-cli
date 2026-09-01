# Architecture Diagrams

This file contains the detailed visual documentation for the `dev-cli` architecture.

## Overview & Layered Architecture

For the high-level system overview, see [architecture.md](architecture.md).

The following diagram illustrates the strict layered dependency direction (no upward imports):

```mermaid
flowchart TD
    CLI[src/cli.rs & src/main.rs] --> Commands[src/commands/*]
    Commands --> Services[src/config.rs, src/ide/*.rs, src/scanner.rs, src/installer.rs]
    Services --> Models[src/models/*]
```

## Command Dispatch Flow

When a user executes a command, it follows this path:

```mermaid
graph LR
    User -->|Args| CLI[src/cli.rs - Clap]
    CLI -->|Parsed Enum| Main[src/main.rs - match]
    Main -->|execute| CMD[src/commands/*.rs - Handlers]
    CMD -->|Use| SVC[src/*.rs - Services]
```

## IDE Detection Pipeline

The detection logic runs multi-stage detection:

```mermaid
flowchart TD
    Start --> Which[which crate - PATH scan]
    Which --> Common[Common Windows dirs: AppData, ~/.local/bin]
    Common --> Dedupe[src/ide/detect.rs - Dedupe]
    Dedupe --> Result[src/ide/registry.rs - InstalledIde]
```

## Config Lifecycle

The configuration is managed with a "load-or-create-defaults" pattern:

```mermaid
sequenceDiagram
    participant App
    participant Config
    participant Filesystem
    
    App->>Config: Config::load()
    Config->>Filesystem: Check DEVCLI_CONFIG_DIR / Default
    Filesystem-->>Config: Return content or NotFound
    alt File Missing
        Config->>Config: Default::default()
        Config->>Filesystem: save()
    else Found
        Config->>Filesystem: parse toml
    end
    Config-->>App: Config struct
```

## CI Pipeline

Automated workflows ensure code quality, coverage, and security:

```mermaid
graph LR
    CI[ci.yml: fmt/lint/test/doc] --> Coverage[coverage.yml: ≥80% gate]
    Coverage --> Security[security.yml: cargo deny]
    Security --> Branch[branch-name.yml]
    Branch --> Release[release.yml]
```

## How to Keep This Current

- **Adding/Renaming Modules:** If you create a new module, update the **Layered Architecture** diagram if it introduces a new service or model, or the **Command Dispatch Flow** if it is a new command handler.
- **Dependency Changes:** If you add a cross-layer dependency, ensure it complies with the layering rules, and update the **Layered Architecture** diagram if necessary.
- **Cross-references:** Keep these diagrams synchronized with the module descriptions in [modules.md](modules.md) and the dependency graph in [dependency-map.md](dependency-map.md).
- **Validation:** When modifying, verify that the module paths in the diagrams match the actual file structure in the `src/` directory.
