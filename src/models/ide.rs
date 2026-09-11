//! IDE type definitions.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

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
