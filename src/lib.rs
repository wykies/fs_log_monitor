// TODO 1: Get sample logs to work with from server

mod cli;
mod notification;
mod state;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
pub use cli::Cli;
use notification::send_notification;
use state::AppState;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let (state_file, config_folder) = get_canonical_folder_and_filename(&cli.state_file)?;
    let mut app_state = AppState::load(state_file).context("failed to load state")?;
    if cli.print_state_only {
        println!("{app_state:#?}");
        return Ok(());
    }
    if let Some(msg) = &cli.test_notification {
        send_notification(msg, &config_folder).context("sending test notification failed")?;
        println!("TEST NOTIFICATION SENT");
        return Ok(());
    }
    if app_state.alive_msg_due() {
        let alive_msg = app_state.generate_alive_msg();
        send_notification(&alive_msg, &config_folder).context("failed to send alive message")?;
    }

    // TODO 1: Read log and see if it has any errors
    // TODO 2: Send notification on errors detected
    // TODO 3: Send notification if no logs detected in over 24 hours or over 6 hours and uptime is less than 24 hours
    if app_state.is_changed() {
        app_state
            .save(&cli.state_file)
            .context("failed to save state")?;
    }
    println!("RUN COMPLETED");
    Ok(())
}

/// Converts the input into it's canonical form and based on the assumption that it is a file also returns the parent folder
fn get_canonical_folder_and_filename<P: AsRef<Path>>(
    file_path: P,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    // TODO 4: Test what this error looks like and if we need to add more context
    let canonical_file_path = file_path.as_ref().canonicalize()?;
    let parent_folder = canonical_file_path
        .parent()
        .ok_or(anyhow!(
            "failed to get parent folder for file: {}",
            canonical_file_path.display()
        ))?
        .to_path_buf();
    Ok((canonical_file_path, parent_folder))
}

pub fn init_state<P: AsRef<Path>>(logs: P, state_file: P) -> anyhow::Result<()> {
    let mut state = AppState::new(logs.as_ref().to_path_buf());
    state.save(state_file)
}
