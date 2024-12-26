use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Update the SteamCMD installation
    Update,
    /// Install an game server
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID of the game server to install
    /// * `path` - The path to install the game server to
    /// * `username` - The username to use when installing the game server
    /// * `password` - The password to use when installing the game server
    Install {
        app_id: u32,
        path: PathBuf,
        username: Option<String>,
        password: Option<String>,
    },
    /// Uninstall a game server
    Uninstall,
    /// List game servers
    List {
        /// Show installed game servers
        #[arg(short, long)]
        installed: bool,
        /// Filter the list of game servers
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Configure the SteamCMD installation
    Config,
}
