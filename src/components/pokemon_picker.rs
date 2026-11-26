use crate::dex::{self, DexEntry};
use crate::format::dex_number;
use crate::search::search;
use dioxus::prelude::*;
use std::rc::Rc;

const INPUT_ID: &str = "pokemon-search";
const LIST_ID: &str = "pokemon-search-results";
/// How many matches the result list shows at once.
const LIMIT: usize = 8;

const OPTION: &str = "flex cursor-pointer items-center gap-3 px-3 py-2.5";
const OPTION_ACTIVE: &str =
    "flex cursor-pointer items-center gap-3 bg-accent-soft px-3 py-2.5 text-accent-strong";

/// A search box with a result list, following the ARIA combobox pattern: typing
/// filters by name or dex number, the arrow keys move through the results, Enter picks
/// one and Escape closes the list. Focus stays in the text field while the keyboard is
/// used; picking an option with the pointer or by touch gives it up.
#[component]
pub fn PokemonPicker(on_select: EventHandler<DexEntry>) -> Element {
    let mut query = use_signal(String::new);
    let mut open = use_signal(|| false);
    // Index into the visible matches of the option the arrow keys are on.
    let mut active = use_signal(|| None::<usize>);
    let mut input = use_signal(|| None::<Rc<MountedData>>);
    let matches = use_memo(move || search(dex::entries(), &query(), LIMIT));

    let mut choose = move |entry: DexEntry| {
        query.set(entry.name.to_string());
        open.set(false);
        active.set(None);
        on_select.call(entry);
    };

    let onkeydown = move |event: KeyboardEvent| {
        let shown = matches.read().entries.len();
        match event.key() {
            Key::ArrowDown | Key::ArrowUp => {
                event.prevent_default();
                let direction = if event.key() == Key::ArrowDown {
                    Step::Next
                } else {
                    Step::Previous
                };
                // The first press on a closed list opens it without skipping an option.
                let from = if open() { active() } else { None };
                active.set(step(from, shown, direction));
                open.set(true);
            }
            // After Escape the field still has its text but no list; Enter brings the
            // list back rather than picking something the user cannot see.
            Key::Enter if !open() => {
                if !query.read().is_empty() {
                    event.prevent_default();
                    open.set(true);
                }
            }
            Key::Enter => {
                event.prevent_default();
                // With no option highlighted, Enter takes the best match for what was
                // typed, but never an arbitrary entry from the unfiltered list.
                let index = match active() {
                    Some(index) => Some(index),
                    None if !query.read().trim().is_empty() => Some(0),
                    None => None,
                };
                let entry = index.and_then(|index| matches.read().entries.get(index).copied());
                if let Some(entry) = entry {
                    choose(entry);
                }
            }
            Key::Escape => {
                event.prevent_default();
                if open() {
                    open.set(false);
                    active.set(None);
                } else {
                    query.set(String::new());
                }
            }
            _ => {}
        }
    };

    let results = matches.read();
    let active_entry = active().and_then(|index| results.entries.get(index).copied());
    let active_id = active_entry.filter(|_| open()).map(option_id);
    let list_shown = open() && !results.entries.is_empty();
    let status = result_status(results.entries.len(), results.total, query.read().trim());

    rsx! {
        div {
            label {
                class: "block text-sm font-medium",
                r#for: INPUT_ID,
                "Find a Pokémon"
            }
            div {
                class: "relative mt-2",
                input {
                    id: INPUT_ID,
                    class: "block w-full rounded-lg border border-control bg-surface py-2.5 pr-11 pl-3 text-base placeholder:text-muted focus:border-accent focus:outline-2 focus:outline-offset-0 focus:outline-accent",
                    r#type: "text",
                    role: "combobox",
                    placeholder: "Name or number",
                    autocomplete: "off",
                    autocapitalize: "off",
                    spellcheck: "false",
                    aria_autocomplete: "list",
                    aria_expanded: "{open}",
                    aria_controls: list_shown.then_some(LIST_ID),
                    aria_activedescendant: active_id,
                    value: "{query}",
                    onmounted: move |event| input.set(Some(event.data())),
                    oninput: move |event| {
                        query.set(event.value());
                        active.set(None);
                        open.set(true);
                    },
                    onkeydown,
                    onfocus: move |_| open.set(true),
                    // Reopens the list after Escape, when the field is still focused.
                    onclick: move |_| open.set(true),
                    onblur: move |_| {
                        open.set(false);
                        active.set(None);
                    },
                }
                if !query.read().is_empty() {
                    button {
                        class: "absolute inset-y-0 right-0 flex w-11 items-center justify-center rounded-r-lg text-xl leading-none text-muted hover:text-ink focus-visible:-outline-offset-2",
                        r#type: "button",
                        aria_label: "Clear search",
                        // Keeps focus (and the on-screen keyboard) in the text field.
                        onmousedown: move |event| event.prevent_default(),
                        onclick: move |_| {
                            query.set(String::new());
                            active.set(None);
                            open.set(true);
                            if let Some(input) = input() {
                                spawn(async move {
                                    let _ = input.set_focus(true).await;
                                });
                            }
                        },
                        span { aria_hidden: "true", "×" }
                    }
                }
                if open() {
                    div {
                        class: "absolute inset-x-0 top-full z-20 mt-1 overflow-hidden rounded-lg border border-line bg-surface shadow-lg",
                        // Pressing on the list must not blur the field, or the list
                        // would close before the click lands.
                        onmousedown: move |event| event.prevent_default(),
                        if list_shown {
                            ul {
                                id: LIST_ID,
                                role: "listbox",
                                aria_label: "Pokémon",
                                for (index, entry) in results.entries.iter().copied().enumerate() {
                                    li {
                                        key: "{entry.id}",
                                        id: option_id(entry),
                                        class: if active_entry == Some(entry) { OPTION_ACTIVE } else { OPTION },
                                        role: "option",
                                        aria_selected: active_entry == Some(entry),
                                        // Movement, not mouseenter: a pointer resting where
                                        // the list appears must not take the highlight.
                                        onmousemove: move |_| {
                                            if active() != Some(index) {
                                                active.set(Some(index));
                                            }
                                        },
                                        onclick: move |_| {
                                            choose(entry);
                                            // Hides the on-screen keyboard, which would
                                            // otherwise cover the card just asked for.
                                            if let Some(input) = input() {
                                                spawn(async move {
                                                    let _ = input.set_focus(false).await;
                                                });
                                            }
                                        },
                                        span {
                                            class: "w-12 shrink-0 font-mono text-sm opacity-70",
                                            {dex_number(entry.id)}
                                        }
                                        span { class: "truncate", "{entry.name}" }
                                    }
                                }
                            }
                        }
                        if let Some(status) = &status {
                            p {
                                class: "border-line px-3 py-2.5 text-sm text-muted not-first:border-t",
                                "{status}"
                            }
                        }
                    }
                }
            }
            // Announces how the list changed to screen reader users as they type.
            p {
                class: "sr-only",
                role: "status",
                if open() {
                    {announcement(results.total)}
                }
            }
        }
    }
}

fn option_id(entry: DexEntry) -> String {
    format!("pokemon-option-{}", entry.id)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Step {
    Next,
    Previous,
}

/// Where the highlight goes when an arrow key is pressed in a list of `len` options.
/// From nothing it enters at the near end, and it wraps around at both ends. An index
/// past the end of the list highlights nothing on screen, so it counts as nothing.
fn step(active: Option<usize>, len: usize, direction: Step) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match (active.filter(|&index| index < len), direction) {
        (None, Step::Next) => 0,
        (None, Step::Previous) => len - 1,
        (Some(index), Step::Next) => (index + 1) % len,
        (Some(index), Step::Previous) => (index + len - 1) % len,
    })
}

/// The note under the result list: why it is empty, or that it has been cut short.
fn result_status(shown: usize, total: usize, query: &str) -> Option<String> {
    if total == 0 {
        Some(format!("No Pokémon match \"{query}\"."))
    } else if shown < total {
        Some(format!(
            "Showing {shown} of {total}. Keep typing to narrow it down."
        ))
    } else {
        None
    }
}

fn announcement(total: usize) -> String {
    match total {
        0 => "No results".to_string(),
        1 => "1 result".to_string(),
        _ => format!("{total} results"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_enter_the_list_at_the_near_end() {
        assert_eq!(step(None, 8, Step::Next), Some(0));
        assert_eq!(step(None, 8, Step::Previous), Some(7));
    }

    #[test]
    fn arrows_move_one_option_and_wrap() {
        assert_eq!(step(Some(0), 8, Step::Next), Some(1));
        assert_eq!(step(Some(7), 8, Step::Next), Some(0));
        assert_eq!(step(Some(3), 8, Step::Previous), Some(2));
        assert_eq!(step(Some(0), 8, Step::Previous), Some(7));
        assert_eq!(step(Some(0), 1, Step::Next), Some(0));
        assert_eq!(step(Some(0), 1, Step::Previous), Some(0));
    }

    #[test]
    fn arrows_do_nothing_in_an_empty_list() {
        assert_eq!(step(None, 0, Step::Next), None);
        assert_eq!(step(Some(2), 0, Step::Previous), None);
    }

    #[test]
    fn an_out_of_range_highlight_counts_as_none() {
        assert_eq!(step(Some(9), 3, Step::Next), step(None, 3, Step::Next));
        assert_eq!(step(Some(3), 3, Step::Next), Some(0));
        assert_eq!(step(Some(9), 3, Step::Previous), Some(2));
    }

    #[test]
    fn status_explains_empty_and_truncated_lists() {
        assert_eq!(
            result_status(0, 0, "zzz").as_deref(),
            Some("No Pokémon match \"zzz\".")
        );
        assert_eq!(
            result_status(8, 153, "pi").as_deref(),
            Some("Showing 8 of 153. Keep typing to narrow it down.")
        );
        assert_eq!(result_status(3, 3, "saur"), None);
    }

    #[test]
    fn announcements_count_results() {
        assert_eq!(announcement(0), "No results");
        assert_eq!(announcement(1), "1 result");
        assert_eq!(announcement(42), "42 results");
    }
}
