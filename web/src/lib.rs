#![forbid(unsafe_code)]

use std::sync::Arc;
use axum_login::tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use unbound_tome_utils::config::get_config;

use crate::web::App;

mod users;
mod web;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "unbound_tome=debug,axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let config = get_config();

    let app = Arc::new(App::new(config).await?.serve().await);

    Ok(())
}