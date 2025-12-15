use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::comments)]
pub struct GetCommentsComment {
    pub id: i32,
    pub author_name: String,
    pub author_hash: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
