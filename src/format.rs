//! Pure text helpers for presenting PokeAPI values.

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Turns a PokeAPI slug into a title: `shell-armor` becomes "Shell Armor" and
/// `power-of-alchemy` "Power of Alchemy". Small words stay lowercase unless they
/// come first.
pub fn title_case_slug(slug: &str) -> String {
    const SMALL_WORDS: [&str; 6] = ["of", "to", "as", "the", "and", "in"];

    // An initialism, which no casing rule would get right.
    if slug == "rks-system" {
        return "RKS System".to_string();
    }
    slug.split('-')
        .filter(|word| !word.is_empty())
        .enumerate()
        .map(|(index, word)| {
            if index > 0 && SMALL_WORDS.contains(&word) {
                word.to_string()
            } else {
                capitalize(word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The display name of a game version, from its PokeAPI slug.
pub fn version_label(slug: &str) -> String {
    // Titles that are not simply their slug in title case.
    let special = match slug {
        "firered" => "FireRed",
        "leafgreen" => "LeafGreen",
        "heartgold" => "HeartGold",
        "soulsilver" => "SoulSilver",
        "xd" => "XD",
        "lets-go-pikachu" => "Let's Go, Pikachu!",
        "lets-go-eevee" => "Let's Go, Eevee!",
        "legends-arceus" => "Legends: Arceus",
        "legends-za" => "Legends: Z-A",
        _ => return title_case_slug(slug),
    };
    special.to_string()
}

/// A national Pokédex number the way the games print it: `#0558`.
pub fn dex_number(id: u32) -> String {
    format!("#{id:04}")
}

/// PokeAPI reports height in decimetres.
pub fn format_height(decimetres: u32) -> String {
    format!("{} m", tenths(decimetres))
}

/// PokeAPI reports weight in hectograms.
pub fn format_weight(hectograms: u32) -> String {
    format!("{} kg", tenths(hectograms))
}

fn tenths(value: u32) -> String {
    format!("{}.{}", value / 10, value % 10)
}

/// Turns PokeAPI flavor text, which keeps the games' hard line breaks, into a single
/// line of prose.
///
/// Newlines and form feeds (page breaks) become single spaces. A soft hyphen marks a
/// word that was split across lines, so it is removed together with the break after it.
/// A real hyphen that ends a line belongs to a compound split there ("bluish-\nwhite"),
/// so it is kept and only the break is removed.
pub fn normalize_flavor_text(raw: &str) -> String {
    const SOFT_HYPHEN: char = '\u{ad}';

    let mut text = String::with_capacity(raw.len());
    let mut joining_word = false;
    let mut pending_space = false;
    // The last character was a hyphen attached to a word, or the `\r` of a `\r\n`
    // right after one.
    let mut after_hyphen = false;
    for c in raw.chars() {
        if c == SOFT_HYPHEN {
            joining_word = true;
            after_hyphen = false;
        } else if c.is_whitespace() {
            if after_hyphen && matches!(c, '\n' | '\x0c') {
                joining_word = true;
            }
            after_hyphen = after_hyphen && c == '\r';
            pending_space = !joining_word;
        } else {
            // A hyphen with a space before it is a dash between words, not part of one.
            after_hyphen = c == '-' && !pending_space && !text.is_empty();
            if pending_space && !text.is_empty() {
                text.push(' ');
            }
            text.push(c);
            pending_space = false;
            joining_word = false;
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_uppercases_only_the_first_letter() {
        assert_eq!(capitalize("bug"), "Bug");
        assert_eq!(capitalize("shell-armor"), "Shell-armor");
        assert_eq!(capitalize("Rock"), "Rock");
        assert_eq!(capitalize("élan"), "Élan");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn slugs_read_as_words() {
        assert_eq!(title_case_slug("shell-armor"), "Shell Armor");
        assert_eq!(title_case_slug("sturdy"), "Sturdy");
        assert_eq!(title_case_slug("black-2"), "Black 2");
        assert_eq!(title_case_slug("-odd--slug-"), "Odd Slug");
        assert_eq!(title_case_slug(""), "");
    }

    #[test]
    fn small_words_stay_lowercase_unless_first() {
        assert_eq!(title_case_slug("power-of-alchemy"), "Power of Alchemy");
        assert_eq!(title_case_slug("good-as-gold"), "Good as Gold");
        assert_eq!(title_case_slug("beads-of-ruin"), "Beads of Ruin");
        assert_eq!(title_case_slug("zero-to-hero"), "Zero to Hero");
        assert_eq!(title_case_slug("as-one-glastrier"), "As One Glastrier");
        assert_eq!(title_case_slug("the-isle-of-armor"), "The Isle of Armor");
        assert_eq!(title_case_slug("-of-ruin"), "Of Ruin");
    }

    #[test]
    fn initialisms_keep_their_capitals() {
        assert_eq!(title_case_slug("rks-system"), "RKS System");
    }

    #[test]
    fn version_labels_read_as_game_titles() {
        assert_eq!(version_label("black-2"), "Black 2");
        assert_eq!(version_label("omega-ruby"), "Omega Ruby");
        assert_eq!(version_label("ultra-sun"), "Ultra Sun");
        assert_eq!(version_label("x"), "X");
        assert_eq!(version_label("firered"), "FireRed");
        assert_eq!(version_label("heartgold"), "HeartGold");
        assert_eq!(version_label("xd"), "XD");
        assert_eq!(version_label("lets-go-eevee"), "Let's Go, Eevee!");
        assert_eq!(version_label("legends-arceus"), "Legends: Arceus");
        assert_eq!(version_label("legends-za"), "Legends: Z-A");
        assert_eq!(version_label("lets-go-pikachu"), "Let's Go, Pikachu!");
        assert_eq!(version_label("the-crown-tundra"), "The Crown Tundra");
        assert_eq!(version_label("some-future-game"), "Some Future Game");
    }

    #[test]
    fn dex_numbers_are_padded_to_four_digits() {
        assert_eq!(dex_number(1), "#0001");
        assert_eq!(dex_number(558), "#0558");
        assert_eq!(dex_number(1009), "#1009");
        assert_eq!(dex_number(12345), "#12345");
    }

    #[test]
    fn height_is_shown_in_metres() {
        assert_eq!(format_height(14), "1.4 m");
        assert_eq!(format_height(10), "1.0 m");
        assert_eq!(format_height(3), "0.3 m");
        assert_eq!(format_height(200), "20.0 m");
    }

    #[test]
    fn weight_is_shown_in_kilograms() {
        assert_eq!(format_weight(2000), "200.0 kg");
        assert_eq!(format_weight(69), "6.9 kg");
        assert_eq!(format_weight(1), "0.1 kg");
        assert_eq!(format_weight(9999), "999.9 kg");
    }

    #[test]
    fn line_breaks_become_single_spaces() {
        assert_eq!(
            normalize_flavor_text("Competing for territory, Crustle\nfight viciously."),
            "Competing for territory, Crustle fight viciously."
        );
    }

    #[test]
    fn form_feeds_become_single_spaces() {
        assert_eq!(
            normalize_flavor_text("planted on its\nback at birth.\x0cThe plant sprouts"),
            "planted on its back at birth. The plant sprouts"
        );
    }

    #[test]
    fn soft_hyphens_rejoin_split_words() {
        assert_eq!(
            normalize_flavor_text("to store up electric\u{ad}\nity. Forests"),
            "to store up electricity. Forests"
        );
        assert_eq!(normalize_flavor_text("elec\u{ad}tricity"), "electricity");
    }

    #[test]
    fn whitespace_is_trimmed_and_collapsed() {
        assert_eq!(normalize_flavor_text("  a \n\n b\x0c\n c \n"), "a b c");
        assert_eq!(normalize_flavor_text("\n\x0c "), "");
        assert_eq!(normalize_flavor_text(""), "");
    }

    #[test]
    fn real_hyphens_are_kept() {
        assert_eq!(
            normalize_flavor_text("a well-known\nfire-type"),
            "a well-known fire-type"
        );
    }

    #[test]
    fn a_hyphen_at_a_line_break_joins_the_compound() {
        assert_eq!(
            normalize_flavor_text("with bluish-\nwhite flames"),
            "with bluish-white flames"
        );
        assert_eq!(normalize_flavor_text("at high-\r\nspeed"), "at high-speed");
        assert_eq!(normalize_flavor_text("ultra-\x0csonic"), "ultra-sonic");
    }

    #[test]
    fn a_hyphen_not_at_a_line_break_keeps_its_spacing() {
        assert_eq!(normalize_flavor_text("well - maybe"), "well - maybe");
        assert_eq!(normalize_flavor_text("well- maybe"), "well- maybe");
        assert_eq!(
            normalize_flavor_text("high-speed\nflight"),
            "high-speed flight"
        );
        assert_eq!(normalize_flavor_text("wait -\nwhat"), "wait - what");
        assert_eq!(normalize_flavor_text("high-\rspeed"), "high- speed");
    }
}
