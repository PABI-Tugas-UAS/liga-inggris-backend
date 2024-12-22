use super::entity::ClubReq;
use super::service;
use crate::config::api::response::BaseResponse;
use axum::response::Json;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

pub fn register() -> Router {
    Router::new()
        .route("/clubs", get(get_clubs))
        .route("/clubs/:id", get(get_club_by_id))
}

pub async fn get_clubs(Query(req): Query<ClubReq>) -> Json<BaseResponse<Vec<serde_json::Value>>> {
    let data = service::get_clubs(req);

    Json(BaseResponse::success("Success fetching clubs", data))
}

pub async fn get_club_by_id(Path(id): Path<u64>) -> Json<BaseResponse<serde_json::Value>> {
    let data = service::get_club_by_id(id);

    match data {
        Some(club) => Json(BaseResponse::success("Success fetching club", club)),
        None => Json(BaseResponse::failure(
            "Club not found",
            serde_json::json!({}),
        )),
    }
}
