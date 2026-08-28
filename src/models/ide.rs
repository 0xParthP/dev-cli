use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ValueEnum,
)]
pub enum Ide {
    Cursor,
    Vscode,
    Claude,
    Terminal,
    Idea,
    Rider,
    Zed,
}

impl Ide {
    pub fn executable(&self) -> &'static str {
        match self {
            Ide::Cursor => "cursor",
            Ide::Vscode => "code",
            Ide::Claude => "claude",
            Ide::Terminal => "wt",
            Ide::Idea => "idea64.exe",
            Ide::Rider => "rider64.exe",
            Ide::Zed => "zed",
        }
    }
}