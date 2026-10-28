use axum::{http::StatusCode, response::Html};

use crate::data::Response;

pub const SITE_URL: &str = "splat.site";

pub fn error_text() -> String {
    // TODO
    "Internal server error".to_owned()
}

const HTML_ERROR: &str = include_str!("../assets/html/error.html");

fn render_error_html(code: StatusCode, message: &str, hint: &str) -> String {
    HTML_ERROR
        .replace("{code}", code.as_str())
        .replace("{message}", message)
        .replace("{hint}", hint)
}

pub fn error_html() -> Html<String> {
    Html(render_error_html(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error",
        "しばらくしてからもうおためしください。",
    ))
}

pub fn error_response() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, error_html())
}

pub fn not_found_html() -> Response {
    (
        StatusCode::NOT_FOUND,
        Html(render_error_html(
            StatusCode::NOT_FOUND,
            "Page Not Found",
            "おさがしのページはみつかりませんでした。",
        )),
    )
}

pub fn escape_html<S: Into<String>>(s: S) -> String {
    s.into()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39")
}
