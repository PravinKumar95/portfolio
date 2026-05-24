mod app;
mod components;

use leptos::prelude::*;
use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::logging::log!("Leptos App Started");
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
