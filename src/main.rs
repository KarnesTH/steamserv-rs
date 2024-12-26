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
            Commands::Update => {
                println!("Updating...");
            }
            Commands::Install {
                app_id,
                server_name,
                username,
            } => {
                if let Some(app_id) = app_id {
                    println!("Installing app with ID {}...", app_id);
                }
                if let Some(server_name) = server_name {
                    println!("Installing app with name {:?}...", server_name);
                }
                if let Some(username) = username {
                    println!("Installing app with username {:?}...", username);
                }
            }
            Commands::Uninstall => {
                println!("Uninstalling...");
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
