use axum::extract::{Query, State};
use reqwest::StatusCode;

use crate::{
    data::{EmbedQuery, Response},
    helper::error_html,
    splatoon::weapon::get_info,
    state::AppState,
};

pub async fn get_weapon(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
) -> Response {
    if let Ok(r) = get_info(state, query).await {
        (StatusCode::OK, r)
    } else {
        error_html()
    }
}
