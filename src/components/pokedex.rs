use crate::components::layout::PageLayout;
use crate::components::pokemon_details::{PokemonDetails, SpriteLayout};
use crate::components::pokemon_picker::PokemonPicker;
use crate::dex::DexEntry;
use dioxus::prelude::*;

#[component]
pub fn Pokedex() -> Element {
    let mut selected = use_signal(|| None::<DexEntry>);

    rsx! {
        PageLayout {
            title: "Pokédex",
            intro: "Look up any Pokémon by name or number.",
            PokemonPicker { on_select: move |entry| selected.set(Some(entry)) }
            div {
                class: "mt-8",
                if let Some(entry) = selected() {
                    PokemonDetails { entry, sprites: SpriteLayout::DefaultAndShiny }
                } else {
                    p {
                        class: "rounded-xl border border-dashed border-control px-5 py-12 text-center text-muted",
                        "Select a Pokémon to view its details."
                    }
                }
            }
        }
    }
}
