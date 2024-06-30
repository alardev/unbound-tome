use axum::{response::IntoResponse, routing::get, Router};
use axum_htmx::HxRequest;
use maud::html;
use crate::web::{middleware::auth::AuthSession, views
};

use crate::web::middleware::i10n::LOCALES;
use fluent_templates::Loader;

pub fn router() -> Router<> {
    Router::new().route("/", get(self::get::homepage))
}

mod get {
    use std::collections::HashMap;

    use axum::Extension;
    use fluent_bundle::FluentValue;
    use unic_langid::langid;

    use crate::web::middleware::i10n::PreferredLanguage;

    use super::*;

    pub async fn homepage(
        auth_session: AuthSession,
        HxRequest(hx_request): HxRequest,
        Extension(PreferredLanguage(preferred_language)): Extension<PreferredLanguage>,
    ) -> impl IntoResponse  {


        let args: HashMap<String, FluentValue> = {
            let mut map: HashMap<String, FluentValue> = HashMap::new();
            map.insert(String::from("name"), 
                auth_session.user.clone()
                    .map_or("nobody".into(), |user| FluentValue::from(user.username)));
            map.insert(String::from("case"), FluentValue::from("vocative"));
            map
        };
    

        if hx_request {
            //partial hx-request
            views::shell::render(
                auth_session.user,
                html!(
                    p { 
                        (LOCALES.lookup_with_args(&preferred_language.unwrap(), "home-greeting", &args))
                    }
                    p { 
                        (LOCALES.lookup_with_args(&langid!("uk"), "home-greeting", &args))
                        (LOCALES.lookup_with_args(&langid!("uk"), "case-lol", &args)) 
                    }
                )
            )
        } else {
            //fullpage load
            views::page(views::shell::render(
                auth_session.user,
                html!(
                    p { (LOCALES.lookup(&preferred_language.unwrap(), "hello-world")) }
                    p { (LOCALES.lookup(&langid!("uk"), "hello-world")) }
                )
            ))
        }
    }
}