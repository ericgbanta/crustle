use super::about::About;
use super::header::Header;
use dioxus::prelude::*;

pub fn Contact() -> Element {
    rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header { name: "Contact".to_string() }
            div {
                class: "text-center",
                a {
                    href: "https://github.com/ericgbanta",
                    target: "_blank",
                    "@ericgbanta"
                }
                br {}
                a {
                    href: "https://github.com/ericgbanta/crustle/issues/new/choose",
                    target: "_blank",
                    "File an issue"
                }
            }
            About {}
        }
    }
}
