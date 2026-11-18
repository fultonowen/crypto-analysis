use model::MarketEvent;
use tokio::sync::mpsc;

use crate::{adapter::exchange::ExchangeAdapter, client::websocket::WsTransport, sink::EventSink};

pub mod adapter;
pub mod client;
pub mod config;
pub mod sink;

/// pipeline glue to set up a WebSocket feed to a trading venue and process messages from it.
/// ['ExchangeAdapter'] allows for trading venue connections to be swapped.
/// ['EventSink'] allows for writing trade events to any backend.
pub async fn run<A: ExchangeAdapter, S: EventSink>(adapter: A, sink: S) -> anyhow::Result<()> {
    let (event_sender, event_receiver) = mpsc::channel::<MarketEvent>(8192);

    let transport = WsTransport::new(adapter);
    let ws = tokio::spawn(async move { transport.run(event_sender).await });
    let store = tokio::spawn(async move { sink::run(event_receiver, sink).await });

    let _ = tokio::join!(ws, store);
    Ok(())
}
