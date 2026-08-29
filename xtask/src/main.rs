use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::{
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ci => ci(),
        Commands::Coverage => run("cargo", &["coverage"]),

        Commands::CoverageSummary => run("cargo", &["coverage-summary"]),
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

            line_coverage = columns[8].trim_end_matches('%').parse::<f64>().unwrap_or(0.0);

            let function = columns[5];
            let region = columns[2];

            println!("Functions : {}%", function.green());
            println!("Regions   : {}%", region.green());
            println!("Lines     : {}%", columns[8].green());
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
