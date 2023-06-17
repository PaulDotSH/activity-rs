#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Header(cx: Scope) -> Element {
    let header_style = r"
        font-size: 32px;
        color: purple;
    ";



    cx.render(rsx! {
        header { 
            style: "{header_style}",
            "Activity RS"
        }
    })
}