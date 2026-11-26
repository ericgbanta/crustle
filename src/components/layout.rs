use crate::components::about::About;
use crate::components::header::Header;
use dioxus::prelude::*;

/// The frame shared by every page: header with navigation, the page heading with an
/// optional line of introduction, the page's content, and the About footer pinned to
/// the bottom.
#[component]
pub fn PageLayout(title: String, intro: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "flex min-h-dvh flex-col",
            Header {}
            main {
                class: "mx-auto w-full max-w-3xl flex-1 px-4 py-8 sm:px-6 sm:py-12",
                h1 {
                    class: "text-3xl font-semibold tracking-tight",
                    "{title}"
                }
                if let Some(intro) = intro {
                    p {
                        class: "mt-2 text-muted",
                        "{intro}"
                    }
                }
                div {
                    class: "mt-8",
                    {children}
                }
            }
            About {}
        }
    }
}
