#![allow(non_snake_case)]

// use super::super::models::*;
use dioxus::prelude::*;
//use pokemon_rs;
use super::about::About;
use super::header::Header;

// Pokedex component
pub fn Pokedex(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "relative flex flex-col min-h-screen",
            Header {name:"Pokédex".into()},
            About {},
        }
    })
}
