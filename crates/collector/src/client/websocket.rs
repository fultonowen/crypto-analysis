use anyhow::anyhow;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::client::error::WebSocketError;

pub struct WsTransport {
    url: String,
    subscribe: Option<String>
}

impl WsTransport {
    pub fn new(url: String, subscribe: Option<String>) -> Self {
        Self { url, subscribe }
    }

    /// Reconnects forever with capped backoff; returns only if the receiver is dropped.
    //todo: Owen need to fix the backoff + retries logic
    pub async fn run(&self, tx: mpsc::Sender<Bytes>) {
        let mut retries = 0usize;
        let mut backoff = std::time::Duration::from_millis(250);
        loop {
            match self.connect_once(&tx).await {
                Ok(()) => return, // consumer gone
                Err(e) => tracing::warn!(error = %e, "ws dropped, reconnecting")
            }
            tokio::time::sleep(backoff).await;
            backoff = (backoff * 2).min(std::time::Duration::from_secs(10));
        }
    }

    async fn connect_once(&self, tx: &mpsc::Sender<Bytes>) -> Result<(), WebSocketError> {
        let (ws, _) = connect_async(&self.url).await?;
        println!("WebSocket connection established");
        let (mut write, mut read) = ws.split();

        if let Some(sub) = &self.subscribe {
            write.send(Message::text(sub.clone())).await?;
        }

        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(t) => {
                    println!("Message Received WS: {t:?}");
                    tracing::debug!("{:?}", t);

                    if tx.send(Bytes::copy_from_slice(t.as_bytes())).await.is_err() {
                        return Ok(());
                    }
                }
                Message::Binary(b) => {
                    if tx.send(b).await.is_err() {
                        return Ok(());
                    }
                }
                Message::Ping(p) => {
                    tracing::debug!("Received ping message from websocket server");
                    write.send(Message::Pong(p)).await?
                },
                Message::Close(_) => {
                    return Err(WebSocketError::UnexpectedError(anyhow!(
                        "Websocket unexpectedly closed"
                    )));
                }
                _ => {}
            }
        }
        Err(WebSocketError::UnexpectedError(anyhow!(
            "WebSocket stream ended"
        )))
    }
}
