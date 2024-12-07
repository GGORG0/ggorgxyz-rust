use yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {
      <h1 class={classes!("title")}>
        {"GGORG"}
      </h1>
    }
}
