use crate::components::layout::PageLayout;
use dioxus::prelude::*;

const ROW: &str = "group flex items-center justify-between gap-4 px-5 py-4 hover:bg-sunken \
    focus-visible:-outline-offset-2";

#[component]
pub fn Contact() -> Element {
    rsx! {
        PageLayout {
            title: "Contact",
            intro: "Questions, bugs and ideas are welcome on GitHub.",
            ul {
                class: "divide-y divide-line overflow-hidden rounded-xl border border-line bg-surface",
                li {
                    a {
                        class: ROW,
                        href: "https://github.com/ericgbanta",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        span {
                            span { class: "block text-sm text-muted", "Maintainer" }
                            span { class: "block font-medium text-accent group-hover:underline", "@ericgbanta" }
                        }
                        ExternalMark {}
                    }
                }
                li {
                    a {
                        class: ROW,
                        href: "https://github.com/ericgbanta/crustle/issues/new/choose",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        span {
                            span { class: "block text-sm text-muted", "Found a problem?" }
                            span { class: "block font-medium text-accent group-hover:underline", "File an issue" }
                        }
                        ExternalMark {}
                    }
                }
            }
        }
    }
}

/// Tells sighted and screen reader users alike that a link leaves the app.
#[component]
fn ExternalMark() -> Element {
    rsx! {
        span {
            class: "shrink-0 text-muted",
            span { aria_hidden: "true", "↗" }
            span { class: "sr-only", "(opens in a new tab)" }
        }
    }
}
