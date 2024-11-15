use crate::application::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn validate_jwt(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let token = match request.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => token,
            Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    match state
        .auth_service
        .parse_and_validate(token, state.user_repository.clone())
        .await
    {
        Ok(_) => next.run(request).await,
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
