#![allow(non_snake_case)]

use super::about::About;
use super::header::Header;
use dioxus::prelude::*;
use pokemon_rs;

pub fn Pokedex(cx: Scope) -> Element {
    let pokemon_names = use_state(cx, || pokemon_rs::get_all(None));

    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Pokédex".into()},
            div {
                class: "flex justify-center my-8 mx-8", // Centering
                div {
                    class: "flex items-center border p-2 rounded", // Adding border
                    span {
                        class: "mr-4 font-bold", // Margin to the right for some spacing
                        "Select a Pokémon:"
                    },
                    select {
                        class: "border rounded p-2 w-full text-lg", // Bigger font
                        option { "Click" },
                        for &pokemon_name in pokemon_names.get().iter() {
                            option { pokemon_name }
                        }
                    }
                }
            },
            About {},
        }
    })
}
