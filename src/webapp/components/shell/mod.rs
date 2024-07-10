use dioxus::prelude::*;

use crate::webapp::{components::{footer::Footer, navbar::NavBar}, Route};

pub fn ContentWrapper() -> Element {
    rsx! {
            div {
                id: "tab-content",
                role: "tabpanel",
                class: "bg-base-200 bg-cover grow h-full",
                style: "background-image: url(https://i.etsystatic.com/18572829/r/il/bfb63e/4985394714/il_1140xN.4985394714_m67f.jpg);",
                Outlet::<Route> {}
            }
    }
}

#[component]
pub fn HeaderFooter() -> Element {
    rsx! {
        NavBar {}
        Outlet::<Route> {}
        Footer {}
    }
}