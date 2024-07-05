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
    use axum_htmx::headers;
    use fluent_bundle::FluentValue;
    use http::HeaderMap;
    use unic_langid::langid;
    use views::determine_view;

    use crate::web::middleware::i10n::PreferredLanguage;

    use super::*;

    pub async fn homepage(
        auth_session: AuthSession,
        HxRequest(hx_request): HxRequest,
        headers: HeaderMap,
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

        let hometitle = LOCALES
            .lookup_with_args(
                &preferred_language.as_ref().unwrap_or(&langid!("en")), 
                "home-hero-title", &args
        );
        let hometext = LOCALES
            .lookup_with_args(
                &preferred_language.as_ref().unwrap_or(&langid!("en")), 
                "home-hero-text", &args
        );

        let homebtn = LOCALES
        .lookup_with_args(
            &preferred_language.as_ref().unwrap_or(&langid!("en")), 
            "home-hero-btn", &args
        );

        determine_view(
            hx_request,
            &auth_session.user,
            views::home::render(hometitle, hometext, homebtn)
        )
    }
}