
/// this schema allows for auto-gen of rust types from GraphQL SDL
#[cynic::schema("server")]
pub mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct TradesBySymbolVariables<'a> {
    pub input: TradeInput<'a>
}

#[derive(cynic::QueryVariables, Debug)]
pub struct Candle1mBySymbolVariables<'a> {
    pub input: TradeInput<'a>
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "Candle1mBySymbolVariables")]
pub struct Candle1mBySymbol {
    #[arguments(input: $input)]
    pub candle1_minute: Vec<Candle1Minute>
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Candle1Minute {
    pub time_bucket: DateTime,
    pub symbol: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "TradesBySymbolVariables")]
pub struct TradesBySymbol {
    #[arguments(input: $input)]
    pub trades: Vec<TradeData>
}

#[derive(cynic::QueryFragment, Debug)]
pub struct TradeData {
    pub trade_time: DateTime,
    pub symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: TradeSide,
    pub exchange: String,
    pub event_time: DateTime
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum TradeSide {
    #[cynic(rename = "buy")]
    Buy,
    #[cynic(rename = "sell")]
    Sell
}

#[derive(cynic::InputObject, Debug)]
pub struct TradeInput<'a> {
    pub symbol: &'a str,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Decimal(pub String);
