use crate::application::dto::Credentials;
use crate::application::AppState;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

fn response(result: Result<String, Box<dyn Error + Send + Sync>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    match result {
        Ok(token) => {
            headers.insert(
                SET_COOKIE,
                format!("jwt={}; HttpOnly; Secure; SameSite=Lax", token)
                    .parse()
                    .unwrap(),
            );
            (StatusCode::OK, headers, Json(json!({})))
        }
        Err(error) => (
            StatusCode::UNAUTHORIZED,
            headers,
            Json(json!({"error" : error.to_string()})),
        ),
    }
}
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Credentials>,
) -> impl IntoResponse {
    let result = state
        .auth_service
        .login(&payload, state.user_repository.clone())
        .await;
    response(result)
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Credentials>,
) -> impl IntoResponse {
    let result = state
        .auth_service
        .register(&payload, state.user_repository.clone())
        .await;
    response(result)
}
