#![allow(non_snake_case)]
#![forbid(unsafe_code)]

#[allow(unused_imports)]
use dioxus::prelude::*;

use tracing::Level;

#[cfg(feature = "server")]
use unbound_tome_utils::config::get_config;

#[cfg(feature = "server")]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[cfg(feature = "server")]
pub mod server;

pub mod webapp;

pub fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();


    // Hydrate the application on the client
    #[cfg(feature = "web")]
    {
        dioxus::web::launch::launch_cfg(webapp::App, dioxus::web::Config::new().hydrate(true));
    }

    #[cfg(feature = "server")]
    {
        use std::sync::Arc;
        use axum::routing::*;
        use server::Context;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let ctx = Context::new(get_config()).await;

                Arc::new(server::serve(ctx.unwrap().into()).await);
            });
    }
}