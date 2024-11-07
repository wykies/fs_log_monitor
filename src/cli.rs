use clap::Parser;

#[derive(Parser, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
#[command(
    author,
    version,
    about,
    long_about = "Monitors the FreeFileSync logs and reports errors"
)]
pub struct Cli {
    /// Print state only and exit
    #[arg(long, short)]
    pub print_state_only: bool, // TODO 1: Implement debugging tool
}
