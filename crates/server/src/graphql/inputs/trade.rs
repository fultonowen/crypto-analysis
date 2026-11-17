use async_graphql::{InputObject, NewType};
use chrono::{DateTime, Utc};

#[derive(NewType)]
pub struct SymbolClassifier(pub String);

#[derive(NewType)]
pub struct Timestamp(pub DateTime<Utc>);

#[derive(InputObject)]
pub struct TradeInput {
    pub symbol: SymbolClassifier,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>
}
