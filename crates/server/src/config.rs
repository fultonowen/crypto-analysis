use framework::{config::EnvConfig, database::DatabaseConfig};
use serde::Deserialize;

use crate::graphql::HttpServerConfig;

/// Base config for the `Server` application
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub http: HttpServerConfig,
    pub database: DatabaseConfig
}

impl EnvConfig for ServerConfig {}
