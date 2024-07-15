#![forbid(unsafe_code)]

pub mod routers;
pub mod middlewares;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use axum::{ middleware, Extension, Router, async_trait, extract::{FromRef, FromRequestParts}};
use axum_login::{
    login_required, tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer}, tracing::{self, Level}, AuthManagerLayerBuilder
};
use oso::{Oso, PolarClass};
use time::Duration;
use tower_http::trace::{self, TraceLayer};
use unbound_tome_utils::config::Config;
use unic_langid::LanguageIdentifier;

use http::{request::Parts, StatusCode};

use middlewares::auth::Backend;

use routers::{
    assets, 
    // auth, 
    health,
};

use middlewares::i10n;

use migration::{sea_orm::{Database, DatabaseConnection}, Migrator, MigratorTrait};
use std::sync::Arc;

use crate::webapp::App;

use domains::{
    campaigns::model::{
        Campaign,
        AUTHORIZATION as CAMPAIGNS_AUTHZ,
    }, 
    users::{model::{
        User as User, 
        AUTHORIZATION as USERS_AUTHZ,
    }, resolver::{UsersService, UsersServiceTrait}}
};

pub struct Context {
    /// The app config
    pub config: &'static Config,

    /// The `Oso` authorization library
    pub oso: Oso,

    /// The `User` entity service
    pub users: Arc<dyn UsersServiceTrait>,

    /// The database connections
    pub db: Arc<DatabaseConnection>,
}

impl Context {
    pub async fn new(config: &'static Config) -> Result<Self, Box<dyn std::error::Error>> {

        
        let db: Arc<DatabaseConnection> = Arc::new(Database::connect(&config.database.url).await?);

        Migrator::up(db.as_ref(), None).await.unwrap();

        // Set up authorization
        let mut oso = Oso::new();

        oso.register_class(User::get_polar_class_builder().name("User").build())?;
        oso.register_class(Campaign::get_polar_class_builder().name("Campaign").build())?;

        oso.load_str(&[USERS_AUTHZ, CAMPAIGNS_AUTHZ].join("\n"))?;

        Ok(Self { 
            config, 
            oso,
            users: Arc::new(UsersService::new(&db)),
            db,
        })
    }
}

#[derive(Clone)]
pub struct ServerState(pub std::sync::Arc<Context>);

#[async_trait]
impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

pub async fn serve(ctx: Arc<Context>) -> Result<(), Box<dyn std::error::Error>> {
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
    let backend = Backend::new(ctx.db.clone(), ctx.config.oauth.clone())?;
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let state = ServerState(ctx.clone());
    // Language list
    //
    let supported_languages = Arc::new(ctx.config.locale.supported_languages
        .iter()
        .filter_map(|lang| lang.parse::<LanguageIdentifier>().ok())
        .collect::<Vec<LanguageIdentifier>>());


    let app = Router::new()
            .serve_dioxus_application(ServeConfig::builder().build(), || VirtualDom::new(App)).await
            // .route_layer(login_required!(Backend, login_url = "/login"))
            // .merge(auth::router(ctx.config.oauth.enabled))
            .merge(health::router())
            .merge(assets::router())
            .layer(auth_layer)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new()
                        .level(Level::DEBUG))
                    .on_response(trace::DefaultOnResponse::new()
                        .level(Level::DEBUG)))
            .layer(Extension(ctx.config))
            .layer(Extension(ctx))
            .layer(Extension(state))
            .layer(middleware::from_fn_with_state(supported_languages.clone(), i10n::extract_preferred_language));

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Axum listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
