use crate::Route;
use crate::components::layout::PageLayout;
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    let path = format!("/{}", route.join("/"));

    rsx! {
        PageLayout {
            title: "Page not found",
            intro: "We are terribly sorry, but the page you requested doesn't exist.",
            div {
                class: "rounded-xl border border-line bg-surface p-5 sm:p-6",
                p { class: "text-sm text-muted", "You tried to open" }
                p {
                    class: "mt-1 font-mono text-sm break-all",
                    "{path}"
                }
                Link {
                    class: "mt-5 inline-flex rounded-md bg-accent-soft px-4 py-2 text-sm font-medium text-accent-strong hover:underline",
                    to: Route::Home {},
                    "Back to Home"
                }
            }
        }
    }
}
