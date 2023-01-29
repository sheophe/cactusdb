use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, io};

#[derive(Debug)]
pub enum ConfigError {
    IOError(String),
    InvalidFormat(String),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::InvalidFormat(err.to_string())
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConfigFormat {
    Json,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OctomqConfig {
    gay: String,
}

impl OctomqConfig {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let file = fs::File::open(path)?;
        let config: Self = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
