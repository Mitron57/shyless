use crate::domain::interfaces::{Repository, ToKey};
use crate::domain::models::User;
use tokio_postgres::types::ToSql;

pub trait UserRepository<E, F, V>:
    Repository<User, Error = E, Key = dyn ToKey<Key = dyn ToSql + Sync + Send>>
{
}
