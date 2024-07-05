use axum::{
    extract::Query, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router
};
use axum_login::tower_sessions::Session;
use oauth2::CsrfToken;
use serde::Deserialize;

use crate::web::middleware::auth::AuthSession;

pub const NEXT_URL_KEY: &str = "auth.next-url";

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}


pub fn router(oauth_enabled: bool) -> Router<()> {
    let mut router = Router::new()
        .route("/login/password", post(self::post::login::password))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
        .route("/register", get(self::get::register))
        .route("/register/password", post(self::post::register::password));

    if oauth_enabled {
        router = router.route("/login/oauth", post(self::post::login::oauth))
        .route("/oauth/callback", get(self::get::callback))
    }

    router
}

mod post {
    use super::*;

    pub(super) mod login {
        use std::ops::Deref;

        use axum::body::{self, Body};
        use axum_htmx::HxRequest;
        use http::Response;
        use maud::html;
        

        use super::*;
        use crate::{web::middleware::auth::{Credentials, PasswordCreds}, web::views};

        pub async fn password(
            mut auth_session: AuthSession,
            HxRequest(hx_request): HxRequest,
            Form(creds): Form<PasswordCreds>,
        ) -> impl IntoResponse {
            let user = match auth_session
                .authenticate(Credentials::Password(creds.clone()))
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => {
                    match hx_request {
                        true => {
                            return (
                                StatusCode::UNAUTHORIZED,
                                html! { ("Invalid credentials".to_string())
                                }).into_response()
                        },
                        false => {
                            return (
                                StatusCode::UNAUTHORIZED,
                                views::page(views::shell::render(
                                    &None,
                                    views::login::render(Some("Invalid credentials".to_string()), None)
                                ))).into_response()
                        }
                    }
                }
                Err(_) => {
                    let mut response = html! {("Something went wrong! Internal Server Error 500!")}.into_response();
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return response
                },
            };

            if auth_session.login(&user).await.is_err() {
                let mut response = html! {("Something went wrong! Internal Server Error 500!")}.into_response();
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return response
            }

            let (parts, body) = html!(div id="navbar" hx-swap-oob="outerHTML" { }).into_response().into_parts();

            if let Some(ref next) = creds.next {
                // Redirect::to(next).into_response()
                let res = Response::builder()
                    .status(200)
                    .header("HX-Location", 
                    format!("{{\"path\":\"{}\", \"target\":\"#tab-content\", \"headers\":{{\"username\": \"{}\"}}}}", next, creds.username))
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(body)
                    .unwrap();
                return res
            } else {
                // Redirect::to("/").into_response()
                let res = Response::builder()
                    .status(200)
                    .header("HX-Location", 
                    format!("{{\"path\":\"{}\", \"target\":\"#tab-content\", \"headers\":{{\"username\": \"{}\"}}}}", "/", creds.username))
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(body)
                    .unwrap();
                return res
            }
        }

        pub async fn oauth(
            auth_session: AuthSession,
            session: Session,
            Form(NextUrl { next }): Form<NextUrl>,
        ) -> impl IntoResponse {
            let (auth_url, csrf_state) = auth_session.backend.authorize_url();

            session
                .insert(CSRF_STATE_KEY, csrf_state.secret())
                .await
                .expect("Serialization should not fail.");

            session
                .insert(NEXT_URL_KEY, next)
                .await
                .expect("Serialization should not fail.");

            Redirect::to(auth_url.as_str()).into_response()
        }
    }

    pub(super) mod register {
        use axum::body::Body;
        use axum_htmx::HxRequest;
        use http::Response;
        use maud::html;

        use super::*;
        use crate::web::{middleware::auth::{Credentials, PasswordCreds}, views};


        pub async fn password(
            mut auth_session: AuthSession,
            HxRequest(hx_request): HxRequest,
            Form(creds): Form<PasswordCreds>,
            // Extension(ctx): Extension<Context>,
        ) -> impl IntoResponse {

            // let user = ctx.users.get_by_username(username, with_roles).await?;

            let user = match auth_session
                .authenticate(Credentials::Password(creds.clone()))
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => {
                    match hx_request {
                        true => {
                            return (
                                StatusCode::UNAUTHORIZED,
                                html! { ("Invalid credentials".to_string())
                                }).into_response()
                        },
                        false => {
                            return (
                                StatusCode::UNAUTHORIZED,
                                views::page(views::shell::render(
                                    &None,
                                    views::login::render(Some("Invalid credentials".to_string()), None)
                                ))).into_response()
                        }
                    }
                }
                Err(_) => {
                    let mut response = html! {("Something went wrong! Internal Server Error 500!")}.into_response();
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return response
                },
            };

            if auth_session.login(&user).await.is_err() {
                let mut response = html! {("Something went wrong! Internal Server Error 500!")}.into_response();
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return response
            }

            if let Some(ref next) = creds.next {
                // Redirect::to(next).into_response()
                let res = Response::builder()
                    .status(200)
                    .header("HX-Location", format!("{{\"path\":\"{}\", \"target\":\"#tab-content\"}}", next))
                    .body(Body::empty())
                    .unwrap();
                return res
            } else {
                // Redirect::to("/").into_response()
                let res = Response::builder()
                    .status(200)
                    .header("HX-Location", "{\"path\":\"/\", \"target\":\"#tab-content\"}")
                    .body(Body::empty())
                    .unwrap();
                return res
            }
        }
    }
}

mod get {
    use axum::body::Body;
    use axum_htmx::HxRequest;
    use http::Response;
    use maud::html;

    use crate::web::middleware::auth::OAuthCreds;
    use crate::web::views;

    use crate::web::{
        routers::auth::NEXT_URL_KEY,
        middleware::auth::{AuthSession, Credentials},
    };
    

    use super::*;

    pub async fn login(
        HxRequest(hx_request): HxRequest,
        Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
        if hx_request {
            //partial hx-request
            views::login::render(None, next)
        } else {
            //fullpage load
            views::page(views::shell::render(
                &None,
                views::login::render(None, next)
            ))
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => {
                let res = Response::builder()
                    .status(200)
                    .header("HX-Location", "{\"path\":\"/\", \"target\":\"#tab-content\"}")
                    .body(Body::empty())
                    .unwrap();
                return res
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn register(
        HxRequest(hx_request): HxRequest,
        Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
        if hx_request {
            //partial hx-request
            views::register::render(None, next)
        } else {
            //fullpage load
            views::page(views::shell::render(
                &None,
                views::register::render(None, next)
            ))
        }
    }

    pub async fn callback(
        mut auth_session: AuthSession,
        session: Session,
        Query(AuthzResp {
            code,
            state: new_state,
        }): Query<AuthzResp>,
    ) -> impl IntoResponse {
        let Ok(Some(old_state)) = session.get(CSRF_STATE_KEY).await else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        let creds = Credentials::OAuth(OAuthCreds {
            code,
            old_state,
            new_state,
        });

        let user = match auth_session.authenticate(creds).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    html! { 
                        ("Invalid credentials"
                            .to_string())
                    }
                )
                    .into_response()
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if let Ok(Some(next)) = session.remove::<String>(NEXT_URL_KEY).await {
            Redirect::to(&next).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }
}