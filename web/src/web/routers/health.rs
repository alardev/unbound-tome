use serde_json::json;

use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router<()> {
    Router::new().route("/health", get(self::get::health_handler))
}

mod get {

    use super::*;

    /// Handle health check requests
    pub async fn health_handler() -> impl IntoResponse {
        json!({
            "code": "200",
            "success": true,
        })
        .to_string()
    }
}