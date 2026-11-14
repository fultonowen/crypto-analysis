use collector::{adapter::binance::BinanceAdapter, config::AppConfig, run, sink::{stdout::StdoutSink}};
use framework::{config::EnvConfig, observability};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting collector");
    let _otel_guard = observability::configure()?;
    let config = AppConfig::from_env()?;
    // todo: read streams from configuration
    let streams = vec![
        "btcusd@ticker",
        "ethusd@ticker",
        "bnbusd@ticker",
        "xrpusd@ticker",
        "solusd@ticker",
        "trxusd@ticker",
        "hypeusd@ticker",
        "zecusd@ticker",
        "dogeusd@ticker",
        "xmrusd@ticker",
        "linkusd@ticker",
        "adausd@ticker",
        "xlmusd@ticker",
        "nearusd@ticker",
        "daiusd@ticker",
        "ltcusd@ticker",
        "avaxusd@ticker",
        "uniusd@ticker",
        "ccusd@ticker",
        "suiusd@ticker",
        "hbarusd@ticker",
        "shibusd@ticker",
        "taousd@ticker",
        "crousd@ticker",
        "qntusd@ticker",
        "pepeusd@ticker",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    run(BinanceAdapter::new(&config.binance, streams), StdoutSink).await
}
