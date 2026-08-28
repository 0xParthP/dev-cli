//! IDE type definitions.
//!
//! Defines the [`Ide`] enum representing all supported integrated development environments.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Supported integrated development environments.
///
/// Enum of IDE types that dev-cli can detect and launch.
///
/// # Variants
///
/// - `Cursor` — [Cursor](https://cursor.sh/) — AI-powered code editor
/// - `Vscode` — VS Code — Microsoft's popular editor
/// - `Claude` — Claude Code — Claude AI editor
/// - `Terminal` — Windows Terminal — Command line interface
/// - `Idea` — IntelliJ IDEA — JetBrains Java IDE
/// - `Rider` — JetBrains Rider — .NET IDE
/// - `Zed` — Zed — High-performance editor
///
/// # Serialization
///
/// Implements Serialize and Deserialize for TOML config files.
/// Implements ValueEnum for Clap CLI parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Ide {
    /// Cursor — AI-powered code editor
    Cursor,
    
    /// VS Code — Visual Studio Code
    Vscode,
    
    /// Claude Code — Claude AI editor
    Claude,
    
    /// Windows Terminal — Terminal/CLI
    Terminal,
    
    /// IntelliJ IDEA — Java IDE
    Idea,
    
    /// JetBrains Rider — .NET IDE
    Rider,
    
    /// Zed — High-performance editor
    Zed,
}
