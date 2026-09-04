use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use directories::BaseDirs;
use owo_colors::OwoColorize;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Instant,
};

#[derive(Parser)]
#[command(name = "cargo xtask")]
#[command(about = "Developer tooling for dev-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the complete local CI pipeline.
    Ci,

    /// Generate HTML coverage.
    Coverage,

    /// Print terminal coverage summary.
    CoverageSummary,

    /// Install the current binary into ~/.local/bin (developer only).
    Install {
        /// Install the release build instead of debug.
        #[arg(long)]
        release: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ci => ci(),
        Commands::Coverage => run("cargo", &["coverage"]),
        Commands::CoverageSummary => run("cargo", &["coverage-summary"]),
        Commands::Install { release } => install(release),
    }
}

fn ci() -> Result<()> {
    let start = Instant::now();

    println!();
    println!("{}", "dev-cli CI Report".bold().cyan());
    println!("{}", "────────────────────────────────────────────".cyan());

    step("Formatting", "cargo", &["fmt-check"])?;
    step("Clippy", "cargo", &["lint"])?;
    step("Security", "cargo", &["security"])?;
    step("Tests", "cargo", &["test-all"])?;
    coverage_step(80.0)?;

    println!();
    println!("{}", "────────────────────────────────────────────".cyan());

    println!("{} {} ({:.2?})", "PASS".green().bold(), "All checks passed".green(), start.elapsed());

    Ok(())
}

fn install(release: bool) -> Result<()> {
    let profile = if release { "release" } else { "debug" };

    println!();
    println!("{}", "Installing dev-cli (developer mode)".bold().cyan());

    // Build latest binary first.
    let mut build = Command::new("cargo");
    build.arg("build");

    if release {
        build.arg("--release");
    }

    let status = build.status().context("Failed to invoke cargo build")?;

    if !status.success() {
        bail!("Build failed.");
    }

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("Couldn't determine workspace root")?
        .to_path_buf();

    let binary = if cfg!(windows) {
        workspace_root.join(format!("target/{profile}/dev.exe"))
    } else {
        workspace_root.join(format!("target/{profile}/dev"))
    };

    let home = BaseDirs::new().context("Couldn't locate home directory")?.home_dir().to_path_buf();

    let install_dir = home.join(".local/bin");
    fs::create_dir_all(&install_dir)?;

    let destination =
        if cfg!(windows) { install_dir.join("dev.exe") } else { install_dir.join("dev") };

    fs::copy(&binary, &destination)
        .with_context(|| format!("Couldn't copy {}", binary.display()))?;

    println!();
    println!("{} Installed successfully", "PASS".green().bold());
    println!("Source      {}", binary.display());
    println!("Destination {}", destination.display());

    if std::env::var_os("PATH")
        .map(|path| !std::env::split_paths(&path).any(|p| p == install_dir))
        .unwrap_or(true)
    {
        println!();
        println!("{}", "PATH reminder".yellow().bold());
        println!("Add this directory to your PATH:");
        println!("{}", install_dir.display());
    }

    Ok(())
}

fn step(name: &str, program: &str, args: &[&str]) -> Result<()> {
    print!("{:<12}", format!("{name}..."));

    let output =
        Command::new(program).args(args).stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

    if output.status.success() {
        println!("{}", "PASS".green());
        Ok(())
    } else {
        println!("{}", "FAIL".red());

        println!();
        println!("{}", "Command output:".yellow());

        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));

        bail!("{name} failed.")
    }
}

fn coverage_step(minimum: f64) -> Result<()> {
    println!("Coverage");

    let output = Command::new("cargo")
        .args(["coverage-summary"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        println!("{}", "FAIL".red());
        bail!("Coverage command failed.");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", "────────────────────────────────────────────".cyan());

    let mut line_coverage = 0.0;

    for line in stdout.lines() {
        if line.starts_with("TOTAL") {
            let columns: Vec<_> = line.split_whitespace().collect();

            let region_coverage = columns[3].trim_end_matches('%').parse::<f64>().unwrap_or(0.0);

            let function_coverage = columns[6].trim_end_matches('%').parse::<f64>().unwrap_or(0.0);

            let line_coverage_value =
                columns[9].trim_end_matches('%').parse::<f64>().unwrap_or(0.0);

            line_coverage = line_coverage_value;

            println!("Functions : {:.2}%", function_coverage);
            println!("Regions   : {:.2}%", region_coverage);
            println!("Lines     : {:.2}%", line_coverage_value);

            break;
        }
    }

    println!();

    if line_coverage >= minimum {
        println!("{} Coverage ({:.2}% ≥ {:.0}%)", "PASS".green().bold(), line_coverage, minimum);
        Ok(())
    } else {
        println!("{} Coverage ({:.2}% < {:.0}%)", "FAIL".red().bold(), line_coverage, minimum);

        bail!("Coverage below threshold.");
    }
}

fn run(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;

    if status.success() { Ok(()) } else { bail!("Command failed.") }
}
