use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub graphql_endpoint: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let graphql_endpoint = std::env::var("APP_GRAPHQL_ENDPOINT")
            .context("Environment variable `APP_GRAPHQL_ENDPOINT` missing")?;
        Ok(Self { graphql_endpoint })
    }
}