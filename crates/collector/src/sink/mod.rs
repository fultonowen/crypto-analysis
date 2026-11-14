pub mod timescaledb;
pub mod stdout;

use model::MarketEvent;


/// ['EventSink'] provides an interface for writing ['MarketEvent']s to a provided data sink output
#[async_trait::async_trait]
pub trait EventSink: Send + 'static {
    async fn write_batch(
        &mut self,
        batch: &[MarketEvent]
    ) -> anyhow::Result<()>;
}
