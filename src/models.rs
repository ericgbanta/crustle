use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
    pub height: i32,
    pub weight: i32,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<NamedApiResource>,
    pub types: Vec<PokemonType>,
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

#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    pub slot: i32,
    pub is_hidden: bool,
    pub ability: NamedApiResource,
}

#[derive(Deserialize, Debug)]
pub struct PokemonType {
    pub slot: i32,
    pub r#type: NamedApiResource,
}

#[derive(Deserialize, Debug)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}
