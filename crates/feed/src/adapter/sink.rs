use model::MarketEvent;

pub trait EventSink: Send + 'static {
    fn write_batch(&mut self, batch: &[MarketEvent]) -> impl Future<Output = anyhow::Result<()>> + Send;
}