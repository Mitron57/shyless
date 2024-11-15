use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use crate::domain::interfaces::{ToKey};

#[derive(Deserialize, Serialize, Clone)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl ToKey for Credentials {
    type Key = dyn ToSql + Sync + Send;

    fn key(&self) -> &Self::Key {
        &self.login
    }
}