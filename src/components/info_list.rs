use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InfoListItem {
    Plain((AttrValue, AttrValue)),
    WithLink((AttrValue, AttrValue, AttrValue)),
}

impl From<(&'static str, &'static str)> for InfoListItem {
    fn from(value: (&'static str, &'static str)) -> Self {
        Self::Plain((value.0.into(), value.1.into()))
    }
}

impl From<(&'static str, &'static str, &'static str)> for InfoListItem {
    fn from(value: (&'static str, &'static str, &'static str)) -> Self {
        Self::WithLink((value.0.into(), value.1.into(), value.2.into()))
    }
}

pub fn info_list(items: &Vec<InfoListItem>) -> Html {
    html! {
        <ul class={classes!("no-margin")}>
            { for items.iter().map(|item| match item {
                InfoListItem::Plain(item) => html! {
                    <li key={item.0.as_str()}>
                        <span class={classes!("bold")}>{&item.0}</span>
                        {": "}
                        <span>{&item.1}</span>
                    </li>
                },
                InfoListItem::WithLink(item) => html! {
                    <li key={item.0.as_str()}>
                        <span class={classes!("bold")}>{&item.0}</span>
                        {": "}
                        <a href={&item.2}>{&item.1}</a>
                    </li>
                }
            }) }
        </ul>
    }
}
