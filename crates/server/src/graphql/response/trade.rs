use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use model::{Side, Trade};
use rust_decimal::Decimal;

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum TradeSide {
    #[graphql(name = "buy")]
    BUY,
    #[graphql(name = "sell")]
    SELL
}

impl From<Side> for TradeSide {
    fn from(value: Side) -> Self {
        match value {
            Side::BUY => TradeSide::BUY,
            Side::SELL => TradeSide::SELL
        }
    }
}

#[derive(SimpleObject)]
pub struct TradeData {
    pub trade_time: DateTime<Utc>,
    pub symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: TradeSide,
    pub exchange: String,
    pub event_time: DateTime<Utc>
}

impl From<Trade> for TradeData {
    fn from(value: Trade) -> Self {
        Self {
            trade_time: value.trade_time,
            symbol: value.symbol,
            price: value.price,
            quantity: value.quantity,
            side: TradeSide::from(value.side),
            exchange: value.exchange.to_string(),
            event_time: value.event_time
        }
    }
}
