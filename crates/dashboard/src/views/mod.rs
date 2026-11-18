pub mod series;

use leptos::prelude::*;
use trading_charts::data::options::{ChartOptions, layout::LayoutOptions};

/// Chart options shared by the demos: the chart follows the size of its container.
pub fn base_options() -> ChartOptions {
    ChartOptions::new()
        .with_auto_size(true)
        .with_layout(LayoutOptions::new().with_attribution_logo(false))
}
