use dioxus::prelude::*;

const GITHUB_ICON: Asset = asset!("/assets/github.svg");

pub fn About() -> Element {
    rsx! {
        p {
            class: "mt-auto p-8 flex items-center italic text-xs",
            a {
                href: "https://github.com/ericgbanta/crustle/",
                target: "_blank",
                img {
                    class: "w-4 sm:w-8 align-middle mr-2",
                    src: GITHUB_ICON,
                }
            }
            " An Open Source project to create a Pokédex using Rust & Dioxus."
        }
    }
}
