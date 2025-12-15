use serde::Serialize;

use crate::{dao::GetCommentsComment, dto::AuthorDTO};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDTO {
    pub id: i32,
    pub author: AuthorDTO,
    pub content: String,
    pub created_at: String,
}

impl From<GetCommentsComment> for CommentDTO {
    fn from(comment: GetCommentsComment) -> Self {
        Self {
            id: comment.id,
            author: AuthorDTO {
                name: comment.author_name,
                hash: comment.author_hash,
            },
            content: comment.content,
            created_at: comment.created_at.to_rfc3339(),
        }
    }
}
