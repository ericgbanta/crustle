use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
    pub height: i32,
    pub weight: i32,
}

#[derive(Deserialize, Debug)]
pub struct PokemonSpecies {
    pub flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: String,
}
