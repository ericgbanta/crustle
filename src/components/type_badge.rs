use crate::format::title_case_slug;
use dioxus::prelude::*;

/// A pill naming one Pokémon type, in that type's color.
#[component]
pub fn TypeBadge(name: String) -> Element {
    let colors = type_colors(&name);
    let label = title_case_slug(&name);

    rsx! {
        span {
            class: "inline-flex items-center rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-wide {colors}",
            "{label}"
        }
    }
}

/// Background and text classes for a PokeAPI type slug.
///
/// Each arm is a complete literal because Tailwind only generates the classes it finds
/// spelled out in the source. The text color is whichever of white and near-black
/// reads better on that background.
fn type_colors(slug: &str) -> &'static str {
    match slug {
        "normal" => "bg-type-normal text-stone-950",
        "fire" => "bg-type-fire text-stone-950",
        "water" => "bg-type-water text-stone-950",
        "electric" => "bg-type-electric text-stone-950",
        "grass" => "bg-type-grass text-stone-950",
        "ice" => "bg-type-ice text-stone-950",
        "fighting" => "bg-type-fighting text-white",
        "poison" => "bg-type-poison text-white",
        "ground" => "bg-type-ground text-stone-950",
        "flying" => "bg-type-flying text-stone-950",
        "psychic" => "bg-type-psychic text-stone-950",
        "bug" => "bg-type-bug text-stone-950",
        "rock" => "bg-type-rock text-stone-950",
        "ghost" => "bg-type-ghost text-white",
        "dragon" => "bg-type-dragon text-white",
        "dark" => "bg-type-dark text-white",
        "steel" => "bg-type-steel text-stone-950",
        "fairy" => "bg-type-fairy text-stone-950",
        // Types PokeAPI may add later, and its non-battle "unknown" and "shadow".
        _ => "bg-sunken text-ink ring-1 ring-inset ring-line",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TYPES: [&str; 18] = [
        "normal", "fire", "water", "electric", "grass", "ice", "fighting", "poison", "ground",
        "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
    ];

    #[test]
    fn every_type_has_its_own_background() {
        let mut backgrounds: Vec<&str> = TYPES
            .iter()
            .map(|slug| {
                let background = type_colors(slug).split(' ').next().unwrap();
                assert_eq!(background, format!("bg-type-{slug}"));
                background
            })
            .collect();
        backgrounds.sort_unstable();
        backgrounds.dedup();
        assert_eq!(backgrounds.len(), TYPES.len());
    }

    #[test]
    fn unknown_types_get_a_neutral_badge() {
        assert_eq!(type_colors("stellar"), type_colors("unknown"));
        assert!(!type_colors("stellar").contains("bg-type-"));
    }
}
