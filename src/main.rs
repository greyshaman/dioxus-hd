use dioxus::prelude::*;

mod backend;
mod components;

use crate::components::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }

        Router::<Route> {}
    }
}

