use axum_login::tracing::debug;
use domains::users::model::Model;
use maud::{html, Markup};

pub mod shell;
pub mod navbar;
pub mod login;
pub mod register;
pub mod footer;

pub fn page(body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                script src="https://cdn.tailwindcss.com" {}
                link href="https://cdn.jsdelivr.net/npm/daisyui/dist/full.min.css" rel="stylesheet" type="text/css" {}
                script src="https://unpkg.com/htmx.org@2.0.0" {}
                script src="https://unpkg.com/htmx-ext-response-targets@2.0.0/response-targets.js" {}

                link rel="icon mask-icon" type="image/svg+xml" href="static/logo.svg";
                link rel="manifest" href="/app.webmanifest";
                title { "Unbound Tome" }

                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta charset="utf-8";
            }

            body hx-ext="response-targets" {
                (body)
            }
        }
    }
}

///Render a full page or a partial if the request is made with HTMX.
pub fn determine_view(hx_request: bool, userdata: &std::option::Option<Model>, body: Markup) -> Markup {
    if hx_request {
        debug!("PARTIAL");
        //partial hx-request
        shell::render(
            &userdata,
            html!(
                (body)
            )
        )
    } else {
        debug!("FULL");
        //fullpage load
        page(shell::render(
            &userdata,
            html!(
                (body)
            )
        ))
    }
}