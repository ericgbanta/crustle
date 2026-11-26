//! Forgiving name and number search over the Pokédex, for the picker.

use crate::dex::DexEntry;

/// The best matches for a query, in ranked order, cut off at the requested limit.
#[derive(Clone, Debug, PartialEq)]
pub struct Matches {
    pub entries: Vec<DexEntry>,
    /// How many entries matched before the limit was applied.
    pub total: usize,
}

/// Finds entries by name or national Pokédex number.
///
/// Names match regardless of case, accents, spaces and punctuation, so "flabebe",
/// "mr mime" and "farfetchd" find Flabébé, Mr. Mime and Farfetch'd. A query made of
/// digits (optionally written "#25" or "0025") matches the dex number. Better matches
/// come first, and ties stay in Pokédex order. A blank query matches everything.
pub fn search(entries: impl IntoIterator<Item = DexEntry>, query: &str, limit: usize) -> Matches {
    let query = Query::parse(query);
    let mut ranked: Vec<(u8, DexEntry)> = entries
        .into_iter()
        .filter_map(|entry| Some((query.rank(entry)?, entry)))
        .collect();
    // Stable, so entries of equal rank keep the order they came in.
    ranked.sort_by_key(|(rank, _)| *rank);

    Matches {
        total: ranked.len(),
        entries: ranked
            .into_iter()
            .take(limit)
            .map(|(_, entry)| entry)
            .collect(),
    }
}

enum Query {
    Blank,
    /// The digits of a dex number without leading zeros; empty if it was all zeros.
    Number(String),
    /// Folded text, never empty.
    Text(String),
    /// Something was typed, but nothing a name could contain.
    Unmatchable,
}

impl Query {
    fn parse(raw: &str) -> Self {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Self::Blank;
        }

        let digits = trimmed.strip_prefix('#').unwrap_or(trimmed).trim_start();
        if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
            return Self::Number(digits.trim_start_matches('0').to_string());
        }

        let folded = fold(trimmed);
        if folded.is_empty() {
            Self::Unmatchable
        } else {
            Self::Text(folded)
        }
    }

    /// Lower is better; `None` means the entry does not match.
    fn rank(&self, entry: DexEntry) -> Option<u8> {
        match self {
            Self::Blank => Some(0),
            Self::Unmatchable => None,
            Self::Number(digits) if digits.is_empty() => None,
            Self::Number(digits) => {
                let id = entry.id.to_string();
                if id == *digits {
                    Some(0)
                } else if id.starts_with(digits.as_str()) {
                    Some(1)
                } else {
                    None
                }
            }
            Self::Text(query) => {
                let words: Vec<String> = entry
                    .name
                    .split(|c: char| c.is_whitespace() || c == '-')
                    .map(fold)
                    .filter(|word| !word.is_empty())
                    .collect();
                let name = words.concat();

                if name == *query {
                    Some(0)
                } else if name.starts_with(query.as_str()) {
                    Some(1)
                } else if (1..words.len())
                    .any(|start| words[start..].concat().starts_with(query.as_str()))
                {
                    // Starts at a later word: "mime" finds Mr. Mime, "null" Type: Null.
                    Some(2)
                } else if name.contains(query.as_str()) {
                    Some(3)
                } else {
                    None
                }
            }
        }
    }
}

/// Reduces text to lowercase ASCII letters and digits: accents are removed, the gender
/// signs are spelled out, and spaces and punctuation are dropped.
fn fold(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '♀' => folded.push_str("female"),
            '♂' => folded.push_str("male"),
            _ => folded.extend(
                c.to_lowercase()
                    .map(strip_accent)
                    .filter(char::is_ascii_alphanumeric),
            ),
        }
    }
    folded
}

fn strip_accent(c: char) -> char {
    match c {
        'à'..='å' => 'a',
        'ç' => 'c',
        'è'..='ë' => 'e',
        'ì'..='ï' => 'i',
        'ñ' => 'n',
        'ò'..='ö' => 'o',
        'ù'..='ü' => 'u',
        'ý' | 'ÿ' => 'y',
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex;

    fn names(query: &str, limit: usize) -> Vec<&'static str> {
        search(dex::entries(), query, limit)
            .entries
            .iter()
            .map(|entry| entry.name)
            .collect()
    }

    fn first(query: &str) -> Option<&'static str> {
        names(query, 1).first().copied()
    }

    #[test]
    fn fold_keeps_only_plain_letters_and_digits() {
        assert_eq!(fold("Flabébé"), "flabebe");
        assert_eq!(fold("Mr. Mime"), "mrmime");
        assert_eq!(fold("Farfetch'd"), "farfetchd");
        assert_eq!(fold("Sirfetch’d"), "sirfetchd");
        assert_eq!(fold("Type: Null"), "typenull");
        assert_eq!(fold("Porygon-Z"), "porygonz");
        assert_eq!(fold("Nidoran♀"), "nidoranfemale");
        assert_eq!(fold("Nidoran♂"), "nidoranmale");
        assert_eq!(fold("ÀÉÎÕÜ"), "aeiou");
        assert_eq!(fold(" .-'"), "");
    }

    #[test]
    fn matching_ignores_case() {
        assert_eq!(first("crustle"), Some("Crustle"));
        assert_eq!(first("CRUSTLE"), Some("Crustle"));
        assert_eq!(first("cRuS"), Some("Crustle"));
    }

    #[test]
    fn matching_ignores_accents_spaces_and_punctuation() {
        assert_eq!(first("flabebe"), Some("Flabébé"));
        assert_eq!(first("Flabébé"), Some("Flabébé"));
        assert_eq!(first("mr mime"), Some("Mr. Mime"));
        assert_eq!(first("mr.mime"), Some("Mr. Mime"));
        assert_eq!(first("farfetchd"), Some("Farfetch'd"));
        assert_eq!(first("farfetch'd"), Some("Farfetch'd"));
        assert_eq!(first("sirfetch'd"), Some("Sirfetch’d"));
        assert_eq!(first("type null"), Some("Type: Null"));
        assert_eq!(first("hooh"), Some("Ho-Oh"));
        assert_eq!(first("ho oh"), Some("Ho-Oh"));
        assert_eq!(first("porygon z"), Some("Porygon-Z"));
        assert_eq!(first("mime jr"), Some("Mime Jr."));
    }

    #[test]
    fn gender_signs_can_be_typed_as_words() {
        assert_eq!(names("nidoran", 2), ["Nidoran♀", "Nidoran♂"]);
        assert_eq!(names("nidoran f", 8), ["Nidoran♀"]);
        assert_eq!(names("nidoran male", 8), ["Nidoran♂"]);
        assert_eq!(names("Nidoran♂", 8), ["Nidoran♂"]);
    }

    #[test]
    fn numbers_match_the_dex_number() {
        assert_eq!(first("386"), Some("Deoxys"));
        assert_eq!(first("#558"), Some("Crustle"));
        assert_eq!(first("# 25"), Some("Pikachu"));
        assert_eq!(first("0025"), Some("Pikachu"));
        assert_eq!(first(" 1 "), Some("Bulbasaur"));
    }

    #[test]
    fn an_exact_number_comes_before_numbers_it_prefixes() {
        assert_eq!(names("15", 4), ["Beedrill", "Mewtwo", "Mew", "Chikorita"]);
        let matches = search(dex::entries(), "15", 4);
        // 15, 150..=159
        assert_eq!(matches.total, 11);
    }

    #[test]
    fn digits_inside_a_name_are_matched_as_text() {
        assert_eq!(first("porygon2"), Some("Porygon2"));
        assert_eq!(first("porygon 2"), Some("Porygon2"));
    }

    #[test]
    fn zero_and_out_of_range_numbers_match_nothing() {
        assert_eq!(search(dex::entries(), "0", 8).total, 0);
        assert_eq!(search(dex::entries(), "000", 8).total, 0);
        assert_eq!(search(dex::entries(), "99999", 8).total, 0);
    }

    #[test]
    fn better_matches_come_first() {
        // Exact name, then names starting with the query, then later words, then
        // anywhere in the name.
        assert_eq!(names("mew", 2), ["Mew", "Mewtwo"]);
        assert_eq!(first("mime"), Some("Mime Jr."));
        assert_eq!(names("mime", 3), ["Mime Jr.", "Mr. Mime"]);
        assert_eq!(first("null"), Some("Type: Null"));
        assert_eq!(first("valiant"), Some("Iron Valiant"));
        let saur = names("saur", 8);
        assert_eq!(saur, ["Bulbasaur", "Ivysaur", "Venusaur"]);
    }

    #[test]
    fn ties_stay_in_pokedex_order() {
        assert_eq!(names("char", 3), ["Charmander", "Charmeleon", "Charizard"]);
    }

    #[test]
    fn blank_query_lists_the_dex_from_the_start() {
        let matches = search(dex::entries(), "  ", 3);
        assert_eq!(matches.total, dex::entries().count());
        let names: Vec<_> = matches.entries.iter().map(|entry| entry.name).collect();
        assert_eq!(names, ["Bulbasaur", "Ivysaur", "Venusaur"]);
    }

    #[test]
    fn limit_caps_entries_but_not_total() {
        let matches = search(dex::entries(), "pi", 5);
        assert_eq!(matches.entries.len(), 5);
        assert!(matches.total > 5);
    }

    #[test]
    fn unmatched_queries_return_nothing() {
        for query in ["missingno", "zzzz", "?!", "#", "-"] {
            let matches = search(dex::entries(), query, 8);
            assert_eq!(matches.total, 0, "{query}");
            assert!(matches.entries.is_empty(), "{query}");
        }
    }

    #[test]
    fn every_name_finds_itself_first() {
        for entry in dex::entries() {
            assert_eq!(first(entry.name), Some(entry.name));
        }
    }
}
