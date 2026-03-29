use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;
use toml;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub user: bool,
    pub battery: bool,
    pub os: bool,
    pub host: bool,
    pub kernel: bool,
    pub uptime: bool,
    pub packages: bool,
    pub shell: bool,
    pub resolution: bool,
    pub de: bool,
    pub wm: bool,
    pub wm_theme: bool,
    pub terminal: bool,
    pub font: bool,
    pub cpu: bool,
    pub gpu: bool,
    pub memory: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            user: true,
            battery: true,
            os: true,
            host: true,
            kernel: true,
            uptime: true,
            packages: true,
            shell: true,
            resolution: true,
            de: true,
            wm: true,
            wm_theme: true,
            terminal: true,
            font: true,
            cpu: true,
            gpu: true,
            memory: true,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    // converts settings into toml format
    pub fn to_toml(&self) -> Result<String, ConfigError>  {
        toml::to_string(self).map_err(|err| {
            ConfigError::new("could not write to your config file", err.to_string())
        })
    }
}

#[derive(Debug)]
pub struct ConfigError {
    message: &'static str,
    err: Option<String>
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.err {
            Some(e) => write!(f, "\nfeofetch config error: {}\nfull error: {}\n", self.message, e),
            None => write!(f, "\nfeofetch config error: {}\n", self.message)
        }
    }
}

impl Error for ConfigError {}

impl ConfigError {

    fn new(message: &'static str, err: impl Into<Option<String>>) -> Self {
        ConfigError {message, err: err.into()}
    }
    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn error(&self) -> &str {
        match &self.err {
            Some(e) => e.as_str(),
            None => ""
        }
    }
}



// if the user's config file already exists, returns Settings contained there
// if it doesn't exist, creates a new config file with default settings and returns a default Settings object
pub fn load_settings() -> Result<Settings, ConfigError> {
    let settings_path = match config_dir() {
        Some(path) => path.join("feofetch"),
        None => return Err(ConfigError::new("could not locate your computer's config directory", None))
    };

    if settings_path.join("config.toml").exists() {
        // read from the existing config file
        let config_content = fs::read_to_string(settings_path.join("config.toml")).map_err(|e|
            {
                ConfigError::new("failed to read your config file", e.to_string())
            })?;

            toml::from_str(&config_content).map_err(|e|
            {
                ConfigError::new("failed to load your settings", e.to_string())
            })
    } else {
        if !settings_path.exists() {
            // try to create the feofetch dir, if it failed, return Err()
            create_dir(&settings_path).map_err(|e|
                {
                    ConfigError::new("failed to create feofetch dir", e.to_string())
                })?;
        }
        let settings = Settings::new();

        let toml_str = toml::to_string(&settings).map_err(|e|
            {ConfigError::new("could not convert settings to toml format", e.to_string())}
        )?;

        fs::write(settings_path.join(settings_path.join("config.toml")), toml_str).map_err(|e|
            {ConfigError::new("failed to write default settings into your config file", e.to_string())}
        )?;

        Ok(settings)
    }
}

pub fn edit_settings() {
}


