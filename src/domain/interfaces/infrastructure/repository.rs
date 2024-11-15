use axum::async_trait;

#[async_trait]
pub trait Repository<I>: Sync + Send {
    type Error;
    type Key: ?Sized;

    async fn get(&self, id: &Self::Key) -> Result<Option<I>, Self::Error>;
    async fn add(&self, item: I) -> Result<i64, Self::Error>;

    async fn erase(&self, id: &Self::Key) -> Result<(), Self::Error>;

    async fn update(&self, item: I) -> Result<(), Self::Error>;
}
