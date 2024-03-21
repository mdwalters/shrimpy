use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <p>{ "Hello, world!" }</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}