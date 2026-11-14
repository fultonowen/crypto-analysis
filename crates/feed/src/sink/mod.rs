pub mod pipeline;

use model::MarketEvent;
use tracing::instrument;

pub struct StdoutSink; // swap for TimescaleSink, KafkaSink, ...

pub trait EventSink: Send + 'static {
    fn write_batch(
        &mut self,
        batch: &[MarketEvent]
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl EventSink for StdoutSink {
    #[instrument(skip(self))]
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        println!("batch = {:#?}", batch);
        println!("flushed {} events", batch.len());
        Ok(())
    }
}
