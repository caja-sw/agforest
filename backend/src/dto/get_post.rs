use serde::Serialize;

use crate::{
    dto::{Author, Category},
    entity::get_post::{CommentEntity, PostEntity},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: i64,
    pub category: Category,
    pub author: Author,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub comments: Vec<Comment>,
}

impl From<(PostEntity, Vec<CommentEntity>)> for Response {
    fn from((post, comments): (PostEntity, Vec<CommentEntity>)) -> Self {
        Self {
            id: post.id,
            category: Category {
                id: post.category_id,
                name: post.category_name,
            },
            author: Author {
                name: post.author_name,
                hash: post.author_hash,
            },
            title: post.title,
            content: post.content,
            created_at: post.created_at.to_rfc3339(),
            comments: comments.into_iter().map(Comment::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i64,
    pub author: Author,
    pub content: String,
    pub created_at: String,
}

impl From<CommentEntity> for Comment {
    fn from(value: CommentEntity) -> Self {
        Self {
            id: value.id,
            author: Author {
                name: value.author_name,
                hash: value.author_hash,
            },
            content: value.content,
            created_at: value.created_at.to_rfc3339(),
        }
    }
}
