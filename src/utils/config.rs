use chrono::{DateTime, Utc};
use inquire::{Confirm, Text};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::utils::{Progress, ProgressStyle};

use super::{run_with_output, run_with_spinner};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub steamcmd_path: PathBuf,
    pub install_path: PathBuf,
    pub last_cache_update: Option<DateTime<Utc>>,
    pub installed_servers: Vec<InstalledServer>,
    pub is_initialized: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstalledServer {
    pub app_id: u32,
    pub name: String,
    pub install_path: PathBuf,
    pub install_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub auto_update: bool,
    pub port: Option<u16>,
    pub login_type: LoginType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoginType {
    SteamAccount,
    Anonymous,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCache {
    pub servers: Vec<ServerInfo>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub app_id: u32,
    pub name: String,
}

impl Config {
    /// Get the path to the config file
    ///
    /// # Returns
    ///
    /// The path to the config file
    ///
    /// # Errors
    ///
    /// If the config directory could not be found or if the path could not be created
    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir().ok_or("Could not find config directory")?;
        Ok(config_path.join("karnes-development/steamserv/config.toml"))
    }

    /// Load the config from the config file
    ///
    /// # Returns
    ///
    /// The loaded config
    ///
    /// # Errors
    ///
    /// If the config file could not be found or if the file could not be read
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_config_path()?;

        if !path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    /// Save the config to the config file
    ///
    /// # Errors
    ///
    /// If the config file could not be written
    ///
    /// # Returns
    ///
    /// Ok if the config was saved successfully
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path()?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Initialize the config
    ///
    /// # Errors
    ///
    /// If the config could not be saved
    ///
    /// # Returns
    ///
    /// Ok if the config was saved successfully
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("You using steamserv the first time. The follow steps configure your enviroment.");
        let select_steamcmd = Confirm::new("Do you have SteamCMD installed?")
            .with_default(false)
            .prompt()?;
        let steamcmd_path = if select_steamcmd {
            Text::new("Please enter the path to the SteamCMD executable:")
                .with_help_message("This is the path to your SteamCMD executable")
                .prompt()?
        } else {
            let path = self.install_steamcmd().await?;
            if path.is_empty() {
                return Err("SteamCMD is required to use steamserv".into());
            }
            path
        };
        let install_path = Text::new("Please enter the path to the server install directory:")
            .with_help_message("This is the path to installing the servers.")
            .with_placeholder("e.g. /home/user/servers")
            .prompt()?;

        let config = Config {
            steamcmd_path: PathBuf::from(steamcmd_path),
            install_path: PathBuf::from(install_path),
            last_cache_update: None,
            installed_servers: Vec::new(),
            is_initialized: true,
        };

        config.save()?;

        Ok(())
    }

    /// Install SteamCMD
    ///
    /// # Returns
    ///
    /// The path to the SteamCMD executable
    ///
    /// # Errors
    ///
    /// If the SteamCMD could not be installed
    async fn install_steamcmd(&self) -> Result<String, Box<dyn std::error::Error>> {
        let install_path = Text::new("Please enter the path to the SteamCMD install directory:")
            .with_help_message("This is the path you want to install SteamCMD.")
            .with_placeholder("e.g. /home/user/steamcmd")
            .prompt()?;
        let confirm = Confirm::new("Do you want to install SteamCMD now?")
            .with_default(true)
            .prompt()?;

        if confirm {
            let steamcmd_url =
                "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
            let steamcmd_path = PathBuf::from(install_path);
            let steamcmd_file = steamcmd_path.join("steamcmd_linux.tar.gz");

            std::fs::create_dir_all(&steamcmd_path)?;

            let mut progress = Progress::new(100, "Downloading SteamCMD", ProgressStyle::Bar)?;
            let mut response = reqwest::get(steamcmd_url).await?;
            let total_size = response.content_length().unwrap_or(0) as usize;
            let mut downloaded = 0;

            let mut content = Vec::new();
            while let Some(chunk) = response.chunk().await? {
                downloaded += chunk.len();
                content.extend_from_slice(&chunk);
                progress.update((downloaded * 100) / total_size)?;
            }

            std::fs::write(&steamcmd_file, &content)?;
            progress.finish()?;

            let mut extract_child = std::process::Command::new("tar")
                .arg("-xzf")
                .arg(&steamcmd_file)
                .arg("-C")
                .arg(&steamcmd_path)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            run_with_spinner(&mut extract_child, "Extracting SteamCMD")?;

            let status = extract_child.wait()?;
            if !status.success() {
                return Err("Could not extract SteamCMD".into());
            }

            std::fs::remove_file(&steamcmd_file)?;

            let mut init_child = std::process::Command::new(steamcmd_path.join("steamcmd.sh"))
                .arg("+quit")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            run_with_output(&mut init_child)?;

            let init_status = init_child.wait()?;
            if !init_status.success() {
                return Err("Could not initialize SteamCMD".into());
            }

            Ok(steamcmd_path
                .join("steamcmd.sh")
                .to_string_lossy()
                .to_string())
        } else {
            Err("SteamCMD is required to use steamserv".into())
        }
    }
}

impl Default for Config {
    /// Create a default config
    fn default() -> Self {
        Self {
            steamcmd_path: PathBuf::from(""),
            install_path: PathBuf::from(""),
            last_cache_update: None,
            installed_servers: Vec::new(),
            is_initialized: false,
        }
    }
}

impl InstalledServer {
    /// Update the last updated timestamp
    ///
    /// # Returns
    ///
    /// The updated installed server
    pub fn update_timestamp(&mut self) {
        self.last_updated = chrono::Local::now().to_utc();
    }
}

impl ServerCache {
    /// Get the path to the server cache file
    ///
    /// # Returns
    ///
    /// The path to the server cache file
    ///
    /// # Errors
    ///
    /// If the cache directory could not be found or if the path could not be created
    fn get_cache_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let cache_path = dirs::config_dir().ok_or("Could not find cache directory")?;
        Ok(cache_path.join("karnes-development/steamserv/cache/server_cache.json"))
    }

    /// Load the server cache from the cache file
    ///
    /// # Returns
    ///
    /// The loaded server cache
    ///
    /// # Errors
    ///
    /// If the cache file could not be found or if the file could not be read
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_cache_path()?;

        if !path.exists() {
            return Ok(ServerCache::default());
        }

        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Save the server cache to the cache file
    ///
    /// # Errors
    ///
    /// If the cache file could not be written
    ///
    /// # Returns
    ///
    /// Ok if the cache was saved successfully
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_cache_path()?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        let content = serde_json::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Update the server cache
    ///
    /// # Returns
    ///
    /// Ok if the cache was updated successfully
    ///
    /// # Errors
    ///
    /// If the cache could not be updated
    pub async fn update_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut progress = Progress::new(100, "Updating server cache", ProgressStyle::Bar)?;
        let mut response =
            reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").await?;
        let total_size = response.content_length().unwrap_or(1) as usize;
        let mut downloaded = 0;

        let mut content = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            downloaded += chunk.len();
            content.extend_from_slice(&chunk);
            if total_size > 0 {
                let progress_value = ((downloaded * 100) / total_size).min(100);
                progress.update(progress_value)?;
            }
        }

        let app_list: serde_json::Value = serde_json::from_slice(&content)?;
        let apps = app_list["applist"]["apps"]
            .as_array()
            .ok_or("Invalid API response format")?;

        self.servers = apps
            .iter()
            .filter_map(|app| {
                let name = app["name"].as_str()?;
                let app_id = app["appid"].as_u64()?;

                if Self::is_game_server(name) {
                    Some(ServerInfo {
                        app_id: app_id as u32,
                        name: name.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();
        self.last_update = Utc::now();
        progress.finish()?;

        self.save()?;

        Ok(())
    }

    /// Check if a server is a game server
    ///
    /// # Arguments
    ///
    /// - `name` - The name of the server
    ///
    /// # Returns
    ///
    /// True if the server is a game server
    fn is_game_server(name: &str) -> bool {
        let name = name.to_lowercase();

        if name.contains("browser") || name.contains("emulator") {
            return false;
        }

        name.contains("dedicated server")
            || name.contains("server tool")
            || name.ends_with("server")
    }
}

impl Default for ServerCache {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            last_update: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_server() {
        let test_names = vec![
            "PalServer",
            "专用服务器",
            "Dedicated Server",
            "サーバー",
            "Server Browser",
            "Masterserver",
        ];

        let expected_results = vec![true, false, true, false, false, true];

        for (name, expected) in test_names.iter().zip(expected_results.iter()) {
            assert_eq!(ServerCache::is_game_server(name), *expected);
        }
    }
}
