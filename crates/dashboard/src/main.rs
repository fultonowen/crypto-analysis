use dashboard::app::App;
use leptos::mount::mount_to_body;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
