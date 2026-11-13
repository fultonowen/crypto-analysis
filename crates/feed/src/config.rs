use serde::Deserialize;
use framework::config::EnvConfig;
use framework::database::DatabaseConfig;
use crate::adapter::config::FeedConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub binance: FeedConfig,
    pub database: DatabaseConfig,
}

impl EnvConfig for AppConfig {}

