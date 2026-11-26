// The `Routable` derive expands the catch-all segment, whose parse cannot fail, into
// code that rustc 1.100 and later report as unreachable. The allow is scoped to this
// module because an attribute on the enum does not reach the derive's output.
#![allow(unreachable_code)]

use crate::components::contact::Contact;
use crate::components::home::Home;
use crate::components::not_found::PageNotFound;
use crate::components::pokedex::Pokedex;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/pokedex")]
    Pokedex {},
    #[route("/contact")]
    Contact {},
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
