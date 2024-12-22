use axum::{http::StatusCode, Json};
use serde_json::json;

pub async fn not_found_route_response() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Route not found" })),
    )
}
