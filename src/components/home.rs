use crate::components::layout::PageLayout;
use crate::components::pokemon_details::{PokemonDetails, SpriteLayout};
use crate::dex;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let pokemon = use_hook(dex::random_entry);

    rsx! {
        PageLayout {
            title: "crustle",
            br {}
            br {}
            h2 {
                class: "text-2xl text-center",
                "Random Pokémon: "
                strong { "{pokemon.name}" }
            }
            PokemonDetails { id: pokemon.id, sprites: SpriteLayout::Large }
        }
    }
}
