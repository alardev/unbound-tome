use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use crate::users::AuthSession;



pub fn router() -> Router<()> {
    Router::new().route("/account", get(self::get::account))
}

mod get {
    use super::*;

    pub async fn account(auth_session: AuthSession) -> impl IntoResponse {

        StatusCode::NOT_IMPLEMENTED.into_response()
        
    }
}