use axum::Json;
use serde_json::json;

pub async fn base_route_response() -> Json<serde_json::Value> {
    Json(json!({ "message": "Welcome to liga-inggris API!" }))
}
