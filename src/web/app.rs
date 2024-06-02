use std::env;

use axum_login::{
    login_required, tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer}, tracing::{self, Level}, AuthManagerLayerBuilder
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use sqlx::PgPool;
use time::Duration;
use tower_http::trace::{self, TraceLayer};

use crate::{
    users::Backend,
    web::{auth, oauth, home, protected},
};

pub struct App {
    db: PgPool,
    client: BasicClient,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv()?;

        let client_id = env::var("CLIENT_ID")
            .map(ClientId::new)
            .expect("CLIENT_ID should be provided.");
        let client_secret = env::var("CLIENT_SECRET")
            .map(ClientSecret::new)
            .expect("CLIENT_SECRET should be provided");

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL should be provided");

        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));

        let db = PgPool::connect(&db_url).await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self { db, client })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = Backend::new(self.db, self.client);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();


        let app = home::router()
            // .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .merge(oauth::router())
            .layer(auth_layer)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new()
                        .level(Level::DEBUG))
                    .on_response(trace::DefaultOnResponse::new()
                        .level(Level::DEBUG)));
        
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app.into_make_service()).await?;
        Ok(())
    }
}