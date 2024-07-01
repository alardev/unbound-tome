use axum::{response::IntoResponse, routing::get, Router};
use axum_htmx::HxRequest;
use maud::html;
use crate::web::{
    views,
    middleware::auth::AuthSession
};

pub fn router() -> Router<()> {
    Router::new().route("/account", get(self::get::account))
}

mod get {

    use std::collections::HashMap;

    use axum::Extension;
    use fluent_bundle::FluentValue;
    use fluent_templates::Loader;
    use unic_langid::langid;
    use views::determine_view;

    use crate::web::middleware::i10n::{PreferredLanguage, LOCALES};

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
        
        let account_page_title = LOCALES
            .lookup_with_args(
                &preferred_language.unwrap_or(langid!("en")), 
                "account-page-title", &args
        );

        determine_view(
            hx_request,
            &auth_session.user,
            html!(

                (account_page_title)
            
            )
        )
    }
}