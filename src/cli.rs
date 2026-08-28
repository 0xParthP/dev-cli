//! CLI argument parsing using Clap derive macros.
//!
//! This module defines the command-line interface for dev-cli using Clap's
//! derive API. All public structs and enums are used for parsing command-line
//! arguments into structured data.
//!
//! # Structure
//!
//! - [`Cli`] — Top-level command struct
//! - [`Commands`] — Available subcommands
//! - Command-specific `Args` and `Subcommand` structs

use clap::{Args, Parser, Subcommand};

use crate::models::ide::Ide;

/// Top-level CLI arguments.
///
/// Parsed from command line and contains a subcommand to dispatch to.
#[derive(Parser)]
#[command(name = "dev")]
#[command(version)]
#[command(about = "Modern Git Project Manager")]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for dev-cli.
///
/// Each variant corresponds to a major feature:
/// - `Project` — List and open projects
/// - `Config` — Manage configuration
/// - `Ide` — List detected IDEs
/// - `Install` — Install globally
/// - `Open` — Shorthand for opening projects
#[derive(Subcommand)]
pub enum Commands {
    /// Project management (list, open).
    Project(ProjectCommand),

    /// Configuration management (show, init, set default IDE).
    Config(ConfigCommand),

    /// IDE management (list detected IDEs).
    Ide(IdeCommand),

    /// Install dev-cli globally in ~/.local/bin.
    Install,

    /// Open a project (shorthand for `project open`).
    ///
    /// Usage: `dev open <PROJECT> [--ide CURSOR]`
    Open(OpenArgs),
}

/// Arguments for opening a project.
///
/// Specifies which project to open and optionally which IDE to use.
#[derive(Args)]
pub struct OpenArgs {
    /// Name of the project to open.
    pub project: String,

    /// IDE to use for opening (overrides config default).
    #[arg(short, long)]
    pub ide: Option<Ide>,
}

/// Subcommands for `dev project`.
///
/// - `List` — List configured projects
/// - `Open` — Open a specific project
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
///
/// - `Init` — Initialize configuration file
/// - `Show` — Display current configuration
/// - `SetDefaultIde` — Set the default IDE
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
///
/// - `List` — List detected IDEs
#[derive(Subcommand)]
pub enum IdeSubcommand {
    /// List all detected IDEs on the system.
    List,
}
