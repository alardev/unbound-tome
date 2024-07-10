use std::str::FromStr;

use axum::{extract::{Request, State}, middleware::Next, response::Response, RequestPartsExt};
use axum_extra::{headers::Header, TypedHeader};
use axum_login::tracing::{self, Level};
use http::{header, HeaderValue};
use thiserror::Error;
use unic_langid::LanguageIdentifier;
use std::sync::Arc;

fluent_templates::static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    pub static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "locales",
        // The language to falback on if something is not present.
        fallback_language: "en",
        // Optional: A fluent resource that is shared with every locale.
        core_locales: "locales/core.ftl",
    };
}

#[derive(Debug, Error)]
pub enum AcceptLanguageError {
    #[error("Invalid value")]
    InvalidValue,
}

pub struct AcceptedLanguage {
    pub value: String,
    pub quality: f32,
}

impl Eq for AcceptedLanguage {}

impl PartialEq for AcceptedLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.quality == other.quality && self.value.eq(&other.value)
    }
}

impl PartialOrd for AcceptedLanguage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AcceptedLanguage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.quality > other.quality {
            std::cmp::Ordering::Greater
        } else if self.quality < other.quality {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl FromStr for AcceptedLanguage {
    type Err = AcceptLanguageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = s.trim().split(';');
        let (value, quality) = (value.next(), value.next());

        let Some(value) = value else {
            return Err(AcceptLanguageError::InvalidValue);
        };

        if value.is_empty() {
            return Err(AcceptLanguageError::InvalidValue);
        }

        let quality = if let Some(quality) = quality.and_then(|q| q.strip_prefix("q=")) {
            quality.parse::<f32>().unwrap_or(0.0)
        } else {
            1.0
        };

        Ok(AcceptedLanguage {
            value: value.to_string(),
            quality,
        })
    }
}

pub struct AcceptLanguage(pub Vec<AcceptedLanguage>);

impl Header for AcceptLanguage {
    fn name() -> &'static axum::http::HeaderName {
        &header::ACCEPT_LANGUAGE
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum_extra::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let value = values.next().ok_or_else(axum_extra::headers::Error::invalid)?;
        let str = value.to_str().expect("Accept-Language must be a string");
        let mut languages = str
            .split(',')
            .map(AcceptedLanguage::from_str)
            .collect::<Result<Vec<AcceptedLanguage>, AcceptLanguageError>>()
            .map_err(|_| axum_extra::headers::Error::invalid())?;

        languages.sort();

        Ok(AcceptLanguage(languages))
    }

    // https://stackoverflow.com/questions/67262381/rust-no-method-named-join-found-for-struct-stdstringstring
    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        let val = self
            .0
            .iter()
            .map(|l| format!("{};q={}", l.value, l.quality))
            .collect::<Vec<String>>()
            .join(",");

        let val = HeaderValue::from_str(&val).expect("Accept-Language must be valid");

        values.extend(std::iter::once(val))
    }
}

#[derive(Clone)]
pub struct PreferredLanguage(pub Option<LanguageIdentifier>);

pub async fn extract_preferred_language(
    State(supported_languages): State<Arc<Vec<LanguageIdentifier>>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {

    let span = tracing::span!(Level::TRACE, "preferred language extraction");
    let _enter = span.enter();

    let (mut parts, body) = request.into_parts();

    let preferred_lang: Option<LanguageIdentifier> =
        if let Ok(TypedHeader(accept)) = parts.extract::<TypedHeader<AcceptLanguage>>().await {
            accept
                .0
                .iter()
                .filter_map(|lang| lang.value.parse::<LanguageIdentifier>().ok())
                .filter(|lang| supported_languages.to_owned().contains(lang))
                .collect::<Vec<LanguageIdentifier>>()
                .first()
                .map(|lang| lang.to_owned())
        } else {
            None
        };

    tracing::event!(
        Level::TRACE,
        "extracted preferred language: {:?}",
        preferred_lang
    );

    let mut request: Request<axum::body::Body> = Request::from_parts(parts, body);
    request
        .extensions_mut()
        .insert(PreferredLanguage(preferred_lang));

    next.run(request).await
}