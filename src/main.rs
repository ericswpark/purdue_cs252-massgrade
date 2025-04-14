use anyhow::{bail, Context, Result};
use clap::Parser;
use massgrade::{check, check_directories};
use cs252chkr::constants::LOC_PATHSPEC;

#[derive(Parser, Debug)]
#[command(name = "massgrade")]
#[command(author = "Eric Park (@ericswpark)")]
struct Cli {
    /// Show git partial logs (`git log -p`). Warning: generates a lot of output
    #[arg(short = 'p', long)]
    git_log_partial: bool,
    /// Ignore pathspec when considering source files
    #[arg(long)]
    ignore_pathspec: bool,
    #[arg(conflicts_with = "ignore_pathspec", long)]
    pathspec: Vec<String>,
    directories: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        bail!("No directories supplied.");
    }

    let pathspec: String = if cli.ignore_pathspec {
        "".to_string()
    } else if !cli.pathspec.is_empty() {
        cli.pathspec.join(" ")
    } else {
        LOC_PATHSPEC.to_string()
    };

    let directories = check_directories(&cli.directories);
    check(&directories, cli.git_log_partial, &pathspec).context("Failed to check directories")?;

    Ok(())
}
