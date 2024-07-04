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
    use views::determine_view;

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
            html!(
                div class="hero bg-base-200/0 flex-1"
                {
                    div class="hero-content flex-col lg:flex-row-reverse" {
                        div class="card lg:card-side lg:w-5/6 glass shadow-xl text-white" {
                            div class="card-body" {
                                h1 class="text-5xl card-title font-bold" {
                                    (hometitle)
                                }
                                p class="text-2xl py-6" {
                                    (hometext)
                                }
                                div class="card-actions justify-center" {
                                    button class="btn btn-primary" {
                                        (homebtn)
                                    }
                                }
                            }
                            figure {
                                img src="https://i.pinimg.com/originals/d5/98/46/d59846b06d0dd2a415c07af101aaf055.png"
                                class="max-w-sm rounded-lg shadow-2xl"
                                {}
                            }
                        }
                    }
                }
            )
        )
    }
}