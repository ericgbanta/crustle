#![allow(non_snake_case)]

use dioxus::prelude::*;

// Constants
const GITHUB_IMG: &'static str = wasm_or_else("github.svg", "public/github.svg");

// Entry point
fn main() {
    dioxus_web::launch(App);
}

// App component
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {},
            About {},
        }
    })
}

// Header component
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            class: "flex justify-between items-center p-8",
            h1 {
                class: "text-4xl",
                "pokérust"
            },
            div {
                class: "flex items-center",
                nav {
                    class: "hidden md:flex",
                    a { class: "px-4 py-2", href: "/", "Home" }
                    a { class: "px-4 py-2", href: "/pokedex", "Pokedex" }
                    a { class: "px-4 py-2", href: "/contact", "Contact" }
                }
            }
        }
    })
}

// About component
pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            class: "mt-auto p-8 flex items-center italic",
            a {
                href: "https://github.com/ericgbanta/pokerust/",
                target: "_blank",
                img {
                    class: "w-4 sm:w-8 align-middle mr-2",
                    src: GITHUB_IMG,
                }
            }
            " An Open Source project to create a Pokédex using Rust & Dioxus."
        }
    })
}

// Utility function
const fn wasm_or_else<'a, T: ?Sized>(then: &'a T, _else: &'a T) -> &'a T {
    if cfg!(target_family = "wasm") {
        then
    } else {
        _else
    }
}
