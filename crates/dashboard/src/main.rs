use dashboard::app::App;
use dashboard::config::AppConfig;
use leptos::prelude::*;

fn main() {
    let app_config = AppConfig::from_env().unwrap();
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();
    mount_to_body(move ||
        view! {
            <App config=app_config />
        }
    );
}
