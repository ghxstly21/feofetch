use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml;



#[derive(Serialize)]
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
    pub fn to_toml(&self) -> Result<String, String>  {
        toml::to_string(self).map_err(|e| {
            format!("feofetch error: could not write to your config file\nmessage: {e}")
        })
    }
}

pub fn load_settings() -> Result<Settings, &'static str> {
    let settings_path = match config_dir() {
        Some(path) => path.join("feofetch"),
        None => return Err("feofetch error: could not locate your computer's config directory")
    };

    if settings_path.exists() {

    } else {
        create_dir(&settings_path).map_err(|e| {format!("feofetch error")})
        match create_dir(&settings_path) {
            Err(_) => return Err("feofetch error: failed to create feofetch dir"),
            _ => {}
        }

        let settings = Settings::new();

        let toml_str = toml::to_string(&settings);


        // for (setting, value) in &settings {
        //     match fs::write(settings_path.join("config.toml"), "") {
        //         Err(_) => return Err("feofetch error: failed to write to your config file."),
        //         _ => {}
        //     }
        // }



        Ok(settings)



    }


}

pub fn edit_settings() {
}


