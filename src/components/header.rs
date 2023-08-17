use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Header component
#[inline_props]
pub fn Header(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        header {
            class: "flex justify-between items-center p-8",
            h1 {
                class: "text-4xl",
                "{name}"
            },
            div {
                class: "flex items-center",
                nav {
                    class: "hidden md:flex",
                    Link { class: "px-4 py-2", to: "/", "Home" }
                    Link { class: "px-4 py-2", to: "/pokedex", "Pokédex" }
                    Link { class: "px-4 py-2", to: "/contact", "Contact" }
                }
            }
        }
    })
}
