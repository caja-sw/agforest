use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryEntity {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct PostEntity {
    pub total_post_count: i64,

    pub id: i64,
    pub author_name: String,
    pub author_hash: String,
    pub title: String,
    pub created_at: DateTime<Utc>,

    pub comment_count: i64,
}
