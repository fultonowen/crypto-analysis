use model::MarketEvent;
use tracing::instrument;

use crate::sink::EventSink;

/// Example [`EventSink`] implementation that processes batches of ['MarketEvent']'s and prints to
/// stdout
pub struct StdoutSink;

#[async_trait::async_trait]
impl EventSink for StdoutSink {
    async fn write_one(&self, event: MarketEvent) -> anyhow::Result<()> {
        println!("market event: {:?}", event);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        println!("flushed {} events, batch = {:#?}", batch.len(), batch);
        Ok(())
    }
}
