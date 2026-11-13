use framework::config::EnvConfig;
use framework::database::Database;
use framework::observability;
use server::config::ServerConfig;
use server::graphql::HttpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _otel_guard = observability::configure()?;
    let config = ServerConfig::from_env()?;
    let pg_conn = Database::connect(&config.database).await?;
    let http_server = HttpServer::new(pg_conn, &config.http).await?;
    http_server.run().await
}