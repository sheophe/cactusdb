pub mod errors;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::{fs, net::SocketAddr};
use toml;

#[derive(ValueEnum, Clone, Copy, Debug, Deserialize)]
pub enum ConfigFormat {
    Toml,
    Yaml,
    Json,
}

#[derive(ValueEnum, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogFormat {
    Text,
    Json,
    Yaml,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogFile {
    #[serde(default = "LogFile::default_filename")]
    pub filename: String,

    #[serde(default = "LogFile::default_max_size")]
    pub max_size: usize,

    #[serde(default)]
    pub max_days: u64,

    #[serde(default)]
    pub max_backups: u64,
}

impl LogFile {
    pub fn default_filename() -> String {
        "stderr".to_string()
    }

    pub fn default_max_size() -> usize {
        4 * 1024 * 1024
    }
}

impl Default for LogFile {
    fn default() -> Self {
        Self {
            filename: Self::default_filename(),
            max_size: Self::default_max_size(),
            max_days: 0,
            max_backups: 0,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QuotaConfig {
    #[serde(default)]
    pub foreground_cpu_time: u64,

    #[serde(default)]
    pub background_cpu_time: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {
    #[serde(default)]
    pub level: LogLevel,

    #[serde(default)]
    pub format: LogFormat,

    #[serde(default)]
    pub enable_timestamp: bool,

    #[serde(default)]
    pub file: LogFile,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    #[serde(default = "ServerConfig::default_addr")]
    pub addr: SocketAddr,

    #[serde(default)]
    pub cluster_addr: Option<SocketAddr>,
}

impl ServerConfig {
    pub fn default_addr() -> SocketAddr {
        "127.0.0.1:1996".parse().unwrap()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: Self::default_addr(),
            cluster_addr: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StorageConfig {
    #[serde(default = "StorageConfig::default_data_dir")]
    pub data_dir: String,
}

impl StorageConfig {
    pub fn default_data_dir() -> String {
        "/usr/local/var/cactusdb".to_string()
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: Self::default_data_dir(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CactusDbConfig {
    #[serde(default)]
    pub quota: QuotaConfig,

    #[serde(default)]
    pub log: LogConfig,

    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub storage: StorageConfig,
}

impl CactusDbConfig {
    pub fn new(path: &str, format: ConfigFormat) -> Result<Self, errors::ConfigError> {
        let data = fs::read_to_string(path)?;
        let data_str = data.as_str();
        Ok(match format {
            ConfigFormat::Toml => toml::from_str(data_str)?,
            ConfigFormat::Yaml => serde_yaml::from_str(data_str)?,
            ConfigFormat::Json => serde_json::from_str(data_str)?,
        })
    }

    pub fn to_formatted_string(&self, format: ConfigFormat) -> Result<String, errors::ConfigError> {
        Ok(match format {
            ConfigFormat::Toml => toml::to_string(self)?,
            ConfigFormat::Yaml => serde_yaml::to_string(self)?,
            ConfigFormat::Json => serde_json::to_string(self)?,
        })
    }
}
