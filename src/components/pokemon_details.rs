use crate::api;
use crate::format::{capitalize, format_height, format_weight};
use crate::models::{Pokemon, PokemonData};
use dioxus::prelude::*;

/// Which sprites the detail view shows, and how.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteLayout {
    /// The default sprite alone, at up to 500px.
    Large,
    /// The default and shiny sprites side by side at double size.
    DefaultAndShiny,
}

/// Loads and shows one Pokémon: sprites, height and weight, types, abilities and its
/// English Pokédex entries. Refetches when `id` (a national Pokédex number) changes.
#[component]
pub fn PokemonDetails(id: ReadSignal<u32>, sprites: SpriteLayout) -> Element {
    let data = use_resource(move || api::fetch_pokemon_data(id()));

    match &*data.read() {
        None => rsx! {
            div {
                class: "flex justify-center my-4",
                "Loading Pokémon data..."
            }
        },
        Some(Err(error)) => rsx! {
            div { "Failed to fetch Pokémon data: {error}" }
        },
        Some(Ok(data)) => details(data, sprites),
    }
}

fn details(data: &PokemonData, sprite_layout: SpriteLayout) -> Element {
    let pokemon = &data.pokemon;
    let height = format_height(pokemon.height);
    let weight = format_weight(pokemon.weight);
    let abilities = ability_list(pokemon);
    let front_default = pokemon.sprites.front_default();
    let front_shiny = pokemon.sprites.front_shiny();
    let stats_class = match sprite_layout {
        SpriteLayout::Large => "flex justify-center space-x-10",
        // Leaves room for the sprites, which are scaled past their layout box.
        SpriteLayout::DefaultAndShiny => "flex justify-center space-x-10 mt-14",
    };

    rsx! {
        div {
            class: "flex justify-center",
            match sprite_layout {
                SpriteLayout::Large => rsx! {
                    if let Some(src) = front_default {
                        img { class: "max-w-[500px] max-h-[500px] mt-5", src }
                    }
                },
                SpriteLayout::DefaultAndShiny => rsx! {
                    if let Some(src) = front_default {
                        img { class: "scale-200 mt-5", src }
                    }
                    if let Some(src) = front_shiny {
                        img { class: "scale-200 mt-5 ml-20", src }
                    }
                },
            }
        }
        br {}
        div {
            class: stats_class,
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
            for (index, pokemon_type) in pokemon.types.iter().enumerate() {
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
                "{abilities}"
            }
        }
        div {
            for entry in &data.flavor_texts {
                div {
                    class: "bg-gray-100 m-4 p-4 rounded-sm shadow-sm",
                    h3 {
                        class: "text-lg font-bold",
                        "Version: {entry.version}"
                    }
                    p {
                        class: "text-gray-700",
                        "{entry.text}"
                    }
                }
            }
        }
    }
}

fn ability_list(pokemon: &Pokemon) -> String {
    pokemon
        .abilities
        .iter()
        .map(|slot| capitalize(&slot.ability.name))
        .collect::<Vec<_>>()
        .join(", ")
}
