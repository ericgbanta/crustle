//! The national Pokédex as known to `pokemon-rs`, which supplies the display names.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DexEntry {
    /// National Pokédex number, which is also the PokeAPI id of the species.
    pub id: u32,
    pub name: &'static str,
}

// `pokemon_rs::get_all` is in national Pokédex order starting at #1, so the number is
// the position plus one.
fn id_for_index(index: usize) -> u32 {
    u32::try_from(index + 1).expect("Pokédex numbers fit in u32")
}

/// Every Pokémon in Pokédex order.
pub fn entries() -> impl Iterator<Item = DexEntry> {
    pokemon_rs::get_all(None)
        .into_iter()
        .enumerate()
        .map(|(index, name)| DexEntry {
            id: id_for_index(index),
            name,
        })
}

pub fn random_entry() -> DexEntry {
    let name = pokemon_rs::random(None);
    entries()
        .find(|entry| entry.name == name)
        .expect("pokemon_rs::random returns a name from pokemon_rs::get_all")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id_of(name: &str) -> Option<u32> {
        entries().find(|entry| entry.name == name).map(|e| e.id)
    }

    #[test]
    fn index_maps_to_one_based_id() {
        assert_eq!(id_for_index(0), 1);
        assert_eq!(id_for_index(557), 558);
    }

    #[test]
    fn ids_are_sequential_from_one() {
        let ids: Vec<u32> = entries().map(|entry| entry.id).collect();
        assert!(ids.len() >= 1008);
        assert!(ids.iter().copied().eq(1..=ids.len() as u32));
    }

    #[test]
    fn names_resolve_to_national_dex_numbers() {
        assert_eq!(id_of("Bulbasaur"), Some(1));
        assert_eq!(id_of("Farfetch'd"), Some(83));
        assert_eq!(id_of("Mr. Mime"), Some(122));
        assert_eq!(id_of("Deoxys"), Some(386));
        assert_eq!(id_of("Crustle"), Some(558));
        assert_eq!(id_of("Type: Null"), Some(772));
        assert_eq!(id_of("Mimikyu"), Some(778));
        assert_eq!(id_of("Iron Valiant"), Some(1006));
        assert_eq!(id_of("Missingno"), None);
    }

    #[test]
    fn names_are_unique() {
        let mut names: Vec<_> = entries().map(|entry| entry.name).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total);
    }
}
