pub mod commands;
pub mod list;

use clap::Parser;
use commands::Commands;
pub use list::handle_list_command;

/// SteamCMD server management tool to install, update, and uninstall game servers.
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
