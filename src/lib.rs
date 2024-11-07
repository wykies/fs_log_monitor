// TODO 1: Read log and see if it has any errors
// TODO 2: Send notification on errors detected
// TODO 3: Send still alive notification
// TODO 3: Send notification if no logs detected in over 24 hours or over 6 hours and uptime is less than 24 hours

mod cli;

pub use cli::Cli;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    todo!()
}
