use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub content: String,
    pub likes: i64,
}