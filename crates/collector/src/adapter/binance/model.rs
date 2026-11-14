use serde::Deserialize;

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
    #[serde(rename(deserialize ="T"))]
    pub trade_time: i64, // trade time (ms)
    #[serde(rename(deserialize = "m"))]
    pub is_maker: bool, // true = buyer is maker, so the aggressor sold
    #[serde(rename(deserialize = "M"))]
    pub ignore: bool,
}

/// Intended for use when subscribing to the @trade stream
#[derive(Debug, Deserialize)]
pub struct TickerPrice<'a> {
    // pub e: &'a str, // event type // todo: see if this works
    #[serde(rename(deserialize = "E"))]
    pub event_time: i64, // Event time
    #[serde(rename(deserialize = "s"))]
    pub symbol: &'a str, // symbol
    #[serde(rename(deserialize = "p"))]
    pub price_change: &'a str, // price change
    #[serde(rename(deserialize = "P"))]
    pub price_change_pct: &'a str, // price change percent
    #[serde(rename(deserialize = "o"))]
    pub open_px: &'a str, // open px
    #[serde(rename(deserialize = "h"))]
    pub high_px: &'a str,
    #[serde(rename(deserialize = "l"))]
    pub low_px: &'a str,
    #[serde(rename(deserialize = "c"))]
    pub last_px: &'a str,
    #[serde(rename(deserialize = "w"))]
    pub w_avg_px: &'a str, // weighted avg price
    #[serde(rename(deserialize = "v"))]
    pub v: &'a str, // total traded base asset volume
    pub q: &'a str, // total traded quote asset volume
    pub O: i64, // statistics open time
    pub C: i64, // statistics close time
    pub F: i64, // first trade id
    pub L: i64, // last trade id
    pub n: i64, // total num trades

}
