use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const SATCHI_MASK: Asset = asset!("/assets/sabigotchi-mask.svg");

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
            ("Inter Tight", wght = ["200..800"]),
            ("Sawarabi Gothic", wght = [400, 700, 800]),
            ("Pixelify Sans", wght = ["400..700"]),
        ]) }

        MainTitle {}
        MainBox {}
        Credits {}
    }
}

#[component]
fn MainBox() -> Element {
    rsx! {
        div { id: "main-box", class: "noisy",
            img { id: "satchi-mask", src: SATCHI_MASK }
            div { id: "div_screen",
                div { id: "screen", class: "scanlines",
                    // todo
                }
            }
            div { id: "control",
                Button { id: "choose", text: "Choose" }
                Button { id: "enter", text: "Enter" }
                Button { id: "cancel", text: "Cancel" }
            }
        }
    }
}

#[component]
fn MainTitle() -> Element {
    rsx! {
        div { id: "div_main-title",
            img { id: "main-logo", src: asset!("/assets/rustlogo.png") },

            h1 { id: "main-title", class: "text",
                span { id: "main-title_color", "錆び" }, "ごっち"
            }
        }
    }
}

#[component]
fn Credits() -> Element {
    rsx! {
        p { id: "credits", class: "text",
            "Made by Guilherme 💛"
        }
    }
}

#[component]
fn Button(id: String, text: String) -> Element {
    rsx! {
        div { class: "div_button",
            div { id: id, class: "button noisy", div { id: "button-background", class: "noisy" } }
            ArcText { text: text }
        }
    }
}

#[component]
fn ArcText(text: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 500 70",
            class: "w-full",

            defs {
                path {
                    id: "curve",
                    d: "M 50 20 Q 250 90 450 10",
                    fill: "none",
                }
            }

            text {
                class: "button-text",
                text_anchor: "middle",
                textPath {
                    href: "#curve",
                    start_offset: "50%",
                    {text}
                }
            }
        }
    }
}
