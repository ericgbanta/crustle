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
    let pokemon_data = use_state(cx, || None::<Pokemon>);
    let species_data = use_state(cx, || None::<PokemonSpecies>);
    let pokemon_url = format!("{}{}", POKEMON_URL, selected_pokemon.get());

    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Pokédex".into()},
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
                            let pokemon_url = format!("{}{}", POKEMON_URL, selected_name.value);
                            let species_url = format!("{}{}", SPECIES_URL, selected_name.value);
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
                "Request URL for Pokémon: {pokemon_url}"
            },
            About {},
        }
    })
}
