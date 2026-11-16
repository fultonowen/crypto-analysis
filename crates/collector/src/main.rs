use collector::{
    adapter::binance::BinanceAdapter, config::AppConfig, run, sink::timescaledb::TimescaleDBSink
};
use framework::{config::EnvConfig, observability};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting collector");
    let otel_guard = observability::configure()?;
    let config = AppConfig::from_env()?;
    // todo: read streams from configuration
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
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let binance = BinanceAdapter::new(&config.binance, streams);
    let sink = TimescaleDBSink::from_config(config.database).await?;
    run(binance, sink).await?;
    otel_guard.shutdown()?;
    Ok(())
}
