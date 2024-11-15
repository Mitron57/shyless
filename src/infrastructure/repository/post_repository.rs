use crate::domain::interfaces;
use crate::domain::interfaces::ToKey;
use crate::domain::models::Post;
use axum::async_trait;
use std::error;
use std::sync::Arc;
use tokio_postgres::{types, Row};

type ToSql = dyn types::ToSql + Sync + Send;
type Error = Box<dyn error::Error + Send + Sync>;
type Storage = dyn interfaces::Storage<Post, dyn ToKey<Key = ToSql>, Error = Error, Rows = Vec<Row>>
    + Send
    + Sync;
pub struct PostRepository {
    storage: Arc<Storage>,
}

impl PostRepository {
    pub fn new(storage: Arc<Storage>) -> PostRepository {
        PostRepository { storage }
    }
}

#[async_trait]
impl interfaces::Repository<Post> for PostRepository {
    type Error = Error;
    type Key = dyn ToKey<Key = ToSql>;

    async fn get(&self, id: &Self::Key) -> Result<Option<Post>, Self::Error> {
        let rows = self.storage.get(id).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = &rows[0];
        Ok(Some(Post {
            id: row.get("id"),
            author_id: row.get("author_id"),
            content: row.get("content"),
            likes: row.get("likes"),
        }))
    }

    async fn add(&self, item: Post) -> Result<i64, Self::Error> {
        let result = self.storage.add(item).await?;
        Ok(result[0].get("id"))
    }

    async fn erase(&self, id: &Self::Key) -> Result<(), Self::Error> {
        self.storage.delete(id).await?;
        Ok(())
    }

    async fn update(&self, item: Post) -> Result<(), Self::Error> {
        self.storage.update(item).await?;
        Ok(())
    }
}
