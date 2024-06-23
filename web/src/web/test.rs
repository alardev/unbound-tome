use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "test.html")]
struct TestTemplate;


pub fn router() -> Router<()> {
    Router::new().route("/test", get(self::get::test))
}

mod get {
    use super::*;

    pub async fn test() -> impl IntoResponse {
        TestTemplate.into_response()
    }
}