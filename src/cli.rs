use clap::{Args, Parser, Subcommand};

use crate::models::ide::Ide;

#[derive(Parser)]
#[command(name = "dev")]
#[command(version)]
#[command(about = "Modern Git Project Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),

    Config(ConfigCommand),

    Ide(IdeCommand),

    Install,

    Open(OpenArgs),
}

#[derive(Args)]
pub struct OpenArgs {
    pub project: String,

    #[arg(short, long)]
    pub ide: Option<Ide>,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    List,

    Open(OpenArgs),
}

#[derive(Args)]
pub struct ProjectCommand {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    Init,

    Show,

    SetDefaultIde {
        ide: Ide,
    },
}

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

#[derive(Args)]
pub struct IdeCommand {
    #[command(subcommand)]
    pub command: IdeSubcommand,
}

#[derive(Subcommand)]
pub enum IdeSubcommand {
    List,
}