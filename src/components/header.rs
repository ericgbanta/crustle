use crate::Route;
use dioxus::prelude::*;

// `Link` sets `aria-current="page"` on the link to the current route, which is what
// marks the active page here.
const NAV_LINK: &str = "rounded-md px-2.5 py-2 text-sm font-medium text-muted hover:bg-sunken \
    hover:text-ink sm:px-3 aria-[current=page]:bg-accent-soft aria-[current=page]:text-accent-strong";

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "border-b border-line bg-surface",
            div {
                class: "mx-auto flex max-w-3xl flex-wrap items-center justify-between gap-x-4 gap-y-1 px-4 py-3 sm:px-6",
                Link {
                    class: "-ml-1 flex items-center gap-2 rounded-md px-1 py-1 text-xl font-semibold tracking-tight",
                    to: Route::Home {},
                    span {
                        class: "size-3 rounded-sm bg-accent",
                        aria_hidden: "true",
                    }
                    "crustle"
                }
                nav {
                    class: "flex gap-1",
                    aria_label: "Main",
                    Link { class: NAV_LINK, to: Route::Home {}, "Home" }
                    Link { class: NAV_LINK, to: Route::Pokedex {}, "Pokédex" }
                    Link { class: NAV_LINK, to: Route::Contact {}, "Contact" }
                }
            }
        }
    }
}
