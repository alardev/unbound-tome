use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "account.html")]
struct AccountTemplate<'a> {
    username: &'a str,
}


pub fn router() -> Router<()> {
    Router::new().route("/account", get(self::get::account))
}

mod get {
    use super::*;

    pub async fn account(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => AccountTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}