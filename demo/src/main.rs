#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Sup, rustaceans 🦀🦀🦀🦀");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "From println to now, Rustaceans 🦀🦀🦀🦀🦀🦀🦀 are increasing in number"
        }
    })
}