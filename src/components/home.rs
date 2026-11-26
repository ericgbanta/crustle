use crate::components::layout::PageLayout;
use crate::components::pokemon_details::{PokemonDetails, SpriteLayout};
use crate::dex;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let pokemon = use_hook(dex::random_entry);

    rsx! {
        PageLayout {
            title: "Random Pokémon",
            intro: "Reload the page to meet another one.",
            PokemonDetails { entry: pokemon, sprites: SpriteLayout::Large }
        }
    }
}
