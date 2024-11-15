use tokio_postgres::types::ToSql;

pub trait ToKey: Sync + Send {
    type Key: ?Sized;
    fn key(&self) -> &Self::Key;
}

impl ToKey for i64 {
    type Key = dyn ToSql + Send + Sync;
    fn key(&self) -> &Self::Key {
        self
    }
}
