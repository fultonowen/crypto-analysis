pub mod dto;

use model::{MarketEvent, Side, Trade};
use serde::Deserialize;
use tracing::instrument;

use crate::adapter::{config::FeedConfig, exchange::ExchangeAdapter};

pub struct BinanceAdapter {
    /// e.g. wss://stream.binance.us:9443
    binance_ws_url: String,
    /// e.g. `btcusdt@trade`
    streams: Vec<String>
}

impl BinanceAdapter {
    pub fn new(config: &FeedConfig, streams: Vec<String>) -> Self {
        Self {
            binance_ws_url: config.ws_url.clone(),
            streams
        }
    }
}

#[derive(Deserialize)]
struct Combined<'a> {
    #[serde(borrow)]
    data: TradeMsg<'a>
}

#[derive(Deserialize)]
struct TradeMsg<'a> {
    s: &'a str,
    t: u64,
    p: &'a str,
    q: &'a str,
    #[serde(rename = "T")]
    ts: u64,
    m: bool // true = buyer is maker, so the aggressor sold
}

impl ExchangeAdapter for BinanceAdapter {
    fn url(&self) -> String {
        format!("{}/ws", self.binance_ws_url)
    }

    fn subscribe_msg(&self) -> Option<String> {
        let init_message = serde_json::json!(
            {
                "method": "SUBSCRIBE",
                "params": self.streams,
                "id": 1,
            }
        );
        Some(init_message.to_string())
    }

    fn parse(&self, raw: &[u8]) -> anyhow::Result<Option<MarketEvent>> {
        println!("Parsing!");
        let Ok(m) = serde_json::from_slice::<TradeMsg>(raw) else {
            return Ok(None);
        };
        println!("Parsing and using....");
        // let m = c.data;
        Ok(Some(MarketEvent::Trade(Trade {
            exchange: "binance",
            symbol: m.s.to_owned(),
            trade_id: m.t,
            price: m.p.parse()?,
            qty: m.q.parse()?,
            ts_ms: m.ts,
            aggressor: if m.m { Side::Sell } else { Side::Buy }
        })))
    }
}
