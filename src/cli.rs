//! CLI argument parsing using Clap derive macros.

use crate::models::ide::Ide;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dev")]
#[command(version)]
#[command(about = "Modern Git Project Manager")]
#[command(long_about = None)]
#[command(help_template = "\n{about-with-newline} {usage-heading} {usage}\n\n {all-args}\n")]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands for dev-cli.
#[derive(Subcommand)]
pub enum Commands {
    /// Project Management
    Project(ProjectCommand),

    /// Configuration Management.
    Config(ConfigCommand),

    /// IDE Management
    Ide(IdeCommand),

    /// Open a project (equivalent to `project open`).
    Open(OpenArgs),
}

/// Arguments for opening a project.
#[derive(Args)]
pub struct OpenArgs {
    /// Name of the project to open.
    pub project: String,

    /// IDE to use for opening (overrides config default).
    #[arg(short, long)]
    pub ide: Option<Ide>,
}

/// Subcommands for `dev project`.
#[derive(Subcommand)]
pub enum ProjectSubcommand {
    /// List all configured project root directories.
    List,

    /// Open a project in an IDE.
    Open(OpenArgs),
}

/// Arguments for `dev project` command.
#[derive(Args)]
pub struct ProjectCommand {
    /// The specific project subcommand.
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

/// Subcommands for `dev config`.
#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Initialize configuration file with defaults.
    Init,

    /// Display current configuration.
    Show,

    /// Set the default IDE to use when opening projects.
    SetDefaultIde {
        /// The IDE to set as default.
        ide: Ide,
    },
}

/// Arguments for `dev config` command.
#[derive(Args)]
pub struct ConfigCommand {
    /// The specific config subcommand.
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

/// Arguments for `dev ide` command.
#[derive(Args)]
pub struct IdeCommand {
    /// The specific IDE subcommand.
    #[command(subcommand)]
    pub command: IdeSubcommand,
}

/// Subcommands for `dev ide`.
#[derive(Subcommand)]
pub enum IdeSubcommand {
    /// List all detected IDEs on the system.
    List,
}
