use framework::database::Database;
use framework::observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    let otel_guard = observability::configure()?;
    let postgres : Database = todo!();
    let sqlite = Sqlite::new(&config.database_url).await?;
    let prometheus = Prometheus::new();
    let blog_service = Service::new(sqlite, prometheus, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(blog_service, server_config).await?;
    http_server.run().await
}