pub mod query;
pub mod handlers;
pub mod subscription;
pub mod schema;

use anyhow::Context;
use async_graphql::{EmptyMutation, Schema};
use axum::routing::{get, post};
use serde::Deserialize;
use tokio::net;
use framework::database::Database;
use crate::graphql::handlers::graphql_ws_handler;
use crate::graphql::query::QueryRoot;
use crate::graphql::subscription::SubscriptionRoot;

/// Configuration for the HTTP server.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpServerConfig {
    pub port: String,
}

#[derive(Clone)]
/// The global application state shared between all request handlers.
pub(super) struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, SubscriptionRoot>,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        database: Database,
        config: &HttpServerConfig,
    ) -> anyhow::Result<Self> {
        let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
            .data(database)
            .finish();

        let state = AppState { schema };
        let router = axum::Router::new()
            .nest("/api", graphql_routes())
            // .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("Server listening on {}", self.listener.local_addr()?);
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn graphql_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(handlers::graphql_playground).post(handlers::graphql_handler))
        .route("/graphql", post(handlers::graphql_handler))
        .route("/ws", get(graphql_ws_handler))
}