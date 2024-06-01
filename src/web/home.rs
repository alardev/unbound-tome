use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_login::tracing;
use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "protected.html")]
struct ProtectedTemplate<'a> {
    username: &'a str,
}

#[derive(Template)]
#[template(path = "public.html")]
struct PublicTemplate;


pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
        tracing::debug!("HIII");
        match auth_session.user {
            Some(user) => ProtectedTemplate {
                username: &user.username,
            }
            .into_response(),

            //None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            None => PublicTemplate.into_response(),
        }
    }
}