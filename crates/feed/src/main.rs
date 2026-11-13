use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use feed::binance::dto::AggTradeData;
use feed::config::AppConfig;
use framework::config::EnvConfig;
use framework::observability;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _otel_guard = observability::configure()?;

    let config = AppConfig::from_env()?;
    println!("Config: {config:?}");
    println!("Starting feed handler");
    let url_str =
        String::from("wss://stream.binance.us:9443/ws"); // /stream?streams=bnbusdt@aggTrade/btcusdt@markPrice");
    println!("Connecting to Binance WebSocket...");
    tracing::info!("Connecting to Binance Websocket...");
    let (ws_stream, _) = connect_async(url_str).await.context("Failed to connect")?;
    let (mut write, mut read) = ws_stream.split();
    let init_message = serde_json::json!(
        {
            "method": "SUBSCRIBE",
            "params": ["btcusd@aggTrade", "ethusd@aggTrade"],
            "id": 1,
        }
    );
    let _ = write.send(Message::Text(Utf8Bytes::from(init_message.to_string()))).await;
    tracing::info!("Sent message to Binance!");
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                println!("{}", text);
                let agg_trade: AggTradeData = serde_json::from_str(&text).unwrap();
                println!(
                    "Event: {}, Symbol: {}, Price: {}, Quantity: {}",
                    agg_trade.e, agg_trade.s, agg_trade.p, agg_trade.q
                );
                tracing::info!(
                    "Event: {}, Symbol: {}, Price: {}, Quantity: {}",
                    agg_trade.e, agg_trade.s, agg_trade.p, agg_trade.q
                );
            }
            Ok(Message::Ping(_))
            | Ok(Message::Pong(_))
            | Ok(Message::Binary(_))
            | Ok(Message::Frame(_)) => {}
            Ok(Message::Close(_)) => {
                tracing::warn!("Connection closed.");
                return Ok(())
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
