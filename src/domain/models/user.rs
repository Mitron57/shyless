use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub password: String,
}


