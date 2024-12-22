mod common;
mod config;
mod modules;

use axum::Router;
use common::utils::env::get_env;
use modules::clubs::controller as clubs_controller;
use modules::matches::controller as matches_controller;
use modules::players::controller as players_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(clubs_controller::register())
        .merge(players_controller::register())
        .merge(matches_controller::register());

    let host = get_env("BE_HOST", "0.0.0.0");
    let port = get_env("BE_PORT", "3000");
    let address = format!("{}:{}", host, port);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server running on http://{}", address);
}
