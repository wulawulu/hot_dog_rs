use dioxus::{html::img, prelude::*};

#[component]
fn App() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog!" }
        }
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "button",
            button { id }
        }
    }
}
