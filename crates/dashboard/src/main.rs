use dashboard::{app::App, config::AppConfig};
use leptos::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();
    let app_config = AppConfig::from_env();
    mount_to_body(move || {
        view! {
            <App config=app_config />
        }
    });
}
