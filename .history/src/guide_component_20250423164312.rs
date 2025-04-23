use dioxus::prelude::Props;


#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}

fn DogApp(props: DogAppProps) -> Element {
    todo!()
}