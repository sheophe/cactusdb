pub mod errors;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::{
    fs,
    net::{AddrParseError, SocketAddr},
    str::FromStr,
};
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
pub struct MaxSize(pub usize);

impl Default for MaxSize {
    fn default() -> Self {
        Self(4096 * 1024 * 1024 * 1024)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogFile {
    #[serde(default)]
    pub filename: String,

    #[serde(default)]
    pub max_size: MaxSize,

    #[serde(default)]
    pub max_days: u64,

    #[serde(default)]
    pub max_backups: u64,
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
pub struct Addr(pub SocketAddr);

impl FromStr for Addr {
    type Err = AddrParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr = SocketAddr::from_str(s)?;
        Ok(Self(addr))
    }
}

impl Default for Addr {
    fn default() -> Self {
        Self("127.0.0.1:1996".parse().unwrap())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    #[serde(default)]
    pub addr: Addr,

    #[serde(default)]
    pub cluster_addr: Option<SocketAddr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataDir(String);

impl Default for DataDir {
    fn default() -> Self {
        Self("/var/lib/cactusdb/data".to_string())
    }
}

impl ToString for DataDir {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StorageConfig {
    #[serde(default)]
    pub data_dir: DataDir,
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
