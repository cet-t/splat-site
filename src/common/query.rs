use crate::data::Response;

pub async fn get_url_builder() -> Response {
    crate::common::url_builder()
}

pub async fn get_docs() -> Response {
    crate::common::docs()
}

pub async fn get_not_found() -> Response {
    crate::helper::not_found_html()
}
