#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use super::utilities::*;
use dioxus::prelude::*;
use pokemon_rs;

const POKEMON_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
const SPECIES_URL: &str = "https://pokeapi.co/api/v2/pokemon-species/";

pub fn Pokedex() -> Element {
    let pokemon_names = use_signal(|| pokemon_rs::get_all(None));
    let mut selected_pokemon = use_signal(|| String::from(""));
    let pokemon_url = format!("{}{}", POKEMON_URL, selected_pokemon.read());
    let species_url = format!("{}{}", SPECIES_URL, selected_pokemon.read());
    let pokemon_url_string = pokemon_url.clone();
    
    let pokemon_data = use_resource(move || {
        let url = pokemon_url.clone();
        async move {
            reqwest::get(&url).await?.json::<Pokemon>().await
        }
    });
    
    let species_data = use_resource(move || {
        let url = species_url.clone();
        async move {
            reqwest::get(&url)
                .await?
                .json::<PokemonSpecies>()
                .await
        }
    });

    if selected_pokemon.read().is_empty() {
        return rsx! {
            div {
                class: "relative flex flex-col min-h-screen",
                Header {name:"Pokédex".to_string()},
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
                                let selected_name = event.value();
                                selected_pokemon.set(selected_name.to_lowercase());
                            },
                            option { "Select One" },
                            for &pokemon_name in pokemon_names.read().iter() {
                                option { "{pokemon_name}" }
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
        }; // End of first if
    } else {
        let pokemon_read = pokemon_data.read();
        let species_read = species_data.read();
        match (pokemon_read.as_ref(), species_read.as_ref()) {
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
                Header {name:"Pokédex".to_string()},
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
                                let selected_name = event.value();
                                selected_pokemon.set(selected_name.to_lowercase());
                            },
                            option { "Select One" },
                            for &pokemon_name in pokemon_names.read().iter() {
                                option { "{pokemon_name}" }
                            }
                        }
                    }
                },
                div {
                    class: "flex justify-center",
                    img {
                        style: "transform: scale(2); margin-top: 20px;",
                        src: pokemon.sprites.front_default.as_deref().unwrap_or("URL not available")
                    },
                    img {
                        style: "transform: scale(2); margin-top: 20px; margin-left: 80px;",
                        src: pokemon.sprites.front_shiny.as_deref().unwrap_or("URL not available")
                    }
                },
                br {}
                div {
                    class: "flex justify-center space-x-10 mt-14",
                    div {
                        class: "text-center",
                        strong { "Height: " }
                        "{pokemon.height as f32 / 10.0} m"
                    },
                    div {
                        class: "text-center",
                        strong { "Weight: " }
                        "{pokemon.weight as f32 / 10.0} kg"
                    }
                },
                // Displaying types
                div {
                    class: "flex justify-center space-x-10",
                    for (index, pokemon_type) in pokemon.types.iter().enumerate() {
                        div {
                            class: "text-center",
                            strong { "Type {index + 1}: " }
                            {capitalize(&pokemon_type.r#type.name)}
                        }
                    }
                },
                div {
                        class: "flex justify-center",
                        div {
                            class: "text-center",
                            strong { "Abilities: " }
                            {abilities_string}
                        }
                },
                div {
                    for entry in english_flavor_texts {
                        div {
                            class: "bg-gray-100 m-4 p-4 rounded shadow",
                            h3 {
                                class: "text-lg font-bold",
                                "Version: {entry.version.name}"
                            },
                            p {
                                class: "text-gray-700",
                                {entry.flavor_text.clone()}
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
            "{pokemon_url_string}"
        }
    },
    (_, _) => rsx! {
        div {
            "Loading Pokémon data..."
            "{pokemon_url_string}"
        }
    },
    }
}
    }
