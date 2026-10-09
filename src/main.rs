mod app;
mod components;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
