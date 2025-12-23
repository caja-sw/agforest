use serde::Serialize;

use crate::{dto::Author, entity::get_posts::PostEntity};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub total_count: i64,
    pub posts: Vec<Post>,
}

impl From<Vec<PostEntity>> for Response {
    fn from(value: Vec<PostEntity>) -> Self {
        Self {
            total_count: value.first().map_or(0, |post| post.total_count),
            posts: value.into_iter().map(Post::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub author: Author,
    pub title: String,
    pub created_at: String,
    pub comment_count: i64,
}

impl From<PostEntity> for Post {
    fn from(value: PostEntity) -> Self {
        Self {
            id: value.id,
            author: Author {
                name: value.author_name,
                hash: value.author_hash,
            },
            title: value.title,
            created_at: value.created_at.to_rfc3339(),
            comment_count: value.comment_count,
        }
    }
}
