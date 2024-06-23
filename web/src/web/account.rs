use axum::{response::IntoResponse, routing::get, Router};
use crate::users::AuthSession;
use axum_htmx::HxRequest;
use maud::html;
use crate::web::views;

pub fn router() -> Router<()> {
    Router::new().route("/account", get(self::get::account))
}

mod get {

    use super::*;

    pub async fn account(
        HxRequest(hx_request): HxRequest,
        auth_session: AuthSession,
    ) -> impl IntoResponse {

        let userdata = auth_session.user.unwrap();

        if hx_request {
            //partial hx-request
            html!(h3 { "Username: " (userdata.username) }
            )
        } else {
            //fullpage load
            views::page(views::shell::render(
                Some(userdata.clone()),
                html!(
                    h3 { "Username: " (userdata.username) }
                )
            ))
        }
        
    }
}