// TODO 1: Get sample logs to work with from server

mod cli;
mod notification;
mod state;

use std::path::Path;

use anyhow::Context;
pub use cli::Cli;
use notification::send_notification;
use state::AppState;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let mut state = AppState::load(&cli.state_file).context("failed to load state")?;
    if cli.print_state_only {
        println!("{state:?}");
        return Ok(());
    }
    if let Some(msg) = &cli.test_notification {
        send_notification(msg).context("sending test notification failed")?;
        return Ok(());
    }

    // TODO 1: Read log and see if it has any errors
    // TODO 2: Send notification on errors detected
    // TODO 3: Send still alive notification
    // TODO 3: Send notification if no logs detected in over 24 hours or over 6 hours and uptime is less than 24 hours
    if state.is_changed() {
        state
            .save(&cli.state_file)
            .context("failed to save state")?;
    }
    Ok(())
}

pub fn init_state<P: AsRef<Path>>(logs: P, state_file: P) -> anyhow::Result<()> {
    let mut state = AppState::new(logs.as_ref().to_path_buf());
    state.save(state_file)
}
