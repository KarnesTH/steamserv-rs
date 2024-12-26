use crate::utils::{Config, InstalledServer, ServerCache, ServerInfo};

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

    println!("Available Servers:");
    println!("{:<10} {}", "APP ID", "NAME");
    println!("{:-50}", "");

    for server in filtered {
        println!("{:<10} {}", server.app_id, server.name);
    }

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

    println!("\nInstalled Servers:");
    println!("{:<10} {:<30} {}", "APP ID", "NAME", "PATH");
    println!("{:-<70}", "");

    for server in filtered {
        println!(
            "{:<10} {:<30} {}",
            server.app_id,
            server.name,
            server.install_path.display()
        );
    }

    Ok(())
}
