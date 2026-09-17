mod cliargs;
mod common;
mod data;
mod helper;
mod rgb;
mod splatoon;
mod state;

use axum::{Router, routing::get};
use clap::Parser;
use tower_http::services::ServeDir;

use crate::{
    cliargs::Cli,
    splatoon::{schedule, weapon},
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let addr = format!("0.0.0.0:{}", cli.port()?);
    println!("start: {addr}");

    let state = AppState::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(common::get_url_builder))
        .route("/docs", get(common::get_docs))
        // rotation
        .route("/open", get(schedule::get_open_now))
        .route("/regular", get(schedule::get_regular_now))
        .route("/open/{*schedule}", get(schedule::get_open_schedule))
        .route("/regular/{*schedule}", get(schedule::get_regular_schedule))
        // weapon
        .route("/weapon", get(weapon::get_weapon))
        .with_state(state)
        .nest_service("/shared-assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
