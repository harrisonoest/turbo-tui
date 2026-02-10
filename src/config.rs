use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub queries: Option<QueriesConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QueriesConfig {
    pub directory: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub server: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default)]
    pub trusted_connection: bool,
    #[serde(default)]
    pub trust_server_certificate: bool,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
