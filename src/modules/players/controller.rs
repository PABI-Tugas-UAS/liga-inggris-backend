use super::service;
use crate::config::api::response::BaseResponse;
use crate::modules::players::entity::PlayerReq;
use axum::response::Json;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

pub fn register() -> Router {
    Router::new()
        .route("/players", get(get_players))
        .route("/players/:id", get(get_player_by_id))
}

pub async fn get_players(
    Query(req): Query<PlayerReq>,
) -> Json<BaseResponse<Vec<serde_json::Value>>> {
    let data = service::get_players(req);

    Json(BaseResponse::success("Success fetching players", data))
}

pub async fn get_player_by_id(Path(id): Path<u64>) -> Json<BaseResponse<serde_json::Value>> {
    let data = service::get_player_by_id(id);

    match data {
        Some(player) => Json(BaseResponse::success("Success fetching player", player)),
        None => Json(BaseResponse::failure(
            "Player not found",
            serde_json::json!({}),
        )),
    }
}
