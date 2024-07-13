#![allow(non_snake_case)]
#![forbid(unsafe_code)]

pub mod pages;
pub mod components;
pub mod service;
pub mod state;
pub mod model;

use dioxus::prelude::*;
use pages::{
    home::Home,
    login::Login,
    account::Account,
    pagenotfound::PageNotFound,
};

use components::shell::{ContentWrapper, HeaderFooter};
use state::AppState;

pub static APP_STATE: GlobalSignal<AppState> = Signal::global(AppState::default);

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


