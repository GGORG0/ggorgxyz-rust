use crate::components::{info_list::InfoList, link_bar::LinkBar, rect_box::RectBox, title::Title};
use yew::prelude::*;

#[function_component]
pub fn MainBox() -> Html {
    html! {
        <RectBox>
            <Title />
            <div class="mainbox-content">
                <h2 class={classes!("no-margin")}>
                    <span class={classes!("wave")}>{"👋"}</span>
                    {" Hi!"}
                </h2>
                <div>
                    {"My name is Grzegorz (Gregory)."}
                    <br />
                    {"I'm a 15 yo programmer from Poland, and am currently attending high school."}
                </div>

                <h3 class={classes!("no-margin")}>{"Some more info about me:"}</h3>

                <InfoList items={vec![
                    ("Pronouns", "he/him").into(),
                    ("Location", "Wrocław, Poland").into(),
                    ("Timezone", "Europe/Warsaw", "https://time.is/Wroclaw").into(),
                    ("OS", "NixOS w/Hyprland", "https://github.com/GGORG0/nix-config").into(),
                    ("Hates", "big tech").into(),
                    ("Loves", "privacy & FOSS").into(),
                ]} />

                <LinkBar items={vec![
                    ("Keyoxide", "GGORG", "keyoxide.svg", "https://keyoxide.org/openpgp4fpr:7536b3da69fbb954460c7311be009af8c726ef1a").into(),
                    ("Codeberg", "GGORG", "codeberg.svg", "https://codeberg.org/GGORG").into(),
                    ("GitHub", "GGORG0", "github.svg", "https://github.com/GGORG0").into(),
                    ("Matrix", "@ggorg:matrix.org", "matrix.svg", "https://matrix.to/#/@ggorg:matrix.org").into(),
                    ("Discord", "ggorg", "discord.svg", "https://discord.com/users/819845763848601611").into(),
                    ("Email", "GGORG0@protonmail.com", "email.svg", "mailto:GGORG0@protonmail.com").into(),
                    ("OpenPGP", "BE009AF8C726EF1A", "pgp.svg", "/assets/BE009AF8C726EF1A.asc").into(),
                ]} placeholder={"Contact me"} />

                <LinkBar items={vec![
                    ("Rust", "rust.svg").into(),
                    ("TypeScript/JavaScript", "typescript.svg").into(),
                    ("React", "react.svg").into(),
                    ("Python", "python.svg").into(),
                    ("Docker", "docker.svg").into(),
                    ("Arduino", "arduino.svg").into(),
                    ("Linux", "linux.svg").into(),
                    ("Nix", "nix.svg").into(),
                    ("Hyprland", "hyprland.svg").into(),
                    ("GrapheneOS", "grapheneos.svg").into(),
                ]} placeholder={"Technologies I know/use"} />

                <a href="https://github.com/GGORG0/ggorgxyz-rust" class={classes!("repo-link")}>
                    {"Made with \u{2764}\u{fe0f} in Neovim using Yew.rs"}
                    <br />
                    {"Public on GitHub under GPL-3.0"}
                </a>
            </div>
        </RectBox>
    }
}
