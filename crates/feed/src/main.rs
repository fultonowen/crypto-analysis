use feed::binance::BinanceAdapter;
use feed::config::AppConfig;
use feed::sink::{pipeline, StdoutSink};
use framework::config::EnvConfig;
use framework::observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting feed");
    let _otel_guard = observability::configure()?;
    let config = AppConfig::from_env()?;
    let streams = vec![
        "btcusd@trade",
        "ethusd@trade",
        "bnbusd@trade",
        "xrpusd@trade",
        "solusd@trade",
        "trxusd@trade",
        "hypeusd@trade",
        "zecusd@trade",
        "dogeusd@trade",
        "xmrusd@trade",
        "linkusd@trade",
        "adausd@trade",
        "xlmusd@trade",
        "nearusd@trade",
        "daiusd@trade",
        "ltcusd@trade",
        "avaxusd@trade",
        "uniusd@trade",
        "ccusd@trade",
        "suiusd@trade",
        "hbarusd@trade",
        "shibusd@trade",
        "taousd@trade",
        "crousd@trade",
        "qntusd@trade",
        "pepeusd@trade",

    ].into_iter().map(String::from).collect();
    pipeline::run(BinanceAdapter::new(&config.binance, streams), StdoutSink).await
}