#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use pokemon_rs;

// Routes
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/pokedex")]
    Pokedex {},
    #[route("/contact")]
    Contact {},
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

// Constants
const GITHUB_IMG: &'static str = wasm_or_else("github.svg", "public/github.svg");

// Entry point
fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> { }
    }
}

// Home component
pub fn Home(cx: Scope) -> Element {
    let random_pokemon = use_state(cx, || pokemon_rs::random(None).to_string());
    let random_pokemon_id = use_state(cx, || {
        pokemon_rs::get_id_by_name(&random_pokemon.get(), None).to_string()
    });
    let id_str = format!("{:03}", random_pokemon_id.get().parse::<u32>().unwrap_or(0));
    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Pokérust".into()},
            h2 {
                class: "text-2xl text-center",
                "Random Pokémon:"
                strong { " {random_pokemon} " }
            },
            div {
                class: "flex justify-center",
                img {
                    style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                    src: "https://raw.githubusercontent.com/HybridShivam/Pokemon/master/assets/images/{id_str}.png",
                }
            },
            About {},
        }
    })
}

// Pokedex component
pub fn Pokedex(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Pokédex".into()},
            About {},
        }
    })
}

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
                    href: "https://github.com/ericgbanta/pokerust/issues/new/choose",
                    target: "_blank",
                    "File an issue"
                }
            },
            About {},
        }
    })
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

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

// About component
pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            class: "mt-auto p-8 flex items-center italic text-xs",
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
