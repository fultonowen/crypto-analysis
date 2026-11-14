use framework::{config::EnvConfig, database::DatabaseConfig};
use serde::Deserialize;

use crate::adapter::config::FeedConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub binance: FeedConfig,
    pub database: DatabaseConfig
}

impl EnvConfig for AppConfig {}
