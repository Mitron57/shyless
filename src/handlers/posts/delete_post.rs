use crate::application::AppState;
use crate::domain::models::User;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i64>,
    request: Request,
) -> impl IntoResponse {
    let token = request.headers()["Authorization"].to_str().unwrap(); //ensured by middleware
    let User { id, .. } = state.auth_service.parse(token).await.unwrap();
    let post = match state
        .post_service
        .get(post_id, state.post_repository.clone())
        .await
    {
        Ok(Some(post)) => post,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Post with this id not found"})),
            )
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error.to_string()})),
            )
        }
    };
    if id == post.author_id {
        if let Err(error) = state
            .post_service
            .delete(post_id, state.post_repository.clone())
            .await
        {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error.to_string()})),
            )
        } else {
            (StatusCode::NO_CONTENT, Json(json!({})))
        }
    } else {
        (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Post with this id does not belong to this user"})),
        )
    }
}
