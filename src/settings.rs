use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, EnumIter, ValueEnum, Display)]
pub enum Setting {
    User,
    Battery,
    OS,
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

#[derive(Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
struct Settings {
    setting_values: BTreeMap<String, bool>,
}

impl Settings {
    fn new(choices: Vec<bool>) -> Self {
        let mut config: BTreeMap<String, bool> = BTreeMap::new();
        for (setting, value) in Setting::iter().zip(choices.into_iter()) {
            config.insert(setting.to_string(), value);
        }
        Settings {setting_values: config}
    }

}

pub fn load_settings() {

}

pub fn edit_settings() {

}

pub fn print_settings() {

}