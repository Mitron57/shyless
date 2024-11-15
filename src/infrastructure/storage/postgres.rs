use crate::domain::interfaces::{Storage, ToKey};
use crate::domain::models::{Post, User};
use axum::async_trait;
use deadpool_postgres::{GenericClient, Manager, Pool, Transaction};
use std::error;
use std::str::FromStr;
use tokio_postgres::types;
use tokio_postgres::{NoTls, Row};

type Error = Box<dyn error::Error + Send + Sync>;
type ToSql = dyn types::ToSql + Sync + Send;

pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    pub async fn new() -> Result<Self, Error> {
        let uri = match std::env::var("DATABASE_URI") {
            Ok(uri) => uri,
            Err(_) => return Err("DATABASE_URI is not set or set in an unsupported format".into()),
        };
        let config = tokio_postgres::Config::from_str(uri.as_str())?;
        let manager = Manager::new(config, NoTls);
        let pool = Pool::builder(manager).build()?;
        Ok(Self { pool })
    }

    async fn end_or_rollback(
        transaction: Transaction<'_>,
        result: Result<Vec<Row>, tokio_postgres::Error>,
    ) -> Result<Vec<Row>, Error> {
        match result {
            Ok(rows) => {
                transaction.commit().await?;
                Ok(rows)
            }
            Err(error) => {
                transaction.rollback().await?;
                Err(error.into())
            }
        }
    }
}

#[async_trait]
impl<Id: ?Sized + ToKey<Key = ToSql>> Storage<User, Id> for Postgres {
    type Error = Error;
    type Rows = Vec<Row>;

    async fn add(&self, item: User) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("INSERT INTO Users(login, password) VALUES ($1, $2) RETURNING id")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .query(&query, &[&item.login, &item.password])
            .await;
        Self::end_or_rollback(transaction, result).await
    }

    async fn get(&self, id: &Id) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("SELECT * FROM Users WHERE login = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&query, &[id.key()]).await;
        Self::end_or_rollback(transaction, result).await
    }

    async fn delete(&self, id: &Id) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("DELETE * FROM Users WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&query, &[id.key()]).await;
        Self::end_or_rollback(transaction, result).await?;
        Ok(())
    }

    async fn update(&self, user: User) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("UPDATE Users SET login = $1, password = $2 WHERE id = $3")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .query(&query, &[&user.login, &user.password])
            .await;
        Self::end_or_rollback(transaction, result).await?;
        Ok(())
    }
}

#[async_trait]
impl<Id: ?Sized + ToKey<Key = ToSql>> Storage<Post, Id> for Postgres {
    type Error = Error;
    type Rows = Vec<Row>;

    async fn add(&self, item: Post) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached(
                "INSERT INTO Posts(author_id, content, likes) VALUES ($1, $2, $3) RETURNING id",
            )
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .query(&query, &[&item.author_id, &item.content, &item.likes])
            .await;
        Self::end_or_rollback(transaction, result).await
    }

    async fn get(&self, id: &Id) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("SELECT * FROM Posts WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&query, &[id.key()]).await;
        Self::end_or_rollback(transaction, result).await
    }

    async fn delete(&self, id: &Id) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("DELETE * FROM Posts WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&query, &[id.key()]).await;
        Self::end_or_rollback(transaction, result).await?;
        Ok(())
    }

    async fn update(&self, post: Post) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let query = connection
            .prepare_cached("UPDATE Posts SET likes = $1, content = $2 WHERE id = $3")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .query(&query, &[&post.likes, &post.content, &post.id])
            .await;
        Self::end_or_rollback(transaction, result).await?;
        Ok(())
    }
}
