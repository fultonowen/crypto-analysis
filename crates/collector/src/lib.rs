use std::time::Duration;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::time::interval;
use model::MarketEvent;
use crate::adapter::exchange::ExchangeAdapter;
use crate::client::websocket::WsTransport;
use crate::sink::EventSink;

pub mod adapter;
pub mod client;
pub mod config;
pub mod sink;


pub async fn run<A: ExchangeAdapter, S: EventSink>(adapter: A, mut sink: S) -> anyhow::Result<()> {
    let (raw_tx, mut raw_rx) = mpsc::channel::<Bytes>(8192);
    let (ev_tx, mut ev_rx) = mpsc::channel::<MarketEvent>(8192);

    let transport = WsTransport::new(adapter.url(), adapter.subscribe_msg());
    let ws = tokio::spawn(async move { transport.run(raw_tx).await });

    // todo: Owen we probably will want to take this task out of here for continuity sake
    // we do not need a task parsing such small messages with the amount of messages we have
    let parse = tokio::spawn(async move {
        while let Some(raw) = raw_rx.recv().await {
            match adapter.parse(&raw) {
                Ok(Some(ev)) => {
                    if ev_tx.send(ev).await.is_err() {
                        break;
                    }
                }
                Ok(None) => {}
                Err(e) => tracing::warn!(error = %e, "parse failed")
            }
        }
    });

    let store = tokio::spawn(async move {
        let mut buf = Vec::with_capacity(512);
        let mut tick = interval(Duration::from_millis(100));
        loop {
            tokio::select! {
                ev = ev_rx.recv() => match ev {
                    Some(e) => { buf.push(e); if buf.len() < 512 { continue } }
                    None => { let _ = sink.write_batch(&buf).await; break }
                },
                _ = tick.tick() => {}
            }
            if !buf.is_empty() {
                if let Err(e) = sink.write_batch(&buf).await {
                    tracing::error!(error = %e);
                }
                buf.clear();
            }
        }
    });

    let _ = tokio::join!(ws, parse, store);
    Ok(())
}