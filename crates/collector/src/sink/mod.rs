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

// todo: tomorrow I was converting the lib code to a simple sink method that handles
// message processing
pub async fn run<S: EventSink>(mut event_rx: mpsc::Receiver<MarketEvent>, sink: S) {
    while let Some(event) = event_rx.recv().await {
        let _ = sink.write_one(event).await; // todo: ignoring result may lose data
    }
}
