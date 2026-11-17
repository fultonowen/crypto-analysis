use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "market_data.trade_side")]
pub enum Side {
    BUY,
    SELL
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Trade {
    pub trade_time: DateTime<Utc>,
    pub symbol: String,
    pub trade_id: i64,
    pub side: Side,
    pub price: Decimal,
    pub quantity: Decimal,
    pub exchange: String,
    pub event_time: DateTime<Utc>
}

#[derive(Debug, sqlx::FromRow)]
pub struct TickerPrice {
    pub event_time: DateTime<Utc>,
    pub symbol: String,
    pub last_price: Decimal,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub w_avg_price: Decimal,
    pub exchange: String
}

#[derive(Debug)]
pub enum MarketEvent {
    Price(TickerPrice),
    Trade(Trade)
}
