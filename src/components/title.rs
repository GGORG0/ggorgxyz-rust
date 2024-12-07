use yew::prelude::*;

pub fn title() -> Html {
    html! {
      <h1 class={classes!("title")}>
        {"GGORG"}
      </h1>
    }
}
