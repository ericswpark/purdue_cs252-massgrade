use clap::Parser;
use anyhow::{bail, Context, Result};
use massgrade::{check, check_directories};

#[derive(Parser, Debug)]
#[command(name = "massgrade")]
#[command(author = "Eric Park (@ericswpark)")]
struct Cli {
    /// Show git partial logs (`git log -p`). Warning: generates a lot of output
    #[arg(short = 'p', long)]
    git_log_partial: bool,
    directories: Vec<String>
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        bail!("No directories supplied.");
    }

    let directories = check_directories(&cli.directories);
    check(&directories, cli.git_log_partial).context("Failed to check directories")?;

    Ok(())
}
