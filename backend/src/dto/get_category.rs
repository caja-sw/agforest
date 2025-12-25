use serde::Serialize;

use crate::{
    dto::Author,
    entity::get_category::{CategoryEntity, PostEntity},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: i64,
    pub name: String,
    pub total_post_count: i64,
    pub posts: Vec<Post>,
}

impl From<(CategoryEntity, i64, Vec<PostEntity>)> for Response {
    fn from((category, total_post_count, posts): (CategoryEntity, i64, Vec<PostEntity>)) -> Self {
        Self {
            id: category.id,
            name: category.name,
            total_post_count,
            posts: posts.into_iter().map(Post::from).collect(),
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
