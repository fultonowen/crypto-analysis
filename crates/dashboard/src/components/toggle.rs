use leptos::{IntoView, component, prelude::*, view};

/// Checkbox bound to a boolean signal.
#[component]
pub fn Toggle(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="toggle">
            <input type="checkbox" prop:checked=move || value.get() on:change=move |_| value.update(|v| *v = !*v) />
            {label}
        </label>
    }
}
