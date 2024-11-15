use crate::application::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .post_service
        .get(post_id, state.post_repository.clone())
        .await
    {
        Ok(Some(post)) => (StatusCode::OK, Json(json!(post))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post with this id not found"})),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}
