use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Header(name: String) -> Element {
    rsx! {
        header {
            class: "flex justify-between items-center p-8",
            h1 {
                class: "text-4xl",
                "{name}"
            }
            div {
                class: "flex items-center",
                nav {
                    class: "hidden md:flex",
                    Link { class: "px-4 py-2", to: Route::Home {}, "Home" }
                    Link { class: "px-4 py-2", to: Route::Pokedex {}, "Pokédex" }
                    Link { class: "px-4 py-2", to: Route::Contact {}, "Contact" }
                }
            }
        }
    }
}
