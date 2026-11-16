use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "market_data.trade_side")]
pub enum Side {
    BUY,
    SELL
}

#[derive(Debug)]
pub struct Trade {
    pub exchange: &'static str,
    pub symbol: String,
    pub trade_id: i64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub trade_time_ms: i64,
    pub event_time_ms: i64,
    pub side: Side
}

#[derive(Debug)]
pub struct TickerPrice {
    pub event_time: i64,
    pub exchange: &'static str,
    pub symbol: String,
    pub last_price: Decimal
}

#[derive(Debug)]
pub enum MarketEvent {
    Price(TickerPrice),
    Trade(Trade)
}

// todo: remove?
// impl MarketEvent {
//     pub fn is_trade(&self) -> bool {
//         match &self {
//             MarketEvent::Trade(_) => true,
//             _ => false
//         }
//     }
//
//     pub fn is_price(&self) -> bool {
//         match &self {
//             MarketEvent::Price(_) => true,
//             _ => false
//         }
//     }
// }
