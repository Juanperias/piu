use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Toml Error {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host_config: HostConfig
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostConfig {
    pub port: u16,
    pub host: String
}

pub async fn read_config(path: String) -> Result<ServerConfig, ConfigError> {
    let content = read_to_string(path).await?;
    let config: ServerConfig = toml::from_str(&content)?;

    Ok(config)
}
