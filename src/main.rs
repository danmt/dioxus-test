#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

pub mod pages;
pub mod functions;
pub mod utils;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const _STYLES_URL: &str = manganis::mg!(file("public/styles.css"));

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<pages::Route> {}
    }
}

