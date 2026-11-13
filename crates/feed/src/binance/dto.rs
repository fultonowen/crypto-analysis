use serde::Deserialize;

// todo: remove
#[derive(Deserialize)]
pub struct AggTradeData {
    pub e: String, // event type
    #[serde(rename = "E")]
    pub e2: i64, // event time (ms)
    pub a: i64, // aggregate trade id
    pub s: String, // symbol
    pub p: String, // price
    pub q: String, // quantity
    pub f: i64, // first trade id
    pub l: i64, // last trade id
    #[serde(rename = "T")]
    pub t: i64, // trade time (ms)
    pub m: bool, //  buyer market maker?
}