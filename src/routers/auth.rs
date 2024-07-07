use axum::{
    extract::Query, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router
};
use axum_login::tower_sessions::Session;
use oauth2::CsrfToken;
use serde::Deserialize;

use crate::middleware::auth::AuthSession;

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
        use http::Response;
        use maud::html;
        use serde_json::json;
        

        use super::*;
        use crate::{middleware::auth::{Credentials, PasswordCreds}, views};

        pub async fn password(
            mut auth_session: AuthSession,
            Form(creds): Form<PasswordCreds>,
        ) -> impl IntoResponse {
            let user = match auth_session
                .authenticate(Credentials::Password(creds.clone()))
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => return (
                                StatusCode::UNAUTHORIZED,
                                "Invalid credentials".to_string().into_response()
                            ),
                Err(_) => {
                    let mut response = "Something went wrong! Internal Server Error 500!".into_response();
                    return (StatusCode::INTERNAL_SERVER_ERROR, response)
                },
            };

            if auth_session.login(&user).await.is_err() {
                let mut response = html! {("Something went wrong! Internal Server Error 500!")}.into_response();
                return (StatusCode::INTERNAL_SERVER_ERROR, response)
            }

            // let (_parts, body) = html!((views::navbar::render_logout_button(creds.username))).into_response().into_parts();

            // if let Some(ref next) = creds.next {
            //     let res = Response::builder()
            //         .status(200)
            //         .header("HX-Location",
            //             json!({
            //                 "path": next,
            //                 "target": "#tab-content",
            //             }).to_string()
            //         )
            //         .header("Content-Type", "text/html; charset=utf-8")
            //         .body(body)
            //         .unwrap();
            //     return res
            // } else {
            //     let res = Response::builder()
            //         .status(200)
            //         .header("HX-Location", 
            //     json!({
            //                 "path": "/",
            //                 "target": "#tab-content",
            //             }).to_string()
            //         )
            //         .header("Content-Type", "text/html; charset=utf-8")
            //         .body(body)
            //         .unwrap();
            //     return res
            // }

            (StatusCode::OK ,"lmao".into_response())
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
        use http::Response;

        use super::*;
        use crate::{middleware::auth::{Credentials, PasswordCreds}, views};


        pub async fn password(
            mut auth_session: AuthSession,
            Form(creds): Form<PasswordCreds>,
            // Extension(ctx): Extension<Context>,
        ) -> impl IntoResponse {

            // let user = ctx.users.get_by_username(username, with_roles).await?;

            let user = match auth_session
                .authenticate(Credentials::Password(creds.clone()))
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => return (
                                StatusCode::UNAUTHORIZED,
                                "Invalid credentials".to_string().into_response()),
                Err(_) => {
                    let mut response = "Something went wrong! Internal Server Error 500!".into_response();
                    return (StatusCode::INTERNAL_SERVER_ERROR, response)
                },
            };

            if auth_session.login(&user).await.is_err() {
                let mut response = "Something went wrong! Internal Server Error 500!".into_response();
                return (StatusCode::INTERNAL_SERVER_ERROR, response)
            }

            // if let Some(ref next) = creds.next {
            //     // Redirect::to(next).into_response()
            //     let res = Response::builder()
            //         .status(200)
            //         .header("HX-Location", format!("{{\"path\":\"{}\", \"target\":\"#tab-content\"}}", next))
            //         .body(Body::empty())
            //         .unwrap();
            //     return res
            // } else {
            //     // Redirect::to("/").into_response()
            //     let res = Response::builder()
            //         .status(200)
            //         .header("HX-Location", "{\"path\":\"/\", \"target\":\"#tab-content\"}")
            //         .body(Body::empty())
            //         .unwrap();
            //     return res
            // }

            (StatusCode::OK ,"lmaoooooo".into_response())

        }
    }
}

mod get {
    use axum::body::Body;
    use http::Response;

    use crate::middleware::auth::OAuthCreds;
    use crate::views;

    use crate::{
        routers::auth::NEXT_URL_KEY,
        middleware::auth::{AuthSession, Credentials},
    };
    

    use super::*;

    pub async fn login(
        Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
            views::login::render(None, next)
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
        Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
            views::register::render(None, next)
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
                        "Invalid credentials"
                            .to_string()
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