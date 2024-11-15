use crate::domain::interfaces;
use crate::domain::interfaces::ToKey;
use crate::domain::models::User;
use axum::async_trait;
use std::error;
use std::sync::Arc;
use tokio_postgres::types;
use tokio_postgres::Row;

type Error = Box<dyn error::Error + Send + Sync>;

type ToSql = dyn types::ToSql + Sync + Send;

type Storage = dyn interfaces::Storage<User, dyn ToKey<Key = ToSql>, Error = Error, Rows = Vec<Row>>
    + Send
    + Sync;
pub struct UserRepository {
    storage: Arc<Storage>,
}

impl UserRepository {
    pub fn new(storage: Arc<Storage>) -> UserRepository {
        UserRepository { storage }
    }
}

#[async_trait]
impl interfaces::Repository<User> for UserRepository {
    type Error = Error;
    type Key = dyn ToKey<Key = ToSql>;

    async fn get(&self, id: &Self::Key) -> Result<Option<User>, Self::Error> {
        let rows = self.storage.get(id).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = &rows[0];
        Ok(Some(User {
            id: row.get("id"),
            login: row.get("login"),
            password: row.get("password"),
        }))
    }

    async fn add(&self, item: User) -> Result<i64, Self::Error> {
        let result = self.storage.add(item).await?;
        Ok(result[0].get("id"))
    }

    async fn erase(&self, id: &Self::Key) -> Result<(), Self::Error> {
        self.storage.delete(id).await?;
        Ok(())
    }

    async fn update(&self, user: User) -> Result<(), Self::Error> {
        self.storage.update(user).await?;
        Ok(())
    }
}

impl interfaces::UserRepository<Error, String, Box<ToSql>> for UserRepository {}
