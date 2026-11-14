#[derive(Debug)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug)]
pub struct Trade {
    pub exchange: &'static str,
    pub symbol: String,
    pub trade_id: u64,
    pub price: f64, // consider fixed-point i64 for production
    pub qty: f64,
    pub ts_ms: u64,
    pub aggressor: Side
}

#[derive(Debug)]
pub struct TickerPrice {
    pub exchange: &'static str,
    pub symbol: String,
    pub last_price: f64,
    pub ts_ms: u64,
}

#[derive(Debug)]
pub enum MarketEvent {
    Price(TickerPrice),
    Trade(Trade)
}

impl MarketEvent {
    pub fn is_trade(&self) -> bool {
        match &self {
            MarketEvent::Trade(_) => true,
            _ => false
        }
    }

    pub fn is_price(&self) -> bool {
        match &self {
            MarketEvent::Price(_) => true,
            _ => false
        }
    }
}