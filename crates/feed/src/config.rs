use serde::Deserialize;
use framework::config::EnvConfig;
use framework::database::DatabaseConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub binance: FeedConfig,
    pub database: DatabaseConfig,
}

impl EnvConfig for AppConfig {}

#[derive(Debug, Clone, Deserialize)]
pub struct FeedConfig {
    pub ws_url: String,
    pub spot_ids: String,
    // todo:
}

