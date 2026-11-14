use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{State, WebSocketUpgrade},
    http::HeaderMap,
    response::{Html, IntoResponse, Response}
};

use crate::graphql::AppState;

pub(super) async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish()
    )
}

pub(super) async fn graphql_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
    req: GraphQLRequest
) -> GraphQLResponse {
    let req = req.into_inner();
    state.schema.execute(req).await.into()
}

pub(super) async fn graphql_ws_handler(
    State(schema): State<AppState>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |socket| {
            GraphQLWebSocket::new(socket, schema.schema.clone(), protocol).serve()
        })
}
