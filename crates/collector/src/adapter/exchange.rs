use model::MarketEvent;

/// ['ExchangeAdapter'] allows for implementing data feeds across multiple
/// different exchanges.
pub trait ExchangeAdapter: Send + 'static {
    fn url(&self) -> String;
    /// Some exchanges need a subscribe frame after connecting
    fn subscribe_msg(&self) -> Option<String> {
        None
    }
    /// Ok(None) = valid but irrelevant frame (acks, heartbeats).
    fn parse(&self, raw: &[u8]) -> anyhow::Result<Option<MarketEvent>>;
}
