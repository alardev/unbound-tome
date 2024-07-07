#![allow(non_snake_case)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! (
        div { style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p {
                "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
            }
        }
    )
}
