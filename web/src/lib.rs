#![forbid(unsafe_code)]

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use unbound_tome_utils::config::get_config;
use crate::web::app::Context;

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

    let ctx = Context::new(config).await?;

    Arc::new(web::app::serve(ctx.into()).await);

    Ok(())
}