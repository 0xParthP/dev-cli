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

impl Ide {
    /// Human-readable display label for TUI badges.
    pub fn display_name(&self) -> &'static str {
        match self {
            Ide::Cursor => "Cursor",
            Ide::Vscode => "VS Code",
            Ide::Claude => "Claude Code",
            Ide::Terminal => "Terminal",
            Ide::Idea => "IntelliJ IDEA",
            Ide::Rider => "Rider",
            Ide::Zed => "Zed",
        }
    }

    /// Icon logo representing the IDE.
    pub fn icon(&self) -> &'static str {
        match self {
            Ide::Cursor => "🎯",
            Ide::Vscode => "🟦",
            Ide::Claude => "🤖",
            Ide::Terminal => "💻",
            Ide::Idea => "💡",
            Ide::Rider => "🚀",
            Ide::Zed => "⚡",
        }
    }

    /// Distinct theme color for each IDE badge.
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            Ide::Cursor => ratatui::style::Color::Rgb(0, 220, 200), // Cyan / Teal
            Ide::Vscode => ratatui::style::Color::Rgb(35, 145, 255), // VS Code Blue
            Ide::Claude => ratatui::style::Color::Rgb(220, 130, 70), // Anthropic Copper
            Ide::Terminal => ratatui::style::Color::Rgb(140, 210, 90), // Terminal Green
            Ide::Idea => ratatui::style::Color::Rgb(255, 80, 140),  // IntelliJ Pink
            Ide::Rider => ratatui::style::Color::Rgb(180, 80, 240), // Rider Purple
            Ide::Zed => ratatui::style::Color::Rgb(255, 195, 45),   // Zed Amber
        }
    }

    /// Return the next IDE variant in cycling sequence.
    pub fn next(&self) -> Self {
        match self {
            Ide::Cursor => Ide::Vscode,
            Ide::Vscode => Ide::Claude,
            Ide::Claude => Ide::Terminal,
            Ide::Terminal => Ide::Idea,
            Ide::Idea => Ide::Rider,
            Ide::Rider => Ide::Zed,
            Ide::Zed => Ide::Cursor,
        }
    }
}
