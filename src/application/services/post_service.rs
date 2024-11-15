use crate::domain::interfaces;
use crate::domain::interfaces::{Repository, ToKey};
use crate::domain::models::Post;
use axum::async_trait;
use std::io::ErrorKind;
use std::sync::Arc;
use std::{error, io};
use tokio_postgres::types;

type Error = Box<dyn error::Error + Sync + Send>;
type ToSql = dyn types::ToSql + Sync + Send;

type PostRepo = dyn Repository<Post, Key = dyn ToKey<Key = ToSql>, Error = Error>;

pub struct PostService;

#[async_trait]
impl interfaces::PostService for PostService {
    type Repository = Arc<PostRepo>;
    type Error = Error;

    async fn create(
        &self,
        author_id: i64,
        content: String,
        repo: Self::Repository,
    ) -> Result<(), Self::Error> {
        let post = Post {
            id: 0,
            author_id,
            content,
            likes: 0,
        };
        repo.add(post).await?;
        Ok(())
    }

    async fn delete(&self, id: i64, repo: Self::Repository) -> Result<(), Self::Error> {
        repo.erase(&id).await?;
        Ok(())
    }

    async fn get(&self, id: i64, repo: Self::Repository) -> Result<Option<Post>, Self::Error> {
        repo.get(&id).await
    }

    async fn like(&self, id: i64, repo: Self::Repository) -> Result<(), Self::Error> {
        let mut post = match repo.get(&id).await {
            Ok(Some(post)) => post,
            Ok(None) => return Err(io::Error::from(ErrorKind::NotFound).into()),
            Err(error) => return Err(error),
        };
        post.likes += 1;
        repo.update(post).await?;
        Ok(())
    }
}
