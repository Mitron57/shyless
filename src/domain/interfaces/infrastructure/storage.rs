use axum::async_trait;

#[async_trait]
pub trait Storage<I, Id: ?Sized> {
    type Error;
    type Rows;

    async fn add(&self, item: I) -> Result<Self::Rows, Self::Error>;
    async fn get(&self, id: &Id) -> Result<Self::Rows, Self::Error>;

    async fn delete(&self, id: &Id) -> Result<(), Self::Error>;

    async fn update(&self, item: I) -> Result<(), Self::Error>;
}
