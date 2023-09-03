#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use super::utilities::*;
use dioxus::prelude::*;
use pokemon_rs;

const POKEMON_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
const SPECIES_URL: &str = "https://pokeapi.co/api/v2/pokemon-species/";

pub fn Pokedex(cx: Scope) -> Element {
    let pokemon_names = use_state(cx, || pokemon_rs::get_all(None));
    let selected_pokemon = use_state(cx, || String::from(""));
    let _pokemon_data = use_state(cx, || None::<Pokemon>);
    let _species_data = use_state(cx, || None::<PokemonSpecies>);
    let pokemon_url = format!("{}{}", POKEMON_URL, selected_pokemon.get());
    let species_url = format!("{}{}", SPECIES_URL, selected_pokemon.get());
    let pokemon_url_string = pokemon_url.clone();
    let pokemon_data = use_future(cx, &(pokemon_url), |pokemon_url: String| async move {
        reqwest::get(&pokemon_url).await?.json::<Pokemon>().await
    });
    let species_data = use_future(cx, &(species_url), |species_url: String| async move {
        reqwest::get(&species_url)
            .await?
            .json::<PokemonSpecies>()
            .await
    });

    if selected_pokemon.get().is_empty() {
        return cx.render(rsx! {
            div {
                class: "relative flex flex-col min-h-screen",
                Header {name:"Pokédex".into()},
                br {}
                br {}
                div {
                    class: "flex justify-center my-8 mx-8",
                    div {
                        class: "flex items-center border p-2 rounded",
                        span {
                            class: "mr-4 font-bold",
                            "Select a Pokémon:"
                        },
                        select {
                            class: "border rounded p-2 w-full text-lg",
                            onchange: move | event | {
                                let selected_name = event.inner().clone();
                                selected_pokemon.set(selected_name.value.clone().to_lowercase());
                                let _pokemon_url = format!("{}{}", POKEMON_URL, selected_name.value);
                                let _species_url = format!("{}{}", SPECIES_URL, selected_name.value);
                            },
                            option { "Select One" },
                            for &pokemon_name in pokemon_names.get().iter() {
                                option { pokemon_name }
                            }
                        }
                    }
                },
                div {
                    class: "flex justify-center my-4",
                    "Select a Pokémon to view its details."
                },
                About {},
            }
        });
    } else {
        cx.render(match (pokemon_data.value(), species_data.value()) {
    (Some(Ok(pokemon)), Some(Ok(species))) => {
        let english_flavor_texts: Vec<_> = species.flavor_text_entries
            .iter()
            .filter(|entry| entry.language.name == "en")
            .collect();

        let abilities_string = pokemon.abilities.iter().map(|pokemon_ability| {
            capitalize(&pokemon_ability.ability.name)
        }).collect::<Vec<String>>().join(", ");

        rsx! {
            div {
                class: "relative flex flex-col min-h-screen",
                Header {name:"Pokédex".into()},
                br {}
                br {}
                div {
                    class: "flex justify-center my-8 mx-8",
                    div {
                        class: "flex items-center border p-2 rounded",
                        span {
                            class: "mr-4 font-bold",
                            "Select a Pokémon:"
                        },
                        select {
                            class: "border rounded p-2 w-full text-lg",
                            onchange: move | event | {
                                let selected_name = event.inner().clone();
                                selected_pokemon.set(selected_name.value.clone().to_lowercase());
                                let _pokemon_url = format!("{}{}", POKEMON_URL, selected_name.value);
                                let _species_url = format!("{}{}", SPECIES_URL, selected_name.value);
                            },
                            option { "Select One" },
                            for &pokemon_name in pokemon_names.get().iter() {
                                option { pokemon_name }
                            }
                        }
                    }
                },
                div {
                    class: "flex justify-center",
                    img {
                        style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                        src: pokemon.sprites.front_default.as_deref().unwrap_or("URL not available")
                    },
                    img {
                        style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                        src: pokemon.sprites.front_shiny.as_deref().unwrap_or("URL not available")
                    }
                },
                br {}
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
                // Displaying types
                div {
                    class: "flex justify-center space-x-10",
                    for (index, pokemon_type) in pokemon.types.iter().enumerate() {
                        div {
                            class: "text-center",
                            strong { format!("Type {}: ", index + 1) }
                            capitalize(&pokemon_type.r#type.name)
                        }
                    }
                },
                div {
                        class: "flex justify-center",
                        div {
                            class: "text-center",
                            strong { "Abilities: " }
                            abilities_string
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
}
