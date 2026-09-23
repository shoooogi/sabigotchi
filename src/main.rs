use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Sabigotchi" }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        { google_fonts!([
            ("Inter Tight", wght = [1, 400])
        ]) }

        MainTitle{}
        MainBox{}
    }
}

#[component]
fn MainBox() -> Element {
    rsx! {
        div { id: "main-box", class: "box",
            div { id: "sabi-box", class: "box",
                
            }
        }
    }
}

#[component]
fn MainTitle() -> Element {
    rsx! {
        h1 { id: "main-title", class: "text",
            "錆びごっち"
        }
    }
}
