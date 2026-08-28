use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Ide {
    Cursor,
    Vscode,
    Claude,
    Terminal,
    Idea,
    Rider,
    Zed,
}
