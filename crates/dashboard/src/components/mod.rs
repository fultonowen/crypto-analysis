use leptos::prelude::*;

#[component]
pub fn ControlledForm(form_item: RwSignal<String>) -> impl IntoView {
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" bind:value=form_item placeholder="Enter name" />
            <button type="submit">"Submit"</button>
        </form>
    }
}