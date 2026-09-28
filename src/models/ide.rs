//! IDE type definitions.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Ide {
    /// Antigravity — Google Antigravity Agentic IDE
    Antigravity,

    /// Antigravity CLI — Antigravity command line tool
    AntigravityCli,

    /// Cursor — AI-powered code editor
    Cursor,

    /// VS Code — Visual Studio Code
    Vscode,

    /// Windsurf — Codeium Windsurf AI IDE
    Windsurf,

    /// Claude Code — Claude AI CLI editor
    Claude,

    /// Sublime Text — Lightweight code editor
    Sublime,

    /// Neovim — Hyperextensible Vim-based text editor
    Neovim,

    /// Zed — High-performance Rust-based editor
    Zed,

    /// IntelliJ IDEA — Java & JVM IDE
    Idea,

    /// PyCharm — Python IDE
    PyCharm,

    /// WebStorm — JavaScript & TypeScript IDE
    WebStorm,

    /// CLion — C & C++ IDE
    CLion,

    /// RustRover — Rust IDE
    RustRover,

    /// GoLand — Go IDE
    GoLand,

    /// Fleet — JetBrains Next-Gen IDE
    Fleet,

    /// JetBrains Rider — .NET & Game Dev IDE
    Rider,

    /// Android Studio — Android Mobile IDE
    AndroidStudio,

    /// Visual Studio — Microsoft IDE
    VisualStudio,

    /// Helix — Modal terminal editor
    Helix,

    /// Windows Terminal — Terminal/CLI
    Terminal,
}

impl Ide {
    fn info(&self) -> (&'static str, &'static str, ratatui::style::Color) {
        use ratatui::style::Color::*;
        match self {
            Ide::Antigravity => ("Antigravity", "🌌", Rgb(140, 90, 255)),
            Ide::AntigravityCli => ("Antigravity CLI", "⚡", Rgb(175, 120, 255)),
            Ide::Cursor => ("Cursor", "🎯", Rgb(0, 220, 200)),
            Ide::Vscode => ("VS Code", "🟦", Rgb(35, 145, 255)),
            Ide::Windsurf => ("Windsurf", "🏄", Rgb(0, 210, 180)),
            Ide::Claude => ("Claude Code", "🤖", Rgb(220, 130, 70)),
            Ide::Sublime => ("Sublime Text", "🧡", Rgb(255, 150, 40)),
            Ide::Neovim => ("Neovim", "🟩", Rgb(87, 175, 77)),
            Ide::Zed => ("Zed", "⚡", Rgb(255, 195, 45)),
            Ide::Idea => ("IntelliJ IDEA", "💡", Rgb(255, 80, 140)),
            Ide::PyCharm => ("PyCharm", "🐍", Rgb(70, 210, 120)),
            Ide::WebStorm => ("WebStorm", "🌐", Rgb(0, 200, 240)),
            Ide::CLion => ("CLion", "⚙️", Rgb(50, 170, 220)),
            Ide::RustRover => ("RustRover", "🦀", Rgb(240, 100, 50)),
            Ide::GoLand => ("GoLand", "🦫", Rgb(60, 200, 210)),
            Ide::Fleet => ("Fleet", "⚡", Rgb(160, 110, 255)),
            Ide::Rider => ("Rider", "🚀", Rgb(180, 80, 240)),
            Ide::AndroidStudio => ("Android Studio", "📱", Rgb(100, 195, 115)),
            Ide::VisualStudio => ("Visual Studio", "💜", Rgb(130, 85, 225)),
            Ide::Helix => ("Helix", "🧬", Rgb(170, 100, 240)),
            Ide::Terminal => ("Terminal", "💻", Rgb(140, 210, 90)),
        }
    }

    /// Human-readable display label for TUI badges.
    pub fn display_name(&self) -> &'static str {
        self.info().0
    }

    /// Icon logo representing the IDE.
    pub fn icon(&self) -> &'static str {
        self.info().1
    }

    /// Distinct theme color for each IDE badge.
    pub fn color(&self) -> ratatui::style::Color {
        self.info().2
    }

    /// Return the next IDE variant in cycling sequence.
    pub fn next(&self) -> Self {
        match self {
            Ide::Antigravity => Ide::AntigravityCli,
            Ide::AntigravityCli => Ide::Cursor,
            Ide::Cursor => Ide::Vscode,
            Ide::Vscode => Ide::Windsurf,
            Ide::Windsurf => Ide::Claude,
            Ide::Claude => Ide::Sublime,
            Ide::Sublime => Ide::Neovim,
            Ide::Neovim => Ide::Zed,
            Ide::Zed => Ide::Idea,
            Ide::Idea => Ide::PyCharm,
            Ide::PyCharm => Ide::WebStorm,
            Ide::WebStorm => Ide::CLion,
            Ide::CLion => Ide::RustRover,
            Ide::RustRover => Ide::GoLand,
            Ide::GoLand => Ide::Fleet,
            Ide::Fleet => Ide::Rider,
            Ide::Rider => Ide::AndroidStudio,
            Ide::AndroidStudio => Ide::VisualStudio,
            Ide::VisualStudio => Ide::Helix,
            Ide::Helix => Ide::Terminal,
            Ide::Terminal => Ide::Antigravity,
        }
    }
}

/// A unified representation of a selected IDE — either a built-in auto-detected IDE or a custom IDE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdeSelection {
    /// A built-in supported IDE (VS Code, Cursor, Antigravity, etc.).
    BuiltIn(Ide),
    /// A user-configured custom IDE referenced by its unique string identifier.
    Custom(String),
}

impl Default for IdeSelection {
    fn default() -> Self {
        IdeSelection::BuiltIn(Ide::Vscode)
    }
}

impl From<Ide> for IdeSelection {
    fn from(ide: Ide) -> Self {
        IdeSelection::BuiltIn(ide)
    }
}

impl std::fmt::Display for IdeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdeSelection::BuiltIn(ide) => write!(f, "{}", ide.display_name()),
            IdeSelection::Custom(id) => write!(f, "{}", id),
        }
    }
}

impl std::str::FromStr for IdeSelection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "antigravity" => Ok(IdeSelection::BuiltIn(Ide::Antigravity)),
            "antigravity-cli" | "agy" => Ok(IdeSelection::BuiltIn(Ide::AntigravityCli)),
            "cursor" => Ok(IdeSelection::BuiltIn(Ide::Cursor)),
            "vscode" | "code" => Ok(IdeSelection::BuiltIn(Ide::Vscode)),
            "windsurf" => Ok(IdeSelection::BuiltIn(Ide::Windsurf)),
            "claude" => Ok(IdeSelection::BuiltIn(Ide::Claude)),
            "sublime" | "subl" => Ok(IdeSelection::BuiltIn(Ide::Sublime)),
            "neovim" | "nvim" | "vim" => Ok(IdeSelection::BuiltIn(Ide::Neovim)),
            "zed" => Ok(IdeSelection::BuiltIn(Ide::Zed)),
            "idea" | "intellij" => Ok(IdeSelection::BuiltIn(Ide::Idea)),
            "pycharm" => Ok(IdeSelection::BuiltIn(Ide::PyCharm)),
            "webstorm" | "wstorm" => Ok(IdeSelection::BuiltIn(Ide::WebStorm)),
            "clion" => Ok(IdeSelection::BuiltIn(Ide::CLion)),
            "rustrover" => Ok(IdeSelection::BuiltIn(Ide::RustRover)),
            "goland" => Ok(IdeSelection::BuiltIn(Ide::GoLand)),
            "fleet" => Ok(IdeSelection::BuiltIn(Ide::Fleet)),
            "rider" => Ok(IdeSelection::BuiltIn(Ide::Rider)),
            "studio" | "android-studio" => Ok(IdeSelection::BuiltIn(Ide::AndroidStudio)),
            "visualstudio" | "devenv" | "vs" => Ok(IdeSelection::BuiltIn(Ide::VisualStudio)),
            "helix" | "hx" => Ok(IdeSelection::BuiltIn(Ide::Helix)),
            "terminal" | "wt" => Ok(IdeSelection::BuiltIn(Ide::Terminal)),
            _ => Ok(IdeSelection::Custom(s.to_string())),
        }
    }
}

impl IdeSelection {
    /// Get display name for this IDE selection.
    pub fn display_name<'a>(
        &'a self,
        custom_ides: &'a [crate::models::custom_ide::CustomIde],
    ) -> &'a str {
        match self {
            IdeSelection::BuiltIn(ide) => ide.display_name(),
            IdeSelection::Custom(id) => {
                if let Some(custom) = custom_ides.iter().find(|c| c.id == *id) {
                    &custom.display_name
                } else {
                    id.as_str()
                }
            }
        }
    }

    /// Icon logo representing the IDE.
    pub fn icon<'a>(&'a self, _custom_ides: &'a [crate::models::custom_ide::CustomIde]) -> &'a str {
        match self {
            IdeSelection::BuiltIn(ide) => ide.icon(),
            IdeSelection::Custom(_) => "📝",
        }
    }

    /// Distinct theme color for each IDE badge.
    pub fn color(
        &self,
        _custom_ides: &[crate::models::custom_ide::CustomIde],
    ) -> ratatui::style::Color {
        match self {
            IdeSelection::BuiltIn(ide) => ide.color(),
            IdeSelection::Custom(_) => ratatui::style::Color::Rgb(255, 180, 50),
        }
    }
}
