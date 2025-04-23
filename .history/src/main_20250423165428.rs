use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "title",
            h1 { "HotDog!" }
        }
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "button",
            button { id:"skip", "Skip" }
            button { id: "save", "save!" }
        }
    }
}


