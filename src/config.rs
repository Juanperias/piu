use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tokio::fs::read_to_string;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Toml Error {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host_config: HostConfig,
    pub bind: HashMap<String, BindConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct BindConfig {
    pub proxy_pass: String,
}

pub async fn read_config(path: String) -> Result<ServerConfig, ConfigError> {
    let content = read_to_string(path).await?;
    let config: ServerConfig = toml::from_str(&content)?;

    Ok(config)
}
