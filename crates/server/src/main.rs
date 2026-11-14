use framework::{config::EnvConfig, database::Database, observability};
use server::{config::ServerConfig, graphql::HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _otel_guard = observability::configure()?;
    let config = ServerConfig::from_env()?;
    let pg_conn = Database::connect(&config.database).await?;
    let http_server = HttpServer::new(pg_conn, &config.http).await?;
    http_server.run().await
}
