use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn RectBox(children: &Html) -> Html {
    html! {
        <div class={classes!("rectbox")}>
            { children.clone() }
        </div>
    }
}
