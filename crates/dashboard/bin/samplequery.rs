use cynic::{QueryBuilder, http::ReqwestExt};
use dashboard::{
    data,
    data::{TradeInput, TradesBySymbol, TradesBySymbolVariables}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = "http://localhost:8080/api/graphql";
    let input = TradesBySymbolVariables {
        input: TradeInput {
            symbol: "ETHUSD",
            start_time: None,
            end_time: None
        }
    };
    let operation = data::TradesBySymbol::build(input);
    let response = reqwest::Client::new()
        .post(host)
        .json(&operation)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}
