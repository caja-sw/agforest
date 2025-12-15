use serde::Serialize;

use crate::{
    dao::GetPostsPost,
    dto::{AuthorDTO, CategoryDTO},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactPostDTO {
    pub id: i32,
    pub author: AuthorDTO,
    pub category: CategoryDTO,
    pub title: String,
    pub created_at: String,
}

impl From<GetPostsPost> for CompactPostDTO {
    fn from(post: GetPostsPost) -> Self {
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
            created_at: post.created_at.to_rfc3339(),
        }
    }
}
