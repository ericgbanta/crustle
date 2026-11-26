use dioxus::prelude::*;

const GITHUB_ICON: Asset = asset!("/assets/github.svg");

#[component]
pub fn About() -> Element {
    rsx! {
        footer {
            class: "border-t border-line",
            div {
                class: "mx-auto flex max-w-3xl items-center gap-3 px-4 py-6 text-sm text-muted sm:px-6",
                a {
                    class: "shrink-0 rounded-full",
                    href: "https://github.com/ericgbanta/crustle/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    img {
                        // The icon is a near-black glyph, so it is inverted on dark backgrounds.
                        class: "size-6 dark:invert",
                        src: GITHUB_ICON,
                        alt: "crustle on GitHub",
                        width: "24",
                        height: "24",
                    }
                }
                p { "An Open Source project to create a Pokédex using Rust & Dioxus." }
            }
        }
    }
}
