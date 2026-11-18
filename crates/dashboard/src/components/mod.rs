use leptos::prelude::*;
use log::info;

#[component]
pub fn ControlledForm(form_item: RwSignal<String>) -> impl IntoView {
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        info!("User changed form_item: {:?}", form_item.get());
    };

    view! {
        <form on:submit=on_submit>
            <label for="tickerName">Ticker Name </label>
            <input type="text" bind:value=form_item placeholder="Enter name" name="tickerName" />
            <button type="submit">"Submit"</button>
        </form>
    }
}
