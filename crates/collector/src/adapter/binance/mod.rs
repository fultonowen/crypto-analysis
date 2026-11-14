pub mod model;

use ::model::{MarketEvent, Side, TickerPrice, Trade};
use crate::adapter::{config::FeedConfig, exchange::ExchangeAdapter};
use crate::adapter::binance::model::Combined;

pub struct BinanceAdapter {
    /// e.g. wss://stream.binance.us:9443
    binance_ws_url: String,
    /// e.g. ["btcusd@ticker", "btcusd@trade"]
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
        let Ok(m) = serde_json::from_slice::<Combined>(raw) else {
            let res = serde_json::from_slice::<Combined>(raw);
            eprintln!("Parsing failed: {res:?}");
            return Ok(None);
        };
        println!("Parsing and using....");
        match m {
            Combined::Trade(m) => {
                Ok(Some(MarketEvent::Trade(Trade {
                    exchange: "binance",
                    symbol: m.symbol.to_owned(),
                    trade_id: m.trade_id as u64,
                    price: m.price.parse()?,
                    qty: m.quantity.parse()?,
                    ts_ms: m.trade_time as u64,
                    aggressor: if m.is_maker { Side::Sell } else { Side::Buy }
                })))
            }
            Combined::Ticker(p) => {
                Ok(Some(
                    MarketEvent::Price(
                        TickerPrice {
                            exchange: "binance",
                            symbol: p.symbol.to_owned(),
                            last_price: p.last_px.parse()?,
                            ts_ms: p.event_time as u64,
                        }
                    )
                ))
            }
        }
    }
}
