use yew::prelude::*;

mod components;
use components::main_box::MainBox;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <MainBox />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
