use serde::Deserialize;

/// Outlines all the supported Binance message streams.
/// Schema deserialization is decided based on the
/// "e" -> event_type entry.
#[derive(Deserialize, Debug)]
#[serde(tag = "e")]
pub enum Combined<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "trade"))]
    Trade(TradeMsg<'a>),
    #[serde(borrow)]
    #[serde(rename(deserialize = "24hrTicker"))]
    Ticker(TickerPrice<'a>)
}

/// Intended for use when subscribing to the @trade stream
#[derive(Debug, Deserialize)]
pub struct TradeMsg<'a> {
    #[serde(rename(deserialize = "E"))]
    pub event_time: i64,
    #[serde(rename(deserialize = "s"))]
    pub symbol: &'a str,
    #[serde(rename(deserialize = "t"))]
    pub trade_id: i64,
    #[serde(rename(deserialize = "p"))]
    pub price: &'a str,
    #[serde(rename(deserialize = "q"))]
    pub quantity: &'a str,
    #[serde(rename(deserialize = "T"))]
    pub trade_time: i64, // trade time (ms)
    #[serde(rename(deserialize = "m"))]
    pub is_maker: bool, // true = buyer is maker, so the aggressor sold
    #[serde(rename(deserialize = "M"))]
    pub ignore: bool
}

/// Intended for use when subscribing to the @ticker stream.
/// Interval or windowed @ticker streams are currently unsupported.
#[derive(Debug, Deserialize)]
pub struct TickerPrice<'a> {
    #[serde(rename(deserialize = "E"))]
    pub event_time: i64,
    #[serde(rename(deserialize = "s"))]
    pub symbol: &'a str,
    #[serde(rename(deserialize = "p"))]
    pub price_change: &'a str,
    #[serde(rename(deserialize = "P"))]
    pub price_change_pct: &'a str,
    #[serde(rename(deserialize = "o"))]
    pub open_px: &'a str, // open px
    #[serde(rename(deserialize = "h"))]
    pub high_px: &'a str,
    #[serde(rename(deserialize = "l"))]
    pub low_px: &'a str,
    #[serde(rename(deserialize = "c"))]
    pub last_px: &'a str,
    #[serde(rename(deserialize = "w"))]
    pub w_avg_px: &'a str,
    #[serde(rename(deserialize = "v"))]
    pub total_asset_volume: &'a str,
    #[serde(rename(deserialize = "q"))]
    pub total_quote_asset_volume: &'a str,
    #[serde(rename(deserialize = "O"))]
    pub stat_open_ts_ms: i64,
    #[serde(rename(deserialize = "C"))]
    pub stat_close_ts_ms: i64,
    #[serde(rename(deserialize = "F"))]
    pub first_trade_id: i64,
    #[serde(rename(deserialize = "L"))]
    pub last_trade_id: i64,
    #[serde(rename(deserialize = "n"))]
    pub total_trades: i64
}
