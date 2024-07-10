#![allow(non_snake_case)]
#![forbid(unsafe_code)]

pub mod pages;
pub mod components;

use dioxus::prelude::*;
use pages::{
    home::Home,
    login::Login,
    account::Account,
    pagenotfound::PageNotFound,
};

use components::shell::{ContentWrapper, HeaderFooter};


#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
    // #[nest("/")]
        #[layout(ContentWrapper)]
        #[route("/")]
        Home {},
        #[route("/account")]
        Account {},
    // #[end_nest]
    #[route("/auth/login")]
    Login {},
    
    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn App() -> Element {
    rsx! (
        Router::<Route> {}
    )
}


