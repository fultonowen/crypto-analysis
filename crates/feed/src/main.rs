use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use framework::observability;

pub mod client;
pub mod domain;
pub mod config;

#[derive(Deserialize)]
pub struct MarkPriceUpdate {
    pub stream: String,
    pub data: MarkPriceUpdateData,
}
#[derive(Deserialize)]
pub struct MarkPriceUpdateData {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub s: String,
    pub p: String,
    #[serde(rename = "P")]
    pub p2: String,
    pub i: String,
    pub r: String,
    #[serde(rename = "T")]
    pub t: i64,
}
#[derive(Deserialize)]
pub struct AggTrade {
    pub stream: String,
    pub data: AggTradeData,
}
#[derive(Deserialize)]
pub struct AggTradeData {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub a: i64,
    pub s: String,
    pub p: String,
    pub q: String,
    pub f: i64,
    pub l: i64,
    #[serde(rename = "T")]
    pub t: i64,
    pub m: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting feed handler");
    let otel_guard = observability::configure()?;
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
    write.send(Message::Text(Utf8Bytes::from(init_message.to_string()))).await;
    println!("Connected to Binance WebSocket");
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let tick_data: serde_json::Value = serde_json::from_str(&text).unwrap();
                let msg_type = &tick_data["data"]["e"];
                if msg_type == "aggTrade" {
                    let agg_trade: AggTrade = serde_json::from_str(&text).unwrap();
                    println!(
                        "Msg: {}, Symbol: {}, Price: {}, Quantity: {}",
                        msg_type, agg_trade.data.s, agg_trade.data.p, agg_trade.data.q
                    );
                }
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
