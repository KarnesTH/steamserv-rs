use clap::Parser;
use steamserv_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    if !config.is_initialized {
        config.init().await?;
    } else {
        let cli = Cli::parse();
        match cli.command {
            Commands::Update => {
                println!("Updating...");
            }
            Commands::Install {
                app_id,
                path,
                username,
                password,
            } => {
                println!(
                    "Installing app with ID {} to path {:?} with username {:?} and password {:?}...",
                    app_id, path, username, password
                );
            }
            Commands::Uninstall => {
                println!("Uninstalling...");
            }
            Commands::List => {
                println!("Listing...");
            }
            Commands::Config => {
                println!("Configuring...");
            }
        }
    }

    Ok(())
}
