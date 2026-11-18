use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub graphql_endpoint: String
}

impl AppConfig {
    pub fn from_env() -> Self {
        let graphql_endpoint = option_env!("APP_GRAPHQL_ENDPOINT")
            .unwrap_or("http://localhost:8080/api/graphql")
            .into();
        Self { graphql_endpoint }
    }
}
