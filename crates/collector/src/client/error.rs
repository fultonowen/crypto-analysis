use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Websocket returned error: {0}")]
    ClientError(#[from] tungstenite::Error),
    #[error("Unexpected websocket error: {0}")]
    UnexpectedError(#[from] anyhow::Error)
}
