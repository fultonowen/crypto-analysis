#[derive(Debug)]
pub enum Side { Buy, Sell }

#[derive(Debug)]
pub struct Trade {
    pub exchange: &'static str,
    pub symbol: String,
    pub trade_id: u64,
    pub price: f64, // consider fixed-point i64 for production
    pub qty: f64,
    pub ts_ms: u64,
    pub aggressor: Side,
}

#[derive(Debug)]
pub enum MarketEvent { Trade(Trade) }