use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::posts)]
pub struct GetPostPost {
    pub id: i32,
    pub author_name: String,
    pub author_hash: String,
    pub category_id: i32,
    pub category_name: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
