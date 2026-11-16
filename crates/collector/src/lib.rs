use model::MarketEvent;
use tokio::sync::mpsc;

use crate::{adapter::exchange::ExchangeAdapter, client::websocket::WsTransport, sink::EventSink};

pub mod adapter;
pub mod client;
pub mod config;
pub mod sink;

pub async fn run<A: ExchangeAdapter, S: EventSink>(adapter: A, sink: S) -> anyhow::Result<()> {
    let (event_sender, event_receiver) = mpsc::channel::<MarketEvent>(8192);

    let transport = WsTransport::new(adapter);
    let ws = tokio::spawn(async move { transport.run(event_sender).await });

    // todo: why are we maintaining a buffer when mpsc already maintains a message queue
    // let store = tokio::spawn(async move {
    //     let mut buf = Vec::with_capacity(512);
    //     let mut tick = interval(Duration::from_millis(100));
    //     loop {
    //         tokio::select! {
    //             ev = event_receiver.recv() => match ev {
    //                 Some(e) => { buf.push(e); if buf.len() < 512 { continue } }
    //                 None => { let _ = sink.write_batch(&buf).await; break }
    //             },
    //             _ = tick.tick() => {}
    //         }
    //         if !buf.is_empty() {
    //             if let Err(e) = sink.write_batch(&buf).await {
    //                 tracing::error!(error = %e);
    //             }
    //             buf.clear();
    //         }
    //     }
    // });

    let store = tokio::spawn(async move { sink::run(event_receiver, sink).await });

    let _ = tokio::join!(ws, store);
    Ok(())
}
