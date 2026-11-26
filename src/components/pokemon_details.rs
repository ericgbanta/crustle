use crate::api::{self, ApiError};
use crate::components::type_badge::TypeBadge;
use crate::dex::DexEntry;
use crate::format::{
    capitalize, dex_number, format_height, format_weight, title_case_slug, version_label,
};
use crate::models::{Pokemon, PokemonData};
use dioxus::prelude::*;

/// Which images the detail view shows, and how.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteLayout {
    /// One large picture: the official artwork, or the default sprite without it.
    Large,
    /// The default and shiny sprites side by side.
    DefaultAndShiny,
}

/// Where the request for one Pokémon stands.
#[derive(Clone, Copy)]
enum Fetch<'a> {
    Loading,
    Failed(&'a ApiError),
    Ready(&'a PokemonData),
}

const SKELETON: &str = "animate-pulse rounded-md bg-line motion-reduce:animate-none";
const LABEL: &str = "text-xs font-medium uppercase tracking-wide text-muted";

/// Loads and shows one Pokémon: pictures, types, height, weight, abilities and its
/// English Pokédex entries. Refetches when `entry` changes.
#[component]
pub fn PokemonDetails(entry: ReadSignal<DexEntry>, sprites: SpriteLayout) -> Element {
    // The response is stored with the id it was requested for. While a new request is
    // in flight the resource still holds the previous Pokémon's response, and the id
    // is what tells the two apart.
    let mut data = use_resource(move || async move {
        let id = entry().id;
        (id, api::fetch_pokemon_data(id).await)
    });

    let entry = entry();
    let response = data.read();
    let fetch = match &*response {
        Some((id, Ok(data))) if *id == entry.id => Fetch::Ready(data),
        Some((id, Err(error))) if *id == entry.id => Fetch::Failed(error),
        _ => Fetch::Loading,
    };

    rsx! {
        article {
            class: "overflow-hidden rounded-xl border border-line bg-surface",
            aria_busy: matches!(fetch, Fetch::Loading),
            if !matches!(fetch, Fetch::Failed(_)) {
                div {
                    class: "flex justify-center gap-6 border-b border-line bg-sunken px-4 py-6 sm:gap-12",
                    {pictures(entry, fetch, sprites)}
                }
            }
            div {
                class: "p-5 sm:p-6",
                div {
                    class: "flex flex-wrap items-end justify-between gap-x-6 gap-y-3",
                    div {
                        p { class: "font-mono text-sm text-muted", {dex_number(entry.id)} }
                        h2 { class: "text-2xl font-semibold tracking-tight", "{entry.name}" }
                    }
                    match fetch {
                        Fetch::Ready(data) => rsx! {
                            ul {
                                class: "flex flex-wrap gap-2",
                                aria_label: "Types",
                                for slot in &data.pokemon.types {
                                    li {
                                        key: "{slot.r#type.name}",
                                        class: "flex",
                                        TypeBadge { name: slot.r#type.name.clone() }
                                    }
                                }
                            }
                        },
                        Fetch::Loading => rsx! {
                            div { class: "h-6 w-28 {SKELETON}" }
                        },
                        Fetch::Failed(_) => rsx! {},
                    }
                }
                match fetch {
                    Fetch::Ready(data) => rsx! {
                        {stats(&data.pokemon)}
                        {pokedex_entries(data)}
                    },
                    Fetch::Loading => rsx! {
                        p { class: "sr-only", role: "status", "Loading {entry.name}" }
                        div {
                            class: "mt-6 space-y-3 border-t border-line pt-5",
                            aria_hidden: "true",
                            div { class: "h-4 w-2/3 {SKELETON}" }
                            div { class: "h-4 w-full {SKELETON}" }
                            div { class: "h-4 w-5/6 {SKELETON}" }
                            div { class: "h-4 w-1/2 {SKELETON}" }
                        }
                    },
                    Fetch::Failed(error) => rsx! {
                        div {
                            class: "mt-6 rounded-lg border border-danger/40 bg-danger-soft p-4",
                            role: "alert",
                            h3 { class: "font-semibold text-danger", "Could not load {entry.name}" }
                            p { class: "mt-1 text-sm break-words", {format!("{}.", capitalize(&error.to_string()))} }
                            button {
                                class: "mt-4 rounded-md border border-control bg-surface px-4 py-2 text-sm font-medium hover:bg-sunken",
                                r#type: "button",
                                // Dropping the failed response puts the card back in its
                                // loading state while the new request runs.
                                onclick: move |_| {
                                    data.clear();
                                    data.restart();
                                },
                                "Try again"
                            }
                        }
                    },
                }
            }
        }
    }
}

/// The picture area. Every state fills the same boxes, so the card keeps its height
/// while the data and then the images load.
fn pictures(entry: DexEntry, fetch: Fetch, layout: SpriteLayout) -> Element {
    let sprites = match fetch {
        Fetch::Ready(data) => Some(&data.pokemon.sprites),
        _ => None,
    };
    let name = entry.name;

    match layout {
        SpriteLayout::Large => {
            let artwork = sprites.and_then(|sprites| sprites.official_artwork());
            let sprite = sprites.and_then(|sprites| sprites.front_default());
            rsx! {
                div {
                    class: "flex aspect-square w-full max-w-56 items-center justify-center sm:max-w-72",
                    if let Some(src) = artwork {
                        img {
                            class: "size-full object-contain",
                            src,
                            alt: "Official artwork of {name}",
                            width: "475",
                            height: "475",
                        }
                    } else if let Some(src) = sprite {
                        img {
                            class: "pixelated size-full object-contain",
                            src,
                            alt: "{name} sprite",
                            width: "96",
                            height: "96",
                        }
                    } else {
                        {picture_placeholder(sprites.is_some())}
                    }
                }
            }
        }
        SpriteLayout::DefaultAndShiny => {
            let default = sprites.and_then(|sprites| sprites.front_default());
            let shiny = sprites.and_then(|sprites| sprites.front_shiny());
            rsx! {
                for (label, alt, src) in [
                    ("Default", format!("{name} sprite"), default),
                    ("Shiny", format!("Shiny {name} sprite"), shiny),
                ] {
                    figure {
                        key: "{label}",
                        class: "w-full max-w-36 sm:max-w-48",
                        div {
                            class: "flex aspect-square w-full items-center justify-center",
                            if let Some(src) = src {
                                img {
                                    class: "pixelated size-full object-contain",
                                    src,
                                    alt,
                                    width: "96",
                                    height: "96",
                                }
                            } else {
                                {picture_placeholder(sprites.is_some())}
                            }
                        }
                        figcaption { class: "mt-2 text-center {LABEL}", "{label}" }
                    }
                }
            }
        }
    }
}

/// What a picture box holds when it has no image: a pulse while the data loads, or a
/// note once it is known that PokeAPI has no such image.
fn picture_placeholder(loaded: bool) -> Element {
    if loaded {
        rsx! {
            p { class: "text-center text-sm text-muted", "No image available" }
        }
    } else {
        rsx! {
            div { class: "size-2/3 {SKELETON}" }
        }
    }
}

fn stats(pokemon: &Pokemon) -> Element {
    rsx! {
        dl {
            class: "mt-6 grid grid-cols-2 gap-x-6 gap-y-4 border-t border-line pt-5 sm:grid-cols-[auto_auto_1fr] sm:gap-x-12",
            div {
                dt { class: LABEL, "Height" }
                dd { class: "mt-1 text-lg", {format_height(pokemon.height)} }
            }
            div {
                dt { class: LABEL, "Weight" }
                dd { class: "mt-1 text-lg", {format_weight(pokemon.weight)} }
            }
            div {
                class: "col-span-2 sm:col-span-1",
                dt { class: LABEL, "Abilities" }
                dd { class: "mt-1 text-lg", {ability_list(pokemon)} }
            }
        }
    }
}

fn pokedex_entries(data: &PokemonData) -> Element {
    rsx! {
        section {
            class: "mt-6 border-t border-line pt-5",
            h3 { class: "text-lg font-semibold", "Pokédex entries" }
            if data.entries.is_empty() {
                p { class: "mt-2 text-muted", "No English Pokédex entries yet." }
            }
            ul {
                class: "mt-1 divide-y divide-line",
                for entry in &data.entries {
                    li {
                        key: "{entry.text}",
                        class: "py-4 last:pb-0",
                        p { class: "leading-relaxed", "{entry.text}" }
                        ul {
                            class: "mt-2.5 flex flex-wrap gap-1.5",
                            aria_label: "Game versions",
                            for version in &entry.versions {
                                li {
                                    key: "{version}",
                                    class: "rounded-md bg-sunken px-2 py-0.5 text-xs text-muted ring-1 ring-inset ring-line",
                                    {version_label(version)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The abilities as a readable list. PokeAPI lists an ability twice when a Pokémon
/// has it both as a regular and as its hidden ability.
fn ability_list(pokemon: &Pokemon) -> String {
    let mut names: Vec<String> = Vec::new();
    for slot in &pokemon.abilities {
        let name = title_case_slug(&slot.ability.name);
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names.join(", ")
}
