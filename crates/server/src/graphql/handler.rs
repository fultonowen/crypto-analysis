use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::http::HeaderMap;
use crate::graphql::AppState;

async fn graphql_handler(
    State(schema): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest
) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}