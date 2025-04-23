use dioxus::prelude::{Element, Props};


#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}

fn DogApp(props: DogAppProps) -> Element {
    todo!()
}