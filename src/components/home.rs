#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use super::utilities::*;
use dioxus::prelude::*;

const POKEMON_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
const SPECIES_URL: &str = "https://pokeapi.co/api/v2/pokemon-species/";

pub fn Home() -> Element {
    let random_pokemon = use_memo(|| pokemon_rs::random(None).to_string());

    let pokemon_data = use_resource(move || {
        let name = random_pokemon().to_lowercase();
        async move {
            reqwest::get(format!("{}{}", POKEMON_URL, name))
                .await?
                .json::<Pokemon>()
                .await
        }
    });

    let species_data = use_resource(move || {
        let name = random_pokemon().to_lowercase();
        async move {
            reqwest::get(format!("{}{}", SPECIES_URL, name))
                .await?
                .json::<PokemonSpecies>()
                .await
        }
    });

    let pokemon = {
        let guard = pokemon_data.read();
        match &*guard {
            Some(Ok(p)) => Ok(Some(p.clone())),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    };

    let species = {
        let guard = species_data.read();
        match &*guard {
            Some(Ok(s)) => Ok(Some(s.clone())),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    };

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

            let id_str = format!("{:03}", pokemon.id);
            let height = format!("{} m", pokemon.height as f32 / 10.0);
            let weight = format!("{} kg", pokemon.weight as f32 / 10.0);
            let name = pokemon.name.clone();
            let types = pokemon.types.clone();

            rsx! {
                div {
                    class: "relative flex flex-col min-h-screen",
                    Header { name: "crustle".to_string() }
                    br {}
                    br {}
                    h2 {
                        class: "text-2xl text-center",
                        "Random Pokémon: "
                        strong { "{name}" }
                    }
                    div {
                        class: "flex justify-center",
                        img {
                            style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                            src: "https://raw.githubusercontent.com/HybridShivam/Pokemon/master/assets/images/{id_str}.png",
                        }
                    }
                    br {}
                    div {
                        class: "flex justify-center space-x-10",
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
                    About {}
                }
            }
        }
        _ => rsx! {
            div { "Loading Pokémon data..." }
        },
    }
}
