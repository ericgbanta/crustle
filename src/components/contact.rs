use super::about::About;
use super::header::Header;
use dioxus::prelude::*;

// Contact component
pub fn Contact(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Contact".into()},
            div {
                class: "text-center",
                a {
                    href: "https://github.com/ericgbanta",
                    target: "_blank",
                    "@ericgbanta"
                }
                // Space between the links
                br {}
                a {
                    href: "https://github.com/ericgbanta/crustle/issues/new/choose",
                    target: "_blank",
                    "File an issue"
                }
            },
            About {},
        }
    })
}
