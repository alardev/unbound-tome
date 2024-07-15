use migration::sea_orm::{ColumnTrait, ActiveModelTrait, Set};
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{tracing::{error, info}, AuthnBackend, UserId};
use domains::users::{self, model::Entity as User};
use migration::sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use oauth2::{
    basic::{BasicClient, BasicRequestTokenError}, reqwest::{async_http_client, AsyncHttpClientError}, url::Url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, TokenResponse, TokenUrl
};
use password_auth::verify_password;
use serde::Deserialize;
use tokio::task;
use unbound_tome_utils::config::OAuth;
use std::sync::Arc;


#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
    Password(PasswordCreds),
    OAuth(OAuthCreds),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordCreds {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCreds {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    login: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    SeaORM(migration::DbErr),

    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    OAuth2(BasicRequestTokenError<AsyncHttpClientError>),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Debug, Clone)]
pub struct Backend {
    conn: Arc<DatabaseConnection>,
    client: Option<BasicClient>,
}

impl Backend {
    pub fn new(conn: Arc<DatabaseConnection>, oauth_config: OAuth ) -> Result<Self, Box<dyn std::error::Error>> {
        
        // TODO: FIX THIS GARBAGE 💀💀💀💀💀

        let client = match oauth_config.enabled {
            true => {
                info!("OAuth2 enabled!");
                
                let client = match oauth_config.client {
                    Some(c) => c, 
                    None => {
                        error!("OAuth2 client section should be configured!");
                        return Err("OAuth2 client section should be configured!".into())
                    }
                };

                let client_id = match client.id {
                    Some(id) => {
                        ClientId::new(id)
                    },
                    None => {
                        error!("OAuth2 client id should be configured!💀");
                        return Err("OAuth2 client id should be configured!".into())
                    }
                };

                let client_secret = match client.secret {
                    Some(secret) => {
                        Some(ClientSecret::new(secret))
                    },
                    None => {
                        error!("OAuth2 client secret should be configured!");
                        return Err("OAuth2 client secret should be configured!".into())
                    }
                };

                let auth_url = match oauth_config.url {
                    Some(u) => {
                        let url = AuthUrl::new(u);
                        match url {
                            Ok(u) => u,
                            Err(_) => {
                                error!("Oauth2 auth url parsing failed!");
                                return Err("Oauth2 auth url parsing failed!".into())
                            }
                        }
                    },
                    None => {
                        error!("OAuth2 auth url should be configured!");
                        return Err("OAuth2 auth url should be configured!".into())
                    }
                };

                let token_url = match oauth_config.token_url {
                    Some(t) => {
                        let tokenurl = TokenUrl::new(t);
                        match tokenurl {
                            Ok(t) => Some(t),
                            Err(_) => {
                                error!("Oauth2 token url parsing failed!");
                                return Err("Oauth2 token url parsing failed!".into())
                            }
                        }
                    },
                    None => {
                        error!("OAuth2 token url should be configured!");
                        return Err("OAuth2 token url should be configured!".into())
                    }
                };
                
                Some(BasicClient::new(
                    client_id, 
                    client_secret,
                    auth_url,
                    token_url
                ))
            },
            false => None
        };

        Ok(Self { conn, client })
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {

        // Client unwrapped due to the fact that this method will only be called by oauth middleware.
        self.client.as_ref().unwrap().authorize_url(CsrfToken::new_random).url()
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = users::model::Model;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Self::Credentials::Password(password_cred) => {
                let user: Option<users::model::Model> = User::find()
                    .filter(domains::users::model::Column::Username.contains(password_cred.username))
                    .one(self.conn.as_ref())
                    .await
                    .map_err(Self::Error::SeaORM)?;

                // Verifying the password is blocking and potentially slow, so we'll do so via
                // `spawn_blocking`.

                // We're using password-based authentication: this works by comparing our form
                // input with an argon2 password hash.
                task::spawn_blocking(|| {
                    Ok(user.filter(|user| {
                        let Some(ref password) = user.password else {
                            return false;
                        };
                        verify_password(password_cred.password, password).is_ok()
                    }))
                })
                .await?
            }

            Self::Credentials::OAuth(oauth_creds) => {
                // Ensure the CSRF state has not been tampered with.
                if oauth_creds.old_state.secret() != oauth_creds.new_state.secret() {
                    return Ok(None);
                };

                // Process authorization code, expecting a token response back.
                let token_res = self
                    .client.as_ref().unwrap()
                    .exchange_code(AuthorizationCode::new(oauth_creds.code))
                    .request_async(async_http_client)
                    .await
                    .map_err(Self::Error::OAuth2)?;

                // Use access token to request user info.
                let user_info = reqwest::Client::new()
                    .get("https://api.github.com/user")
                    .header(USER_AGENT.as_str(), "axum-login") // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
                    .header(
                        AUTHORIZATION.as_str(),
                        format!("Bearer {}", token_res.access_token().secret()),
                    )
                    .send()
                    .await
                    .map_err(Self::Error::Reqwest)?
                    .json::<UserInfo>()
                    .await
                    .map_err(Self::Error::Reqwest)?;

                let user = users::model::ActiveModel {
                    username: Set(user_info.login),
                    access_token: Set(Some(token_res.access_token().secret().to_owned())),
                    ..Default::default()
                };

                let res = user.insert(self.conn.as_ref())
                    .await
                    .map_err(Self::Error::SeaORM)?;

                Ok(Some(res))
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(User::find_by_id(*user_id)
            .one(self.conn.as_ref())
            .await
            .map_err(Self::Error::SeaORM)?)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
// pub type AuthSession = axum_login::AuthSession<Backend>;

pub struct Session(
    pub axum_login::AuthSession<Backend>,
);

impl std::ops::Deref for Session {
    type Target = axum_login::AuthSession<Backend>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for Session
where
    S: std::marker::Sync + std::marker::Send,
{
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(parts: &mut http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        //
        axum_login::AuthSession::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                // let ss = parts.extensions.get::<ServerState>().unwrap();
                // let dbp = ss.0.clone();
                Session(auth_session)
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}