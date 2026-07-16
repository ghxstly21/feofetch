use std::fmt::{Display, Formatter};
use crate::settings::ConfigError;

#[derive(Debug)]
pub enum FeoError {
    ConfigError(crate::settings::ConfigError),
}

impl Display for FeoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            FeoError::ConfigError(ce) => ce.fmt(f)
        }
    }
}

impl From<ConfigError> for FeoError {
    fn from(value: ConfigError) -> Self {
        FeoError::ConfigError(value)
    }
}