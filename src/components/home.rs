#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use super::utilities::*;
use dioxus::prelude::*;
use pokemon_rs;

// Home component
pub fn Home() -> Element {
    let random_pokemon = use_signal(|| pokemon_rs::random(None).to_string());
    let random_pokemon_id = use_signal(|| {
        pokemon_rs::get_id_by_name(&random_pokemon.read(), None).to_string()
    });
    let id_str = format!("{:03}", random_pokemon_id.read().parse::<u32>().unwrap_or(0));
    let pokemon_url = format!(
        "https://pokeapi.co/api/v2/pokemon/{}",
        random_pokemon.read().to_lowercase()
    );
    let species_url = format!(
        "https://pokeapi.co/api/v2/pokemon-species/{}",
        random_pokemon.read().to_lowercase()
    );

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
                Header {name:"crustle".to_string()},
                br {}
                br {}
                h2 {
                    class: "text-2xl text-center",
                    "Random Pokémon:"
                    strong { "{random_pokemon.read()}" }
                },
                div {
                    class: "flex justify-center",
                    img {
                        style: "max-width: 500px; max-height: 500px; margin-top: 20px;",
                        src: "https://raw.githubusercontent.com/HybridShivam/Pokemon/master/assets/images/{id_str}.png",
                    }
                },
                br {}
                div {
                    class: "flex justify-center space-x-10",
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
            p { "{pokemon_url_string}" }
        }
    },
    (_, _) => rsx! {
        div {
            "Loading Pokémon data..."
            p { "{pokemon_url_string}" }
        }
    },
}
}
