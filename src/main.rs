use anyhow::Context;
use clap::Parser;
use fs_log_monitor::{init_state, run, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if let Some(logs_path) = &cli.init {
        return init_state(logs_path, &cli.state_file).context("failed to initialize state");
    }
    run(&cli)
}
