use dioxus::prelude::*;

// Constants
const GITHUB_IMG: &'static str = wasm_or_else("github.svg", "public/github.svg");

// About component
pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            class: "mt-auto p-8 flex items-center italic text-xs",
            a {
                href: "https://github.com/ericgbanta/crustle/",
                target: "_blank",
                img {
                    class: "w-4 sm:w-8 align-middle mr-2",
                    src: GITHUB_IMG,
                }
            }
            " An Open Source project to create a Pokédex using Rust & Dioxus."
        }
    })
}

// Utility function
const fn wasm_or_else<'a, T: ?Sized>(then: &'a T, _else: &'a T) -> &'a T {
    if cfg!(target_family = "wasm") {
        then
    } else {
        _else
    }
}
