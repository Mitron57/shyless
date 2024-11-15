use crate::application::AppState;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::io;
use std::sync::Arc;

pub async fn like_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i64>,
    _: Request,
) -> impl IntoResponse {
    match state
        .post_service
        .like(post_id, state.post_repository.clone())
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(error) => {
            if let Some(io_error) = error.downcast_ref::<io::Error>() {
                if io_error.kind() == io::ErrorKind::NotFound {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Post with this id is not found"})),
                    );
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error.to_string() })),
            )
        }
    }
}
