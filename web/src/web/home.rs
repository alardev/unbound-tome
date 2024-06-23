use axum::{response::IntoResponse, routing::get, Router};
use crate::users::AuthSession;
use axum_htmx::HxRequest;
use maud::html;
use crate::web::views;


pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::homepage))
}

mod get {
    use super::*;

    pub async fn homepage(
        auth_session: AuthSession,
        HxRequest(hx_request): HxRequest
    ) -> impl IntoResponse  {

        if hx_request {
            //partial hx-request
            views::shell::render(
                auth_session.user,
                html!(
                    p { "Homepage partial load" }
                )
            )
        } else {
            //fullpage load
            views::page(views::shell::render(
                auth_session.user,
                html!(
                    p { "Homepage Full load" }
                )
            ))
        }
    }
}