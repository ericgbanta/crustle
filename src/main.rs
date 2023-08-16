#![allow(non_snake_case)]
mod models;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use models::*;
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
    let pokemon_url = format!(
        "https://pokeapi.co/api/v2/pokemon/{}",
        random_pokemon.get().to_lowercase()
    );
    let species_url = format!(
        "https://pokeapi.co/api/v2/pokemon-species/{}",
        random_pokemon.get().to_lowercase()
    );

    let pokemon_url_string = pokemon_url.clone();

    let pokemon_data = use_future(cx, (), |_| async move {
        reqwest::get(&pokemon_url).await?.json::<Pokemon>().await
    });

    let species_data = use_future(cx, (), |_| async move {
        reqwest::get(&species_url)
            .await?
            .json::<PokemonSpecies>()
            .await
    });
    cx.render(match (pokemon_data.value(), species_data.value()) {
    (Some(Ok(pokemon)), Some(Ok(species))) => {
        let english_flavor_texts: Vec<_> = species.flavor_text_entries
            .iter()
            .filter(|entry| entry.language.name == "en")
            .collect();

        rsx! {
            div {
                class: "relative flex flex-col min-h-screen",
                Header {name:"crustle".into()},
                br {}
                br {}
                h2 {
                    class: "text-2xl text-center",
                    "Random Pokémon:"
                    strong { format!(" {}", random_pokemon.get()) }
                },
                div {
                    class: "flex justify-center",
                    img {
                        style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                        src: "https://raw.githubusercontent.com/HybridShivam/Pokemon/master/assets/images/{id_str}.png",
                    }
                },
                div {
                    class: "flex justify-center space-x-10",
                    div {
                        class: "text-center",
                        strong { "Height: " }
                        format!("{} m", pokemon.height as f32 / 10.0)
                    },
                    div {
                        class: "text-center",
                        strong { "Weight: " }
                        format!("{} kg", pokemon.weight as f32 / 10.0)
                    }
                },
                div {
                    for entry in english_flavor_texts {
                        div {
                            class: "bg-gray-100 m-4 p-4 rounded shadow",
                            h3 {
                                class: "text-lg font-bold",
                                format!("Version: {}", entry.version.name)
                            },
                            p {
                                class: "text-gray-700",
                                entry.flavor_text.clone()
                            }
                        }
                    }
                },
                About {},
            }
        }
    },
    (Some(Err(_)), _) | (_, Some(Err(_))) => rsx! {
        div {
            "Failed to fetch Pokémon data."
            pokemon_url_string
        }
    },
    (_, _) => rsx! {
        div {
            "Loading Pokémon data..."
            pokemon_url_string
        }
    },
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
                    href: "https://github.com/ericgbanta/crustle/issues/new/choose",
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
                href: "https://github.com/ericgbanta/crustle/",
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
