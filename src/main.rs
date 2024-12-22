mod common;
mod config;
mod modules;

use axum::Router;
use modules::clubs::controller as clubs_controller;
use modules::matches::controller as matches_controller;
use modules::players::controller as players_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(clubs_controller::register())
        .merge(players_controller::register())
        .merge(matches_controller::register());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
