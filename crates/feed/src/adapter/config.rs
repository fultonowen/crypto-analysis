use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FeedConfig {
    pub ws_url: String,
}
