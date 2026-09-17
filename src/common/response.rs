use axum::response::Html;
use reqwest::StatusCode;

use crate::data::Response;

pub const HTML_URL_BUILDER: &str = include_str!("../../assets/html/url_builder.html");

pub fn url_builder() -> Response {
    (StatusCode::OK, Html(HTML_URL_BUILDER.to_owned()))
}

pub const HTML_DOCS: &str = include_str!("../../assets/html/docs.html");

pub fn docs() -> Response {
    (StatusCode::OK, Html(HTML_DOCS.to_owned()))
}
