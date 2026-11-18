use leptos::prelude::*;

use crate::{components::form::ControlledForm, config::AppConfig, views::series::SeriesView};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Demo {
    Series
}

impl Demo {
    const ALL: [Demo; 1] = [Demo::Series];

    fn title(self) -> &'static str {
        match self {
            Demo::Series => "Series"
        }
    }

    fn description(self) -> &'static str {
        match self {
            Demo::Series => {
                "The six series types, price lines, and a moving average. \
                Changing the type mounts another series component."
            }
        }
    }
}

#[component]
pub fn App(config: AppConfig) -> impl IntoView {
    provide_context(config);
    let demo = RwSignal::new(Demo::Series);
    let form_item = RwSignal::new(String::from("BTCUSD"));
    view! {
        <main>
            <h1>"Binance Trading Charts"</h1>
            <nav>
                {Demo::ALL
                    .into_iter()
                    .map(|item| {
                        view! {
                            <button class:active=move || demo.get() == item on:click=move |_| demo.set(item)>
                                {item.title()}
                            </button>
                        }
                    })
                    .collect_view()}
            </nav>
            <p class="description">{move || demo.get().description()}</p>
            <ControlledForm form_item=form_item />
            {move || match demo.get() {
                Demo::Series => view! { <SeriesView form_item=form_item /> }.into_any(),
            }}
        </main>
    }
}
