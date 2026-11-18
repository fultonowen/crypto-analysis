//! Helpers for calculating different graphing items
use trading_charts::data::{Candlestick, OhlcvData, ValueData};

fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn candlesticks(data: &[OhlcvData]) -> Vec<Candlestick> {
    data.iter().map(OhlcvData::candlestick).collect()
}

pub fn closes(data: &[OhlcvData]) -> Vec<ValueData> {
    data.iter()
        .map(|item| ValueData::new(item.time, item.close))
        .collect()
}

/// Simple moving average of the close prices; the first items are whitespace.
pub fn moving_average(data: &[OhlcvData], length: usize) -> Vec<ValueData> {
    data.iter()
        .enumerate()
        .map(|(index, item)| {
            if index + 1 < length {
                return ValueData::whitespace(item.time);
            }

            let sum: f64 = data[index + 1 - length..=index]
                .iter()
                .map(|item| item.close)
                .sum();

            ValueData::new(item.time, round(sum / length as f64))
        })
        .collect()
}

/// Simple vwap data over time points, since times are given as 1 minute bars already
pub fn vwap_rolling(data: &[OhlcvData]) -> Vec<ValueData> {
    let (_, data) = data.iter().fold((0f64, Vec::new()), |mut acc, item| {
        let typical = (item.high + item.low + item.close) / 3.0;
        let pvt = item.volume * typical;
        acc.0 += pvt;
        let vwap = ValueData::new(item.time, round(pvt / item.volume));
        acc.1.push(vwap);
        acc
    });
    data
}

/// Relative strength index of the close prices; the first items are whitespace.
pub fn rsi(data: &[OhlcvData], length: usize) -> Vec<ValueData> {
    let mut gains = 0.0;
    let mut losses = 0.0;

    data.iter()
        .enumerate()
        .map(|(index, item)| {
            if index == 0 {
                return ValueData::whitespace(item.time);
            }

            let change = item.close - data[index - 1].close;
            let (gain, loss) = (change.max(0.0), (-change).max(0.0));

            if index <= length {
                gains += gain / length as f64;
                losses += loss / length as f64;
            } else {
                gains = (gains * (length - 1) as f64 + gain) / length as f64;
                losses = (losses * (length - 1) as f64 + loss) / length as f64;
            }

            if index < length {
                ValueData::whitespace(item.time)
            } else if losses == 0.0 {
                ValueData::new(item.time, 100.0)
            } else {
                ValueData::new(item.time, round(100.0 - 100.0 / (1.0 + gains / losses)))
            }
        })
        .collect()
}
