use clap::Parser;
use steamserv_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    if !config.is_initialized {
        config.init().await?;
        println!("Creating initial server cache...");
        let mut cache = ServerCache::default();
        cache.update_cache().await?;
        println!("Setup complete! You can now use steamserv.");
    } else {
        let cli = Cli::parse();
        match cli.command {
            Commands::Update { server_name } => {
                SteamCMD::update(server_name)?;
            }
            Commands::Install {
                app_id,
                server_name,
                username,
            } => {
                SteamCMD::install(app_id, server_name, username)?;
            }
            Commands::Uninstall { server_name } => {
                SteamCMD::uninstall(server_name)?;
            }
            Commands::List { installed, filter } => {
                handle_list_command(installed, filter).await?;
            }
            Commands::Config => {
                println!("Configuring...");
            }
        }
    }

    Ok(())
}
