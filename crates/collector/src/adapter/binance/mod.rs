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

#[cfg(test)]
pub mod tests {
    use std::assert_matches;
    use super::*;
    use serde_json::json;

    impl Default for BinanceAdapter {
        fn default() -> Self {
            Self {
                binance_ws_url: "".to_string(),
                streams: vec![],
                reconnection_attempts: 0,
            }
        }
    }
    #[test]
    fn ticker_event_parsed() {
        let json = json!({"e":"24hrTicker","E":1794680561362i64,"s":"ETHUSD","p":"-105.41000000","P":"-5.440","w":"1879.43621343","x":"1938.08000000","c":"1832.32000000","Q":"0.00880000","b":"1832.30000000","B":"0.00880000","a":"1832.44000000","A":"0.00880000","o":"1937.73000000","h":"1954.70000000","l":"1810.55000000","v":"1788.10160000","q":"3360622.90032800","O":1794594161303i64,"C":1794680561303i64,"F":51771873,"L":51799254,"n":27382});
        let json_str = json.to_string();
        let bytes = json_str.as_bytes();
        let adapter = BinanceAdapter::default();
        let market_event = adapter.parse(bytes);
        assert_matches!(market_event, Ok(Some(MarketEvent::Price(_))));
    }

    #[test]
    fn trade_event_parsed() {
        let json = json!({"e":"trade","E":1794980390633i64,"s":"ETHUSD","t":51901834,"p":"1813.91000000","q":"0.00890000","T":1794980390633i64,"m":false,"M":true});
        let json_str = json.to_string();
        let bytes = json_str.as_bytes();
        let adapter = BinanceAdapter::default();
        let market_event = adapter.parse(bytes);
        assert_matches!(market_event, Ok(Some(MarketEvent::Trade(_))));
    }

    #[test]
    fn non_covered_event_ignored() {
        let json = json!({"e":"ignored_stream", "vwap": "1831.910000000"});
        let json_str = json.to_string();
        let bytes = json_str.as_bytes();
        let adapter = BinanceAdapter::default();
        let market_event = adapter.parse(bytes);
        assert_matches!(market_event, Ok(None));
    }

    #[test]
    fn maker_flag_parsed_correctly() {
        let json = json!({"e":"trade","E":1794980390633i64,"s":"ETHUSD","t":51901834,"p":"1813.91000000","q":"0.00890000","T":1794980390633i64,"m":false,"M":true});
        let json_str = json.to_string();
        let bytes = json_str.as_bytes();
        let adapter = BinanceAdapter::default();
        let market_event = adapter.parse(bytes);
        assert_matches!(market_event, Ok(Some(MarketEvent::Trade(x))) if x.side == Side::BUY);
    }
}
