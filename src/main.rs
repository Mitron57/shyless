use crate::application::{AppState, JwtService, PostService};
use crate::handlers::{
    auth::{login, register},
    middlewares::validate_jwt,
    posts::{create_post, delete_post, get_post, like_post},
};
use crate::infrastructure::{PostRepository, Postgres, UserRepository};
use axum::middleware;
use axum::routing::{delete, get, post};
use axum::Router;
use log::info;
use std::error::Error;
use std::sync::Arc;

mod application;
mod domain;
mod handlers;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if dotenvy::dotenv().is_err() {
        info!("There's no .env file, trying to use manually specified variables");
    }
    env_logger::init();
    let addr = std::env::var("HOSTADDR")?;
    let postgres = Arc::new(Postgres::new().await?);
    let user_repository = Arc::new(UserRepository::new(postgres.clone()));
    let post_repository = Arc::new(PostRepository::new(postgres));
    let auth_service = Arc::new(JwtService::new()?);
    let post_service = Arc::new(PostService);

    let state = Arc::new(AppState {
        post_service,
        user_repository,
        post_repository,
        auth_service,
    });

    let auth = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state.clone());

    let posts = Router::new()
        .route("/posts", post(create_post))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_jwt))
        .route("/posts/:post_id", get(get_post))
        .route("/posts/:post_id", delete(delete_post))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_jwt))
        .route("/posts/:post_id/likes", post(like_post))
        .with_state(state.clone());

    let app = Router::new().merge(auth).merge(posts).with_state(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    Ok(axum::serve(listener, app).await?)
}
