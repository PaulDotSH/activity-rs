#![allow(non_snake_case)]
use dioxus::prelude::*;

mod activity;

fn main() {
    let describe = include_str!("../assets/describe.txt");
    let mime = include_str!("../assets/mime.txt");
    let draw = include_str!("../assets/draw.txt");

    let mut instance = activity::Activity::new(describe, mime, draw, 4);
    println!("{:?}", instance.get_next_word());
    println!("{:?}", instance.get_next_word());
    println!("{:?}", instance.get_next_word());
    println!("{:?}", instance.get_next_word());
    println!("{:?}", instance.get_next_word());
    println!("{:?}", instance.get_next_word());

    #[cfg(target_family = "wasm")]
    dioxus_web::launch(app);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()));
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}