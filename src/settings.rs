use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs::create_dir_all;
use toml;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use clap::ValueEnum;
use owo_colors::{OwoColorize, Stream::Stdout};

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

    pub fn get(&self, s: Setting) -> bool {
        match s {
            Setting::User => self.user,
            Setting::Battery => self.battery,
            Setting::Os => self.os,
            Setting::Host => self.host,
            Setting::Kernel => self.kernel,
            Setting::Uptime => self.uptime,
            Setting::Packages => self.packages,
            Setting::Shell => self.shell,
            Setting::Resolution => self.resolution,
            Setting::De => self.de,
            Setting::Wm => self.wm,
            Setting::WmTheme => self.wm_theme,
            Setting::Terminal => self.terminal,
            Setting::Font => self.font,
            Setting::Cpu => self.cpu,
            Setting::Gpu => self.gpu,
            Setting::Memory => self.memory
        }
    }

    pub fn set(&mut self, s: Setting, enabled: bool) {
        match s {
            Setting::User => self.user = enabled,
            Setting::Battery => self.battery = enabled,
            Setting::Os => self.os = enabled,
            Setting::Host => self.host = enabled,
            Setting::Kernel => self.kernel = enabled,
            Setting::Uptime => self.uptime = enabled,
            Setting::Packages => self.packages = enabled,
            Setting::Shell => self.shell = enabled,
            Setting::Resolution => self.resolution = enabled,
            Setting::De => self.de = enabled,
            Setting::Wm => self.wm = enabled,
            Setting::WmTheme => self.wm_theme = enabled,
            Setting::Terminal => self.terminal = enabled,
            Setting::Font => self.font = enabled,
            Setting::Cpu => self.cpu = enabled,
            Setting::Gpu => self.gpu = enabled,
            Setting::Memory => self.memory = enabled
        }
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (setting, enabled) in [
            ("User", self.user),
            ("Battery", self.battery),
            ("OS", self.os),
            ("Host", self.host),
            ("Kernel", self.kernel),
            ("Uptime", self.uptime),
            ("Packages", self.packages),
            ("Shell", self.shell),
            ("Resolution", self.resolution),
            ("DE", self.de),
            ("WM", self.wm),
            ("WM Theme", self.wm_theme),
            ("Terminal", self.terminal),
            ("Font", self.font),
            ("CPU", self.cpu),
            ("GPU", self.gpu),
            ("Memory", self.memory)
        ] {
            if enabled {
                writeln!(f, "{}", "show".if_supports_color(Stdout, |text| text.green()))?;
            } else {
                writeln!(f, "{}", "hide".if_supports_color(Stdout, |text| text.red()))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Setting {
    User,
    Battery,
    Os,
    Host,
    Kernel,
    Uptime,
    Packages,
    Shell,
    Resolution,
    De,
    Wm,
    WmTheme,
    Terminal,
    Font,
    Cpu,
    Gpu,
    Memory,
}




#[derive(Debug)]
pub struct ConfigError {
    message: &'static str,
    err: Option<String>
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nfeofetch config error: {}", self.message)?;
        if let Some(e) = &self.err {
            writeln!(f, "full error: {e}")?;
        }
        Ok(())
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
        create_dir_all(&settings_path).map_err(|e|
        ConfigError::new("failed to create feofetch dir", e.to_string()))?;
        let settings = Settings::new();

        let toml_str = toml::to_string(&settings).map_err(|e|
            {ConfigError::new("could not convert settings to toml format", e.to_string())}
        )?;

        fs::write(settings_path.join("config.toml"), toml_str).map_err(|e|
            {ConfigError::new("failed to write default settings into your config file", e.to_string())}
        )?;

        Ok(settings)
    }
}

pub fn edit_settings() {
}

pub fn print_settings() -> Result<(), ConfigError> {
    println!("{}", load_settings()?);
    Ok(())
}


