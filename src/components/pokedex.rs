use crate::components::layout::PageLayout;
use crate::components::pokemon_details::{PokemonDetails, SpriteLayout};
use crate::dex;
use dioxus::prelude::*;

#[component]
pub fn Pokedex() -> Element {
    // Dex id of the chosen Pokémon; `None` while the placeholder option is selected.
    let mut selected = use_signal(|| None::<u32>);

    rsx! {
        PageLayout {
            title: "Pokédex",
            br {}
            br {}
            div {
                class: "flex justify-center my-8 mx-8",
                div {
                    class: "flex items-center border border-gray-200 p-2 rounded-sm",
                    span {
                        class: "mr-4 font-bold",
                        "Select a Pokémon:"
                    }
                    select {
                        class: "border border-gray-200 rounded-sm p-2 w-full text-lg",
                        onchange: move |event| selected.set(event.value().parse().ok()),
                        option { value: "", "Select One" }
                        for entry in dex::entries() {
                            option { key: "{entry.id}", value: "{entry.id}", "{entry.name}" }
                        }
                    }
                }
            }

            if let Some(id) = selected() {
                PokemonDetails { id, sprites: SpriteLayout::DefaultAndShiny }
            } else {
                div {
                    class: "flex justify-center my-4",
                    "Select a Pokémon to view its details."
                }
            }
        }
    }
}
