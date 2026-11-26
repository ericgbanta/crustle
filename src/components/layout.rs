use crate::components::about::About;
use crate::components::header::Header;
use dioxus::prelude::*;

/// The frame shared by every page: header with navigation, the page's content, and
/// the About footer pinned to the bottom.
#[component]
pub fn PageLayout(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header { title }
            {children}
            About {}
        }
    }
}
