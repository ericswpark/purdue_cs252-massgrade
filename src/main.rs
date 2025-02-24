use clap::Parser;
use anyhow::{bail, Context, Result};
use massgrade::{check, check_directories};

#[derive(Parser, Debug)]
#[command(name = "massgrade")]
#[command(author = "Eric Park (@ericswpark)")]
struct Cli {
    directories: Vec<String>
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        bail!("No directories supplied.");
    }

    let directories = check_directories(&cli.directories);
    check(&directories).context("Failed to check directories")?;

    Ok(())
}
