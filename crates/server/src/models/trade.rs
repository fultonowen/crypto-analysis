use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Candle1Min {
    pub time_bucket: DateTime<Utc>,
    pub symbol: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal
}
