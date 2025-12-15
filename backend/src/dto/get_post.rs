use serde::Serialize;

use crate::{
    dao::GetPostPost,
    dto::{AuthorDTO, CategoryDTO},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostResponse {
    pub id: i32,
    pub author: AuthorDTO,
    pub category: CategoryDTO,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

impl From<GetPostPost> for GetPostResponse {
    fn from(post: GetPostPost) -> Self {
        Self {
            id: post.id,
            author: AuthorDTO {
                name: post.author_name,
                hash: post.author_hash,
            },
            category: CategoryDTO {
                id: post.category_id,
                name: post.category_name,
            },
            title: post.title,
            content: post.content,
            created_at: post.created_at.to_rfc3339(),
        }
    }
}
