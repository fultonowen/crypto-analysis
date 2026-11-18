use leptos::{IntoView, component, prelude::*, view};

/// Toolbar of buttons selecting one value among several.
#[component]
pub fn Choice<T>(
    label: &'static str,
    options: Vec<(T, &'static str)>,
    value: RwSignal<T>
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static
{
    view! {
        <div class="toolbar">
            <span class="label">{label}</span>
            {options
                .into_iter()
                .map(|(option, text)| {
                    let selected = option.clone();

                    view! {
                        <button
                            class:active=move || value.with(|value| *value == selected)
                            on:click=move |_| value.set(option.clone())
                        >
                            {text}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
