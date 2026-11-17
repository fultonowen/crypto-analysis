use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::models::trade::Candle1Min;

#[derive(SimpleObject)]
pub struct Candle1Minute {
    pub time_bucket: DateTime<Utc>,
    pub symbol: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal
}

impl From<Candle1Min> for Candle1Minute {
    fn from(value: Candle1Min) -> Self {
        Self {
            time_bucket: value.time_bucket,
            symbol: value.symbol,
            open: value.open,
            high: value.high,
            low: value.low,
            close: value.close,
            volume: value.volume
        }
    }
}
