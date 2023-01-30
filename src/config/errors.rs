use serde::Deserialize;
use std::{fmt, io};
use thiserror::Error;
use toml;

#[derive(Error, Deserialize)]
pub enum ConfigError {
    #[error("i/o error: {0}")]
    IoError(String),

    #[error("format error: {0}")]
    FormatError(String),

    #[error("address error: {0}")]
    AddressError(String),
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        Self::IoError(format!("{}", error).to_lowercase())
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::FormatError(format!("{}", error).to_lowercase())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        Self::FormatError(format!("{}", error).to_lowercase())
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(error: toml::ser::Error) -> Self {
        Self::FormatError(format!("{}", error).to_lowercase())
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        Self::FormatError(format!("{}", error).to_lowercase())
    }
}

impl From<std::net::AddrParseError> for ConfigError {
    fn from(error: std::net::AddrParseError) -> Self {
        Self::AddressError(format!("{}", error).to_lowercase())
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
