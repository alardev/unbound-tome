use migration::sea_orm::{ColumnTrait, ActiveModelTrait, Set};
use async_trait::async_trait;
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{AuthnBackend, UserId};
use domains::appuser::{self, Entity as Appuser};
use migration::sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use oauth2::{
    basic::{BasicClient, BasicRequestTokenError},
    reqwest::{async_http_client, AsyncHttpClientError},
    url::Url,
    AuthorizationCode, CsrfToken, TokenResponse,
};
use password_auth::verify_password;
use serde::Deserialize;
use tokio::task;
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
    client: BasicClient,
}

impl Backend {
    pub fn new(conn: Arc<DatabaseConnection>, client: BasicClient) -> Self {
        Self { conn, client }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client.authorize_url(CsrfToken::new_random).url()
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = appuser::Model;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Self::Credentials::Password(password_cred) => {
                let user: Option<appuser::Model> = Appuser::find()
                    .filter(domains::appuser::Column::Username.contains(password_cred.username))
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
                    .client
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

                let user = appuser::ActiveModel {
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
        Ok(Appuser::find_by_id(*user_id)
            .one(self.conn.as_ref())
            .await
            .map_err(Self::Error::SeaORM)?)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;