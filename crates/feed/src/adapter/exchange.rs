use model::MarketEvent;

///
pub trait ExchangeAdapter: Send + 'static {
    fn url(&self) -> String;
    /// Some exchanges need a subscribe frame after connecting; Binance combined streams don't.
    fn subscribe_msg(&self) -> Option<String> {
        None
    }
    /// Ok(None) = valid but irrelevant frame (acks, heartbeats).
    fn parse(&self, raw: &[u8]) -> anyhow::Result<Option<MarketEvent>>;
}
