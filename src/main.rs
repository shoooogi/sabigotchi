use dioxus::prelude::*;
use dioxus_google_fonts::google_fonts;
use dioxus_sdk_time::{self, use_interval};
use std::time::Duration;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const SATCHI_MASK: Asset = asset!("/assets/sabigotchi-mask.svg");
const SPRITE_SHEET: Asset = asset!("/assets/sasa.png");

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
        div { id: "main-box", class: "noisy", style: "--sprite-sheet: url({SPRITE_SHEET})",
            img { id: "satchi-mask", src: SATCHI_MASK }
            div { id: "div_screen",
                div { id: "screen", class: "scanlines",
                    Sprite { anim: true, frame_index: 0, fps: 3, total_frames: 93 }
                    // fazer sprite de fase ovo (feito), bebê/criança (fazer o bichinho ficar piquitucho), adolescente (feito), adulto (colocar gravata quando fazer), idoso, anjo
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

#[component]
fn Sprite(anim: bool, frame_index: u8, fps: u32, total_frames: u32) -> Element {
    let mut auto_frame = use_signal(|| 0u32);

    use_interval(Duration::from_millis(1000 / fps as u64), move |_| {
        if anim {
            auto_frame.set((auto_frame() + 1) % total_frames);
        }
    });

    let frame = if anim { auto_frame() } else { frame_index as u32 };

    rsx! {
        div { class: "sprite", style: "--frame: {frame}" }
    }
}
