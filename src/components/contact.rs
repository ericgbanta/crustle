use crate::components::layout::PageLayout;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        PageLayout {
            title: "Contact",
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
        }
    }
}
