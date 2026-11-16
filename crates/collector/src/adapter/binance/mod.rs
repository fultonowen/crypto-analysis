pub mod model;

use ::model::{MarketEvent, Side, TickerPrice, Trade};

use crate::adapter::{binance::model::Combined, config::FeedConfig, exchange::ExchangeAdapter};

pub struct BinanceAdapter {
    /// e.g. wss://stream.binance.us:9443
    binance_ws_url: String,
    /// e.g. ["btcusd@ticker", "btcusd@trade"]
    streams: Vec<String>,
    reconnection_attempts: u32
}

impl BinanceAdapter {
    pub fn new(config: &FeedConfig, streams: Vec<String>) -> Self {
        Self {
            binance_ws_url: config.ws_url.clone(),
            streams,
            reconnection_attempts: config.max_connection_attempts as u32
        }
    }
}

impl ExchangeAdapter for BinanceAdapter {
    fn url(&self) -> String {
        format!("{}/ws", self.binance_ws_url)
    }

    fn connection_attempts(&self) -> u32 {
        self.reconnection_attempts
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
        let Ok(m) = serde_json::from_slice::<Combined>(raw) else {
            let res = serde_json::from_slice::<Combined>(raw);
            tracing::warn!(raw=?raw, "parsing binance message failed");
            eprintln!("Parsing failed: {res:?}"); // todo: take this out
            return Ok(None);
        };
        println!("Parsing and using...."); // todo: remove
        match m {
            Combined::Trade(m) => Ok(Some(MarketEvent::Trade(Trade {
                exchange: "binance",
                symbol: m.symbol.to_owned(),
                trade_id: m.trade_id,
                price: m.price.parse()?,
                quantity: m.quantity.parse()?,
                trade_time_ms: m.trade_time,
                event_time_ms: m.event_time,
                side: if m.is_maker { Side::SELL } else { Side::BUY }
            }))),
            Combined::Ticker(p) => Ok(Some(MarketEvent::Price(TickerPrice {
                event_time: p.event_time,
                exchange: "binance",
                symbol: p.symbol.to_owned(),
                last_price: p.last_px.parse()?
            })))
        }
    }
}
