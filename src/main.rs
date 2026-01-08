use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;
mod utils;
mod wasm;

use app::App;

fn main() {
    // QoL tweak
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
