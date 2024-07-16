use axum::{
    extract::Query, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router
};
use axum_login::tower_sessions::Session;
use oauth2::CsrfToken;
use serde::Deserialize;

use crate::server::middlewares::auth::Session as AuthSession;
use crate::server::middlewares::auth::PasswordCreds;
use crate::server::middlewares::auth::Credentials;
use crate::server::middlewares::auth::OAuthCreds;
use axum::response::Response;
use axum::body::Body;

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

pub async fn login_password(
    mut auth_session: AuthSession,
    username: String,
    password: String,
    // Form(creds): Form<PasswordCreds>,
) -> Response<Body> {

    let creds = PasswordCreds {
        username,
        password,
        next: None,
    };
    
    let user = match auth_session.0
        .authenticate(Credentials::Password(creds.clone()))
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => 
            return (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
            ).into_response(),
        Err(_) => {
            let mut response = 
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong! Internal Server Error 500!",
            ).into_response();
        },
    };

    if auth_session.login(&user).await.is_err() {
        let mut response = 
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong! Internal Server Error 500!",
        ).into_response();
    }

    if let Some(ref next) = creds.next {
        return Redirect::to(next).into_response()
    } else {
        return Redirect::to("/").into_response()
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

    let user = match auth_session.0.authenticate(creds).await {
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

    if auth_session.0.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Ok(Some(next)) = session.remove::<String>(NEXT_URL_KEY).await {
        Redirect::to(&next).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.0.logout().await {
        Ok(_) => {
            let res = Response::builder()
                .status(200)
                .body(Body::empty())
                .unwrap();
            return res
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn register_password(
    mut auth_session: AuthSession,
    Form(creds): Form<PasswordCreds>,
    // Extension(ctx): Extension<Context>,
) -> impl IntoResponse {

    // let user = ctx.users.get_by_username(username, with_roles).await?;

    let user = match auth_session.0
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

    if auth_session.0.login(&user).await.is_err() {
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

pub async fn login_oauth(
    auth_session: AuthSession,
    session: Session,
    Form(NextUrl { next }): Form<NextUrl>,
) -> impl IntoResponse {
    let (auth_url, csrf_state) = auth_session.0.backend.authorize_url();

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

