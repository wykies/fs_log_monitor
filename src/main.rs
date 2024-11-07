use clap::Parser;
use fs_log_monitor::{run, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(&cli)?;
    Ok(())
}
