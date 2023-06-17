#![allow(non_snake_case)]
use dioxus::prelude::*;
use components::{header::Header, footer::Footer, content::Content};

mod activity;
mod components;


fn main() {
    let describe = include_str!("../assets/describe.txt");
    let mime = include_str!("../assets/mime.txt");
    let draw = include_str!("../assets/draw.txt");

    let mut instance = activity::Activity::new(describe, mime, draw, 4);

    #[cfg(target_family = "wasm")]
    dioxus_web::launch(app);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new());
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/styles.css" },
        Header {}
        Content {}
        Footer {}
    })
}