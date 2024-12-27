use std::path::PathBuf;

use inquire::{Confirm, Password, Select, Text};

use crate::utils::{run_with_output, Config, InstalledServer, ServerCache};

pub struct SteamCMD {
    pub login: Option<(String, String)>,
    pub force_install_dir: Option<String>,
    pub app_update: Option<u32>,
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

        let force_install_dir = match server_name {
            Some(server_name) => {
                let confirm = Confirm::new(&format!(
                    "Would you like to install the server with this name {}?",
                    server_name
                ))
                .prompt()?;
                if confirm {
                    server_name
                } else {
                    Text::new("Please enter the name of the game server:")
                        .with_placeholder("e.g. TestServer")
                        .with_help_message("It's the name for your game server folder.")
                        .prompt()?
                }
            }
            None => Text::new("Please enter the name of the game server:")
                .with_placeholder("e.g. TestServer")
                .with_help_message("It's the name for your game server folder.")
                .prompt()?,
        };

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
                    Some(app_id.parse::<u32>()?)
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

        let server_name = match app_id {
            Some(app_id) => Self::check_app_id(app_id)?,
            None => "".to_string(),
        };

        let install_path = PathBuf::from(force_install_dir.clone());

        let steamcmd = SteamCMD {
            login,
            force_install_dir: Some(force_install_dir),
            app_update,
        };

        let auto_update = Confirm::new("Would you like to enable auto updates?").prompt()?;

        Self::execute_install_command(steamcmd, config.steamcmd_path.clone())?;

        let server = InstalledServer {
            app_id: app_id.unwrap(),
            name: server_name,
            install_path,
            install_date: chrono::Local::now().to_utc(),
            last_updated: chrono::Local::now().to_utc(),
            auto_update,
            port: None,
        };

        config.installed_servers.push(server);

        config.save()?;

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
                steamcmd.force_install_dir.clone().unwrap()
            ))
            .arg(format!(
                "+login {} {}",
                steamcmd.login.clone().unwrap().0,
                steamcmd.login.unwrap().1,
            ))
            .arg(format!(
                "+app_update {} validate",
                steamcmd.app_update.unwrap()
            ))
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
}
