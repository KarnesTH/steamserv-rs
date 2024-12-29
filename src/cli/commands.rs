use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Update the game server
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the game server to use
    Update {
        #[arg(short, long)]
        server_name: Option<String>,
    },
    /// Install an game server
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID of the game server
    /// * `server_name` - The name of the game server
    /// * `username` - The username of the Steam account to use
    Install {
        #[arg(short, long)]
        app_id: Option<u32>,
        #[arg(short, long)]
        server_name: Option<String>,
        #[arg(short, long)]
        username: Option<String>,
    },
    /// Uninstall a game server
    Uninstall,
    /// List game servers
    ///
    /// # Arguments
    ///
    /// * `installed` - Show installed game servers
    /// * `filter` - Filter the list of game servers
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
