use axum::{
    routing::get, Router
};

pub fn router() -> Router {
    Router::new().nest_service("/static", get(self::get::handler))
}

mod get {

    use axum::{
        body::Body, http::{Request, Response, StatusCode, Uri}, Extension
    };
    use axum_login::tracing::debug;
    use tower::ServiceExt;
    use tower_http::services::ServeDir;
    use unbound_tome_utils::config::Config;
    
    #[axum::debug_handler]
    pub async fn handler(
        uri: Uri,
        Extension(config): Extension<&'static Config>
    ) -> Result<Response<Body>, (StatusCode, String)> {

        let res = get_static_file(uri.clone(), config.assets_path.clone()).await?;
    
        if res.status() == StatusCode::NOT_FOUND {
            // try with `.html`
            // TODO: handle if the Uri has query parameters
            match format!("{}.html", uri).parse() {
                Ok(uri_html) => get_static_file(uri_html, config.assets_path.clone()).await,
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
            }
        } else {
            Ok(res)
        }
    }
    
    async fn get_static_file(
        uri: Uri,
        assets_path: String
    ) -> Result<Response<Body>, (StatusCode, String)> {
        let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
    
        debug!("{:?} {:?}", uri, &assets_path);
    
        // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
        match ServeDir::new(&assets_path).oneshot(req).await {
            Ok(res) => Ok(res.map(Body::new)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )),
        }
    }

}

