pub mod stdout;
pub mod timescaledb;

use model::MarketEvent;
use tokio::sync::mpsc;

/// ['EventSink'] provides an interface for writing ['MarketEvent']s to a provided data sink output
#[async_trait::async_trait]
pub trait EventSink: Send + 'static {
    async fn write_one(&self, event: MarketEvent) -> anyhow::Result<()>;
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()>;
}

// todo: with high throughput for MarketEvents, writing one event at a time is vastly inefficient
pub async fn run<S: EventSink>(mut event_rx: mpsc::Receiver<MarketEvent>, sink: S) {
    while let Some(event) = event_rx.recv().await {
        if let Err(e) = sink.write_one(event).await {
            tracing::error!(?e, "error while writing event");
        }
    }
}
