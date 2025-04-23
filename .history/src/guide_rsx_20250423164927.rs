use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog!" }
        }
        div {  }
    }
}
