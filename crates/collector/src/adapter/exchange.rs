use model::MarketEvent;

/// ['ExchangeAdapter'] allows for implementing data feeds across multiple
/// different exchanges.
pub trait ExchangeAdapter: Send + Sync + 'static {
    fn url(&self) -> String;
    fn connection_attempts(&self) -> u32;
    /// Some exchanges need a subscribe frame after connecting
    fn subscribe_msg(&self) -> Option<String> {
        None
    }
    /// Parse raw binary message from the exchange into ['MarketEvent'].
    /// Deserialization may fail and so
    /// Ok(None) = valid but irrelevant frame (acks, heartbeats).
    fn parse(&self, raw: &[u8]) -> anyhow::Result<Option<MarketEvent>>;
}
