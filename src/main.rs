pub mod backend;
mod components;

use crate::components::*;

use dioxus::prelude::*;

#[derive(Routable,Clone,PartialEq)]
enum Route{
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    
    #[route("/favorites")]
    Favorites,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
