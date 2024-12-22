mod common;
mod config;
mod modules;

use axum::Router;
use modules::clubs::controller as clubs_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(clubs_controller::register())
        .merge(players_controller::register());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
