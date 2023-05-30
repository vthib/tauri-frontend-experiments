mod api;
mod app;
mod components;
mod icons;
mod stores;
mod utils;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
