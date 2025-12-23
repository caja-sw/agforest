use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostEntity {
    pub id: i64,
    pub category_id: i64,
    pub category_name: String,
    pub author_name: String,
    pub author_hash: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CommentEntity {
    pub id: i64,
    pub author_name: String,
    pub author_hash: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
