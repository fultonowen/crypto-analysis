use std::str::FromStr;

use cynic::{GraphQlResponse, QueryBuilder};
use trading_charts::data::{OhlcvData, UTCTimestamp};

use crate::data::{Candle1mBySymbol, Candle1mBySymbolVariables, TradeInput};

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
    let response = reqwest::Client::new()
        .post(host)
        .json(&operation)
        .send()
        .await
        .unwrap();
    log::info!("response retrieved from GraphQL API");
    let graphql_response = response
        .json::<GraphQlResponse<Candle1mBySymbol>>()
        .await
        .unwrap();
    log::info!("response deserialized from GraphQL API");
    graphql_response
        .data
        .unwrap()
        .candle1_minute
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
