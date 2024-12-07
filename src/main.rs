use yew::prelude::*;

mod components;
use components::main_box::main_box;

#[function_component]
fn App() -> Html {
    html! {
        <>
            { main_box() }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
