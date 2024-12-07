use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(Debug, Clone, PartialEq)]
enum LinkBarTextEntry {
    Item(LinkBarItem),
    Placeholder(AttrValue),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkBarItem {
    pub key: AttrValue,
    pub value: Option<AttrValue>,
    pub icon: AttrValue,
    pub url: Option<AttrValue>,
}

impl From<(&'static str, &'static str, &'static str, &'static str)> for LinkBarItem {
    fn from(value: (&'static str, &'static str, &'static str, &'static str)) -> Self {
        Self {
            key: value.0.into(),
            value: Some(value.1.into()),
            icon: value.2.into(),
            url: Some(value.3.into()),
        }
    }
}

impl From<(&'static str, &'static str, &'static str)> for LinkBarItem {
    fn from(value: (&'static str, &'static str, &'static str)) -> Self {
        Self {
            key: value.0.into(),
            value: Some(value.1.into()),
            icon: value.2.into(),
            url: None,
        }
    }
}

impl From<(&'static str, &'static str)> for LinkBarItem {
    fn from(value: (&'static str, &'static str)) -> Self {
        Self {
            key: value.0.into(),
            value: None,
            icon: value.1.into(),
            url: None,
        }
    }
}

#[autoprops]
#[function_component]
pub fn LinkBar(items: &Vec<LinkBarItem>, placeholder: &AttrValue) -> Html {
    let hovered_item =
        use_state_eq::<LinkBarTextEntry, _>(|| LinkBarTextEntry::Placeholder(placeholder.clone()));

    let on_item_hover = {
        let hovered_item = hovered_item.clone();
        Callback::from(move |item| {
            hovered_item.set(LinkBarTextEntry::Item(item));
        })
    };

    html! {
        <div class={classes!("link-bar-container")}>
            <LinkBarText item={(*hovered_item).clone()} />

            <div class={classes!("link-bar")}>
                { for items.iter().map(|item| {
                    let active = match (*hovered_item).clone() {
                        LinkBarTextEntry::Item(x) => *item == x,
                        _ => false,
                    };

                    html! {
                        <LinkBarIcon
                            key={item.key.as_str()}
                            item={item.clone()}
                            on_hover={&on_item_hover}
                            active={active}
                        />
                    }
                }) }
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn LinkBarText(item: &LinkBarTextEntry) -> Html {
    html! {
        <div class={classes!("link-bar-text")}>
            { match item {
                LinkBarTextEntry::Item(item) => html!{
                    <a href={&item.url}>
                        <span class={classes!("bold")}>{&item.key}</span>
                        {if item.value.is_some() { ": " } else { "" }}
                        <span>{&item.value}</span>
                    </a>
                },
                LinkBarTextEntry::Placeholder(text) => html!{
                    <span class={classes!("bold")}>{&text}</span>
                }
            }}
        </div>
    }
}

#[autoprops]
#[function_component]
fn LinkBarIcon(item: &LinkBarItem, on_hover: Callback<LinkBarItem>, active: bool) -> Html {
    let cb = {
        let item = item.clone();

        Callback::from(move |_| {
            on_hover.emit(item.clone());
        })
    };

    let active_class = if active { Some("active") } else { None };

    html! {
        <a class={classes!("link-bar-item", active_class)} href={&item.url} onpointerover={cb}>
            <img alt={&item.key} src={format!("assets/{}", &item.icon)} />
        </a>
    }
}
