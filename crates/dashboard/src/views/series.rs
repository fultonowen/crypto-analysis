use std::time::Duration;

use leptos::prelude::*;
use trading_charts::{
    chart::Chart,
    data::{
        Candlestick, HistogramData, LegendOptions, MarkerPosition, MarkerShape, MarkerStyle,
        OhlcvData, PriceLineOptions, ValueData, options::LineWidth, register_marker_style
    },
    series::{
        areas::{AreaSeries, AreaSeriesOptions},
        bars::{BarSeries, BarSeriesOptions},
        baselines::{BaselineBaseValue, BaselineSeries, BaselineSeriesOptions},
        candlesticks::{CandleStickSeries, CandlestickOptions},
        histograms::{HistogramSeries, HistogramSeriesOptions},
        lines::{LineSeries, LineSeriesOptions}
    }
};

use super::{Choice, base_options};
use crate::{
    data2::{candlesticks, closes, moving_average},
    query
};

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Candlestick,
    Bar,
    Line,
    Area,
    Baseline,
    Histogram
}

/// Range of the items replaced by whitespace items: empty slots on the time scale.
const GAP: std::ops::Range<usize> = 60..68;

fn closes_with_gap(data: &[OhlcvData]) -> Vec<ValueData> {
    closes(data)
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            if GAP.contains(&index) {
                ValueData::whitespace(item.time())
            } else {
                item
            }
        })
        .collect()
}

fn candles_with_gap(data: &[OhlcvData]) -> Vec<Candlestick> {
    candlesticks(data)
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            if GAP.contains(&index) {
                Candlestick::whitespace(item.time())
            } else {
                item
            }
        })
        .collect()
}

#[component]
pub fn SeriesDemo(form_item: RwSignal<String>) -> impl IntoView {
    let candles: RwSignal<Vec<Candlestick>> = RwSignal::new(vec![]);
    let average: RwSignal<f64> = RwSignal::new(0f64);
    let values: RwSignal<Vec<ValueData>> = RwSignal::new(vec![]);
    let histogram: RwSignal<Vec<HistogramData>> = RwSignal::new(vec![]);
    let ma: RwSignal<Vec<ValueData>> = RwSignal::new(vec![]);
    let price_lines: RwSignal<Vec<PriceLineOptions>> = RwSignal::new(vec![]);
    let base_series_options = RwSignal::new(BaselineSeriesOptions::new());

    let async_data = LocalResource::new(move || {
        let value = form_item.get().clone();
        query::trades_by_symbol(value)
    });

    let handle = set_interval_with_handle(
        move || {
            async_data.refetch();
            if let Some(data) = async_data.get() {
                candles.set(candles_with_gap(&data));
                let avg = data.iter().map(|item| item.close).sum::<f64>() / data.len() as f64;
                average.set(avg);
                values.set(closes_with_gap(&data));
                let histogram_v = data
                    .iter()
                    .map(|item| HistogramData::new(item.time, item.close - item.open))
                    .collect();
                histogram.set(histogram_v);
                ma.set(moving_average(&data, 20));
                price_lines.set(vec![
                    PriceLineOptions::new(average.get())
                        .with_title("Average")
                        .with_color("#64748b")
                        .with_line_width(LineWidth::W2),
                ]);
                base_series_options.set(
                    BaselineSeriesOptions::new()
                        .with_title("Close vs average")
                        .with_base_value(BaselineBaseValue::price(average.get()))
                )
            }
        },
        Duration::from_secs(10)
    );

    on_cleanup(move || {
        if let Ok(handle) = handle {
            handle.clear();
        }
    });

    let _ = register_marker_style(
        "earnings",
        &MarkerStyle::new()
            .with_shape(MarkerShape::Circle)
            .with_position(MarkerPosition::AboveBar)
            .with_color("#db2777")
            .with_text("E")
    );

    // todo: my example here, may remove
    // let data = move || async_data.get().unwrap_or_else(|| vec![]);
    // let average = move || {
    //     let xs = data();
    //     xs.iter().map(|item| item.close).sum::<f64>() / xs.len() as f64
    // };
    let kind = RwSignal::new(Kind::Candlestick);
    // let candles = Signal::stored(candles_with_gap(&data));
    // let values = Signal::stored(closes_with_gap(&data));
    // let histogram = Signal::stored(
    //     data.iter()
    //         .map(|item| HistogramData::new(item.time, item.close - item.open))
    //         .collect::<Vec<_>>()
    // );
    // let ma = Signal::stored(moving_average(&data, 20));

    let markers = Signal::stored(vec![]);
    // let price_lines = Signal::stored(vec![
    //     PriceLineOptions::new(average)
    //         .with_title("Average")
    //         .with_color("#64748b")
    //         .with_line_width(LineWidth::W2),
    // ]);
    let legend = Signal::stored(Some(LegendOptions::new().with_text("1 Minute Bars")));

    view! {
        <Choice
            label="Series type"
            value=kind
            options=vec![
                (Kind::Candlestick, "Candlestick"),
                (Kind::Bar, "Bar"),
                (Kind::Line, "Line"),
                (Kind::Area, "Area"),
                (Kind::Baseline, "Baseline"),
                (Kind::Histogram, "Histogram"),
            ]
        />
        <Chart options=base_options() legend=legend class="chart">
            {move || match kind.get() {
                Kind::Candlestick => view! {
                    <CandleStickSeries
                        data=move || candles.get()
                        markers=markers
                        price_lines=price_lines
                        options=CandlestickOptions::new().with_title("Candles").with_wick_up_color("#16a34a")
                    />
                }
                .into_any(),
                Kind::Bar => view! {
                    <BarSeries
                        data=candles
                        markers=markers
                        price_lines=price_lines
                        options=BarSeriesOptions::new().with_title("Bars").with_thin_bars(false)
                    />
                }
                .into_any(),
                Kind::Line => view! {
                    <LineSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=LineSeriesOptions::new().with_title("Close").with_color("#2563eb")
                    />
                }
                .into_any(),
                Kind::Area => view! {
                    <AreaSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=AreaSeriesOptions::new()
                            .with_title("Close")
                            .with_line_color("#0891b2")
                            .with_top_color("rgba(8, 145, 178, 0.4)")
                            .with_bottom_color("rgba(8, 145, 178, 0.02)")
                    />
                }
                .into_any(),
                Kind::Baseline => view! {
                    <BaselineSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=base_series_options
                    />
                }
                .into_any(),
                Kind::Histogram => view! {
                    <HistogramSeries
                        data=histogram
                        markers=markers
                        options=HistogramSeriesOptions::new().with_title("Close - open").with_color("#6366f1")
                    />
                }
                .into_any(),
            }}
            <LineSeries
                data=ma
                options=LineSeriesOptions::new()
                    .with_title("MA 20")
                    .with_color("#f97316")
                    .with_line_width(LineWidth::W1)
                    .with_price_line_visible(false)
                    .with_last_value_visible(false)
            />
        </Chart>
    }
}
