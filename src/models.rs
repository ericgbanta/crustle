//! PokeAPI response models, limited to the fields the app reads, plus the combined
//! [`PokemonData`] the detail view renders.

use crate::format::normalize_flavor_text;
use serde::Deserialize;

/// `GET /pokemon/{id}`
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Pokemon {
    /// In decimetres.
    pub height: u32,
    /// In hectograms.
    pub weight: u32,
    pub abilities: Vec<PokemonAbility>,
    pub types: Vec<PokemonType>,
    pub sprites: PokemonSprites,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PokemonAbility {
    pub ability: NamedApiResource,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PokemonType {
    pub r#type: NamedApiResource,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PokemonSprites {
    front_default: Option<String>,
    front_shiny: Option<String>,
    #[serde(default)]
    other: Option<OtherSprites>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct OtherSprites {
    #[serde(rename = "official-artwork", default)]
    official_artwork: Option<Artwork>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Artwork {
    front_default: Option<String>,
}

impl PokemonSprites {
    pub fn front_default(&self) -> Option<&str> {
        non_empty(&self.front_default)
    }

    pub fn front_shiny(&self) -> Option<&str> {
        non_empty(&self.front_shiny)
    }

    /// The large illustration, as opposed to the in-game pixel sprites.
    pub fn official_artwork(&self) -> Option<&str> {
        let artwork = self.other.as_ref()?.official_artwork.as_ref()?;
        non_empty(&artwork.front_default)
    }
}

fn non_empty(url: &Option<String>) -> Option<&str> {
    url.as_deref().filter(|url| !url.is_empty())
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NamedApiResource {
    pub name: String,
}

/// `GET /pokemon-species/{id}`
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PokemonSpecies {
    pub flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: NamedApiResource,
    pub version: NamedApiResource,
}

impl PokemonSpecies {
    /// The English Pokédex entries, in API order, with their text normalized.
    pub fn into_english_flavor_texts(self) -> Vec<FlavorText> {
        self.flavor_text_entries
            .into_iter()
            .filter(|entry| entry.language.name == "en")
            .map(|entry| FlavorText {
                version: entry.version.name,
                text: normalize_flavor_text(&entry.flavor_text),
            })
            .collect()
    }
}

/// A Pokédex entry as written in one game version.
#[derive(Clone, Debug, PartialEq)]
pub struct FlavorText {
    /// PokeAPI version slug, e.g. `black-2`.
    pub version: String,
    pub text: String,
}

/// A Pokédex entry together with every game version that words it identically.
#[derive(Clone, Debug, PartialEq)]
pub struct PokedexEntry {
    pub text: String,
    /// PokeAPI version slugs, in the order the versions were listed.
    pub versions: Vec<String>,
}

/// Merges entries with identical text, keeping the order in which each text first
/// appears. Paired and re-released games mostly repeat one another word for word.
pub fn group_flavor_texts(flavor_texts: Vec<FlavorText>) -> Vec<PokedexEntry> {
    let mut entries: Vec<PokedexEntry> = Vec::new();
    for FlavorText { version, text } in flavor_texts {
        match entries.iter_mut().find(|entry| entry.text == text) {
            Some(entry) => {
                if !entry.versions.contains(&version) {
                    entry.versions.push(version);
                }
            }
            None => entries.push(PokedexEntry {
                text,
                versions: vec![version],
            }),
        }
    }
    entries
}

/// Everything the detail view shows for one Pokémon.
#[derive(Clone, Debug, PartialEq)]
pub struct PokemonData {
    pub pokemon: Pokemon,
    pub entries: Vec<PokedexEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Trimmed from https://pokeapi.co/api/v2/pokemon/558, keeping some fields the
    // models do not declare to show they are ignored.
    const POKEMON_JSON: &str = r#"{
        "id": 558,
        "name": "crustle",
        "base_experience": 170,
        "height": 14,
        "weight": 2000,
        "abilities": [
            {"ability": {"name": "sturdy", "url": "https://pokeapi.co/api/v2/ability/5/"}, "is_hidden": false, "slot": 1},
            {"ability": {"name": "shell-armor", "url": "https://pokeapi.co/api/v2/ability/75/"}, "is_hidden": false, "slot": 2},
            {"ability": {"name": "weak-armor", "url": "https://pokeapi.co/api/v2/ability/133/"}, "is_hidden": true, "slot": 3}
        ],
        "forms": [{"name": "crustle", "url": "https://pokeapi.co/api/v2/pokemon-form/558/"}],
        "sprites": {
            "back_default": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/back/558.png",
            "back_female": null,
            "front_default": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/558.png",
            "front_female": null,
            "front_shiny": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/558.png",
            "other": {
                "home": {"front_default": "https://example.invalid/home/558.png"},
                "official-artwork": {
                    "front_default": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/558.png",
                    "front_shiny": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/shiny/558.png"
                }
            }
        },
        "types": [
            {"slot": 1, "type": {"name": "bug", "url": "https://pokeapi.co/api/v2/type/7/"}},
            {"slot": 2, "type": {"name": "rock", "url": "https://pokeapi.co/api/v2/type/6/"}}
        ]
    }"#;

    // Trimmed from https://pokeapi.co/api/v2/pokemon-species/558 (entries 1, 2 and 4),
    // with an entry from /pokemon-species/25 for its soft hyphens and form feed.
    const SPECIES_JSON: &str = r#"{
        "id": 558,
        "name": "crustle",
        "capture_rate": 75,
        "flavor_text_entries": [
            {
                "flavor_text": "Competing for territory, Crustle\nfight viciously. The one whose boulder\nis broken is the loser of the battle.",
                "language": {"name": "en", "url": "https://pokeapi.co/api/v2/language/9/"},
                "version": {"name": "black", "url": "https://pokeapi.co/api/v2/version/17/"}
            },
            {
                "flavor_text": "Il se bat violemment contre ses\ncongénères pour son territoire.",
                "language": {"name": "fr", "url": "https://pokeapi.co/api/v2/language/5/"},
                "version": {"name": "black", "url": "https://pokeapi.co/api/v2/version/17/"}
            },
            {
                "flavor_text": "It raises its tail\nto check its sur\u00ad\nroundings. The\ftail is sometimes\nstruck by light\u00ad\nning in this pose.",
                "language": {"name": "en", "url": "https://pokeapi.co/api/v2/language/9/"},
                "version": {"name": "black-2", "url": "https://pokeapi.co/api/v2/version/21/"}
            }
        ]
    }"#;

    fn pokemon() -> Pokemon {
        serde_json::from_str(POKEMON_JSON).expect("pokemon fixture deserializes")
    }

    #[test]
    fn pokemon_deserializes_from_api_shape() {
        let pokemon = pokemon();
        assert_eq!(pokemon.height, 14);
        assert_eq!(pokemon.weight, 2000);

        let abilities: Vec<_> = pokemon
            .abilities
            .iter()
            .map(|a| a.ability.name.as_str())
            .collect();
        assert_eq!(abilities, ["sturdy", "shell-armor", "weak-armor"]);

        let types: Vec<_> = pokemon
            .types
            .iter()
            .map(|t| t.r#type.name.as_str())
            .collect();
        assert_eq!(types, ["bug", "rock"]);

        assert_eq!(
            pokemon.sprites.front_default(),
            Some(
                "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/558.png"
            )
        );
        assert_eq!(
            pokemon.sprites.front_shiny(),
            Some(
                "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/558.png"
            )
        );
    }

    #[test]
    fn missing_sprites_are_none() {
        let sprites: PokemonSprites =
            serde_json::from_str(r#"{"front_default": null, "front_shiny": ""}"#).unwrap();
        assert_eq!(sprites.front_default(), None);
        assert_eq!(sprites.front_shiny(), None);

        let sprites: PokemonSprites = serde_json::from_str("{}").unwrap();
        assert_eq!(sprites.front_default(), None);
    }

    #[test]
    fn official_artwork_is_read_from_the_nested_sprites() {
        assert_eq!(
            pokemon().sprites.official_artwork(),
            Some(
                "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/558.png"
            )
        );
    }

    #[test]
    fn missing_official_artwork_is_none() {
        for json in [
            "{}",
            r#"{"other": null}"#,
            r#"{"other": {}}"#,
            r#"{"other": {"official-artwork": null}}"#,
            r#"{"other": {"home": {"front_default": "x"}}}"#,
            r#"{"other": {"official-artwork": {}}}"#,
            r#"{"other": {"official-artwork": {"front_default": null}}}"#,
            r#"{"other": {"official-artwork": {"front_default": ""}}}"#,
        ] {
            let sprites: PokemonSprites = serde_json::from_str(json).unwrap();
            assert_eq!(sprites.official_artwork(), None, "{json}");
        }
    }

    fn flavor(version: &str, text: &str) -> FlavorText {
        FlavorText {
            version: version.into(),
            text: text.into(),
        }
    }

    #[test]
    fn identical_texts_are_grouped_with_their_versions() {
        let grouped = group_flavor_texts(vec![
            flavor("black", "It fights viciously."),
            flavor("white", "It fights viciously."),
            flavor("black-2", "It carries a boulder."),
            flavor("x", "It fights viciously."),
            flavor("white-2", "It carries a boulder."),
            flavor("sword", "Its boulder is heavy."),
        ]);
        assert_eq!(
            grouped,
            [
                PokedexEntry {
                    text: "It fights viciously.".into(),
                    versions: vec!["black".into(), "white".into(), "x".into()],
                },
                PokedexEntry {
                    text: "It carries a boulder.".into(),
                    versions: vec!["black-2".into(), "white-2".into()],
                },
                PokedexEntry {
                    text: "Its boulder is heavy.".into(),
                    versions: vec!["sword".into()],
                },
            ]
        );
    }

    #[test]
    fn grouping_does_not_repeat_a_version() {
        let grouped = group_flavor_texts(vec![flavor("red", "Same."), flavor("red", "Same.")]);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].versions, ["red"]);
    }

    #[test]
    fn grouping_nothing_gives_nothing() {
        assert_eq!(group_flavor_texts(Vec::new()), []);
    }

    #[test]
    fn species_deserializes_from_api_shape() {
        let species: PokemonSpecies = serde_json::from_str(SPECIES_JSON).unwrap();
        assert_eq!(species.flavor_text_entries.len(), 3);
        assert_eq!(species.flavor_text_entries[1].language.name, "fr");
        assert_eq!(species.flavor_text_entries[2].version.name, "black-2");
    }

    #[test]
    fn english_flavor_texts_are_filtered_and_normalized() {
        let species: PokemonSpecies = serde_json::from_str(SPECIES_JSON).unwrap();
        assert_eq!(
            species.into_english_flavor_texts(),
            [
                FlavorText {
                    version: "black".into(),
                    text: "Competing for territory, Crustle fight viciously. The one whose \
                           boulder is broken is the loser of the battle."
                        .into(),
                },
                FlavorText {
                    version: "black-2".into(),
                    text: "It raises its tail to check its surroundings. The tail is \
                           sometimes struck by lightning in this pose."
                        .into(),
                },
            ]
        );
    }
}
