use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostEntity {
    pub id: i64,
    pub author_name: String,
    pub author_hash: String,
    pub title: String,
    pub created_at: DateTime<Utc>,

    pub total_count: i64,

    pub comment_count: i64,
}
