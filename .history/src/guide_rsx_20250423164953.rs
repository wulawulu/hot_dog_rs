use dioxus::{html::img, prelude::*};

#[component]
fn App() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog!" }
        }
        div { id: "dogview",
            img{ src}  }
    }
}
