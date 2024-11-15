use axum::async_trait;
use crate::domain::models::Post;

#[async_trait]
pub trait PostService {
    type Repository;
    type Error;
    async fn create(&self, author_id: i64, content: String, repo: Self::Repository) -> Result<(), Self::Error>;
    async fn delete(&self, id: i64, repo: Self::Repository) -> Result<(), Self::Error>;
    async fn get(&self, id: i64, repo: Self::Repository) -> Result<Option<Post>, Self::Error>;
    async fn like (&self, id: i64, repo: Self::Repository) -> Result<(), Self::Error>;
}