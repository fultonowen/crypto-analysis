use std::time::Duration;

use leptos::prelude::*;
use trading_charts::{
    chart::Chart,
    data::{
        Candlestick, HistogramData, LegendOptions, PriceLineOptions, ValueData, options::LineWidth
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

use super::base_options;
use crate::{
    api::query,
    components::choice::Choice,
    config::AppConfig,
    transform::{candlesticks, closes, moving_average, vwap_rolling}
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

#[component]
pub fn SeriesView(form_item: RwSignal<String>) -> impl IntoView {
    let candles: RwSignal<Vec<Candlestick>> = RwSignal::new(vec![]);
    let average: RwSignal<f64> = RwSignal::new(0f64);
    let values: RwSignal<Vec<ValueData>> = RwSignal::new(vec![]);
    let histogram: RwSignal<Vec<HistogramData>> = RwSignal::new(vec![]);
    let ma: RwSignal<Vec<ValueData>> = RwSignal::new(vec![]);
    let vwap: RwSignal<Vec<ValueData>> = RwSignal::new(vec![]);
    let price_lines: RwSignal<Vec<PriceLineOptions>> = RwSignal::new(vec![]);
    let base_series_options = RwSignal::new(BaselineSeriesOptions::new());

    let async_data = LocalResource::new(move || {
        let context = expect_context::<AppConfig>();
        let value = form_item.get().clone();
        query::trades_by_symbol(context.graphql_endpoint, value)
    });

    let handle = set_interval_with_handle(
        move || {
            async_data.refetch();
            if let Some(data) = async_data.get() {
                candles.set(candlesticks(&data));
                let avg = data.iter().map(|item| item.close).sum::<f64>() / data.len() as f64;
                average.set(avg);
                values.set(closes(&data));
                let histogram_v = data
                    .iter()
                    .map(|item| HistogramData::new(item.time, item.close - item.open))
                    .collect();
                histogram.set(histogram_v);
                ma.set(moving_average(&data, 20));
                vwap.set(vwap_rolling(&data));
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

    let kind = RwSignal::new(Kind::Candlestick);

    let markers = Signal::stored(vec![]);
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
                    .with_title("MA 20 min")
                    .with_color("#f97316")
                    .with_line_width(LineWidth::W1)
                    .with_price_line_visible(false)
                    .with_last_value_visible(false)
            />
            <LineSeries
                data=vwap
                options=LineSeriesOptions::new()
                    .with_title("Vwap")
                    .with_color("#0891b2")
                    .with_line_width(LineWidth::W1)
                    .with_price_line_visible(false)
                    .with_last_value_visible(false)
            />
        </Chart>
    }
}
