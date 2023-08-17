#![allow(non_snake_case)]

use super::super::models::*;
use super::about::About;
use super::header::Header;
use dioxus::prelude::*;
use pokemon_rs;

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

        let abilities_string = pokemon.abilities.iter().map(|pokemon_ability| {
            pokemon_ability.ability.name.clone()
        }).collect::<Vec<String>>().join(", ");

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
                            pokemon_type.r#type.name.clone()
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
