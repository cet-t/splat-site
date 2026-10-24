use axum::extract::{Query, State};
use reqwest::StatusCode;

use crate::{
    data::{EmbedQuery, Response},
    helper::error_response,
    splatoon::weapon::get_info,
    state::AppState,
};

pub async fn get_weapon(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
) -> Response {
    get_info(state, query)
        .await
        .map(|r| (StatusCode::OK, r))
        .unwrap_or(error_response())
}
