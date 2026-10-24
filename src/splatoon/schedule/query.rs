use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Html,
};

use crate::{
    data::{EmbedQuery, ScheduleInput},
    helper::error_response,
    splatoon::schedule::{Mode, get_info},
    state::AppState,
};

pub async fn get_open_schedule(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> (StatusCode, Html<String>) {
    get_info(state, schedule, Mode::BankaraOpen, query)
        .await
        .map(|h| (StatusCode::OK, h))
        .unwrap_or_else(|_| error_response())
}

pub async fn get_open_now(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
) -> (StatusCode, Html<String>) {
    get_info(state, ScheduleInput::Now, Mode::BankaraOpen, query)
        .await
        .map(|h| (StatusCode::OK, h))
        .unwrap_or_else(|_| error_response())
}

pub async fn get_regular_schedule(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
    Path(schedule): Path<ScheduleInput>,
) -> (StatusCode, Html<String>) {
    get_info(state, schedule, Mode::Regular, query)
        .await
        .map(|h| (StatusCode::OK, h))
        .unwrap_or_else(|_| error_response())
}

pub async fn get_regular_now(
    State(state): State<AppState>,
    Query(query): Query<EmbedQuery>,
) -> (StatusCode, Html<String>) {
    get_info(state, ScheduleInput::Now, Mode::Regular, query)
        .await
        .map(|h| (StatusCode::OK, h))
        .unwrap_or_else(|_| error_response())
}
