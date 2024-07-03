use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use std::env;

/// The default `Config` instance
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

/// Database pool config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbPool {
    /// Database pool min
    pub min: Option<i16>,
    /// Database pool max
    pub max: Option<i16>,
}

/// Database config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    /// Database hostname/IP
    pub hostname: String,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
    /// Database name
    pub name: String,
    /// Database port
    pub port: u16,
    /// Full database url
    pub url: String,
    /// Database debug logging
    pub debug: bool,
    /// Database pool config
    pub pool: DbPool,
}

/// Valkey config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valkey {
    /// Valkey url
    pub url: String,
}

/// Auth client config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthClient {
    /// OAuth2 client id
    pub id: Option<String>,
    /// OAuth2 client secret
    pub secret: Option<String>,
}

/// Auth config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuth {
    /// OAuth2 switch
    pub enabled: bool,
    /// OAuth2 url
    pub url: Option<String>,
    /// OAuth2 Access Token url
    pub token_url: Option<String>,
    /// OAuth2 audience
    pub audience: Option<String>,
    /// Auth client config
    pub client: Option<OAuthClient>,
}

/// Locale config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Locale {
    /// Supported languages list
    pub supported_languages: Vec<String>
}

impl Default for Locale {
    fn default() -> Locale {
        Locale {
            supported_languages: vec!["en".to_string()]
        }
    }
}


/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The application's run mode (typically "development" or "production")
    pub run_mode: String,
    /// The port to bind to
    pub port: u16,
    /// Database config
    pub database: Database,
    /// Valkey config
    pub valkey: Valkey,
    /// Auth config
    pub oauth: OAuth,
    /// Locale config
    pub locale: Locale,
    /// Assets config
    pub assets_path: String
}

impl Config {
    /// Create a new Config by merging in various sources
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());

        let config: Config = Figment::new()
            // Load defaults
            .merge(Toml::file("config/default.toml"))
            // Load local overrides
            .merge(Toml::file("config/local.toml"))
            // Load run mode overrides
            .merge(Toml::file(format!("config/{}.toml", run_mode)))
            // Load environment variables
            .merge(
                // Support the nested structure of the config manually
                Env::raw()
                    // Split the Database variables
                    .map(|key| {
                        key.as_str()
                            .replace("DATABASE_POOL_", "DATABASE.POOL.")
                            .into()
                    })
                    .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
                    // Split the Valkey variables
                    .map(|key| key.as_str().replace("VALKEY_", "VALKEY.").into())
                    // Split the Auth variables
                    .map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
                    .map(|key| key.as_str().replace("OAUTH_", "OAUTH.").into()),
            )
            // Serialize and freeze
            .extract()?;

        Ok(config)
    }

    /// Return true if the `run_mode` is "development"
    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}