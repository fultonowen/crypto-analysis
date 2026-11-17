pub mod message;

use chrono::DateTime;
use model::{MarketEvent, Side, TickerPrice, Trade};

use crate::adapter::{binance::message::Combined, config::FeedConfig, exchange::ExchangeAdapter};

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
            tracing::warn!(raw=?raw, "parsing binance message failed");
            return Ok(None);
        };
        match m {
            Combined::Trade(m) => Ok(Some(MarketEvent::Trade(Trade {
                exchange: "binance".to_owned(),
                symbol: m.symbol.to_owned(),
                trade_id: m.trade_id,
                price: m.price.parse()?,
                quantity: m.quantity.parse()?,
                trade_time: DateTime::from_timestamp_millis(m.trade_time)
                    .ok_or(anyhow::anyhow!("trade time is invalid"))?,
                event_time: DateTime::from_timestamp_millis(m.event_time)
                    .ok_or(anyhow::anyhow!("event time is invalid"))?,
                side: if m.is_maker { Side::SELL } else { Side::BUY }
            }))),
            Combined::Ticker(p) => Ok(Some(MarketEvent::Price(TickerPrice {
                event_time: DateTime::from_timestamp_millis(p.event_time)
                    .ok_or(anyhow::anyhow!("event time is invalid"))?,
                exchange: "binance".to_owned(),
                symbol: p.symbol.to_owned(),
                last_price: p.last_px.parse()?,
                open_price: p.open_px.parse()?,
                high_price: p.high_px.parse()?,
                low_price: p.low_px.parse()?,
                w_avg_price: p.w_avg_px.parse()?
            })))
        }
    }
}
