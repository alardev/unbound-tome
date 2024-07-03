use std::collections::HashMap;

use axum::{response::IntoResponse, routing::get, Router};
use axum_htmx::HxRequest;
use fluent_bundle::FluentValue;
use fluent_templates::Loader;
use maud::html;
use unic_langid::{langid, LanguageIdentifier};
use crate::web::{
    middleware::{auth::AuthSession, i10n::LOCALES}, views
};

pub fn router() -> Router<()> {
    Router::new().route("/account", get(self::get::account))
}

mod get {

    use std::collections::HashMap;

    use axum::Extension;
    use fluent_bundle::FluentValue;
    use views::determine_view;

    use crate::web::middleware::i10n::PreferredLanguage;

    use super::*;

    pub async fn account(
        HxRequest(hx_request): HxRequest,
        auth_session: AuthSession,
        Extension(PreferredLanguage(preferred_language)): Extension<PreferredLanguage>,
    ) -> impl IntoResponse {

        let args: HashMap<String, FluentValue> = {
            let mut map: HashMap<String, FluentValue> = HashMap::new();
            map.insert(String::from("name"), 
                auth_session.user.clone()
                    .map_or("nobody".into(), |user| FluentValue::from(user.username)));
            map
        };
        
        determine_view(
            hx_request,
            &auth_session.user,
            html!(

                h1 class="text-2xl font-semibold" { (get_message(preferred_language, "acc-title".to_string())) }
                div class="flex items-center" {
                    hr class="flex-grow border-t border-accent" {}
                } 
            )
        )
    }
}

fn get_message(
    preferred_language: Option<LanguageIdentifier>, 
    key: String
) -> String {
    LOCALES.lookup(
        &preferred_language.unwrap_or(langid!("en")), 
        &key,
    )
}

fn get_message_args(
    preferred_language: Option<LanguageIdentifier>, 
    key: String, 
    args: HashMap<String, FluentValue>
) -> String {
    LOCALES.lookup_with_args(
        &preferred_language.unwrap_or(langid!("en")), 
        &key,
        &args
    )
}