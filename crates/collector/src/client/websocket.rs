use anyhow::anyhow;
use futures_util::{SinkExt, StreamExt};
use model::MarketEvent;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{adapter::exchange::ExchangeAdapter, client::error::WebSocketError};

pub struct WsTransport<T: ExchangeAdapter> {
    adapter: T
}

impl<T> WsTransport<T>
where
    T: ExchangeAdapter
{
    pub fn new(adapter: T) -> Self {
        Self { adapter }
    }

    /// Reconnects for 'connection_attempts' with capped backoff; returns only if the receiver is
    /// dropped.
    pub async fn run(&self, tx: mpsc::Sender<MarketEvent>) {
        let mut retries = 0u32;
        let mut backoff = std::time::Duration::from_millis(250);
        loop {
            match self.connect_once(&tx).await {
                Ok(()) => return, // this branch indicates that the tasks have all joined
                Err(e) if retries < self.adapter.connection_attempts() => {
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2u32.pow(retries)).min(std::time::Duration::from_secs(10));
                    retries += 1;
                    tracing::warn!(error = %e, "ws dropped, reconnecting")
                }
                Err(e) => {
                    tracing::error!(error = %e, "ws dropped");
                    return;
                }
            }
        }
    }

    async fn connect_once(&self, tx: &mpsc::Sender<MarketEvent>) -> Result<(), WebSocketError> {
        let (ws, _) = connect_async(&self.adapter.url()).await?;
        tracing::debug!("websocket connection established");
        let (mut write, mut read) = ws.split();

        if let Some(sub) = &self.adapter.subscribe_msg() {
            write.send(Message::text(sub.clone())).await?;
        }

        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(t) => {
                    tracing::debug!("websocket received {:?}", t);
                    let _ = self.handle_msg(tx, t.as_bytes()).await; // todo: exit only if this fails
                }
                Message::Binary(b) => {
                    tracing::debug!("received bytes");
                    let _ = self.handle_msg(tx, &b).await; // todo: exit only if this fails
                }
                Message::Ping(p) => {
                    tracing::debug!("received ping message from websocket server");
                    write.send(Message::Pong(p)).await?
                }
                Message::Close(reason) => {
                    tracing::error!(error = ?reason, "websocket closed");
                    return Err(WebSocketError::UnexpectedError(anyhow!(
                        "websocket unexpectedly closed"
                    )));
                }
                _ => {}
            }
        }
        Err(WebSocketError::UnexpectedError(anyhow!(
            "websocket stream ended"
        )))
    }

    async fn handle_msg(
        &self,
        tx: &mpsc::Sender<MarketEvent>,
        bytes: &[u8]
    ) -> Result<(), WebSocketError> {
        match self.adapter.parse(bytes) {
            Ok(Some(event)) => {
                if let Err(_) = tx.send(event).await {
                    return Ok(()); // todo: fix this, the stream
                }
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => {
                tracing::warn!(error = %e, "websocket parse failed, ignoring");
                Ok(())
            }
        }
    }
}
