use std::{path::PathBuf, process::Output};

use inquire::{Confirm, Password, Select, Text};

use crate::utils::{
    config::{LoginType, Platform},
    run_with_output, Config, InstalledServer, ServerCache,
};

pub struct SteamCMD {
    pub login: (String, String),
    pub force_install_dir: String,
    pub app_update: u32,
}

impl SteamCMD {
    /// Install a game server
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID of the game server
    /// * `server_name` - The name of the game server
    /// * `username` - The username of the Steam account to use
    ///
    /// # Returns
    ///
    /// Ok if the game server was installed successfully
    ///
    /// # Errors
    ///
    /// If the game server could not be installed
    pub fn install(
        app_id: Option<u32>,
        server_name: Option<String>,
        username: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        println!("Welcome to your installation guide");

        let force_install_dir = Self::get_force_install_dir(config.clone(), server_name)?;

        let login = match username {
            Some(username) => Self::get_login(Some(username))?,
            None => Self::get_login(None)?,
        };

        let app_update = match app_id {
            Some(app_id) => Self::get_app_update(Some(app_id))?,
            None => Self::get_app_update(None)?,
        };

        let server_name = force_install_dir.split('/').last().unwrap().to_string();

        let login_type = match login.clone() {
            (username, _) => match username.as_str() {
                "anonymous" => LoginType::Anonymous,
                _ => LoginType::SteamAccount,
            },
        };

        let install_path = PathBuf::from(&force_install_dir);

        match Self::ceck_platform(config.clone(), app_update, Some(login.clone())) {
            Ok(platforms) => {
                if platforms.is_empty() {
                    return Err("Could not detect the platform".into());
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        let steamcmd = SteamCMD {
            login,
            force_install_dir,
            app_update,
        };

        Self::execute_install_command(steamcmd, config.steamcmd_path.clone())?;

        let server = InstalledServer {
            app_id: app_update,
            name: server_name,
            install_path,
            install_date: chrono::Local::now().to_utc(),
            last_updated: chrono::Local::now().to_utc(),
            port: None,
            login_type,
        };

        config.installed_servers.push(server);

        config.save()?;

        println!("Server installation successfully.");

        Ok(())
    }

    /// Update a game server
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the game server
    ///
    /// # Returns
    ///
    /// Ok if the game server was updated successfully
    ///
    /// # Errors
    ///
    /// If the game server could not be updated
    pub fn update(server_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;
        let servers: Vec<InstalledServer> = config.installed_servers.clone();

        let server_names = servers
            .iter()
            .map(|s| s.name.clone())
            .collect::<Vec<String>>();

        let server_name = match server_name {
            Some(server_name) => {
                if server_names.contains(&server_name) {
                    server_name
                } else {
                    Text::new("Please enter the name of the game server:")
                        .with_placeholder("e.g. TestServer")
                        .with_help_message("It's the name for your game server folder.")
                        .prompt()?
                }
            }
            None => {
                let server_name =
                    Select::new("Please select the game server to update", server_names)
                        .with_help_message("Which of this game servers you will update?")
                        .prompt()?;
                server_name
            }
        };

        let server = servers.iter().find(|s| s.name == server_name).unwrap();

        let login = match server.login_type {
            LoginType::Anonymous => ("anonymous".to_string(), "".to_string()),
            LoginType::SteamAccount => {
                let username = Text::new("Please enter your steam username:").prompt()?;
                let password = Password::new("Please enter your password for your steam account.")
                    .without_confirmation()
                    .prompt()?;
                (username, password)
            }
        };

        let force_install_dir = server.install_path.clone();
        let app_update = server.app_id;

        let steamcmd = SteamCMD {
            login,
            force_install_dir: force_install_dir.display().to_string(),
            app_update,
        };

        Self::execute_install_command(steamcmd, config.steamcmd_path.clone())?;

        if let Some(server) = config
            .installed_servers
            .iter_mut()
            .find(|s| s.name == server_name)
        {
            server.update_timestamp();
            config.save()?;
        }

        println!("Server update successfully.");

        Ok(())
    }

    /// Uninstall a game server
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the game server
    ///
    /// # Returns
    ///
    /// Ok if the game server was uninstalled successfully
    ///
    /// # Errors
    ///
    /// If the game server could not be uninstalled
    pub fn uninstall(server_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;
        let servers: Vec<InstalledServer> = config.installed_servers.clone();

        let server_names = servers
            .iter()
            .map(|s| s.name.clone())
            .collect::<Vec<String>>();

        let server_name = match server_name {
            Some(server_name) => {
                if server_names.contains(&server_name) {
                    server_name
                } else {
                    Text::new("Please enter the name of the game server:")
                        .with_placeholder("e.g. TestServer")
                        .with_help_message("It's the name for your game server folder.")
                        .prompt()?
                }
            }
            None => {
                let server_name =
                    Select::new("Please select the game server to uninstall", server_names)
                        .with_help_message("Which of this game servers you will uninstall?")
                        .prompt()?;
                server_name
            }
        };

        let server = servers.iter().find(|s| s.name == server_name).unwrap();

        let force_install_dir = server.install_path.clone();

        let confirm = Confirm::new(&format!(
            "Are you sure you want to uninstall the server {}?",
            server_name
        ))
        .prompt()?;

        if confirm {
            std::fs::remove_dir_all(force_install_dir)?;
            config.installed_servers.retain(|s| s.name != server_name);
            config.save()?;
            println!("Server uninstalled successfully.");
        }

        Ok(())
    }

    /// Execute the install command
    ///
    /// # Arguments
    ///
    /// * `steamcmd` - The SteamCMD configuration
    /// * `steamcmd_path` - The path to the SteamCMD installation
    ///
    /// # Returns
    ///
    /// Ok if the install command was executed successfully
    ///
    /// # Errors
    ///
    /// If the install command could not be executed
    fn execute_install_command(
        steamcmd: SteamCMD,
        steamcmd_path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut install_child = std::process::Command::new(steamcmd_path)
            .arg(format!(
                "+force_install_dir {}",
                steamcmd.force_install_dir.clone()
            ))
            .arg(format!(
                "+login {} {}",
                steamcmd.login.clone().0,
                steamcmd.login.1,
            ))
            .arg(format!("+app_update {} validate", steamcmd.app_update))
            .arg("+quit")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        run_with_output(&mut install_child)?;

        let install_status = install_child.wait()?;
        if !install_status.success() {
            return Err("Could not install game server".into());
        }
        Ok(())
    }

    /// Check the Steam App ID
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID of the game server
    ///
    /// # Returns
    ///
    /// The name of the game server
    ///
    /// # Errors
    ///
    /// If the game server could not be found
    fn check_app_id(app_id: u32) -> Result<String, Box<dyn std::error::Error>> {
        let servers = ServerCache::load()?;
        let server = if let Some(server) = servers.servers.iter().find(|s| s.app_id == app_id) {
            server
        } else {
            return Err("Could not find server".into());
        };
        Ok(server.name.clone())
    }

    /// Get the force install directory
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration
    /// * `server_name` - The name of the game server
    ///
    /// # Returns
    ///
    /// The force install directory
    ///
    /// # Errors
    ///
    /// If the force install directory could not be found
    fn get_force_install_dir(
        config: Config,
        server_name: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let force_install_dir = match server_name {
            Some(server_name) => {
                let confirm = Confirm::new(&format!(
                    "Would you like to install the server with this name {}?",
                    server_name
                ))
                .prompt()?;
                if confirm {
                    format!("{}/{}", config.install_path.display(), server_name)
                } else {
                    Text::new("Please enter the name of the game server:")
                        .with_placeholder("e.g. TestServer")
                        .with_help_message("It's the name for your game server folder.")
                        .prompt()?;
                    format!("{}/{}", config.install_path.display(), server_name)
                }
            }
            None => {
                let name = Text::new("Please enter the name of the game server:")
                    .with_placeholder("e.g. TestServer")
                    .with_help_message("It's the name for your game server folder.")
                    .prompt()?;
                format!("{}/{}", config.install_path.display(), name)
            }
        };
        Ok(force_install_dir)
    }

    /// Get the login information
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the Steam account
    ///
    /// # Returns
    ///
    /// The login information
    ///
    /// # Errors
    ///
    /// If the login information could not be found
    fn get_login(username: Option<String>) -> Result<(String, String), Box<dyn std::error::Error>> {
        let login = match username {
            Some(username) => match username.as_str() {
                "anonymous" => Some(("anonymous".to_string(), "".to_string())),
                _ => {
                    let password =
                        Password::new("Please enter your password for your steam account.")
                            .without_confirmation()
                            .prompt()?;
                    Some((username, password))
                }
            },
            None => {
                let login_type = vec!["anonymous", "steam account"];
                let select_login = Select::new("Please select your login", login_type)
                    .with_help_message("Which of this logins you will use?")
                    .prompt()?;
                match select_login {
                    "anonymous" => Some(("anonymous".to_string(), "".to_string())),
                    "steam account" => {
                        let username = Text::new("Please enter your steam username:").prompt()?;
                        let password =
                            Password::new("Please enter your password for your steam account.")
                                .without_confirmation()
                                .prompt()?;
                        Some((username, password))
                    }
                    _ => None,
                }
            }
        };
        Ok(login.unwrap())
    }

    /// Get the app update
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID of the game server
    ///
    /// # Returns
    ///
    /// The app update
    ///
    /// # Errors
    ///
    /// If the app update could not be found
    fn get_app_update(app_id: Option<u32>) -> Result<u32, Box<dyn std::error::Error>> {
        let app_update = match app_id {
            Some(app_id) => {
                let app_name = Self::check_app_id(app_id)?;
                let confirm = Confirm::new(&format!(
                    "Would you like to install the server for {}?",
                    app_name
                ))
                .prompt()?;
                if confirm {
                    Some(app_id)
                } else {
                    let app_id =
                        Text::new("Please enter the Steam App ID of the game server.").prompt()?;
                    app_id.parse::<u32>().ok()
                }
            }
            None => {
                let app_id =
                    Text::new("Please enter the Steam App ID of the game server.").prompt()?;
                let app_name = Self::check_app_id(app_id.parse::<u32>()?)?;
                let confirm = Confirm::new(&format!(
                    "Would you like to install the server for {}?",
                    app_name
                ))
                .prompt()?;
                if confirm {
                    Some(app_id.parse::<u32>()?)
                } else {
                    let app_id =
                        Text::new("Please enter the Steam App ID of the game server.").prompt()?;
                    Some(app_id.parse::<u32>()?)
                }
            }
        };

        Ok(app_update.unwrap())
    }

    /// Detect the platforms for a game server
    ///
    /// # Arguments
    ///
    /// - `config` - The config
    /// - `app_id` - The app id of the server
    ///
    /// # Returns
    ///
    /// The detected platforms
    ///
    /// # Errors
    ///
    /// If the platforms could not be detected
    fn ceck_platform(
        config: Config,
        app_id: u32,
        login: Option<(String, String)>,
    ) -> Result<Vec<Platform>, Box<dyn std::error::Error>> {
        let mut platforms = Vec::new();

        let linux = Self::execute_status_command(app_id, login.clone(), "linux", config.clone())?;

        let windows = Self::execute_status_command(app_id, login, "windows", config)?;

        let linux_output = String::from_utf8_lossy(&linux.stdout);
        let windows_output = String::from_utf8_lossy(&windows.stdout);

        if !linux_output.contains("Invalid Platform") && !linux_output.contains("unknown") {
            platforms.push(Platform::Linux);
        }

        if !windows_output.contains("Invalid Platform") && !windows_output.contains("unknown") {
            platforms.push(Platform::Windows);
        }

        Ok(platforms)
    }

    fn execute_status_command(
        app_id: u32,
        login: Option<(String, String)>,
        platform: &str,
        config: Config,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        let command = std::process::Command::new(&config.steamcmd_path)
            .arg("+sSteamCmdForcePlatformType")
            .arg(platform)
            .arg("+login")
            .arg(login.clone().unwrap().0)
            .arg(login.clone().unwrap().1)
            .arg("+app_status")
            .arg(app_id.to_string())
            .arg("+quit")
            .output();

        Ok(command?)
    }
}
