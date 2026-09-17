use axum::{http::StatusCode, response::Html};

use crate::data::Response;

pub const SITE_URL: &str = "splat.site";

pub fn error_text() -> String {
    // TODO
    "Internal server error".to_owned()
}

pub fn error_html() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(error_text()))
}

pub fn escape_html<S: Into<String>>(s: S) -> String {
    s.into()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
