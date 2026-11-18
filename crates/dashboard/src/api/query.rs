use std::str::FromStr;

use cynic::{GraphQlResponse, QueryBuilder};
use trading_charts::data::{OhlcvData, UTCTimestamp};
use crate::api::models::{Candle1mBySymbol, Candle1mBySymbolVariables, TradeInput};

/// Querying the GraphQL server to retrieve OHLCV data
pub async fn trades_by_symbol(symbol: String) -> Vec<OhlcvData> {
    log::info!("query trades_by_symbol: {}", symbol);
    let host = "http://localhost:8080/api/graphql"; // todo: get from env vars
    let input = Candle1mBySymbolVariables {
        input: TradeInput {
            symbol: symbol.as_str(),
            start_time: None,
            end_time: None
        }
    };
    let operation = Candle1mBySymbol::build(input);
    let Ok(response) = reqwest::Client::new()
        .post(host)
        .json(&operation)
        .send()
        .await else {
        log::error!("querying GraphQL API failed");
        return vec![];
    };
    let Ok(graphql_response) = response
        .json::<GraphQlResponse<Candle1mBySymbol>>()
        .await else {
        log::error!("deserializing GraphQL API response failed");
        return vec![];
    };
    log::info!("response deserialized from GraphQL API");
    let Some(candles) = graphql_response.data else {
        log::error!("Candle1mBySymbol returned no data");
        return vec![];
    };

    candles.candle1_minute
    .into_iter()
    .map(|x| OhlcvData {
        time: UTCTimestamp::from_str(&x.time_bucket.0).unwrap(),
        open: x.open.0.parse().unwrap(),
        high: x.high.0.parse().unwrap(),
        low: x.low.0.parse().unwrap(),
        close: x.close.0.parse().unwrap(),
        volume: x.volume.0.parse().unwrap()
    })
    .collect()
}
