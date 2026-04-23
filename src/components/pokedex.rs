#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use super::utilities::*;
use dioxus::prelude::*;

const POKEMON_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
const SPECIES_URL: &str = "https://pokeapi.co/api/v2/pokemon-species/";

pub fn Pokedex() -> Element {
    let pokemon_names = use_memo(|| pokemon_rs::get_all(None));
    let mut selected_pokemon = use_signal(String::new);

    let pokemon_data = use_resource(move || {
        let name = selected_pokemon();
        async move {
            if name.is_empty() {
                return Ok(None);
            }
            reqwest::get(format!("{}{}", POKEMON_URL, name))
                .await?
                .json::<Pokemon>()
                .await
                .map(Some)
        }
    });

    let species_data = use_resource(move || {
        let name = selected_pokemon();
        async move {
            if name.is_empty() {
                return Ok(None);
            }
            reqwest::get(format!("{}{}", SPECIES_URL, name))
                .await?
                .json::<PokemonSpecies>()
                .await
                .map(Some)
        }
    });

    let pokemon = {
        let guard = pokemon_data.read();
        match &*guard {
            Some(Ok(Some(p))) => Ok(Some(p.clone())),
            Some(Ok(None)) => Ok(None),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    };

    let species = {
        let guard = species_data.read();
        match &*guard {
            Some(Ok(Some(s))) => Ok(Some(s.clone())),
            Some(Ok(None)) => Ok(None),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    };

    let names = pokemon_names();
    let selected = selected_pokemon();

    rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header { name: "Pokédex".to_string() }
            br {}
            br {}
            div {
                class: "flex justify-center my-8 mx-8",
                div {
                    class: "flex items-center border p-2 rounded",
                    span {
                        class: "mr-4 font-bold",
                        "Select a Pokémon:"
                    }
                    select {
                        class: "border rounded p-2 w-full text-lg",
                        onchange: move |event| {
                            let val = event.value().to_lowercase();
                            selected_pokemon.set(val);
                        },
                        option { value: "", "Select One" }
                        for name in names.iter() {
                            option { "{name}" }
                        }
                    }
                }
            }

            match (pokemon, species) {
                (Err(e), _) | (_, Err(e)) => rsx! {
                    div { "Failed to fetch Pokémon data: {e}" }
                },
                (Ok(Some(pokemon)), Ok(Some(species))) => {
                    let english_flavor_texts: Vec<_> = species
                        .flavor_text_entries
                        .iter()
                        .filter(|e| e.language.name == "en")
                        .cloned()
                        .collect();

                    let abilities_string = pokemon
                        .abilities
                        .iter()
                        .map(|pa| capitalize(&pa.ability.name))
                        .collect::<Vec<String>>()
                        .join(", ");

                    let height = format!("{} m", pokemon.height as f32 / 10.0);
                    let weight = format!("{} kg", pokemon.weight as f32 / 10.0);
                    let front_default = pokemon.sprites.front_default.clone().unwrap_or_default();
                    let front_shiny = pokemon.sprites.front_shiny.clone().unwrap_or_default();
                    let types = pokemon.types.clone();

                    rsx! {
                        div {
                            class: "flex justify-center",
                            img {
                                style: "transform: scale(2); margin-top: 20px;",
                                src: "{front_default}",
                            }
                            img {
                                style: "transform: scale(2); margin-top: 20px; margin-left: 80px;",
                                src: "{front_shiny}",
                            }
                        }
                        br {}
                        div {
                            class: "flex justify-center space-x-10 mt-14",
                            div {
                                class: "text-center",
                                strong { "Height: " }
                                "{height}"
                            }
                            div {
                                class: "text-center",
                                strong { "Weight: " }
                                "{weight}"
                            }
                        }
                        div {
                            class: "flex justify-center space-x-10",
                            for (index, pokemon_type) in types.iter().enumerate() {
                                div {
                                    class: "text-center",
                                    strong { "Type {index + 1}: " }
                                    {capitalize(&pokemon_type.r#type.name)}
                                }
                            }
                        }
                        div {
                            class: "flex justify-center",
                            div {
                                class: "text-center",
                                strong { "Abilities: " }
                                "{abilities_string}"
                            }
                        }
                        div {
                            for entry in english_flavor_texts {
                                div {
                                    class: "bg-gray-100 m-4 p-4 rounded shadow",
                                    h3 {
                                        class: "text-lg font-bold",
                                        "Version: {entry.version.name}"
                                    }
                                    p {
                                        class: "text-gray-700",
                                        "{entry.flavor_text}"
                                    }
                                }
                            }
                        }
                    }
                },
                _ if !selected.is_empty() => rsx! {
                    div {
                        class: "flex justify-center my-4",
                        "Loading Pokémon data..."
                    }
                },
                _ => rsx! {
                    div {
                        class: "flex justify-center my-4",
                        "Select a Pokémon to view its details."
                    }
                },
            }

            About {}
        }
    }
}
