mod api;
mod components;
mod dex;
mod format;
mod models;
mod route;
mod search;

use dioxus::prelude::*;
pub use route::Route;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
