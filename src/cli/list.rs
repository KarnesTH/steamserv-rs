use std::path::PathBuf;

use crate::utils::{Config, InstalledServer, ServerCache, ServerInfo};

trait ServerDisplay {
    fn get_app_id(&self) -> u32;
    fn get_name(&self) -> &str;
    fn get_path(&self) -> Option<&PathBuf>;
}

impl ServerDisplay for ServerInfo {
    fn get_app_id(&self) -> u32 {
        self.app_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_path(&self) -> Option<&PathBuf> {
        None
    }
}

impl ServerDisplay for InstalledServer {
    fn get_app_id(&self) -> u32 {
        self.app_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_path(&self) -> Option<&PathBuf> {
        Some(&self.install_path)
    }
}

enum ServerType {
    Installed,
    Available,
}

/// Handle the `list` command
///
/// # Arguments
///
/// * `installed` - Show installed servers
/// * `filter` - Filter the list of servers
///
/// # Returns
///
/// Returns `Ok(())` if the command was successful, otherwise an error
///
/// # Errors
///
/// Returns an error if the command fails
pub async fn handle_list_command(
    installed: bool,
    filter: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if installed {
        let config = Config::load()?;
        list_installed_servers(&config.installed_servers, filter)
    } else {
        let cache = ServerCache::load()?;
        list_available_servers(&cache.servers, filter)
    }
}

/// List the available servers
///
/// # Arguments
///
/// * `servers` - The list of available servers
/// * `filter` - Filter the list of servers
///
/// # Returns
///
/// Returns `Ok(())` if the command was successful, otherwise an error
///
/// # Errors
///
/// Returns an error if the command fails
fn list_available_servers(
    servers: &[ServerInfo],
    filter: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filtered: Vec<_> = if let Some(filter) = filter {
        servers
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&filter.to_lowercase()))
            .collect()
    } else {
        servers.iter().collect()
    };

    display_output(ServerType::Available, &filtered)?;

    Ok(())
}

/// List the installed servers
///
/// # Arguments
///
/// * `servers` - The list of installed servers
/// * `filter` - Filter the list of servers
///
/// # Returns
///
/// Returns `Ok(())` if the command was successful, otherwise an error
///
/// # Errors
///
/// Returns an error if the command fails
fn list_installed_servers(
    servers: &[InstalledServer],
    filter: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filtered: Vec<_> = if let Some(filter) = filter {
        servers
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&filter.to_lowercase()))
            .collect()
    } else {
        servers.iter().collect()
    };

    display_output(ServerType::Installed, &filtered)?;

    Ok(())
}

/// Display the output of the list command
///
/// # Arguments
///
/// * `server_type` - The type of server to display
/// * `servers` - The list of servers to display
///
/// # Returns
///
/// Returns `Ok(())` if the command was successful, otherwise an error
///
/// # Errors
///
/// Returns an error if the command fails
fn display_output<T: ServerDisplay>(
    server_type: ServerType,
    servers: &[&T],
) -> Result<(), Box<dyn std::error::Error>> {
    match server_type {
        ServerType::Installed => {
            println!("Installed Servers:");
            println!("{:<10} {:<50} {:<80}", "APP ID", "NAME", "PATH");
            println!("{:-<140}", "");

            for server in servers {
                if let Some(path) = server.get_path() {
                    println!(
                        "{:<10} {:<50} {:<80}",
                        server.get_app_id(),
                        server.get_name(),
                        path.display()
                    );
                }
            }
        }
        ServerType::Available => {
            println!("Available Servers:");
            println!("{:<10} {:<50}", "APP ID", "NAME");
            println!("{:-60}", "");

            for server in servers {
                println!("{:<10} {:<50}", server.get_app_id(), server.get_name());
            }
        }
    }

    Ok(())
}
