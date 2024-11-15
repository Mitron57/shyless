use crate::application::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{body, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> impl IntoResponse {
    let token = request.headers()["Authorization"].to_str().unwrap(); //ensured by middleware
    let user = state.auth_service.parse(token).await.unwrap();
    let content = match body::to_bytes(request.into_body(), usize::MAX).await {
        Ok(content) => content,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid request body"})),
            )
        }
    };
    let content = match String::from_utf8(content.to_vec()) {
        Ok(content) => content,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid UTF-8 data"})),
            )
        }
    };
    if let Err(error) = state
        .post_service
        .create(user.id, content, state.post_repository.clone())
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        );
    }
    (StatusCode::CREATED, Json(json!({})))
}
