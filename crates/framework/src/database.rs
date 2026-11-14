use std::ops::Deref;

use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    ConnectionError(#[from] sqlx::Error)
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32
}

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: PgPool
}

impl Database {
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, DbError> {
        let conn = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.url)
            .await?;
        Ok(Self { pool: conn.into() })
    }
}

impl Deref for Database {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
