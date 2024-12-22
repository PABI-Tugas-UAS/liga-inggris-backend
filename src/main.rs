mod common;
mod config;
mod modules;

use axum::{routing::get, Router};
use common::utils::env::get_env;
use config::base_route_response::base_route_response;
use config::not_found_route_response::not_found_route_response;
use modules::clubs::controller as clubs_controller;
use modules::matches::controller as matches_controller;
use modules::players::controller as players_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(base_route_response))
        .merge(clubs_controller::register())
        .merge(players_controller::register())
        .merge(matches_controller::register())
        .fallback(not_found_route_response);

    let host = get_env("BE_HOST", "0.0.0.0");
    let port = get_env("BE_PORT", "3000");
    let address = format!("{}:{}", host, port);

    println!("Server running on http://{}", address);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
