//! PokeAPI client: the only module that builds URLs or talks to the network.

use crate::models::{Pokemon, PokemonData, PokemonSpecies};
use futures_util::future::try_join;
use serde::de::DeserializeOwned;
use std::fmt;

const BASE_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Debug)]
pub struct ApiError(reqwest::Error);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(status) = self.0.status() {
            write!(f, "PokéAPI responded with {status}")
        } else if self.0.is_decode() {
            write!(f, "PokéAPI sent a response in an unexpected format")
        } else {
            write!(f, "could not reach PokéAPI ({})", self.0)
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self(error)
    }
}

// Resources are addressed by national Pokédex number rather than by name: names need
// slug rules ("Mr. Mime" is `mr-mime`), and `/pokemon/{name}` 404s for species whose
// default form carries a suffix (`deoxys-normal`), whereas ids resolve for every species.
fn resource_url(resource: &str, dex_id: u32) -> String {
    format!("{BASE_URL}/{resource}/{dex_id}")
}

async fn get_json<T: DeserializeOwned>(url: String) -> Result<T, ApiError> {
    let response = reqwest::get(url).await?.error_for_status()?;
    Ok(response.json().await?)
}

pub async fn fetch_pokemon(dex_id: u32) -> Result<Pokemon, ApiError> {
    get_json(resource_url("pokemon", dex_id)).await
}

pub async fn fetch_species(dex_id: u32) -> Result<PokemonSpecies, ApiError> {
    get_json(resource_url("pokemon-species", dex_id)).await
}

/// Fetches everything the detail view shows, running both requests concurrently.
pub async fn fetch_pokemon_data(dex_id: u32) -> Result<PokemonData, ApiError> {
    let (pokemon, species) = try_join(fetch_pokemon(dex_id), fetch_species(dex_id)).await?;
    Ok(PokemonData {
        pokemon,
        flavor_texts: species.into_english_flavor_texts(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_use_the_dex_id() {
        assert_eq!(
            resource_url("pokemon", 122),
            "https://pokeapi.co/api/v2/pokemon/122"
        );
        assert_eq!(
            resource_url("pokemon-species", 386),
            "https://pokeapi.co/api/v2/pokemon-species/386"
        );
    }
}
