use axum::{
    http::{Response, StatusCode},
    response::{IntoResponse, Json},
    routing::{get, post},
    Error, Router,
};
use serde_json::json;

pub fn post_routes() -> Router {
    Router::new().route("/", get(post_handler))
}

pub async fn post_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"status": "incorrect password가나다라"})),
    )
}
