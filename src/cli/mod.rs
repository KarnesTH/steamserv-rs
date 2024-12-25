pub mod commands;

use clap::Parser;
use commands::Commands;

/// SteamCMD server management tool to install, update, and uninstall game servers.
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
