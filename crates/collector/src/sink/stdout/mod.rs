use tracing::instrument;
use model::MarketEvent;
use crate::sink::EventSink;

/// Example [`EventSink`] implementation that processes batches of ['MarketEvent']'s and prints to stdout
pub struct StdoutSink; // swap for TimescaleSink, KafkaSink, ...

#[async_trait::async_trait]
impl EventSink for StdoutSink {
    #[instrument(skip(self))]
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        println!("batch = {:#?}", batch);
        println!("flushed {} events", batch.len());
        Ok(())
    }
}
