use dioxus::prelude::*;

pub fn About() -> Element {
    rsx! {
        p {
            class: "mt-auto p-8 flex items-center italic text-xs",
            a {
                href: "https://github.com/ericgbanta/crustle/",
                target: "_blank",
                img {
                    class: "w-4 sm:w-8 align-middle mr-2",
                    src: "github.svg",
                }
            }
            " An Open Source project to create a Pokédex using Rust & Dioxus."
        }
    }
}
