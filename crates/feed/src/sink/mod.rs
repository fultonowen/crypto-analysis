pub mod pipeline;

use model::MarketEvent;
use crate::adapter::sink::EventSink;

pub struct StdoutSink; // swap for TimescaleSink, KafkaSink, ...

impl EventSink for StdoutSink {
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        println!("batch = {:#?}", batch);
        println!("flushed {} events", batch.len());
        Ok(())
    }
}
