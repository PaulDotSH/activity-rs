#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Footer(cx: Scope) -> Element {
    let footer_style = r"
        font-size: 32px;
        color: purple;
    ";


    cx.render(rsx! {
        footer { 
            style: "{footer_style}",
            "Footer RS"
        }
    })
}