use super::entity::MatchReq;
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
        .route("/matches", get(get_matches))
        .route("/matches/:id", get(get_match_by_id))
}

pub async fn get_matches(
    Query(req): Query<MatchReq>,
) -> Json<BaseResponse<Vec<serde_json::Value>>> {
    let data = service::get_matches(req);

    Json(BaseResponse::success("Success fetching matches", data))
}

pub async fn get_match_by_id(Path(id): Path<u64>) -> Json<BaseResponse<serde_json::Value>> {
    let data = service::get_match_by_id(id);

    match data {
        Some(_match) => Json(BaseResponse::success("Success fetching match", _match)),
        None => Json(BaseResponse::failure(
            "match not found",
            serde_json::json!({}),
        )),
    }
}
