use axum_login::{
    login_required, tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer}, tracing::{self, Level}, AuthManagerLayerBuilder
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use oso::{Oso, PolarClass};
use time::Duration;
use tower_http::trace::{self, TraceLayer};
use unbound_tome_utils::config::Config;

use crate::{
    users::Backend,
    web::{account, auth, home, oauth},
};

use unbound_tome_service::sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};
use std::sync::Arc;

use domains::appuser::{User as Appuser, AUTHORIZATION as USERS_AUTHZ,};

pub struct App {
    /// The app config
    pub config: &'static Config,

    /// The database connections
    pub db: Arc<DatabaseConnection>,

    /// The `Oso` authorization library
    pub oso: Oso,

    /// The OAuth2 basic client
    pub client: BasicClient,
}

impl App {
    pub async fn new(config: &'static Config) -> Result<Self, Box<dyn std::error::Error>> {

        let db: Arc<DatabaseConnection> = Arc::new(Database::connect(&config.database.url).await?);

        Migrator::up(db.as_ref(), None).await.unwrap();

        // Set up authorization
        let mut oso = Oso::new();


        oso.register_class(Appuser::get_polar_class_builder().name("User").build())?;

        oso.load_str(&[USERS_AUTHZ].join("\n"))?;

        let client = BasicClient::new(
            config.auth.client.id.clone().map(ClientId::new).expect("CLIENT_ID should be provided."), 
            config.auth.client.secret.clone().map(ClientSecret::new), 
            AuthUrl::new(config.auth.url.clone())?, 
            Some(TokenUrl::new(config.auth.token_url.clone())?)
        );

        Ok(Self { 
            config, 
            db, 
            oso,
            client 
        })
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


        let app = account::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .merge(oauth::router())
            .merge(home::router())
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